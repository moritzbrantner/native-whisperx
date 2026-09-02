use std::io::Read;
use std::time::Instant;

use audio_analysis_transcription::{
    BoundedPcmWindow as UpstreamPcmWindow, BoundedPcmWindowConfig, BoundedPcmWindowSession,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::workflow::CancellationHandle;

const DEFAULT_STABILITY_TOLERANCE_SECONDS: f64 = 0.4;
pub const LIVE_PCM_SAMPLE_RATE: u32 = 16_000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiveWindowingConfig {
    pub window_seconds: f64,
    pub hop_seconds: f64,
    pub finalize_lag_seconds: f64,
    pub max_buffer_lag_seconds: f64,
    pub stability_tolerance_seconds: f64,
}

impl Default for LiveWindowingConfig {
    fn default() -> Self {
        Self {
            window_seconds: 5.0,
            hop_seconds: 2.5,
            finalize_lag_seconds: 5.0,
            max_buffer_lag_seconds: 30.0,
            stability_tolerance_seconds: DEFAULT_STABILITY_TOLERANCE_SECONDS,
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum LiveWindowingError {
    #[error("{field} must be finite and greater than zero")]
    InvalidPositiveSeconds { field: &'static str },
    #[error("hop_seconds must not exceed window_seconds")]
    HopExceedsWindow,
    #[error("invalid bounded Near-Live Window configuration: {message}")]
    InvalidBoundedWindowConfig { message: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiveAsrSegmentCandidate {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub start_at_utc: String,
    pub end_at_utc: String,
    pub text: String,
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiveWindowTranscriptObservation {
    pub session_id: String,
    pub window_start_seconds: f64,
    pub window_end_seconds: f64,
    pub window_start_at_utc: String,
    pub window_end_at_utc: String,
    pub latest_ingested_audio_seconds: f64,
    pub segments: Vec<LiveAsrSegmentCandidate>,
}

/// One ordered progress observation from a Live Feed Transcription session.
///
/// These events are operational telemetry for embedding applications. They are
/// intentionally separate from [`LiveTranscriptEvent`], which remains the
/// transcript output contract.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum LiveTranscriptionProgressEvent {
    SessionStart {
        session_id: String,
    },
    WindowStart {
        session_id: String,
        window_index: usize,
        start_seconds: f64,
        end_seconds: f64,
    },
    ModelResolutionStart {
        session_id: String,
        window_index: usize,
        provider: String,
        model_id: String,
    },
    ModelResolutionEnd {
        session_id: String,
        window_index: usize,
        provider: String,
        model_id: String,
        source: String,
    },
    ModelDownloadStart {
        session_id: String,
        window_index: usize,
        provider: String,
        model_id: String,
    },
    ModelDownloadEnd {
        session_id: String,
        window_index: usize,
        provider: String,
        model_id: String,
        duration_seconds: f64,
    },
    ModelLoadStart {
        session_id: String,
        window_index: usize,
        provider: String,
        model_id: String,
    },
    ModelLoadEnd {
        session_id: String,
        window_index: usize,
        provider: String,
        model_id: String,
        duration_seconds: f64,
    },
    ModelReuse {
        session_id: String,
        window_index: usize,
        provider: String,
        model_id: String,
    },
    WindowEnd {
        session_id: String,
        window_index: usize,
        start_seconds: f64,
        end_seconds: f64,
        duration_seconds: f64,
    },
    Completed {
        session_id: String,
        processed_audio_seconds: f64,
        window_count: usize,
        final_segment_count: u64,
        duration_seconds: f64,
    },
    Failure {
        session_id: String,
        window_index: Option<usize>,
        message: String,
        duration_seconds: f64,
    },
    Cancelled {
        session_id: String,
        next_window_index: usize,
        processed_audio_seconds: f64,
        final_segment_count: u64,
        duration_seconds: f64,
    },
}

pub trait LiveTranscriptionProgressObserver {
    fn observe(&mut self, event: LiveTranscriptionProgressEvent);
}

#[derive(Debug, Default)]
pub struct NoopLiveTranscriptionProgressObserver;

impl LiveTranscriptionProgressObserver for NoopLiveTranscriptionProgressObserver {
    fn observe(&mut self, _event: LiveTranscriptionProgressEvent) {}
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{message}")]
pub struct LiveWindowProcessingError {
    message: String,
}

impl LiveWindowProcessingError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<LiveWindowingError> for LiveWindowProcessingError {
    fn from(error: LiveWindowingError) -> Self {
        Self::new(error.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LivePcmWindow {
    pub window_index: usize,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub latest_ingested_audio_seconds: f64,
    pub samples: Vec<f32>,
}

pub trait LivePcmWindowProcessor {
    fn process_window(
        &mut self,
        window: LivePcmWindow,
        progress: &mut dyn LiveTranscriptionProgressObserver,
    ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowProcessingError>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct LivePcmIngestionReport {
    pub processed_audio_seconds: f64,
    pub processed_sample_count: usize,
    pub window_count: usize,
    pub events: Vec<LiveTranscriptEvent>,
}

impl LivePcmIngestionReport {
    pub fn failed(&self) -> bool {
        self.events.iter().any(|event| {
            matches!(
                event,
                LiveTranscriptEvent::Error(_)
                    | LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                        reason: LiveSessionEndReason::Error,
                        ..
                    })
            )
        })
    }
}

#[derive(Debug)]
pub struct LivePcmIngestionSession {
    session_id: String,
    windows: BoundedPcmWindowSession,
    next_sequence: u64,
    final_segment_count: u64,
    failed: bool,
    active_window_index: Option<usize>,
}

struct LiveIngestionContext<'a> {
    processor: &'a mut dyn LivePcmWindowProcessor,
    events: &'a mut Vec<LiveTranscriptEvent>,
    sink: &'a mut dyn FnMut(&LiveTranscriptEvent) -> Result<(), LiveWindowProcessingError>,
    progress: &'a mut dyn LiveTranscriptionProgressObserver,
    cancellation: &'a CancellationHandle,
    started: Instant,
}

struct LiveProductWindowState<'a> {
    session_id: &'a str,
    next_sequence: &'a mut u64,
    final_segment_count: &'a mut u64,
    failed: &'a mut bool,
    active_window_index: &'a mut Option<usize>,
}

impl LivePcmIngestionSession {
    pub fn new(
        session_id: impl Into<String>,
        config: LiveWindowingConfig,
    ) -> Result<Self, LiveWindowingError> {
        validate_config(config)?;
        let windows =
            BoundedPcmWindowSession::new(bounded_pcm_window_config(config)?).map_err(|error| {
                LiveWindowingError::InvalidBoundedWindowConfig {
                    message: error.to_string(),
                }
            })?;
        Ok(Self {
            session_id: session_id.into(),
            windows,
            next_sequence: 1,
            final_segment_count: 0,
            failed: false,
            active_window_index: None,
        })
    }

    pub fn ingest_reader(
        &mut self,
        reader: &mut dyn Read,
        processor: &mut dyn LivePcmWindowProcessor,
    ) -> LivePcmIngestionReport {
        let cancellation = CancellationHandle::new();
        let mut progress = NoopLiveTranscriptionProgressObserver;
        self.ingest_reader_with_control(
            reader,
            processor,
            &mut |_| Ok(()),
            &mut progress,
            &cancellation,
        )
    }

    pub fn ingest_reader_with_event_sink(
        &mut self,
        reader: &mut dyn Read,
        processor: &mut dyn LivePcmWindowProcessor,
        sink: &mut dyn FnMut(&LiveTranscriptEvent) -> Result<(), LiveWindowProcessingError>,
    ) -> LivePcmIngestionReport {
        let cancellation = CancellationHandle::new();
        let mut progress = NoopLiveTranscriptionProgressObserver;
        self.ingest_reader_with_control(reader, processor, sink, &mut progress, &cancellation)
    }

    pub fn ingest_reader_with_observer(
        &mut self,
        reader: &mut dyn Read,
        processor: &mut dyn LivePcmWindowProcessor,
        sink: &mut dyn FnMut(&LiveTranscriptEvent) -> Result<(), LiveWindowProcessingError>,
        progress: &mut dyn LiveTranscriptionProgressObserver,
    ) -> LivePcmIngestionReport {
        let cancellation = CancellationHandle::new();
        self.ingest_reader_with_control(reader, processor, sink, progress, &cancellation)
    }

    pub fn ingest_reader_with_control(
        &mut self,
        reader: &mut dyn Read,
        processor: &mut dyn LivePcmWindowProcessor,
        sink: &mut dyn FnMut(&LiveTranscriptEvent) -> Result<(), LiveWindowProcessingError>,
        progress: &mut dyn LiveTranscriptionProgressObserver,
        cancellation: &CancellationHandle,
    ) -> LivePcmIngestionReport {
        let started = Instant::now();
        progress.observe(LiveTranscriptionProgressEvent::SessionStart {
            session_id: self.session_id.clone(),
        });

        let mut events = Vec::new();
        if cancellation.is_cancelled() {
            self.finish_cancelled(&mut events, sink, progress, started);
            return self.report(events);
        }

        let mut pending_bytes = Vec::new();
        let mut read_buffer = [0_u8; 8192];
        loop {
            if cancellation.is_cancelled() {
                break;
            }
            match reader.read(&mut read_buffer) {
                Ok(0) => {
                    if !pending_bytes.is_empty() {
                        self.emit_error(
                            &mut events,
                            sink,
                            progress,
                            started,
                            format!(
                                "truncated f32le PCM frame: {} trailing byte(s)",
                                pending_bytes.len()
                            ),
                        );
                    }
                    break;
                }
                Ok(read_len) => {
                    let mut bytes = Vec::with_capacity(pending_bytes.len() + read_len);
                    bytes.extend_from_slice(&pending_bytes);
                    bytes.extend_from_slice(&read_buffer[..read_len]);
                    let complete_len = bytes.len() - (bytes.len() % 4);
                    pending_bytes.clear();
                    pending_bytes.extend_from_slice(&bytes[complete_len..]);
                    let samples = decode_f32le_samples(&bytes[..complete_len]);
                    let ingest_result = {
                        let mut context = LiveIngestionContext {
                            processor,
                            events: &mut events,
                            sink,
                            progress,
                            cancellation,
                            started,
                        };
                        self.ingest_samples(&samples, &mut context)
                    };
                    if let Err(message) = ingest_result {
                        if cancellation.is_cancelled() {
                            break;
                        }
                        self.emit_error(&mut events, sink, progress, started, message);
                        break;
                    }
                }
                Err(error) => {
                    self.emit_error(
                        &mut events,
                        sink,
                        progress,
                        started,
                        format!("live PCM reader failed: {error}"),
                    );
                    break;
                }
            }
        }

        if cancellation.is_cancelled() && !self.failed {
            self.finish_cancelled(&mut events, sink, progress, started);
        } else {
            self.finish(&mut events, sink, progress, started);
        }
        self.report(events)
    }

    fn ingest_samples(
        &mut self,
        samples: &[f32],
        context: &mut LiveIngestionContext<'_>,
    ) -> Result<(), String> {
        let latest_ingested_audio_seconds = (self.windows.stats().samples_ingested
            + samples.len() as u64) as f64
            / LIVE_PCM_SAMPLE_RATE as f64;
        let Self {
            session_id,
            windows,
            next_sequence,
            final_segment_count,
            failed,
            active_window_index,
            ..
        } = self;
        let hop_samples = windows.config().hop_samples;
        let mut state = LiveProductWindowState {
            session_id,
            next_sequence,
            final_segment_count,
            failed,
            active_window_index,
        };
        let cancellation = context.cancellation;
        let mut callback_error = None;
        let mut product_stopped = false;
        let mut process = |window| match process_live_pcm_window(
            hop_samples,
            window,
            latest_ingested_audio_seconds,
            &mut state,
            context,
        ) {
            Ok(()) if *state.failed => {
                product_stopped = true;
                Err(media_core::DetectError::InvalidArgument(
                    "Native WhisperX live session stopped after a transcript error".to_string(),
                ))
            }
            Ok(()) => Ok(()),
            Err(message) => {
                callback_error = Some(message.clone());
                Err(media_core::DetectError::InvalidArgument(message))
            }
        };
        let result = windows.ingest(samples, &mut process, &|| cancellation.is_cancelled());
        finish_bounded_window_call(result, callback_error, product_stopped, cancellation)
    }

    fn emit_error(
        &mut self,
        events: &mut Vec<LiveTranscriptEvent>,
        sink: &mut dyn FnMut(&LiveTranscriptEvent) -> Result<(), LiveWindowProcessingError>,
        progress: &mut dyn LiveTranscriptionProgressObserver,
        started: Instant,
        message: String,
    ) {
        self.failed = true;
        progress.observe(LiveTranscriptionProgressEvent::Failure {
            session_id: self.session_id.clone(),
            window_index: self.active_window_index,
            message: message.clone(),
            duration_seconds: started.elapsed().as_secs_f64(),
        });
        let sequence = self.next_sequence();
        self.push_event(
            events,
            sink,
            LiveTranscriptEvent::Error(LiveTranscriptError {
                session_id: self.session_id.clone(),
                sequence,
                message,
                recoverable: false,
            }),
        );
    }

    fn finish_cancelled(
        &mut self,
        events: &mut Vec<LiveTranscriptEvent>,
        sink: &mut dyn FnMut(&LiveTranscriptEvent) -> Result<(), LiveWindowProcessingError>,
        progress: &mut dyn LiveTranscriptionProgressObserver,
        started: Instant,
    ) {
        progress.observe(LiveTranscriptionProgressEvent::Cancelled {
            session_id: self.session_id.clone(),
            next_window_index: self.window_count(),
            processed_audio_seconds: self.processed_audio_seconds(),
            final_segment_count: self.final_segment_count,
            duration_seconds: started.elapsed().as_secs_f64(),
        });
        self.push_session_end(events, sink, LiveSessionEndReason::Cancelled);
    }

    fn finish(
        &mut self,
        events: &mut Vec<LiveTranscriptEvent>,
        sink: &mut dyn FnMut(&LiveTranscriptEvent) -> Result<(), LiveWindowProcessingError>,
        progress: &mut dyn LiveTranscriptionProgressObserver,
        started: Instant,
    ) {
        let reason = if self.failed {
            LiveSessionEndReason::Error
        } else {
            progress.observe(LiveTranscriptionProgressEvent::Completed {
                session_id: self.session_id.clone(),
                processed_audio_seconds: self.processed_audio_seconds(),
                window_count: self.window_count(),
                final_segment_count: self.final_segment_count,
                duration_seconds: started.elapsed().as_secs_f64(),
            });
            LiveSessionEndReason::Completed
        };
        self.push_session_end(events, sink, reason);
    }

    fn push_session_end(
        &mut self,
        events: &mut Vec<LiveTranscriptEvent>,
        sink: &mut dyn FnMut(&LiveTranscriptEvent) -> Result<(), LiveWindowProcessingError>,
        reason: LiveSessionEndReason,
    ) {
        let sequence = self.next_sequence();
        self.push_event(
            events,
            sink,
            LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                session_id: self.session_id.clone(),
                sequence,
                reason,
                processed_audio_seconds: self.processed_audio_seconds(),
                final_segment_count: self.final_segment_count,
            }),
        );
    }

    fn push_event(
        &mut self,
        events: &mut Vec<LiveTranscriptEvent>,
        sink: &mut dyn FnMut(&LiveTranscriptEvent) -> Result<(), LiveWindowProcessingError>,
        event: LiveTranscriptEvent,
    ) {
        let _ = sink(&event);
        events.push(event);
    }

    fn report(&self, events: Vec<LiveTranscriptEvent>) -> LivePcmIngestionReport {
        LivePcmIngestionReport {
            processed_audio_seconds: self.processed_audio_seconds(),
            processed_sample_count: self.windows.stats().samples_ingested as usize,
            window_count: self.window_count(),
            events,
        }
    }

    fn next_sequence(&mut self) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        sequence
    }

    fn processed_audio_seconds(&self) -> f64 {
        self.windows.stats().input_duration_seconds
    }

    fn window_count(&self) -> usize {
        let processed = self.windows.stats().windows_processed as usize;
        if self.failed && self.active_window_index.is_some() {
            processed.saturating_sub(1)
        } else {
            processed
        }
    }
}

fn seconds_to_sample_index(seconds: f64) -> usize {
    (seconds * LIVE_PCM_SAMPLE_RATE as f64).round() as usize
}

fn bounded_pcm_window_config(
    config: LiveWindowingConfig,
) -> Result<BoundedPcmWindowConfig, LiveWindowingError> {
    if config.hop_seconds > config.window_seconds {
        return Err(LiveWindowingError::HopExceedsWindow);
    }
    let window_samples = seconds_to_sample_index(config.window_seconds);
    let hop_samples = seconds_to_sample_index(config.hop_seconds);
    if window_samples == 0 {
        return Err(LiveWindowingError::InvalidPositiveSeconds {
            field: "window_seconds",
        });
    }
    if hop_samples == 0 {
        return Err(LiveWindowingError::InvalidPositiveSeconds {
            field: "hop_seconds",
        });
    }
    BoundedPcmWindowConfig::new(
        LIVE_PCM_SAMPLE_RATE,
        window_samples,
        hop_samples,
        window_samples,
    )
    .map_err(|error| LiveWindowingError::InvalidBoundedWindowConfig {
        message: error.to_string(),
    })
}

fn decode_f32le_samples(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|sample_bytes| {
            f32::from_le_bytes([
                sample_bytes[0],
                sample_bytes[1],
                sample_bytes[2],
                sample_bytes[3],
            ])
        })
        .collect()
}

fn process_live_pcm_window(
    hop_samples: usize,
    window: UpstreamPcmWindow,
    latest_ingested_audio_seconds: f64,
    state: &mut LiveProductWindowState<'_>,
    context: &mut LiveIngestionContext<'_>,
) -> Result<(), String> {
    let window_index = (window.start_sample / hop_samples as u64) as usize;
    let start_seconds = window.start_sample as f64 / LIVE_PCM_SAMPLE_RATE as f64;
    let end_seconds = start_seconds + window.duration_seconds(LIVE_PCM_SAMPLE_RATE);
    let window_started = Instant::now();
    context
        .progress
        .observe(LiveTranscriptionProgressEvent::WindowStart {
            session_id: state.session_id.to_string(),
            window_index,
            start_seconds,
            end_seconds,
        });
    *state.active_window_index = Some(window_index);
    let window_events = context
        .processor
        .process_window(
            LivePcmWindow {
                window_index,
                start_seconds,
                end_seconds,
                latest_ingested_audio_seconds,
                samples: window.samples,
            },
            context.progress,
        )
        .map_err(|error| format!("live PCM window processing failed: {error}"))?;
    let cancelled_during_window = context.cancellation.is_cancelled();
    *state.final_segment_count += window_events
        .iter()
        .filter(|event| matches!(event, LiveTranscriptEvent::Final(_)))
        .count() as u64;
    for mut event in window_events {
        if cancelled_during_window && matches!(event, LiveTranscriptEvent::Partial(_)) {
            continue;
        }
        let error_message = match &event {
            LiveTranscriptEvent::Error(error) => Some(error.message.clone()),
            _ => None,
        };
        set_live_event_sequence(&mut event, take_next_sequence(state.next_sequence));
        if let Some(message) = error_message {
            *state.failed = true;
            context
                .progress
                .observe(LiveTranscriptionProgressEvent::Failure {
                    session_id: state.session_id.to_string(),
                    window_index: *state.active_window_index,
                    message,
                    duration_seconds: context.started.elapsed().as_secs_f64(),
                });
        }
        let _ = (context.sink)(&event);
        context.events.push(event);
        if *state.failed {
            break;
        }
    }
    if *state.failed {
        return Ok(());
    }
    context
        .progress
        .observe(LiveTranscriptionProgressEvent::WindowEnd {
            session_id: state.session_id.to_string(),
            window_index,
            start_seconds,
            end_seconds,
            duration_seconds: window_started.elapsed().as_secs_f64(),
        });
    *state.active_window_index = None;
    Ok(())
}

fn finish_bounded_window_call(
    result: media_core::Result<()>,
    callback_error: Option<String>,
    product_stopped: bool,
    cancellation: &CancellationHandle,
) -> Result<(), String> {
    if let Some(message) = callback_error {
        return Err(message);
    }
    if product_stopped || cancellation.is_cancelled() {
        return Ok(());
    }
    result.map_err(map_bounded_window_error)
}

fn map_bounded_window_error(error: media_core::DetectError) -> String {
    match error {
        media_core::DetectError::InvalidArgument(message)
            if message == "PCM samples must be finite" =>
        {
            "non-finite f32le PCM sample".to_string()
        }
        other => format!("live PCM ingestion failed: {other}"),
    }
}

fn take_next_sequence(next_sequence: &mut u64) -> u64 {
    let sequence = *next_sequence;
    *next_sequence += 1;
    sequence
}

fn set_live_event_sequence(event: &mut LiveTranscriptEvent, sequence: u64) {
    match event {
        LiveTranscriptEvent::SessionStarted(event) => event.sequence = sequence,
        LiveTranscriptEvent::Partial(event) => event.sequence = sequence,
        LiveTranscriptEvent::Final(event) => event.sequence = sequence,
        LiveTranscriptEvent::Error(event) => event.sequence = sequence,
        LiveTranscriptEvent::SessionEnded(event) => event.sequence = sequence,
    }
}

#[derive(Debug, Clone)]
pub struct LiveWindowState {
    config: LiveWindowingConfig,
    next_sequence: u64,
    final_segment_count: u64,
    pending_segments: Vec<PendingLiveSegment>,
    finalized_segments: Vec<FinalizedLiveSegmentKey>,
    failed: bool,
}

impl LiveWindowState {
    pub fn new(config: LiveWindowingConfig) -> Result<Self, LiveWindowingError> {
        validate_config(config)?;

        Ok(Self {
            config,
            next_sequence: 1,
            final_segment_count: 0,
            pending_segments: Vec::new(),
            finalized_segments: Vec::new(),
            failed: false,
        })
    }

    pub fn observe_window(
        &mut self,
        observation: LiveWindowTranscriptObservation,
    ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowingError> {
        let buffer_lag_seconds =
            observation.latest_ingested_audio_seconds - observation.window_end_seconds;
        if buffer_lag_seconds > self.config.max_buffer_lag_seconds {
            self.failed = true;
            return Ok(vec![LiveTranscriptEvent::Error(LiveTranscriptError {
                session_id: observation.session_id,
                sequence: self.next_sequence(),
                message: format!(
                    "processing fell behind live input by {buffer_lag_seconds:.3} seconds"
                ),
                recoverable: false,
            })]);
        }

        let stable_segments = self.mark_stable_segments(&observation);
        let sequence = self.next_sequence();
        let partial_segments = observation
            .segments
            .iter()
            .map(|segment| LivePartialSegment {
                start_seconds: segment.start_seconds,
                end_seconds: segment.end_seconds,
                text: segment.text.clone(),
                language: segment.language.clone(),
            })
            .collect();
        let mut events = vec![LiveTranscriptEvent::Partial(LivePartialTranscript {
            session_id: observation.session_id.clone(),
            sequence,
            window_start_seconds: observation.window_start_seconds,
            window_end_seconds: observation.window_end_seconds,
            window_start_at_utc: observation.window_start_at_utc.clone(),
            window_end_at_utc: observation.window_end_at_utc.clone(),
            text: observation
                .segments
                .iter()
                .map(|segment| segment.text.trim())
                .filter(|text| !text.is_empty())
                .collect::<Vec<_>>()
                .join(" "),
            segments: partial_segments,
        })];

        for stable_segment in stable_segments {
            if stable_segment.end_seconds
                <= observation.window_end_seconds - self.config.finalize_lag_seconds
            {
                events.push(LiveTranscriptEvent::Final(
                    self.finalize_segment(observation.session_id.clone(), stable_segment),
                ));
            }
        }

        self.pending_segments
            .retain(|segment| !(segment.stable && segment.finalized));
        self.add_pending_segments(observation);

        Ok(events)
    }

    pub fn final_segment_count(&self) -> u64 {
        self.final_segment_count
    }

    pub fn has_failed(&self) -> bool {
        self.failed
    }

    fn next_sequence(&mut self) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        sequence
    }

    fn mark_stable_segments(
        &mut self,
        observation: &LiveWindowTranscriptObservation,
    ) -> Vec<PendingLiveSegment> {
        let mut stable_segments = Vec::new();
        let finalized_segments = &self.finalized_segments;
        let stability_tolerance_seconds = self.config.stability_tolerance_seconds;

        for pending in &mut self.pending_segments {
            if pending.finalized
                || segment_matches_finalized(
                    finalized_segments,
                    stability_tolerance_seconds,
                    pending.start_seconds,
                    pending.end_seconds,
                    &pending.normalized_text,
                )
                || !windows_overlap(
                    pending.window_start_seconds,
                    pending.window_end_seconds,
                    observation.window_start_seconds,
                    observation.window_end_seconds,
                )
            {
                continue;
            }

            if observation.segments.iter().any(|candidate| {
                normalized_text(&candidate.text) == pending.normalized_text
                    && seconds_within_tolerance(
                        candidate.start_seconds,
                        pending.start_seconds,
                        self.config.stability_tolerance_seconds,
                    )
                    && seconds_within_tolerance(
                        candidate.end_seconds,
                        pending.end_seconds,
                        self.config.stability_tolerance_seconds,
                    )
            }) {
                pending.stable = true;
                stable_segments.push(pending.clone());
            }
        }

        stable_segments
    }

    fn finalize_segment(
        &mut self,
        session_id: String,
        mut stable_segment: PendingLiveSegment,
    ) -> LiveFinalTranscriptSegment {
        self.final_segment_count += 1;
        stable_segment.finalized = true;
        self.finalized_segments.push(FinalizedLiveSegmentKey {
            start_seconds: stable_segment.start_seconds,
            end_seconds: stable_segment.end_seconds,
            normalized_text: stable_segment.normalized_text.clone(),
        });

        if let Some(pending) = self
            .pending_segments
            .iter_mut()
            .find(|pending| pending.id == stable_segment.id)
        {
            pending.finalized = true;
        }

        LiveFinalTranscriptSegment {
            session_id,
            sequence: self.next_sequence(),
            segment_id: format!("seg-{segment_id:06}", segment_id = self.final_segment_count),
            start_seconds: stable_segment.start_seconds,
            end_seconds: stable_segment.end_seconds,
            start_at_utc: stable_segment.start_at_utc,
            end_at_utc: stable_segment.end_at_utc,
            text: stable_segment.normalized_text,
            language: stable_segment.language,
        }
    }

    fn add_pending_segments(&mut self, observation: LiveWindowTranscriptObservation) {
        for segment in observation.segments {
            let normalized_text = normalized_text(&segment.text);
            if normalized_text.is_empty()
                || self.segment_matches_finalized(
                    segment.start_seconds,
                    segment.end_seconds,
                    &normalized_text,
                )
            {
                continue;
            }

            self.pending_segments.push(PendingLiveSegment {
                id: format!(
                    "{:.3}:{:.3}:{}",
                    segment.start_seconds, segment.end_seconds, normalized_text
                ),
                window_start_seconds: observation.window_start_seconds,
                window_end_seconds: observation.window_end_seconds,
                start_seconds: segment.start_seconds,
                end_seconds: segment.end_seconds,
                start_at_utc: segment.start_at_utc,
                end_at_utc: segment.end_at_utc,
                normalized_text,
                language: segment.language,
                stable: false,
                finalized: false,
            });
        }
    }

    fn segment_matches_finalized(
        &self,
        start_seconds: f64,
        end_seconds: f64,
        normalized_text: &str,
    ) -> bool {
        segment_matches_finalized(
            &self.finalized_segments,
            self.config.stability_tolerance_seconds,
            start_seconds,
            end_seconds,
            normalized_text,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PendingLiveSegment {
    id: String,
    window_start_seconds: f64,
    window_end_seconds: f64,
    start_seconds: f64,
    end_seconds: f64,
    start_at_utc: String,
    end_at_utc: String,
    normalized_text: String,
    language: Option<String>,
    stable: bool,
    finalized: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct FinalizedLiveSegmentKey {
    start_seconds: f64,
    end_seconds: f64,
    normalized_text: String,
}

fn segment_matches_finalized(
    finalized_segments: &[FinalizedLiveSegmentKey],
    tolerance_seconds: f64,
    start_seconds: f64,
    end_seconds: f64,
    normalized_text: &str,
) -> bool {
    finalized_segments.iter().any(|finalized| {
        finalized.normalized_text == normalized_text
            && seconds_within_tolerance(finalized.start_seconds, start_seconds, tolerance_seconds)
            && seconds_within_tolerance(finalized.end_seconds, end_seconds, tolerance_seconds)
    })
}

fn validate_config(config: LiveWindowingConfig) -> Result<(), LiveWindowingError> {
    validate_positive_seconds("window_seconds", config.window_seconds)?;
    validate_positive_seconds("hop_seconds", config.hop_seconds)?;
    validate_positive_seconds("finalize_lag_seconds", config.finalize_lag_seconds)?;
    validate_positive_seconds("max_buffer_lag_seconds", config.max_buffer_lag_seconds)?;
    validate_positive_seconds(
        "stability_tolerance_seconds",
        config.stability_tolerance_seconds,
    )
}

fn validate_positive_seconds(field: &'static str, value: f64) -> Result<(), LiveWindowingError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(LiveWindowingError::InvalidPositiveSeconds { field })
    }
}

fn normalized_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn seconds_within_tolerance(left: f64, right: f64, tolerance: f64) -> bool {
    (left - right).abs() <= tolerance
}

fn windows_overlap(first_start: f64, first_end: f64, second_start: f64, second_end: f64) -> bool {
    first_start < second_end && second_start < first_end
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub enum LiveTranscriptEvent {
    SessionStarted(LiveSessionStarted),
    Partial(LivePartialTranscript),
    Final(LiveFinalTranscriptSegment),
    Error(LiveTranscriptError),
    SessionEnded(LiveSessionEnded),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveSessionStarted {
    pub session_id: String,
    pub sequence: u64,
    pub source: String,
    pub ingest_started_at_utc: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub model_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LivePartialTranscript {
    pub session_id: String,
    pub sequence: u64,
    pub window_start_seconds: f64,
    pub window_end_seconds: f64,
    pub window_start_at_utc: String,
    pub window_end_at_utc: String,
    pub text: String,
    pub segments: Vec<LivePartialSegment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LivePartialSegment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveFinalTranscriptSegment {
    pub session_id: String,
    pub sequence: u64,
    pub segment_id: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub start_at_utc: String,
    pub end_at_utc: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTranscriptError {
    pub session_id: String,
    pub sequence: u64,
    pub message: String,
    pub recoverable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveSessionEnded {
    pub session_id: String,
    pub sequence: u64,
    pub reason: LiveSessionEndReason,
    pub processed_audio_seconds: f64,
    pub final_segment_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LiveSessionEndReason {
    Completed,
    Error,
    Cancelled,
}

pub fn live_transcript_events_to_jsonl(
    events: &[LiveTranscriptEvent],
) -> Result<String, serde_json::Error> {
    let mut jsonl = String::new();
    for event in events {
        jsonl.push_str(&serde_json::to_string(event)?);
        jsonl.push('\n');
    }
    Ok(jsonl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn near_live_window_state_emits_partial_before_any_final() {
        let mut state = LiveWindowState::new(LiveWindowingConfig {
            window_seconds: 5.0,
            hop_seconds: 2.5,
            finalize_lag_seconds: 5.0,
            max_buffer_lag_seconds: 30.0,
            stability_tolerance_seconds: 0.4,
        })
        .expect("valid live config");

        let events = state
            .observe_window(LiveWindowTranscriptObservation {
                session_id: "session-1".to_string(),
                window_start_seconds: 0.0,
                window_end_seconds: 5.0,
                window_start_at_utc: "2026-06-22T15:30:00Z".to_string(),
                window_end_at_utc: "2026-06-22T15:30:05Z".to_string(),
                latest_ingested_audio_seconds: 5.0,
                segments: vec![LiveAsrSegmentCandidate {
                    start_seconds: 0.4,
                    end_seconds: 1.8,
                    start_at_utc: "2026-06-22T15:30:00.400Z".to_string(),
                    end_at_utc: "2026-06-22T15:30:01.800Z".to_string(),
                    text: "hello wor".to_string(),
                    language: Some("en".to_string()),
                }],
            })
            .expect("window is accepted");

        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], LiveTranscriptEvent::Partial(_)));
        assert_eq!(state.final_segment_count(), 0);
    }

    #[test]
    fn overlapping_matching_window_promotes_stable_segment_after_finalize_lag() {
        let mut state = LiveWindowState::new(LiveWindowingConfig::default()).expect("valid config");

        state
            .observe_window(observation(
                "session-1",
                0.0,
                5.0,
                5.0,
                vec![candidate(0.4, 1.8, " hello   world ")],
            ))
            .expect("first window accepted");

        let events = state
            .observe_window(observation(
                "session-1",
                2.5,
                7.5,
                7.5,
                vec![candidate(0.45, 1.75, "hello world")],
            ))
            .expect("second window accepted");

        assert_eq!(events.len(), 2);
        assert!(matches!(events[0], LiveTranscriptEvent::Partial(_)));
        assert_eq!(
            events[1],
            LiveTranscriptEvent::Final(LiveFinalTranscriptSegment {
                session_id: "session-1".to_string(),
                sequence: 3,
                segment_id: "seg-000001".to_string(),
                start_seconds: 0.4,
                end_seconds: 1.8,
                start_at_utc: "2026-06-22T15:30:00.400Z".to_string(),
                end_at_utc: "2026-06-22T15:30:01.800Z".to_string(),
                text: "hello world".to_string(),
                language: Some("en".to_string()),
            })
        );
        assert_eq!(state.final_segment_count(), 1);
    }

    #[test]
    fn finalized_segments_are_not_revised_by_later_windows() {
        let mut state = LiveWindowState::new(LiveWindowingConfig::default()).expect("valid config");

        state
            .observe_window(observation(
                "session-1",
                0.0,
                5.0,
                5.0,
                vec![candidate(0.4, 1.8, "hello world")],
            ))
            .expect("first window accepted");
        let finalizing_events = state
            .observe_window(observation(
                "session-1",
                2.5,
                7.5,
                7.5,
                vec![candidate(0.45, 1.75, "hello world")],
            ))
            .expect("second window accepted");
        let revised_events = state
            .observe_window(observation(
                "session-1",
                5.0,
                10.0,
                10.0,
                vec![candidate(0.42, 1.78, "changed text")],
            ))
            .expect("later window accepted");
        let matching_events_after_final = state
            .observe_window(observation(
                "session-1",
                7.4,
                12.4,
                12.4,
                vec![candidate(0.44, 1.76, "hello world")],
            ))
            .expect("matching later window accepted");

        let final_events = finalizing_events
            .iter()
            .chain(revised_events.iter())
            .chain(matching_events_after_final.iter())
            .filter_map(|event| match event {
                LiveTranscriptEvent::Final(final_segment) => Some(final_segment),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(final_events.len(), 1);
        assert_eq!(final_events[0].text, "hello world");
        assert_eq!(state.final_segment_count(), 1);
    }

    #[test]
    fn lag_failure_emits_error_instead_of_dropping_speech() {
        let mut state = LiveWindowState::new(LiveWindowingConfig {
            window_seconds: 5.0,
            hop_seconds: 2.5,
            finalize_lag_seconds: 5.0,
            max_buffer_lag_seconds: 3.0,
            stability_tolerance_seconds: 0.4,
        })
        .expect("valid config");

        let events = state
            .observe_window(observation(
                "session-1",
                0.0,
                5.0,
                9.1,
                vec![candidate(0.4, 1.8, "hello world")],
            ))
            .expect("lag is reported as an event");

        assert_eq!(
            events,
            vec![LiveTranscriptEvent::Error(LiveTranscriptError {
                session_id: "session-1".to_string(),
                sequence: 1,
                message: "processing fell behind live input by 4.100 seconds".to_string(),
                recoverable: false,
            })]
        );
        assert!(state.has_failed());
    }

    fn observation(
        session_id: &str,
        window_start_seconds: f64,
        window_end_seconds: f64,
        latest_ingested_audio_seconds: f64,
        segments: Vec<LiveAsrSegmentCandidate>,
    ) -> LiveWindowTranscriptObservation {
        LiveWindowTranscriptObservation {
            session_id: session_id.to_string(),
            window_start_seconds,
            window_end_seconds,
            window_start_at_utc: format!("2026-06-22T15:30:{window_start_seconds:02.0}Z"),
            window_end_at_utc: format!("2026-06-22T15:30:{window_end_seconds:02.0}Z"),
            latest_ingested_audio_seconds,
            segments,
        }
    }

    fn candidate(start_seconds: f64, end_seconds: f64, text: &str) -> LiveAsrSegmentCandidate {
        LiveAsrSegmentCandidate {
            start_seconds,
            end_seconds,
            start_at_utc: format!("2026-06-22T15:30:{start_seconds:06.3}Z"),
            end_at_utc: format!("2026-06-22T15:30:{end_seconds:06.3}Z"),
            text: text.to_string(),
            language: Some("en".to_string()),
        }
    }

    #[test]
    fn session_started_serializes_camel_case_event_contract() {
        let event = LiveTranscriptEvent::SessionStarted(LiveSessionStarted {
            session_id: "session-1".to_string(),
            sequence: 1,
            source: "rtsp://camera/live".to_string(),
            ingest_started_at_utc: "2026-06-22T15:30:00Z".to_string(),
            sample_rate: 16_000,
            channels: 1,
            model_id: "tiny.en".to_string(),
            language: Some("en".to_string()),
        });

        let json = serde_json::to_value(&event).expect("event serializes");

        assert_eq!(
            json,
            serde_json::json!({
                "event": "sessionStarted",
                "sessionId": "session-1",
                "sequence": 1,
                "source": "rtsp://camera/live",
                "ingestStartedAtUtc": "2026-06-22T15:30:00Z",
                "sampleRate": 16000,
                "channels": 1,
                "modelId": "tiny.en",
                "language": "en"
            })
        );
    }

    #[test]
    fn live_transcript_events_serialize_all_jsonl_shapes() {
        let events = vec![
            LiveTranscriptEvent::Partial(LivePartialTranscript {
                session_id: "session-1".to_string(),
                sequence: 2,
                window_start_seconds: 0.0,
                window_end_seconds: 5.0,
                window_start_at_utc: "2026-06-22T15:30:00Z".to_string(),
                window_end_at_utc: "2026-06-22T15:30:05Z".to_string(),
                text: "hello wor".to_string(),
                segments: vec![LivePartialSegment {
                    start_seconds: 0.4,
                    end_seconds: 1.8,
                    text: "hello wor".to_string(),
                    language: Some("en".to_string()),
                }],
            }),
            LiveTranscriptEvent::Final(LiveFinalTranscriptSegment {
                session_id: "session-1".to_string(),
                sequence: 3,
                segment_id: "seg-000001".to_string(),
                start_seconds: 0.4,
                end_seconds: 1.9,
                start_at_utc: "2026-06-22T15:30:00.400Z".to_string(),
                end_at_utc: "2026-06-22T15:30:01.900Z".to_string(),
                text: "hello world".to_string(),
                language: Some("en".to_string()),
            }),
            LiveTranscriptEvent::Error(LiveTranscriptError {
                session_id: "session-1".to_string(),
                sequence: 4,
                message: "processing fell behind live input".to_string(),
                recoverable: false,
            }),
            LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                session_id: "session-1".to_string(),
                sequence: 5,
                reason: LiveSessionEndReason::Error,
                processed_audio_seconds: 12.5,
                final_segment_count: 1,
            }),
        ];

        let jsonl = live_transcript_events_to_jsonl(&events).expect("events serialize");
        let lines = jsonl.lines().collect::<Vec<_>>();

        assert_eq!(lines.len(), 4);
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(lines[0]).expect("partial json"),
            serde_json::json!({
                "event": "partial",
                "sessionId": "session-1",
                "sequence": 2,
                "windowStartSeconds": 0.0,
                "windowEndSeconds": 5.0,
                "windowStartAtUtc": "2026-06-22T15:30:00Z",
                "windowEndAtUtc": "2026-06-22T15:30:05Z",
                "text": "hello wor",
                "segments": [{
                    "startSeconds": 0.4,
                    "endSeconds": 1.8,
                    "text": "hello wor",
                    "language": "en"
                }]
            })
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(lines[1]).expect("final json"),
            serde_json::json!({
                "event": "final",
                "sessionId": "session-1",
                "sequence": 3,
                "segmentId": "seg-000001",
                "startSeconds": 0.4,
                "endSeconds": 1.9,
                "startAtUtc": "2026-06-22T15:30:00.400Z",
                "endAtUtc": "2026-06-22T15:30:01.900Z",
                "text": "hello world",
                "language": "en"
            })
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(lines[2]).expect("error json"),
            serde_json::json!({
                "event": "error",
                "sessionId": "session-1",
                "sequence": 4,
                "message": "processing fell behind live input",
                "recoverable": false
            })
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(lines[3]).expect("ended json"),
            serde_json::json!({
                "event": "sessionEnded",
                "sessionId": "session-1",
                "sequence": 5,
                "reason": "error",
                "processedAudioSeconds": 12.5,
                "finalSegmentCount": 1
            })
        );
        assert!(jsonl.ends_with('\n'));
    }
}

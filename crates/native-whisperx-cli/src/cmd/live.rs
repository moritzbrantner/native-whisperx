//! Live Feed Transcription CLI surface and FFmpeg command planning.

use super::*;
use std::io::Read;

use native_whisperx::{
    LiveSessionEndReason, LiveSessionEnded, LiveTranscriptError, LiveTranscriptEvent, LiveWindow,
    LiveWindowingConfig,
};

const LIVE_PCM_SAMPLE_RATE: u32 = 16_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FfmpegCommandPlan {
    pub(crate) program: OsString,
    pub(crate) args: Vec<OsString>,
}

pub(crate) fn live_transcribe_command(args: LiveTranscribeArgs) -> anyhow::Result<()> {
    validate_live_transcribe_args(&args)?;
    let ffmpeg = plan_ffmpeg_command(&args);
    if args.print_ffmpeg_plan {
        println!("{}", ffmpeg.to_json()?);
        return Ok(());
    }
    anyhow::bail!(
        "live-transcribe execution is not implemented yet; this slice only plans the FFmpeg command"
    );
}

pub(crate) fn plan_ffmpeg_command(args: &LiveTranscribeArgs) -> FfmpegCommandPlan {
    let mut ffmpeg_args = Vec::new();
    ffmpeg_args.extend(args.ffmpeg_input_options.iter().map(OsString::from));
    ffmpeg_args.push(OsString::from("-i"));
    ffmpeg_args.push(OsString::from(&args.source));
    ffmpeg_args.push(OsString::from("-vn"));
    ffmpeg_args.extend(args.ffmpeg_output_options.iter().map(OsString::from));
    ffmpeg_args.extend([
        OsString::from("-ac"),
        OsString::from("1"),
        OsString::from("-ar"),
        OsString::from("16000"),
        OsString::from("-f"),
        OsString::from("f32le"),
        OsString::from("pipe:1"),
    ]);

    FfmpegCommandPlan {
        program: OsString::from(&args.ffmpeg_bin),
        args: ffmpeg_args,
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct LivePcmWindow {
    pub(crate) start_seconds: f64,
    pub(crate) end_seconds: f64,
    pub(crate) samples: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct LivePcmIngestionReport {
    pub(crate) processed_audio_seconds: f64,
    pub(crate) processed_sample_count: usize,
    pub(crate) window_count: usize,
    pub(crate) events: Vec<LiveTranscriptEvent>,
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) trait LivePcmWindowProcessor {
    fn process_window(&mut self, window: LivePcmWindow)
        -> anyhow::Result<Vec<LiveTranscriptEvent>>;
}

#[derive(Debug, Clone)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct LivePcmIngestionSession {
    session_id: String,
    config: LiveWindowingConfig,
    samples: Vec<f32>,
    next_window_start_seconds: f64,
    next_sequence: u64,
    window_count: usize,
    final_segment_count: u64,
    failed: bool,
}

impl LivePcmIngestionSession {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn new(
        session_id: impl Into<String>,
        config: LiveWindowingConfig,
    ) -> anyhow::Result<Self> {
        native_whisperx::LiveWindowPlanner::new(config)?;
        Ok(Self {
            session_id: session_id.into(),
            config,
            samples: Vec::new(),
            next_window_start_seconds: 0.0,
            next_sequence: 1,
            window_count: 0,
            final_segment_count: 0,
            failed: false,
        })
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn ingest_reader(
        &mut self,
        reader: &mut dyn Read,
        processor: &mut dyn LivePcmWindowProcessor,
    ) -> LivePcmIngestionReport {
        let mut events = Vec::new();
        let mut pending_bytes = Vec::new();
        let mut read_buffer = [0_u8; 8192];

        loop {
            match reader.read(&mut read_buffer) {
                Ok(0) => {
                    if !pending_bytes.is_empty() {
                        self.emit_error(
                            &mut events,
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

                    if let Err(message) =
                        self.ingest_pcm_bytes(&bytes[..complete_len], processor, &mut events)
                    {
                        self.emit_error(&mut events, message);
                        break;
                    }
                }
                Err(error) => {
                    self.emit_error(&mut events, format!("live PCM reader failed: {error}"));
                    break;
                }
            }
        }

        if !self.failed {
            if let Err(message) = self.process_ready_windows(processor, &mut events) {
                self.emit_error(&mut events, message);
            }
        }

        events.push(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
            session_id: self.session_id.clone(),
            sequence: self.next_sequence(),
            reason: if self.failed {
                LiveSessionEndReason::Error
            } else {
                LiveSessionEndReason::Completed
            },
            processed_audio_seconds: self.processed_audio_seconds(),
            final_segment_count: self.final_segment_count,
        }));

        LivePcmIngestionReport {
            processed_audio_seconds: self.processed_audio_seconds(),
            processed_sample_count: self.samples.len(),
            window_count: self.window_count,
            events,
        }
    }

    fn ingest_pcm_bytes(
        &mut self,
        bytes: &[u8],
        processor: &mut dyn LivePcmWindowProcessor,
        events: &mut Vec<LiveTranscriptEvent>,
    ) -> Result<(), String> {
        for sample_bytes in bytes.chunks_exact(4) {
            let sample = f32::from_le_bytes([
                sample_bytes[0],
                sample_bytes[1],
                sample_bytes[2],
                sample_bytes[3],
            ]);
            if !sample.is_finite() {
                return Err(format!(
                    "non-finite f32le PCM sample at feed sample {}",
                    self.samples.len()
                ));
            }
            self.samples.push(sample);
        }

        self.process_ready_windows(processor, events)
    }

    fn process_ready_windows(
        &mut self,
        processor: &mut dyn LivePcmWindowProcessor,
        events: &mut Vec<LiveTranscriptEvent>,
    ) -> Result<(), String> {
        while self.next_window_start_seconds + self.config.window_seconds
            <= self.processed_audio_seconds()
        {
            let window = LiveWindow {
                start_seconds: self.next_window_start_seconds,
                end_seconds: self.next_window_start_seconds + self.config.window_seconds,
            };
            let start_sample = seconds_to_sample_index(window.start_seconds);
            let end_sample = seconds_to_sample_index(window.end_seconds);
            let window_samples = self.samples[start_sample..end_sample].to_vec();
            let window_events = processor
                .process_window(LivePcmWindow {
                    start_seconds: window.start_seconds,
                    end_seconds: window.end_seconds,
                    samples: window_samples,
                })
                .map_err(|error| format!("live PCM window processing failed: {error}"))?;
            self.final_segment_count += window_events
                .iter()
                .filter(|event| matches!(event, LiveTranscriptEvent::Final(_)))
                .count() as u64;
            events.extend(window_events);
            self.window_count += 1;
            self.next_window_start_seconds += self.config.hop_seconds;
        }

        Ok(())
    }

    fn emit_error(&mut self, events: &mut Vec<LiveTranscriptEvent>, message: String) {
        self.failed = true;
        events.push(LiveTranscriptEvent::Error(LiveTranscriptError {
            session_id: self.session_id.clone(),
            sequence: self.next_sequence(),
            message,
            recoverable: false,
        }));
    }

    fn next_sequence(&mut self) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        sequence
    }

    fn processed_audio_seconds(&self) -> f64 {
        self.samples.len() as f64 / LIVE_PCM_SAMPLE_RATE as f64
    }
}

fn seconds_to_sample_index(seconds: f64) -> usize {
    (seconds * LIVE_PCM_SAMPLE_RATE as f64).round() as usize
}

impl FfmpegCommandPlan {
    fn to_json(&self) -> anyhow::Result<String> {
        let program = self.program.to_string_lossy();
        let args = self
            .args
            .iter()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        Ok(serde_json::to_string_pretty(&serde_json::json!({
            "program": program.as_ref(),
            "args": args,
        }))?)
    }
}

fn validate_live_transcribe_args(args: &LiveTranscribeArgs) -> anyhow::Result<()> {
    for (name, value) in [
        ("--window-seconds", args.window_seconds),
        ("--hop-seconds", args.hop_seconds),
        ("--finalize-lag-seconds", args.finalize_lag_seconds),
        ("--max-buffer-lag-seconds", args.max_buffer_lag_seconds),
    ] {
        if !value.is_finite() || value <= 0.0 {
            anyhow::bail!("{name} must be a positive finite number");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    use native_whisperx::{
        LiveAsrSegmentCandidate, LiveWindowState, LiveWindowTranscriptObservation,
    };

    #[test]
    fn ffmpeg_plan_places_input_options_before_source_and_output_options_before_pcm_settings() {
        let args = live_args_with_options(["-rtsp_transport", "tcp"], ["-hide_banner", "-nostats"]);

        let plan = plan_ffmpeg_command(&args);
        let argv = plan_args(&plan);

        assert_eq!(plan.program, OsString::from("ffmpeg"));
        assert_eq!(
            argv,
            [
                "-rtsp_transport",
                "tcp",
                "-i",
                "rtsp://example.test/live",
                "-vn",
                "-hide_banner",
                "-nostats",
                "-ac",
                "1",
                "-ar",
                "16000",
                "-f",
                "f32le",
                "pipe:1",
            ]
        );
    }

    #[test]
    fn ffmpeg_plan_defaults_to_audio_only_stdout_pcm() {
        let args = live_args_with_options([], []);

        let plan = plan_ffmpeg_command(&args);
        let argv = plan_args(&plan);

        assert_eq!(
            argv,
            [
                "-i",
                "rtsp://example.test/live",
                "-vn",
                "-ac",
                "1",
                "-ar",
                "16000",
                "-f",
                "f32le",
                "pipe:1",
            ]
        );
    }

    fn live_args_with_options<const I: usize, const O: usize>(
        input_options: [&str; I],
        output_options: [&str; O],
    ) -> LiveTranscribeArgs {
        LiveTranscribeArgs {
            source: "rtsp://example.test/live".to_string(),
            model: "small".to_string(),
            model_dir: None,
            model_cache_only: false,
            language: None,
            ffmpeg_bin: "ffmpeg".to_string(),
            ffmpeg_input_options: input_options.into_iter().map(ToString::to_string).collect(),
            ffmpeg_output_options: output_options
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            window_seconds: 5.0,
            hop_seconds: 2.5,
            finalize_lag_seconds: 5.0,
            max_buffer_lag_seconds: 30.0,
            print_ffmpeg_plan: false,
        }
    }

    fn plan_args(plan: &FfmpegCommandPlan) -> Vec<&str> {
        plan.args
            .iter()
            .map(|arg| arg.to_str().expect("test args are utf8"))
            .collect()
    }

    #[test]
    fn pcm_ingestion_consumes_little_endian_f32le_chunks_and_tracks_feed_seconds() {
        let mut reader = io::Cursor::new(pcm_bytes(
            (0..seconds_to_sample_index(7.5))
                .map(|index| index as f32 / 1000.0)
                .collect(),
        ));
        let mut processor = RecordingWindowProcessor::default();
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert_eq!(report.processed_sample_count, 120_000);
        assert_eq!(report.processed_audio_seconds, 7.5);
        assert_eq!(report.window_count, 2);
        assert_eq!(
            processor.windows,
            vec![
                WindowSummary {
                    start_seconds: 0.0,
                    end_seconds: 5.0,
                    sample_count: 80_000,
                    first_sample: 0.0,
                    last_sample: 79.999,
                },
                WindowSummary {
                    start_seconds: 2.5,
                    end_seconds: 7.5,
                    sample_count: 80_000,
                    first_sample: 40.0,
                    last_sample: 119.999,
                },
            ]
        );
        assert!(matches!(
            report.events.last(),
            Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                reason: LiveSessionEndReason::Completed,
                processed_audio_seconds: 7.5,
                ..
            }))
        ));
    }

    #[test]
    fn pcm_ingestion_handles_reads_split_across_sample_boundaries() {
        let bytes = pcm_bytes(vec![1.25; seconds_to_sample_index(5.0)]);
        let mut reader = ChunkedReader {
            bytes,
            offset: 0,
            max_chunk_len: 3,
        };
        let mut processor = RecordingWindowProcessor::default();
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert_eq!(report.processed_sample_count, 80_000);
        assert_eq!(report.window_count, 1);
        assert_eq!(processor.windows[0].first_sample, 1.25);
        assert_eq!(processor.windows[0].last_sample, 1.25);
        assert!(matches!(
            report.events.last(),
            Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                reason: LiveSessionEndReason::Completed,
                ..
            }))
        ));
    }

    #[test]
    fn pcm_ingestion_eof_flushes_eligible_final_events_before_session_end() {
        let mut reader = io::Cursor::new(pcm_bytes(vec![0.0; seconds_to_sample_index(7.5)]));
        let mut processor = StableWindowProcessor {
            state: LiveWindowState::new(LiveWindowingConfig {
                window_seconds: 5.0,
                hop_seconds: 2.5,
                finalize_lag_seconds: 5.0,
                max_buffer_lag_seconds: 30.0,
                stability_tolerance_seconds: 0.4,
            })
            .expect("valid window state"),
        };
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert!(report
            .events
            .iter()
            .any(|event| matches!(event, LiveTranscriptEvent::Final(_))));
        assert!(matches!(
            report.events.last(),
            Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                reason: LiveSessionEndReason::Completed,
                final_segment_count: 1,
                ..
            }))
        ));
    }

    #[test]
    fn pcm_ingestion_reports_truncated_sample_bytes() {
        let mut reader = io::Cursor::new(vec![0x00, 0x00, 0x80]);
        let mut processor = RecordingWindowProcessor::default();
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert_eq!(report.window_count, 0);
        assert!(matches!(
            &report.events[0],
            LiveTranscriptEvent::Error(LiveTranscriptError {
                message,
                recoverable: false,
                ..
            }) if message == "truncated f32le PCM frame: 3 trailing byte(s)"
        ));
        assert!(matches!(
            report.events.last(),
            Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                reason: LiveSessionEndReason::Error,
                ..
            }))
        ));
    }

    #[test]
    fn pcm_ingestion_reports_non_finite_samples_as_malformed_pcm() {
        let mut reader = io::Cursor::new(pcm_bytes(vec![0.0, f32::NAN]));
        let mut processor = RecordingWindowProcessor::default();
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert!(matches!(
            &report.events[0],
            LiveTranscriptEvent::Error(LiveTranscriptError {
                message,
                recoverable: false,
                ..
            }) if message == "non-finite f32le PCM sample at feed sample 1"
        ));
        assert_eq!(report.processed_sample_count, 1);
    }

    #[test]
    fn pcm_ingestion_propagates_reader_errors_as_error_events() {
        let mut reader = FailingReader { did_fail: false };
        let mut processor = RecordingWindowProcessor::default();
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert!(matches!(
            &report.events[0],
            LiveTranscriptEvent::Error(LiveTranscriptError {
                message,
                recoverable: false,
                ..
            }) if message == "live PCM reader failed: injected read failure"
        ));
        assert!(matches!(
            report.events.last(),
            Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                reason: LiveSessionEndReason::Error,
                ..
            }))
        ));
    }

    #[test]
    fn pcm_ingestion_propagates_window_processor_errors_as_error_events() {
        let mut reader = io::Cursor::new(pcm_bytes(vec![0.0; seconds_to_sample_index(5.0)]));
        let mut processor = FailingWindowProcessor;
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert!(matches!(
            &report.events[0],
            LiveTranscriptEvent::Error(LiveTranscriptError {
                message,
                recoverable: false,
                ..
            }) if message == "live PCM window processing failed: injected processor failure"
        ));
    }

    fn short_ingestion_session() -> LivePcmIngestionSession {
        LivePcmIngestionSession::new(
            "session-1",
            LiveWindowingConfig {
                window_seconds: 5.0,
                hop_seconds: 2.5,
                finalize_lag_seconds: 5.0,
                max_buffer_lag_seconds: 30.0,
                stability_tolerance_seconds: 0.4,
            },
        )
        .expect("valid ingestion session")
    }

    fn pcm_bytes(samples: Vec<f32>) -> Vec<u8> {
        samples
            .into_iter()
            .flat_map(f32::to_le_bytes)
            .collect::<Vec<_>>()
    }

    #[derive(Debug, Clone, PartialEq)]
    struct WindowSummary {
        start_seconds: f64,
        end_seconds: f64,
        sample_count: usize,
        first_sample: f32,
        last_sample: f32,
    }

    #[derive(Default)]
    struct RecordingWindowProcessor {
        windows: Vec<WindowSummary>,
    }

    impl LivePcmWindowProcessor for RecordingWindowProcessor {
        fn process_window(
            &mut self,
            window: LivePcmWindow,
        ) -> anyhow::Result<Vec<LiveTranscriptEvent>> {
            self.windows.push(WindowSummary {
                start_seconds: window.start_seconds,
                end_seconds: window.end_seconds,
                sample_count: window.samples.len(),
                first_sample: *window.samples.first().expect("window has samples"),
                last_sample: *window.samples.last().expect("window has samples"),
            });
            Ok(Vec::new())
        }
    }

    struct StableWindowProcessor {
        state: LiveWindowState,
    }

    impl LivePcmWindowProcessor for StableWindowProcessor {
        fn process_window(
            &mut self,
            window: LivePcmWindow,
        ) -> anyhow::Result<Vec<LiveTranscriptEvent>> {
            Ok(self.state.observe_window(LiveWindowTranscriptObservation {
                session_id: "session-1".to_string(),
                window_start_seconds: window.start_seconds,
                window_end_seconds: window.end_seconds,
                window_start_at_utc: format!("2026-06-22T15:30:{:02.0}Z", window.start_seconds),
                window_end_at_utc: format!("2026-06-22T15:30:{:02.0}Z", window.end_seconds),
                latest_ingested_audio_seconds: window.end_seconds,
                segments: vec![LiveAsrSegmentCandidate {
                    start_seconds: 0.4,
                    end_seconds: 1.8,
                    start_at_utc: "2026-06-22T15:30:00.400Z".to_string(),
                    end_at_utc: "2026-06-22T15:30:01.800Z".to_string(),
                    text: "hello world".to_string(),
                    language: Some("en".to_string()),
                }],
            })?)
        }
    }

    struct FailingReader {
        did_fail: bool,
    }

    struct ChunkedReader {
        bytes: Vec<u8>,
        offset: usize,
        max_chunk_len: usize,
    }

    impl Read for ChunkedReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.offset >= self.bytes.len() {
                return Ok(0);
            }

            let read_len = self
                .max_chunk_len
                .min(buf.len())
                .min(self.bytes.len() - self.offset);
            buf[..read_len].copy_from_slice(&self.bytes[self.offset..self.offset + read_len]);
            self.offset += read_len;
            Ok(read_len)
        }
    }

    impl Read for FailingReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            if self.did_fail {
                Ok(0)
            } else {
                self.did_fail = true;
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "injected read failure",
                ))
            }
        }
    }

    struct FailingWindowProcessor;

    impl LivePcmWindowProcessor for FailingWindowProcessor {
        fn process_window(
            &mut self,
            _window: LivePcmWindow,
        ) -> anyhow::Result<Vec<LiveTranscriptEvent>> {
            anyhow::bail!("injected processor failure")
        }
    }
}

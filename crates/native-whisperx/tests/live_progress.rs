use std::io::Cursor;

use native_whisperx::{
    CancellationHandle, LiveFinalTranscriptSegment, LivePartialTranscript, LivePcmIngestionSession,
    LivePcmWindow, LivePcmWindowProcessor, LiveSessionEndReason, LiveSessionEnded,
    LiveTranscriptError, LiveTranscriptEvent, LiveTranscriptionProgressEvent,
    LiveTranscriptionProgressObserver, LiveWindowProcessingError, LiveWindowingConfig,
};

#[test]
fn cancellation_before_first_near_live_window_prevents_it_from_starting() {
    let cancellation = CancellationHandle::new();
    cancellation.cancel();
    let mut session = session();
    let mut reader = Cursor::new(pcm_bytes(5.0));
    let mut processor = RecordingProcessor::default();
    let mut progress = RecordingProgress::default();

    let report = session.ingest_reader_with_control(
        &mut reader,
        &mut processor,
        &mut |_| Ok(()),
        &mut progress,
        &cancellation,
    );

    assert_eq!(processor.window_count, 0);
    assert!(matches!(
        report.events.last(),
        Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
            reason: LiveSessionEndReason::Cancelled,
            ..
        }))
    ));
    assert!(matches!(
        progress.events.as_slice(),
        [
            LiveTranscriptionProgressEvent::SessionStart { .. },
            LiveTranscriptionProgressEvent::Cancelled { .. }
        ]
    ));
}

#[test]
fn cancellation_between_near_live_windows_starts_no_later_window() {
    let cancellation = CancellationHandle::new();
    let mut session = session();
    let mut reader = Cursor::new(pcm_bytes(7.5));
    let mut processor = RecordingProcessor::default();
    let mut progress = CancelAfterFirstWindow {
        cancellation: cancellation.clone(),
        events: Vec::new(),
    };

    let report = session.ingest_reader_with_control(
        &mut reader,
        &mut processor,
        &mut |_| Ok(()),
        &mut progress,
        &cancellation,
    );

    assert_eq!(processor.window_count, 1);
    assert!(matches!(
        report.events.last(),
        Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
            reason: LiveSessionEndReason::Cancelled,
            ..
        }))
    ));
    assert!(matches!(
        progress.events.as_slice(),
        [
            LiveTranscriptionProgressEvent::SessionStart { .. },
            LiveTranscriptionProgressEvent::WindowStart {
                window_index: 0,
                ..
            },
            LiveTranscriptionProgressEvent::WindowEnd {
                window_index: 0,
                ..
            },
            LiveTranscriptionProgressEvent::Cancelled {
                next_window_index: 1,
                ..
            }
        ]
    ));
}

#[test]
fn cancellation_during_running_window_keeps_final_events_and_discards_partial_output() {
    let cancellation = CancellationHandle::new();
    let mut session = session();
    let mut reader = Cursor::new(pcm_bytes(7.5));
    let mut processor = CancellingProcessor {
        cancellation: cancellation.clone(),
        window_count: 0,
    };
    let mut progress = RecordingProgress::default();

    let report = session.ingest_reader_with_control(
        &mut reader,
        &mut processor,
        &mut |_| Ok(()),
        &mut progress,
        &cancellation,
    );

    assert_eq!(processor.window_count, 1);
    assert!(report
        .events
        .iter()
        .any(|event| matches!(event, LiveTranscriptEvent::Final(_))));
    assert!(!report
        .events
        .iter()
        .any(|event| matches!(event, LiveTranscriptEvent::Partial(_))));
    assert!(matches!(
        report.events.last(),
        Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
            reason: LiveSessionEndReason::Cancelled,
            final_segment_count: 1,
            ..
        }))
    ));
}

#[test]
fn live_progress_orders_session_window_model_and_completion_facts() {
    let mut session = session();
    let mut reader = Cursor::new(pcm_bytes(5.0));
    let mut processor = ModelReportingProcessor;
    let mut progress = RecordingProgress::default();
    let cancellation = CancellationHandle::new();

    let report = session.ingest_reader_with_control(
        &mut reader,
        &mut processor,
        &mut |_| Ok(()),
        &mut progress,
        &cancellation,
    );

    assert!(matches!(
        report.events.last(),
        Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
            reason: LiveSessionEndReason::Completed,
            ..
        }))
    ));
    assert!(matches!(
        progress.events.as_slice(),
        [
            LiveTranscriptionProgressEvent::SessionStart { .. },
            LiveTranscriptionProgressEvent::WindowStart {
                window_index: 0,
                ..
            },
            LiveTranscriptionProgressEvent::ModelLoadStart {
                window_index: 0,
                ..
            },
            LiveTranscriptionProgressEvent::ModelLoadEnd {
                window_index: 0,
                ..
            },
            LiveTranscriptionProgressEvent::WindowEnd {
                window_index: 0,
                ..
            },
            LiveTranscriptionProgressEvent::Completed {
                window_count: 1,
                ..
            }
        ]
    ));
}

#[test]
fn live_progress_reports_window_failure_before_typed_error_end() {
    let mut session = session();
    let mut reader = Cursor::new(pcm_bytes(5.0));
    let mut processor = FailingProcessor;
    let mut progress = RecordingProgress::default();
    let cancellation = CancellationHandle::new();

    let report = session.ingest_reader_with_control(
        &mut reader,
        &mut processor,
        &mut |_| Ok(()),
        &mut progress,
        &cancellation,
    );

    assert!(matches!(
        progress.events.as_slice(),
        [
            LiveTranscriptionProgressEvent::SessionStart { .. },
            LiveTranscriptionProgressEvent::WindowStart {
                window_index: 0,
                ..
            },
            LiveTranscriptionProgressEvent::Failure {
                window_index: Some(0),
                message,
                ..
            }
        ] if message == "live PCM window processing failed: injected failure"
    ));
    assert!(matches!(
        report.events.as_slice(),
        [
            LiveTranscriptEvent::Error(_),
            LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                reason: LiveSessionEndReason::Error,
                ..
            })
        ]
    ));
}

#[test]
fn live_transcript_error_is_also_reported_as_progress_failure() {
    let mut session = session();
    let mut reader = Cursor::new(pcm_bytes(5.0));
    let mut processor = TranscriptErrorProcessor;
    let mut progress = RecordingProgress::default();
    let cancellation = CancellationHandle::new();

    let report = session.ingest_reader_with_control(
        &mut reader,
        &mut processor,
        &mut |_| Ok(()),
        &mut progress,
        &cancellation,
    );

    assert_eq!(report.window_count, 0);
    assert!(matches!(
        progress.events.as_slice(),
        [
            LiveTranscriptionProgressEvent::SessionStart { .. },
            LiveTranscriptionProgressEvent::WindowStart {
                window_index: 0,
                ..
            },
            LiveTranscriptionProgressEvent::Failure {
                window_index: Some(0),
                message,
                ..
            }
        ] if message == "processing fell behind live input"
    ));
}

fn session() -> LivePcmIngestionSession {
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
    .expect("valid live session")
}

fn pcm_bytes(seconds: f64) -> Vec<u8> {
    vec![0.0_f32; (seconds * 16_000.0) as usize]
        .into_iter()
        .flat_map(f32::to_le_bytes)
        .collect()
}

#[derive(Default)]
struct RecordingProcessor {
    window_count: usize,
}

impl LivePcmWindowProcessor for RecordingProcessor {
    fn process_window(
        &mut self,
        _window: LivePcmWindow,
        _progress: &mut dyn LiveTranscriptionProgressObserver,
    ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowProcessingError> {
        self.window_count += 1;
        Ok(Vec::new())
    }
}

struct CancellingProcessor {
    cancellation: CancellationHandle,
    window_count: usize,
}

impl LivePcmWindowProcessor for CancellingProcessor {
    fn process_window(
        &mut self,
        window: LivePcmWindow,
        _progress: &mut dyn LiveTranscriptionProgressObserver,
    ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowProcessingError> {
        self.window_count += 1;
        self.cancellation.cancel();
        Ok(vec![
            LiveTranscriptEvent::Partial(LivePartialTranscript {
                session_id: "session-1".to_string(),
                sequence: 0,
                window_start_seconds: window.start_seconds,
                window_end_seconds: window.end_seconds,
                window_start_at_utc: "2026-07-18T10:00:00Z".to_string(),
                window_end_at_utc: "2026-07-18T10:00:05Z".to_string(),
                text: "unstable".to_string(),
                segments: Vec::new(),
            }),
            LiveTranscriptEvent::Final(LiveFinalTranscriptSegment {
                session_id: "session-1".to_string(),
                sequence: 0,
                segment_id: "seg-000001".to_string(),
                start_seconds: 0.2,
                end_seconds: 1.0,
                start_at_utc: "2026-07-18T10:00:00.200Z".to_string(),
                end_at_utc: "2026-07-18T10:00:01Z".to_string(),
                text: "stable".to_string(),
                language: Some("en".to_string()),
            }),
        ])
    }
}

struct ModelReportingProcessor;

impl LivePcmWindowProcessor for ModelReportingProcessor {
    fn process_window(
        &mut self,
        window: LivePcmWindow,
        progress: &mut dyn LiveTranscriptionProgressObserver,
    ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowProcessingError> {
        progress.observe(LiveTranscriptionProgressEvent::ModelLoadStart {
            session_id: "session-1".to_string(),
            window_index: window.window_index,
            provider: "native".to_string(),
            model_id: "tiny.en".to_string(),
        });
        progress.observe(LiveTranscriptionProgressEvent::ModelLoadEnd {
            session_id: "session-1".to_string(),
            window_index: window.window_index,
            provider: "native".to_string(),
            model_id: "tiny.en".to_string(),
            duration_seconds: 0.01,
        });
        Ok(Vec::new())
    }
}

struct FailingProcessor;

impl LivePcmWindowProcessor for FailingProcessor {
    fn process_window(
        &mut self,
        _window: LivePcmWindow,
        _progress: &mut dyn LiveTranscriptionProgressObserver,
    ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowProcessingError> {
        Err(LiveWindowProcessingError::new("injected failure"))
    }
}

struct TranscriptErrorProcessor;

impl LivePcmWindowProcessor for TranscriptErrorProcessor {
    fn process_window(
        &mut self,
        _window: LivePcmWindow,
        _progress: &mut dyn LiveTranscriptionProgressObserver,
    ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowProcessingError> {
        Ok(vec![LiveTranscriptEvent::Error(LiveTranscriptError {
            session_id: "session-1".to_string(),
            sequence: 0,
            message: "processing fell behind live input".to_string(),
            recoverable: false,
        })])
    }
}

#[derive(Default)]
struct RecordingProgress {
    events: Vec<LiveTranscriptionProgressEvent>,
}

impl LiveTranscriptionProgressObserver for RecordingProgress {
    fn observe(&mut self, event: LiveTranscriptionProgressEvent) {
        self.events.push(event);
    }
}

struct CancelAfterFirstWindow {
    cancellation: CancellationHandle,
    events: Vec<LiveTranscriptionProgressEvent>,
}

impl LiveTranscriptionProgressObserver for CancelAfterFirstWindow {
    fn observe(&mut self, event: LiveTranscriptionProgressEvent) {
        if matches!(
            event,
            LiveTranscriptionProgressEvent::WindowEnd {
                window_index: 0,
                ..
            }
        ) {
            self.cancellation.cancel();
        }
        self.events.push(event);
    }
}

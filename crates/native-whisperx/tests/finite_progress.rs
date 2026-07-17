use std::path::PathBuf;

use native_whisperx::{
    run_many_with_control, run_with_control, AlignmentConfig, AsrConfig, CancellationHandle,
    DiarizationConfig, FiniteTranscriptionOutcome, InputSource, MultiInputTranscriptionOutcome,
    NativeWhisperxConfig, OutputConfig, TranscriptionProgressEvent, TranscriptionProgressObserver,
    TranslationConfig, VadConfig, VadMethod,
};

#[derive(Default)]
struct RecordingObserver {
    events: Vec<TranscriptionProgressEvent>,
}

#[test]
fn multi_input_cancellation_does_not_start_eligible_work_and_reports_it_unfinished() {
    let cancellation = CancellationHandle::new();
    cancellation.cancel();
    let mut observer = RecordingObserver::default();

    let outcome = run_many_with_control(
        vec![
            invalid_config("first.wav"),
            NativeWhisperxConfig {
                vad: VadConfig {
                    method: VadMethod::Silero,
                    ..VadConfig::default()
                },
                ..invalid_config("second.wav")
            },
        ],
        &mut observer,
        &cancellation,
    )
    .expect("cooperative cancellation is distinct from failure");

    let MultiInputTranscriptionOutcome::Cancelled {
        completed,
        unfinished,
        ..
    } = outcome
    else {
        panic!("expected cancelled Multi-Input Transcription Run");
    };
    assert!(completed.is_empty());
    assert_eq!(unfinished.len(), 2);
    assert_eq!(unfinished[0].input(), std::path::Path::new("first.wav"));
    assert_eq!(unfinished[1].input(), std::path::Path::new("second.wav"));
    assert!(!observer
        .events
        .iter()
        .any(|event| matches!(event, TranscriptionProgressEvent::FileStart { .. })));
    assert!(!observer
        .events
        .iter()
        .any(|event| matches!(event, TranscriptionProgressEvent::Failure { .. })));
}

impl TranscriptionProgressObserver for RecordingObserver {
    fn observe(&mut self, event: TranscriptionProgressEvent) {
        self.events.push(event);
    }
}

fn invalid_config(input: &str) -> NativeWhisperxConfig {
    NativeWhisperxConfig {
        input: InputSource::Path {
            path: PathBuf::from(input),
        },
        asr: AsrConfig::default(),
        translation: TranslationConfig::default(),
        vad: VadConfig::default(),
        alignment: AlignmentConfig::default(),
        diarization: DiarizationConfig::default(),
        output: OutputConfig {
            formats: Vec::new(),
            ..OutputConfig::default()
        },
    }
}

#[test]
fn cancellation_before_finite_workflow_returns_typed_outcome_without_failure() {
    let cancellation = CancellationHandle::new();
    cancellation.cancel();
    let mut observer = RecordingObserver::default();

    let outcome = run_with_control(
        invalid_config("cancelled.wav"),
        &mut observer,
        &cancellation,
    )
    .expect("cooperative cancellation is an outcome, not a workflow failure");

    assert!(matches!(outcome, FiniteTranscriptionOutcome::Cancelled(_)));
    assert!(observer
        .events
        .iter()
        .any(|event| matches!(event, TranscriptionProgressEvent::Cancelled { .. })));
    assert!(!observer
        .events
        .iter()
        .any(|event| matches!(event, TranscriptionProgressEvent::Failure { .. })));
}

#[test]
fn cancellation_handle_can_be_requested_from_another_thread() {
    let cancellation = CancellationHandle::new();
    let worker_handle = cancellation.clone();

    std::thread::spawn(move || worker_handle.cancel())
        .join()
        .expect("cancelling thread should finish");

    assert!(cancellation.is_cancelled());
}

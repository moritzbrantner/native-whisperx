use std::path::PathBuf;

use native_whisperx::{
    run, run_many, run_many_selected_media, run_many_selected_media_with_control,
    run_many_with_control, run_selected_media, run_selected_media_with_control, run_with_control,
    AlignmentConfig, AsrConfig, CancellationHandle, DiarizationConfig, FiniteTranscriptionOutcome,
    InputSource, MultiInputTranscriptionOutcome, NativeWhisperxConfig, NativeWhisperxError,
    NativeWhisperxReport, OutputConfig, SelectedMediaError, SelectedMediaInput,
    TranscriptionProgressEvent, TranscriptionProgressObserver, TranslationConfig, VadConfig,
    VadMethod,
};

#[derive(Default)]
struct RecordingObserver {
    events: Vec<TranscriptionProgressEvent>,
}

#[test]
fn legacy_native_whisperx_error_remains_exhaustively_matchable_downstream() {
    fn legacy_error_kind(error: &NativeWhisperxError) -> &'static str {
        match error {
            NativeWhisperxError::FeatureDisabled { .. } => "feature-disabled",
            NativeWhisperxError::InvalidConfig(_) => "invalid-config",
            NativeWhisperxError::Transcription(_) => "transcription",
            NativeWhisperxError::Import(_) => "import",
            #[cfg(feature = "translation")]
            NativeWhisperxError::LegacyPytorchWeights(_) => "legacy-pytorch",
            NativeWhisperxError::Json(_) => "json",
            NativeWhisperxError::Io(_) => "io",
        }
    }

    assert_eq!(
        legacy_error_kind(&NativeWhisperxError::InvalidConfig(
            "selected media must not add a legacy error variant".to_string()
        )),
        "invalid-config"
    );
}

#[test]
fn legacy_and_selected_entrypoints_keep_separate_error_types_downstream() {
    let _: fn(NativeWhisperxConfig) -> Result<NativeWhisperxReport, NativeWhisperxError> = run;
    let _: fn(Vec<NativeWhisperxConfig>) -> Result<Vec<NativeWhisperxReport>, NativeWhisperxError> =
        run_many;
    let _: fn(
        NativeWhisperxConfig,
        SelectedMediaInput,
    ) -> Result<NativeWhisperxReport, SelectedMediaError> = run_selected_media;
    let _: fn(
        Vec<NativeWhisperxConfig>,
        SelectedMediaInput,
    ) -> Result<Vec<NativeWhisperxReport>, SelectedMediaError> = run_many_selected_media;
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
fn cancellation_before_selected_media_workflow_returns_before_decode() {
    let cancellation = CancellationHandle::new();
    cancellation.cancel();
    let mut observer = RecordingObserver::default();

    let outcome = run_selected_media_with_control(
        invalid_config("missing-selected-media.mkv"),
        SelectedMediaInput::new(0),
        &mut observer,
        &cancellation,
    )
    .expect("cooperative cancellation must win before selected-media decode");

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
fn cancellation_before_selected_media_multi_input_keeps_all_inputs_unfinished() {
    let cancellation = CancellationHandle::new();
    cancellation.cancel();
    let mut observer = RecordingObserver::default();

    let outcome = run_many_selected_media_with_control(
        vec![
            invalid_config("first-selected-media.mkv"),
            invalid_config("second-selected-media.mkv"),
        ],
        SelectedMediaInput::new(0),
        &mut observer,
        &cancellation,
    )
    .expect("cooperative cancellation must win before selected-media decode");

    let MultiInputTranscriptionOutcome::Cancelled {
        completed,
        unfinished,
        ..
    } = outcome
    else {
        panic!("expected cancelled selected-media Multi-Input Transcription Run");
    };
    assert!(completed.is_empty());
    assert_eq!(unfinished.len(), 2);
    assert_eq!(
        unfinished[0].input(),
        std::path::Path::new("first-selected-media.mkv")
    );
    assert_eq!(
        unfinished[1].input(),
        std::path::Path::new("second-selected-media.mkv")
    );
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

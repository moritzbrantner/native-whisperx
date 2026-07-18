#![cfg(feature = "translation")]

use native_whisperx::{
    import_whisperx_json, translate_transcription_with_control, CancellationHandle,
    CuratedLanguage, DevicePreference, NativeOpusMtTranslationProvider,
    NativeOpusMtTranslationProviderConfig, TranscriptionProgressEvent,
    TranscriptionProgressObserver, TranslationPlan,
};
use std::path::PathBuf;

#[derive(Default)]
struct RecordingProgress {
    events: Vec<TranscriptionProgressEvent>,
}

impl TranscriptionProgressObserver for RecordingProgress {
    fn observe(&mut self, event: TranscriptionProgressEvent) {
        self.events.push(event);
    }
}

#[test]
fn public_native_provider_reports_cache_only_resolution_for_the_canonical_leg() {
    let transcript = import_whisperx_json(include_bytes!(
        "../../../tests/fixtures/whisperx-parity-sample.json"
    ))
    .expect("checked-in WhisperX fixture");
    let source: native_whisperx::TranscriptionPipelineResponse =
        serde_json::from_value(serde_json::json!({
        "accepted": true,
        "operation": "transcribe",
        "provider": "native",
        "modelId": "fixture",
        "transcript": transcript,
        "vadSegments": [],
        "alignment": null,
        "diarization": null,
        "artifacts": [],
        "diagnostics": []
        }))
        .expect("public pipeline response fixture");
    let source_before = source.clone();
    let plan = TranslationPlan::new(CuratedLanguage::English, CuratedLanguage::German)
        .expect("curated direct plan");
    let model_dir = tempfile::tempdir().expect("isolated model cache");
    let config = NativeOpusMtTranslationProviderConfig {
        model_dir: Some(model_dir.path().to_path_buf()),
        model_cache_only: true,
        device: DevicePreference::Cpu,
        ..Default::default()
    };
    let mut provider = NativeOpusMtTranslationProvider::new(config);
    let cancellation = CancellationHandle::new();
    let mut progress = RecordingProgress::default();

    let error = translate_transcription_with_control(
        &source,
        &plan,
        &mut provider,
        4,
        PathBuf::from("whisperx-parity-sample.json"),
        &mut progress,
        &cancellation,
    )
    .expect_err("an empty cache must fail without downloading");

    assert_eq!(source, source_before);
    assert!(error.to_string().contains("Helsinki-NLP/opus-mt-en-de"));
    assert!(progress.events.iter().any(|event| matches!(
        event,
        TranscriptionProgressEvent::ModelResolutionStart {
            file_index: 4,
            provider,
            model_id,
            ..
        } if provider == "marian-candle" && model_id == "Helsinki-NLP/opus-mt-en-de"
    )));
    assert!(!progress
        .events
        .iter()
        .any(|event| matches!(event, TranscriptionProgressEvent::ModelDownloadStart { .. })));
}

#[test]
#[ignore = "requires RUN_NATIVE_TRANSLATION_TESTS=1 and Hugging Face model access"]
fn public_native_provider_translates_the_checked_in_fixture() {
    if std::env::var("RUN_NATIVE_TRANSLATION_TESTS")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }

    let transcript = import_whisperx_json(include_bytes!(
        "../../../tests/fixtures/whisperx-parity-sample.json"
    ))
    .expect("checked-in WhisperX fixture");
    let source: native_whisperx::TranscriptionPipelineResponse =
        serde_json::from_value(serde_json::json!({
            "accepted": true,
            "operation": "transcribe",
            "provider": "native",
            "modelId": "fixture",
            "transcript": transcript,
            "vadSegments": [],
            "alignment": null,
            "diarization": null,
            "artifacts": [],
            "diagnostics": []
        }))
        .expect("public pipeline response fixture");
    let source_before = source.clone();
    let plan = TranslationPlan::new(CuratedLanguage::English, CuratedLanguage::German)
        .expect("curated direct plan");
    let model_dir = tempfile::tempdir().expect("isolated model cache");
    let mut provider =
        NativeOpusMtTranslationProvider::new(NativeOpusMtTranslationProviderConfig {
            model_dir: Some(model_dir.path().to_path_buf()),
            device: DevicePreference::Cpu,
            ..Default::default()
        });
    let cancellation = CancellationHandle::new();
    let mut progress = RecordingProgress::default();

    let outcome = translate_transcription_with_control(
        &source,
        &plan,
        &mut provider,
        0,
        PathBuf::from("whisperx-parity-sample.json"),
        &mut progress,
        &cancellation,
    )
    .expect("native OPUS-MT translation");

    assert_eq!(source, source_before);
    let native_whisperx::TranslatedTranscriptionOutcome::Completed(result) = outcome else {
        panic!("uncancelled translation should complete");
    };
    assert_eq!(result.transcript().language.as_deref(), Some("de"));
    assert_eq!(result.legs(), plan.legs());
    assert!(progress.events.iter().any(|event| matches!(
        event,
        TranscriptionProgressEvent::ModelDownloadStart { model_id, .. }
            if model_id == plan.legs()[0].model_id()
    )));
}

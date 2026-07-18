#![cfg(feature = "translation")]

use native_whisperx::{
    import_whisperx_json, translate_transcription_with_control, CancellationHandle,
    CuratedLanguage, DevicePreference, NativeOpusMtTranslationProvider,
    NativeOpusMtTranslationProviderConfig, TranscriptionProgressEvent,
    TranscriptionProgressObserver, TranslationPlan,
};
use sha2::{Digest, Sha256};
use std::{
    io::Read,
    path::{Path, PathBuf},
};

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

#[test]
#[ignore = "requires RUN_NATIVE_TRANSLATION_TESTS=1 and the pinned OPUS-MT cache"]
fn public_native_provider_executes_pinned_legacy_pickle_direct_and_pivot_models() {
    if std::env::var("RUN_NATIVE_TRANSLATION_TESTS")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    let model_dir = std::env::var_os("NATIVE_WHISPERX_OPUS_MT_CACHE")
        .map(PathBuf::from)
        .expect("NATIVE_WHISPERX_OPUS_MT_CACHE must point to the pinned cache root");
    for (model, revision, expected_sha256) in [
        (
            "opus-mt-de-en",
            "1a922f3b32a8e809e17a47d4b32142d8105924e5",
            "e743c3070f61f477cb62fe95ef2c9be2e77f3e488cb6b8030ff8a19e8295c87d",
        ),
        (
            "opus-mt-ROMANCE-en",
            "e9ca9975e3972afd80732f08ce01d3a1339f47f8",
            "9d77bbbd43a214959e027ffc8713fbe31f8609d14827fba645f1361ca20a6f3a",
        ),
        (
            "opus-mt-en-nl",
            "8aad73b34ff36c090e7fc8a2eb7e2e7cca235d31",
            "8b2ff97027f9b35904984dca8508ab633dfffc4e58c7fbedb7eb236d2a937a36",
        ),
    ] {
        let weights = model_dir
            .join(format!("models--Helsinki-NLP--{model}"))
            .join("snapshots")
            .join(revision)
            .join("pytorch_model.bin");
        assert_eq!(sha256(&weights), expected_sha256, "{model} weights changed");
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
    let direct_plan = TranslationPlan::new(CuratedLanguage::German, CuratedLanguage::English)
        .expect("curated direct plan");
    let mut provider =
        NativeOpusMtTranslationProvider::new(NativeOpusMtTranslationProviderConfig {
            model_dir: Some(model_dir),
            model_cache_only: true,
            device: DevicePreference::Cpu,
            max_new_tokens: 1,
        });
    let cancellation = CancellationHandle::new();
    let mut progress = RecordingProgress::default();

    let direct = translate_transcription_with_control(
        &source,
        &direct_plan,
        &mut provider,
        0,
        PathBuf::from("whisperx-parity-sample.json"),
        &mut progress,
        &cancellation,
    )
    .expect("the canonical legacy-pickle weights should load and execute");
    assert!(matches!(
        direct,
        native_whisperx::TranslatedTranscriptionOutcome::Completed(_)
    ));
    assert_eq!(source, source_before);
    assert_timing_evidence(&progress, 1);

    let pivot_plan = TranslationPlan::new(CuratedLanguage::Portuguese, CuratedLanguage::Dutch)
        .expect("curated English pivot plan");
    let mut pivot_progress = RecordingProgress::default();
    let pivot = translate_transcription_with_control(
        &source,
        &pivot_plan,
        &mut provider,
        1,
        PathBuf::from("whisperx-parity-sample.json"),
        &mut pivot_progress,
        &cancellation,
    )
    .expect("both canonical legacy-pickle pivot models should load and execute");
    assert!(matches!(
        pivot,
        native_whisperx::TranslatedTranscriptionOutcome::Completed(_)
    ));
    assert_eq!(source, source_before);
    assert_timing_evidence(&pivot_progress, 2);
    let loaded_models = pivot_progress
        .events
        .iter()
        .filter_map(|event| match event {
            TranscriptionProgressEvent::ModelLoadEnd { model_id, .. } => Some(model_id.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        loaded_models,
        [
            "Helsinki-NLP/opus-mt-ROMANCE-en",
            "Helsinki-NLP/opus-mt-en-nl"
        ]
    );
}

fn assert_timing_evidence(progress: &RecordingProgress, expected_legs: usize) {
    let leg_durations = progress
        .events
        .iter()
        .filter_map(|event| match event {
            TranscriptionProgressEvent::TranslationLegEnd {
                duration_seconds, ..
            } => Some(*duration_seconds),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(leg_durations.len(), expected_legs);
    assert!(leg_durations
        .iter()
        .all(|duration| duration.is_finite() && *duration >= 0.0));
    assert!(progress.events.iter().any(|event| matches!(
        event,
        TranscriptionProgressEvent::TaskEnd { duration_seconds, .. }
            if duration_seconds.is_finite() && *duration_seconds >= 0.0
    )));
}

fn sha256(path: &Path) -> String {
    let mut file = std::fs::File::open(path).expect("pinned weights");
    let mut digest = Sha256::new();
    let mut buffer = [0; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).expect("read pinned weights");
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    format!("{:x}", digest.finalize())
}

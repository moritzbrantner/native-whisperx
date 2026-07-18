#![cfg(feature = "translation")]

use native_whisperx::{
    import_whisperx_json, translate_transcription_with_control, CancellationHandle,
    CuratedLanguage, DevicePreference, NativeOpusMtTranslationProvider,
    NativeOpusMtTranslationProviderConfig, TranscriptionProgressEvent,
    TranscriptionProgressObserver, TranslationPlan,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Default)]
struct RecordingProgress {
    events: Vec<TranscriptionProgressEvent>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RealTranslationFixture {
    schema_version: u32,
    models: Vec<PinnedModel>,
    cases: Vec<RealTranslationCase>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PinnedModel {
    model_id: String,
    revision: String,
    sha256: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RealTranslationCase {
    name: String,
    source_language: CuratedLanguage,
    target_language: CuratedLanguage,
    source_text: String,
    model_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RealTranslationReport {
    schema_version: u32,
    cache_only: bool,
    cases: Vec<RealTranslationCaseReport>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RealTranslationCaseReport {
    name: String,
    source_language: CuratedLanguage,
    target_language: CuratedLanguage,
    model_ids: Vec<String>,
    leg_seconds: Vec<f64>,
    total_seconds: f64,
    translated_text: String,
}

impl TranscriptionProgressObserver for RecordingProgress {
    fn observe(&mut self, event: TranscriptionProgressEvent) {
        self.events.push(event);
    }
}

#[test]
fn checked_in_real_model_fixture_pins_direct_and_english_pivot_plans() {
    let fixture = real_translation_fixture();

    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.cases.len(), 2);
    for case in &fixture.cases {
        let plan = TranslationPlan::new(case.source_language, case.target_language)
            .expect("fixture languages must produce a curated plan");
        assert_eq!(
            plan.legs()
                .iter()
                .map(|leg| leg.model_id())
                .collect::<Vec<_>>(),
            case.model_ids
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            "{} must pin the public plan's ordered model legs",
            case.name
        );
        assert!(!case.source_text.trim().is_empty());
    }
}

fn real_translation_fixture() -> RealTranslationFixture {
    serde_json::from_slice(include_bytes!(
        "../../../tests/fixtures/real-opus-mt-translation.json"
    ))
    .expect("checked-in real OPUS-MT translation fixture")
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
fn cancelled_public_native_provider_run_never_resolves_a_model_or_changes_the_source() {
    let fixture = real_translation_fixture();
    let case = &fixture.cases[1];
    let source = source_response(case);
    let source_before = source.clone();
    let plan = TranslationPlan::new(case.source_language, case.target_language)
        .expect("checked-in pivot plan");
    let model_dir = tempfile::tempdir().expect("isolated model cache");
    let mut provider =
        NativeOpusMtTranslationProvider::new(NativeOpusMtTranslationProviderConfig {
            model_dir: Some(model_dir.path().to_path_buf()),
            model_cache_only: true,
            device: DevicePreference::Cpu,
            ..Default::default()
        });
    let cancellation = CancellationHandle::new();
    cancellation.cancel();
    let mut progress = RecordingProgress::default();

    let outcome = translate_transcription_with_control(
        &source,
        &plan,
        &mut provider,
        0,
        PathBuf::from("portuguese-fixture.json"),
        &mut progress,
        &cancellation,
    )
    .expect("cooperative cancellation is not a translation failure");

    assert!(matches!(
        outcome,
        native_whisperx::TranslatedTranscriptionOutcome::Cancelled(_)
    ));
    assert_eq!(source, source_before);
    assert!(progress
        .events
        .iter()
        .any(|event| matches!(event, TranscriptionProgressEvent::Cancelled { .. })));
    assert!(!progress.events.iter().any(|event| matches!(
        event,
        TranscriptionProgressEvent::ModelResolutionStart { .. }
            | TranscriptionProgressEvent::ModelDownloadStart { .. }
            | TranscriptionProgressEvent::ModelLoadStart { .. }
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
    let fixture = real_translation_fixture();
    for model in &fixture.models {
        let weights = model_dir
            .join(format!("models--{}", model.model_id.replace('/', "--")))
            .join("snapshots")
            .join(&model.revision)
            .join("pytorch_model.bin");
        assert_eq!(
            sha256(&weights),
            model.sha256,
            "{} weights changed",
            model.model_id
        );
    }
    let mut provider =
        NativeOpusMtTranslationProvider::new(NativeOpusMtTranslationProviderConfig {
            model_dir: Some(model_dir),
            model_cache_only: true,
            device: DevicePreference::Cpu,
            max_new_tokens: 16,
        });
    let cancellation = CancellationHandle::new();
    let mut report = RealTranslationReport {
        schema_version: 1,
        cache_only: true,
        cases: Vec::new(),
    };

    for (case_index, case) in fixture.cases.iter().enumerate() {
        let source = source_response(case);
        let source_before = source.clone();
        let plan = TranslationPlan::new(case.source_language, case.target_language)
            .expect("checked-in curated plan");
        let mut progress = RecordingProgress::default();
        let outcome = translate_transcription_with_control(
            &source,
            &plan,
            &mut provider,
            case_index,
            PathBuf::from(format!("{}.json", case.name)),
            &mut progress,
            &cancellation,
        )
        .expect("pinned cache-only OPUS-MT translation");
        let native_whisperx::TranslatedTranscriptionOutcome::Completed(result) = outcome else {
            panic!("uncancelled real translation should complete");
        };
        assert_eq!(source, source_before);
        assert_eq!(
            result.transcript().language.as_deref(),
            Some(case.target_language.code())
        );
        let translated_text = result.transcript().text_or_joined();
        assert!(!translated_text.trim().is_empty());
        assert_cache_only_progress(&progress, &case.model_ids);
        let (leg_seconds, total_seconds) = timing_evidence(&progress, case.model_ids.len());
        report.cases.push(RealTranslationCaseReport {
            name: case.name.clone(),
            source_language: case.source_language,
            target_language: case.target_language,
            model_ids: case.model_ids.clone(),
            leg_seconds,
            total_seconds,
            translated_text,
        });
    }

    let report_json = serde_json::to_string_pretty(&report).expect("translation evidence JSON");
    println!("{report_json}");
    if let Some(path) = std::env::var_os("NATIVE_WHISPERX_TRANSLATION_REPORT") {
        std::fs::write(&path, report_json).expect("write requested translation evidence report");
    }
}

fn source_response(case: &RealTranslationCase) -> native_whisperx::TranscriptionPipelineResponse {
    serde_json::from_value(serde_json::json!({
        "accepted": true,
        "operation": "transcribe",
        "provider": "native",
        "modelId": "fixture",
        "transcript": {
            "text": case.source_text,
            "language": case.source_language.code(),
            "segments": [{
                "index": 0,
                "startSeconds": 0.0,
                "endSeconds": 1.0,
                "text": case.source_text,
                "language": case.source_language.code(),
                "isFinal": true,
                "words": [],
                "chars": [],
                "attributes": {}
            }],
            "attributes": {}
        },
        "vadSegments": [],
        "alignment": null,
        "diarization": null,
        "artifacts": [],
        "diagnostics": []
    }))
    .expect("public pipeline response fixture")
}

fn assert_cache_only_progress(progress: &RecordingProgress, expected_models: &[String]) {
    assert!(!progress.events.iter().any(|event| matches!(
        event,
        TranscriptionProgressEvent::ModelDownloadStart { .. }
            | TranscriptionProgressEvent::ModelDownloadEnd { .. }
    )));
    let resolved = progress
        .events
        .iter()
        .filter_map(|event| match event {
            TranscriptionProgressEvent::ModelResolutionEnd {
                model_id, source, ..
            } => Some((model_id.as_str(), source.as_str())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        resolved,
        expected_models
            .iter()
            .map(|model_id| (model_id.as_str(), "hugging-face-cache"))
            .collect::<Vec<_>>()
    );
}

fn timing_evidence(progress: &RecordingProgress, expected_legs: usize) -> (Vec<f64>, f64) {
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
    let total_seconds = progress
        .events
        .iter()
        .find_map(|event| match event {
            TranscriptionProgressEvent::TaskEnd {
                task: native_whisperx::TranscriptionProgressTask::Translation,
                duration_seconds,
                ..
            } => Some(*duration_seconds),
            _ => None,
        })
        .expect("translation total timing");
    assert!(total_seconds.is_finite() && total_seconds >= 0.0);
    (leg_durations, total_seconds)
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

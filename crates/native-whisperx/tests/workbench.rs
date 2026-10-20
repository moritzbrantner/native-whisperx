#![cfg(all(
    feature = "native",
    feature = "translation",
    feature = "pyannote-diarization"
))]

use native_whisperx::{
    prepare_workbench, DevicePreference, OutputFormat, WorkbenchRequest, WorkbenchStage,
};

#[test]
fn selected_workbench_tasks_resolve_to_one_ordered_native_workflow() {
    let request: WorkbenchRequest = serde_json::from_value(serde_json::json!({
        "input": "interview.wav", "align": true, "identifySpeakers": true,
        "translation": {"modelId": "Helsinki-NLP/opus-mt-en-de", "sourceLanguage": "en", "targetLanguage": "de"},
        "language": "en", "formats": ["json", "srt"], "device": "cpu"
    })).expect("typed product request");
    let workflow = prepare_workbench(request).expect("workflow selection");
    assert_eq!(
        workflow.stages(),
        &[
            WorkbenchStage::Transcribe,
            WorkbenchStage::Align,
            WorkbenchStage::IdentifySpeakers,
            WorkbenchStage::Translate,
            WorkbenchStage::WriteOutputs,
        ]
    );
    let config = workflow.config();
    assert!(config.alignment.enabled);
    assert!(config.diarization.enabled);
    assert!(config.translation.enabled);
    assert_eq!(config.asr.device, DevicePreference::Cpu);
    assert_eq!(config.asr.language.as_deref(), Some("en"));
    assert_eq!(
        config.output.formats,
        [OutputFormat::Json, OutputFormat::Srt]
    );
    assert_eq!(
        config.output.output_dir.as_deref(),
        Some(std::path::Path::new(".")),
        "Input-Local Output must emit real files"
    );
}

#[test]
fn explicit_workbench_diarization_bundles_select_the_supported_model_family() {
    let request: WorkbenchRequest = serde_json::from_value(serde_json::json!({
        "input": "interview.wav", "identifySpeakers": true,
        "vadBundle": "models/vad", "diarizationBundle": "models/diarization"
    }))
    .expect("request");
    let prepared = prepare_workbench(request).expect("selection");
    assert_eq!(
        prepared.config().diarization.model_id,
        "pyannote/speaker-diarization-community-1"
    );
    assert_eq!(
        prepared.config().vad.method,
        native_whisperx::VadMethod::Pyannote
    );
}

#[cfg(feature = "pyannote-vad")]
#[test]
fn translation_preserves_selected_vad_instead_of_silently_using_energy() {
    let temp = tempfile::tempdir().expect("empty explicit VAD directory");
    let request: WorkbenchRequest = serde_json::from_value(serde_json::json!({
        "input": "in-memory.wav", "language": "en",
        "model": "workbench-fixture/missing-model",
        "vadBundle": temp.path(),
        "translation": {"modelId": "workbench-fixture/missing-translation", "sourceLanguage": "en", "targetLanguage": "de"}
    })).expect("request");
    let mut config = prepare_workbench(request).expect("selection").into_config();
    config.output.output_dir = None;
    config.input = native_whisperx::InputSource::Samples {
        samples: vec![0.0; 1600],
        sample_rate: 16000,
        channels: 1,
        source: None,
    };
    let error = native_whisperx::run(config)
        .expect_err("selected VAD resource is absent")
        .to_string();
    assert!(
        error.contains("pyannote VAD model path") && error.contains("does not exist"),
        "the selected VAD must fail before ASR/translation model resolution: {error}"
    );
}

#[test]
fn workbench_auto_update_is_explicit_and_keeps_embeddings_internal() {
    for enabled in [false, true] {
        let request: WorkbenchRequest = serde_json::from_value(serde_json::json!({
            "input": "interview.wav", "identifySpeakers": true, "autoUpdateSpeakers": enabled
        }))
        .unwrap();
        let workflow = prepare_workbench(request).unwrap();
        assert_eq!(workflow.config().diarization.save_draft_speakers, enabled);
        assert!(!workflow.config().diarization.return_speaker_embeddings);
    }
}

#[test]
fn workbench_rejects_auto_update_without_a_speaker_task() {
    let request: WorkbenchRequest = serde_json::from_value(
        serde_json::json!({"input":"interview.wav", "autoUpdateSpeakers":true}),
    )
    .unwrap();
    assert!(prepare_workbench(request)
        .unwrap_err()
        .to_string()
        .contains("requires Identify speakers"));
}

#[test]
fn named_preset_round_trip_keeps_one_task_configuration_without_input_or_secrets() {
    use native_whisperx::{WorkbenchPreset, WorkbenchSettings};
    let settings = WorkbenchSettings {
        model: "tiny.en".into(),
        language: Some("en".into()),
        align: true,
        formats: vec![OutputFormat::Srt, OutputFormat::NativeJson],
        ..Default::default()
    };
    let preset = WorkbenchPreset::new("Interview transcript", settings).unwrap();
    let encoded = serde_json::to_string(&preset).unwrap();
    assert!(!encoded.contains("input"));
    assert!(!encoded.contains("token"));
    let reopened: WorkbenchPreset = serde_json::from_str(&encoded).unwrap();
    reopened.validate().unwrap();
    let prepared = prepare_workbench(reopened.settings.for_input("another.wav")).unwrap();
    assert_eq!(
        prepared.stages(),
        &[
            WorkbenchStage::Transcribe,
            WorkbenchStage::Align,
            WorkbenchStage::WriteOutputs
        ]
    );
    assert_eq!(prepared.config().asr.model_id, "tiny.en");
    assert_eq!(
        prepared.config().output.formats,
        [OutputFormat::Srt, OutputFormat::NativeJson]
    );
    assert!(prepared.config().asr.model_cache_only);
    assert!(prepared.config().alignment.model_cache_only);
}

#[test]
fn preset_preflight_reports_a_cache_miss_without_starting_or_downloading() {
    let temp = tempfile::tempdir().unwrap();
    let missing = temp.path().join("absent-cache");
    let settings = native_whisperx::WorkbenchSettings {
        model_dir: Some(missing.clone()),
        device: DevicePreference::Cpu,
        ..Default::default()
    };
    let prepared = prepare_workbench(settings.for_input("recording.wav")).unwrap();
    let report = native_whisperx::preflight_workbench(prepared.config());
    assert!(!report.ready);
    assert!(report
        .checks
        .iter()
        .any(|check| !check.available && check.message.contains("cache")));
    assert!(!missing.exists());
}

#[test]
fn preset_settings_reject_session_data_and_unknown_runtime_fields() {
    for field in ["input", "hfToken", "progress", "transcript", "sessionToken"] {
        let raw = serde_json::json!({(field): "must not be persisted"});
        assert!(
            serde_json::from_value::<native_whisperx::WorkbenchSettings>(raw).is_err(),
            "accepted {field}"
        );
    }
}

#[test]
fn preset_preflight_checks_local_files_without_parsing_weights_and_checks_alignment_separately() {
    let temp = tempfile::tempdir().unwrap();
    for file in [
        "config.json",
        "generation_config.json",
        "tokenizer.json",
        "preprocessor_config.json",
        "model.safetensors",
    ] {
        std::fs::write(temp.path().join(file), b"not model data").unwrap();
    }
    let settings = native_whisperx::WorkbenchSettings {
        model_dir: Some(temp.path().into()),
        device: DevicePreference::Cpu,
        ..Default::default()
    };
    let mut config = prepare_workbench(settings.for_input("recording.wav"))
        .unwrap()
        .into_config();
    let report = native_whisperx::preflight_workbench(&config);
    assert!(report.ready, "{:?}", report.checks);
    config.alignment.enabled = true;
    config.alignment.model_dir = Some(temp.path().join("missing-alignment"));
    let report = native_whisperx::preflight_workbench(&config);
    assert!(!report.ready);
    assert!(report
        .checks
        .iter()
        .any(|check| check.resource == "transcription" && check.available));
    assert!(report
        .checks
        .iter()
        .any(|check| check.resource == "alignment" && !check.available));
    assert!(!temp.path().join("missing-alignment").exists());
}

#[test]
fn saved_translation_presets_reject_invalid_or_pivot_pairs() {
    for (source, target, model) in [
        ("en", "en", "Helsinki-NLP/opus-mt-en-de"),
        ("xx", "de", "Helsinki-NLP/opus-mt-xx-de"),
        ("fr", "nl", "Helsinki-NLP/opus-mt-fr-nl"),
    ] {
        let settings = native_whisperx::WorkbenchSettings {
            language: Some(source.into()),
            translation: Some(native_whisperx::WorkbenchTranslation {
                model_id: model.into(),
                source_language: source.into(),
                target_language: target.into(),
            }),
            ..Default::default()
        };
        assert!(
            native_whisperx::WorkbenchPreset::new("Unsupported pair", settings).is_err(),
            "accepted {source} to {target}"
        );
    }
}

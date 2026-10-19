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

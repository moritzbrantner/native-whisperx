#![cfg(all(feature = "native", feature = "pyannote-diarization"))]

use native_whisperx::{
    preflight_workbench, prepare_workbench, ConfigSelection, DevicePreference,
    SpeakerDirectorySelection, WorkbenchSettings,
};

#[test]
#[ignore = "requires the approved caller-owned PYANNOTE_COMMUNITY_BUNDLE; no inference runtime is initialized"]
fn preflight_rejects_an_incompatible_library_before_loading_any_model() {
    let bundle = std::env::var_os("PYANNOTE_COMMUNITY_BUNDLE").expect("approved pyannote bundle");
    let temp = tempfile::tempdir().unwrap();
    for name in [
        "config.json",
        "generation_config.json",
        "tokenizer.json",
        "preprocessor_config.json",
        "model.safetensors",
    ] {
        std::fs::write(
            temp.path().join(name),
            b"not model data; inspection must not load it",
        )
        .unwrap();
    }
    let directory = temp.path().join("index");
    std::fs::create_dir(&directory).unwrap();
    let library = directory.join("library.json");
    let original = include_bytes!("../../../tests/fixtures/speaker-library-with-drafts.json");
    std::fs::write(&library, original).unwrap();
    let modified = std::fs::metadata(&library).unwrap().modified().unwrap();
    let mut config = prepare_workbench(
        WorkbenchSettings {
            identify_speakers: true,
            model_dir: Some(temp.path().into()),
            diarization_bundle: Some(bundle.into()),
            device: DevicePreference::Cpu,
            speaker_directory: Some(SpeakerDirectorySelection {
                explicit_path: Some(directory),
                ..Default::default()
            }),
            ..Default::default()
        }
        .for_input("recording.wav"),
    )
    .unwrap()
    .into_config();
    // Isolate the model/library compatibility check from separate VAD resources.
    config.vad.enabled = false;
    config.vad.method = native_whisperx::VadMethod::Energy;
    config.vad.selection = ConfigSelection::Explicit;
    let report = preflight_workbench(&config);
    assert!(!report.ready, "{:?}", report.checks);
    assert!(
        report
            .checks
            .iter()
            .any(|check| check.resource == "transcription" && check.available),
        "{:?}",
        report.checks
    );
    assert!(
        report
            .checks
            .iter()
            .any(|check| check.resource == "speaker library"
                && !check.available
                && check.message.contains("incompatible embedding model")),
        "{:?}",
        report.checks
    );
    assert_eq!(std::fs::read(&library).unwrap(), original);
    assert_eq!(
        std::fs::metadata(&library).unwrap().modified().unwrap(),
        modified
    );
}

use std::path::PathBuf;

use native_whisperx::{
    inspect_workflow_mapping, AlignmentConfig, AsrConfig, DiarizationConfig, InputSource,
    NativeWhisperxConfig, OutputConfig, TranslationConfig, VadConfig,
};

fn config_with_model(model_id: &str) -> NativeWhisperxConfig {
    NativeWhisperxConfig {
        input: InputSource::Path {
            path: PathBuf::from("sample.wav"),
        },
        asr: AsrConfig {
            model_id: model_id.to_string(),
            ..AsrConfig::default()
        },
        translation: TranslationConfig::default(),
        vad: VadConfig::default(),
        alignment: AlignmentConfig::default(),
        diarization: DiarizationConfig::default(),
        output: OutputConfig::default(),
    }
}

#[test]
fn advertised_whisper_aliases_map_to_canonical_hugging_face_repositories() {
    for (alias, repository) in [
        ("tiny", "openai/whisper-tiny"),
        ("tiny.en", "openai/whisper-tiny.en"),
        ("base", "openai/whisper-base"),
        ("base.en", "openai/whisper-base.en"),
        ("small", "openai/whisper-small"),
        ("small.en", "openai/whisper-small.en"),
        ("medium", "openai/whisper-medium"),
        ("medium.en", "openai/whisper-medium.en"),
        ("large", "openai/whisper-large-v3"),
        ("large-v1", "openai/whisper-large-v1"),
        ("large-v2", "openai/whisper-large-v2"),
        ("large-v3", "openai/whisper-large-v3"),
        ("large-v3-turbo", "openai/whisper-large-v3-turbo"),
    ] {
        let mapping = inspect_workflow_mapping(&config_with_model(alias))
            .unwrap_or_else(|error| panic!("advertised alias `{alias}` should map: {error}"));

        assert_eq!(mapping["provider"]["kind"], "candleWhisper", "alias `{alias}`");
        assert_eq!(
            mapping["provider"]["modelId"], repository,
            "alias `{alias}`"
        );
    }
}

#[test]
fn explicit_hugging_face_repository_ids_pass_through_unchanged() {
    let mapping = inspect_workflow_mapping(&config_with_model("acme/candle-whisper"))
        .expect("explicit Hugging Face repository IDs should remain valid");

    assert_eq!(mapping["provider"]["modelId"], "acme/candle-whisper");
}

#[test]
fn unadvertised_whisper_aliases_fail_before_runtime_setup() {
    let error = inspect_workflow_mapping(&config_with_model("unknown"))
        .expect_err("unadvertised aliases must fail before model resolution");

    assert!(
        error
            .to_string()
            .contains("unsupported native Candle Whisper model alias `unknown`"),
        "unexpected error: {error}"
    );
}

#[test]
fn explicit_multilingual_requests_keep_the_stable_autoregressive_decode_runtime() {
    let mut config = config_with_model("small");
    config.asr.language = Some("de".to_string());
    config.asr.batch_chunks = true;
    config.asr.max_batch_size = Some(4);

    let mapping = inspect_workflow_mapping(&config)
        .expect("explicit multilingual native ASR should map without loading a model");

    assert_eq!(mapping["provider"]["decodeRuntime"], "autoregressiveKvCache");
}

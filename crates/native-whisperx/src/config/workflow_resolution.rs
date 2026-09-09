//! Product-level workflow configuration resolution before provider mapping.

use super::{AsrProvider, AutomaticWorkflowSelection, NativeWhisperxConfig, NativeWhisperxError};

pub fn resolve_automatic_workflow_selection(
    config: &NativeWhisperxConfig,
) -> Result<AutomaticWorkflowSelection, NativeWhisperxError> {
    let mut normalized = config.clone();
    normalize_native_asr(&mut normalized)?;
    super::automatic_workflow_selection::resolve_automatic_workflow_selection(&normalized)
}

fn normalize_native_asr(config: &mut NativeWhisperxConfig) -> Result<(), NativeWhisperxError> {
    if config.asr.provider != AsrProvider::Native {
        return Ok(());
    }

    config.asr.model_id = canonical_native_whisper_model_id(&config.asr.model_id)
        .ok_or_else(|| {
            NativeWhisperxError::InvalidConfig(format!(
                "unsupported native Candle Whisper model alias `{}`; expected an advertised Whisper alias or a Hugging Face repository ID",
                config.asr.model_id
            ))
        })?
        .to_string();

    if explicit_multilingual_language(config.asr.language.as_deref()) {
        config.asr.batch_chunks = false;
        config.asr.max_batch_size = Some(1);
    }

    Ok(())
}

fn explicit_multilingual_language(language: Option<&str>) -> bool {
    language.is_some_and(|language| !language.trim().eq_ignore_ascii_case("en"))
}

fn canonical_native_whisper_model_id(model_id: &str) -> Option<&str> {
    match model_id {
        "tiny" => Some("openai/whisper-tiny"),
        "tiny.en" => Some("openai/whisper-tiny.en"),
        "base" => Some("openai/whisper-base"),
        "base.en" => Some("openai/whisper-base.en"),
        "small" => Some("openai/whisper-small"),
        "small.en" => Some("openai/whisper-small.en"),
        "medium" => Some("openai/whisper-medium"),
        "medium.en" => Some("openai/whisper-medium.en"),
        "large" => Some("openai/whisper-large-v3"),
        "large-v1" => Some("openai/whisper-large-v1"),
        "large-v2" => Some("openai/whisper-large-v2"),
        "large-v3" => Some("openai/whisper-large-v3"),
        "large-v3-turbo" => Some("openai/whisper-large-v3-turbo"),
        repository if looks_like_hugging_face_repository_id(repository) => Some(repository),
        _ => None,
    }
}

fn looks_like_hugging_face_repository_id(model_id: &str) -> bool {
    let mut parts = model_id.split('/');
    matches!(
        (parts.next(), parts.next(), parts.next()),
        (Some(owner), Some(repository), None) if !owner.is_empty() && !repository.is_empty()
    )
}

#[cfg(test)]
mod tests {
    use audio_analysis_transcription::CandleWhisperTimingMode;

    use super::*;
    use crate::config::{
        AlignmentConfig, AsrConfig, DiarizationConfig, InputSource, OutputConfig,
        TranslationConfig, VadConfig,
    };
    use std::path::PathBuf;

    fn config(model_id: &str) -> NativeWhisperxConfig {
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
    fn normalizes_advertised_aliases_without_changing_repository_ids() {
        let mut alias = config("large-v3-turbo");
        normalize_native_asr(&mut alias).expect("advertised alias");
        assert_eq!(alias.asr.model_id, "openai/whisper-large-v3-turbo");

        let mut repository = config("acme/candle-whisper");
        normalize_native_asr(&mut repository).expect("repository ID");
        assert_eq!(repository.asr.model_id, "acme/candle-whisper");
    }

    #[test]
    fn multilingual_requests_disable_tensor_batching() {
        let mut config = config("small");
        config.asr.language = Some("de".to_string());
        config.asr.batch_chunks = true;
        config.asr.max_batch_size = Some(8);

        normalize_native_asr(&mut config).expect("multilingual configuration");

        assert!(!config.asr.batch_chunks);
        assert_eq!(config.asr.max_batch_size, Some(1));
    }

    #[test]
    fn multilingual_no_align_uses_whisperx_window_contract() {
        let mut config = config("small");
        config.asr.language = Some("de".to_string());
        config.alignment.enabled = false;

        let request = crate::config_mapping::build_native_request_config_for_workflow(&config)
            .expect("multilingual no-align request should map");

        assert_eq!(
            request.window.timing_mode,
            CandleWhisperTimingMode::NoTimestamps
        );
        assert_eq!(request.window.leading_context_seconds, 0.0);
        assert_eq!(request.window.trailing_context_seconds, 0.0);

        config.alignment.enabled = true;
        let aligned = crate::config_mapping::build_native_request_config_for_workflow(&config)
            .expect("aligned multilingual request should preserve defaults");
        assert_eq!(aligned.window.timing_mode, CandleWhisperTimingMode::Auto);
        assert_eq!(aligned.window.leading_context_seconds, 0.25);
        assert_eq!(aligned.window.trailing_context_seconds, 0.04);
    }
}

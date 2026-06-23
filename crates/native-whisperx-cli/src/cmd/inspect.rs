//! Model inspection command for resolved native asset paths.

use super::*;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct InspectModelsArgs {
    #[arg(long, visible_alias = "whisper_bundle")]
    pub(crate) whisper_bundle: Option<PathBuf>,
    #[arg(long, default_value = "small")]
    pub(crate) model: String,
    #[arg(long = "compute-type", visible_alias = "compute_type")]
    pub(crate) compute_type: Option<String>,
    #[arg(long = "no-align", visible_alias = "no_align")]
    pub(crate) no_align: bool,
    #[arg(long, visible_alias = "alignment_bundle")]
    pub(crate) alignment_bundle: Option<PathBuf>,
    #[arg(
        long = "align-model",
        visible_alias = "align_model",
        default_value = "facebook/wav2vec2-base-960h"
    )]
    pub(crate) alignment_model: String,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    pub(crate) model_cache_only: bool,
    #[arg(long = "translation-model", visible_alias = "translation_model")]
    pub(crate) translation_model: Option<String>,
    #[arg(long = "translation-bundle", visible_alias = "translation_bundle")]
    pub(crate) translation_bundle: Option<PathBuf>,
    #[arg(
        long = "translation-source-language",
        visible_alias = "translation_source_language"
    )]
    pub(crate) translation_source_language: Option<String>,
    #[arg(
        long = "translation-target-language",
        visible_alias = "translation_target_language"
    )]
    pub(crate) translation_target_language: Option<String>,
    #[arg(
        long = "translation-max-new-tokens",
        visible_alias = "translation_max_new_tokens",
        default_value_t = 256
    )]
    pub(crate) translation_max_new_tokens: usize,
    #[arg(long = "interpolate-method", visible_alias = "interpolate_method", value_enum, default_value_t = CliAlignmentInterpolationMethod::Nearest)]
    pub(crate) interpolate_method: CliAlignmentInterpolationMethod,
    #[arg(
        long = "return-char-alignments",
        visible_alias = "return_char_alignments"
    )]
    pub(crate) return_char_alignments: bool,
    #[arg(long, visible_alias = "speaker_embedding_bundle")]
    pub(crate) speaker_embedding_bundle: Option<PathBuf>,
    #[arg(
        long = "speaker-assignment-policy",
        visible_alias = "speaker_assignment_policy",
        value_enum,
        default_value_t = CliAssignmentPolicy::Majority
    )]
    pub(crate) speaker_assignment_policy: CliAssignmentPolicy,
}

pub(crate) fn inspect_models_command(args: InspectModelsArgs) -> anyhow::Result<()> {
    let config = NativeWhisperxConfig {
        input: InputSource::Path {
            path: PathBuf::from("inspect-only.wav"),
        },
        asr: AsrConfig {
            model_id: args.model,
            compute_type: args.compute_type,
            whisper_bundle: args.whisper_bundle,
            model_dir: args.model_dir.clone(),
            model_cache_only: args.model_cache_only,
            task: if args.translation_model.is_some() || args.translation_bundle.is_some() {
                TranscriptionTask::Translate
            } else {
                TranscriptionTask::Transcribe
            },
            ..AsrConfig::default()
        },
        translation: translation_config(
            args.translation_model,
            args.translation_bundle,
            args.model_dir.clone(),
            args.model_cache_only,
            args.translation_source_language,
            args.translation_target_language,
            args.translation_max_new_tokens,
        ),
        vad: VadConfig::default(),
        alignment: alignment_config(
            args.no_align,
            args.alignment_model,
            args.alignment_bundle,
            args.model_dir,
            args.model_cache_only,
            args.interpolate_method,
            args.return_char_alignments,
        ),
        diarization: DiarizationConfig {
            enabled: args.speaker_embedding_bundle.is_some(),
            speaker_embedding_model_bundle: args.speaker_embedding_bundle,
            assignment_policy: args.speaker_assignment_policy.into(),
            ..DiarizationConfig::default()
        },
        output: OutputConfig::default(),
    };
    let request = build_transcription_request(&config)?;

    if config.translation.enabled {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "request": request,
                "translation": config.translation,
            }))?
        );
    } else {
        println!("{}", serde_json::to_string_pretty(&request)?);
    }
    Ok(())
}

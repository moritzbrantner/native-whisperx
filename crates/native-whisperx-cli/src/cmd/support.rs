//! Shared CLI command helpers for path, speaker, alignment, and task mapping.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use native_whisperx::{
    AlignmentConfig, AlignmentInterpolationMethod, OutputFormat, SpeakerDirectoryScope,
    SpeakerDirectorySelection, TranscriptionTask, TranslationConfig,
};

use crate::{
    CliAlignmentInterpolationMethod, CliDevicePreference, CliOutputFormat,
    CliSpeakerDirectoryScope, CliTask, SpeakerDirectoryArgs,
};

pub(crate) fn compatible_args() -> Vec<OsString> {
    let args = std::env::args_os().collect::<Vec<_>>();
    let Some(first_arg) = args.get(1).and_then(|arg| arg.to_str()) else {
        return args;
    };
    if first_arg.starts_with('-') || is_native_subcommand(first_arg) {
        return args;
    }

    let mut normalized = Vec::with_capacity(args.len() + 1);
    normalized.push(args[0].clone());
    normalized.push(OsString::from("transcribe"));
    normalized.extend(args.into_iter().skip(1));
    normalized
}

fn is_native_subcommand(value: &str) -> bool {
    matches!(
        value,
        "transcribe"
            | "live-transcribe"
            | "import-whisperx"
            | "speakers"
            | "inspect-models"
            | "parity"
            | "parity-fixtures"
            | "parity-bench"
            | "parity-summary"
            | "parity-preflight"
            | "parity-goldens"
            | "__parity-fixture-case"
            | "__parity-bench-case"
    )
}

pub(crate) fn resolve_cli_path_with_root(path: PathBuf, root: &Path) -> PathBuf {
    if path.is_relative() {
        root.join(path)
    } else {
        path
    }
}

pub(crate) fn absolute_from_cwd(path: PathBuf) -> anyhow::Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path);
    }
    Ok(std::env::current_dir()?.join(path))
}

impl From<CliOutputFormat> for OutputFormat {
    fn from(value: CliOutputFormat) -> Self {
        match value {
            CliOutputFormat::All => Self::All,
            CliOutputFormat::Json => Self::Json,
            CliOutputFormat::NativeJson => Self::NativeJson,
            CliOutputFormat::Srt => Self::Srt,
            CliOutputFormat::Vtt => Self::Vtt,
            CliOutputFormat::Txt => Self::Txt,
            CliOutputFormat::Tsv => Self::Tsv,
            CliOutputFormat::Aud => Self::Audacity,
        }
    }
}

impl From<CliTask> for TranscriptionTask {
    fn from(value: CliTask) -> Self {
        match value {
            CliTask::Transcribe => Self::Transcribe,
            CliTask::Translate => Self::Translate,
        }
    }
}

impl From<CliDevicePreference> for native_whisperx::DevicePreference {
    fn from(value: CliDevicePreference) -> Self {
        match value {
            CliDevicePreference::Auto => Self::Auto,
            CliDevicePreference::Cpu => Self::Cpu,
            CliDevicePreference::Cuda => Self::Cuda,
        }
    }
}

impl From<CliSpeakerDirectoryScope> for SpeakerDirectoryScope {
    fn from(value: CliSpeakerDirectoryScope) -> Self {
        match value {
            CliSpeakerDirectoryScope::Auto => Self::Auto,
            CliSpeakerDirectoryScope::Local => Self::Local,
            CliSpeakerDirectoryScope::Global => Self::Global,
        }
    }
}

impl TryFrom<SpeakerDirectoryArgs> for SpeakerDirectorySelection {
    type Error = anyhow::Error;

    fn try_from(value: SpeakerDirectoryArgs) -> anyhow::Result<Self> {
        validate_speaker_directory_args(&value)?;
        Ok(Self {
            scope: value.scope.into(),
            explicit_path: value.speaker_directory.or(value.speaker_store),
        })
    }
}

pub(crate) fn validate_speaker_directory_args(args: &SpeakerDirectoryArgs) -> anyhow::Result<()> {
    if let (Some(directory), Some(store)) = (&args.speaker_directory, &args.speaker_store) {
        let current_dir = std::env::current_dir()?;
        let directory = resolve_cli_path_with_root(directory.clone(), &current_dir);
        let store = resolve_cli_path_with_root(store.clone(), &current_dir);
        if directory != store {
            anyhow::bail!(
                "--speaker-directory and --speaker-store must point to the same path when both are provided"
            );
        }
    }
    Ok(())
}

pub(crate) fn alignment_config(
    no_align: bool,
    model_id: String,
    model_bundle: Option<PathBuf>,
    model_dir: Option<PathBuf>,
    model_cache_only: bool,
    interpolate_method: CliAlignmentInterpolationMethod,
    return_char_alignments: bool,
) -> AlignmentConfig {
    AlignmentConfig {
        enabled: !no_align,
        model_id,
        model_bundle,
        model_dir,
        model_cache_only,
        interpolate_method: interpolate_method.into(),
        return_char_alignments,
    }
}

pub(crate) fn translation_config(
    model_id: Option<String>,
    model_bundle: Option<PathBuf>,
    model_dir: Option<PathBuf>,
    model_cache_only: bool,
    source_language: Option<String>,
    target_language: Option<String>,
    max_new_tokens: usize,
) -> TranslationConfig {
    TranslationConfig {
        enabled: model_id.is_some() || model_bundle.is_some(),
        model_id: model_id.map(|model_id| normalize_translation_model_id(&model_id)),
        model_bundle,
        model_dir,
        model_cache_only,
        source_language,
        target_language,
        max_new_tokens,
    }
}

fn normalize_translation_model_id(model_id: &str) -> String {
    let trimmed = model_id.trim();
    match trimmed.to_ascii_lowercase().as_str() {
        "helsinki/opus-mt-de-en" | "opus-mt-de-en" | "helsinki:de-en" => {
            "Helsinki-NLP/opus-mt-de-en".to_string()
        }
        _ if trimmed.eq_ignore_ascii_case("Helsinki-NLP/opus-mt-de-en") => {
            "Helsinki-NLP/opus-mt-de-en".to_string()
        }
        _ => trimmed.to_string(),
    }
}

impl From<CliAlignmentInterpolationMethod> for AlignmentInterpolationMethod {
    fn from(value: CliAlignmentInterpolationMethod) -> Self {
        match value {
            CliAlignmentInterpolationMethod::Nearest => Self::Nearest,
            CliAlignmentInterpolationMethod::Linear => Self::Linear,
            CliAlignmentInterpolationMethod::Ignore => Self::Ignore,
        }
    }
}

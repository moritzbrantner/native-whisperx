use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::{Args, Parser, Subcommand, ValueEnum};
use native_whisperx::{
    build_transcription_request, compare_with_whisperx, import_whisperx_json, run, AlignmentConfig,
    AlignmentInterpolationMethod, AsrConfig, AsrProvider, DevicePreference, DiarizationConfig,
    ExternalWhisperxConfig, InputSource, NativeWhisperxConfig, OutputConfig, OutputFormat,
    ParityConfig, VadConfig,
};

#[derive(Debug, Parser)]
#[command(name = "native-whisperx")]
#[command(about = "WhisperX-style workflows composed from Rust building-block crates.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Transcribe(TranscribeArgs),
    ImportWhisperx(ImportWhisperxArgs),
    InspectModels(InspectModelsArgs),
    Parity(ParityArgs),
}

#[derive(Debug, Args)]
struct TranscribeArgs {
    input: PathBuf,
    #[arg(long, visible_alias = "whisper_bundle")]
    whisper_bundle: Option<PathBuf>,
    #[arg(long, default_value = "openai/whisper-large-v3-turbo")]
    model: String,
    #[arg(long)]
    language: Option<String>,
    #[arg(long, value_enum, default_value_t = CliDevicePreference::Auto)]
    device: CliDevicePreference,
    #[arg(long = "no-align", visible_alias = "no_align")]
    no_align: bool,
    #[arg(long, visible_alias = "alignment_bundle")]
    alignment_bundle: Option<PathBuf>,
    #[arg(
        long = "align-model",
        visible_alias = "align_model",
        default_value = "facebook/wav2vec2-base-960h"
    )]
    alignment_model: String,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    model_cache_only: bool,
    #[arg(long = "interpolate-method", visible_alias = "interpolate_method", value_enum, default_value_t = CliAlignmentInterpolationMethod::Nearest)]
    interpolate_method: CliAlignmentInterpolationMethod,
    #[arg(
        long = "return-char-alignments",
        visible_alias = "return_char_alignments"
    )]
    return_char_alignments: bool,
    #[arg(long, visible_alias = "speaker_embedding_bundle")]
    speaker_embedding_bundle: Option<PathBuf>,
    #[arg(long, visible_alias = "speaker_embedding_model_file")]
    speaker_embedding_model_file: Option<String>,
    #[arg(long, visible_alias = "speaker_embedding_dim")]
    speaker_embedding_dim: Option<usize>,
    #[arg(long, visible_alias = "speaker_embedding_sample_rate")]
    speaker_embedding_sample_rate: Option<u32>,
    #[arg(long, visible_alias = "min_speakers")]
    min_speakers: Option<usize>,
    #[arg(long, visible_alias = "max_speakers")]
    max_speakers: Option<usize>,
    #[arg(long, visible_alias = "output_dir")]
    output_dir: Option<PathBuf>,
    #[arg(long)]
    basename: Option<String>,
    #[arg(
        long = "format",
        visible_alias = "output_format",
        value_enum,
        default_values_t = [CliOutputFormat::Json]
    )]
    formats: Vec<CliOutputFormat>,
}

#[derive(Debug, Args)]
struct ImportWhisperxArgs {
    whisperx_json: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct InspectModelsArgs {
    #[arg(long, visible_alias = "whisper_bundle")]
    whisper_bundle: Option<PathBuf>,
    #[arg(long, default_value = "openai/whisper-large-v3-turbo")]
    model: String,
    #[arg(long = "no-align", visible_alias = "no_align")]
    no_align: bool,
    #[arg(long, visible_alias = "alignment_bundle")]
    alignment_bundle: Option<PathBuf>,
    #[arg(
        long = "align-model",
        visible_alias = "align_model",
        default_value = "facebook/wav2vec2-base-960h"
    )]
    alignment_model: String,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    model_cache_only: bool,
    #[arg(long = "interpolate-method", visible_alias = "interpolate_method", value_enum, default_value_t = CliAlignmentInterpolationMethod::Nearest)]
    interpolate_method: CliAlignmentInterpolationMethod,
    #[arg(
        long = "return-char-alignments",
        visible_alias = "return_char_alignments"
    )]
    return_char_alignments: bool,
    #[arg(long, visible_alias = "speaker_embedding_bundle")]
    speaker_embedding_bundle: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct ParityArgs {
    input: PathBuf,
    #[arg(long, visible_alias = "whisperx_command")]
    whisperx_command: Option<PathBuf>,
    #[arg(long, visible_alias = "whisperx_model", default_value = "large-v2")]
    whisperx_model: String,
    #[arg(long, visible_alias = "expected_json")]
    expected_json: Option<PathBuf>,
    #[arg(long, visible_alias = "whisper_bundle")]
    whisper_bundle: Option<PathBuf>,
    #[arg(long, default_value = "openai/whisper-large-v3-turbo")]
    model: String,
    #[arg(long, value_enum, default_value_t = CliDevicePreference::Auto)]
    device: CliDevicePreference,
    #[arg(long = "no-align", visible_alias = "no_align")]
    no_align: bool,
    #[arg(long, visible_alias = "alignment_bundle")]
    alignment_bundle: Option<PathBuf>,
    #[arg(
        long = "align-model",
        visible_alias = "align_model",
        default_value = "facebook/wav2vec2-base-960h"
    )]
    alignment_model: String,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    model_cache_only: bool,
    #[arg(long = "interpolate-method", visible_alias = "interpolate_method", value_enum, default_value_t = CliAlignmentInterpolationMethod::Nearest)]
    interpolate_method: CliAlignmentInterpolationMethod,
    #[arg(
        long = "return-char-alignments",
        visible_alias = "return_char_alignments"
    )]
    return_char_alignments: bool,
    #[arg(long, visible_alias = "speaker_embedding_bundle")]
    speaker_embedding_bundle: Option<PathBuf>,
    #[arg(long, visible_alias = "min_speakers")]
    min_speakers: Option<usize>,
    #[arg(long, visible_alias = "max_speakers")]
    max_speakers: Option<usize>,
    #[arg(long)]
    language: Option<String>,
    #[arg(long, visible_alias = "output_dir")]
    output_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliOutputFormat {
    Json,
    NativeJson,
    Srt,
    Vtt,
    Txt,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliDevicePreference {
    Auto,
    Cpu,
    Cuda,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliAlignmentInterpolationMethod {
    Nearest,
    Linear,
    Ignore,
}

fn main() -> anyhow::Result<()> {
    match Cli::parse_from(compatible_args()).command {
        Command::Transcribe(args) => transcribe_command(args),
        Command::ImportWhisperx(args) => import_whisperx_command(args),
        Command::InspectModels(args) => inspect_models_command(args),
        Command::Parity(args) => parity_command(args),
    }
}

fn compatible_args() -> Vec<OsString> {
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
        "transcribe" | "import-whisperx" | "inspect-models" | "parity"
    )
}

fn transcribe_command(args: TranscribeArgs) -> anyhow::Result<()> {
    let report = run(NativeWhisperxConfig {
        input: InputSource::Path { path: args.input },
        asr: AsrConfig {
            provider: AsrProvider::Native,
            model_id: args.model,
            language: args.language,
            whisper_bundle: args.whisper_bundle,
            device: args.device.into(),
            ..AsrConfig::default()
        },
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
            enabled: args.speaker_embedding_bundle.is_some()
                || args.min_speakers.is_some()
                || args.max_speakers.is_some(),
            speaker_embedding_model_bundle: args.speaker_embedding_bundle,
            speaker_embedding_model_file: args.speaker_embedding_model_file,
            speaker_embedding_dimension: args.speaker_embedding_dim,
            speaker_embedding_sample_rate: args.speaker_embedding_sample_rate,
            min_speakers: args.min_speakers,
            max_speakers: args.max_speakers,
            ..DiarizationConfig::default()
        },
        output: OutputConfig {
            output_dir: args.output_dir,
            formats: args.formats.into_iter().map(Into::into).collect(),
            basename: args.basename,
            pretty_json: true,
        },
    })?;

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn import_whisperx_command(args: ImportWhisperxArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.whisperx_json)
        .with_context(|| format!("failed to read {}", args.whisperx_json.display()))?;
    let transcript = import_whisperx_json(&bytes)?;
    let json = serde_json::to_string_pretty(&transcript)?;
    if let Some(output) = args.output {
        fs::write(&output, json)
            .with_context(|| format!("failed to write {}", output.display()))?;
    } else {
        println!("{json}");
    }
    Ok(())
}

fn inspect_models_command(args: InspectModelsArgs) -> anyhow::Result<()> {
    let request = build_transcription_request(&NativeWhisperxConfig {
        input: InputSource::Path {
            path: PathBuf::from("inspect-only.wav"),
        },
        asr: AsrConfig {
            model_id: args.model,
            whisper_bundle: args.whisper_bundle,
            ..AsrConfig::default()
        },
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
            ..DiarizationConfig::default()
        },
        output: OutputConfig::default(),
    })?;

    println!("{}", serde_json::to_string_pretty(&request)?);
    Ok(())
}

fn parity_command(args: ParityArgs) -> anyhow::Result<()> {
    let report = compare_with_whisperx(ParityConfig {
        input: args.input,
        expected_json: args.expected_json,
        native_asr: AsrConfig {
            provider: AsrProvider::Native,
            model_id: args.model,
            whisper_bundle: args.whisper_bundle,
            device: args.device.into(),
            ..AsrConfig::default()
        },
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
            enabled: args.speaker_embedding_bundle.is_some()
                || args.min_speakers.is_some()
                || args.max_speakers.is_some(),
            speaker_embedding_model_bundle: args.speaker_embedding_bundle,
            min_speakers: args.min_speakers,
            max_speakers: args.max_speakers,
            ..DiarizationConfig::default()
        },
        whisperx: ExternalWhisperxConfig {
            command: args
                .whisperx_command
                .unwrap_or_else(|| PathBuf::from("whisperx")),
            model: args.whisperx_model,
            output_dir: args.output_dir.clone(),
            ..ExternalWhisperxConfig::default()
        },
        language: args.language,
        output: OutputConfig {
            output_dir: args.output_dir,
            formats: vec![OutputFormat::Json],
            basename: Some("whisperx-parity".to_string()),
            pretty_json: true,
        },
    })?;

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

impl From<CliOutputFormat> for OutputFormat {
    fn from(value: CliOutputFormat) -> Self {
        match value {
            CliOutputFormat::Json => Self::Json,
            CliOutputFormat::NativeJson => Self::NativeJson,
            CliOutputFormat::Srt => Self::Srt,
            CliOutputFormat::Vtt => Self::Vtt,
            CliOutputFormat::Txt => Self::Txt,
        }
    }
}

impl From<CliDevicePreference> for DevicePreference {
    fn from(value: CliDevicePreference) -> Self {
        match value {
            CliDevicePreference::Auto => Self::Auto,
            CliDevicePreference::Cpu => Self::Cpu,
            CliDevicePreference::Cuda => Self::Cuda,
        }
    }
}

fn alignment_config(
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

impl From<CliAlignmentInterpolationMethod> for AlignmentInterpolationMethod {
    fn from(value: CliAlignmentInterpolationMethod) -> Self {
        match value {
            CliAlignmentInterpolationMethod::Nearest => Self::Nearest,
            CliAlignmentInterpolationMethod::Linear => Self::Linear,
            CliAlignmentInterpolationMethod::Ignore => Self::Ignore,
        }
    }
}

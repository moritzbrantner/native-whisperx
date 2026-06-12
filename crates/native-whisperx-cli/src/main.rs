use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::{Args, Parser, Subcommand, ValueEnum};
use native_whisperx::{
    build_transcription_request, compare_with_whisperx, import_whisperx_json, run, AlignmentConfig,
    AsrConfig, AsrProvider, DevicePreference, DiarizationConfig, ExternalWhisperxConfig,
    InputSource, NativeWhisperxConfig, OutputConfig, OutputFormat, ParityConfig, VadConfig,
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
    #[arg(long)]
    whisper_bundle: Option<PathBuf>,
    #[arg(long, default_value = "openai/whisper-large-v3-turbo")]
    model: String,
    #[arg(long)]
    language: Option<String>,
    #[arg(long, value_enum, default_value_t = CliDevicePreference::Auto)]
    device: CliDevicePreference,
    #[arg(long)]
    alignment_bundle: Option<PathBuf>,
    #[arg(long, default_value = "facebook/wav2vec2-base-960h")]
    alignment_model: String,
    #[arg(long)]
    speaker_embedding_bundle: Option<PathBuf>,
    #[arg(long)]
    speaker_embedding_model_file: Option<String>,
    #[arg(long)]
    speaker_embedding_dim: Option<usize>,
    #[arg(long)]
    speaker_embedding_sample_rate: Option<u32>,
    #[arg(long)]
    min_speakers: Option<usize>,
    #[arg(long)]
    max_speakers: Option<usize>,
    #[arg(long)]
    output_dir: Option<PathBuf>,
    #[arg(long)]
    basename: Option<String>,
    #[arg(long = "format", value_enum, default_values_t = [CliOutputFormat::Json])]
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
    #[arg(long)]
    whisper_bundle: Option<PathBuf>,
    #[arg(long, default_value = "openai/whisper-large-v3-turbo")]
    model: String,
    #[arg(long)]
    alignment_bundle: Option<PathBuf>,
    #[arg(long, default_value = "facebook/wav2vec2-base-960h")]
    alignment_model: String,
    #[arg(long)]
    speaker_embedding_bundle: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct ParityArgs {
    input: PathBuf,
    #[arg(long)]
    whisperx_command: Option<PathBuf>,
    #[arg(long, default_value = "large-v2")]
    whisperx_model: String,
    #[arg(long)]
    expected_json: Option<PathBuf>,
    #[arg(long)]
    language: Option<String>,
    #[arg(long)]
    output_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliOutputFormat {
    Json,
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

fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Command::Transcribe(args) => transcribe_command(args),
        Command::ImportWhisperx(args) => import_whisperx_command(args),
        Command::InspectModels(args) => inspect_models_command(args),
        Command::Parity(args) => parity_command(args),
    }
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
        alignment: AlignmentConfig {
            enabled: args.alignment_bundle.is_some(),
            model_id: args.alignment_model,
            model_bundle: args.alignment_bundle,
        },
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
        alignment: AlignmentConfig {
            enabled: args.alignment_bundle.is_some(),
            model_id: args.alignment_model,
            model_bundle: args.alignment_bundle,
        },
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

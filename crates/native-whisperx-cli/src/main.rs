use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use native_whisperx::{
    build_transcription_request, compare_with_whisperx, import_whisperx_json, run_many,
    AlignmentConfig, AlignmentInterpolationMethod, AsrConfig, AsrProvider, DevicePreference,
    DiarizationConfig, ExternalWhisperxConfig, InputSource, NativeWhisperxConfig, OutputConfig,
    OutputFormat, ParityConfig, SegmentResolution, SubtitleConfig, TranscriptionTask, VadConfig,
    VadMethod, WhisperxDecodeConfig,
};

#[derive(Debug, Parser)]
#[command(name = "native-whisperx")]
#[command(version)]
#[command(about = "WhisperX-style workflows composed from Rust building-block crates.")]
struct Cli {
    #[arg(short = 'P', long = "python-version", action = ArgAction::SetTrue)]
    python_version: bool,
    #[command(subcommand)]
    command: Option<Command>,
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
    #[arg(required = true)]
    input: Vec<PathBuf>,
    #[arg(long, value_enum, default_value_t = CliProvider::Native)]
    provider: CliProvider,
    #[arg(long, visible_alias = "whisper_bundle")]
    whisper_bundle: Option<PathBuf>,
    #[arg(long, default_value = "small")]
    model: String,
    #[arg(long, value_enum, default_value_t = CliTask::Transcribe)]
    task: CliTask,
    #[arg(long)]
    language: Option<String>,
    #[arg(long, value_enum, default_value_t = CliDevicePreference::Auto)]
    device: CliDevicePreference,
    #[arg(long, visible_alias = "device_index")]
    device_index: Option<String>,
    #[arg(long, visible_alias = "batch_size")]
    batch_size: Option<usize>,
    #[arg(long, visible_alias = "compute_type")]
    compute_type: Option<String>,
    #[arg(long, num_args = 0..=1, default_missing_value = "true")]
    verbose: Option<String>,
    #[arg(long = "log-level", visible_alias = "log_level")]
    log_level: Option<String>,
    #[arg(long = "print-progress", visible_alias = "print_progress", action = ArgAction::SetTrue)]
    print_progress: bool,
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
    #[arg(long, action = ArgAction::SetTrue)]
    diarize: bool,
    #[arg(
        long,
        visible_alias = "diarize_model",
        default_value = "pyannote/speaker-diarization-community-1"
    )]
    diarize_model: String,
    #[arg(long, visible_alias = "speaker_embeddings", action = ArgAction::SetTrue)]
    speaker_embeddings: bool,
    #[arg(long, visible_alias = "hf_token")]
    hf_token: Option<String>,
    #[arg(long, visible_alias = "min_speakers")]
    min_speakers: Option<usize>,
    #[arg(long, visible_alias = "max_speakers")]
    max_speakers: Option<usize>,
    #[arg(long, short = 'o', visible_alias = "output_dir")]
    output_dir: Option<PathBuf>,
    #[arg(long)]
    basename: Option<String>,
    #[arg(
        long = "format",
        short = 'f',
        alias = "output-format",
        visible_alias = "output_format",
        value_enum,
        default_values_t = [CliOutputFormat::Json]
    )]
    formats: Vec<CliOutputFormat>,
    #[arg(long, visible_alias = "vad_method", value_enum, default_value_t = CliVadMethod::Energy)]
    vad_method: CliVadMethod,
    #[arg(long, visible_alias = "vad_onset")]
    vad_onset: Option<f32>,
    #[arg(long, visible_alias = "vad_offset")]
    vad_offset: Option<f32>,
    #[arg(long, visible_alias = "chunk_size")]
    chunk_size: Option<f64>,
    #[arg(long = "vad-model-bundle", visible_alias = "vad_model_bundle")]
    vad_model_bundle: Option<PathBuf>,
    #[arg(long = "vad-model-file", visible_alias = "vad_model_file")]
    vad_model_file: Option<String>,
    #[arg(long = "vad-input-name", visible_alias = "vad_input_name")]
    vad_input_name: Option<String>,
    #[arg(long = "vad-output-name", visible_alias = "vad_output_name")]
    vad_output_name: Option<String>,
    #[arg(long, value_delimiter = ',')]
    temperature: Vec<f32>,
    #[arg(long, visible_alias = "best_of")]
    best_of: Option<usize>,
    #[arg(long, visible_alias = "beam_size")]
    beam_size: Option<usize>,
    #[arg(long)]
    patience: Option<f32>,
    #[arg(long, visible_alias = "length_penalty")]
    length_penalty: Option<f32>,
    #[arg(long, visible_alias = "suppress_tokens")]
    suppress_tokens: Option<String>,
    #[arg(long, visible_alias = "suppress_numerals", action = ArgAction::SetTrue)]
    suppress_numerals: bool,
    #[arg(long, visible_alias = "initial_prompt")]
    initial_prompt: Option<String>,
    #[arg(long)]
    hotwords: Option<String>,
    #[arg(long, visible_alias = "condition_on_previous_text")]
    condition_on_previous_text: Option<bool>,
    #[arg(long)]
    fp16: Option<bool>,
    #[arg(long, visible_alias = "compression_ratio_threshold")]
    compression_ratio_threshold: Option<f32>,
    #[arg(long, visible_alias = "logprob_threshold")]
    logprob_threshold: Option<f32>,
    #[arg(long, visible_alias = "no_speech_threshold")]
    no_speech_threshold: Option<f32>,
    #[arg(long)]
    threads: Option<usize>,
    #[arg(long, visible_alias = "max_line_width")]
    max_line_width: Option<usize>,
    #[arg(long, visible_alias = "max_line_count")]
    max_line_count: Option<usize>,
    #[arg(long, visible_alias = "highlight_words", action = ArgAction::SetTrue)]
    highlight_words: bool,
    #[arg(long, visible_alias = "segment_resolution", value_enum, default_value_t = CliSegmentResolution::Segment)]
    segment_resolution: CliSegmentResolution,
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
    #[arg(long, default_value = "small")]
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
    #[arg(long, visible_alias = "whisperx_model", default_value = "small")]
    whisperx_model: String,
    #[arg(long, visible_alias = "expected_json")]
    expected_json: Option<PathBuf>,
    #[arg(long, visible_alias = "whisper_bundle")]
    whisper_bundle: Option<PathBuf>,
    #[arg(long, default_value = "small")]
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
    All,
    Json,
    NativeJson,
    Srt,
    Vtt,
    Txt,
    Tsv,
    Aud,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliProvider {
    Native,
    ExternalWhisperx,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliTask {
    Transcribe,
    Translate,
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

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliVadMethod {
    Energy,
    Pyannote,
    Silero,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliSegmentResolution {
    Segment,
    Chunk,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_from(compatible_args());
    if cli.python_version {
        println!(
            "native-whisperx {} (Rust runtime)",
            env!("CARGO_PKG_VERSION")
        );
        return Ok(());
    }
    match cli.command {
        Some(Command::Transcribe(args)) => transcribe_command(args),
        Some(Command::ImportWhisperx(args)) => import_whisperx_command(args),
        Some(Command::InspectModels(args)) => inspect_models_command(args),
        Some(Command::Parity(args)) => parity_command(args),
        None => {
            Cli::parse_from([OsString::from("native-whisperx"), OsString::from("--help")]);
            Ok(())
        }
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
    validate_transcribe_args(&args)?;
    let configs = args
        .input
        .iter()
        .cloned()
        .map(|input| transcribe_config(&args, input))
        .collect::<Vec<_>>();
    let reports = run_many(configs)?;

    if reports.len() == 1 {
        println!("{}", serde_json::to_string_pretty(&reports[0])?);
    } else {
        println!("{}", serde_json::to_string_pretty(&reports)?);
    }
    Ok(())
}

fn validate_transcribe_args(args: &TranscribeArgs) -> anyhow::Result<()> {
    let subtitle_layout_requested =
        args.highlight_words || args.max_line_width.is_some() || args.max_line_count.is_some();
    if args.no_align && subtitle_layout_requested {
        anyhow::bail!(
            "--highlight_words, --max_line_width, and --max_line_count require alignment; remove --no_align"
        );
    }
    if args.task == CliTask::Translate && args.provider == CliProvider::Native && !args.no_align {
        anyhow::bail!(
            "--task translate is not supported by native alignment yet; pass --no_align or use --provider external-whisperx"
        );
    }
    if args.speaker_embeddings && !args.diarize && args.provider == CliProvider::Native {
        anyhow::bail!("--speaker_embeddings requires --diarize in native mode");
    }
    if args.basename.is_some() && args.input.len() > 1 {
        anyhow::bail!("--basename cannot be used with multiple input files");
    }
    Ok(())
}

fn transcribe_config(args: &TranscribeArgs, input: PathBuf) -> NativeWhisperxConfig {
    let output_dir = args.output_dir.clone();
    let provider = match args.provider {
        CliProvider::Native => AsrProvider::Native,
        CliProvider::ExternalWhisperx => AsrProvider::ExternalWhisperX,
    };
    let diarize = args.diarize
        || args.speaker_embeddings
        || args.speaker_embedding_bundle.is_some()
        || args.min_speakers.is_some()
        || args.max_speakers.is_some();

    NativeWhisperxConfig {
        input: InputSource::Path { path: input },
        asr: AsrConfig {
            provider,
            task: args.task.into(),
            model_id: args.model.clone(),
            language: args.language.clone(),
            whisper_bundle: args.whisper_bundle.clone(),
            device: args.device.into(),
            device_index: args.device_index.clone(),
            compute_type: args.compute_type.clone(),
            batch_chunks: true,
            max_batch_size: args.batch_size,
            decode: decode_config(args),
            external_whisperx: ExternalWhisperxConfig {
                model: args.model.clone(),
                output_dir: output_dir.clone(),
                extra_args: logging_extra_args(args),
                ..ExternalWhisperxConfig::default()
            },
        },
        vad: VadConfig {
            method: args.vad_method.into(),
            onset: args.vad_onset,
            offset: args.vad_offset,
            chunk_size: args.chunk_size,
            model_bundle: args.vad_model_bundle.clone(),
            model_file: args.vad_model_file.clone(),
            input_name: args.vad_input_name.clone(),
            output_name: args.vad_output_name.clone(),
            ..VadConfig::default()
        },
        alignment: alignment_config(
            args.no_align
                || args.task == CliTask::Translate && args.provider == CliProvider::Native,
            args.alignment_model.clone(),
            args.alignment_bundle.clone(),
            args.model_dir.clone(),
            args.model_cache_only,
            args.interpolate_method,
            args.return_char_alignments,
        ),
        diarization: DiarizationConfig {
            enabled: diarize,
            model_id: args.diarize_model.clone(),
            hf_token: args.hf_token.clone(),
            return_speaker_embeddings: args.speaker_embeddings,
            speaker_embedding_model_bundle: args.speaker_embedding_bundle.clone(),
            speaker_embedding_model_file: args.speaker_embedding_model_file.clone(),
            speaker_embedding_dimension: args.speaker_embedding_dim,
            speaker_embedding_sample_rate: args.speaker_embedding_sample_rate,
            min_speakers: args.min_speakers,
            max_speakers: args.max_speakers,
            ..DiarizationConfig::default()
        },
        output: OutputConfig {
            output_dir,
            formats: args.formats.iter().copied().map(Into::into).collect(),
            basename: args.basename.clone(),
            pretty_json: true,
            subtitles: SubtitleConfig {
                max_line_width: args.max_line_width,
                max_line_count: args.max_line_count,
                highlight_words: args.highlight_words,
                segment_resolution: args.segment_resolution.into(),
            },
        },
    }
}

fn decode_config(args: &TranscribeArgs) -> WhisperxDecodeConfig {
    WhisperxDecodeConfig {
        temperature: args.temperature.clone(),
        best_of: args.best_of,
        beam_size: args.beam_size,
        patience: args.patience,
        length_penalty: args.length_penalty,
        suppress_tokens: args.suppress_tokens.clone(),
        suppress_numerals: args.suppress_numerals,
        initial_prompt: args.initial_prompt.clone(),
        hotwords: args.hotwords.clone(),
        condition_on_previous_text: args.condition_on_previous_text,
        fp16: args.fp16,
        compression_ratio_threshold: args.compression_ratio_threshold,
        logprob_threshold: args.logprob_threshold,
        no_speech_threshold: args.no_speech_threshold,
        threads: args.threads,
    }
}

fn logging_extra_args(args: &TranscribeArgs) -> Vec<String> {
    let mut extra_args = Vec::new();
    if let Some(verbose) = &args.verbose {
        extra_args.extend(["--verbose".to_string(), verbose.clone()]);
    }
    if let Some(log_level) = &args.log_level {
        extra_args.extend(["--log-level".to_string(), log_level.clone()]);
    }
    if args.print_progress {
        extra_args.push("--print_progress".to_string());
    }
    extra_args
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
            subtitles: SubtitleConfig::default(),
        },
    })?;

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
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

impl From<CliDevicePreference> for DevicePreference {
    fn from(value: CliDevicePreference) -> Self {
        match value {
            CliDevicePreference::Auto => Self::Auto,
            CliDevicePreference::Cpu => Self::Cpu,
            CliDevicePreference::Cuda => Self::Cuda,
        }
    }
}

impl From<CliVadMethod> for VadMethod {
    fn from(value: CliVadMethod) -> Self {
        match value {
            CliVadMethod::Energy => Self::Energy,
            CliVadMethod::Pyannote => Self::Pyannote,
            CliVadMethod::Silero => Self::Silero,
        }
    }
}

impl From<CliSegmentResolution> for SegmentResolution {
    fn from(value: CliSegmentResolution) -> Self {
        match value {
            CliSegmentResolution::Segment => Self::Segment,
            CliSegmentResolution::Chunk => Self::Chunk,
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

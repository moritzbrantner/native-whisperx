use std::collections::HashSet;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

use anyhow::Context;
use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use native_whisperx::{
    build_transcription_request, compare_with_whisperx, import_whisperx_json, run_many,
    run_parity_fixture_suite, run_parity_preflight, AlignmentConfig, AlignmentInterpolationMethod,
    AsrConfig, AsrProvider, DevicePreference, DiarizationConfig, ExpectedOutputFile,
    ExternalWhisperxConfig, InputSource, NativeWhisperxConfig, OutputConfig, OutputFormat,
    ParityConfig, ParityFixtureCase, ParityFixtureSuite, SegmentResolution, SubtitleConfig,
    TranscriptionTask, TranslationConfig, VadConfig, VadMethod, WhisperxDecodeConfig,
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
    ParityFixtures(ParityFixturesArgs),
    ParityPreflight(ParityPreflightArgs),
    ParityGoldens(ParityGoldensArgs),
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
    #[arg(long = "translation-model", visible_alias = "translation_model")]
    translation_model: Option<String>,
    #[arg(long = "translation-bundle", visible_alias = "translation_bundle")]
    translation_bundle: Option<PathBuf>,
    #[arg(
        long = "translation-source-language",
        visible_alias = "translation_source_language"
    )]
    translation_source_language: Option<String>,
    #[arg(
        long = "translation-target-language",
        visible_alias = "translation_target_language"
    )]
    translation_target_language: Option<String>,
    #[arg(
        long = "translation-max-new-tokens",
        visible_alias = "translation_max_new_tokens",
        default_value_t = 256
    )]
    translation_max_new_tokens: usize,
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
    #[arg(long, visible_alias = "segment_resolution", value_enum, default_value_t = CliSegmentResolution::Sentence)]
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
    #[arg(long = "translation-model", visible_alias = "translation_model")]
    translation_model: Option<String>,
    #[arg(long = "translation-bundle", visible_alias = "translation_bundle")]
    translation_bundle: Option<PathBuf>,
    #[arg(
        long = "translation-source-language",
        visible_alias = "translation_source_language"
    )]
    translation_source_language: Option<String>,
    #[arg(
        long = "translation-target-language",
        visible_alias = "translation_target_language"
    )]
    translation_target_language: Option<String>,
    #[arg(
        long = "translation-max-new-tokens",
        visible_alias = "translation_max_new_tokens",
        default_value_t = 256
    )]
    translation_max_new_tokens: usize,
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

#[derive(Debug, Args)]
struct ParityFixturesArgs {
    manifest: PathBuf,
    #[arg(long)]
    root: Option<PathBuf>,
    #[arg(long, visible_alias = "whisperx_command")]
    whisperx_command: Option<PathBuf>,
    #[arg(long = "output-dir", visible_alias = "output_dir")]
    output_dir: Option<PathBuf>,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    model_cache_only: bool,
}

#[derive(Debug, Args)]
struct ParityPreflightArgs {
    manifest: PathBuf,
    #[arg(long)]
    root: Option<PathBuf>,
    #[arg(
        long,
        visible_alias = "whisperx_command",
        default_value = ".audio-tools/whisperx-venv/bin/whisperx"
    )]
    whisperx_command: PathBuf,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    model_dir: Option<PathBuf>,
    #[arg(long = "require-expected", visible_alias = "require_expected")]
    require_expected: bool,
    #[arg(long = "include-non-gating", visible_alias = "include_non_gating")]
    include_non_gating: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct ParityGoldensArgs {
    manifest: PathBuf,
    #[arg(long)]
    root: Option<PathBuf>,
    #[arg(
        long,
        visible_alias = "whisperx_command",
        default_value = ".audio-tools/whisperx-venv/bin/whisperx"
    )]
    whisperx_command: PathBuf,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    model_cache_only: bool,
    #[arg(long = "case")]
    cases: Vec<String>,
    #[arg(long = "include-non-gating", visible_alias = "include_non_gating")]
    include_non_gating: bool,
    #[arg(long)]
    overwrite: bool,
    #[arg(long = "dry-run", visible_alias = "dry_run")]
    dry_run: bool,
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
    #[value(alias = "segment")]
    Sentence,
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
        Some(Command::ParityFixtures(args)) => parity_fixtures_command(args),
        Some(Command::ParityPreflight(args)) => parity_preflight_command(args),
        Some(Command::ParityGoldens(args)) => parity_goldens_command(args),
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
        "transcribe"
            | "import-whisperx"
            | "inspect-models"
            | "parity"
            | "parity-fixtures"
            | "parity-preflight"
            | "parity-goldens"
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
    if args.task == CliTask::Translate
        && args.provider == CliProvider::Native
        && !args.no_align
        && args.translation_model.is_none()
        && args.translation_bundle.is_none()
    {
        anyhow::bail!(
            "--task translate is not supported with native alignment yet; pass --no-align or use --provider external-whisperx"
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
            model_dir: args.model_dir.clone(),
            model_cache_only: args.model_cache_only,
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
        translation: translation_config(
            args.translation_model.clone(),
            args.translation_bundle.clone(),
            args.model_dir.clone(),
            args.model_cache_only,
            args.translation_source_language.clone(),
            args.translation_target_language.clone(),
            args.translation_max_new_tokens,
        ),
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
                || args.task == CliTask::Translate
                    && args.provider == CliProvider::Native
                    && args.translation_model.is_none()
                    && args.translation_bundle.is_none(),
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
            model_dir: args.model_dir.clone(),
            model_cache_only: args.model_cache_only,
            device: args.device.into(),
            ..AsrConfig::default()
        },
        translation: TranslationConfig::default(),
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

fn parity_fixtures_command(args: ParityFixturesArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.manifest)
        .with_context(|| format!("failed to read {}", args.manifest.display()))?;
    let mut suite: ParityFixtureSuite = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.manifest.display()))?;
    let root = args
        .root
        .or_else(smoke_root_from_env_or_dotenv)
        .with_context(|| {
            "parity-fixtures requires --root, SMOKE_ROOT, or SMOKE_ROOT in .env for local audio, expected JSON, and model cache paths"
        })?;
    let root = absolute_from_cwd(root)?;
    let whisperx_command = args.whisperx_command.map(absolute_from_cwd).transpose()?;
    let output_dir = args.output_dir.map(absolute_from_cwd).transpose()?;
    let model_dir = args.model_dir.map(absolute_from_cwd).transpose()?;

    for fixture in &mut suite.fixtures {
        if let Some(command) = &whisperx_command {
            fixture.whisperx.command = command.clone();
        }
        if let Some(output_dir) = &output_dir {
            fixture.output.output_dir = Some(output_dir.join(&fixture.name));
        }
        if let Some(model_dir) = &model_dir {
            fixture.native_asr.model_dir = Some(model_dir.clone());
            fixture.alignment.model_dir = Some(model_dir.clone());
        }
        if args.model_cache_only {
            fixture.native_asr.model_cache_only = true;
            fixture.alignment.model_cache_only = true;
        }
    }

    let report = run_parity_fixture_suite(suite, Some(&root))?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    if !report.passed {
        anyhow::bail!("one or more parity fixtures failed");
    }
    Ok(())
}

fn parity_preflight_command(args: ParityPreflightArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.manifest)
        .with_context(|| format!("failed to read {}", args.manifest.display()))?;
    let suite: ParityFixtureSuite = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.manifest.display()))?;
    let root = smoke_root_or_arg(args.root, "parity-preflight")?;
    let manifest = absolute_from_cwd(args.manifest)?;
    let whisperx_command = absolute_from_cwd(args.whisperx_command)?;
    let model_dir = args
        .model_dir
        .map(absolute_from_cwd)
        .transpose()?
        .unwrap_or_else(|| root.join("models"));

    let report = run_parity_preflight(
        suite,
        manifest,
        root,
        whisperx_command,
        model_dir,
        args.require_expected,
        args.include_non_gating,
    );
    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_preflight_report(&report);
    }
    if !report.passed {
        anyhow::bail!("parity preflight failed");
    }
    Ok(())
}

fn parity_goldens_command(args: ParityGoldensArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.manifest)
        .with_context(|| format!("failed to read {}", args.manifest.display()))?;
    let mut suite: ParityFixtureSuite = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.manifest.display()))?;
    let root = smoke_root_or_arg(args.root, "parity-goldens")?;
    let whisperx_command = absolute_from_cwd(args.whisperx_command)?;
    let model_dir = args
        .model_dir
        .map(absolute_from_cwd)
        .transpose()?
        .unwrap_or_else(|| root.join("models"));
    let filters = args.cases.iter().cloned().collect::<HashSet<_>>();
    let mut selected = Vec::new();

    for mut fixture in suite.fixtures.drain(..) {
        if !args.include_non_gating && !fixture.gating {
            continue;
        }
        if !filters.is_empty() && !filters.contains(&fixture.name) {
            continue;
        }
        if fixture.expected_json.is_none() && fixture.expected_outputs.is_empty() {
            continue;
        }
        fixture.input = resolve_cli_path_with_root(fixture.input, &root);
        fixture.expected_json = fixture
            .expected_json
            .take()
            .map(|path| resolve_cli_path_with_root(path, &root));
        for output in &mut fixture.expected_outputs {
            output.path = resolve_cli_path_with_root(output.path.clone(), &root);
        }
        selected.push(fixture);
    }

    for case_name in &filters {
        if !suite_case_name_exists(&selected, case_name) {
            anyhow::bail!("no golden-generating case named `{case_name}` matched the manifest");
        }
    }

    if selected.is_empty() {
        println!("No golden-generating cases matched.");
        return Ok(());
    }

    for fixture in selected {
        let plan = build_golden_plan(
            &fixture,
            &root,
            &whisperx_command,
            &model_dir,
            args.model_cache_only,
        )?;
        ensure_golden_targets_can_write(&plan, args.overwrite, args.dry_run)?;
        if args.dry_run {
            print_golden_plan(&plan);
            continue;
        }
        fs::create_dir_all(&plan.generated_dir)
            .with_context(|| format!("failed to create {}", plan.generated_dir.display()))?;
        let status = ProcessCommand::new(&plan.command)
            .args(&plan.args)
            .status()
            .with_context(|| format!("failed to run {}", plan.command.display()))?;
        if !status.success() {
            anyhow::bail!(
                "WhisperX golden generation for `{}` failed with status {status}",
                fixture.name
            );
        }
        copy_golden_outputs(&plan, args.overwrite)?;
    }

    Ok(())
}

fn smoke_root_or_arg(root: Option<PathBuf>, command: &str) -> anyhow::Result<PathBuf> {
    let root = root
        .or_else(smoke_root_from_env_or_dotenv)
        .with_context(|| {
            format!("{command} requires --root, SMOKE_ROOT, or SMOKE_ROOT in .env for local audio, expected JSON, and model cache paths")
        })?;
    absolute_from_cwd(root)
}

fn smoke_root_from_env_or_dotenv() -> Option<PathBuf> {
    std::env::var_os("SMOKE_ROOT")
        .map(PathBuf::from)
        .or_else(|| dotenv_value("SMOKE_ROOT").map(PathBuf::from))
}

fn dotenv_value(key: &str) -> Option<String> {
    let contents = fs::read_to_string(".env").ok()?;
    for line in contents.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let trimmed = trimmed.strip_prefix("export ").unwrap_or(trimmed);
        let Some((candidate, value)) = trimmed.split_once('=') else {
            continue;
        };
        if candidate.trim() != key {
            continue;
        }
        let value = value.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .or_else(|| {
                value
                    .strip_prefix('\'')
                    .and_then(|value| value.strip_suffix('\''))
            })
            .unwrap_or(value);
        if value.is_empty() {
            return None;
        }
        return Some(value.to_string());
    }
    None
}

fn print_preflight_report(report: &native_whisperx::ParityPreflightReport) {
    println!(
        "Parity preflight: {}",
        if report.passed { "passed" } else { "failed" }
    );
    println!("manifest: {}", report.manifest.display());
    println!("root: {}", report.root.display());
    println!("whisperx command: {}", report.whisperx_command.display());
    println!("model dir: {}", report.model_dir.display());
    println!(
        "source checkout tag: {}",
        report.source_checkout_tag.as_deref().unwrap_or("<missing>")
    );
    for case in &report.cases {
        println!(
            "{} [{}]: {}",
            case.name,
            if case.gating { "gating" } else { "non-gating" },
            if case.passed { "passed" } else { "failed" }
        );
        for missing in &case.missing {
            println!("  missing: {missing}");
        }
        for warning in &case.warnings {
            println!("  warning: {warning}");
        }
    }
}

#[derive(Debug)]
struct GoldenPlan {
    case_name: String,
    command: PathBuf,
    args: Vec<String>,
    generated_dir: PathBuf,
    copies: Vec<GoldenCopy>,
}

#[derive(Debug)]
struct GoldenCopy {
    format: OutputFormat,
    source: PathBuf,
    target: PathBuf,
}

fn build_golden_plan(
    fixture: &ParityFixtureCase,
    root: &Path,
    whisperx_command: &Path,
    model_dir: &Path,
    model_cache_only: bool,
) -> anyhow::Result<GoldenPlan> {
    let generated_dir = root
        .join("expected")
        .join("whisperx-3.8.6")
        .join("generated")
        .join(&fixture.name);
    let requested_formats = golden_requested_formats(fixture)?;
    let output_format = golden_output_format(fixture, &requested_formats);
    let input_stem = fixture
        .input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .with_context(|| format!("input {} has no UTF-8 file stem", fixture.input.display()))?;

    let mut args = vec![
        fixture.input.display().to_string(),
        "--model".to_string(),
        fixture.whisperx.model.clone(),
        "--model_dir".to_string(),
        model_dir.display().to_string(),
    ];
    if model_cache_only
        || fixture.native_asr.model_cache_only
        || fixture.alignment.model_cache_only
        || fixture.translation.model_cache_only
    {
        args.extend(["--model_cache_only".to_string(), "True".to_string()]);
    }
    if let Some(language) = &fixture.language {
        args.extend(["--language".to_string(), language.clone()]);
    }
    match fixture.native_asr.device {
        DevicePreference::Auto => {}
        DevicePreference::Cpu => args.extend(["--device".to_string(), "cpu".to_string()]),
        DevicePreference::Cuda => args.extend(["--device".to_string(), "cuda".to_string()]),
    }
    if let Some(device_index) = &fixture.native_asr.device_index {
        args.extend(["--device_index".to_string(), device_index.clone()]);
    }
    if let Some(compute_type) = fixture
        .native_asr
        .compute_type
        .as_ref()
        .or(fixture.whisperx.compute_type.as_ref())
    {
        args.extend(["--compute_type".to_string(), compute_type.clone()]);
    }
    if let Some(batch_size) = fixture
        .native_asr
        .max_batch_size
        .or(fixture.whisperx.batch_size)
    {
        args.extend(["--batch_size".to_string(), batch_size.to_string()]);
    }
    args.extend(["--output_format".to_string(), output_format.to_string()]);
    args.extend([
        "--output_dir".to_string(),
        generated_dir.display().to_string(),
    ]);
    push_golden_args(fixture, &mut args)?;

    let mut copies = Vec::new();
    if let Some(expected_json) = &fixture.expected_json {
        copies.push(GoldenCopy {
            format: OutputFormat::Json,
            source: generated_dir.join(format!("{input_stem}.json")),
            target: expected_json.clone(),
        });
    }
    for expected_output in &fixture.expected_outputs {
        copies.push(GoldenCopy {
            format: expected_output.format,
            source: generated_dir.join(format!(
                "{input_stem}.{}",
                expected_output.format.extension()
            )),
            target: expected_output.path.clone(),
        });
    }
    copies = dedup_copies(copies);

    Ok(GoldenPlan {
        case_name: fixture.name.clone(),
        command: whisperx_command.to_path_buf(),
        args,
        generated_dir,
        copies,
    })
}

fn golden_requested_formats(fixture: &ParityFixtureCase) -> anyhow::Result<Vec<OutputFormat>> {
    let mut formats = Vec::new();
    if fixture.expected_json.is_some() {
        formats.push(OutputFormat::Json);
    }
    for ExpectedOutputFile { format, .. } in &fixture.expected_outputs {
        if *format == OutputFormat::NativeJson {
            anyhow::bail!(
                "case `{}` requests native-json, which Python WhisperX cannot generate",
                fixture.name
            );
        }
        formats.push(*format);
    }
    Ok(formats)
}

fn golden_output_format(fixture: &ParityFixtureCase, formats: &[OutputFormat]) -> &'static str {
    if fixture
        .output
        .formats
        .iter()
        .any(|format| *format == OutputFormat::All)
        || formats.iter().any(|format| *format == OutputFormat::All)
        || formats.len() > 1
    {
        "all"
    } else {
        formats
            .first()
            .copied()
            .unwrap_or(OutputFormat::Json)
            .as_transcription_format()
    }
}

fn push_golden_args(fixture: &ParityFixtureCase, args: &mut Vec<String>) -> anyhow::Result<()> {
    args.extend([
        "--task".to_string(),
        fixture.native_asr.task.as_whisperx_arg().to_string(),
    ]);
    if !fixture.alignment.enabled {
        args.push("--no_align".to_string());
    } else {
        args.extend([
            "--align_model".to_string(),
            fixture
                .whisperx
                .align_model
                .clone()
                .unwrap_or_else(|| fixture.alignment.model_id.clone()),
        ]);
        if fixture.alignment.return_char_alignments {
            args.push("--return_char_alignments".to_string());
        }
    }
    if fixture.vad.method != VadMethod::Energy {
        args.extend([
            "--vad_method".to_string(),
            fixture.vad.method.as_whisperx_arg().to_string(),
        ]);
    }
    push_cli_arg_display(args, "--vad_onset", fixture.vad.onset);
    push_cli_arg_display(args, "--vad_offset", fixture.vad.offset);
    push_cli_arg_display(args, "--chunk_size", fixture.vad.chunk_size);

    let decode = &fixture.native_asr.decode;
    if !decode.temperature.is_empty() {
        args.extend([
            "--temperature".to_string(),
            decode
                .temperature
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ]);
    }
    push_cli_arg_display(args, "--best_of", decode.best_of);
    push_cli_arg_display(args, "--beam_size", decode.beam_size);
    push_cli_arg_display(args, "--patience", decode.patience);
    push_cli_arg_display(args, "--length_penalty", decode.length_penalty);
    push_cli_arg(args, "--suppress_tokens", decode.suppress_tokens.as_deref());
    if decode.suppress_numerals {
        args.push("--suppress_numerals".to_string());
    }
    push_cli_arg(args, "--initial_prompt", decode.initial_prompt.as_deref());
    push_cli_arg(args, "--hotwords", decode.hotwords.as_deref());
    push_cli_arg_bool(
        args,
        "--condition_on_previous_text",
        decode.condition_on_previous_text,
    );
    push_cli_arg_bool(args, "--fp16", decode.fp16);
    push_cli_arg_display(
        args,
        "--compression_ratio_threshold",
        decode.compression_ratio_threshold,
    );
    push_cli_arg_display(args, "--logprob_threshold", decode.logprob_threshold);
    push_cli_arg_display(args, "--no_speech_threshold", decode.no_speech_threshold);
    push_cli_arg_display(args, "--threads", decode.threads);

    if fixture.diarization.enabled {
        args.push("--diarize".to_string());
        args.extend([
            "--diarize_model".to_string(),
            fixture.diarization.model_id.clone(),
        ]);
        push_cli_arg_display(args, "--min_speakers", fixture.diarization.min_speakers);
        push_cli_arg_display(args, "--max_speakers", fixture.diarization.max_speakers);
        if let Some(token) = fixture
            .diarization
            .hf_token
            .clone()
            .or_else(|| {
                fixture
                    .diarization
                    .hf_token_env
                    .as_ref()
                    .and_then(|name| std::env::var(name).ok())
            })
            .or_else(|| {
                fixture
                    .whisperx
                    .hf_token_env
                    .as_ref()
                    .and_then(|name| std::env::var(name).ok())
            })
        {
            args.extend(["--hf_token".to_string(), token]);
        }
    }
    if fixture.diarization.return_speaker_embeddings {
        args.push("--speaker_embeddings".to_string());
    }
    push_cli_arg_display(
        args,
        "--max_line_width",
        fixture.output.subtitles.max_line_width,
    );
    push_cli_arg_display(
        args,
        "--max_line_count",
        fixture.output.subtitles.max_line_count,
    );
    if fixture.output.subtitles.highlight_words {
        args.extend(["--highlight_words".to_string(), "True".to_string()]);
    }
    args.extend([
        "--segment_resolution".to_string(),
        match fixture.output.subtitles.segment_resolution {
            SegmentResolution::Sentence => "sentence",
            SegmentResolution::Chunk => "chunk",
        }
        .to_string(),
    ]);
    args.extend(fixture.whisperx.extra_args.clone());
    Ok(())
}

fn push_cli_arg(args: &mut Vec<String>, flag: &str, value: Option<&str>) {
    if let Some(value) = value {
        args.extend([flag.to_string(), value.to_string()]);
    }
}

fn push_cli_arg_display<T: std::fmt::Display>(
    args: &mut Vec<String>,
    flag: &str,
    value: Option<T>,
) {
    if let Some(value) = value {
        args.extend([flag.to_string(), value.to_string()]);
    }
}

fn push_cli_arg_bool(args: &mut Vec<String>, flag: &str, value: Option<bool>) {
    if let Some(value) = value {
        args.extend([
            flag.to_string(),
            if value { "True" } else { "False" }.to_string(),
        ]);
    }
}

fn dedup_copies(copies: Vec<GoldenCopy>) -> Vec<GoldenCopy> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for copy in copies {
        if seen.insert(copy.target.clone()) {
            deduped.push(copy);
        }
    }
    deduped
}

fn ensure_golden_targets_can_write(
    plan: &GoldenPlan,
    overwrite: bool,
    dry_run: bool,
) -> anyhow::Result<()> {
    if overwrite || dry_run {
        return Ok(());
    }
    for copy in &plan.copies {
        if copy.target.exists() {
            anyhow::bail!(
                "refusing to overwrite existing golden {}; pass --overwrite",
                copy.target.display()
            );
        }
    }
    Ok(())
}

fn copy_golden_outputs(plan: &GoldenPlan, overwrite: bool) -> anyhow::Result<()> {
    for copy in &plan.copies {
        if copy.target.exists() && !overwrite {
            anyhow::bail!(
                "refusing to overwrite existing golden {}; pass --overwrite",
                copy.target.display()
            );
        }
        let parent = copy
            .target
            .parent()
            .with_context(|| format!("target {} has no parent", copy.target.display()))?;
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
        fs::copy(&copy.source, &copy.target).with_context(|| {
            format!(
                "failed to copy generated {} output from {} to {}",
                copy.format.as_transcription_format(),
                copy.source.display(),
                copy.target.display()
            )
        })?;
    }
    Ok(())
}

fn print_golden_plan(plan: &GoldenPlan) {
    println!("case: {}", plan.case_name);
    println!("command: {}", shell_command(&plan.command, &plan.args));
    for copy in &plan.copies {
        println!(
            "target: {} <= {}",
            copy.target.display(),
            copy.source.display()
        );
    }
}

fn shell_command(command: &Path, args: &[String]) -> String {
    std::iter::once(shell_quote(&command.display().to_string()))
        .chain(args.iter().map(|arg| shell_quote(arg)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn shell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || "-_./:=,".contains(character))
    {
        return value.to_string();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn suite_case_name_exists(cases: &[ParityFixtureCase], name: &str) -> bool {
    cases.iter().any(|case| case.name == name)
}

fn resolve_cli_path_with_root(path: PathBuf, root: &Path) -> PathBuf {
    if path.is_relative() {
        root.join(path)
    } else {
        path
    }
}

fn absolute_from_cwd(path: PathBuf) -> anyhow::Result<PathBuf> {
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
            CliSegmentResolution::Sentence => Self::Sentence,
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

fn translation_config(
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
        model_id,
        model_bundle,
        model_dir,
        model_cache_only,
        source_language,
        target_language,
        max_new_tokens,
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

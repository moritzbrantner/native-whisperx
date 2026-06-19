use std::collections::HashSet;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, ExitStatus, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Context;
use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use native_whisperx::{
    build_transcription_request, compare_with_whisperx, import_whisperx_json, run, run_many,
    run_parity_fixture_suite, run_parity_preflight, AlignmentConfig, AlignmentInterpolationMethod,
    AsrConfig, AsrProvider, AssignmentPolicy, DevicePreference, DiarizationConfig,
    ExpectedOutputFile, ExpectedTranscriptTarget, ExternalWhisperxConfig, InputSource,
    NativeWhisperxConfig, OutputComparisonMode, OutputConfig, OutputFormat, ParityComparisonConfig,
    ParityConfig, ParityFixtureCase, ParityFixtureCaseReport, ParityFixtureSuite,
    ParityFixtureSuiteReport, SegmentResolution, SubtitleConfig, TranscriptionTask,
    TranslationConfig, VadConfig, VadMethod, WhisperxDecodeConfig,
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
    Transcribe(Box<TranscribeArgs>),
    ImportWhisperx(ImportWhisperxArgs),
    InspectModels(InspectModelsArgs),
    Parity(ParityArgs),
    ParityFixtures(ParityFixturesArgs),
    ParityBench(ParityBenchArgs),
    ParitySummary(ParitySummaryArgs),
    ParityPreflight(ParityPreflightArgs),
    ParityGoldens(ParityGoldensArgs),
    #[command(name = "__parity-fixture-case", hide = true)]
    ParityFixtureCase(ParityFixtureCaseArgs),
    #[command(name = "__parity-bench-case", hide = true)]
    ParityBenchCase(ParityBenchCaseArgs),
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
    #[arg(long, visible_alias = "diarize_model")]
    diarize_model: Option<String>,
    #[arg(long, visible_alias = "speaker_embeddings", action = ArgAction::SetTrue)]
    speaker_embeddings: bool,
    #[arg(long, visible_alias = "hf_token")]
    hf_token: Option<String>,
    #[arg(long, visible_alias = "min_speakers")]
    min_speakers: Option<usize>,
    #[arg(long, visible_alias = "max_speakers")]
    max_speakers: Option<usize>,
    #[arg(
        long = "speaker-assignment-policy",
        visible_alias = "speaker_assignment_policy",
        value_enum,
        default_value_t = CliAssignmentPolicy::Majority
    )]
    speaker_assignment_policy: CliAssignmentPolicy,
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
    #[arg(
        long = "speaker-assignment-policy",
        visible_alias = "speaker_assignment_policy",
        value_enum,
        default_value_t = CliAssignmentPolicy::Majority
    )]
    speaker_assignment_policy: CliAssignmentPolicy,
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
    #[arg(long = "case")]
    cases: Vec<String>,
    #[arg(long = "case-timeout-seconds", visible_alias = "case_timeout_seconds")]
    case_timeout_seconds: Option<u64>,
    #[arg(
        long = "require-non-gating-passed",
        visible_alias = "require_non_gating_passed"
    )]
    require_non_gating_passed: bool,
}

#[derive(Debug, Args)]
struct ParityBenchArgs {
    manifest: PathBuf,
    #[arg(long)]
    root: Option<PathBuf>,
    #[arg(long, visible_alias = "whisperx_command")]
    whisperx_command: Option<PathBuf>,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    model_cache_only: bool,
    #[arg(long = "iterations", default_value_t = 3)]
    iterations: usize,
    #[arg(long = "warmups", default_value_t = 1)]
    warmups: usize,
    #[arg(long = "case")]
    cases: Vec<String>,
    #[arg(long = "case-timeout-seconds", visible_alias = "case_timeout_seconds")]
    case_timeout_seconds: Option<u64>,
    #[arg(long = "native-only", visible_alias = "native_only")]
    native_only: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct ParitySummaryArgs {
    report: PathBuf,
}

#[derive(Debug, Args)]
struct ParityFixtureCaseArgs {
    #[arg(long)]
    fixture: PathBuf,
    #[arg(long)]
    root: PathBuf,
    #[arg(long)]
    report: PathBuf,
}

#[derive(Debug, Args)]
struct ParityBenchCaseArgs {
    #[arg(long)]
    fixture: PathBuf,
    #[arg(long)]
    report: PathBuf,
    #[arg(long)]
    iterations: usize,
    #[arg(long)]
    warmups: usize,
    #[arg(long = "native-only", visible_alias = "native_only")]
    native_only: bool,
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
enum CliAssignmentPolicy {
    Majority,
    NearestStart,
    StrictContained,
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
        Some(Command::Transcribe(args)) => transcribe_command(*args),
        Some(Command::ImportWhisperx(args)) => import_whisperx_command(args),
        Some(Command::InspectModels(args)) => inspect_models_command(args),
        Some(Command::Parity(args)) => parity_command(args),
        Some(Command::ParityFixtures(args)) => parity_fixtures_command(args),
        Some(Command::ParityBench(args)) => parity_bench_command(args),
        Some(Command::ParitySummary(args)) => parity_summary_command(args),
        Some(Command::ParityPreflight(args)) => parity_preflight_command(args),
        Some(Command::ParityGoldens(args)) => parity_goldens_command(args),
        Some(Command::ParityFixtureCase(args)) => parity_fixture_case_command(args),
        Some(Command::ParityBenchCase(args)) => parity_bench_case_command(args),
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
            | "parity-bench"
            | "parity-summary"
            | "parity-preflight"
            | "parity-goldens"
            | "__parity-fixture-case"
            | "__parity-bench-case"
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
        && args.translation_model.is_none()
        && args.translation_bundle.is_none()
    {
        anyhow::bail!(
            "native --task translate requires --translation-model or --translation-bundle; use --provider external-whisperx for WhisperX built-in translation"
        );
    }
    if args.speaker_embeddings && args.provider == CliProvider::Native {
        anyhow::bail!(
            "native provider does not produce WhisperX-compatible speaker embeddings; use --provider external-whisperx"
        );
    }
    if args.provider == CliProvider::Native
        && args
            .diarize_model
            .as_deref()
            .is_some_and(is_pyannote_diarization_model)
    {
        anyhow::bail!(
            "pyannote diarization models require --provider external-whisperx; native diarization uses native-spectral-speaker-baseline"
        );
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
    let diarize_model = args
        .diarize_model
        .clone()
        .unwrap_or_else(|| match args.provider {
            CliProvider::Native => DiarizationConfig::default().model_id,
            CliProvider::ExternalWhisperx => "pyannote/speaker-diarization-community-1".to_string(),
        });

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
            model_id: diarize_model,
            hf_token: args.hf_token.clone(),
            return_speaker_embeddings: args.speaker_embeddings,
            speaker_embedding_model_bundle: args.speaker_embedding_bundle.clone(),
            speaker_embedding_model_file: args.speaker_embedding_model_file.clone(),
            speaker_embedding_dimension: args.speaker_embedding_dim,
            speaker_embedding_sample_rate: args.speaker_embedding_sample_rate,
            min_speakers: args.min_speakers,
            max_speakers: args.max_speakers,
            assignment_policy: args.speaker_assignment_policy.into(),
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

fn is_pyannote_diarization_model(model_id: &str) -> bool {
    model_id
        .trim()
        .to_ascii_lowercase()
        .starts_with("pyannote/")
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
    let config = NativeWhisperxConfig {
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

fn parity_command(args: ParityArgs) -> anyhow::Result<()> {
    let report = compare_with_whisperx(ParityConfig {
        input: args.input,
        expected_json: args.expected_json,
        expected_target: ExpectedTranscriptTarget::Native,
        comparison: ParityComparisonConfig::default(),
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
        whisperx_diarization: None,
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
    let suite_report_path = output_dir
        .as_ref()
        .map(|output_dir| output_dir.join("report.json"));
    let model_dir = args.model_dir.map(absolute_from_cwd).transpose()?;
    let filters = args.cases.iter().cloned().collect::<HashSet<_>>();

    for case_name in &filters {
        if !suite_case_name_exists(&suite.fixtures, case_name) {
            anyhow::bail!("no fixture case named {case_name} matched the manifest");
        }
    }

    if !filters.is_empty() {
        suite
            .fixtures
            .retain(|fixture| filters.contains(&fixture.name));
    }

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

    let report = run_parity_fixture_suite_with_progress(
        suite,
        root.clone(),
        args.case_timeout_seconds.map(Duration::from_secs),
        args.require_non_gating_passed,
    )?;
    if let Some(report_path) = &suite_report_path {
        if let Some(parent) = report_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create parity fixture report directory {}",
                    parent.display()
                )
            })?;
        }
        fs::write(report_path, serde_json::to_vec_pretty(&report)?).with_context(|| {
            format!(
                "failed to write parity fixture report {}",
                report_path.display()
            )
        })?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    if !report.passed {
        anyhow::bail!("one or more parity fixtures failed");
    }
    Ok(())
}

fn run_parity_fixture_suite_with_progress(
    suite: ParityFixtureSuite,
    root: PathBuf,
    case_timeout: Option<Duration>,
    require_non_gating_passed: bool,
) -> anyhow::Result<ParityFixtureSuiteReport> {
    let total = suite.fixtures.len();
    let mut cases = Vec::with_capacity(total);

    for (index, fixture) in suite.fixtures.into_iter().enumerate() {
        let case_number = index + 1;
        let case_name = fixture.name.clone();
        let gating = fixture.gating;
        let started_at = unix_timestamp_string(SystemTime::now());
        let start = Instant::now();
        eprintln!(
            "parity-fixtures: starting case {case_number}/{total}: {case_name}{}",
            if gating { " [gating]" } else { "" }
        );

        let fixture_timeout = fixture.timeout_seconds.map(Duration::from_secs);
        let timeout = case_timeout.or(fixture_timeout);
        let mut case = run_single_parity_fixture_case(fixture, root.clone(), timeout)?;
        let elapsed = start.elapsed();
        case.started_at = Some(started_at);
        case.elapsed_seconds = Some(duration_seconds(elapsed));
        case.timed_out = case.error.as_deref().is_some_and(is_timeout_error);
        if case.timed_out {
            eprintln!(
                "parity-fixtures: timed out case {case_number}/{total}: {case_name} after {}",
                format_duration(elapsed)
            );
        } else if case.passed {
            eprintln!(
                "parity-fixtures: completed case {case_number}/{total}: {case_name} passed in {}",
                format_duration(elapsed)
            );
        } else {
            eprintln!(
                "parity-fixtures: completed case {case_number}/{total}: {case_name} failed in {}",
                format_duration(elapsed)
            );
        }
        cases.push(case);
    }

    let passed = cases
        .iter()
        .filter(|case| require_non_gating_passed || case.gating)
        .all(|case| case.passed);
    Ok(ParityFixtureSuiteReport { passed, cases })
}

fn run_single_parity_fixture_case(
    fixture: ParityFixtureCase,
    root: PathBuf,
    case_timeout: Option<Duration>,
) -> anyhow::Result<ParityFixtureCaseReport> {
    let name = fixture.name.clone();
    let gating = fixture.gating;
    let Some(timeout) = case_timeout else {
        return run_single_parity_fixture_case_now(fixture, root);
    };
    if timeout.is_zero() {
        let error = format!(
            "parity fixture case `{name}` exceeded timeout of {}",
            format_duration(timeout)
        );
        return Ok(failed_parity_fixture_case(name, gating, error));
    }

    let temp_prefix = parity_case_temp_prefix(&name);
    let fixture_path = temp_prefix.with_extension("fixture.json");
    let report_path = temp_prefix.with_extension("report.json");
    fs::write(&fixture_path, serde_json::to_vec(&fixture)?)?;

    let result = run_single_parity_fixture_case_child(&fixture_path, &root, &report_path, timeout)
        .and_then(|status| {
            if !status.success() {
                let error =
                    format!("parity fixture case `{name}` worker exited with status {status}");
                return Ok(failed_parity_fixture_case(name.clone(), gating, error));
            }
            let bytes = fs::read(&report_path).with_context(|| {
                format!(
                    "parity fixture case `{name}` worker did not write {}",
                    report_path.display()
                )
            })?;
            serde_json::from_slice::<ParityFixtureCaseReport>(&bytes).map_err(anyhow::Error::from)
        });

    let _ = fs::remove_file(&fixture_path);
    let _ = fs::remove_file(&report_path);

    match result {
        Ok(case) => Ok(case),
        Err(error) if is_timeout_error(&error.to_string()) => {
            Ok(failed_parity_fixture_case(name, gating, error.to_string()))
        }
        Err(error) => Err(error),
    }
}

fn run_single_parity_fixture_case_child(
    fixture_path: &Path,
    root: &Path,
    report_path: &Path,
    timeout: Duration,
) -> anyhow::Result<ExitStatus> {
    let mut child = ProcessCommand::new(std::env::current_exe()?)
        .arg("__parity-fixture-case")
        .arg("--fixture")
        .arg(fixture_path)
        .arg("--root")
        .arg(root)
        .arg("--report")
        .arg(report_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| "failed to spawn parity fixture case worker")?;

    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!(
                "parity fixture case worker exceeded timeout of {}",
                format_duration(timeout)
            );
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn parity_case_temp_prefix(case_name: &str) -> PathBuf {
    let millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let safe_name = case_name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>();
    std::env::temp_dir().join(format!(
        "native-whisperx-parity-{safe_name}-{}-{millis}",
        std::process::id()
    ))
}

fn run_single_parity_fixture_case_now(
    fixture: ParityFixtureCase,
    root: PathBuf,
) -> anyhow::Result<ParityFixtureCaseReport> {
    let name = fixture.name.clone();
    let gating = fixture.gating;
    let report = run_parity_fixture_suite(
        ParityFixtureSuite {
            fixtures: vec![fixture],
        },
        Some(&root),
    )?;
    Ok(report.cases.into_iter().next().unwrap_or_else(|| {
        failed_parity_fixture_case(
            name.clone(),
            gating,
            format!("parity fixture case `{name}` produced no report"),
        )
    }))
}

fn parity_fixture_case_command(args: ParityFixtureCaseArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.fixture)
        .with_context(|| format!("failed to read {}", args.fixture.display()))?;
    let fixture: ParityFixtureCase = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.fixture.display()))?;
    let report = run_single_parity_fixture_case_now(fixture, args.root)?;
    fs::write(&args.report, serde_json::to_vec(&report)?)
        .with_context(|| format!("failed to write {}", args.report.display()))?;
    Ok(())
}

fn parity_bench_command(args: ParityBenchArgs) -> anyhow::Result<()> {
    if args.iterations == 0 {
        anyhow::bail!("--iterations must be greater than zero");
    }
    let bytes = fs::read(&args.manifest)
        .with_context(|| format!("failed to read {}", args.manifest.display()))?;
    let mut suite: ParityFixtureSuite = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.manifest.display()))?;
    let root = smoke_root_or_arg(args.root, "parity-bench")?;
    let whisperx_command = args.whisperx_command.map(absolute_from_cwd).transpose()?;
    let model_dir = args
        .model_dir
        .map(absolute_from_cwd)
        .transpose()?
        .unwrap_or_else(|| root.join("models"));
    let filters = args.cases.iter().cloned().collect::<HashSet<_>>();

    for case_name in &filters {
        if !suite_case_name_exists(&suite.fixtures, case_name) {
            anyhow::bail!("no fixture case named {case_name} matched the manifest");
        }
    }
    if !filters.is_empty() {
        suite
            .fixtures
            .retain(|fixture| filters.contains(&fixture.name));
    }

    let mut case_results = Vec::with_capacity(suite.fixtures.len());
    for mut fixture in suite.fixtures {
        prepare_fixture_for_cli_run(
            &mut fixture,
            &root,
            whisperx_command.as_ref(),
            &model_dir,
            args.model_cache_only,
        );
        let timeout = args
            .case_timeout_seconds
            .or(fixture.timeout_seconds)
            .map(Duration::from_secs);
        let options = BenchRunOptions {
            iterations: args.iterations,
            warmups: args.warmups,
            native_only: args.native_only,
        };
        let case_result = run_parity_bench_case_with_timeout(&fixture, options, timeout)
            .unwrap_or_else(|error| {
                failed_parity_bench_case(&fixture, options, false, error.to_string())
            });
        case_results.push(case_result);
    }

    let passed = case_results
        .iter()
        .all(|case| case["passed"].as_bool().unwrap_or(true));
    let report = serde_json::json!({
        "passed": passed,
        "iterations": args.iterations,
        "warmups": args.warmups,
        "nativeOnly": args.native_only,
        "caseTimeoutSeconds": args.case_timeout_seconds,
        "cases": case_results,
    });
    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_parity_bench_report(&report);
    }
    Ok(())
}

fn parity_bench_case_command(args: ParityBenchCaseArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.fixture)
        .with_context(|| format!("failed to read {}", args.fixture.display()))?;
    let fixture: ParityFixtureCase = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.fixture.display()))?;
    set_ort_dylib_path_from_fixture_if_missing(&fixture);
    let options = BenchRunOptions {
        iterations: args.iterations,
        warmups: args.warmups,
        native_only: args.native_only,
    };
    let report = run_parity_bench_case(&fixture, options).unwrap_or_else(|error| {
        failed_parity_bench_case(&fixture, options, false, error.to_string())
    });
    fs::write(&args.report, serde_json::to_vec(&report)?)
        .with_context(|| format!("failed to write {}", args.report.display()))?;
    Ok(())
}

fn prepare_fixture_for_cli_run(
    fixture: &mut ParityFixtureCase,
    root: &Path,
    whisperx_command: Option<&PathBuf>,
    model_dir: &Path,
    model_cache_only: bool,
) {
    fixture.input = resolve_cli_path_with_root(fixture.input.clone(), root);
    if let Some(command) = whisperx_command {
        fixture.whisperx.command = command.clone();
    }
    fixture.native_asr.whisper_bundle = fixture
        .native_asr
        .whisper_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.native_asr.model_dir = Some(model_dir.to_path_buf());
    fixture.alignment.model_bundle = fixture
        .alignment
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.alignment.model_dir = Some(model_dir.to_path_buf());
    fixture.translation.model_bundle = fixture
        .translation
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.translation.model_dir = Some(model_dir.to_path_buf());
    fixture.vad.model_bundle = fixture
        .vad
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.diarization.speaker_embedding_model_bundle = fixture
        .diarization
        .speaker_embedding_model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    if model_cache_only {
        fixture.native_asr.model_cache_only = true;
        fixture.alignment.model_cache_only = true;
        fixture.translation.model_cache_only = true;
    }
    if fixture.output.output_dir.is_none() {
        fixture.output.output_dir = Some(root.join("out").join("parity-bench").join(&fixture.name));
    }
}

fn set_ort_dylib_path_from_fixture_if_missing(fixture: &ParityFixtureCase) {
    if std::env::var_os("ORT_DYLIB_PATH").is_some() {
        return;
    }
    let Some(path) = inferred_ort_dylib_path(fixture) else {
        return;
    };
    std::env::set_var("ORT_DYLIB_PATH", path);
}

fn inferred_ort_dylib_path(fixture: &ParityFixtureCase) -> Option<PathBuf> {
    inferred_ort_dylib_path_with_env(fixture, std::env::var_os("ORT_DYLIB_PATH"))
}

fn inferred_ort_dylib_path_with_env(
    fixture: &ParityFixtureCase,
    ort_dylib_path: Option<OsString>,
) -> Option<PathBuf> {
    if ort_dylib_path.is_some() {
        return None;
    }
    if !matches!(fixture.vad.method, VadMethod::Silero | VadMethod::Pyannote) {
        return None;
    }
    let env_root = fixture.whisperx.command.parent()?.parent()?;
    find_onnxruntime_dylib(env_root)
}

fn find_onnxruntime_dylib(env_root: &Path) -> Option<PathBuf> {
    let lib_dir = env_root.join("lib");
    let mut candidates = Vec::new();
    for python_dir in fs::read_dir(&lib_dir).ok()?.filter_map(Result::ok) {
        let file_name = python_dir.file_name();
        if !file_name.to_string_lossy().starts_with("python") {
            continue;
        }
        let capi_dir = python_dir
            .path()
            .join("site-packages")
            .join("onnxruntime")
            .join("capi");
        let Ok(entries) = fs::read_dir(capi_dir) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if name.starts_with("libonnxruntime.so") || name.starts_with("libonnxruntime.dylib") {
                candidates.push(path);
            }
        }
    }
    candidates.sort();
    candidates.into_iter().next()
}

#[derive(Debug, Clone, Copy)]
struct BenchRunOptions {
    iterations: usize,
    warmups: usize,
    native_only: bool,
}

fn run_parity_bench_case_with_timeout(
    fixture: &ParityFixtureCase,
    options: BenchRunOptions,
    timeout: Option<Duration>,
) -> anyhow::Result<serde_json::Value> {
    let Some(timeout) = timeout else {
        return run_parity_bench_case(fixture, options);
    };
    if timeout.is_zero() {
        return Ok(failed_parity_bench_case(
            fixture,
            options,
            true,
            format!(
                "parity benchmark case `{}` exceeded timeout of {}",
                fixture.name,
                format_duration(timeout)
            ),
        ));
    }

    let temp_prefix = parity_case_temp_prefix(&fixture.name);
    let fixture_path = temp_prefix.with_extension("bench-fixture.json");
    let report_path = temp_prefix.with_extension("bench-report.json");
    fs::write(&fixture_path, serde_json::to_vec(fixture)?)?;

    let result =
        run_parity_bench_case_child(&fixture_path, &report_path, fixture, options, timeout)
            .and_then(|status| {
                if !status.success() {
                    return Ok(failed_parity_bench_case(
                        fixture,
                        options,
                        false,
                        format!(
                            "parity benchmark case `{}` worker exited with status {status}",
                            fixture.name
                        ),
                    ));
                }
                let bytes = fs::read(&report_path).with_context(|| {
                    format!(
                        "parity benchmark case `{}` worker did not write {}",
                        fixture.name,
                        report_path.display()
                    )
                })?;
                serde_json::from_slice::<serde_json::Value>(&bytes).map_err(anyhow::Error::from)
            });

    let _ = fs::remove_file(&fixture_path);
    let _ = fs::remove_file(&report_path);

    match result {
        Ok(case) => Ok(case),
        Err(error) if is_timeout_error(&error.to_string()) => Ok(failed_parity_bench_case(
            fixture,
            options,
            true,
            error.to_string(),
        )),
        Err(error) => Err(error),
    }
}

fn run_parity_bench_case_child(
    fixture_path: &Path,
    report_path: &Path,
    fixture: &ParityFixtureCase,
    options: BenchRunOptions,
    timeout: Duration,
) -> anyhow::Result<ExitStatus> {
    let mut command = ProcessCommand::new(std::env::current_exe()?);
    command
        .arg("__parity-bench-case")
        .arg("--fixture")
        .arg(fixture_path)
        .arg("--report")
        .arg(report_path)
        .arg("--iterations")
        .arg(options.iterations.to_string())
        .arg("--warmups")
        .arg(options.warmups.to_string())
        .args(options.native_only.then_some("--native-only"))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if let Some(ort_dylib_path) = inferred_ort_dylib_path(fixture) {
        command.env("ORT_DYLIB_PATH", ort_dylib_path);
    }
    let mut child = command
        .spawn()
        .with_context(|| "failed to spawn parity benchmark case worker")?;

    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!(
                "parity benchmark case worker exceeded timeout of {}",
                format_duration(timeout)
            );
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn run_parity_bench_case(
    fixture: &ParityFixtureCase,
    options: BenchRunOptions,
) -> anyhow::Result<serde_json::Value> {
    for warmup in 0..options.warmups {
        eprintln!(
            "parity-bench: warming case {} iteration {}/{}",
            fixture.name,
            warmup + 1,
            options.warmups
        );
        run_single_bench_iteration(fixture, options.native_only, warmup + 1, true)?;
    }

    let mut iterations_json = Vec::with_capacity(options.iterations);
    for iteration in 0..options.iterations {
        eprintln!(
            "parity-bench: starting case {} iteration {}/{}",
            fixture.name,
            iteration + 1,
            options.iterations
        );
        iterations_json.push(run_single_bench_iteration(
            fixture,
            options.native_only,
            iteration + 1,
            false,
        )?);
    }
    let passed = iterations_json
        .iter()
        .all(bench_iteration_passes_speed_gate);
    Ok(serde_json::json!({
        "name": fixture.name,
        "gating": fixture.gating,
        "passed": passed,
        "timedOut": false,
        "nativeOnly": options.native_only,
        "warmups": options.warmups,
        "iterations": iterations_json,
    }))
}

fn run_single_bench_iteration(
    fixture: &ParityFixtureCase,
    native_only: bool,
    iteration: usize,
    warmup: bool,
) -> anyhow::Result<serde_json::Value> {
    let (native_report, native_elapsed) = timed_run(native_bench_config(fixture))?;
    let whisperx_run = if native_only {
        None
    } else {
        Some(timed_run(whisperx_bench_config(fixture))?)
    };
    let audio_duration = fixture
        .clip_seconds
        .or_else(|| inferred_audio_duration_seconds(&native_report))
        .or_else(|| {
            whisperx_run
                .as_ref()
                .and_then(|(report, _)| inferred_audio_duration_seconds(report))
        });
    let whisperx_json = whisperx_run
        .as_ref()
        .map(|(report, elapsed)| bench_run_json(report, *elapsed, audio_duration, false));
    let whisperx_elapsed = whisperx_run
        .as_ref()
        .map(|(_, elapsed)| duration_seconds(*elapsed));
    let whisperx_realtime = whisperx_run.as_ref().and_then(|(_, elapsed)| {
        audio_duration.map(|duration| duration_seconds(*elapsed) / duration)
    });
    let native_elapsed_seconds = duration_seconds(native_elapsed);
    let native_phases =
        bench_phase_json(&native_report.response.diagnostics, native_elapsed_seconds);
    let speed = bench_speed_comparison(native_elapsed_seconds, whisperx_elapsed);
    Ok(serde_json::json!({
        "iteration": iteration,
        "warmup": warmup,
        "nativeElapsedSeconds": native_elapsed_seconds,
        "whisperxElapsedSeconds": whisperx_elapsed,
        "audioDurationSeconds": audio_duration,
        "nativeRealtimeFactor": audio_duration.map(|duration| native_elapsed_seconds / duration),
        "whisperxRealtimeFactor": whisperx_realtime,
        "nativeFasterThanWhisperx": speed.native_faster_than_whisperx,
        "nativeSpeedupRatio": speed.native_speedup_ratio,
        "nativeTotalSeconds": native_phases
            .get("nativeTotalSeconds")
            .and_then(serde_json::Value::as_f64),
        "decodeSeconds": native_phases
            .get("decodeSeconds")
            .and_then(serde_json::Value::as_f64),
        "vadSeconds": native_phases
            .get("vadSeconds")
            .and_then(serde_json::Value::as_f64),
        "asrSeconds": native_phases
            .get("asrSeconds")
            .and_then(serde_json::Value::as_f64),
        "alignmentSeconds": native_phases
            .get("alignmentSeconds")
            .and_then(serde_json::Value::as_f64),
        "diarizationSeconds": native_phases
            .get("diarizationSeconds")
            .and_then(serde_json::Value::as_f64),
        "outputSeconds": native_phases
            .get("outputSeconds")
            .and_then(serde_json::Value::as_f64),
        "peakRssBytes": serde_json::Value::Null,
        "cudaActive": diagnostic_bool(&native_report.response.diagnostics, "cuda"),
        "alignmentCudaActive": diagnostic_bool(&native_report.response.diagnostics, "alignmentCuda"),
        "alignmentDevice": diagnostic_value(&native_report.response.diagnostics, "alignmentDevice"),
        "modelId": fixture.native_asr.model_id,
        "chunkCount": diagnostic_value(&native_report.response.diagnostics, "chunkCount"),
        "batchCount": diagnostic_value(&native_report.response.diagnostics, "batchCount"),
        "batchExecution": diagnostic_value(&native_report.response.diagnostics, "batchExecution"),
        "alignmentBatchExecution": diagnostic_value(&native_report.response.diagnostics, "alignmentBatchExecution"),
        "diarizationWindowExecution": diagnostic_value(&native_report.response.diagnostics, "diarizationWindowExecution"),
        "nativeDiagnostics": native_report.response.diagnostics.clone(),
        "whisperxDiagnostics": whisperx_run
            .as_ref()
            .map(|(report, _)| report.response.diagnostics.clone())
            .unwrap_or_default(),
        "native": bench_run_json_from_phases(
            &native_report,
            native_elapsed_seconds,
            audio_duration,
            true,
            native_phases,
        ),
        "whisperx": whisperx_json,
    }))
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BenchSpeedComparison {
    native_faster_than_whisperx: Option<bool>,
    native_speedup_ratio: Option<f64>,
}

fn bench_speed_comparison(
    native_elapsed_seconds: f64,
    whisperx_elapsed_seconds: Option<f64>,
) -> BenchSpeedComparison {
    let native_elapsed_seconds = finite_positive_seconds(native_elapsed_seconds);
    let whisperx_elapsed_seconds = whisperx_elapsed_seconds.and_then(finite_positive_seconds);
    BenchSpeedComparison {
        native_faster_than_whisperx: native_elapsed_seconds
            .zip(whisperx_elapsed_seconds)
            .map(|(native, whisperx)| native < whisperx),
        native_speedup_ratio: native_elapsed_seconds
            .zip(whisperx_elapsed_seconds)
            .map(|(native, whisperx)| whisperx / native),
    }
}

fn finite_positive_seconds(value: f64) -> Option<f64> {
    (value.is_finite() && value > 0.0).then_some(value)
}

fn bench_iteration_passes_speed_gate(iteration: &serde_json::Value) -> bool {
    iteration
        .get("nativeFasterThanWhisperx")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true)
}

fn failed_parity_bench_case(
    fixture: &ParityFixtureCase,
    options: BenchRunOptions,
    timed_out: bool,
    error: String,
) -> serde_json::Value {
    serde_json::json!({
        "name": fixture.name,
        "gating": fixture.gating,
        "passed": false,
        "timedOut": timed_out,
        "nativeOnly": options.native_only,
        "warmups": options.warmups,
        "iterations": [],
        "error": error,
    })
}

fn timed_run(
    config: NativeWhisperxConfig,
) -> anyhow::Result<(native_whisperx::NativeWhisperxReport, Duration)> {
    let start = Instant::now();
    let report = run(config).map_err(anyhow::Error::from)?;
    Ok((report, start.elapsed()))
}

fn native_bench_config(fixture: &ParityFixtureCase) -> NativeWhisperxConfig {
    let mut asr = fixture.native_asr.clone();
    asr.provider = AsrProvider::Native;
    asr.language = fixture.language.clone();
    asr.max_batch_size = asr.max_batch_size.or(fixture.whisperx.batch_size);
    NativeWhisperxConfig {
        input: InputSource::Path {
            path: fixture.input.clone(),
        },
        asr,
        translation: fixture.translation.clone(),
        vad: fixture.vad.clone(),
        alignment: fixture.alignment.clone(),
        diarization: fixture.diarization.clone(),
        output: fixture.output.clone(),
    }
}

fn whisperx_bench_config(fixture: &ParityFixtureCase) -> NativeWhisperxConfig {
    NativeWhisperxConfig {
        input: InputSource::Path {
            path: fixture.input.clone(),
        },
        asr: AsrConfig {
            provider: AsrProvider::ExternalWhisperX,
            task: fixture.native_asr.task,
            language: fixture.language.clone(),
            device: fixture.native_asr.device,
            device_index: fixture.native_asr.device_index.clone(),
            model_dir: fixture.native_asr.model_dir.clone(),
            model_cache_only: fixture.native_asr.model_cache_only
                || fixture.alignment.model_cache_only,
            max_batch_size: fixture.whisperx.batch_size,
            external_whisperx: fixture.whisperx.clone(),
            ..AsrConfig::default()
        },
        translation: TranslationConfig::default(),
        vad: fixture.vad.clone(),
        alignment: fixture.alignment.clone(),
        diarization: fixture
            .whisperx_diarization
            .clone()
            .unwrap_or_else(|| fixture.diarization.clone()),
        output: fixture.output.clone(),
    }
}

fn bench_run_json(
    report: &native_whisperx::NativeWhisperxReport,
    elapsed: Duration,
    audio_duration: Option<f64>,
    native: bool,
) -> serde_json::Value {
    let elapsed_seconds = duration_seconds(elapsed);
    let phases = bench_phase_json(&report.response.diagnostics, elapsed_seconds);
    bench_run_json_from_phases(report, elapsed_seconds, audio_duration, native, phases)
}

fn bench_run_json_from_phases(
    report: &native_whisperx::NativeWhisperxReport,
    elapsed_seconds: f64,
    audio_duration: Option<f64>,
    native: bool,
    phases: serde_json::Value,
) -> serde_json::Value {
    let diagnostics = &report.response.diagnostics;
    serde_json::json!({
        "elapsedSeconds": elapsed_seconds,
        "realtimeFactor": audio_duration.map(|duration| elapsed_seconds / duration),
        "phases": phases,
        "counters": bench_counter_json(diagnostics),
        "runtime": bench_runtime_json(diagnostics, native),
        "diagnostics": diagnostics,
    })
}

fn bench_phase_json(diagnostics: &[String], total_elapsed_seconds: f64) -> serde_json::Value {
    serde_json::json!({
        "decodeSeconds": diagnostic_f64(diagnostics, "phaseDecodeSeconds"),
        "vadSeconds": diagnostic_f64(diagnostics, "phaseVadSeconds"),
        "asrSeconds": diagnostic_f64(diagnostics, "phaseAsrSeconds"),
        "alignmentSeconds": diagnostic_f64(diagnostics, "phaseAlignmentSeconds"),
        "diarizationSeconds": diagnostic_f64(diagnostics, "phaseDiarizationSeconds"),
        "outputSeconds": diagnostic_f64(diagnostics, "phaseOutputSeconds"),
        "nativeTotalSeconds": diagnostic_f64(diagnostics, "phaseNativeTotalSeconds"),
        "totalElapsedSeconds": total_elapsed_seconds,
    })
}

fn bench_counter_json(diagnostics: &[String]) -> serde_json::Value {
    let model_source = diagnostic_value(diagnostics, "asrModelSource");
    let asr_cache_hit = model_source.as_deref() == Some("hugging-face-cache");
    serde_json::json!({
        "decodeSamples": diagnostic_usize(diagnostics, "phaseDecodeSamples"),
        "vadSegments": diagnostic_usize(diagnostics, "phaseVadSegments"),
        "vadWindows": diagnostic_usize(diagnostics, "phaseVadWindows"),
        "asrSegments": diagnostic_usize(diagnostics, "phaseAsrSegments"),
        "alignmentWords": diagnostic_usize(diagnostics, "phaseAlignmentWords"),
        "diarizationSpeakers": diagnostic_usize(diagnostics, "phaseDiarizationSpeakers"),
        "diarizationSegments": diagnostic_usize(diagnostics, "phaseDiarizationSegments"),
        "chunkCount": diagnostic_usize(diagnostics, "chunkCount"),
        "batchCount": diagnostic_usize(diagnostics, "batchCount"),
        "modelLoadCount": if diagnostics.iter().any(|item| item.starts_with("asrModelId=")) { 1 } else { 0 },
        "asrCacheHitCount": if asr_cache_hit { 1 } else { 0 },
    })
}

fn bench_runtime_json(diagnostics: &[String], native: bool) -> serde_json::Value {
    serde_json::json!({
        "provider": if native { "native" } else { "whisperx" },
        "cuda": diagnostic_bool(diagnostics, "cuda"),
        "device": diagnostic_value(diagnostics, "device"),
        "alignmentCuda": diagnostic_bool(diagnostics, "alignmentCuda"),
        "alignmentDevice": diagnostic_value(diagnostics, "alignmentDevice"),
        "modelId": diagnostic_value(diagnostics, "asrModelId"),
        "modelSource": diagnostic_value(diagnostics, "asrModelSource"),
        "modelResolved": diagnostic_value(diagnostics, "asrModelResolved"),
        "modelRuntimeReused": false,
        "processReusedAcrossIterations": true,
    })
}

fn inferred_audio_duration_seconds(report: &native_whisperx::NativeWhisperxReport) -> Option<f64> {
    let transcript = serde_json::to_value(&report.response.transcript).ok()?;
    let segment_max = transcript
        .get("segments")
        .and_then(|segments| segments.as_array())
        .into_iter()
        .flatten()
        .filter_map(|segment| segment.get("end").and_then(|end| end.as_f64()))
        .fold(None, max_option_f64);
    let vad_max = report
        .response
        .vad_segments
        .iter()
        .map(|segment| segment.end_seconds)
        .fold(None, max_option_f64);
    match (segment_max, vad_max) {
        (Some(segment), Some(vad)) => Some(segment.max(vad)),
        (Some(segment), None) => Some(segment),
        (None, Some(vad)) => Some(vad),
        (None, None) => None,
    }
}

fn max_option_f64(max: Option<f64>, value: f64) -> Option<f64> {
    Some(max.map_or(value, |max| max.max(value)))
}

fn diagnostic_bool(diagnostics: &[String], key: &str) -> Option<bool> {
    diagnostic_value(diagnostics, key).and_then(|value| match value.as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    })
}

fn diagnostic_value(diagnostics: &[String], key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    diagnostics
        .iter()
        .find_map(|diagnostic| diagnostic.strip_prefix(&prefix).map(ToOwned::to_owned))
}

fn diagnostic_f64(diagnostics: &[String], key: &str) -> Option<f64> {
    diagnostic_value(diagnostics, key).and_then(|value| value.parse::<f64>().ok())
}

fn diagnostic_usize(diagnostics: &[String], key: &str) -> Option<usize> {
    diagnostic_value(diagnostics, key).and_then(|value| value.parse::<usize>().ok())
}

fn print_parity_bench_report(report: &serde_json::Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".to_string())
    );
}

fn failed_parity_fixture_case(
    name: String,
    gating: bool,
    error: String,
) -> ParityFixtureCaseReport {
    ParityFixtureCaseReport {
        name,
        gating,
        passed: false,
        started_at: None,
        elapsed_seconds: None,
        timed_out: is_timeout_error(&error),
        report: None,
        missing_required_diagnostics: Vec::new(),
        expected_output_matches: Vec::new(),
        failure_summary: vec![error.clone()],
        error: Some(error),
    }
}

fn is_timeout_error(error: &str) -> bool {
    error.contains("exceeded timeout")
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let millis = duration.subsec_millis();
    if seconds == 0 {
        format!("{millis}ms")
    } else if millis == 0 {
        format!("{seconds}s")
    } else {
        format!("{seconds}.{millis:03}s")
    }
}

fn duration_seconds(duration: Duration) -> f64 {
    duration.as_secs_f64()
}

fn unix_timestamp_string(time: SystemTime) -> String {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("{}.{}", duration.as_secs(), duration.subsec_millis()),
        Err(_) => "0.000".to_string(),
    }
}

fn parity_summary_command(args: ParitySummaryArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.report)
        .with_context(|| format!("failed to read {}", args.report.display()))?;
    let report: ParityFixtureSuiteReport = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.report.display()))?;
    println!(
        "{}",
        serde_json::to_string_pretty(&parity_summary_json(&report))?
    );
    Ok(())
}

fn parity_summary_json(report: &ParityFixtureSuiteReport) -> serde_json::Value {
    serde_json::json!({
        "passed": report.passed,
        "gatingFailures": report
            .cases
            .iter()
            .filter(|case| case.gating && !case.passed)
            .map(parity_case_gating_failure_json)
            .collect::<Vec<_>>(),
        "cases": report.cases.iter().map(parity_case_summary_json).collect::<Vec<_>>(),
    })
}

fn parity_case_gating_failure_json(case: &ParityFixtureCaseReport) -> serde_json::Value {
    serde_json::json!({
        "name": case.name,
        "strictComparisonFailures": strict_comparison_failures(case),
        "missingRequiredDiagnostics": case.missing_required_diagnostics,
        "elapsedSeconds": case.elapsed_seconds,
        "startedAt": case.started_at,
        "timedOut": case.timed_out,
    })
}

fn parity_case_summary_json(case: &ParityFixtureCaseReport) -> serde_json::Value {
    let expected_target = case
        .report
        .as_ref()
        .map(|report| serde_json::json!(report.expected_target));
    let strict_comparison_failures = strict_comparison_failures(case);
    let report_only_differences = report_only_differences(case);
    let expected_json_matches = case.report.as_ref().and_then(|report| {
        report.expected.as_ref().map(|_| {
            let text = report.expected_text_matches.unwrap_or(true);
            let segment_count = report.expected_segment_count_matches.unwrap_or(true);
            serde_json::json!({
                "passed": text && segment_count,
                "text": text,
                "segmentCount": segment_count,
            })
        })
    });

    serde_json::json!({
        "name": case.name,
        "passed": case.passed,
        "gating": case.gating,
        "expectedTarget": expected_target,
        "strictComparisonFailures": strict_comparison_failures,
        "reportOnlyDifferences": report_only_differences,
        "expectedJsonMatches": expected_json_matches,
        "missingRequiredDiagnostics": case.missing_required_diagnostics,
        "elapsedSeconds": case.elapsed_seconds,
        "startedAt": case.started_at,
        "timedOut": case.timed_out,
    })
}

fn strict_comparison_failures(case: &ParityFixtureCaseReport) -> Vec<String> {
    let mut failures = Vec::new();
    if let Some(error) = &case.error {
        failures.push(error.clone());
    }
    if let Some(report) = &case.report {
        if !report.comparison.passed {
            failures.extend(
                report
                    .comparison
                    .differences
                    .iter()
                    .filter(|difference| !is_report_only_difference(difference))
                    .cloned(),
            );
        }
        if report.expected_text_matches == Some(false) {
            failures.push("expected transcript text differs".to_string());
        }
        if report.expected_segment_count_matches == Some(false) {
            failures.push("expected transcript segment count differs".to_string());
        }
    }
    failures.extend(
        case.expected_output_matches
            .iter()
            .filter(|output| output.gating && !output.passed)
            .filter_map(output_difference_summary),
    );
    failures.extend(
        case.missing_required_diagnostics
            .iter()
            .map(|diagnostic| format!("missing required diagnostic: {diagnostic}")),
    );
    failures
}

fn report_only_differences(case: &ParityFixtureCaseReport) -> Vec<String> {
    let mut differences = Vec::new();
    if let Some(report) = &case.report {
        differences.extend(report.comparison.diagnostic_differences.iter().cloned());
        differences.extend(
            report
                .comparison
                .differences
                .iter()
                .filter(|difference| is_report_only_difference(difference))
                .cloned(),
        );
    }
    differences.extend(
        case.expected_output_matches
            .iter()
            .filter(|output| !output.gating && !output.passed)
            .filter_map(output_difference_summary),
    );
    differences
}

fn is_report_only_difference(difference: &str) -> bool {
    difference.starts_with("report-only: ")
}

fn output_difference_summary(output: &native_whisperx::ExpectedOutputComparison) -> Option<String> {
    output.difference.as_ref().map(|difference| {
        format!(
            "{} {} output differs: {difference}",
            output.format.as_transcription_format(),
            output_comparison_name(output.comparison)
        )
    })
}

fn output_comparison_name(comparison: OutputComparisonMode) -> &'static str {
    match comparison {
        OutputComparisonMode::Exact => "exact",
        OutputComparisonMode::JsonSemantic => "jsonSemantic",
        OutputComparisonMode::SubtitleSemantic => "subtitleSemantic",
    }
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
    if fixture.output.formats.contains(&OutputFormat::All)
        || formats.contains(&OutputFormat::All)
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

    let whisperx_diarization = fixture
        .whisperx_diarization
        .as_ref()
        .unwrap_or(&fixture.diarization);
    if whisperx_diarization.enabled {
        args.push("--diarize".to_string());
        args.extend([
            "--diarize_model".to_string(),
            whisperx_diarization.model_id.clone(),
        ]);
        push_cli_arg_display(args, "--min_speakers", whisperx_diarization.min_speakers);
        push_cli_arg_display(args, "--max_speakers", whisperx_diarization.max_speakers);
        if let Some(token) = fixture
            .whisperx_diarization
            .as_ref()
            .and_then(|diarization| diarization.hf_token.clone())
            .or_else(|| whisperx_diarization.hf_token.clone())
            .or_else(|| {
                whisperx_diarization
                    .hf_token_env
                    .as_ref()
                    .and_then(|name| std::env::var(name).ok())
            })
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
    if whisperx_diarization.return_speaker_embeddings {
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

impl From<CliAssignmentPolicy> for AssignmentPolicy {
    fn from(value: CliAssignmentPolicy) -> Self {
        match value {
            CliAssignmentPolicy::Majority => Self::Majority,
            CliAssignmentPolicy::NearestStart => Self::NearestStart,
            CliAssignmentPolicy::StrictContained => Self::StrictContained,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speed_comparison_reports_native_faster_and_speedup_ratio() {
        let comparison = bench_speed_comparison(10.0, Some(25.0));

        assert_eq!(comparison.native_faster_than_whisperx, Some(true));
        assert_eq!(comparison.native_speedup_ratio, Some(2.5));
    }

    #[test]
    fn speed_comparison_reports_native_regression() {
        let comparison = bench_speed_comparison(25.0, Some(10.0));

        assert_eq!(comparison.native_faster_than_whisperx, Some(false));
        assert_eq!(comparison.native_speedup_ratio, Some(0.4));
    }

    #[test]
    fn speed_comparison_is_absent_without_reference_run() {
        let comparison = bench_speed_comparison(10.0, None);

        assert_eq!(comparison.native_faster_than_whisperx, None);
        assert_eq!(comparison.native_speedup_ratio, None);
    }

    #[test]
    fn speed_gate_fails_only_when_reference_proves_native_slower() {
        assert!(!bench_iteration_passes_speed_gate(&serde_json::json!({
            "nativeFasterThanWhisperx": false
        })));
        assert!(bench_iteration_passes_speed_gate(&serde_json::json!({
            "nativeFasterThanWhisperx": true
        })));
        assert!(bench_iteration_passes_speed_gate(&serde_json::json!({})));
    }

    #[test]
    fn native_bench_config_uses_whisperx_batch_size_when_native_is_unspecified() {
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            native_asr: AsrConfig {
                max_batch_size: None,
                ..AsrConfig::default()
            },
            whisperx: ExternalWhisperxConfig {
                batch_size: Some(8),
                ..ExternalWhisperxConfig::default()
            },
            ..bench_fixture_defaults()
        };

        let config = native_bench_config(&fixture);

        assert_eq!(config.asr.max_batch_size, Some(8));
    }

    #[test]
    fn native_bench_config_keeps_explicit_native_batch_size() {
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            native_asr: AsrConfig {
                max_batch_size: Some(6),
                ..AsrConfig::default()
            },
            whisperx: ExternalWhisperxConfig {
                batch_size: Some(8),
                ..ExternalWhisperxConfig::default()
            },
            ..bench_fixture_defaults()
        };

        let config = native_bench_config(&fixture);

        assert_eq!(config.asr.max_batch_size, Some(6));
    }

    #[test]
    fn whisperx_bench_config_uses_native_fixture_device_target() {
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            native_asr: AsrConfig {
                device: DevicePreference::Cuda,
                device_index: Some("0".to_string()),
                ..AsrConfig::default()
            },
            ..bench_fixture_defaults()
        };

        let config = whisperx_bench_config(&fixture);

        assert_eq!(config.asr.device, DevicePreference::Cuda);
        assert_eq!(config.asr.device_index.as_deref(), Some("0"));
    }

    #[test]
    fn whisperx_bench_config_uses_fixture_reference_batch_size() {
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            whisperx: ExternalWhisperxConfig {
                batch_size: Some(8),
                ..ExternalWhisperxConfig::default()
            },
            ..bench_fixture_defaults()
        };

        let config = whisperx_bench_config(&fixture);

        assert_eq!(config.asr.max_batch_size, Some(8));
    }

    #[test]
    fn infers_ort_dylib_path_from_whisperx_environment_for_native_onnx_vad() {
        let temp = tempfile::tempdir().expect("tempdir");
        let whisperx = temp.path().join("bin").join("whisperx");
        fs::create_dir_all(whisperx.parent().expect("bin")).expect("bin dir");
        fs::write(&whisperx, "").expect("whisperx");
        let capi = temp
            .path()
            .join("lib")
            .join("python3.11")
            .join("site-packages")
            .join("onnxruntime")
            .join("capi");
        fs::create_dir_all(&capi).expect("capi dir");
        let dylib = capi.join("libonnxruntime.so.1.27.0");
        fs::write(&dylib, "").expect("dylib");
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            vad: VadConfig {
                method: VadMethod::Silero,
                ..VadConfig::default()
            },
            whisperx: ExternalWhisperxConfig {
                command: whisperx,
                ..ExternalWhisperxConfig::default()
            },
            ..bench_fixture_defaults()
        };

        assert_eq!(
            inferred_ort_dylib_path_with_env(&fixture, None),
            Some(dylib)
        );
    }

    #[test]
    fn does_not_infer_ort_dylib_when_env_is_explicit() {
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            vad: VadConfig {
                method: VadMethod::Silero,
                ..VadConfig::default()
            },
            ..bench_fixture_defaults()
        };

        assert_eq!(
            inferred_ort_dylib_path_with_env(&fixture, Some(OsString::from("/explicit/lib.so"))),
            None
        );
    }

    #[test]
    fn does_not_infer_ort_dylib_for_energy_vad() {
        let temp = tempfile::tempdir().expect("tempdir");
        let whisperx = temp.path().join("bin").join("whisperx");
        fs::create_dir_all(whisperx.parent().expect("bin")).expect("bin dir");
        fs::write(&whisperx, "").expect("whisperx");
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            vad: VadConfig {
                method: VadMethod::Energy,
                ..VadConfig::default()
            },
            whisperx: ExternalWhisperxConfig {
                command: whisperx,
                ..ExternalWhisperxConfig::default()
            },
            ..bench_fixture_defaults()
        };

        assert_eq!(inferred_ort_dylib_path_with_env(&fixture, None), None);
    }

    #[test]
    fn bench_phase_json_exposes_native_total_seconds() {
        let phases = bench_phase_json(
            &[
                "phaseDecodeSeconds=0.100000".to_string(),
                "phaseVadSeconds=0.200000".to_string(),
                "phaseAsrSeconds=0.300000".to_string(),
                "phaseAlignmentSeconds=0.400000".to_string(),
                "phaseOutputSeconds=0.500000".to_string(),
                "phaseNativeTotalSeconds=1.500000".to_string(),
            ],
            1.6,
        );

        assert_eq!(phases["decodeSeconds"], serde_json::json!(0.1));
        assert_eq!(phases["vadSeconds"], serde_json::json!(0.2));
        assert_eq!(phases["asrSeconds"], serde_json::json!(0.3));
        assert_eq!(phases["alignmentSeconds"], serde_json::json!(0.4));
        assert_eq!(phases["outputSeconds"], serde_json::json!(0.5));
        assert_eq!(phases["nativeTotalSeconds"], serde_json::json!(1.5));
        assert_eq!(phases["totalElapsedSeconds"], serde_json::json!(1.6));
    }

    fn bench_fixture_defaults() -> ParityFixtureCase {
        ParityFixtureCase {
            name: String::new(),
            gating: false,
            input: PathBuf::new(),
            clip_seconds: None,
            timeout_seconds: None,
            expected_json: None,
            expected_target: native_whisperx::ExpectedTranscriptTarget::Native,
            comparison: ParityComparisonConfig::default(),
            expected_outputs: Vec::new(),
            native_asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            whisperx_diarization: None,
            whisperx: ExternalWhisperxConfig::default(),
            language: None,
            output: OutputConfig::default(),
            required_diagnostics: Vec::new(),
        }
    }
}

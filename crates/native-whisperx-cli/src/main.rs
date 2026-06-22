use std::ffi::OsString;
use std::path::PathBuf;

mod cmd;
mod ui;

use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use native_whisperx::{AssignmentPolicy, SegmentResolution, SpeakerCorrectionRange, VadMethod};

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
    Speakers(SpeakersArgs),
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
    #[command(name = "__parity-bench-multi-input-case", hide = true)]
    ParityBenchMultiInputCase(ParityBenchMultiInputCaseArgs),
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
    #[arg(
        long = "diarization-model-bundle",
        visible_alias = "diarization_model_bundle"
    )]
    diarization_model_bundle: Option<PathBuf>,
    #[arg(
        long = "diarization-manifest-file",
        visible_alias = "diarization_manifest_file"
    )]
    diarization_manifest_file: Option<String>,
    #[arg(
        long = "diarization-segmentation-model-file",
        visible_alias = "diarization_segmentation_model_file"
    )]
    diarization_segmentation_model_file: Option<String>,
    #[arg(
        long = "diarization-embedding-model-file",
        visible_alias = "diarization_embedding_model_file"
    )]
    diarization_embedding_model_file: Option<String>,
    #[arg(
        long = "diarization-plda-transform-file",
        visible_alias = "diarization_plda_transform_file"
    )]
    diarization_plda_transform_file: Option<String>,
    #[arg(
        long = "diarization-plda-model-file",
        visible_alias = "diarization_plda_model_file"
    )]
    diarization_plda_model_file: Option<String>,
    #[arg(
        long = "diarization-clustering-config-file",
        visible_alias = "diarization_clustering_config_file"
    )]
    diarization_clustering_config_file: Option<String>,
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
    #[command(flatten)]
    speaker_directory: SpeakerDirectoryArgs,
    #[arg(long = "no-speaker-library", visible_alias = "no_speaker_library", action = ArgAction::SetTrue)]
    no_speaker_library: bool,
    #[arg(long = "no-speaker-store", visible_alias = "no_speaker_store", action = ArgAction::SetTrue)]
    no_speaker_store: bool,
    #[arg(long = "no-save-draft-speakers", visible_alias = "no_save_draft_speakers", action = ArgAction::SetTrue)]
    no_save_draft_speakers: bool,
    #[arg(long = "no-use-draft-speakers", visible_alias = "no_use_draft_speakers", action = ArgAction::SetTrue)]
    no_use_draft_speakers: bool,
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
struct SpeakersArgs {
    #[command(subcommand)]
    command: SpeakersCommand,
}

#[derive(Debug, Subcommand)]
enum SpeakersCommand {
    Path(SpeakersPathArgs),
    List(SpeakersListArgs),
    Correct(SpeakersCorrectArgs),
    Validate(SpeakersValidateArgs),
    RebuildTrace(SpeakersRebuildTraceArgs),
    Open(SpeakersOpenArgs),
}

#[derive(Debug, Args)]
struct SpeakersPathArgs {
    #[command(flatten)]
    directory: SpeakerDirectoryArgs,
}

#[derive(Debug, Args)]
struct SpeakersListArgs {
    #[command(flatten)]
    directory: SpeakerDirectoryArgs,
    #[arg(long = "include-drafts", visible_alias = "include_drafts", action = ArgAction::SetTrue)]
    include_drafts: bool,
}

#[derive(Debug, Args)]
struct SpeakersCorrectArgs {
    #[arg(long)]
    transcript: PathBuf,
    #[arg(long)]
    audio: PathBuf,
    #[arg(long = "from")]
    from_speaker: String,
    #[arg(long = "to")]
    to_label: String,
    #[arg(long = "speaker-id", visible_alias = "speaker_id")]
    speaker_id: Option<String>,
    #[command(flatten)]
    directory: SpeakerDirectoryArgs,
    #[arg(long = "range", value_parser = crate::cmd::speaker::parse_speaker_range)]
    ranges: Vec<SpeakerCorrectionRange>,
    #[arg(long = "output-dir", short = 'o', visible_alias = "output_dir")]
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
}

#[derive(Debug, Args)]
struct SpeakersValidateArgs {
    #[command(flatten)]
    directory: SpeakerDirectoryArgs,
}

#[derive(Debug, Args)]
struct SpeakersRebuildTraceArgs {
    #[command(flatten)]
    directory: SpeakerDirectoryArgs,
    #[arg(long = "scan-root", visible_alias = "scan_root")]
    scan_root: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct SpeakersOpenArgs {
    #[command(flatten)]
    directory: SpeakerDirectoryArgs,
    #[arg(long = "no-browser", visible_alias = "no_browser", action = ArgAction::SetTrue)]
    no_browser: bool,
    #[arg(long = "print-url", visible_alias = "print_url", action = ArgAction::SetTrue)]
    print_url: bool,
    #[arg(long, default_value_t = 0)]
    port: u16,
}

#[derive(Debug, Clone, Args)]
struct SpeakerDirectoryArgs {
    #[arg(long, value_enum, default_value_t = CliSpeakerDirectoryScope::Auto)]
    scope: CliSpeakerDirectoryScope,
    #[arg(long = "speaker-directory", visible_alias = "speaker_directory")]
    speaker_directory: Option<PathBuf>,
    #[arg(long = "speaker-store", visible_alias = "speaker_store")]
    speaker_store: Option<PathBuf>,
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
    #[arg(long = "report-only", visible_alias = "report_only")]
    report_only: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct ParitySummaryArgs {
    report: PathBuf,
    #[arg(long = "preflight-report", visible_alias = "preflight_report")]
    preflight_report: Option<PathBuf>,
    #[arg(long = "allow-missing-report", visible_alias = "allow_missing_report")]
    allow_missing_report: bool,
    #[arg(long)]
    suite: Option<String>,
    #[arg(long)]
    features: Option<String>,
    #[arg(long)]
    runner: Option<String>,
    #[arg(long)]
    manifest: Option<PathBuf>,
    #[arg(long = "output-dir", visible_alias = "output_dir")]
    output_dir: Option<PathBuf>,
    #[arg(long = "smoke-root", visible_alias = "smoke_root")]
    smoke_root: Option<PathBuf>,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    model_dir: Option<PathBuf>,
    #[arg(long = "whisperx-command", visible_alias = "whisperx_command")]
    whisperx_command: Option<PathBuf>,
    #[arg(long = "progress-log", visible_alias = "progress_log")]
    progress_log: Option<PathBuf>,
    #[arg(long = "ort-dylib-path", visible_alias = "ort_dylib_path")]
    ort_dylib_path: Option<PathBuf>,
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
struct ParityBenchMultiInputCaseArgs {
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

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliSpeakerDirectoryScope {
    Auto,
    Local,
    Global,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_from(cmd::support::compatible_args());
    if cli.python_version {
        println!(
            "native-whisperx {} (Rust runtime)",
            env!("CARGO_PKG_VERSION")
        );
        return Ok(());
    }
    match cli.command {
        Some(Command::Transcribe(args)) => cmd::transcribe::transcribe_command(*args),
        Some(Command::ImportWhisperx(args)) => cmd::transcribe::import_whisperx_command(args),
        Some(Command::Speakers(args)) => cmd::speaker::speakers_command(args),
        Some(Command::InspectModels(args)) => cmd::inspect::inspect_models_command(args),
        Some(Command::Parity(args)) => cmd::parity::parity_command(args),
        Some(Command::ParityFixtures(args)) => cmd::parity::parity_fixtures_command(args),
        Some(Command::ParityBench(args)) => cmd::parity::parity_bench_command(args),
        Some(Command::ParitySummary(args)) => cmd::parity::parity_summary_command(args),
        Some(Command::ParityPreflight(args)) => cmd::parity::parity_preflight_command(args),
        Some(Command::ParityGoldens(args)) => cmd::parity::parity_goldens_command(args),
        Some(Command::ParityFixtureCase(args)) => cmd::parity::parity_fixture_case_command(args),
        Some(Command::ParityBenchCase(args)) => cmd::parity::parity_bench_case_command(args),
        Some(Command::ParityBenchMultiInputCase(args)) => {
            cmd::parity::parity_bench_multi_input_case_command(args)
        }
        None => {
            Cli::parse_from([OsString::from("native-whisperx"), OsString::from("--help")]);
            Ok(())
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

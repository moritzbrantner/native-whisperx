use std::ffi::OsString;
use std::path::PathBuf;

mod args;
mod cmd;
mod ui;

use clap::{ArgAction, Args, Parser, Subcommand};
use native_whisperx::SpeakerCorrectionRange;

#[allow(unused_imports)]
pub(crate) use args::{
    CliAlignmentInterpolationMethod, CliAssignmentPolicy, CliDevicePreference, CliOutputFormat,
    CliProvider, CliSegmentResolution, CliSpeakerDirectoryScope, CliTask, CliVadMethod,
    TranscribeArgs,
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
pub(crate) struct SpeakerDirectoryArgs {
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
        None => {
            Cli::parse_from([OsString::from("native-whisperx"), OsString::from("--help")]);
            Ok(())
        }
    }
}

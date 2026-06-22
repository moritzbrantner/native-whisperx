use std::ffi::OsString;
use std::path::PathBuf;

mod args;
mod cmd;
mod preflight;
mod ui;

use clap::{ArgAction, Args, Parser, Subcommand};
use native_whisperx::SpeakerCorrectionRange;

#[allow(unused_imports)]
pub(crate) use args::{
    CliAlignmentInterpolationMethod, CliAssignmentPolicy, CliDevicePreference, CliOutputFormat,
    CliProvider, CliSegmentResolution, CliSpeakerDirectoryScope, CliTask, CliVadMethod,
    TranscribeArgs,
};
use cmd::import::ImportWhisperxArgs;
use cmd::inspect::InspectModelsArgs;
use cmd::parity::{
    ParityArgs, ParityBenchArgs, ParityBenchCaseArgs, ParityFixtureCaseArgs, ParityFixturesArgs,
    ParityGoldensArgs, ParitySummaryArgs,
};
use preflight::ParityPreflightArgs;

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
        Some(Command::ImportWhisperx(args)) => cmd::import::import_whisperx_command(args),
        Some(Command::Speakers(args)) => cmd::speaker::speakers_command(args),
        Some(Command::InspectModels(args)) => cmd::inspect::inspect_models_command(args),
        Some(Command::Parity(args)) => cmd::parity::parity_command(args),
        Some(Command::ParityFixtures(args)) => cmd::parity::parity_fixtures_command(args),
        Some(Command::ParityBench(args)) => cmd::parity::parity_bench_command(args),
        Some(Command::ParitySummary(args)) => cmd::parity::parity_summary_command(args),
        Some(Command::ParityPreflight(args)) => preflight::parity_preflight_command(args),
        Some(Command::ParityGoldens(args)) => cmd::parity::parity_goldens_command(args),
        Some(Command::ParityFixtureCase(args)) => cmd::parity::parity_fixture_case_command(args),
        Some(Command::ParityBenchCase(args)) => cmd::parity::parity_bench_case_command(args),
        None => {
            Cli::parse_from([OsString::from("native-whisperx"), OsString::from("--help")]);
            Ok(())
        }
    }
}

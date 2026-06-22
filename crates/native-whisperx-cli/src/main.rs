//! native-whisperx CLI argument parsing and command dispatch.

use std::ffi::OsString;

mod args;
mod cmd;
mod preflight;
mod ui;

use clap::{ArgAction, Parser, Subcommand};

#[allow(unused_imports)]
pub(crate) use args::{
    CliAlignmentInterpolationMethod, CliAssignmentPolicy, CliDevicePreference, CliOutputFormat,
    CliProvider, CliSegmentResolution, CliSpeakerDirectoryScope, CliTask, CliVadMethod,
    TranscribeArgs,
};
use cmd::import::ImportWhisperxArgs;
use cmd::inspect::InspectModelsArgs;
use cmd::parity::{
    ParityArgs, ParityBenchArgs, ParityBenchCaseArgs, ParityBenchMultiInputCaseArgs,
    ParityFixtureCaseArgs, ParityFixturesArgs, ParityGoldensArgs, ParitySummaryArgs,
};
use cmd::speaker::SpeakersArgs;
use preflight::ParityPreflightArgs;

pub(crate) use cmd::speaker::SpeakerDirectoryArgs;

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
        Some(Command::ParityBenchMultiInputCase(args)) => {
            cmd::parity::parity_bench_multi_input_case_command(args)
        }
        None => {
            Cli::parse_from([OsString::from("native-whisperx"), OsString::from("--help")]);
            Ok(())
        }
    }
}

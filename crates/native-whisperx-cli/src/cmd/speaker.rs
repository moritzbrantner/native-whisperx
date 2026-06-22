//! Speaker Directory CLI commands and local management UI launcher.

use std::fs;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::path::PathBuf;

use crate::ui;
use crate::{CliOutputFormat, CliSpeakerDirectoryScope};
use anyhow::Context;
use clap::{ArgAction, Args, Subcommand};
use native_whisperx::{
    correct_speaker, import_whisperx_json, list_speaker_profiles, rebuild_speaker_trace,
    resolve_speaker_directory, validate_speaker_library, InputSource, OutputConfig, OutputFormat,
    ResolvedSpeakerDirectoryScope, SpeakerCorrectionRange, SpeakerCorrectionRequest,
    SubtitleConfig,
};

use super::resolve_cli_path_with_root;

#[derive(Debug, Args)]
pub(crate) struct SpeakersArgs {
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
    #[arg(long = "range", value_parser = parse_speaker_range)]
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
    pub(crate) scope: CliSpeakerDirectoryScope,
    #[arg(long = "speaker-directory", visible_alias = "speaker_directory")]
    pub(crate) speaker_directory: Option<PathBuf>,
    #[arg(long = "speaker-store", visible_alias = "speaker_store")]
    pub(crate) speaker_store: Option<PathBuf>,
}

pub(crate) fn speakers_command(args: SpeakersArgs) -> anyhow::Result<()> {
    match args.command {
        SpeakersCommand::Path(args) => speakers_path_command(args),
        SpeakersCommand::List(args) => speakers_list_command(args),
        SpeakersCommand::Correct(args) => speakers_correct_command(args),
        SpeakersCommand::Validate(args) => speakers_validate_command(args),
        SpeakersCommand::RebuildTrace(args) => speakers_rebuild_trace_command(args),
        SpeakersCommand::Open(args) => speakers_open_command(args),
    }
}

fn speakers_path_command(args: SpeakersPathArgs) -> anyhow::Result<()> {
    let resolved = resolve_cli_speaker_directory(args.directory)?;
    println!("{}", resolved.path.display());
    Ok(())
}

fn speakers_list_command(args: SpeakersListArgs) -> anyhow::Result<()> {
    let profiles = list_speaker_profiles(args.directory.try_into()?, args.include_drafts)?;
    println!("{}", serde_json::to_string_pretty(&profiles)?);
    Ok(())
}

fn speakers_correct_command(args: SpeakersCorrectArgs) -> anyhow::Result<()> {
    let transcript_bytes = fs::read(&args.transcript)
        .with_context(|| format!("failed to read {}", args.transcript.display()))?;
    let transcript = import_whisperx_json(&transcript_bytes)?;
    validate_corrected_output_does_not_overwrite_source(&args)?;
    let report = correct_speaker(SpeakerCorrectionRequest {
        transcript,
        audio: InputSource::Path { path: args.audio },
        from_speaker: args.from_speaker,
        to_label: args.to_label,
        speaker_id: args.speaker_id,
        ranges: args.ranges,
        speaker_directory: args.directory.try_into()?,
        output: OutputConfig {
            output_dir: args.output_dir,
            formats: args.formats.iter().copied().map(Into::into).collect(),
            basename: args.basename,
            pretty_json: true,
            subtitles: SubtitleConfig::default(),
        },
    })?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn validate_corrected_output_does_not_overwrite_source(
    args: &SpeakersCorrectArgs,
) -> anyhow::Result<()> {
    let Some(output_dir) = &args.output_dir else {
        return Ok(());
    };
    let basename = args
        .basename
        .clone()
        .or_else(|| {
            args.transcript
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|stem| stem.to_string())
        })
        .unwrap_or_else(|| "transcript".to_string());
    let current_dir = std::env::current_dir()?;
    let source = resolve_cli_path_with_root(args.transcript.clone(), &current_dir);
    let output_dir = resolve_cli_path_with_root(output_dir.clone(), &current_dir);
    for format in args
        .formats
        .iter()
        .copied()
        .flat_map(expand_cli_output_format)
    {
        let output = output_dir.join(format!("{basename}.{}", format.extension()));
        if output == source {
            anyhow::bail!(
                "speaker correction refuses to overwrite the source transcript {}; choose a different --output-dir or --basename",
                source.display()
            );
        }
    }
    Ok(())
}

fn expand_cli_output_format(format: CliOutputFormat) -> Vec<OutputFormat> {
    match OutputFormat::from(format) {
        OutputFormat::All => vec![
            OutputFormat::Json,
            OutputFormat::Srt,
            OutputFormat::Vtt,
            OutputFormat::Txt,
            OutputFormat::Tsv,
            OutputFormat::Audacity,
        ],
        format => vec![format],
    }
}

pub(crate) fn parse_speaker_range(value: &str) -> Result<SpeakerCorrectionRange, String> {
    let Some((start, end)) = value.split_once("..") else {
        return Err("speaker correction ranges must use START..END".to_string());
    };
    let start_seconds = start
        .parse::<f64>()
        .map_err(|error| format!("invalid speaker correction range start: {error}"))?;
    let end_seconds = end
        .parse::<f64>()
        .map_err(|error| format!("invalid speaker correction range end: {error}"))?;
    let range = SpeakerCorrectionRange {
        start_seconds,
        end_seconds,
    };
    if !start_seconds.is_finite() || !end_seconds.is_finite() || end_seconds <= start_seconds {
        return Err(
            "speaker correction ranges must be finite and have positive duration".to_string(),
        );
    }
    Ok(range)
}

fn speakers_validate_command(args: SpeakersValidateArgs) -> anyhow::Result<()> {
    let resolved = resolve_cli_speaker_directory(args.directory)?;
    let validation = validate_speaker_library(&resolved.path)?;
    println!(
        "Speaker Library valid: {} (profiles: {})",
        validation.path.display(),
        validation.profile_count
    );
    Ok(())
}

fn speakers_rebuild_trace_command(args: SpeakersRebuildTraceArgs) -> anyhow::Result<()> {
    let resolved = resolve_cli_speaker_directory(args.directory)?;
    let current_dir = std::env::current_dir()?;
    let scan_root = match args.scan_root {
        Some(path) => resolve_cli_path_with_root(path, &current_dir),
        None if resolved.scope == ResolvedSpeakerDirectoryScope::Global => {
            anyhow::bail!(
                "global Speaker Directory trace rebuilds require --scan-root to avoid indexing unrelated files"
            );
        }
        None => current_dir,
    };

    let report = rebuild_speaker_trace(&resolved.path, &scan_root)?;
    for error in &report.trace.errors {
        eprintln!(
            "warning: {}: {}",
            error.source_file.display(),
            error.message
        );
    }
    let file_count = report
        .trace
        .speakers
        .iter()
        .map(|speaker| speaker.files.len())
        .sum::<usize>();
    println!(
        "Speaker Trace rebuilt: {} (speakers: {}, files: {}, errors: {})",
        report.trace_path.display(),
        report.trace.speakers.len(),
        file_count,
        report.trace.errors.len()
    );
    println!(
        "Trace scan report: scanned files: {}, accepted entries: {}, ignored non-JSON files: {}, malformed JSON errors: {}",
        report.stats.scanned_files,
        report.stats.accepted_entries,
        report.stats.ignored_non_json_files,
        report.stats.malformed_json_errors
    );
    Ok(())
}

fn speakers_open_command(args: SpeakersOpenArgs) -> anyhow::Result<()> {
    let resolved = resolve_cli_speaker_directory(args.directory)?;
    let session_token = ui::server::generate_session_token();
    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, args.port)))
        .context("failed to bind Speaker Directory Web UI to loopback")?;
    let local_addr = listener.local_addr()?;
    let url = format!("http://{}:{}/", local_addr.ip(), local_addr.port());

    if args.print_url {
        println!("{url}");
    } else {
        println!("Speaker Directory Web UI: {url}");
    }
    std::io::stdout().flush()?;

    if !args.no_browser && !args.print_url {
        if let Err(error) = ui::server::open_browser(&url) {
            eprintln!("warning: failed to open browser: {error}");
        }
    }

    ui::server::serve_speaker_directory(listener, resolved, session_token)
}

fn resolve_cli_speaker_directory(
    args: SpeakerDirectoryArgs,
) -> anyhow::Result<native_whisperx::ResolvedSpeakerDirectory> {
    let current_dir = std::env::current_dir()?;
    Ok(resolve_speaker_directory(&args.try_into()?, &current_dir)?)
}

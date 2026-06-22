use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use native_whisperx::{run_parity_preflight, ParityFixtureSuite};

use crate::cmd::parity::smoke_root_or_arg;
use crate::cmd::support::absolute_from_cwd;

#[derive(Debug, Args)]
pub(crate) struct ParityPreflightArgs {
    pub(crate) manifest: PathBuf,
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,
    #[arg(
        long,
        visible_alias = "whisperx_command",
        default_value = ".audio-tools/whisperx-venv/bin/whisperx"
    )]
    pub(crate) whisperx_command: PathBuf,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "require-expected", visible_alias = "require_expected")]
    pub(crate) require_expected: bool,
    #[arg(long = "include-non-gating", visible_alias = "include_non_gating")]
    pub(crate) include_non_gating: bool,
    #[arg(long)]
    pub(crate) json: bool,
}

pub(crate) fn parity_preflight_command(args: ParityPreflightArgs) -> anyhow::Result<()> {
    let bytes = std::fs::read(&args.manifest)
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

//! Offline verification commands for caller-provided model bundles.

use clap::{Args, ValueEnum};
use native_whisperx::{verify_pyannote_diarization_bundle, verify_pyannote_vad_bundle};

use super::*;

#[derive(Debug, Args)]
pub(crate) struct BundleVerifyArgs {
    #[arg(long)]
    pub(crate) bundle: PathBuf,
    #[arg(long, value_enum)]
    pub(crate) kind: CliBundleKind,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliBundleKind {
    PyannoteVad,
    PyannoteDiarization,
}

pub(crate) fn bundle_verify_command(args: BundleVerifyArgs) -> anyhow::Result<()> {
    match args.kind {
        CliBundleKind::PyannoteVad => {
            println!(
                "{}",
                serde_json::to_string_pretty(&verify_pyannote_vad_bundle(&args.bundle)?)?
            );
        }
        CliBundleKind::PyannoteDiarization => {
            println!(
                "{}",
                serde_json::to_string_pretty(&verify_pyannote_diarization_bundle(&args.bundle)?)?
            );
        }
    }
    Ok(())
}

use super::*;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct ImportWhisperxArgs {
    pub(crate) whisperx_json: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

pub(crate) fn import_whisperx_command(args: ImportWhisperxArgs) -> anyhow::Result<()> {
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

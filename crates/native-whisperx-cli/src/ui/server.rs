//! Local HTTP server lifecycle for the Speaker Directory management UI.

use std::net::TcpListener;
use std::process::Command as ProcessCommand;

use anyhow::Context;

use crate::ui::handlers::handle_speaker_directory_request;

pub(crate) fn serve_speaker_directory(
    listener: TcpListener,
    mut resolved: native_whisperx::ResolvedSpeakerDirectory,
    session_token: String,
) -> anyhow::Result<()> {
    let mut workbench =
        super::workbench::WorkbenchSession::new(std::env::current_dir()?, resolved.path.clone())?;
    let mut library = super::library::LibrarySession::default();
    let mut review = super::review::ReviewSession::default();
    for stream in listener.incoming() {
        let stream = stream?;
        if let Err(error) = handle_speaker_directory_request(
            stream,
            &mut resolved,
            &session_token,
            &mut workbench,
            &mut library,
            &mut review,
        ) {
            eprintln!("warning: failed to serve Speaker Directory request: {error}");
        }
    }
    Ok(())
}

pub(crate) fn generate_session_token() -> anyhow::Result<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes)
        .map_err(|error| anyhow::anyhow!("secure session entropy unavailable: {error}"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

pub(crate) fn open_browser(url: &str) -> anyhow::Result<()> {
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = ProcessCommand::new("open");
        command.arg(url);
        command
    };
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = ProcessCommand::new("cmd");
        command.args(["/C", "start", "", url]);
        command
    };
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    let mut command = {
        let mut command = ProcessCommand::new("xdg-open");
        command.arg(url);
        command
    };

    command
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .context("browser launcher did not start")?;
    Ok(())
}

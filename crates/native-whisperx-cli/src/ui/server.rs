use std::io::Read;
use std::net::TcpListener;
use std::process::Command as ProcessCommand;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;

use crate::ui::handlers::handle_speaker_directory_request;

pub(crate) fn serve_speaker_directory(
    listener: TcpListener,
    resolved: native_whisperx::ResolvedSpeakerDirectory,
    session_token: String,
) -> anyhow::Result<()> {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(error) =
                    handle_speaker_directory_request(stream, &resolved, &session_token)
                {
                    eprintln!("warning: failed to serve Speaker Directory request: {error}");
                }
            }
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

pub(crate) fn generate_session_token() -> String {
    let mut bytes = [0u8; 32];
    if fill_session_token_bytes(&mut bytes).is_err() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        let fallback = format!("{}:{now}:{:p}", std::process::id(), &bytes);
        for (index, byte) in fallback.as_bytes().iter().enumerate() {
            bytes[index % bytes.len()] ^= *byte;
        }
    }
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(unix)]
fn fill_session_token_bytes(bytes: &mut [u8]) -> std::io::Result<()> {
    let mut file = std::fs::File::open("/dev/urandom")?;
    file.read_exact(bytes)
}

#[cfg(not(unix))]
fn fill_session_token_bytes(_bytes: &mut [u8]) -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "platform random source unavailable",
    ))
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

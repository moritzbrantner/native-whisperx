use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;

use crate::ui::handlers::SPEAKER_DIRECTORY_HTML;

pub(crate) fn serve_speaker_directory(
    listener: TcpListener,
    resolved: native_whisperx::ResolvedSpeakerDirectory,
    session_token: String,
) -> anyhow::Result<()> {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(error) = handle_speaker_directory_request(stream, &resolved, &session_token) {
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

fn handle_speaker_directory_request(
    mut stream: TcpStream,
    resolved: &native_whisperx::ResolvedSpeakerDirectory,
    session_token: &str,
) -> anyhow::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let mut headers = Vec::<(String, String)>::new();
    let mut content_length = 0usize;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header)? == 0 || header == "\r\n" || header == "\n" {
            break;
        }
        if let Some((name, value)) = header.trim_end().split_once(':') {
            let name = name.trim().to_string();
            let value = value.trim().to_string();
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.parse::<usize>().unwrap_or(0);
            }
            headers.push((name, value));
        }
    }
    let mut body = vec![0; content_length];
    if content_length > 0 {
        reader.read_exact(&mut body)?;
    }
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let target = parts.next().unwrap_or("/");
    let path = target.split_once('?').map_or(target, |(path, _)| path);

    match (method, path) {
        ("GET", "/") | ("GET", "/index.html") => {
            let html = SPEAKER_DIRECTORY_HTML.replace("__SESSION_TOKEN__", session_token);
            write_http_response(
                &mut stream,
                200,
                "OK",
                "text/html; charset=utf-8",
                &html,
            )
        }
        ("GET", "/api/state") => match native_whisperx::read_speaker_directory_state(resolved) {
            Ok(state) => write_http_response(
                &mut stream,
                200,
                "OK",
                "application/json; charset=utf-8",
                &serde_json::to_string_pretty(&state)?,
            ),
            Err(error) => write_http_response(
                &mut stream,
                500,
                "Internal Server Error",
                "text/plain; charset=utf-8",
                &format!("failed to read Speaker Directory state: {error}\n"),
            ),
        },
        ("POST", "/api/trace/rebuild") => {
            if !speaker_directory_token_authorized(&headers, session_token) {
                return write_http_response(
                    &mut stream,
                    403,
                    "Forbidden",
                    "text/plain; charset=utf-8",
                    "missing or invalid Speaker Directory session token\n",
                );
            }
            match rebuild_speaker_trace_from_web_request(resolved, &body) {
                Ok(response) => write_http_response(
                    &mut stream,
                    200,
                    "OK",
                    "application/json; charset=utf-8",
                    &serde_json::to_string_pretty(&response)?,
                ),
                Err(error) => write_http_response(
                    &mut stream,
                    400,
                    "Bad Request",
                    "text/plain; charset=utf-8",
                    &format!("{error}\n"),
                ),
            }
        }
        ("PUT", path) if path.starts_with("/api/profiles/") => {
            if !speaker_directory_token_authorized(&headers, session_token) {
                return write_http_response(
                    &mut stream,
                    403,
                    "Forbidden",
                    "text/plain; charset=utf-8",
                    "missing or invalid Speaker Directory session token\n",
                );
            }
            let profile_id = match speaker_profile_id_from_api_path(path) {
                Ok(profile_id) => profile_id,
                Err(message) => {
                    return write_http_response(
                        &mut stream,
                        400,
                        "Bad Request",
                        "text/plain; charset=utf-8",
                        &format!("{message}\n"),
                    );
                }
            };
            match serde_json::from_slice::<native_whisperx::SpeakerProfileEdit>(&body) {
                Ok(edit) => match native_whisperx::update_speaker_profile(&resolved.path, &profile_id, edit) {
                    Ok(_) => write_speaker_directory_state_response(&mut stream, resolved),
                    Err(error) => write_http_response(
                        &mut stream,
                        400,
                        "Bad Request",
                        "text/plain; charset=utf-8",
                        &format!("{error}\n"),
                    ),
                },
                Err(error) => write_http_response(
                    &mut stream,
                    400,
                    "Bad Request",
                    "text/plain; charset=utf-8",
                    &format!("malformed Speaker Library profile edit JSON: {error}\n"),
                ),
            }
        }
        ("DELETE", path) if path.starts_with("/api/profiles/") => {
            if !speaker_directory_token_authorized(&headers, session_token) {
                return write_http_response(
                    &mut stream,
                    403,
                    "Forbidden",
                    "text/plain; charset=utf-8",
                    "missing or invalid Speaker Directory session token\n",
                );
            }
            let profile_id = match speaker_profile_id_from_api_path(path) {
                Ok(profile_id) => profile_id,
                Err(message) => {
                    return write_http_response(
                        &mut stream,
                        400,
                        "Bad Request",
                        "text/plain; charset=utf-8",
                        &format!("{message}\n"),
                    );
                }
            };
            match native_whisperx::delete_speaker_profile(&resolved.path, &profile_id) {
                Ok(_) => write_speaker_directory_state_response(&mut stream, resolved),
                Err(error) => write_http_response(
                    &mut stream,
                    400,
                    "Bad Request",
                    "text/plain; charset=utf-8",
                    &format!("{error}\n"),
                ),
            }
        }
        ("POST", "/api/profiles") => {
            if !speaker_directory_token_authorized(&headers, session_token) {
                return write_http_response(
                    &mut stream,
                    403,
                    "Forbidden",
                    "text/plain; charset=utf-8",
                    "missing or invalid Speaker Directory session token\n",
                );
            }
            match native_whisperx::reject_draft_speaker_profile_creation() {
                Ok(()) => write_speaker_directory_state_response(&mut stream, resolved),
                Err(error) => write_http_response(
                    &mut stream,
                    400,
                    "Bad Request",
                    "text/plain; charset=utf-8",
                    &format!("{error}\n"),
                ),
            }
        }
        ("GET", _) => write_http_response(
            &mut stream,
            404,
            "Not Found",
            "text/plain; charset=utf-8",
            "not found\n",
        ),
        _ => write_http_response(
            &mut stream,
            405,
            "Method Not Allowed",
            "text/plain; charset=utf-8",
            "Speaker Directory UI does not support this request\n",
        ),
    }
}

fn rebuild_speaker_trace_from_web_request(
    resolved: &native_whisperx::ResolvedSpeakerDirectory,
    body: &[u8],
) -> anyhow::Result<serde_json::Value> {
    let request = if body.is_empty() {
        serde_json::Value::Object(serde_json::Map::new())
    } else {
        serde_json::from_slice::<serde_json::Value>(body)
            .context("malformed Speaker Trace rescan JSON")?
    };
    let request = request.as_object().ok_or_else(|| {
        anyhow::anyhow!("Speaker Trace rescan request must be a JSON object")
    })?;
    if let Some(field) = request.keys().find(|field| field.as_str() != "scanRoot") {
        anyhow::bail!("unknown field `{field}`");
    }
    let requested_scan_root = match request.get("scanRoot") {
        Some(serde_json::Value::String(path)) if !path.trim().is_empty() => Some(PathBuf::from(path)),
        Some(serde_json::Value::String(_)) | Some(serde_json::Value::Null) | None => None,
        Some(_) => anyhow::bail!("Speaker Trace scanRoot must be a string"),
    };
    let current_dir = std::env::current_dir()?;
    let scan_root = match requested_scan_root {
        Some(path) => crate::cmd::resolve_cli_path_with_root(path, &current_dir),
        None if resolved.scope == native_whisperx::ResolvedSpeakerDirectoryScope::Global => {
            anyhow::bail!(
                "global Speaker Directory trace rescans require scanRoot to avoid indexing unrelated files"
            );
        }
        None => current_dir,
    };

    let report = native_whisperx::rebuild_speaker_trace(&resolved.path, &scan_root)?;
    let state = native_whisperx::read_speaker_directory_state(resolved)?;
    Ok(serde_json::json!({
        "state": state,
        "report": report
    }))
}

fn write_speaker_directory_state_response(
    stream: &mut TcpStream,
    resolved: &native_whisperx::ResolvedSpeakerDirectory,
) -> anyhow::Result<()> {
    match native_whisperx::read_speaker_directory_state(resolved) {
        Ok(state) => write_http_response(
            stream,
            200,
            "OK",
            "application/json; charset=utf-8",
            &serde_json::to_string_pretty(&state)?,
        ),
        Err(error) => write_http_response(
            stream,
            500,
            "Internal Server Error",
            "text/plain; charset=utf-8",
            &format!("failed to read Speaker Directory state: {error}\n"),
        ),
    }
}

fn speaker_directory_token_authorized(headers: &[(String, String)], session_token: &str) -> bool {
    headers
        .iter()
        .any(|(name, value)| name.eq_ignore_ascii_case("x-native-whisperx-session-token") && value == session_token)
}

fn speaker_profile_id_from_api_path(path: &str) -> Result<String, String> {
    let raw = path
        .strip_prefix("/api/profiles/")
        .ok_or_else(|| "Speaker Library profile path is malformed".to_string())?;
    if raw.is_empty() {
        return Err("Speaker Library profile id must not be empty".to_string());
    }
    percent_decode_path_segment(raw)
}

fn percent_decode_path_segment(value: &str) -> Result<String, String> {
    let mut bytes = Vec::with_capacity(value.len());
    let value = value.as_bytes();
    let mut index = 0;
    while index < value.len() {
        match value[index] {
            b'%' => {
                let Some(hex) = value.get(index + 1..index + 3) else {
                    return Err(
                        "Speaker Library profile id contains invalid percent encoding".to_string(),
                    );
                };
                let hex = std::str::from_utf8(hex).map_err(|_| {
                    "Speaker Library profile id contains invalid percent encoding".to_string()
                })?;
                let byte = u8::from_str_radix(hex, 16).map_err(|_| {
                    "Speaker Library profile id contains invalid percent encoding".to_string()
                })?;
                bytes.push(byte);
                index += 3;
            }
            byte => {
                bytes.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8(bytes).map_err(|_| "Speaker Library profile id must be valid UTF-8".to_string())
}

fn write_http_response(
    stream: &mut TcpStream,
    status_code: u16,
    reason: &str,
    content_type: &str,
    body: &str,
) -> anyhow::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status_code} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\n\r\n",
        body.len()
    )?;
    stream.write_all(body.as_bytes())?;
    Ok(())
}

//! HTTP handlers and embedded assets for the Speaker Directory management UI.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;

use anyhow::Context;

const SPEAKER_DIRECTORY_REACT_HTML: &str =
    include_str!("../../speaker-directory-ui/dist/index.html");
const SPEAKER_DIRECTORY_REACT_ASSETS: &[SpeakerDirectoryAsset] = &[
    SpeakerDirectoryAsset {
        path: "/assets/index-anWxHn8k.js",
        content_type: "text/javascript; charset=utf-8",
        body: include_bytes!("../../speaker-directory-ui/dist/assets/index-anWxHn8k.js"),
    },
    SpeakerDirectoryAsset {
        path: "/assets/index-DJHzQzhI.css",
        content_type: "text/css; charset=utf-8",
        body: include_bytes!("../../speaker-directory-ui/dist/assets/index-DJHzQzhI.css"),
    },
];

struct SpeakerDirectoryAsset {
    path: &'static str,
    content_type: &'static str,
    body: &'static [u8],
}

pub(crate) fn handle_speaker_directory_request(
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
            let html = speaker_directory_react_html(session_token)?;
            write_http_response(&mut stream, 200, "OK", "text/html; charset=utf-8", &html)
        }
        ("GET", path) if path.starts_with("/assets/") => {
            match SPEAKER_DIRECTORY_REACT_ASSETS
                .iter()
                .find(|asset| asset.path == path)
            {
                Some(asset) => write_http_response_bytes(
                    &mut stream,
                    200,
                    "OK",
                    asset.content_type,
                    asset.body,
                ),
                None => write_http_response(
                    &mut stream,
                    404,
                    "Not Found",
                    "text/plain; charset=utf-8",
                    "not found\n",
                ),
            }
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
                Ok(edit) => {
                    match native_whisperx::update_speaker_profile(&resolved.path, &profile_id, edit)
                    {
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

fn speaker_directory_react_html(session_token: &str) -> anyhow::Result<String> {
    let token = serde_json::to_string(session_token)?;
    let injection = format!("    <script>window.nativeWhisperxSessionToken = {token};</script>\n");
    if let Some(index) = SPEAKER_DIRECTORY_REACT_HTML.find("    <script ") {
        let mut html = String::with_capacity(SPEAKER_DIRECTORY_REACT_HTML.len() + injection.len());
        html.push_str(&SPEAKER_DIRECTORY_REACT_HTML[..index]);
        html.push_str(&injection);
        html.push_str(&SPEAKER_DIRECTORY_REACT_HTML[index..]);
        Ok(html)
    } else {
        let mut html = SPEAKER_DIRECTORY_REACT_HTML.to_string();
        html.push_str(&injection);
        Ok(html)
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
    let request = request
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("Speaker Trace rescan request must be a JSON object"))?;
    if let Some(field) = request.keys().find(|field| field.as_str() != "scanRoot") {
        anyhow::bail!("unknown field `{field}`");
    }
    let requested_scan_root = match request.get("scanRoot") {
        Some(serde_json::Value::String(path)) if !path.trim().is_empty() => {
            Some(PathBuf::from(path))
        }
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
    headers.iter().any(|(name, value)| {
        name.eq_ignore_ascii_case("x-native-whisperx-session-token") && value == session_token
    })
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
    String::from_utf8(bytes)
        .map_err(|_| "Speaker Library profile id must be valid UTF-8".to_string())
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

fn write_http_response_bytes(
    stream: &mut TcpStream,
    status_code: u16,
    reason: &str,
    content_type: &str,
    body: &[u8],
) -> anyhow::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status_code} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\n\r\n",
        body.len()
    )?;
    stream.write_all(body)?;
    Ok(())
}

//! HTTP handlers and embedded assets for the Speaker Directory management UI.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;

use anyhow::Context;

pub(crate) const SPEAKER_DIRECTORY_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Speaker Directory</title>
  <style>
    :root {
      color-scheme: light;
      font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
      background: #f5f6f8;
      color: #171b22;
    }
    body {
      margin: 0;
    }
    header {
      background: #ffffff;
      border-bottom: 1px solid #d9dde5;
      padding: 24px 32px 18px;
    }
    main {
      max-width: 1120px;
      margin: 0 auto;
      padding: 24px 24px 40px;
    }
    h1, h2, h3 {
      margin: 0;
      letter-spacing: 0;
    }
    h1 {
      font-size: 28px;
      line-height: 1.2;
    }
    h2 {
      font-size: 18px;
      margin-bottom: 12px;
    }
    h3 {
      font-size: 15px;
      margin-bottom: 8px;
    }
    .summary {
      display: grid;
      grid-template-columns: repeat(3, minmax(0, 1fr));
      gap: 12px;
      margin-top: 18px;
    }
    .metric, .item {
      background: #ffffff;
      border: 1px solid #d9dde5;
      border-radius: 8px;
    }
    .metric {
      padding: 12px 14px;
      min-width: 0;
    }
    .label {
      color: #5b6472;
      font-size: 12px;
      text-transform: uppercase;
    }
    .value {
      font-size: 15px;
      margin-top: 5px;
      overflow-wrap: anywhere;
    }
    .grid {
      display: grid;
      grid-template-columns: minmax(280px, 0.9fr) minmax(320px, 1.1fr);
      gap: 16px;
      align-items: start;
    }
    .panel {
      margin-bottom: 22px;
    }
    .list {
      display: grid;
      gap: 10px;
    }
    .item {
      padding: 12px;
    }
    .row {
      display: flex;
      justify-content: space-between;
      gap: 12px;
      align-items: baseline;
    }
    .mono {
      font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace;
      font-size: 13px;
      overflow-wrap: anywhere;
    }
    .muted {
      color: #5b6472;
    }
    .status {
      display: inline-flex;
      align-items: center;
      border-radius: 999px;
      padding: 3px 9px;
      background: #edf2f7;
      font-size: 12px;
      text-transform: capitalize;
    }
    .valid {
      background: #e7f6ed;
      color: #17663a;
    }
    .invalid {
      background: #ffe9e6;
      color: #9a2516;
    }
    .missing {
      background: #fff4db;
      color: #77510e;
    }
    .metadata, .span {
      margin-top: 8px;
      padding-top: 8px;
      border-top: 1px solid #eceff3;
    }
    .profile-form {
      display: grid;
      gap: 8px;
      margin-top: 10px;
    }
    .trace-controls {
      display: grid;
      gap: 8px;
      margin-bottom: 12px;
    }
    .report {
      display: grid;
      gap: 5px;
      margin-top: 8px;
      font-size: 13px;
    }
    input, textarea {
      width: 100%;
      box-sizing: border-box;
      border: 1px solid #cbd2dc;
      border-radius: 6px;
      padding: 8px 10px;
      font: inherit;
      background: #ffffff;
      color: #171b22;
    }
    textarea {
      min-height: 78px;
      resize: vertical;
      font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace;
      font-size: 13px;
    }
    .actions {
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
    }
    button {
      border: 1px solid #253043;
      border-radius: 6px;
      background: #253043;
      color: #ffffff;
      padding: 7px 10px;
      font: inherit;
      cursor: pointer;
    }
    button.secondary {
      background: #ffffff;
      color: #9a2516;
      border-color: #e0b4ae;
    }
    .error {
      color: #9a2516;
    }
    @media (max-width: 760px) {
      header {
        padding: 20px;
      }
      main {
        padding: 18px;
      }
      .summary, .grid {
        grid-template-columns: 1fr;
      }
      .row {
        display: block;
      }
    }
  </style>
</head>
<body>
  <header>
    <h1>Speaker Directory</h1>
    <div class="summary">
      <div class="metric"><div class="label">Scope</div><div id="scope" class="value">...</div></div>
      <div class="metric"><div class="label">Library</div><div id="library-status" class="value">...</div></div>
      <div class="metric"><div class="label">Trace</div><div id="trace-status" class="value">...</div></div>
    </div>
  </header>
  <main>
    <section class="panel">
      <h2>Backing Path</h2>
      <div id="path" class="mono"></div>
    </section>
    <div class="grid">
      <section class="panel">
        <h2>Profiles</h2>
        <div id="profiles" class="list"></div>
      </section>
      <section class="panel">
        <h2>Speaker Trace</h2>
        <form id="trace-rescan-form" class="trace-controls">
          <input id="trace-scan-root" type="text" placeholder="Scan root" aria-label="Speaker Trace scan root">
          <div class="actions">
            <button type="submit">Rescan</button>
          </div>
          <div id="trace-rescan-error" class="error"></div>
          <div id="trace-rescan-report" class="report muted"></div>
        </form>
        <div id="trace" class="list"></div>
      </section>
    </div>
  </main>
  <script>
    const sessionToken = "__SESSION_TOKEN__";
    const text = (value) => value === undefined || value === null || value === "" ? "none" : String(value);
    const el = (tag, className, value) => {
      const node = document.createElement(tag);
      if (className) node.className = className;
      if (value !== undefined) node.textContent = value;
      return node;
    };
    const status = (value) => {
      const node = el("span", `status ${value}`, value);
      return node;
    };
    const metadataText = (metadata) => {
      const entries = Object.entries(metadata || {});
      if (!entries.length) return "metadata: none";
      return entries.map(([key, value]) => `${key}: ${value}`).join(" | ");
    };
    const metadataLines = (metadata) => Object.entries(metadata || {})
      .map(([key, value]) => `${key}=${value}`)
      .join("\n");
    const parseMetadata = (value) => {
      const metadata = {};
      for (const rawLine of value.split(/\r?\n/)) {
        const line = rawLine.trim();
        if (!line) continue;
        const index = line.indexOf("=");
        if (index <= 0) throw new Error("metadata lines must use key=value");
        metadata[line.slice(0, index).trim()] = line.slice(index + 1).trim();
      }
      return metadata;
    };
    const renderRescanReport = (report) => {
      const node = document.getElementById("trace-rescan-report");
      node.replaceChildren();
      if (!report) return;
      const stats = report.stats || {};
      node.append(el("div", "", `Scanned files: ${stats.scannedFiles ?? 0}`));
      node.append(el("div", "", `Accepted entries: ${stats.acceptedEntries ?? 0}`));
      node.append(el("div", "", `Ignored non-JSON files: ${stats.ignoredNonJsonFiles ?? 0}`));
      node.append(el("div", "", `Malformed JSON errors: ${stats.malformedJsonErrors ?? 0}`));
    };
    const api = async (path, options = {}) => {
      const response = await fetch(path, {
        ...options,
        headers: {
          "Content-Type": "application/json",
          "X-Native-Whisperx-Session-Token": sessionToken,
          ...(options.headers || {})
        }
      });
      if (!response.ok) {
        throw new Error((await response.text()).trim() || response.statusText);
      }
      return response.json();
    };
    const renderProfile = (profiles, profile) => {
      const item = el("article", "item");
      const row = el("div", "row");
      row.append(el("strong", "", text(profile.label)));
      row.append(el("span", "mono muted", text(profile.id)));
      item.append(row);
      item.append(el("div", "metadata muted", metadataText(profile.metadata)));

      const form = el("form", "profile-form");
      const label = document.createElement("input");
      label.value = profile.label || "";
      label.setAttribute("aria-label", "Speaker label");
      const metadata = document.createElement("textarea");
      metadata.value = metadataLines(profile.metadata);
      metadata.setAttribute("aria-label", "Speaker metadata");
      const actions = el("div", "actions");
      const save = document.createElement("button");
      save.type = "submit";
      save.textContent = "Save";
      const remove = document.createElement("button");
      remove.type = "button";
      remove.className = "secondary";
      remove.textContent = "Delete";
      const error = el("div", "error");
      actions.append(save, remove);
      form.append(label, metadata, actions, error);
      form.addEventListener("submit", async (event) => {
        event.preventDefault();
        error.textContent = "";
        try {
          await api(`/api/profiles/${encodeURIComponent(profile.id)}`, {
            method: "PUT",
            body: JSON.stringify({
              id: profile.id,
              label: label.value,
              metadata: parseMetadata(metadata.value)
            })
          });
          await refresh();
        } catch (err) {
          error.textContent = err.message;
        }
      });
      remove.addEventListener("click", async () => {
        error.textContent = "";
        try {
          await api(`/api/profiles/${encodeURIComponent(profile.id)}`, { method: "DELETE" });
          await refresh();
        } catch (err) {
          error.textContent = err.message;
        }
      });
      item.append(form);
      profiles.append(item);
    };
    const render = (state) => {
      document.getElementById("scope").textContent = state.scope;
      document.getElementById("path").textContent = state.path;
      document.getElementById("library-status").replaceChildren(status(state.library.status));
      document.getElementById("trace-status").replaceChildren(status(state.trace.status));

      const profiles = document.getElementById("profiles");
      profiles.replaceChildren();
      if (!state.profiles.length) {
        profiles.append(el("div", "muted", "No enrolled profiles"));
      }
      for (const profile of state.profiles) {
        renderProfile(profiles, profile);
      }

      const trace = document.getElementById("trace");
      trace.replaceChildren();
      if (!state.trace.speakers.length) {
        trace.append(el("div", "muted", state.trace.message || "No trace groups"));
      }
      for (const speaker of state.trace.speakers) {
        const item = el("article", "item");
        item.append(el("h3", "", speaker.label || speaker.anonymousLabel || speaker.profileId));
        item.append(el("div", "mono muted", speaker.profileId || speaker.anonymousLabel));
        for (const file of speaker.files) {
          const fileNode = el("div", "metadata");
          fileNode.append(el("div", "mono", file.sourceFile));
          fileNode.append(el("div", "muted", `${file.segmentCount} segments | ${file.totalDuration.toFixed(3)}s`));
          for (const span of file.spans) {
            const spanNode = el("div", "span muted");
            spanNode.textContent = `${text(span.startSeconds)}-${text(span.endSeconds)}: ${span.snippet}`;
            fileNode.append(spanNode);
          }
          item.append(fileNode);
        }
        trace.append(item);
      }
      for (const traceError of state.trace.errors || []) {
        const item = el("article", "item error");
        item.append(el("strong", "", "Malformed JSON"));
        item.append(el("div", "mono", traceError.sourceFile));
        item.append(el("div", "", traceError.message));
        trace.append(item);
      }
    };
    const refresh = async () => {
      const response = await fetch("/api/state");
      render(await response.json());
    };
    document.getElementById("trace-rescan-form").addEventListener("submit", async (event) => {
      event.preventDefault();
      const error = document.getElementById("trace-rescan-error");
      const scanRoot = document.getElementById("trace-scan-root").value.trim();
      error.textContent = "";
      try {
        const response = await api("/api/trace/rebuild", {
          method: "POST",
          body: JSON.stringify(scanRoot ? { scanRoot } : {})
        });
        render(response.state);
        renderRescanReport(response.report);
      } catch (err) {
        error.textContent = err.message;
      }
    });
    refresh().catch((error) => {
      document.getElementById("trace").textContent = error.message;
    });
  </script>
</body>
</html>
"#;

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
            let html = SPEAKER_DIRECTORY_HTML.replace("__SESSION_TOKEN__", session_token);
            write_http_response(&mut stream, 200, "OK", "text/html; charset=utf-8", &html)
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

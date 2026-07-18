use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command as ProcessCommand, Stdio};

#[test]
fn default_cli_packaging_includes_release_runtime_paths() {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let output = ProcessCommand::new(cargo)
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--locked",
            "--offline",
            "--manifest-path",
        ])
        .arg(manifest)
        .output()
        .expect("cargo metadata should run");
    assert!(
        output.status.success(),
        "cargo metadata should describe default CLI packaging: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("cargo metadata json");
    let default_features = metadata["packages"]
        .as_array()
        .and_then(|packages| {
            packages
                .iter()
                .find(|package| package["name"] == "native-whisperx-cli")
        })
        .and_then(|package| package["features"]["default"].as_array())
        .expect("native-whisperx-cli default features");
    let includes = |feature: &str| {
        default_features
            .iter()
            .any(|enabled| enabled.as_str() == Some(feature))
    };

    for feature in ["pyannote-vad", "pyannote-diarization", "translation"] {
        assert!(
            includes(feature),
            "default native-whisperx-cli packaging should include {feature} code paths"
        );
    }

    for feature in ["cuda", "whisperx-compat"] {
        assert!(
            !includes(feature),
            "default native-whisperx-cli packaging should not force {feature}"
        );
    }
}

#[test]
fn imports_whisperx_fixture_to_stdout() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/whisperx-parity-sample.json");
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("import-whisperx")
        .arg(fixture)
        .assert()
        .success()
        .stdout(predicate::str::contains("hello world second speaker"));
}

#[test]
fn speakers_path_local_scope_prints_project_speaker_directory() {
    let temp = tempfile::tempdir().expect("tempdir");
    let expected = temp.path().join(".native-whisperx/speakers");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args(["speakers", "path", "--scope", "local"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            expected.to_string_lossy().as_ref().to_string(),
        ));
}

#[test]
fn speakers_path_auto_prefers_existing_local_speaker_directory() {
    let temp = tempfile::tempdir().expect("tempdir");
    let local = temp.path().join(".native-whisperx/speakers");
    fs::create_dir_all(&local).expect("local speaker directory");
    let global_root = temp.path().join("global-data");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .env("XDG_DATA_HOME", &global_root)
        .args(["speakers", "path"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            local.to_string_lossy().as_ref().to_string(),
        ));
}

#[test]
fn speakers_path_explicit_directory_overrides_local_directory() {
    let temp = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(temp.path().join(".native-whisperx/speakers"))
        .expect("local speaker directory");
    let expected = temp.path().join("controlled-speakers");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args([
            "speakers",
            "path",
            "--speaker-directory",
            "controlled-speakers",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            expected.to_string_lossy().as_ref().to_string(),
        ));
}

#[test]
fn speakers_path_accepts_speaker_store_alias() {
    let temp = tempfile::tempdir().expect("tempdir");
    let expected = temp.path().join("controlled-speakers");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args(["speakers", "path", "--speaker-store", "controlled-speakers"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            expected.to_string_lossy().as_ref().to_string(),
        ));
}

#[test]
fn speakers_path_rejects_conflicting_directory_and_store_aliases() {
    let temp = tempfile::tempdir().expect("tempdir");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args([
            "speakers",
            "path",
            "--speaker-directory",
            "one",
            "--speaker-store",
            "two",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--speaker-directory and --speaker-store must point to the same path",
        ));
}

#[cfg(target_os = "linux")]
#[test]
fn speakers_path_global_scope_uses_xdg_data_home_convention() {
    let temp = tempfile::tempdir().expect("tempdir");
    let global_root = temp.path().join("global-data");
    let expected = global_root.join("native-whisperx/speakers");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .env("XDG_DATA_HOME", &global_root)
        .args(["speakers", "path", "--scope", "global"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            expected.to_string_lossy().as_ref().to_string(),
        ));
}

#[test]
fn speakers_validate_accepts_valid_speaker_library() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join("speakers");
    fs::create_dir_all(&directory).expect("speaker directory");
    fs::write(directory.join("library.json"), valid_speaker_library_json()).expect("library");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["speakers", "validate", "--speaker-directory"])
        .arg(&directory)
        .assert()
        .success()
        .stdout(predicate::str::contains("Speaker Library valid"))
        .stdout(predicate::str::contains("profiles: 1"));
}

#[test]
fn speakers_validate_reports_specific_error_for_incompatible_library() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join("speakers");
    fs::create_dir_all(&directory).expect("speaker directory");
    fs::write(
        directory.join("library.json"),
        valid_speaker_library_json().replace("\"values\": [1.0, 0.0]", "\"values\": [2.0, 0.0]"),
    )
    .expect("library");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["speakers", "validate", "--speaker-directory"])
        .arg(&directory)
        .assert()
        .failure()
        .stderr(predicate::str::contains("malformed or incompatible"))
        .stderr(predicate::str::contains("normalized"));
}

#[test]
fn speakers_list_excludes_drafts_by_default() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join("speakers");
    fs::create_dir_all(&directory).expect("speaker directory");
    fs::write(
        directory.join("library.json"),
        draft_and_confirmed_speaker_library_json(),
    )
    .expect("library");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    let output = command
        .args(["speakers", "list", "--speaker-directory"])
        .arg(&directory)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let profiles: serde_json::Value = serde_json::from_slice(&output).expect("profiles json");

    assert_eq!(profiles.as_array().expect("profiles").len(), 1);
    assert_eq!(profiles[0]["speakerId"], "speaker-a");
    assert_eq!(profiles[0]["status"], "confirmed");
}

#[test]
fn speakers_list_include_drafts_outputs_confirmed_and_draft_profiles() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join("speakers");
    fs::create_dir_all(&directory).expect("speaker directory");
    fs::write(
        directory.join("library.json"),
        draft_and_confirmed_speaker_library_json(),
    )
    .expect("library");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    let output = command
        .args(["speakers", "list", "--speaker-store"])
        .arg(&directory)
        .arg("--include-drafts")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let profiles: serde_json::Value = serde_json::from_slice(&output).expect("profiles json");

    let profiles = profiles.as_array().expect("profiles");
    assert_eq!(profiles.len(), 2);
    assert!(profiles
        .iter()
        .any(|profile| profile["speakerId"] == "draft-speaker-b" && profile["status"] == "draft"));
}

#[test]
fn speakers_list_missing_library_outputs_empty_json_array() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join("speakers");
    fs::create_dir_all(&directory).expect("speaker directory");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["speakers", "list", "--speaker-directory"])
        .arg(&directory)
        .assert()
        .success()
        .stdout(predicate::str::contains("[]"));
}

#[test]
fn speakers_rebuild_trace_uses_local_project_scan_root_by_default() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join(".native-whisperx/speakers");
    fs::create_dir_all(&directory).expect("speaker directory");
    fs::write(directory.join("library.json"), valid_speaker_library_json()).expect("library");
    fs::write(
        temp.path().join("transcript.json"),
        r#"{"segments": [{"id": 0, "start": 0.0, "end": 1.25, "text": "hello", "speaker": "speaker-a"}]}"#,
    )
    .expect("transcript");
    fs::write(temp.path().join("transcript.srt"), "ignored").expect("srt");
    fs::write(temp.path().join("broken.json"), "{").expect("broken json");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args(["speakers", "rebuild-trace", "--scope", "local"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Speaker Trace rebuilt"))
        .stdout(predicate::str::contains("speakers: 1"))
        .stdout(predicate::str::contains("errors: 1"))
        .stderr(predicate::str::contains("broken.json"))
        .stderr(predicate::str::contains("malformed transcript JSON"));

    let trace = fs::read_to_string(directory.join("speaker-trace.json")).expect("trace");
    assert!(trace.contains("\"profileId\": \"speaker-a\""));
    assert!(trace.contains("\"segmentCount\": 1"));
    assert!(trace.contains("\"totalDuration\": 1.25"));
    assert!(trace.contains("\"snippet\": \"hello\""));
    assert!(!trace.contains("transcript.srt"));
}

#[cfg(target_os = "linux")]
#[test]
fn speakers_rebuild_trace_global_scope_requires_scan_root() {
    let temp = tempfile::tempdir().expect("tempdir");
    let global_root = temp.path().join("global-data");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .env("XDG_DATA_HOME", &global_root)
        .args(["speakers", "rebuild-trace", "--scope", "global"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("require --scan-root"));
}

#[test]
fn speakers_open_print_url_prints_bare_loopback_url() {
    let temp = tempfile::tempdir().expect("tempdir");
    let mut child = ProcessCommand::new(env!("CARGO_BIN_EXE_native-whisperx"))
        .current_dir(temp.path())
        .args(["speakers", "open", "--scope", "local", "--print-url"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn speakers open");
    let stdout = child.stdout.take().expect("stdout");
    let mut child = ChildGuard(child);
    let mut stdout = BufReader::new(stdout);
    let mut first_line = String::new();
    stdout.read_line(&mut first_line).expect("url line");

    assert!(
        first_line.starts_with("http://127.0.0.1:"),
        "expected bare loopback URL, got {first_line}"
    );

    child.stop();
}

#[test]
fn speakers_open_no_browser_serves_read_only_loopback_state() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join(".native-whisperx/speakers");
    fs::create_dir_all(&directory).expect("speaker directory");
    fs::write(directory.join("library.json"), valid_speaker_library_json()).expect("library");
    fs::write(
        directory.join("speaker-trace.json"),
        r#"{
          "version": 1,
          "scanRoot": "/tmp/native-whisperx-test-output",
          "speakers": [{
            "kind": "enrolled",
            "profileId": "speaker-a",
            "label": "Speaker A",
            "files": [{
              "sourceFile": "/tmp/native-whisperx-test-output/transcript.json",
              "segmentCount": 2,
              "totalDuration": 2.5,
              "spans": [
                {"startSeconds": 0.0, "endSeconds": 1.0, "snippet": "hello"},
                {"startSeconds": 1.0, "endSeconds": 2.5, "snippet": "world"}
              ]
            }]
          }],
          "errors": []
        }"#,
    )
    .expect("trace");

    let mut child = ProcessCommand::new(env!("CARGO_BIN_EXE_native-whisperx"))
        .current_dir(temp.path())
        .args(["speakers", "open", "--scope", "local", "--no-browser"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn speakers open");
    let stdout = child.stdout.take().expect("stdout");
    let mut child = ChildGuard(child);
    let mut stdout = BufReader::new(stdout);
    let mut first_line = String::new();
    stdout.read_line(&mut first_line).expect("url line");
    let url = extract_url(&first_line);

    assert!(
        url.starts_with("http://127.0.0.1:"),
        "expected loopback URL, got {url}"
    );

    let base = url.trim_end_matches('/');
    let (status, headers, html) = http_response("GET", base, &[], None);
    assert_eq!(status, 200);
    assert!(headers.contains("Content-Type: text/html; charset=utf-8"));
    assert!(headers.contains("X-Content-Type-Options: nosniff"));
    assert!(
        html.contains("window.nativeWhisperxSessionToken"),
        "root page should expose the Speaker Directory session token"
    );
    assert!(
        html.find("window.nativeWhisperxSessionToken") < html.find(r#"type="module""#),
        "session token must be injected before the React bundle starts"
    );
    assert!(
        html.contains(r#"<div id="root"></div>"#),
        "root page should serve the React application shell"
    );
    assert!(
        !html.contains("const sessionToken"),
        "root page should not serve the previous embedded static implementation"
    );
    let script_path = html_asset_path(&html, r#"src=""#);
    let stylesheet_path = html_asset_path(&html, r#"href=""#);

    let (status, headers, script) =
        http_response("GET", &format!("{base}{script_path}"), &[], None);
    assert_eq!(status, 200);
    assert!(headers.contains("Content-Type: text/javascript; charset=utf-8"));
    assert!(headers.contains("X-Content-Type-Options: nosniff"));
    assert!(script.contains("nativeWhisperxSessionToken"));

    let (status, headers, stylesheet) =
        http_response("GET", &format!("{base}{stylesheet_path}"), &[], None);
    assert_eq!(status, 200);
    assert!(headers.contains("Content-Type: text/css; charset=utf-8"));
    assert!(headers.contains("X-Content-Type-Options: nosniff"));
    assert!(stylesheet.contains(":root"));

    let (status, body) = http_request("GET", &format!("{}/api/state", url.trim_end_matches('/')));
    assert_eq!(status, 200);
    let state: serde_json::Value = serde_json::from_str(&body).expect("state json");
    assert_eq!(state["scope"], "local");
    assert_eq!(state["path"], directory.to_string_lossy().as_ref());
    assert_eq!(state["library"]["status"], "valid");
    assert_eq!(state["library"]["profileCount"], 1);
    assert_eq!(state["profiles"][0]["id"], "speaker-a");
    assert_eq!(state["profiles"][0]["label"], "Speaker A");
    assert_eq!(state["profiles"][0]["metadata"]["note"], "fixture");
    assert_eq!(state["trace"]["status"], "valid");
    assert_eq!(state["trace"]["speakers"][0]["profileId"], "speaker-a");
    assert_eq!(
        state["trace"]["speakers"][0]["files"][0]["sourceFile"],
        "/tmp/native-whisperx-test-output/transcript.json"
    );
    assert_eq!(state["trace"]["speakers"][0]["files"][0]["segmentCount"], 2);
    assert_eq!(
        state["trace"]["speakers"][0]["files"][0]["totalDuration"],
        2.5
    );
    assert_eq!(
        state["trace"]["speakers"][0]["files"][0]["spans"][0]["snippet"],
        "hello"
    );

    let (status, body) = http_request("POST", &format!("{}/api/state", url.trim_end_matches('/')));
    assert_eq!(status, 405);
    assert!(body.contains("does not support this request"));

    child.stop();
}

#[test]
fn speakers_open_no_browser_rescans_trace_with_session_token() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join(".native-whisperx/speakers");
    fs::create_dir_all(&directory).expect("speaker directory");
    fs::write(directory.join("library.json"), valid_speaker_library_json()).expect("library");
    fs::write(
        temp.path().join("transcript.json"),
        r#"{"segments": [{"id": 0, "start": 0.0, "end": 1.25, "text": "hello", "speaker": "speaker-a"}]}"#,
    )
    .expect("transcript");
    fs::write(temp.path().join("transcript.srt"), "ignored").expect("srt");

    let mut child = ProcessCommand::new(env!("CARGO_BIN_EXE_native-whisperx"))
        .current_dir(temp.path())
        .args(["speakers", "open", "--scope", "local", "--no-browser"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn speakers open");
    let stdout = child.stdout.take().expect("stdout");
    let mut child = ChildGuard(child);
    let mut stdout = BufReader::new(stdout);
    let mut first_line = String::new();
    stdout.read_line(&mut first_line).expect("url line");
    let url = extract_url(&first_line);
    let base = url.trim_end_matches('/');
    let (_, html) = http_request("GET", base);
    let token = extract_session_token(&html);

    let (status, body) = http_request_with_body(
        "POST",
        &format!("{base}/api/trace/rebuild"),
        &[],
        Some("{}"),
    );
    assert_eq!(status, 403);
    assert!(body.contains("missing or invalid"));
    assert!(!directory.join("speaker-trace.json").exists());

    let (status, body) = http_request_with_body(
        "POST",
        &format!("{base}/api/trace/rebuild"),
        &[("X-Native-Whisperx-Session-Token", &token)],
        Some("{}"),
    );
    assert_eq!(status, 200, "{body}");
    let response: serde_json::Value = serde_json::from_str(&body).expect("rescan json");
    assert_eq!(response["report"]["stats"]["scannedFiles"], 3);
    assert_eq!(response["report"]["stats"]["acceptedEntries"], 1);
    assert_eq!(response["report"]["stats"]["ignoredNonJsonFiles"], 1);
    assert_eq!(response["report"]["stats"]["malformedJsonErrors"], 0);
    assert_eq!(response["state"]["trace"]["status"], "valid");
    assert_eq!(
        response["state"]["trace"]["speakers"][0]["profileId"],
        "speaker-a"
    );
    assert_eq!(
        response["state"]["trace"]["speakers"][0]["files"][0]["spans"][0]["snippet"],
        "hello"
    );

    let trace = fs::read_to_string(directory.join("speaker-trace.json")).expect("trace");
    assert!(trace.contains("\"profileId\": \"speaker-a\""));
    assert!(!trace.contains("transcript.srt"));

    child.stop();
}

#[test]
fn speakers_open_no_browser_rescan_reports_malformed_json_errors() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join(".native-whisperx/speakers");
    fs::create_dir_all(&directory).expect("speaker directory");
    fs::write(directory.join("library.json"), valid_speaker_library_json()).expect("library");
    fs::write(
        temp.path().join("valid.json"),
        r#"{"segments": [{"id": 0, "start": 0.0, "end": 1.0, "text": "ok", "speaker": "speaker-a"}]}"#,
    )
    .expect("valid json");
    fs::write(temp.path().join("broken.json"), "{").expect("broken json");
    fs::write(temp.path().join("notes.txt"), "ignored").expect("text");

    let mut child = ProcessCommand::new(env!("CARGO_BIN_EXE_native-whisperx"))
        .current_dir(temp.path())
        .args(["speakers", "open", "--scope", "local", "--no-browser"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn speakers open");
    let stdout = child.stdout.take().expect("stdout");
    let mut child = ChildGuard(child);
    let mut stdout = BufReader::new(stdout);
    let mut first_line = String::new();
    stdout.read_line(&mut first_line).expect("url line");
    let url = extract_url(&first_line);
    let base = url.trim_end_matches('/');
    let (_, html) = http_request("GET", base);
    let token = extract_session_token(&html);

    let (status, body) = http_request_with_body(
        "POST",
        &format!("{base}/api/trace/rebuild"),
        &[("X-Native-Whisperx-Session-Token", &token)],
        Some("{}"),
    );
    assert_eq!(status, 200, "{body}");
    let response: serde_json::Value = serde_json::from_str(&body).expect("rescan json");
    assert_eq!(response["report"]["stats"]["scannedFiles"], 4);
    assert_eq!(response["report"]["stats"]["acceptedEntries"], 1);
    assert_eq!(response["report"]["stats"]["ignoredNonJsonFiles"], 1);
    assert_eq!(response["report"]["stats"]["malformedJsonErrors"], 1);
    assert_eq!(
        response["state"]["trace"]["errors"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(response["state"]["trace"]["errors"][0]["sourceFile"]
        .as_str()
        .unwrap()
        .ends_with("broken.json"));
    assert!(response["state"]["trace"]["errors"][0]["message"]
        .as_str()
        .unwrap()
        .contains("malformed transcript JSON"));

    let trace = fs::read_to_string(directory.join("speaker-trace.json")).expect("trace");
    assert!(trace.contains("broken.json"));
    assert!(trace.contains("speaker-a"));

    child.stop();
}

#[cfg(target_os = "linux")]
#[test]
fn speakers_open_no_browser_global_rescan_requires_scan_root() {
    let temp = tempfile::tempdir().expect("tempdir");
    let global_root = temp.path().join("global-data");

    let mut child = ProcessCommand::new(env!("CARGO_BIN_EXE_native-whisperx"))
        .current_dir(temp.path())
        .env("XDG_DATA_HOME", &global_root)
        .args(["speakers", "open", "--scope", "global", "--no-browser"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn speakers open");
    let stdout = child.stdout.take().expect("stdout");
    let mut child = ChildGuard(child);
    let mut stdout = BufReader::new(stdout);
    let mut first_line = String::new();
    stdout.read_line(&mut first_line).expect("url line");
    let url = extract_url(&first_line);
    let base = url.trim_end_matches('/');
    let (_, html) = http_request("GET", base);
    let token = extract_session_token(&html);

    let (status, body) = http_request_with_body(
        "POST",
        &format!("{base}/api/trace/rebuild"),
        &[("X-Native-Whisperx-Session-Token", &token)],
        Some("{}"),
    );
    assert_eq!(status, 400);
    assert!(body.contains("require scanRoot"));
    assert!(!global_root
        .join("native-whisperx/speakers/speaker-trace.json")
        .exists());

    child.stop();
}

#[test]
fn speakers_open_no_browser_edits_and_deletes_profiles_with_session_token() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join(".native-whisperx/speakers");
    fs::create_dir_all(&directory).expect("speaker directory");
    fs::write(
        directory.join("library.json"),
        two_profile_speaker_library_json(),
    )
    .expect("library");

    let mut child = ProcessCommand::new(env!("CARGO_BIN_EXE_native-whisperx"))
        .current_dir(temp.path())
        .args(["speakers", "open", "--scope", "local", "--no-browser"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn speakers open");
    let stdout = child.stdout.take().expect("stdout");
    let mut child = ChildGuard(child);
    let mut stdout = BufReader::new(stdout);
    let mut first_line = String::new();
    stdout.read_line(&mut first_line).expect("url line");
    let url = extract_url(&first_line);
    let base = url.trim_end_matches('/');
    let (_, html) = http_request("GET", base);
    let token = extract_session_token(&html);

    let (status, body) = http_request_with_body(
        "PUT",
        &format!("{base}/api/profiles/speaker-a"),
        &[("X-Native-Whisperx-Session-Token", &token)],
        Some(
            r#"{"id":"speaker-a","label":"Renamed Speaker","metadata":{"note":"changed","role":"host"}}"#,
        ),
    );
    assert_eq!(status, 200, "{body}");
    let state: serde_json::Value = serde_json::from_str(&body).expect("state json");
    assert_eq!(state["profiles"][0]["id"], "speaker-a");
    assert_eq!(state["profiles"][0]["label"], "Renamed Speaker");
    assert_eq!(state["profiles"][0]["metadata"]["role"], "host");
    let saved = fs::read_to_string(directory.join("library.json")).expect("saved library");
    assert!(saved.contains("\"id\": \"speaker-a\""));
    assert!(saved.contains("\"label\": \"Renamed Speaker\""));

    let (status, body) = http_request_with_body(
        "DELETE",
        &format!("{base}/api/profiles/speaker-b"),
        &[("X-Native-Whisperx-Session-Token", &token)],
        None,
    );
    assert_eq!(status, 200, "{body}");
    let state: serde_json::Value = serde_json::from_str(&body).expect("state json");
    assert_eq!(state["library"]["profileCount"], 1);
    assert_eq!(state["profiles"].as_array().expect("profiles").len(), 1);
    let saved = fs::read_to_string(directory.join("library.json")).expect("saved library");
    assert!(!saved.contains("\"id\": \"speaker-b\""));

    let (status, body) = http_request_with_body(
        "POST",
        &format!("{base}/api/profiles"),
        &[("X-Native-Whisperx-Session-Token", &token)],
        Some(r#"{"id":"draft","label":"Draft"}"#),
    );
    assert_eq!(status, 400);
    assert!(body.contains("without embeddings is not supported"));

    child.stop();
}

#[test]
fn speakers_open_no_browser_rejects_missing_or_invalid_session_token() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join(".native-whisperx/speakers");
    fs::create_dir_all(&directory).expect("speaker directory");
    fs::write(directory.join("library.json"), valid_speaker_library_json()).expect("library");

    let mut child = ProcessCommand::new(env!("CARGO_BIN_EXE_native-whisperx"))
        .current_dir(temp.path())
        .args(["speakers", "open", "--scope", "local", "--no-browser"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn speakers open");
    let stdout = child.stdout.take().expect("stdout");
    let mut child = ChildGuard(child);
    let mut stdout = BufReader::new(stdout);
    let mut first_line = String::new();
    stdout.read_line(&mut first_line).expect("url line");
    let url = extract_url(&first_line);
    let base = url.trim_end_matches('/');

    let (status, body) = http_request_with_body(
        "PUT",
        &format!("{base}/api/profiles/speaker-a"),
        &[],
        Some(r#"{"label":"Unauthorized"}"#),
    );
    assert_eq!(status, 403);
    assert!(body.contains("missing or invalid"));

    let (status, body) = http_request_with_body(
        "PUT",
        &format!("{base}/api/profiles/speaker-a"),
        &[("X-Native-Whisperx-Session-Token", "wrong-token")],
        Some(r#"{"label":"Unauthorized"}"#),
    );
    assert_eq!(status, 403);
    assert!(body.contains("missing or invalid"));
    let saved = fs::read_to_string(directory.join("library.json")).expect("saved library");
    assert!(saved.contains("\"label\": \"Speaker A\""));

    child.stop();
}

#[test]
fn speakers_open_no_browser_rejects_invalid_profile_edit_without_writing() {
    let temp = tempfile::tempdir().expect("tempdir");
    let directory = temp.path().join(".native-whisperx/speakers");
    fs::create_dir_all(&directory).expect("speaker directory");
    fs::write(directory.join("library.json"), valid_speaker_library_json()).expect("library");

    let mut child = ProcessCommand::new(env!("CARGO_BIN_EXE_native-whisperx"))
        .current_dir(temp.path())
        .args(["speakers", "open", "--scope", "local", "--no-browser"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn speakers open");
    let stdout = child.stdout.take().expect("stdout");
    let mut child = ChildGuard(child);
    let mut stdout = BufReader::new(stdout);
    let mut first_line = String::new();
    stdout.read_line(&mut first_line).expect("url line");
    let url = extract_url(&first_line);
    let base = url.trim_end_matches('/');
    let (_, html) = http_request("GET", base);
    let token = extract_session_token(&html);

    let (status, body) = http_request_with_body(
        "PUT",
        &format!("{base}/api/profiles/speaker-a"),
        &[("X-Native-Whisperx-Session-Token", &token)],
        Some(r#"{"id":"speaker-renamed","label":"Renamed"}"#),
    );
    assert_eq!(status, 400);
    assert!(body.contains("profile ids are immutable"));
    let saved = fs::read_to_string(directory.join("library.json")).expect("saved library");
    assert!(saved.contains("\"id\": \"speaker-a\""));
    assert!(saved.contains("\"label\": \"Speaker A\""));

    let (status, body) = http_request_with_body(
        "PUT",
        &format!("{base}/api/profiles/speaker-a"),
        &[("X-Native-Whisperx-Session-Token", &token)],
        Some(r#"{"profileId":"speaker-renamed","label":"Renamed"}"#),
    );
    assert_eq!(status, 400);
    assert!(body.contains("unknown field"));
    let saved = fs::read_to_string(directory.join("library.json")).expect("saved library");
    assert!(saved.contains("\"id\": \"speaker-a\""));
    assert!(saved.contains("\"label\": \"Speaker A\""));

    let (status, body) = http_request_with_body(
        "PUT",
        &format!("{base}/api/profiles/speaker-a"),
        &[("X-Native-Whisperx-Session-Token", &token)],
        Some(r#"{"label":"   "}"#),
    );
    assert_eq!(status, 400);
    assert!(body.contains("profile label must not be empty"));
    let saved = fs::read_to_string(directory.join("library.json")).expect("saved library");
    assert!(saved.contains("\"label\": \"Speaker A\""));

    child.stop();
}

#[test]
fn inspect_models_prints_request_shape() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args([
            "inspect-models",
            "--whisper-bundle",
            "models/whisper",
            "--alignment-bundle",
            "models/wav2vec2",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("candleWhisper"))
        .stdout(predicate::str::contains("\"computeType\": \"automatic\""))
        .stdout(predicate::str::contains("models/wav2vec2"));
}

#[test]
fn inspect_models_prints_native_compute_type() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args([
            "inspect-models",
            "--whisper-bundle",
            "models/whisper",
            "--compute-type",
            "fp16",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"computeType\": \"fp16\""));
}

#[test]
fn inspect_models_shows_alignment_enabled_by_default() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["inspect-models", "--whisper-bundle", "models/whisper"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"enabled\": true"))
        .stdout(predicate::str::contains("facebook/wav2vec2-base-960h"));
}

#[test]
fn inspect_models_no_align_maps_to_disabled_alignment() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args([
            "inspect-models",
            "--whisper-bundle",
            "models/whisper",
            "--no-align",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"enabled\": false"));
}

#[test]
fn inspect_models_maps_model_cache_to_native_asr() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args([
            "inspect-models",
            "--model",
            "tiny.en",
            "--model-dir",
            "models",
            "--model-cache-only",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"modelId\": \"tiny.en\""))
        .stdout(predicate::str::contains("\"modelDir\": \"models\""))
        .stdout(predicate::str::contains("\"modelCacheOnly\": true"));
}

#[test]
fn inspect_models_prints_translation_section_when_requested() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args([
            "inspect-models",
            "--translation-model",
            "Helsinki-NLP/opus-mt-de-en",
            "--model-dir",
            "models",
            "--model-cache-only",
            "--translation-target-language",
            "en",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"translation\""))
        .stdout(predicate::str::contains(
            "\"modelId\": \"Helsinki-NLP/opus-mt-de-en\"",
        ))
        .stdout(predicate::str::contains("\"targetLanguage\": \"en\""))
        .stdout(predicate::str::contains("\"modelDir\": \"models\""))
        .stdout(predicate::str::contains("\"modelCacheOnly\": true"));
}

#[test]
fn inspect_models_parses_translation_model_underscore_alias() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["inspect-models", "--translation_model", "opus-mt-de-en"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"translation\""))
        .stdout(predicate::str::contains(
            "\"modelId\": \"Helsinki-NLP/opus-mt-de-en\"",
        ));
}

#[test]
fn transcribe_help_lists_whisperx_386_contract() {
    let help = command_stdout(["transcribe", "--help"]);
    for expected in [
        "<INPUT>...",
        "--provider",
        "--model",
        "--task",
        "--language",
        "--device",
        "--device-index",
        "--batch-size",
        "--compute-type",
        "--verbose [<VERBOSE>]",
        "--log-level",
        "--print-progress",
        "--report",
        "--no-align",
        "--align-model",
        "--model-dir",
        "--model-cache-only",
        "--translation-model",
        "--translation-bundle",
        "--translation-source-language",
        "--translation-target-language",
        "--translation-max-new-tokens",
        "--interpolate-method",
        "--return-char-alignments",
        "--vad-method",
        "--vad-onset",
        "--vad-offset",
        "--chunk-size",
        "--vad-model-bundle",
        "--vad-model-file",
        "--vad-input-name",
        "--vad-output-name",
        "--diarize",
        "--diarize-model",
        "--speaker-embeddings",
        "--hf-token",
        "--min-speakers",
        "--max-speakers",
        "--speaker-assignment-policy",
        "--scope",
        "--speaker-directory",
        "--no-speaker-library",
        "--temperature",
        "--best-of",
        "--beam-size",
        "--patience",
        "--length-penalty",
        "--suppress-tokens",
        "--suppress-numerals",
        "--initial-prompt",
        "--hotwords",
        "--condition-on-previous-text",
        "--fp16",
        "--compression-ratio-threshold",
        "--logprob-threshold",
        "--no-speech-threshold",
        "--threads",
        "--max-line-width",
        "--max-line-count",
        "--highlight-words",
        "--segment-resolution",
        "--output-dir",
        "--format",
        "all",
        "tsv",
        "aud",
        "native-json",
        "sentence",
        "chunk",
    ] {
        assert!(help.contains(expected), "help should contain `{expected}`");
    }
}

#[test]
fn transcribe_help_lists_auto_vad_default_and_explicit_choices() {
    let help = command_stdout(["transcribe", "--help"]);

    assert!(
        help.contains(
            "--vad-method <VAD_METHOD>\n          [default: auto] [aliases: --vad_method] [possible values: auto, energy, pyannote, silero]"
        ),
        "help should list auto as the VAD default while preserving explicit choices"
    );
}

#[test]
fn inspect_models_native_diarization_defaults_to_native_model() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args([
            "inspect-models",
            "--speaker_embedding_bundle",
            "models/speakers",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"modelId\": \"native-spectral-speaker-baseline\"",
        ));
}

#[test]
fn inspect_models_maps_strict_contained_speaker_assignment_policy() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args([
            "inspect-models",
            "--speaker_embedding_bundle",
            "models/speakers",
            "--speaker-assignment-policy",
            "strict-contained",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"assignmentPolicy\": \"strictContained\"",
        ));
}

#[test]
fn transcribe_segment_resolution_accepts_whisperx_values_and_legacy_alias() {
    for value in ["sentence", "chunk", "segment"] {
        let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
        command
            .args(["input.wav", "--segment_resolution", value, "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("--segment-resolution"));
    }
}

#[test]
fn top_level_help_lists_underscore_aliases() {
    let help = command_stdout(["input.wav", "--help"]);
    for expected in [
        "--output_format",
        "--align_model",
        "--vad_method",
        "--vad_model_bundle",
        "--vad_model_file",
        "--translation_model",
        "--translation_bundle",
        "--translation_source_language",
        "--translation_target_language",
        "--translation_max_new_tokens",
        "--hf_token",
        "--max_line_width",
    ] {
        assert!(help.contains(expected), "help should contain `{expected}`");
    }
}

#[test]
fn transcribe_help_lists_native_json_format() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["transcribe", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("native-json"))
        .stdout(predicate::str::contains("tsv"))
        .stdout(predicate::str::contains("aud"))
        .stdout(predicate::str::contains("output_format"));
}

#[test]
fn top_level_help_lists_parity_fixtures() {
    let help = command_stdout(["--help"]);
    assert!(help.contains("live-transcribe"));
    assert!(help.contains("parity-fixtures"));
    assert!(help.contains("parity-bench"));
    assert!(help.contains("parity-summary"));
    assert!(help.contains("parity-preflight"));
    assert!(help.contains("parity-goldens"));
}

#[test]
fn live_transcribe_help_lists_live_feed_options() {
    let help = command_stdout(["live-transcribe", "--help"]);
    for expected in [
        "<SOURCE>",
        "--model",
        "--model-dir",
        "--model-cache-only",
        "--language",
        "--ffmpeg-bin",
        "--ffmpeg-input-option",
        "--ffmpeg-output-option",
        "--window-seconds",
        "--hop-seconds",
        "--finalize-lag-seconds",
        "--max-buffer-lag-seconds",
    ] {
        assert!(help.contains(expected), "help should contain `{expected}`");
    }
}

#[test]
fn live_transcribe_parses_live_feed_options_before_help() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args([
            "live-transcribe",
            "rtsp://example.test/live",
            "--model",
            "tiny.en",
            "--model-dir",
            "models",
            "--model-cache-only",
            "--language",
            "en",
            "--ffmpeg-bin",
            "custom-ffmpeg",
            "--ffmpeg-input-option",
            "-rtsp_transport",
            "--ffmpeg-input-option",
            "tcp",
            "--ffmpeg-output-option",
            "-hide_banner",
            "--window-seconds",
            "5",
            "--hop-seconds",
            "2.5",
            "--finalize-lag-seconds",
            "5",
            "--max-buffer-lag-seconds",
            "30",
            "--help",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("--ffmpeg-input-option"))
        .stdout(predicate::str::contains("--max-buffer-lag-seconds"));
}

#[test]
fn live_transcribe_prints_ffmpeg_plan_without_launching_ffmpeg() {
    let output = Command::cargo_bin("native-whisperx")
        .expect("binary should build")
        .args([
            "live-transcribe",
            "rtsp://example.test/live",
            "--ffmpeg-bin",
            "custom-ffmpeg",
            "--ffmpeg-input-option",
            "-rtsp_transport",
            "--ffmpeg-input-option",
            "tcp",
            "--ffmpeg-output-option",
            "-hide_banner",
            "--print-ffmpeg-plan",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let plan: serde_json::Value = serde_json::from_slice(&output).expect("ffmpeg plan json");

    assert_eq!(plan["program"], "custom-ffmpeg");
    assert_eq!(
        plan["args"],
        serde_json::json!([
            "-rtsp_transport",
            "tcp",
            "-i",
            "rtsp://example.test/live",
            "-vn",
            "-hide_banner",
            "-ac",
            "1",
            "-ar",
            "16000",
            "-f",
            "f32le",
            "pipe:1"
        ])
    );
}

#[cfg(unix)]
#[test]
fn live_transcribe_runs_fake_ffmpeg_and_emits_jsonl_events() {
    let temp = tempfile::tempdir().expect("tempdir");
    let ffmpeg = fake_ffmpeg_script(temp.path(), 7.5, 0);

    let output = Command::cargo_bin("native-whisperx")
        .expect("binary should build")
        .args([
            "live-transcribe",
            "fake-live-source",
            "--ffmpeg-bin",
            ffmpeg.to_str().expect("script path utf8"),
            "--model",
            "tiny.en",
            "--language",
            "en",
            "--__fake-live-asr-text",
            "hello world",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let events = jsonl_events(&output);
    let event_names = events
        .iter()
        .map(|event| event["event"].as_str().expect("event name"))
        .collect::<Vec<_>>();
    let sequences = events
        .iter()
        .map(|event| event["sequence"].as_u64().expect("sequence"))
        .collect::<Vec<_>>();

    assert!(event_names.starts_with(&["sessionStarted", "partial"]));
    assert!(event_names.contains(&"final"));
    assert_eq!(event_names.last(), Some(&"sessionEnded"));
    assert_eq!(sequences, (0..sequences.len() as u64).collect::<Vec<_>>());
    assert_eq!(events[0]["sampleRate"], 16000);
    assert_eq!(events[0]["channels"], 1);
    assert_eq!(events[0]["modelId"], "tiny.en");
    assert_eq!(events[0]["language"], "en");
    assert!(events
        .iter()
        .any(|event| event["event"] == "partial" && event["windowStartAtUtc"].is_string()));
    assert!(events.iter().any(|event| event["event"] == "final"
        && event["startSeconds"] == 0.4
        && event["startAtUtc"].is_string()
        && event["text"] == "hello world"));
    assert_eq!(
        events.last().expect("session ended")["reason"],
        serde_json::json!("completed")
    );
}

#[cfg(unix)]
#[test]
fn live_transcribe_emits_error_and_exits_nonzero_when_buffer_lag_exceeds_limit() {
    let temp = tempfile::tempdir().expect("tempdir");
    let ffmpeg = fake_ffmpeg_script(temp.path(), 5.25, 0);

    let output = Command::cargo_bin("native-whisperx")
        .expect("binary should build")
        .args([
            "live-transcribe",
            "fake-live-source",
            "--ffmpeg-bin",
            ffmpeg.to_str().expect("script path utf8"),
            "--max-buffer-lag-seconds",
            "0.001",
            "--__fake-live-asr-text",
            "hello world",
        ])
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();
    let events = jsonl_events(&output);

    assert!(events.iter().any(|event| {
        event["event"] == "error"
            && event["message"]
                .as_str()
                .is_some_and(|message| message.contains("processing fell behind live input"))
            && event["recoverable"] == false
    }));
    assert_eq!(
        events.last().expect("session ended")["reason"],
        serde_json::json!("error")
    );
}

#[test]
fn parity_fixtures_workflow_exposes_final_full_surface_gate_with_performance_gate() {
    let workflow =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/parity-fixtures.yml");
    let workflow = fs::read_to_string(workflow).expect("workflow should exist");

    assert!(workflow.contains("- final-full-surface"));
    assert!(workflow.contains("manifest=\"tests/parity/full-resource-fixtures.json\""));
    assert!(workflow.contains("fixture_args+=(\"--require-non-gating-passed\")"));
    assert!(workflow.contains("preflight_report=$output_dir/preflight.json"));
    assert!(workflow.contains("\"--allow-missing-report\""));
    assert!(workflow.contains("\"--preflight-report\""));
    assert!(workflow.contains("${{ steps.parity.outputs.raw_report }}"));
    assert!(workflow.contains("${{ steps.parity.outputs.preflight_report }}"));
    assert!(workflow.contains("${{ steps.parity.outputs.summary_report }}"));
    assert!(workflow.contains("${{ steps.parity.outputs.benchmark_report }}"));
    assert!(workflow.contains("${{ steps.parity.outputs.multi_input_benchmark_report }}"));
    assert!(workflow.contains("${{ steps.parity.outputs.progress_log }}"));
    assert!(workflow.contains("Run Rust-Native benchmark ladder"));
    assert!(workflow.contains("tests/parity/rust-native-bench-fixtures.json"));
    assert!(workflow.contains("Run Rust-Native multi-input benchmark report"));
    assert!(workflow.contains("tests/parity/rust-native-multi-input-bench-fixtures.json"));
    assert!(workflow.contains("rust-native-multi-input-bench.json"));
    assert!(workflow.contains("--report-only"));
    assert!(workflow.contains("nativeFasterThanWhisperx"));
    assert!(workflow.contains("nativeSpeedupRatio >= 1.001"));
    assert!(workflow.contains("benchmark report passed="));
    assert!(!workflow.contains("\"--native-only\""));
}

#[test]
fn parity_fixtures_help_lists_local_suite_options() {
    let help = command_stdout(["parity-fixtures", "--help"]);
    for expected in [
        "<MANIFEST>",
        "--root",
        "--whisperx-command",
        "--output-dir",
        "--model-dir",
        "--model-cache-only",
        "--case",
        "--case-timeout-seconds",
        "--require-non-gating-passed",
    ] {
        assert!(help.contains(expected), "help should contain `{expected}`");
    }
}

#[test]
fn parity_bench_help_lists_benchmark_options() {
    let help = command_stdout(["parity-bench", "--help"]);
    for expected in [
        "<MANIFEST>",
        "--root",
        "--whisperx-command",
        "--model-dir",
        "--model-cache-only",
        "--iterations",
        "--warmups",
        "--case",
        "--case-timeout-seconds",
        "--native-only",
        "--report-only",
        "--json",
    ] {
        assert!(help.contains(expected), "help should contain `{expected}`");
    }
}

#[test]
fn parity_bench_empty_manifest_fails_before_reporting_success() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    fs::write(&manifest, r#"{"fixtures":[]}"#).expect("manifest");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-bench")
        .arg(&manifest)
        .arg("--root")
        .arg(temp.path())
        .arg("--iterations")
        .arg("1")
        .arg("--warmups")
        .arg("1")
        .arg("--native-only")
        .arg("--case-timeout-seconds")
        .arg("900")
        .arg("--json")
        .assert()
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains(
            "parity benchmark manifest must contain at least one fixture case",
        ));
}

#[test]
fn parity_bench_rust_native_ladder_cases_are_selectable_with_timeout_reporting() {
    let temp = tempfile::tempdir().expect("tempdir");
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/parity/rust-native-bench-fixtures.json");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-bench")
        .arg(fixture)
        .arg("--root")
        .arg(temp.path())
        .arg("--native-only")
        .arg("--case")
        .arg("shrek-retold-3m-large-v3-turbo-cuda")
        .arg("--case")
        .arg("shrek-retold-10m-large-v3-turbo-cuda")
        .arg("--case-timeout-seconds")
        .arg("0")
        .arg("--json")
        .assert()
        .failure()
        .stdout(predicate::str::contains("\"passed\": false"))
        .stdout(predicate::str::contains("\"timedOut\": true"))
        .stdout(predicate::str::contains(
            "\"name\": \"shrek-retold-3m-large-v3-turbo-cuda\"",
        ))
        .stdout(predicate::str::contains(
            "\"name\": \"shrek-retold-10m-large-v3-turbo-cuda\"",
        ))
        .stdout(
            predicate::str::contains("\"name\": \"shrek-retold-30s-large-v3-turbo-cuda\"").not(),
        );
}

#[test]
fn parity_bench_multi_input_case_is_selectable_with_report_only_timeout() {
    let temp = tempfile::tempdir().expect("tempdir");
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/parity/rust-native-multi-input-bench-fixtures.json");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-bench")
        .arg(fixture)
        .arg("--root")
        .arg(temp.path())
        .arg("--native-only")
        .arg("--case")
        .arg("shrek-retold-5x3m-large-v3-turbo-cuda")
        .arg("--case-timeout-seconds")
        .arg("0")
        .arg("--report-only")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"passed\": false"))
        .stdout(predicate::str::contains("\"kind\": \"multiInput\""))
        .stdout(predicate::str::contains("\"timedOut\": true"))
        .stdout(predicate::str::contains(
            "\"name\": \"shrek-retold-5x3m-large-v3-turbo-cuda\"",
        ));
}

#[test]
fn parity_bench_native_only_case_error_still_emits_json_report() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    fs::write(
        &manifest,
        r#"{
          "fixtures": [
            {
              "name": "missing-audio",
              "input": "audio/missing.wav",
              "nativeAsr": { "modelId": "tiny.en" },
              "alignment": { "enabled": false }
            }
          ]
        }"#,
    )
    .expect("manifest");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-bench")
        .arg(&manifest)
        .arg("--root")
        .arg(temp.path())
        .arg("--native-only")
        .arg("--iterations")
        .arg("1")
        .arg("--warmups")
        .arg("0")
        .arg("--json")
        .assert()
        .failure()
        .stdout(predicate::str::contains("\"passed\": false"))
        .stdout(predicate::str::contains("\"name\": \"missing-audio\""))
        .stdout(predicate::str::contains("\"error\""));
}

#[test]
fn parity_bench_report_only_exits_success_on_failed_case() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    fs::write(
        &manifest,
        r#"{
          "fixtures": [
            {
              "name": "missing-audio",
              "input": "audio/missing.wav",
              "nativeAsr": { "modelId": "tiny.en" },
              "alignment": { "enabled": false }
            }
          ]
        }"#,
    )
    .expect("manifest");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-bench")
        .arg(&manifest)
        .arg("--root")
        .arg(temp.path())
        .arg("--native-only")
        .arg("--iterations")
        .arg("1")
        .arg("--warmups")
        .arg("0")
        .arg("--report-only")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"passed\": false"))
        .stdout(predicate::str::contains("\"name\": \"missing-audio\""))
        .stdout(predicate::str::contains("\"error\""));
}

#[test]
#[ignore = "requires SMOKE_ROOT with Shrek-derived 30s audio, cached large-v3-turbo CUDA assets, and Silero VAD"]
fn parity_bench_rust_native_ladder_30s_native_only_smoke_emits_failure_json() {
    let smoke_root = std::env::var_os("SMOKE_ROOT").expect("SMOKE_ROOT must be set");
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/parity/rust-native-bench-fixtures.json");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-bench")
        .arg(fixture)
        .arg("--root")
        .arg(smoke_root)
        .arg("--native-only")
        .arg("--model-cache-only")
        .arg("--case")
        .arg("shrek-retold-30s-large-v3-turbo-cuda")
        .arg("--case-timeout-seconds")
        .arg("900")
        .arg("--json")
        .assert()
        .failure()
        .stdout(predicate::str::contains(
            "\"name\": \"shrek-retold-30s-large-v3-turbo-cuda\"",
        ))
        .stdout(predicate::str::contains("\"nativeOnly\": true"))
        .stdout(predicate::str::contains("\"native\""))
        .stdout(predicate::str::contains("\"phases\""))
        .stdout(predicate::str::contains("\"realtimeFactor\""));
}

#[test]
fn parity_summary_compacts_fixture_report() {
    let temp = tempfile::tempdir().expect("tempdir");
    let report = temp.path().join("report.json");
    fs::write(
        &report,
        r#"{
          "passed": true,
          "cases": [
            {
              "name": "case-a",
              "gating": true,
              "passed": true,
              "startedAt": "1710000000.000",
              "elapsedSeconds": 1.25,
              "timedOut": false,
              "missingRequiredDiagnostics": [],
              "expectedOutputMatches": [
                {
                  "format": "srt",
                  "comparison": "exact",
                  "gating": false,
                  "expectedPath": "expected.srt",
                  "actualPath": "actual.srt",
                  "passed": false,
                  "difference": "byte mismatch"
                }
              ],
              "failureSummary": []
            }
          ]
        }"#,
    )
    .expect("report");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-summary")
        .arg(&report)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"passed\": true"))
        .stdout(predicate::str::contains("\"elapsedSeconds\": 1.25"))
        .stdout(predicate::str::contains("\"strictComparisonFailures\": []"))
        .stdout(predicate::str::contains("\"reportOnlyDifferences\": ["))
        .stdout(predicate::str::contains(
            "srt exact output differs: byte mismatch",
        ));
}

#[test]
fn parity_summary_reports_gating_failures() {
    let temp = tempfile::tempdir().expect("tempdir");
    let report = temp.path().join("report.json");
    fs::write(
        &report,
        r#"{
          "passed": false,
          "cases": [
            {
              "name": "gating-case",
              "gating": true,
              "passed": false,
              "missingRequiredDiagnostics": [
                "asrModelSource=hugging-face-cache"
              ],
              "error": "segment timing differs at segment 0: native start=0.000s native end=1.000s, reference start=0.250s reference end=1.000s, start_delta=0.250s end_delta=0.000s tolerance=0.100s",
              "expectedOutputMatches": [],
              "failureSummary": []
            },
            {
              "name": "report-only-case",
              "gating": false,
              "passed": false,
              "error": "non-gating failed",
              "missingRequiredDiagnostics": [],
              "expectedOutputMatches": [],
              "failureSummary": []
            }
          ]
        }"#,
    )
    .expect("report");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    let output = command
        .arg("parity-summary")
        .arg(&report)
        .output()
        .expect("summary command should run");

    assert!(
        output.status.success(),
        "summary should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let summary: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("summary should be json");
    let failures = summary["gatingFailures"]
        .as_array()
        .expect("summary should include gatingFailures");
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0]["name"], "gating-case");
    let failure_text = serde_json::to_string(&failures[0]).expect("failure json");
    assert!(failure_text.contains("missing required diagnostic: asrModelSource=hugging-face-cache"));
    assert!(failure_text.contains("reference start=0.250s"));
}

#[test]
fn parity_summary_reports_preflight_failures_when_fixture_report_is_missing() {
    let temp = tempfile::tempdir().expect("tempdir");
    let report = temp.path().join("missing-report.json");
    let preflight = temp.path().join("preflight.json");
    let smoke_root = temp.path().join("smoke");
    let model_dir = smoke_root.join("models");
    let output_dir = smoke_root.join("out/final-full-surface-parity");
    let progress_log = output_dir.join("progress.log");
    fs::write(
        &preflight,
        format!(
            r#"{{
          "passed": false,
          "manifest": "{manifest}",
          "root": "{root}",
          "whisperxCommand": "{whisperx_command}",
          "modelDir": "{model_dir}",
          "sourceCheckoutTag": null,
          "cases": [
            {{
              "name": "gating-case",
              "gating": true,
              "passed": false,
              "missing": ["expected JSON {root}/expected/gating-case.json does not exist"],
              "warnings": []
            }},
            {{
              "name": "report-only-case",
              "gating": false,
              "passed": false,
              "missing": ["audio {root}/audio/report-only.wav does not exist"],
              "warnings": []
            }}
          ]
        }}"#,
            manifest = "tests/parity/full-resource-fixtures.json",
            root = smoke_root.display(),
            whisperx_command = ".audio-tools/whisperx-venv/bin/whisperx",
            model_dir = model_dir.display()
        ),
    )
    .expect("preflight");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    let output = command
        .arg("parity-summary")
        .arg(&report)
        .arg("--allow-missing-report")
        .arg("--preflight-report")
        .arg(&preflight)
        .arg("--suite")
        .arg("final-full-surface")
        .arg("--features")
        .arg("whisperx-compat,media-decode,silero-vad,pyannote-vad,pyannote-diarization,cuda")
        .arg("--runner")
        .arg("self-hosted")
        .arg("--manifest")
        .arg("tests/parity/full-resource-fixtures.json")
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--smoke-root")
        .arg(&smoke_root)
        .arg("--model-dir")
        .arg(&model_dir)
        .arg("--whisperx-command")
        .arg(".audio-tools/whisperx-venv/bin/whisperx")
        .arg("--progress-log")
        .arg(&progress_log)
        .arg("--ort-dylib-path")
        .arg("/opt/onnxruntime/lib/libonnxruntime.so")
        .output()
        .expect("summary command should run");

    assert!(
        output.status.success(),
        "summary should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let summary: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("summary should be json");
    assert_eq!(summary["passed"], false);
    assert_eq!(summary["workflow"]["suite"], "final-full-surface");
    assert_eq!(summary["workflow"]["features"][0], "whisperx-compat");
    assert_eq!(summary["workflow"]["runner"], "self-hosted");
    assert_eq!(
        summary["workflow"]["smokeRoot"],
        smoke_root.display().to_string()
    );
    assert_eq!(
        summary["workflow"]["modelDir"],
        model_dir.display().to_string()
    );
    assert_eq!(
        summary["workflow"]["ortDylibPath"],
        "/opt/onnxruntime/lib/libonnxruntime.so"
    );
    assert_eq!(summary["rawReportMissing"], true);
    assert_eq!(summary["preflight"]["passed"], false);
    assert_eq!(summary["gatingFailures"][0]["name"], "gating-case");
    assert_eq!(summary["gatingFailures"][0]["kind"], "preflight");
    assert_eq!(summary["nonGatingFailures"][0]["name"], "report-only-case");
    assert_eq!(summary["nonGatingFailures"][0]["kind"], "preflight");
    assert_eq!(summary["skippedCases"].as_array().unwrap().len(), 2);
    assert_eq!(summary["skippedCases"][0]["reason"], "preflight failed");
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_fixtures_requires_root_or_smoke_root() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    fs::write(&manifest, r#"{"fixtures":[]}"#).expect("manifest");
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .env_remove("SMOKE_ROOT")
        .current_dir(temp.path())
        .arg("parity-fixtures")
        .arg(&manifest)
        .assert()
        .failure()
        .stderr(predicate::str::contains("SMOKE_ROOT"))
        .stderr(predicate::str::contains("--root"));
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_fixtures_manifest_parse_errors_name_manifest() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    fs::write(&manifest, b"not json").expect("manifest");
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-fixtures")
        .arg(&manifest)
        .arg("--root")
        .arg(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to parse"))
        .stderr(predicate::str::contains(
            manifest.to_string_lossy().as_ref(),
        ));
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_fixtures_case_filter_runs_matching_case() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    fs::write(
        &manifest,
        r#"{
          "fixtures": [
            { "name": "case-a", "input": "audio/a.wav" },
            { "name": "case-b", "input": "audio/b.wav" }
          ]
        }"#,
    )
    .expect("manifest");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-fixtures")
        .arg(&manifest)
        .arg("--root")
        .arg(temp.path())
        .arg("--case")
        .arg("case-a")
        .assert()
        .failure()
        .stdout(predicate::str::contains("\"name\": \"case-a\""))
        .stdout(predicate::str::contains("\"name\": \"case-b\"").not())
        .stderr(predicate::str::contains(
            "parity-fixtures: starting case 1/1: case-a",
        ))
        .stderr(predicate::str::contains(
            "parity-fixtures: completed case 1/1: case-a failed",
        ))
        .stderr(predicate::str::contains(
            "one or more parity fixtures failed",
        ));
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_fixtures_case_filter_rejects_missing_case() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    fs::write(
        &manifest,
        r#"{
          "fixtures": [
            { "name": "case-a", "input": "audio/a.wav" }
          ]
        }"#,
    )
    .expect("manifest");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-fixtures")
        .arg(&manifest)
        .arg("--root")
        .arg(temp.path())
        .arg("--case")
        .arg("missing")
        .assert()
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains(
            "no fixture case named missing matched the manifest",
        ));
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_fixtures_case_filter_accepts_multiple_cases() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    fs::write(
        &manifest,
        r#"{
          "fixtures": [
            { "name": "case-a", "input": "audio/a.wav" },
            { "name": "case-b", "input": "audio/b.wav" },
            { "name": "case-c", "input": "audio/c.wav" }
          ]
        }"#,
    )
    .expect("manifest");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-fixtures")
        .arg(&manifest)
        .arg("--root")
        .arg(temp.path())
        .arg("--case")
        .arg("case-a")
        .arg("--case")
        .arg("case-c")
        .assert()
        .failure()
        .stdout(predicate::str::contains("\"name\": \"case-a\""))
        .stdout(predicate::str::contains("\"name\": \"case-c\""))
        .stdout(predicate::str::contains("\"name\": \"case-b\"").not())
        .stderr(predicate::str::contains(
            "one or more parity fixtures failed",
        ));
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_fixtures_case_timeout_reports_bounded_failure() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    let output_dir = temp.path().join("out");
    fs::write(
        &manifest,
        r#"{
          "fixtures": [
            { "name": "slow-case", "input": "audio/a.wav" }
          ]
        }"#,
    )
    .expect("manifest");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-fixtures")
        .arg(&manifest)
        .arg("--root")
        .arg(temp.path())
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--case-timeout-seconds")
        .arg("0")
        .assert()
        .failure()
        .stdout(predicate::str::contains("\"name\": \"slow-case\""))
        .stdout(predicate::str::contains("exceeded timeout"))
        .stderr(predicate::str::contains(
            "parity-fixtures: timed out case 1/1: slow-case",
        ))
        .stderr(predicate::str::contains(
            "one or more parity fixtures failed",
        ));

    let report = fs::read_to_string(output_dir.join("report.json")).expect("fixture report");
    assert!(report.contains("\"name\": \"slow-case\""));
    assert!(report.contains("\"timedOut\": true"));
}

#[test]
fn parity_preflight_help_lists_resource_checks() {
    let help = command_stdout(["parity-preflight", "--help"]);
    for expected in [
        "<MANIFEST>",
        "--root",
        "--whisperx-command",
        "--model-dir",
        "--require-expected",
        "--include-non-gating",
    ] {
        assert!(help.contains(expected), "help should contain `{expected}`");
    }
}

#[test]
fn parity_goldens_help_lists_generation_options() {
    let help = command_stdout(["parity-goldens", "--help"]);
    for expected in [
        "<MANIFEST>",
        "--root",
        "--whisperx-command",
        "--model-dir",
        "--model-cache-only",
        "--case",
        "--include-non-gating",
        "--overwrite",
        "--dry-run",
    ] {
        assert!(help.contains(expected), "help should contain `{expected}`");
    }
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_goldens_dry_run_prints_whisperx_command_without_writing() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path().join("smoke");
    let manifest = temp.path().join("fixtures.json");
    fs::write(
        &manifest,
        r#"{
          "fixtures": [
            {
              "name": "tiny-output-all-defaults",
              "input": "audio/input.wav",
              "nativeAsr": { "modelId": "tiny.en" },
              "alignment": { "enabled": true, "modelId": "facebook/wav2vec2-base-960h" },
              "whisperx": { "model": "tiny.en" },
              "language": "en",
              "expectedOutputs": [
                { "format": "txt", "path": "expected/whisperx-3.8.6/tiny-output-all-defaults.txt" },
                { "format": "srt", "path": "expected/whisperx-3.8.6/tiny-output-all-defaults.srt" }
              ]
            }
          ]
        }"#,
    )
    .expect("manifest");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-goldens")
        .arg(&manifest)
        .arg("--root")
        .arg(&root)
        .arg("--whisperx-command")
        .arg("/bin/true")
        .arg("--case")
        .arg("tiny-output-all-defaults")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("command: /bin/true"))
        .stdout(predicate::str::contains("--output_format all"))
        .stdout(predicate::str::contains("--return_char_alignments").not())
        .stdout(predicate::str::contains("tiny-output-all-defaults.txt"));

    assert!(
        !root.exists(),
        "dry run should not write generated directories"
    );
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_goldens_dry_run_emits_char_alignment_flag_only_when_requested() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path().join("smoke");
    let manifest = temp.path().join("fixtures.json");
    fs::write(
        &manifest,
        r#"{
          "fixtures": [
            {
              "name": "tiny-en-char-alignments",
              "input": "audio/input.wav",
              "expectedJson": "expected/tiny-en-char-alignments.whisperx.json",
              "nativeAsr": { "modelId": "tiny.en" },
              "alignment": {
                "enabled": true,
                "modelId": "facebook/wav2vec2-base-960h",
                "returnCharAlignments": true
              },
              "whisperx": { "model": "tiny.en" },
              "language": "en"
            }
          ]
        }"#,
    )
    .expect("manifest");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-goldens")
        .arg(&manifest)
        .arg("--root")
        .arg(&root)
        .arg("--whisperx-command")
        .arg("/bin/true")
        .arg("--case")
        .arg("tiny-en-char-alignments")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("--return_char_alignments"))
        .stdout(predicate::str::contains("--return_char_alignments true").not())
        .stdout(predicate::str::contains("--return_char_alignments false").not());
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_goldens_dry_run_passes_highlight_words_bool_value() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path().join("smoke");
    let manifest = temp.path().join("fixtures.json");
    fs::write(
        &manifest,
        r#"{
          "fixtures": [
            {
              "name": "tiny-output-subtitles-highlight",
              "input": "audio/input.wav",
              "nativeAsr": { "modelId": "tiny.en" },
              "alignment": { "enabled": true, "modelId": "facebook/wav2vec2-base-960h" },
              "whisperx": { "model": "tiny.en" },
              "language": "en",
              "output": {
                "formats": ["srt"],
                "basename": "tiny-output-subtitles-highlight",
                "subtitles": { "highlightWords": true }
              },
              "expectedOutputs": [
                { "format": "srt", "path": "expected/whisperx-3.8.6/tiny-output-subtitles-highlight.srt" }
              ]
            }
          ]
        }"#,
    )
    .expect("manifest");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-goldens")
        .arg(&manifest)
        .arg("--root")
        .arg(&root)
        .arg("--whisperx-command")
        .arg("/bin/true")
        .arg("--case")
        .arg("tiny-output-subtitles-highlight")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("--highlight_words True"))
        .stdout(predicate::str::contains("--highlight_words --").not());
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_goldens_dry_run_maps_translation_fixture_to_python_translate() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path().join("smoke");
    let manifest = temp.path().join("fixtures.json");
    fs::write(
        &manifest,
        r#"{
          "fixtures": [
            {
              "name": "small-de-translate-cache",
              "input": "audio/input-de.wav",
              "expectedJson": "expected/whisperx-3.8.6/small-de-translate-cache.json",
              "nativeAsr": { "task": "translate", "modelId": "small" },
              "translation": {
                "enabled": true,
                "modelId": "Helsinki-NLP/opus-mt-de-en",
                "modelCacheOnly": true,
                "sourceLanguage": "de",
                "targetLanguage": "en"
              },
              "alignment": { "enabled": true, "modelId": "facebook/wav2vec2-base-960h" },
              "whisperx": { "model": "small" },
              "language": "de"
            }
          ]
        }"#,
    )
    .expect("manifest");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-goldens")
        .arg(&manifest)
        .arg("--root")
        .arg(&root)
        .arg("--whisperx-command")
        .arg("/bin/true")
        .arg("--case")
        .arg("small-de-translate-cache")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("--task translate"))
        .stdout(predicate::str::contains("--model_cache_only True"))
        .stdout(predicate::str::contains("small-de-translate-cache.json"))
        .stdout(predicate::str::contains("--translation-model").not());

    assert!(
        !root.exists(),
        "dry run should not write generated directories"
    );
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_preflight_reads_smoke_root_from_dotenv() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    let smoke_root = temp.path().join("smoke-root");
    fs::create_dir_all(smoke_root.join("models")).expect("smoke root");
    fs::write(&manifest, r#"{"fixtures":[]}"#).expect("manifest");
    fs::write(temp.path().join(".env"), "SMOKE_ROOT=smoke-root\n").expect("dotenv");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .env_remove("SMOKE_ROOT")
        .current_dir(temp.path())
        .arg("parity-preflight")
        .arg(&manifest)
        .arg("--whisperx-command")
        .arg("/bin/true")
        .assert()
        .success()
        .stdout(predicate::str::contains("Parity preflight: passed"))
        .stdout(predicate::str::contains(
            smoke_root.to_string_lossy().as_ref(),
        ));
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_preflight_manifest_parse_errors_name_manifest() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    fs::write(&manifest, b"not json").expect("manifest");
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("parity-preflight")
        .arg(&manifest)
        .arg("--root")
        .arg(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to parse"))
        .stderr(predicate::str::contains(
            manifest.to_string_lossy().as_ref(),
        ));
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_preflight_requires_root_or_smoke_root() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    fs::write(&manifest, r#"{"fixtures":[]}"#).expect("manifest");
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .env_remove("SMOKE_ROOT")
        .current_dir(temp.path())
        .arg("parity-preflight")
        .arg(&manifest)
        .assert()
        .failure()
        .stderr(predicate::str::contains("SMOKE_ROOT"))
        .stderr(predicate::str::contains("--root"));
}

#[cfg(feature = "whisperx-compat")]
#[test]
fn parity_goldens_requires_root_or_smoke_root() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
    fs::write(&manifest, r#"{"fixtures":[]}"#).expect("manifest");
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .env_remove("SMOKE_ROOT")
        .current_dir(temp.path())
        .arg("parity-goldens")
        .arg(&manifest)
        .assert()
        .failure()
        .stderr(predicate::str::contains("SMOKE_ROOT"))
        .stderr(predicate::str::contains("--root"));
}

#[test]
fn checked_in_asr_fixture_manifest_parses() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/parity/asr-fixtures.json");
    let bytes = fs::read(&fixture).expect("fixture manifest");
    let parsed: native_whisperx::ParityFixtureSuite =
        serde_json::from_slice(&bytes).expect("valid manifest schema");
    assert_eq!(parsed.fixtures.len(), 12);
    assert_eq!(
        parsed
            .fixtures
            .iter()
            .filter(|fixture| fixture.gating)
            .count(),
        12
    );
    assert_eq!(
        parsed
            .fixtures
            .iter()
            .filter(|fixture| !fixture.gating)
            .count(),
        0
    );
    assert!(parsed
        .fixtures
        .iter()
        .any(|fixture| !fixture.expected_outputs.is_empty()));
    assert!(parsed
        .fixtures
        .iter()
        .filter(|fixture| !fixture.expected_outputs.is_empty())
        .any(|fixture| fixture.gating));
    assert!(parsed.fixtures.iter().any(|fixture| {
        fixture.name == "tiny-output-all-defaults"
            && fixture.gating
            && fixture.output.formats == vec![native_whisperx::OutputFormat::All]
            && fixture.expected_outputs.iter().any(|expected| {
                expected.format == native_whisperx::OutputFormat::Audacity && expected.gating
            })
            && fixture.expected_outputs.iter().any(|expected| {
                expected.format == native_whisperx::OutputFormat::Json
                    && expected.comparison == native_whisperx::OutputComparisonMode::JsonSemantic
                    && expected.gating
            })
            && !fixture
                .expected_outputs
                .iter()
                .any(|expected| expected.format == native_whisperx::OutputFormat::NativeJson)
    }));
    assert!(parsed.fixtures.iter().any(|fixture| {
        fixture.name == "tiny-output-subtitles-highlight"
            && fixture.gating
            && fixture.expected_outputs.len() == 4
            && fixture
                .expected_outputs
                .iter()
                .any(|expected| !expected.gating)
    }));
    assert!(parsed
        .fixtures
        .iter()
        .any(|fixture| fixture.name == "small-de-translate-cache"
            && fixture.gating
            && fixture.translation.enabled
            && fixture.translation.model_cache_only));
    assert!(parsed.fixtures.iter().any(|fixture| {
        fixture.name == "tiny-en-no-align-cache"
            && fixture.gating
            && fixture.comparison.segment_timing
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "asrModelSource=hugging-face-cache")
    }));
    assert!(parsed.fixtures.iter().any(|fixture| {
        fixture.name == "small-en-no-align-cache"
            && fixture.gating
            && fixture.comparison.segment_timing
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "asrModelSource=hugging-face-cache")
    }));
    assert!(parsed.fixtures.iter().any(|fixture| {
        fixture.name == "small-de-no-align-cache"
            && fixture.gating
            && !fixture.comparison.text
            && !fixture.comparison.segment_text
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "asrModelSource=hugging-face-cache")
    }));
    assert!(parsed.fixtures.iter().any(|fixture| {
        fixture.name == "tiny-en-alignment-alias-cache"
            && fixture.gating
            && fixture.comparison.segment_timing
            && fixture.comparison.word_timing
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "alignmentModelId=facebook/wav2vec2-base-960h")
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "alignmentModelSource=hugging-face-cache")
    }));
}

#[test]
fn checked_in_full_resource_fixture_manifest_parses() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/parity/full-resource-fixtures.json");
    let bytes = fs::read(&fixture).expect("fixture manifest");
    let parsed: native_whisperx::ParityFixtureSuite =
        serde_json::from_slice(&bytes).expect("valid manifest schema");
    assert_eq!(parsed.fixtures.len(), 4);
    assert!(parsed.fixtures.iter().any(|fixture| {
        fixture.name == "silero-vad-tiny-en"
            && fixture.gating
            && fixture.vad.method == native_whisperx::VadMethod::Silero
            && fixture.comparison.vad_segment_count
            && fixture.comparison.vad_segment_timing
    }));
    assert!(parsed.fixtures.iter().any(|fixture| {
        fixture.name == "pyannote-vad-tiny-en"
            && fixture.gating
            && fixture.vad.method == native_whisperx::VadMethod::Pyannote
            && fixture.comparison.vad_segment_count
            && fixture.comparison.vad_segment_timing
    }));
    for fixture in parsed.fixtures.iter().filter(|fixture| {
        fixture.name == "diarization-two-speaker-pyannote-reference"
            || fixture.name == "diarization-speaker-embeddings-pyannote-reference"
    }) {
        assert_eq!(
            fixture.expected_target,
            native_whisperx::ExpectedTranscriptTarget::Whisperx
        );
        assert_eq!(fixture.timeout_seconds, Some(240));
        assert!(fixture
            .required_diagnostics
            .iter()
            .any(|diagnostic| diagnostic == "cuda=true"));
        assert!(fixture
            .required_diagnostics
            .iter()
            .any(|diagnostic| diagnostic == "diarizationSpeakerCount=2"));
    }
}

#[test]
fn checked_in_rust_native_bench_fixture_manifest_parses() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/parity/rust-native-bench-fixtures.json");
    let bytes = fs::read(&fixture).expect("fixture manifest");
    let raw: serde_json::Value = serde_json::from_slice(&bytes).expect("valid manifest json");
    let parsed: native_whisperx::ParityFixtureSuite =
        serde_json::from_slice(&bytes).expect("valid manifest schema");
    assert_eq!(parsed.fixtures.len(), 3);
    assert!(parsed.fixtures.iter().all(|fixture| !fixture.gating));
    assert!(parsed.fixtures.iter().all(|fixture| {
        fixture.native_asr.model_id == "large-v3-turbo"
            && fixture.native_asr.device == native_whisperx::DevicePreference::Cuda
            && fixture.native_asr.max_batch_size == Some(8)
            && fixture.vad.method == native_whisperx::VadMethod::Silero
            && fixture.alignment.enabled
            && fixture.alignment.model_id == "facebook/wav2vec2-base-960h"
            && fixture.whisperx.compute_type.is_none()
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "alignmentCuda=true")
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "alignmentDevice=cuda:0")
    }));
    assert!(raw["fixtures"]
        .as_array()
        .expect("fixtures")
        .iter()
        .all(|fixture| fixture["alignment"].get("device").is_none()
            && fixture["whisperx"].get("computeType").is_none()));
    let generated_clips = raw["metadata"]["generatedClips"]
        .as_array()
        .expect("generated clip metadata");
    assert_eq!(generated_clips.len(), 3);
    assert!(generated_clips
        .iter()
        .any(|clip| clip["durationSeconds"].as_u64() == Some(30)));
    assert!(generated_clips
        .iter()
        .any(|clip| clip["durationSeconds"].as_u64() == Some(180)));
    assert!(generated_clips
        .iter()
        .any(|clip| clip["durationSeconds"].as_u64() == Some(600)));
    assert!(parsed
        .fixtures
        .iter()
        .any(|fixture| fixture.name == "shrek-retold-30s-large-v3-turbo-cuda"));
    assert!(parsed
        .fixtures
        .iter()
        .any(|fixture| fixture.name == "shrek-retold-3m-large-v3-turbo-cuda"));
    assert!(parsed
        .fixtures
        .iter()
        .any(|fixture| fixture.name == "shrek-retold-10m-large-v3-turbo-cuda"));
}

#[test]
fn checked_in_rust_native_multi_input_bench_fixture_manifest_parses() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/parity/rust-native-multi-input-bench-fixtures.json");
    let bytes = fs::read(&fixture).expect("fixture manifest");
    let raw: serde_json::Value = serde_json::from_slice(&bytes).expect("valid manifest json");
    let parsed: native_whisperx::ParityFixtureSuite =
        serde_json::from_slice(&bytes).expect("valid manifest schema");

    assert!(parsed.fixtures.is_empty());
    assert_eq!(parsed.multi_input_fixtures.len(), 1);
    let fixture = &parsed.multi_input_fixtures[0];
    assert_eq!(fixture.name, "shrek-retold-5x3m-large-v3-turbo-cuda");
    assert!(!fixture.gating);
    assert_eq!(fixture.inputs.len(), 5);
    assert_eq!(fixture.clip_seconds_per_input, Some(180.0));
    assert!(fixture.inputs.iter().all(|input| input
        .to_string_lossy()
        .starts_with("audio/shrek-retold-5x3m-slice-")));
    assert_eq!(fixture.native_asr.model_id, "large-v3-turbo");
    assert_eq!(
        fixture.native_asr.device,
        native_whisperx::DevicePreference::Cuda
    );
    assert_eq!(fixture.native_asr.max_batch_size, Some(8));
    assert_eq!(fixture.vad.method, native_whisperx::VadMethod::Silero);
    assert!(fixture.alignment.enabled);
    assert_eq!(fixture.alignment.model_id, "facebook/wav2vec2-base-960h");
    assert!(fixture.output.basename.is_none());
    assert!(fixture.whisperx.compute_type.is_none());
    assert!(fixture
        .required_diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "alignmentCuda=true"));
    assert!(fixture
        .required_diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "alignmentDevice=cuda:0"));
    assert!(raw["multiInputFixtures"]
        .as_array()
        .expect("multi-input fixtures")
        .iter()
        .all(|fixture| fixture["alignment"].get("device").is_none()
            && fixture["whisperx"].get("computeType").is_none()
            && fixture["output"].get("basename").is_none()));
    let generated_clips = raw["metadata"]["generatedClips"]
        .as_array()
        .expect("generated clip metadata");
    assert_eq!(generated_clips.len(), 5);
    assert!(generated_clips.iter().all(|clip| {
        clip["durationSeconds"].as_u64() == Some(180)
            && clip["case"].as_str() == Some("shrek-retold-5x3m-large-v3-turbo-cuda")
    }));
    for offset in ["00:00:00", "00:18:00", "00:36:00", "00:54:00", "01:12:00"] {
        assert!(generated_clips
            .iter()
            .any(|clip| clip["sourceOffset"].as_str() == Some(offset)));
    }
}

#[test]
fn parity_matrix_uses_final_surface_statuses_only() {
    let matrix = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/parity-matrix.md");
    let matrix = fs::read_to_string(matrix).expect("parity matrix");
    let allowed = [
        "rust-native complete",
        "blocked",
        "reference-only",
        "intentionally unsupported",
    ];
    let mut checked_rows = 0;

    for line in matrix.lines() {
        if !line.starts_with("| ") || line.starts_with("| Area ") || line.starts_with("| ---") {
            continue;
        }
        let escaped = line.replace("\\|", "__PIPE__");
        let columns = escaped.split('|').map(str::trim).collect::<Vec<_>>();
        if columns.len() < 5 {
            continue;
        }
        let status = columns[3].trim_matches('`');
        if !allowed.contains(&status) {
            panic!("unexpected parity matrix status `{status}` in row `{line}`");
        }
        checked_rows += 1;
    }

    assert!(
        checked_rows >= 30,
        "expected CLI surface rows to be checked"
    );
}

#[test]
fn parity_fixture_manifest_accepts_gating_and_expected_outputs() {
    let parsed: native_whisperx::ParityFixtureSuite = serde_json::from_str(
        r#"{
          "fixtures": [
            {
              "name": "non-gating-output",
              "gating": false,
              "input": "audio/input.wav",
              "expectedOutputs": [
                {
                  "format": "json",
                  "path": "expected/output.json",
                  "comparison": "jsonSemantic"
                }
              ]
            }
          ]
        }"#,
    )
    .expect("manifest should parse");

    assert!(!parsed.fixtures[0].gating);
    assert_eq!(parsed.fixtures[0].expected_outputs.len(), 1);
}

#[test]
fn transcribe_rejects_highlight_without_alignment() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--no_align", "--highlight_words"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("require alignment"));
}

#[test]
fn transcribe_rejects_max_line_width_without_alignment() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--no_align", "--max_line_width", "42"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("require alignment"));
}

#[test]
fn transcribe_rejects_max_line_count_without_alignment() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--no_align", "--max_line_count", "2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("require alignment"));
}

#[test]
fn transcribe_rejects_native_translate_without_no_align() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--task", "translate"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "native --task translate requires --translation-model or --translation-bundle",
        ));
}

#[test]
fn transcribe_rejects_native_speaker_embeddings() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--diarize", "--speaker_embeddings"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "native speaker embeddings require --diarize-model pyannote/... and --diarization-model-bundle",
        ));
}

#[test]
fn transcribe_rejects_native_explicit_pyannote_diarize_model() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args([
            "input.wav",
            "--diarize",
            "--diarize-model",
            "pyannote/speaker-diarization-community-1",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "native pyannote diarization requires --diarization-model-bundle",
        ));
}

#[test]
fn transcribe_rejects_native_diarization_bundle_without_pyannote_model() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args([
            "input.wav",
            "--diarization-model-bundle",
            "/models/pyannote-diarization",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "native --diarization-model-bundle requires --diarize-model pyannote/...",
        ));
}

#[test]
fn transcribe_rejects_native_pyannote_before_audio_io() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--vad_method", "pyannote"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("pyannote"))
        .stderr(predicate::str::contains("native decode failed").not());
}

#[cfg(feature = "pyannote-vad")]
#[test]
fn transcribe_rejects_native_pyannote_without_bundle_before_audio_io() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--vad-method", "pyannote"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("native pyannote VAD requires"))
        .stderr(predicate::str::contains("--vad-model-bundle"))
        .stderr(predicate::str::contains("native decode failed").not());
}

#[test]
fn transcribe_diarize_defaults_to_automatic_pyannote_resource_resolution() {
    let temp = tempfile::tempdir().expect("tempdir");
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args([
            "transcribe",
            "missing.wav",
            "--diarize",
            "--model-cache-only",
            "--no-align",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "failed to resolve automatic Workflow Composition resources before transcription",
        ))
        .stderr(predicate::str::contains("automatic pyannote VAD"))
        .stderr(predicate::str::contains("automatic pyannote diarization"))
        .stderr(predicate::str::contains("cache-only=true"));
}

#[test]
fn transcribe_explicit_lower_quality_diarization_does_not_use_automatic_pyannote_lookup() {
    let temp = tempfile::tempdir().expect("tempdir");
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args([
            "transcribe",
            "missing.wav",
            "--diarize",
            "--vad-method",
            "energy",
            "--diarize-model",
            "native-spectral-speaker-baseline",
            "--model-cache-only",
            "--no-align",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("native decode failed"))
        .stderr(predicate::str::contains("automatic pyannote").not());
}

#[cfg(not(feature = "silero-vad"))]
#[test]
fn transcribe_rejects_native_silero_without_feature_before_audio_io() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--vad_method", "silero"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("silero-vad feature"));
}

#[cfg(feature = "silero-vad")]
#[test]
fn transcribe_rejects_native_silero_without_bundle_before_audio_io() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--vad-method", "silero"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("native Silero VAD requires"))
        .stderr(predicate::str::contains("--vad-model-bundle"))
        .stderr(predicate::str::contains("native decode failed").not());
}

#[test]
fn native_transcribe_failure_prints_plain_progress_without_report_json() {
    let temp = tempfile::tempdir().expect("tempdir");
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args([
            "transcribe",
            "missing.wav",
            "--no-align",
            "--model-cache-only",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("progress run start total_files=1"))
        .stdout(predicate::str::contains(
            "progress file start index=1/1 input=missing.wav",
        ))
        .stdout(predicate::str::contains(
            "progress failure file=1 input=missing.wav task=none",
        ))
        .stdout(predicate::str::contains("\"response\"").not())
        .stderr(predicate::str::contains("automatic pyannote").not());
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn transcribe_report_writes_single_report_file() {
    let fake = FakeWhisperx::new();
    fs::write(fake.root().join("input.wav"), b"fake audio").expect("input");
    let report = fake.root().join("report.json");

    let mut command = fake.command();
    command
        .current_dir(fake.root())
        .arg("transcribe")
        .arg("input.wav")
        .args([
            "--provider",
            "external-whisperx",
            "--no-align",
            "--format",
            "json",
            "--report",
        ])
        .arg(&report)
        .assert()
        .success()
        .stdout(predicate::str::contains("fake transcript text").not());

    let report_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(report).expect("report")).expect("report json");
    assert!(
        report_json.is_object(),
        "single-input report should be an object"
    );
    assert_eq!(report_json["response"]["transcript"]["source"], "input.wav");
    assert!(
        report_json.get("workflowSelection").is_none(),
        "external WhisperX report JSON should not invent native workflow selection metadata"
    );
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn transcribe_report_writes_multi_report_array() {
    let fake = FakeWhisperx::new();
    fs::write(fake.root().join("first.wav"), b"fake audio").expect("first");
    fs::write(fake.root().join("second.wav"), b"fake audio").expect("second");
    let report = fake.root().join("report.json");

    let mut command = fake.command();
    command
        .current_dir(fake.root())
        .args([
            "transcribe",
            "first.wav",
            "second.wav",
            "--provider",
            "external-whisperx",
            "--no-align",
            "--format",
            "json",
            "--report",
        ])
        .arg(&report)
        .assert()
        .success()
        .stdout(predicate::str::contains("fake transcript text").not());

    let report_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(report).expect("report")).expect("report json");
    let reports = report_json
        .as_array()
        .expect("multi-input report should be an array");
    assert_eq!(reports.len(), 2);
    assert_eq!(reports[0]["response"]["transcript"]["source"], "first.wav");
    assert_eq!(reports[1]["response"]["transcript"]["source"], "second.wav");
}

#[test]
fn transcribe_rejects_basename_with_multiple_inputs() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["one.wav", "two.wav", "--basename", "fixed"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("multiple input files"));
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn transcribe_expands_relative_glob_inputs() {
    let fake = FakeWhisperx::new();
    let audio_dir = fake.root().join("audio");
    fs::create_dir_all(&audio_dir).expect("audio dir");
    fs::write(audio_dir.join("b.wav"), b"fake audio").expect("b wav");
    fs::write(audio_dir.join("a.wav"), b"fake audio").expect("a wav");

    let mut command = fake.command();
    command
        .current_dir(fake.root())
        .args([
            "transcribe",
            "audio/*.wav",
            "--provider",
            "external-whisperx",
            "--no-align",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"source\": \"audio/a.wav\""))
        .stdout(predicate::str::contains("\"source\": \"audio/b.wav\""));

    assert!(audio_dir.join("a.json").is_file());
    assert!(audio_dir.join("b.json").is_file());
    let argv = fs::read_to_string(fake.argv_path()).expect("argv");
    assert!(argv.contains("audio/a.wav"));
    assert!(argv.contains("audio/b.wav"));
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn transcribe_expands_absolute_glob_inputs() {
    let fake = FakeWhisperx::new();
    let audio_dir = fake.root().join("absolute-audio");
    fs::create_dir_all(&audio_dir).expect("audio dir");
    let first = audio_dir.join("one.wav");
    let second = audio_dir.join("two.wav");
    fs::write(&first, b"fake audio").expect("one wav");
    fs::write(&second, b"fake audio").expect("two wav");
    let pattern = audio_dir.join("*.wav");

    let mut command = fake.command();
    command
        .current_dir(fake.root())
        .arg("transcribe")
        .arg(pattern)
        .args([
            "--provider",
            "external-whisperx",
            "--no-align",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(first.to_string_lossy().as_ref()))
        .stdout(predicate::str::contains(second.to_string_lossy().as_ref()));
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn transcribe_accepts_common_finite_media_paths() {
    let fake = FakeWhisperx::new();
    fs::write(fake.root().join("input.mp3"), b"fake audio").expect("mp3");
    fs::write(fake.root().join("clip.mp4"), b"fake video audio").expect("mp4");

    let mut command = fake.command();
    command
        .current_dir(fake.root())
        .args([
            "transcribe",
            "input.mp3",
            "clip.mp4",
            "--provider",
            "external-whisperx",
            "--no-align",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"source\": \"input.mp3\""))
        .stdout(predicate::str::contains("\"source\": \"clip.mp4\""));

    assert!(fake.root().join("input.json").is_file());
    assert!(fake.root().join("clip.json").is_file());
    let argv = fs::read_to_string(fake.argv_path()).expect("argv");
    assert!(argv.contains("input.mp3"));
    assert!(argv.contains("clip.mp4"));
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn transcribe_expands_mixed_media_glob_patterns() {
    let fake = FakeWhisperx::new();
    let media_dir = fake.root().join("media");
    fs::create_dir_all(&media_dir).expect("media dir");
    fs::write(media_dir.join("lecture.mp4"), b"fake video audio").expect("mp4");
    fs::write(media_dir.join("meeting.mp3"), b"fake audio").expect("mp3");
    fs::write(media_dir.join("notes.txt"), b"not media").expect("text");

    let mut command = fake.command();
    command
        .current_dir(fake.root())
        .args([
            "transcribe",
            "media/*.mp3",
            "media/*.mp4",
            "--provider",
            "external-whisperx",
            "--no-align",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"source\": \"media/meeting.mp3\"",
        ))
        .stdout(predicate::str::contains(
            "\"source\": \"media/lecture.mp4\"",
        ))
        .stdout(predicate::str::contains("notes.txt").not());

    assert!(media_dir.join("meeting.json").is_file());
    assert!(media_dir.join("lecture.json").is_file());
    assert!(!media_dir.join("notes.json").exists());
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn transcribe_broad_glob_does_not_filter_unsupported_files() {
    let fake = FakeWhisperx::new();
    let media_dir = fake.root().join("media");
    fs::create_dir_all(&media_dir).expect("media dir");
    fs::write(media_dir.join("clip.mp3"), b"fake audio").expect("mp3");
    fs::write(media_dir.join("corrupted.bin"), b"not real media").expect("bin");

    let mut command = fake.command();
    command
        .current_dir(fake.root())
        .args([
            "transcribe",
            "media/*",
            "--provider",
            "external-whisperx",
            "--no-align",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"source\": \"media/clip.mp3\""))
        .stdout(predicate::str::contains(
            "\"source\": \"media/corrupted.bin\"",
        ));

    let argv = fs::read_to_string(fake.argv_path()).expect("argv");
    assert!(argv.contains("media/clip.mp3"));
    assert!(argv.contains("media/corrupted.bin"));
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn transcribe_accepts_concrete_input_with_glob_metacharacters() {
    let fake = FakeWhisperx::new();
    let input = "Shrek Retold - Full Movie [pM70TROZQsI].webm";
    fs::write(fake.root().join(input), b"fake audio").expect("input");

    let mut command = fake.command();
    command
        .current_dir(fake.root())
        .args([
            "transcribe",
            input,
            "--provider",
            "external-whisperx",
            "--no-align",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(input));
}

#[test]
fn transcribe_fails_unmatched_glob() {
    let temp = tempfile::tempdir().expect("tempdir");
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args(["transcribe", "missing-*.wav", "--no-align"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("matched no input files"))
        .stderr(predicate::str::contains("missing-*.wav"));
}

#[test]
fn transcribe_rejects_glob_directory_match() {
    let temp = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(temp.path().join("audio-dir")).expect("audio dir");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args(["transcribe", "audio-*", "--no-align"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("matched non-file input"))
        .stderr(predicate::str::contains("audio-dir"));
}

#[test]
fn transcribe_rejects_basename_after_glob_expands_to_multiple_inputs() {
    let temp = tempfile::tempdir().expect("tempdir");
    fs::write(temp.path().join("one.wav"), b"fake audio").expect("one wav");
    fs::write(temp.path().join("two.wav"), b"fake audio").expect("two wav");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args(["transcribe", "*.wav", "--basename", "fixed"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("multiple input files"));
}

#[test]
fn transcribe_rejects_basename_after_mixed_media_globs_expand_to_multiple_inputs() {
    let temp = tempfile::tempdir().expect("tempdir");
    fs::write(temp.path().join("one.mp3"), b"fake audio").expect("one mp3");
    fs::write(temp.path().join("two.mp4"), b"fake video audio").expect("two mp4");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args(["transcribe", "*.mp3", "*.mp4", "--basename", "fixed"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("multiple input files"));
}

#[test]
fn transcribe_rejects_explicit_output_dir_collisions() {
    let temp = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(temp.path().join("day1")).expect("day1");
    fs::create_dir_all(temp.path().join("day2")).expect("day2");
    fs::write(temp.path().join("day1/audio.wav"), b"fake audio").expect("day1 audio");
    fs::write(temp.path().join("day2/audio.wav"), b"fake audio").expect("day2 audio");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args([
            "transcribe",
            "day1/audio.wav",
            "day2/audio.wav",
            "--output-dir",
            "out",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("output basename collision"))
        .stderr(predicate::str::contains("audio"))
        .stderr(predicate::str::contains("day1/audio.wav"))
        .stderr(predicate::str::contains("day2/audio.wav"));
}

#[test]
fn transcribe_rejects_explicit_output_dir_collisions_for_media_inputs() {
    let temp = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(temp.path().join("day1")).expect("day1");
    fs::create_dir_all(temp.path().join("day2")).expect("day2");
    fs::write(temp.path().join("day1/audio.mp3"), b"fake audio").expect("day1 audio");
    fs::write(temp.path().join("day2/audio.mp4"), b"fake video audio").expect("day2 audio");

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .args([
            "transcribe",
            "day1/audio.mp3",
            "day2/audio.mp4",
            "--output-dir",
            "out",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("output basename collision"))
        .stderr(predicate::str::contains("audio"))
        .stderr(predicate::str::contains("day1/audio.mp3"))
        .stderr(predicate::str::contains("day2/audio.mp4"));
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn transcribe_allows_same_stem_without_output_dir() {
    let fake = FakeWhisperx::new();
    let first_dir = fake.root().join("day1");
    let second_dir = fake.root().join("day2");
    fs::create_dir_all(&first_dir).expect("day1");
    fs::create_dir_all(&second_dir).expect("day2");
    fs::write(first_dir.join("audio.wav"), b"fake audio").expect("day1 audio");
    fs::write(second_dir.join("audio.wav"), b"fake audio").expect("day2 audio");

    let mut command = fake.command();
    command
        .current_dir(fake.root())
        .args([
            "transcribe",
            "day1/audio.wav",
            "day2/audio.wav",
            "--provider",
            "external-whisperx",
            "--no-align",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"source\": \"day1/audio.wav\""))
        .stdout(predicate::str::contains("\"source\": \"day2/audio.wav\""));

    assert!(first_dir.join("audio.json").is_file());
    assert!(second_dir.join("audio.json").is_file());
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn transcribe_uses_input_local_output_for_media_inputs_without_output_dir() {
    let fake = FakeWhisperx::new();
    let first_dir = fake.root().join("day1");
    let second_dir = fake.root().join("day2");
    fs::create_dir_all(&first_dir).expect("day1");
    fs::create_dir_all(&second_dir).expect("day2");
    fs::write(first_dir.join("audio.mp3"), b"fake audio").expect("day1 audio");
    fs::write(second_dir.join("audio.mp4"), b"fake video audio").expect("day2 audio");

    let mut command = fake.command();
    command
        .current_dir(fake.root())
        .args([
            "transcribe",
            "day1/audio.mp3",
            "day2/audio.mp4",
            "--provider",
            "external-whisperx",
            "--no-align",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"source\": \"day1/audio.mp3\""))
        .stdout(predicate::str::contains("\"source\": \"day2/audio.mp4\""));

    assert!(first_dir.join("audio.json").is_file());
    assert!(second_dir.join("audio.json").is_file());
}

#[cfg(all(unix, not(feature = "whisperx-compat")))]
#[test]
fn external_provider_fails_before_spawning_when_compatibility_is_disabled() {
    let temp = tempfile::tempdir().expect("tempdir");
    let fake = temp.path().join("whisperx");
    let marker = temp.path().join("spawned");
    fs::write(
        &fake,
        "#!/usr/bin/env sh\nset -eu\ntouch \"$NATIVE_WHISPERX_TEST_MARKER\"\n",
    )
    .expect("fake WhisperX");
    let mut permissions = fs::metadata(&fake).expect("metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake, permissions).expect("chmod");
    fs::write(temp.path().join("input.wav"), b"fake audio").expect("input");
    let original_path = std::env::var_os("PATH").unwrap_or_default();
    let test_path = format!(
        "{}:{}",
        temp.path().display(),
        original_path.to_string_lossy()
    );

    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .current_dir(temp.path())
        .env("PATH", test_path)
        .env("NATIVE_WHISPERX_TEST_MARKER", &marker)
        .args(["input.wav", "--provider", "external-whisperx"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("external WhisperX provider"))
        .stderr(predicate::str::contains("whisperx-compat"))
        .stderr(predicate::str::contains("feature is disabled"));

    assert!(
        !marker.exists(),
        "feature-disabled CLI must not spawn WhisperX"
    );
}

#[cfg(not(feature = "whisperx-compat"))]
#[test]
fn parity_oracle_command_fails_before_native_execution_when_compatibility_is_disabled() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["parity", "missing.wav"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Python WhisperX parity oracle"))
        .stderr(predicate::str::contains("whisperx-compat"))
        .stderr(predicate::str::contains("feature is disabled"))
        .stderr(predicate::str::contains("native decode failed").not());
}

#[test]
fn external_translate_help_parses_without_running_audio() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args([
            "input.wav",
            "--task",
            "translate",
            "--provider",
            "external-whisperx",
            "--help",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("--provider"));
}

#[test]
fn verbose_bool_forms_parse_before_help() {
    for args in [
        vec!["input.wav", "--verbose", "--help"],
        vec!["input.wav", "--verbose", "false", "--help"],
    ] {
        let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
        command
            .args(args)
            .assert()
            .success()
            .stdout(predicate::str::contains("--verbose"));
    }
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn external_whisperx_fake_command_forwards_args_and_imports_json() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempfile::tempdir().expect("tempdir");
    let fake = temp.path().join("whisperx");
    let argv_path = temp.path().join("argv.txt");
    let output_dir = temp.path().join("out");
    fs::write(
        &fake,
        r#"#!/usr/bin/env sh
set -eu
printf '%s\n' "$@" > "$NATIVE_WHISPERX_FAKE_ARGV"
out=""
prev=""
for arg in "$@"; do
  if [ "$prev" = "--output_dir" ]; then
    out="$arg"
  fi
  prev="$arg"
done
mkdir -p "$out"
cat > "$out/fake.json" <<'JSON'
{
  "language": "en",
  "text": "fake transcript text",
  "segments": [
    {
      "id": 0,
      "start": 0.0,
      "end": 1.0,
      "text": "fake transcript text",
      "words": [
        {"word": "fake", "start": 0.0, "end": 0.2}
      ]
    }
  ],
  "word_segments": [
    {"word": "fake", "start": 0.0, "end": 0.2}
  ]
}
JSON
"#,
    )
    .expect("write fake whisperx");
    let mut permissions = fs::metadata(&fake).expect("fake metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake, permissions).expect("chmod fake whisperx");

    let original_path = std::env::var_os("PATH").unwrap_or_default();
    let test_path = format!(
        "{}:{}",
        temp.path().display(),
        original_path.to_string_lossy()
    );
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .env("PATH", test_path)
        .env("NATIVE_WHISPERX_FAKE_ARGV", &argv_path)
        .args([
            "input.wav",
            "--provider",
            "external-whisperx",
            "--model",
            "small",
            "--language",
            "en",
            "--device",
            "cpu",
            "--batch_size",
            "8",
            "--compute_type",
            "int8",
            "--model_cache_only",
            "--vad_method",
            "silero",
            "--vad_onset",
            "0.5",
            "--vad_offset",
            "0.363",
            "--chunk_size",
            "20",
            "--beam_size",
            "5",
            "--print-progress",
            "--diarize",
            "--hf_token",
            "fake-token",
            "--output_dir",
        ])
        .arg(&output_dir)
        .args(["--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("fake transcript text"));

    let argv = fs::read_to_string(argv_path).expect("captured argv");
    for expected in [
        "input.wav",
        "--model\nsmall",
        "--language\nen",
        "--device\ncpu",
        "--batch_size\n8",
        "--compute_type\nint8",
        "--model_cache_only\nTrue",
        "--vad_method\nsilero",
        "--vad_onset\n0.5",
        "--vad_offset\n0.363",
        "--chunk_size\n20",
        "--beam_size\n5",
        "--print_progress",
        "--diarize",
        "--diarize_model\npyannote/speaker-diarization-community-1",
        "--hf_token\nfake-token",
        "--output_format\njson",
        "--output_dir",
        output_dir.to_string_lossy().as_ref(),
    ] {
        assert!(argv.contains(expected), "argv should contain `{expected}`");
    }
}

#[test]
fn runtime_version_short_flag_works() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("-P")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust runtime"));
}

#[test]
fn transcribe_no_align_alias_normalizes_to_disabled_alignment() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--no_align", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--no-align"))
        .stdout(predicate::str::contains("--whisper-bundle"));
}

#[test]
fn top_level_input_uses_transcribe_shape() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("native-json"))
        .stdout(predicate::str::contains("--whisper-bundle"));
}

fn command_stdout<const N: usize>(args: [&str; N]) -> String {
    let output = Command::cargo_bin("native-whisperx")
        .expect("binary should build")
        .args(args)
        .output()
        .expect("command should run");
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("stdout should be utf8")
}

#[cfg(all(unix, feature = "whisperx-compat"))]
struct FakeWhisperx {
    temp: tempfile::TempDir,
    argv_path: PathBuf,
}

#[cfg(all(unix, feature = "whisperx-compat"))]
impl FakeWhisperx {
    fn new() -> Self {
        use std::os::unix::fs::PermissionsExt;

        let temp = tempfile::tempdir().expect("tempdir");
        let fake = temp.path().join("whisperx");
        let argv_path = temp.path().join("argv.txt");
        fs::write(
            &fake,
            r#"#!/usr/bin/env sh
set -eu
printf '%s\n' "$@" >> "$NATIVE_WHISPERX_FAKE_ARGV"
audio="$1"
out=""
prev=""
for arg in "$@"; do
  if [ "$prev" = "--output_dir" ]; then
    out="$arg"
  fi
  prev="$arg"
done
mkdir -p "$out"
stem="$(basename "$audio")"
stem="${stem%.*}"
rm -f "$out"/*.json
cat > "$out/$stem.json" <<JSON
{
  "language": "en",
  "text": "fake transcript text for $audio",
  "segments": [
    {
      "id": 0,
      "start": 0.0,
      "end": 1.0,
      "text": "fake transcript text for $audio",
      "words": [
        {"word": "fake", "start": 0.0, "end": 0.2}
      ]
    }
  ],
  "word_segments": [
    {"word": "fake", "start": 0.0, "end": 0.2}
  ]
}
JSON
"#,
        )
        .expect("write fake whisperx");
        let mut permissions = fs::metadata(&fake).expect("fake metadata").permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&fake, permissions).expect("chmod fake whisperx");

        Self { temp, argv_path }
    }

    fn root(&self) -> &Path {
        self.temp.path()
    }

    fn argv_path(&self) -> &Path {
        &self.argv_path
    }

    fn command(&self) -> Command {
        let original_path = std::env::var_os("PATH").unwrap_or_default();
        let test_path = format!(
            "{}:{}",
            self.temp.path().display(),
            original_path.to_string_lossy()
        );
        let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
        command
            .env("PATH", test_path)
            .env("NATIVE_WHISPERX_FAKE_ARGV", &self.argv_path);
        command
    }
}

fn valid_speaker_library_json() -> &'static str {
    r#"{
      "version": 1,
      "embedding_model": {
        "family": "SpeechBrain",
        "name": "spkrec",
        "version": "1",
        "dimensions": 2
      },
      "profiles": [{
        "id": "speaker-a",
        "label": "Speaker A",
        "embeddings": [{
          "values": [1.0, 0.0],
          "model": {
            "family": "SpeechBrain",
            "name": "spkrec",
            "version": "1",
            "dimensions": 2
          },
          "sample_rate": 16000
        }],
        "metadata": {
          "note": "fixture"
        }
      }]
    }"#
}

fn two_profile_speaker_library_json() -> String {
    valid_speaker_library_json().replace(
        r#"{
        "id": "speaker-a",
        "label": "Speaker A",
        "embeddings": [{
          "values": [1.0, 0.0],
          "model": {
            "family": "SpeechBrain",
            "name": "spkrec",
            "version": "1",
            "dimensions": 2
          },
          "sample_rate": 16000
        }],
        "metadata": {
          "note": "fixture"
        }
      }"#,
        r#"{
        "id": "speaker-a",
        "label": "Speaker A",
        "embeddings": [{
          "values": [1.0, 0.0],
          "model": {
            "family": "SpeechBrain",
            "name": "spkrec",
            "version": "1",
            "dimensions": 2
          },
          "sample_rate": 16000
        }],
        "metadata": {
          "note": "fixture"
        }
      },
      {
        "id": "speaker-b",
        "label": "Speaker B",
        "embeddings": [{
          "values": [0.0, 1.0],
          "model": {
            "family": "SpeechBrain",
            "name": "spkrec",
            "version": "1",
            "dimensions": 2
          },
          "sample_rate": 16000
        }],
        "metadata": {
          "note": "second fixture"
        }
      }"#,
    )
}

fn draft_and_confirmed_speaker_library_json() -> String {
    valid_speaker_library_json().replace(
        r#"{
        "id": "speaker-a",
        "label": "Speaker A",
        "embeddings": [{
          "values": [1.0, 0.0],
          "model": {
            "family": "SpeechBrain",
            "name": "spkrec",
            "version": "1",
            "dimensions": 2
          },
          "sample_rate": 16000
        }],
        "metadata": {
          "note": "fixture"
        }
      }"#,
        r#"{
        "id": "speaker-a",
        "label": "Speaker A",
        "embeddings": [{
          "values": [1.0, 0.0],
          "model": {
            "family": "SpeechBrain",
            "name": "spkrec",
            "version": "1",
            "dimensions": 2
          },
          "sample_rate": 16000
        }],
        "metadata": {
          "note": "fixture"
        }
      },
      {
        "id": "draft-speaker-b",
        "label": "Draft Speaker B",
        "embeddings": [{
          "values": [0.0, 1.0],
          "model": {
            "family": "SpeechBrain",
            "name": "spkrec",
            "version": "1",
            "dimensions": 2
          },
          "sample_rate": 16000
        }],
        "metadata": {
          "status": "draft",
          "detectedLabel": "speaker_1"
        }
      }"#,
    )
}

fn extract_url(line: &str) -> String {
    let start = line.find("http://").expect("line should contain URL");
    line[start..].trim().to_string()
}

fn extract_session_token(html: &str) -> String {
    let marker = r#"window.nativeWhisperxSessionToken = ""#;
    let start = html.find(marker).expect("session token marker") + marker.len();
    let rest = &html[start..];
    let end = rest.find('"').expect("session token end");
    rest[..end].to_string()
}

fn html_asset_path(html: &str, attribute: &str) -> String {
    let start = html.find(attribute).expect("asset attribute") + attribute.len();
    let rest = &html[start..];
    let end = rest.find('"').expect("asset attribute end");
    rest[..end].to_string()
}

fn http_request(method: &str, url: &str) -> (u16, String) {
    http_request_with_body(method, url, &[], None)
}

fn http_request_with_body(
    method: &str,
    url: &str,
    headers: &[(&str, &str)],
    body: Option<&str>,
) -> (u16, String) {
    let (status, _headers, body) = http_response(method, url, headers, body);
    (status, body)
}

fn http_response(
    method: &str,
    url: &str,
    headers: &[(&str, &str)],
    body: Option<&str>,
) -> (u16, String, String) {
    let without_scheme = url.strip_prefix("http://").expect("http URL");
    let (address, path) = match without_scheme.split_once('/') {
        Some((address, path)) => (address, format!("/{path}")),
        None => (without_scheme, "/".to_string()),
    };
    let mut stream = TcpStream::connect(address).expect("connect to server");
    write!(
        stream,
        "{method} {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n"
    )
    .expect("request");
    for (name, value) in headers {
        write!(stream, "{name}: {value}\r\n").expect("header");
    }
    if let Some(body) = body {
        write!(
            stream,
            "Content-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        )
        .expect("body");
    } else {
        write!(stream, "\r\n").expect("headers end");
    }

    let mut response = String::new();
    stream.read_to_string(&mut response).expect("read response");
    let (head, body) = response.split_once("\r\n\r\n").expect("http response");
    let status = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .expect("status code")
        .parse::<u16>()
        .expect("numeric status");
    (status, head.to_string(), body.to_string())
}

#[cfg(unix)]
fn fake_ffmpeg_script(temp: &Path, seconds: f64, exit_code: i32) -> PathBuf {
    let pcm = temp.join("fake-live.pcm");
    fs::write(&pcm, f32le_silence(seconds)).expect("fake PCM");
    let script = temp.join("fake-ffmpeg.sh");
    fs::write(
        &script,
        format!("#!/bin/sh\ncat '{}'\nexit {exit_code}\n", pcm.display()),
    )
    .expect("fake ffmpeg script");
    let mut permissions = fs::metadata(&script)
        .expect("script metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&script, permissions).expect("script executable");
    script
}

#[cfg(unix)]
fn f32le_silence(seconds: f64) -> Vec<u8> {
    let sample_count = (seconds * 16_000.0).round() as usize;
    (0..sample_count)
        .flat_map(|_| 0.0_f32.to_le_bytes())
        .collect()
}

fn jsonl_events(output: &[u8]) -> Vec<serde_json::Value> {
    std::str::from_utf8(output)
        .expect("utf8 jsonl")
        .lines()
        .map(|line| serde_json::from_str(line).expect("json event"))
        .collect()
}

struct ChildGuard(Child);

impl ChildGuard {
    fn stop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        self.stop();
    }
}

use assert_cmd::Command;
use serde_json::Value;
use std::path::{Path, PathBuf};

#[test]
#[ignore = "requires SMOKE_ROOT with real audio and a cached Whisper model"]
fn native_asr_hf_cache_only_uses_cached_model() {
    let smoke_root = smoke_root();
    let audio = smoke_root.join("audio/native-transcription-smoke.wav");
    let model_dir = smoke_root.join("models");
    assert!(
        audio.is_file(),
        "required smoke audio is missing: {}",
        audio.display()
    );
    assert!(
        model_dir.is_dir(),
        "required smoke model cache root is missing: {}",
        model_dir.display()
    );

    let output_dir = tempfile::tempdir().expect("temp output dir");
    let output = native_asr_cache_command(&audio, &model_dir, output_dir.path())
        .output()
        .expect("native-whisperx should run");
    assert!(
        output.status.success(),
        "command failed\nstderr:\n{}\nstdout:\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let report: Value = serde_json::from_slice(&output.stdout).expect("stdout should be JSON");
    assert_eq!(
        report.pointer("/response/accepted"),
        Some(&Value::Bool(true)),
        "response.accepted should be true"
    );

    let diagnostics = report
        .pointer("/response/diagnostics")
        .and_then(Value::as_array)
        .expect("response.diagnostics should be an array");
    assert_contains_diagnostic(diagnostics, "asrModelSource=hugging-face-cache");
    assert_contains_diagnostic(diagnostics, "asrModelId=openai/whisper-tiny.en");
    assert!(
        diagnostics
            .iter()
            .filter_map(Value::as_str)
            .any(|entry| entry.starts_with("asrModelResolved=")),
        "diagnostics should include asrModelResolved=..., got {diagnostics:?}"
    );

    let segments = report
        .pointer("/response/transcript/segments")
        .and_then(Value::as_array)
        .expect("response.transcript.segments should be an array");
    assert!(
        !segments.is_empty(),
        "transcript segments should not be empty"
    );

    let transcript_text = report
        .pointer("/response/transcript/text")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let first_segment_text = segments
        .first()
        .and_then(|segment| segment.get("text"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    assert!(
        !transcript_text.trim().is_empty() || !first_segment_text.trim().is_empty(),
        "transcript text or first segment text should be non-empty"
    );

    let output_files = report
        .get("outputFiles")
        .and_then(Value::as_array)
        .expect("outputFiles should be an array");
    let json_output_path = output_files
        .iter()
        .find(|file| file.get("format").and_then(Value::as_str) == Some("json"))
        .and_then(|file| file.get("path"))
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .expect("outputFiles should contain a json output path");
    assert!(
        json_output_path.is_file(),
        "json output path should exist: {}",
        json_output_path.display()
    );
}

#[test]
#[ignore = "requires SMOKE_ROOT with real audio"]
fn native_asr_cache_only_missing_model_reports_required_files() {
    let smoke_root = smoke_root();
    let audio = smoke_root.join("audio/native-transcription-smoke.wav");
    assert!(
        audio.is_file(),
        "required smoke audio is missing: {}",
        audio.display()
    );

    let empty_model_dir = tempfile::tempdir().expect("empty model cache root");
    let output_dir = tempfile::tempdir().expect("temp output dir");
    let output = native_asr_cache_command(&audio, empty_model_dir.path(), output_dir.path())
        .output()
        .expect("native-whisperx should run");
    assert!(
        !output.status.success(),
        "command should fail when cache-only model files are missing\nstdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    for expected in [
        "failed to resolve native Candle Whisper model",
        "openai/whisper-tiny.en",
        "config.json",
        "generation_config.json",
        "tokenizer.json",
        "preprocessor_config.json",
        "model.safetensors",
        "cache-only=true",
    ] {
        assert!(
            stderr.contains(expected),
            "stderr should contain `{expected}`\nstderr:\n{stderr}"
        );
    }
}

fn native_asr_cache_command(audio: &Path, model_dir: &Path, output_dir: &Path) -> Command {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("transcribe")
        .arg(audio)
        .args(["--model", "tiny.en", "--model-dir"])
        .arg(model_dir)
        .args([
            "--model-cache-only",
            "--language",
            "en",
            "--no-align",
            "--format",
            "json",
            "--output-dir",
        ])
        .arg(output_dir);
    command
}

fn smoke_root() -> PathBuf {
    std::env::var_os("SMOKE_ROOT")
        .map(PathBuf::from)
        .expect("SMOKE_ROOT must be set to run native ASR cache smoke tests")
}

fn assert_contains_diagnostic(diagnostics: &[Value], expected: &str) {
    assert!(
        diagnostics
            .iter()
            .filter_map(Value::as_str)
            .any(|entry| entry == expected),
        "diagnostics should contain exactly `{expected}`, got {diagnostics:?}"
    );
}

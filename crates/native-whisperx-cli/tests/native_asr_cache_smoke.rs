use assert_cmd::Command;
use serde_json::Value;
use std::path::{Path, PathBuf};

#[test]
#[ignore = "requires SMOKE_ROOT with real audio and a cached Whisper model"]
fn native_asr_cache_hf_only_uses_cached_model() {
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
#[ignore = "requires SMOKE_ROOT with real audio and a cached Whisper model"]
fn native_asr_cache_active_row_batch_reports_ordered_row_state() {
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
        .args(["--batch-size", "2", "--chunk-size", "1"])
        .output()
        .expect("native-whisperx should run");
    assert!(
        output.status.success(),
        "command failed\nstderr:\n{}\nstdout:\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let report: Value = serde_json::from_slice(&output.stdout).expect("stdout should be JSON");
    let diagnostics = report
        .pointer("/response/diagnostics")
        .and_then(Value::as_array)
        .expect("response.diagnostics should be an array");
    assert_contains_diagnostic(
        diagnostics,
        "batchExecution=candle-whisper-active-row-tensor-batch",
    );
    assert_contains_diagnostic(diagnostics, "activeRowCompaction=true");
    assert_contains_diagnostic(diagnostics, "cacheReuse=self-and-cross-attention");
    assert_numeric_diagnostic_at_least(diagnostics, "completedRowCount", 2);
    assert_numeric_diagnostic_at_least(diagnostics, "effectiveActiveBatchSize", 2);
    assert_numeric_diagnostic_at_least(diagnostics, "activeRowCompactionCount", 1);
    assert_has_diagnostic_key(diagnostics, "effectiveActiveBatchSizes");
    assert_has_diagnostic_key(diagnostics, "timestampTokensRequested");
    assert_has_diagnostic_key(diagnostics, "timestampTokensPresent");
    assert_has_diagnostic_key(diagnostics, "timestampSegmentsRejected");

    let segments = report
        .pointer("/response/transcript/segments")
        .and_then(Value::as_array)
        .expect("response.transcript.segments should be an array");
    assert!(
        segments.len() >= 2,
        "active-row smoke should produce multiple ordered segments, got {segments:?}"
    );
    assert_segments_are_ordered(segments);
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

fn assert_has_diagnostic_key(diagnostics: &[Value], key: &str) {
    let prefix = format!("{key}=");
    assert!(
        diagnostics
            .iter()
            .filter_map(Value::as_str)
            .any(|entry| entry.starts_with(&prefix)),
        "diagnostics should contain `{key}=...`, got {diagnostics:?}"
    );
}

fn assert_numeric_diagnostic_at_least(diagnostics: &[Value], key: &str, minimum: u64) {
    let value = diagnostics
        .iter()
        .filter_map(Value::as_str)
        .find_map(|entry| entry.strip_prefix(&format!("{key}=")))
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or_else(|| {
            panic!("diagnostics should contain numeric `{key}=...`: {diagnostics:?}")
        });
    assert!(
        value >= minimum,
        "`{key}` should be at least {minimum}, got {value}; diagnostics: {diagnostics:?}"
    );
}

fn assert_segments_are_ordered(segments: &[Value]) {
    let mut previous_index = None;
    let mut previous_start = None;
    for segment in segments {
        let index = segment
            .get("index")
            .and_then(Value::as_u64)
            .expect("segment should include an integer index");
        if let Some(previous_index) = previous_index {
            assert_eq!(
                index,
                previous_index + 1,
                "segment indexes should remain contiguous in output order: {segments:?}"
            );
        }
        previous_index = Some(index);

        if let Some(start) = segment.get("startSeconds").and_then(Value::as_f64) {
            if let Some(previous_start) = previous_start {
                assert!(
                    start >= previous_start,
                    "segment starts should be monotonic in output order: {segments:?}"
                );
            }
            previous_start = Some(start);
        }
    }
}

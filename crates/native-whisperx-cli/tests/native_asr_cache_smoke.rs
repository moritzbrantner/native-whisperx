use assert_cmd::Command;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::time::Duration;

const NATIVE_ASR_SMOKE_TIMEOUT: Duration = Duration::from_secs(10 * 60);

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
    let report = run_with_report(native_asr_cache_command(
        &audio,
        &model_dir,
        output_dir.path(),
    ));

    let diagnostics = report
        .get("diagnostics")
        .and_then(Value::as_array)
        .expect("diagnostics should be an array");
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
        .pointer("/transcript/segments")
        .and_then(Value::as_array)
        .expect("transcript.segments should be an array");
    assert!(
        !segments.is_empty(),
        "transcript segments should not be empty"
    );

    let transcript_text = report
        .pointer("/transcript/text")
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
    let mut command = native_asr_cache_command(&audio, &model_dir, output_dir.path());
    command.args(["--batch-size", "2", "--chunk-size", "1"]);
    let report = run_with_report(command);
    let diagnostics = report
        .get("diagnostics")
        .and_then(Value::as_array)
        .expect("diagnostics should be an array");
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
        .pointer("/transcript/segments")
        .and_then(Value::as_array)
        .expect("transcript.segments should be an array");
    assert!(
        segments.len() >= 2,
        "active-row smoke should produce multiple ordered segments, got {segments:?}"
    );
    assert_segments_are_ordered(segments);
}

/// Native WhisperX #418: continuous music makes the energy VAD merge full 30 s
/// chunks. Their context padding used to spill a tail window and start the next
/// chunk 0.25 s early, so unaligned segment starts moved backward and the whole
/// file failed strict validation.
///
/// Audio: first 40 s of "Blender 3.6 LTS - Reel" by Blender
/// (https://www.youtube.com/watch?v=BnY2JdBrvaI), CC BY 3.0. Kept out of Git
/// with the other smoke media; prepare it with
/// `yt-dlp -f bestaudio -x --audio-format wav -o reel.wav https://www.youtube.com/watch?v=BnY2JdBrvaI`
/// then `ffmpeg -t 40 -i reel.wav -ac 1 -ar 16000 -c:a libopus -b:a 24k
/// "$SMOKE_ROOT/audio/blender-3.6-lts-reel-0-40s.opus"`.
#[test]
#[ignore = "requires SMOKE_ROOT with the CC BY music excerpt and a cached Whisper model"]
fn native_asr_music_chunk_boundaries_keep_segments_chronological() {
    let smoke_root = smoke_root();
    let audio = smoke_root.join("audio/blender-3.6-lts-reel-0-40s.opus");
    assert!(
        audio.is_file(),
        "required smoke audio is missing: {}",
        audio.display()
    );
    let output_dir = tempfile::tempdir().expect("temp output dir");
    let report_path = output_dir.path().join("music-report.json");
    let output = native_asr_cache_command(&audio, &smoke_root.join("models"), output_dir.path())
        .arg("--report")
        .arg(&report_path)
        .output()
        .expect("native-whisperx should run");
    assert!(
        output.status.success(),
        "command failed\nstderr:\n{}\nstdout:\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let report: Value =
        serde_json::from_slice(&std::fs::read(&report_path).expect("report should be written"))
            .expect("report should be JSON");
    let segments = report
        .pointer("/transcript/segments")
        .and_then(Value::as_array)
        .expect("transcript.segments should be an array");
    assert_segments_are_ordered(segments);
    for segment in segments {
        let start = segment.get("startSeconds").and_then(Value::as_f64);
        let end = segment.get("endSeconds").and_then(Value::as_f64);
        if let (Some(start), Some(end)) = (start, end) {
            assert!(
                end > start,
                "context-only segments should be dropped: {segments:?}"
            );
        }
    }
}

/// Native WhisperX #434: without `--language` the multilingual model never
/// detected one and decoded punctuation only (",", ".", "....").
///
/// Audio: "Blender Conference 2026 Recap" by Blender
/// (https://www.youtube.com/watch?v=sO_lhW8av3U), CC BY, English speech over
/// music. Kept out of Git with the other smoke media; prepare it with
/// `yt-dlp -f bestaudio -x --audio-format wav -o recap.wav https://www.youtube.com/watch?v=sO_lhW8av3U`
/// then `ffmpeg -i recap.wav -ac 1 -ar 16000 -c:a libopus -b:a 24k
/// "$SMOKE_ROOT/audio/blender-conference-2026-recap.opus"`. Needs
/// `openai/whisper-small` in `$SMOKE_ROOT/models`.
#[test]
#[ignore = "requires SMOKE_ROOT with the CC BY recap and a cached multilingual Whisper-small"]
fn native_asr_without_language_detects_english_under_music() {
    let smoke_root = smoke_root();
    let audio = smoke_root.join("audio/blender-conference-2026-recap.opus");
    assert!(
        audio.is_file(),
        "required smoke audio is missing: {}",
        audio.display()
    );
    let output_dir = tempfile::tempdir().expect("temp output dir");
    let report_path = output_dir.path().join("language-report.json");
    let output = Command::cargo_bin("native-whisperx")
        .expect("binary should build")
        .arg("transcribe")
        .arg(&audio)
        .args(["--model", "small", "--model-dir"])
        .arg(smoke_root.join("models"))
        .args([
            "--model-cache-only",
            "--vad-method",
            "energy",
            "--no-align",
            "--format",
            "json",
            "--output-dir",
        ])
        .arg(output_dir.path())
        .arg("--report")
        .arg(&report_path)
        .timeout(NATIVE_ASR_SMOKE_TIMEOUT)
        .output()
        .expect("native-whisperx should run");
    assert!(
        output.status.success(),
        "command failed\nstderr:\n{}\nstdout:\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let report: Value =
        serde_json::from_slice(&std::fs::read(&report_path).expect("report should be written"))
            .expect("report should be JSON");
    assert_eq!(
        report
            .pointer("/transcript/language")
            .and_then(Value::as_str),
        Some("en")
    );
    let segments = report
        .pointer("/transcript/segments")
        .and_then(Value::as_array)
        .expect("transcript.segments should be an array");
    let text = segments
        .iter()
        .filter_map(|segment| segment.get("text").and_then(Value::as_str))
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        text.contains("Blender"),
        "expected the English recap, got {text:?}"
    );
}

#[test]
#[ignore = "requires SMOKE_ROOT with real audio and a cached Whisper model"]
fn native_asr_positive_threads_report_the_request_scoped_decoder_count() {
    let smoke_root = smoke_root();
    let audio = smoke_root.join("audio/native-transcription-smoke.wav");
    let model_dir = smoke_root.join("models");
    let output_dir = tempfile::tempdir().expect("temp output dir");
    let report_path = output_dir.path().join("thread-report.json");

    let output = native_asr_cache_command(&audio, &model_dir, output_dir.path())
        .args(["--threads", "2", "--report"])
        .arg(&report_path)
        .output()
        .expect("native-whisperx should run");
    assert!(
        output.status.success(),
        "command failed\nstderr:\n{}\nstdout:\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let report: Value = serde_json::from_slice(
        &std::fs::read(&report_path).expect("thread-count report should be written"),
    )
    .expect("thread-count report should be JSON");
    let diagnostics = report
        .get("diagnostics")
        .and_then(Value::as_array)
        .expect("diagnostics should be an array");
    assert_contains_diagnostic(diagnostics, "decoderThreads=2");
}

#[cfg(feature = "cuda")]
#[test]
#[ignore = "requires SMOKE_ROOT, CUDA_SMOKE_DEVICE_INDEX>0, a cached Whisper model, and that CUDA device"]
fn native_asr_cuda_smoke_uses_a_non_default_available_device() {
    let device_index = std::env::var("CUDA_SMOKE_DEVICE_INDEX")
        .expect("CUDA_SMOKE_DEVICE_INDEX must select a non-default available CUDA device")
        .parse::<usize>()
        .expect("CUDA_SMOKE_DEVICE_INDEX must be a non-negative integer");
    assert!(
        device_index > 0,
        "CUDA smoke must exercise a non-default device"
    );

    let smoke_root = smoke_root();
    let audio = smoke_root.join("audio/native-transcription-smoke.wav");
    let model_dir = smoke_root.join("models");
    let output_dir = tempfile::tempdir().expect("temp output dir");
    let report_path = output_dir.path().join("cuda-device-report.json");
    let output = native_asr_cache_command(&audio, &model_dir, output_dir.path())
        .args([
            "--device",
            "cuda",
            "--compute-type",
            "float32",
            "--device-index",
        ])
        .arg(device_index.to_string())
        .arg("--report")
        .arg(&report_path)
        .output()
        .expect("native-whisperx should run on the selected CUDA device");
    assert!(
        output.status.success(),
        "command failed\nstderr:\n{}\nstdout:\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let report: Value = serde_json::from_slice(
        &std::fs::read(&report_path).expect("CUDA device report should be written"),
    )
    .expect("CUDA device report should be JSON");
    let diagnostics = report
        .get("diagnostics")
        .and_then(Value::as_array)
        .expect("diagnostics should be an array");
    assert_contains_diagnostic(diagnostics, &format!("device=cuda:{device_index}"));
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

#[test]
#[ignore = "requires NATIVE_WHISPERX_Q8_BUNDLE and NATIVE_WHISPERX_Q8_WAV"]
fn native_q8_cpu_cli_real_resource_smoke() {
    let bundle = std::env::var_os("NATIVE_WHISPERX_Q8_BUNDLE")
        .map(PathBuf::from)
        .expect("NATIVE_WHISPERX_Q8_BUNDLE must point to a caller-owned Q8 bundle");
    let audio = std::env::var_os("NATIVE_WHISPERX_Q8_WAV")
        .map(PathBuf::from)
        .expect("NATIVE_WHISPERX_Q8_WAV must point to a caller-owned WAV");
    assert!(
        bundle.is_dir(),
        "Q8 bundle is missing: {}",
        bundle.display()
    );
    assert!(audio.is_file(), "Q8 WAV is missing: {}", audio.display());

    let run_dir = tempfile::tempdir().expect("Q8 smoke run dir");
    let output_dir = run_dir.path().join("outputs");
    std::fs::create_dir(&output_dir).expect("Q8 smoke output dir");
    let report_path = run_dir.path().join("report.json");
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    let output = command
        .arg("transcribe")
        .arg(&audio)
        .args(["--provider", "native", "--device", "cpu"])
        .args(["--compute-type", "int8", "--whisper-bundle"])
        .arg(&bundle)
        .args(["--no-align", "--format", "json", "--report"])
        .arg(&report_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .output()
        .expect("native Q8 CLI smoke should run");
    assert!(
        output.status.success(),
        "Q8 command failed\nstderr:\n{}\nstdout:\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let report: Value =
        serde_json::from_slice(&std::fs::read(&report_path).expect("Q8 report should be written"))
            .expect("Q8 report JSON");
    assert!(report
        .pointer("/transcript/segments")
        .is_some_and(Value::is_array));
    assert!(report.get("outputFiles").is_some_and(Value::is_array));

    let diagnostics = report
        .get("diagnostics")
        .and_then(Value::as_array)
        .expect("Q8 diagnostics");
    for expected in [
        "provider=candle-whisper",
        "requestedComputeType=int8",
        "resolvedComputeType=int8",
        "computeType=int8",
        "modelFormat=gguf-q8_0",
        "asrModelSource=explicit-bundle",
    ] {
        assert_contains_diagnostic(diagnostics, expected);
    }
    for key in [
        "cacheReuse",
        "generatedTokenCount",
        "phaseTiming.encoderSeconds",
        "phaseTiming.decoderSeconds",
        "phaseTiming.asrSeconds",
        "phaseAsrSeconds",
        "phaseAsrModelLoadSeconds",
    ] {
        assert_has_diagnostic_key(diagnostics, key);
    }
}

/// Runs `command` with `--report` and returns the parsed report. Progress
/// goes to stderr, so stdout must stay empty.
fn run_with_report(mut command: Command) -> Value {
    let report_dir = tempfile::tempdir().expect("temp report dir");
    let report_path = report_dir.path().join("report.json");
    let output = command
        .arg("--report")
        .arg(&report_path)
        .output()
        .expect("native-whisperx should run");
    assert!(
        output.status.success(),
        "command failed\nstderr:\n{}\nstdout:\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        output.stdout.is_empty(),
        "stdout should stay free of progress lines:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
    serde_json::from_slice(&std::fs::read(&report_path).expect("report should be written"))
        .expect("report should be JSON")
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
    command.timeout(NATIVE_ASR_SMOKE_TIMEOUT);
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

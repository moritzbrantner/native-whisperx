use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

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
        .stdout(predicate::str::contains("models/wav2vec2"));
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
        "--no-align",
        "--align-model",
        "--model-dir",
        "--model-cache-only",
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
    ] {
        assert!(help.contains(expected), "help should contain `{expected}`");
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
            "--task translate is not supported with native alignment yet",
        ));
}

#[test]
fn transcribe_rejects_speaker_embeddings_without_diarize() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--speaker_embeddings"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("requires --diarize"));
}

#[test]
fn transcribe_rejects_native_pyannote_before_audio_io() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["input.wav", "--vad_method", "pyannote"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("pyannote"));
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

#[test]
fn transcribe_rejects_basename_with_multiple_inputs() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["one.wav", "two.wav", "--basename", "fixed"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("multiple input files"));
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

#[cfg(unix)]
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
        "--vad_method\nsilero",
        "--vad_onset\n0.5",
        "--vad_offset\n0.363",
        "--chunk_size\n20",
        "--beam_size\n5",
        "--diarize",
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

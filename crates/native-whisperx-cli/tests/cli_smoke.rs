use assert_cmd::Command;
use predicates::prelude::*;
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
fn transcribe_help_lists_native_json_format() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["transcribe", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("native-json"))
        .stdout(predicate::str::contains("output_format"));
}

#[test]
fn transcribe_help_lists_alignment_parity_flags() {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .args(["transcribe", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--no-align"))
        .stdout(predicate::str::contains("--align-model"))
        .stdout(predicate::str::contains("--model-dir"))
        .stdout(predicate::str::contains("--model-cache-only"))
        .stdout(predicate::str::contains("--interpolate-method"))
        .stdout(predicate::str::contains("--return-char-alignments"));
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

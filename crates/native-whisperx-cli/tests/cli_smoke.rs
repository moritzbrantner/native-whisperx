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
    assert!(help.contains("parity-fixtures"));
    assert!(help.contains("parity-preflight"));
    assert!(help.contains("parity-goldens"));
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
    ] {
        assert!(help.contains(expected), "help should contain `{expected}`");
    }
}

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

#[test]
fn parity_fixtures_case_timeout_reports_bounded_failure() {
    let temp = tempfile::tempdir().expect("tempdir");
    let manifest = temp.path().join("fixtures.json");
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
        8
    );
    assert_eq!(
        parsed
            .fixtures
            .iter()
            .filter(|fixture| !fixture.gating)
            .count(),
        4
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
            && !fixture.expected_outputs.is_empty()
    }));
    assert!(parsed.fixtures.iter().any(|fixture| {
        fixture.name == "tiny-output-subtitles-highlight"
            && !fixture.gating
            && fixture.expected_outputs.len() == 2
    }));
    assert!(parsed
        .fixtures
        .iter()
        .any(|fixture| fixture.name == "small-de-translate-cache"
            && fixture.translation.enabled
            && fixture.translation.model_cache_only));
}

#[test]
fn checked_in_full_resource_fixture_manifest_parses() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/parity/full-resource-fixtures.json");
    let bytes = fs::read(&fixture).expect("fixture manifest");
    let parsed: native_whisperx::ParityFixtureSuite =
        serde_json::from_slice(&bytes).expect("valid manifest schema");
    assert_eq!(parsed.fixtures.len(), 3);
    assert!(parsed.fixtures.iter().all(|fixture| !fixture.gating));
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
            "--task translate is not supported by the published native provider yet",
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
            "native provider does not produce WhisperX-compatible speaker embeddings",
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
            "pyannote diarization models require --provider external-whisperx",
        ));
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

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
    assert!(help.contains("parity-bench"));
    assert!(help.contains("parity-summary"));
    assert!(help.contains("parity-preflight"));
    assert!(help.contains("parity-goldens"));
}

#[test]
fn parity_fixtures_workflow_exposes_final_full_surface_gate_without_performance_gate() {
    let workflow =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/parity-fixtures.yml");
    let workflow = fs::read_to_string(workflow).expect("workflow should exist");

    assert!(workflow.contains("- final-full-surface"));
    assert!(workflow.contains("manifest=\"tests/parity/full-resource-fixtures.json\""));
    assert!(workflow.contains("fixture_args+=(\"--require-non-gating-passed\")"));
    assert!(!workflow.contains("Run Rust-Native benchmark ladder"));
    assert!(!workflow.contains("nativeFasterThanWhisperx"));
    assert!(!workflow.contains("benchmark report passed="));
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
        "--json",
    ] {
        assert!(help.contains(expected), "help should contain `{expected}`");
    }
}

#[test]
fn parity_bench_json_empty_manifest_has_stable_top_level_shape() {
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
        .success()
        .stdout(predicate::str::contains("\"passed\": true"))
        .stdout(predicate::str::contains("\"iterations\": 1"))
        .stdout(predicate::str::contains("\"warmups\": 1"))
        .stdout(predicate::str::contains("\"nativeOnly\": true"))
        .stdout(predicate::str::contains("\"caseTimeoutSeconds\": 900"))
        .stdout(predicate::str::contains("\"cases\": []"));
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
        .success()
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
        .success()
        .stdout(predicate::str::contains("\"passed\": false"))
        .stdout(predicate::str::contains("\"name\": \"missing-audio\""))
        .stdout(predicate::str::contains("\"error\""));
}

#[test]
#[ignore = "requires SMOKE_ROOT with Shrek-derived 30s audio, cached large-v3-turbo CUDA assets, and Silero VAD"]
fn parity_bench_rust_native_ladder_30s_smoke_emits_json() {
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
        .success()
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

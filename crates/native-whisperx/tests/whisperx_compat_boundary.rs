use std::path::{Path, PathBuf};

use native_whisperx::{
    run, AlignmentConfig, AsrConfig, AsrProvider, DiarizationConfig, ExternalWhisperxConfig,
    InputSource, NativeWhisperxConfig, OutputConfig, TranslationConfig, VadConfig,
};
#[cfg(not(feature = "whisperx-compat"))]
use native_whisperx::{run_parity_preflight, ParityFixtureSuite};

#[test]
fn external_provider_configuration_remains_serializable_without_runtime_compatibility() {
    let config: AsrConfig = serde_json::from_str(r#"{"provider":"externalWhisperX"}"#)
        .expect("external provider configuration should deserialize");

    assert_eq!(config.provider, AsrProvider::ExternalWhisperX);
    assert_eq!(
        serde_json::to_value(config)
            .expect("external provider configuration should serialize")
            .get("provider")
            .and_then(serde_json::Value::as_str),
        Some("externalWhisperX")
    );
}

#[cfg(not(feature = "whisperx-compat"))]
#[test]
fn external_provider_fails_with_explicit_feature_disabled_error() {
    let error = run(external_config(
        PathBuf::from("whisperx-must-not-run"),
        PathBuf::from("input.wav"),
        PathBuf::from("output"),
        None,
    ))
    .expect_err("feature-disabled external execution should fail");

    let message = error.to_string();
    assert!(message.contains("external WhisperX provider"), "{message}");
    assert!(message.contains("whisperx-compat"), "{message}");
    assert!(message.contains("feature is disabled"), "{message}");
}

#[cfg(all(unix, not(feature = "whisperx-compat")))]
#[test]
fn parity_preflight_reports_disabled_feature_without_spawning_oracle() {
    let temp = tempfile::tempdir().expect("tempdir");
    let command = temp.path().join("whisperx");
    let marker = temp.path().join("spawned");
    write_executable(
        &command,
        r#"#!/usr/bin/env sh
set -eu
touch "$NATIVE_WHISPERX_PREFLIGHT_MARKER"
"#,
    );
    let suite: ParityFixtureSuite =
        serde_json::from_str(r#"{"fixtures":[{"name":"compatibility","input":"input.wav"}]}"#)
            .expect("fixture suite");

    let report = with_env_var("NATIVE_WHISPERX_PREFLIGHT_MARKER", &marker, || {
        run_parity_preflight(
            suite,
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            command,
            temp.path().join("models"),
            false,
            false,
        )
    });

    assert!(!report.passed);
    assert!(report.cases[0]
        .missing
        .iter()
        .any(|message| message.contains("whisperx-compat") && message.contains("disabled")));
    assert!(
        !marker.exists(),
        "preflight must not spawn a disabled oracle"
    );
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn fake_whisperx_proves_arguments_json_import_and_diagnostics() {
    let temp = tempfile::tempdir().expect("tempdir");
    let command = temp.path().join("whisperx");
    let argv = temp.path().join("argv.txt");
    let output_dir = temp.path().join("whisperx-output");
    let report_dir = temp.path().join("native-output");
    std::fs::write(temp.path().join("input.wav"), b"fake audio").expect("input");
    write_executable(
        &command,
        r#"#!/usr/bin/env sh
set -eu
printf '%s\n' "$@" > "$NATIVE_WHISPERX_TEST_ARGV"
out=""
prev=""
for arg in "$@"; do
  if [ "$prev" = "--output_dir" ]; then out="$arg"; fi
  prev="$arg"
done
mkdir -p "$out"
cat > "$out/input.json" <<'JSON'
{"language":"en","segments":[{"id":0,"start":0.0,"end":1.0,"text":"compatibility transcript","words":[]}]}
JSON
"#,
    );

    let report = with_env_var("NATIVE_WHISPERX_TEST_ARGV", &argv, || {
        run(external_config(
            command,
            temp.path().join("input.wav"),
            output_dir,
            None,
        ))
    })
    .expect("feature-enabled fake WhisperX should run");

    assert_eq!(
        report.response.transcript.segments[0].text,
        "compatibility transcript"
    );
    let captured = std::fs::read_to_string(argv).expect("captured argv");
    assert!(captured.contains("--model\nsmall"), "{captured}");
    assert!(captured.contains("--language\nen"), "{captured}");
    assert!(captured.contains("--batch_size\n8"), "{captured}");
    assert!(captured.contains("--output_format\njson"), "{captured}");
    assert!(report
        .response
        .diagnostics
        .iter()
        .any(|entry| entry.contains("ran WhisperX output")));
    assert!(report
        .response
        .diagnostics
        .contains(&"parsed WhisperX JSON through text-transcripts".to_string()));
    assert!(report_dir.exists());
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn fake_whisperx_timeout_is_bounded_and_reported() {
    let temp = tempfile::tempdir().expect("tempdir");
    let command = temp.path().join("whisperx");
    write_executable(
        &command,
        r#"#!/usr/bin/env sh
set -eu
sleep 5
"#,
    );

    let error = run(external_config(
        command,
        temp.path().join("input.wav"),
        temp.path().join("whisperx-output"),
        Some(1),
    ))
    .expect_err("slow fake WhisperX should time out");

    let message = error.to_string();
    assert!(message.contains("timed out after 1 seconds"), "{message}");
}

#[cfg(unix)]
#[test]
fn native_provider_never_spawns_configured_whisperx_command() {
    let temp = tempfile::tempdir().expect("tempdir");
    let command = temp.path().join("whisperx");
    let marker = temp.path().join("spawned");
    write_executable(
        &command,
        r#"#!/usr/bin/env sh
set -eu
touch "$NATIVE_WHISPERX_NATIVE_MARKER"
"#,
    );
    let mut config = external_config(
        command,
        temp.path().join("missing.wav"),
        temp.path().join("whisperx-output"),
        None,
    );
    config.asr.provider = AsrProvider::Native;

    let _ = with_env_var("NATIVE_WHISPERX_NATIVE_MARKER", &marker, || run(config));

    assert!(
        !marker.exists(),
        "native provider must not spawn Python WhisperX"
    );
}

fn external_config(
    command: PathBuf,
    input: PathBuf,
    whisperx_output: PathBuf,
    timeout_seconds: Option<u64>,
) -> NativeWhisperxConfig {
    let native_output = whisperx_output.with_file_name("native-output");
    NativeWhisperxConfig {
        input: InputSource::Path { path: input },
        asr: AsrConfig {
            provider: AsrProvider::ExternalWhisperX,
            language: Some("en".to_string()),
            max_batch_size: Some(8),
            external_whisperx: ExternalWhisperxConfig {
                command,
                output_dir: Some(whisperx_output),
                timeout_seconds,
                ..ExternalWhisperxConfig::default()
            },
            ..AsrConfig::default()
        },
        translation: TranslationConfig::default(),
        vad: VadConfig::default(),
        alignment: AlignmentConfig {
            enabled: false,
            ..AlignmentConfig::default()
        },
        diarization: DiarizationConfig::default(),
        output: OutputConfig {
            output_dir: Some(native_output),
            ..OutputConfig::default()
        },
    }
}

#[cfg(unix)]
fn write_executable(path: &Path, contents: &str) {
    use std::os::unix::fs::PermissionsExt;

    std::fs::write(path, contents).expect("write executable");
    let mut permissions = std::fs::metadata(path).expect("metadata").permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).expect("chmod executable");
}

fn with_env_var<T>(name: &str, value: &Path, run: impl FnOnce() -> T) -> T {
    let previous = std::env::var_os(name);
    std::env::set_var(name, value);
    let result = run();
    match previous {
        Some(previous) => std::env::set_var(name, previous),
        None => std::env::remove_var(name),
    }
    result
}

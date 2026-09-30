use std::path::{Path, PathBuf};

use native_whisperx::{
    run, run_whisperx_oracle, AlignmentConfig, AsrConfig, AsrProvider, DiarizationConfig,
    InputSource, NativeWhisperxConfig, OutputConfig, TranslationConfig, VadConfig,
    WhisperxOracleConfig,
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

#[test]
fn product_workflow_rejects_retired_python_provider() {
    let error = run(external_config(
        PathBuf::from("whisperx-must-not-run"),
        PathBuf::from("input.wav"),
        PathBuf::from("output"),
        None,
    ))
    .expect_err("normal product execution should reject Python");

    let message = error.to_string();
    assert!(
        message.contains("Python WhisperX product provider is retired"),
        "{message}"
    );
    assert!(message.contains("run_whisperx_oracle"), "{message}");
}

#[test]
fn controlled_selected_and_batch_workflows_reject_python_before_resources() {
    use native_whisperx::{
        run_many, run_many_reusing_native_provider, run_many_selected_media,
        run_many_selected_media_with_control, run_many_with_control, run_selected_media,
        run_selected_media_with_control, run_with_control, CancellationHandle,
        NoopTranscriptionProgressObserver, SelectedMediaInput,
    };
    let config = external_config(
        PathBuf::from("whisperx-must-not-run"),
        PathBuf::from("missing-input.wav"),
        PathBuf::from("must-not-write"),
        None,
    );
    let mut observer = NoopTranscriptionProgressObserver;
    let cancellation = CancellationHandle::new();
    let selected = SelectedMediaInput::new(0);
    let errors = [
        run_many_reusing_native_provider(vec![config.clone()])
            .unwrap_err()
            .to_string(),
        run_selected_media(config.clone(), selected)
            .unwrap_err()
            .to_string(),
        run_with_control(config.clone(), &mut observer, &cancellation)
            .unwrap_err()
            .to_string(),
        run_selected_media_with_control(config.clone(), selected, &mut observer, &cancellation)
            .unwrap_err()
            .to_string(),
        run_many(vec![config.clone()]).unwrap_err().to_string(),
        run_many_selected_media(vec![config.clone()], selected)
            .unwrap_err()
            .to_string(),
        run_many_with_control(vec![config.clone()], &mut observer, &cancellation)
            .unwrap_err()
            .to_string(),
        run_many_selected_media_with_control(vec![config], selected, &mut observer, &cancellation)
            .unwrap_err()
            .to_string(),
    ];
    for error in errors {
        assert!(
            error.contains("Python WhisperX product provider is retired"),
            "{error}"
        );
    }
}

#[cfg(not(feature = "whisperx-compat"))]
#[test]
fn explicit_oracle_requires_non_default_compatibility_feature() {
    let error = run_whisperx_oracle(external_config(
        PathBuf::from("whisperx-must-not-run"),
        PathBuf::from("missing-input.wav"),
        PathBuf::from("must-not-write"),
        None,
    ))
    .unwrap_err()
    .to_string();
    assert!(error.contains("Python WhisperX parity oracle"), "{error}");
    assert!(error.contains("feature is disabled"), "{error}");
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
        let mut config = external_config(command, temp.path().join("input.wav"), output_dir, None);
        config.asr.provider = AsrProvider::Native;
        run_whisperx_oracle(config)
    })
    .expect("feature-enabled fake WhisperX should run");

    assert_eq!(
        report.transcript.segments[0].text,
        "compatibility transcript"
    );
    let captured = std::fs::read_to_string(argv).expect("captured argv");
    assert!(captured.contains("--model\nsmall"), "{captured}");
    assert!(captured.contains("--language\nen"), "{captured}");
    assert!(captured.contains("--batch_size\n8"), "{captured}");
    assert!(captured.contains("--output_format\njson"), "{captured}");
    assert!(report
        .diagnostics
        .iter()
        .any(|entry| entry.contains("ran WhisperX output")));
    assert!(report
        .diagnostics
        .contains(&"parsed WhisperX JSON in the transcription provider adapter".to_string()));
    let serialized = serde_json::to_value(&report).expect("report should serialize");
    assert!(serialized.get("response").is_none());
    assert_eq!(serialized["provenance"]["provider"], "whisperx-command");
    assert_eq!(serialized["provenance"]["modelId"], "small");
    assert_eq!(serialized["transcript"]["language"], "en");
    assert!(report_dir.exists());
}

#[cfg(all(unix, feature = "whisperx-compat"))]
#[test]
fn legacy_external_path_is_delegated_without_native_predecode() {
    let temp = tempfile::tempdir().expect("tempdir");
    let command = temp.path().join("whisperx");
    let input = temp.path().join("missing.wav");
    write_executable(
        &command,
        r#"#!/usr/bin/env sh
set -eu
out=""
prev=""
for arg in "$@"; do
  if [ "$prev" = "--output_dir" ]; then out="$arg"; fi
  prev="$arg"
done
mkdir -p "$out"
cat > "$out/missing.json" <<'JSON'
{"language":"en","segments":[{"id":0,"start":0.0,"end":1.0,"text":"delegated legacy path","words":[]}]}
JSON
"#,
    );

    let report = run_whisperx_oracle(external_config(
        command,
        input,
        temp.path().join("whisperx-output"),
        None,
    ))
    .expect("legacy external paths should reach delegated WhisperX unchanged");

    assert_eq!(report.transcript.segments[0].text, "delegated legacy path");
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

    let error = run_whisperx_oracle(external_config(
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
            external_whisperx: WhisperxOracleConfig {
                command,
                output_dir: Some(whisperx_output),
                timeout_seconds,
                ..WhisperxOracleConfig::default()
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

    let staging = path.with_extension("staging");
    std::fs::write(&staging, contents).expect("write executable");
    let mut permissions = std::fs::metadata(&staging).expect("metadata").permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&staging, permissions).expect("chmod executable");
    std::fs::rename(staging, path).expect("publish executable");
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

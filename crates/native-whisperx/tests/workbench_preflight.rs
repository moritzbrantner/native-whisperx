#[cfg(not(feature = "native"))]
#[test]
fn preflight_reports_disabled_native_feature_without_resolving_resources() {
    use native_whisperx::{preflight_workbench, NativeWhisperxConfig};
    let config: NativeWhisperxConfig = serde_json::from_value(serde_json::json!({
        "input":{"kind":"path","path":"recording.wav"}
    }))
    .unwrap();
    let report = preflight_workbench(&config);
    assert!(!report.ready);
    assert_eq!(report.checks.len(), 1);
    assert!(report.checks[0].message.contains("native feature"));
}

#[cfg(all(feature = "native", not(feature = "cuda")))]
#[test]
fn preflight_never_substitutes_cpu_for_explicit_cuda() {
    use native_whisperx::{preflight_workbench, NativeWhisperxConfig};
    let config: NativeWhisperxConfig = serde_json::from_value(serde_json::json!({
        "input":{"kind":"path","path":"recording.wav"},"asr":{"device":"cuda"}
    }))
    .unwrap();
    let report = preflight_workbench(&config);
    assert!(!report.ready);
    assert!(report
        .checks
        .iter()
        .any(|check| !check.available && check.message.contains("cuda feature")));
}

#[cfg(not(feature = "native"))]
#[test]
fn saved_settings_remain_readable_when_this_build_cannot_execute_them() {
    let preset = native_whisperx::WorkbenchPreset::new(
        "Reusable transcript",
        native_whisperx::WorkbenchSettings::default(),
    )
    .unwrap();
    let encoded = serde_json::to_string(&preset).unwrap();
    let restored: native_whisperx::WorkbenchPreset = serde_json::from_str(&encoded).unwrap();
    restored.validate().unwrap();
    assert!(
        native_whisperx::prepare_workbench(restored.settings.for_input("recording.wav")).is_err()
    );
}

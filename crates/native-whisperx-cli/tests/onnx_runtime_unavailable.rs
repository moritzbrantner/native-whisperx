//! A missing or too-old ONNX Runtime must fail ONNX VAD runs with the typed
//! "ONNX Runtime unavailable" error instead of hanging (#436, vox#159).
#![cfg(any(feature = "silero-vad", feature = "pyannote-vad"))]

use assert_cmd::Command;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// The system ONNX Runtime on Ubuntu 26.04 is 1.23, older than ort rc.15 requires.
const SYSTEM_ORT_1_23: &str = "/usr/lib/x86_64-linux-gnu/libonnxruntime.so.1.23.2";

struct Fixture {
    dir: tempfile::TempDir,
}

impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        write_silence_wav(&dir.path().join("in.wav"));
        for (bundle, model) in [
            ("silero", "silero_vad.onnx"),
            ("pyannote", "segmentation.onnx"),
        ] {
            std::fs::create_dir_all(dir.path().join(bundle)).expect("bundle dir");
            std::fs::write(dir.path().join(bundle).join(model), b"not a model")
                .expect("model file");
        }
        Self { dir }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.path().join(name)
    }

    fn vad_args(&self, method: &str) -> Vec<String> {
        vec![
            self.path("in.wav").display().to_string(),
            "--vad-method".to_string(),
            method.to_string(),
            "--vad-model-bundle".to_string(),
            self.path(method).display().to_string(),
            "--model-cache-only".to_string(),
        ]
    }

    fn command(&self, ort_dylib: &Path) -> Command {
        let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
        command
            .current_dir(self.dir.path())
            .env("ORT_DYLIB_PATH", ort_dylib)
            .timeout(Duration::from_secs(60));
        command
    }

    fn assert_transcribe_reports(&self, method: &str, ort_dylib: &Path, reason: &str) {
        let output = self
            .command(ort_dylib)
            .arg("transcribe")
            .args(self.vad_args(method))
            .arg("--output-dir")
            .arg(self.path("out"))
            .output()
            .expect("run transcribe");
        assert_eq!(output.status.code(), Some(1), "{method}: {output:?}");
        assert_unavailable(&String::from_utf8_lossy(&output.stderr), reason);
    }

    fn assert_serve_reports(&self, method: &str, ort_dylib: &Path, reason: &str) {
        let request = serde_json::json!({ "id": 7, "args": self.vad_args(method) });
        let output = self
            .command(ort_dylib)
            .arg("serve")
            .write_stdin(format!("{request}\n"))
            .output()
            .expect("run serve");
        assert!(output.status.success(), "{method}: {output:?}");
        let answer: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("one JSON answer line");
        assert_eq!(answer["id"], 7);
        assert_unavailable(answer["error"].as_str().expect("error answer"), reason);
    }
}

fn assert_unavailable(message: &str, reason: &str) {
    assert!(message.contains("ONNX Runtime unavailable"), "{message}");
    assert!(message.contains(reason), "{message}");
    assert!(message.contains("set ORT_DYLIB_PATH"), "{message}");
}

fn write_silence_wav(path: &Path) {
    let samples = 16_000u32;
    let mut bytes = Vec::with_capacity(44 + samples as usize * 2);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + samples * 2).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&16_000u32.to_le_bytes());
    bytes.extend_from_slice(&32_000u32.to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&(samples * 2).to_le_bytes());
    bytes.resize(44 + samples as usize * 2, 0);
    std::fs::write(path, bytes).expect("write wav");
}

fn methods() -> Vec<&'static str> {
    let mut methods = Vec::new();
    if cfg!(feature = "silero-vad") {
        methods.push("silero");
    }
    if cfg!(feature = "pyannote-vad") {
        methods.push("pyannote");
    }
    methods
}

#[test]
fn missing_onnx_runtime_fails_transcribe_and_serve_instead_of_hanging() {
    let fixture = Fixture::new();
    let missing = fixture.path("libonnxruntime-missing.so");
    for method in methods() {
        fixture.assert_transcribe_reports(method, &missing, "cannot load");
        fixture.assert_serve_reports(method, &missing, "cannot load");
    }
}

#[test]
fn onnx_runtime_1_23_fails_transcribe_and_serve_instead_of_hanging() {
    let old = Path::new(SYSTEM_ORT_1_23);
    if !old.is_file() {
        eprintln!("skipping: {SYSTEM_ORT_1_23} is not installed");
        return;
    }
    let fixture = Fixture::new();
    for method in methods() {
        fixture.assert_transcribe_reports(method, old, "version 1.23.2");
        fixture.assert_serve_reports(method, old, "version 1.23.2");
    }
}

//! `TranscriptionSession` with Silero VAD and no ONNX Runtime returns the typed
//! "ONNX Runtime unavailable" error instead of hanging (#436, vox#159).
//! Its own test binary: ONNX Runtime is loaded once per process.
#![cfg(feature = "silero-vad")]

use native_whisperx::{
    AlignmentConfig, AsrConfig, DiarizationConfig, InputSource, NativeWhisperxConfig, OutputConfig,
    TranscriptionSession, TranslationConfig, VadConfig, VadMethod,
};

#[test]
fn session_without_onnx_runtime_reports_unavailable_and_keeps_serving() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("in.wav");
    std::fs::write(&input, b"RIFF").expect("input file");
    std::fs::write(dir.path().join("silero_vad.onnx"), b"not a model").expect("model file");
    std::env::set_var(
        "ORT_DYLIB_PATH",
        dir.path().join("libonnxruntime-missing.so"),
    );

    let config = NativeWhisperxConfig {
        input: InputSource::Path { path: input },
        asr: AsrConfig {
            model_cache_only: true,
            ..AsrConfig::default()
        },
        vad: VadConfig {
            method: VadMethod::Silero,
            model_bundle: Some(dir.path().to_path_buf()),
            ..VadConfig::default()
        },
        output: OutputConfig {
            output_dir: Some(dir.path().join("out")),
            ..OutputConfig::default()
        },
        translation: TranslationConfig::default(),
        alignment: AlignmentConfig::default(),
        diarization: DiarizationConfig::default(),
    };

    let mut session = TranscriptionSession::new();
    // A second run on the same session must fail the same way, not deadlock.
    for _ in 0..2 {
        let error = session
            .run(config.clone())
            .expect_err("Silero must fail without an ONNX Runtime dylib");
        let message = error.to_string();
        assert!(message.contains("ONNX Runtime unavailable"), "{message}");
        assert!(message.contains("ORT_DYLIB_PATH"), "{message}");
    }
}

use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn bundle_verify_emits_sanitized_json_for_a_valid_pyannote_vad_bundle() {
    let bundle = tempfile::tempdir().expect("bundle directory");
    fs::write(bundle.path().join("segmentation.onnx"), b"model").expect("model");
    fs::write(bundle.path().join("MODEL_PROVENANCE.md"), b"provenance").expect("provenance");
    fs::write(
        bundle.path().join("pyannote_vad_manifest.json"),
        r#"{
          "schemaVersion": 1,
          "kind": "pyannote-vad",
          "source": {
            "modelId": "pyannote/segmentation-3.0",
            "revision": "e66f3d3b9eb0873085418a7b813d3b369bf160bb",
            "license": "MIT"
          },
          "conversion": {
            "command": "python scripts/convert_pyannote_segmentation.py",
            "python": "3.12.0",
            "packages": {"torch": "2.8.0", "pyannote.audio": "3.0.0"},
            "onnxOpset": 17,
            "inputHashes": {
              "pytorch_model.bin": "da85c29829d4002daedd676e012936488234d9255e65e86dfab9bec6b1729298",
              "config.yaml": "fa65a47a751602f04cc570135007d76859b69e8f9f1bfdf5878a5145980d4263",
              "README.md": "a37bc19811cc1a52a4c128c33207813b1558b4e49b050b03e814d0a96d14f05d",
              "LICENSE": "63a777ad4b3c7aed4b260b084d8fb49ec781c46c70c6b599ca5d2402ef7ebe50"
            }
          },
          "tensorContract": {
            "inputName": "waveform",
            "inputShape": [1, 1, 160000],
            "outputName": "scores",
            "sampleRate": 16000,
            "windowSeconds": 10.0,
            "frameCount": 589,
            "localSpeakerCount": 3
          },
          "numericalComparison": {"tolerance": 0.0001, "fixtureSeed": 217, "maxAbsoluteDifference": 0.00001},
          "files": {
            "segmentation.onnx": "9372c470eeadd5ecd9c3c74c2b3cb633f8e2f2fad799250a0f70d652b6b825e4",
            "MODEL_PROVENANCE.md": "96d815328a42cb4ef89d5e0b7a1df6be43b484832c83a7b4596d8402c7c0b12b"
          }
        }"#,
    )
    .expect("manifest");

    Command::cargo_bin("native-whisperx")
        .expect("CLI binary")
        .args([
            "bundle-verify",
            "--kind",
            "pyannote-vad",
            "--bundle",
            bundle.path().to_str().expect("UTF-8 path"),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"kind\": \"pyannote-vad\""))
        .stdout(predicate::str::contains("\"sourceModelId\""));
}

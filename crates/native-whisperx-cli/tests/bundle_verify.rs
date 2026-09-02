use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn bundle_verify_emits_sanitized_json_for_a_valid_pyannote_vad_bundle() {
    let bundle = tempfile::tempdir().expect("bundle directory");
    let model = pyannote_model();
    fs::write(bundle.path().join("segmentation.onnx"), &model).expect("model");
    fs::write(
        bundle.path().join("pyannote_vad_manifest.json"),
        r#"{
          "schemaVersion": 1,
          "kind": "pyannote-vad",
          "source": {
            "modelId": "pyannote/segmentation-3.0",
            "revision": "e66f3d3b9eb0873085418a7b813d3b369bf160bb"
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
          "files": {
            "segmentation.onnx": "1b3f6a15a90401b4add70bbd4bcd3c1709a313a70573707617515b5d286a0fcf"
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
        .stdout(predicate::str::contains("\"inputName\": \"waveform\""));

    Command::cargo_bin("native-whisperx")
        .expect("CLI binary")
        .args([
            "inspect-models",
            "--vad-model-bundle",
            bundle.path().to_str().expect("UTF-8 path"),
            "--no-align",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"pyannoteVadBundleInspection\""))
        .stdout(predicate::str::contains("\"inputName\": \"waveform\""));
}

fn pyannote_model() -> Vec<u8> {
    let input = value_info("waveform", &[1, 1, 160_000]);
    let output = value_info("scores", &[1, 589, 3]);
    let mut graph = len_field(11, input);
    graph.extend(len_field(12, output));
    len_field(7, graph)
}

fn value_info(name: &str, dimensions: &[u64]) -> Vec<u8> {
    let mut shape = Vec::new();
    for dimension in dimensions {
        shape.extend(len_field(1, varint_field(1, *dimension)));
    }
    let mut tensor_type = varint_field(1, 1);
    tensor_type.extend(len_field(2, shape));
    let mut value = len_field(1, name.as_bytes().to_vec());
    value.extend(len_field(2, len_field(1, tensor_type)));
    value
}

fn len_field(field: u64, value: Vec<u8>) -> Vec<u8> {
    let mut bytes = varint((field << 3) | 2);
    bytes.extend(varint(value.len() as u64));
    bytes.extend(value);
    bytes
}

fn varint_field(field: u64, value: u64) -> Vec<u8> {
    let mut bytes = varint(field << 3);
    bytes.extend(varint(value));
    bytes
}

fn varint(mut value: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        bytes.push(byte);
        if value == 0 {
            return bytes;
        }
    }
}

#[test]
fn bundle_verify_rejects_partial_and_malformed_pyannote_diarization_bundles_offline() {
    let bundle = tempfile::tempdir().expect("bundle directory");
    fs::write(
        bundle.path().join("pyannote_diarization_manifest.json"),
        r#"{
          "schemaVersion": 1,
          "kind": "pyannote-diarization",
          "source": {
            "modelId": "pyannote/speaker-diarization-community-1",
            "revision": "3533c8cf8e369892e6b79ff1bf80f7b0286a54ee",
            "license": "CC-BY-4.0"
          },
          "conversion": {
            "command": "python scripts/convert_pyannote_community.py",
            "python": "3.11.15",
            "packages": {"torch": "2.8.0"},
            "onnxOpset": 17,
            "inputHashes": {
              "config.yaml": "5ce2bfa9a938dc132cec1172592d65173cbb8f444ea1e4133f10f9391de155be",
              "README.md": "2db91f9265bd81f1653ff088b5bff22bf6aebebea03328513af65501643f8a31",
              "segmentation/pytorch_model.bin": "7ad24338d844fb95985486eb1a464e32d229f6d7a03c9abe60f978bacf3f816e",
              "embedding/pytorch_model.bin": "6f10ff60898a1d185fa22e1d11e0bfa8a92efec811f11bca48cb8cafebefd929",
              "embedding/README.md": "fa9e5105ae95edb231d841476cdb91eef4be0621c372ed4f7d3421294b5f8ad7",
              "plda/plda.npz": "9b77bcd840692710dd3496f62ecfeed8d8e5f002fd991b785079b244eab7d255",
              "plda/xvec_transform.npz": "325f1ce8e48f7e55e9c8aa47e05d2766b7c48c4b25b8de8dd751e7a4cc5fbe8f",
              "plda/README.md": "e1316dbbeb3261431478d48ceebbd4bba395c3587e7b80c254dbab00f1209d0a"
            }
          },
          "modelId": "pyannote/speaker-diarization-community-1",
          "sampleRate": 16000,
          "labelFormat": "SPEAKER_{:02}",
          "segmentation": {
            "inputName": "waveform", "outputName": "segmentations",
            "durationSeconds": 10.0, "stepRatio": 0.1, "powerset": true,
            "frames": 589, "localSpeakers": 3
          },
          "embedding": {
            "waveformInputName": "waveform", "maskInputName": "masks",
            "outputName": "embeddings", "dimension": 256, "maskFrames": 589
          },
          "clustering": {
            "kind": "vbx", "threshold": 0.6, "fa": 0.07, "fb": 0.8,
            "maxIters": 20, "minActiveRatio": 0.2, "constrainedAssignment": true
          },
          "numericalComparison": {
            "tolerance": 0.0001, "fixtureSeed": 218,
            "segmentationMaxAbsoluteDifference": 0.00001,
            "embeddingMaxAbsoluteDifference": 0.00001
          },
          "endToEndComparison": {
            "fixtureSha256": "6b8ec683ab0bf8aa931e3fe2d31b53f47427384692452b4f2542eb9a2e76da90",
            "requestedSpeakers": 2,
            "assignedSpeakers": 2,
            "turnCount": 4,
            "embeddingsFinite": true
          },
          "artifactSetSha256": "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
          "files": {}
        }"#,
    )
    .expect("manifest");

    Command::cargo_bin("native-whisperx")
        .expect("CLI binary")
        .args([
            "bundle-verify",
            "--kind",
            "pyannote-diarization",
            "--bundle",
            bundle.path().to_str().expect("UTF-8 path"),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("segmentation model"));

    let manifest_path = bundle.path().join("pyannote_diarization_manifest.json");
    let mut manifest: serde_json::Value = serde_json::from_slice(
        &fs::read(&manifest_path).expect("read partial diarization manifest"),
    )
    .expect("parse partial diarization manifest");
    manifest["files"]["segmentation.onnx"] = serde_json::json!("not-a-sha256");
    fs::write(
        manifest_path,
        serde_json::to_vec(&manifest).expect("serialize malformed diarization manifest"),
    )
    .expect("write malformed diarization manifest");

    Command::cargo_bin("native-whisperx")
        .expect("CLI binary")
        .args([
            "bundle-verify",
            "--kind",
            "pyannote-diarization",
            "--bundle",
            bundle.path().to_str().expect("UTF-8 path"),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "invalid local pyannote diarization bundle",
        ));
}

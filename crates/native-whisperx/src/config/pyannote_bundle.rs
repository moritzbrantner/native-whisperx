//! Offline verification for immutable native pyannote VAD bundles.

use std::{collections::BTreeMap, fs, path::Path};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::NativeWhisperxError;

pub const PYANNOTE_SEGMENTATION_MODEL_ID: &str = "pyannote/segmentation-3.0";
pub const PYANNOTE_SEGMENTATION_REVISION: &str = "e66f3d3b9eb0873085418a7b813d3b369bf160bb";

const MANIFEST_FILE: &str = "pyannote_vad_manifest.json";
const MODEL_FILE: &str = "segmentation.onnx";
const PROVENANCE_FILE: &str = "MODEL_PROVENANCE.md";

/// Sanitized result from validating a caller-provided native pyannote VAD bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PyannoteVadBundleVerification {
    pub kind: &'static str,
    pub source_model_id: String,
    pub source_revision: String,
    pub verified_files: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Manifest {
    schema_version: u32,
    kind: String,
    source: Source,
    conversion: Conversion,
    tensor_contract: TensorContract,
    numerical_comparison: NumericalComparison,
    files: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Source {
    model_id: String,
    revision: String,
    license: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Conversion {
    command: String,
    python: String,
    packages: BTreeMap<String, String>,
    onnx_opset: u32,
    input_hashes: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TensorContract {
    input_name: String,
    input_shape: [usize; 3],
    output_name: String,
    sample_rate: u32,
    window_seconds: f64,
    frame_count: usize,
    local_speaker_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NumericalComparison {
    tolerance: f64,
    fixture_seed: u64,
    max_absolute_difference: f64,
}

/// Validate a local pyannote VAD bundle without opening a network connection or model weights.
pub fn verify_pyannote_vad_bundle(
    bundle: &Path,
) -> Result<PyannoteVadBundleVerification, NativeWhisperxError> {
    let manifest_path = bundle.join(MANIFEST_FILE);
    let manifest: Manifest =
        serde_json::from_slice(&fs::read(&manifest_path).map_err(|error| {
            invalid_bundle(
                bundle,
                format!("missing or unreadable {MANIFEST_FILE}: {error}"),
            )
        })?)
        .map_err(|error| invalid_bundle(bundle, format!("invalid {MANIFEST_FILE}: {error}")))?;

    validate_manifest(bundle, &manifest)?;
    let mut verified_files = Vec::with_capacity(2);
    for file in [MODEL_FILE, PROVENANCE_FILE] {
        let expected_hash = manifest.files.get(file).ok_or_else(|| {
            invalid_bundle(
                bundle,
                format!("manifest does not checksum required file `{file}`"),
            )
        })?;
        validate_sha256(bundle, file, expected_hash)?;
        let actual_hash = sha256_file(&bundle.join(file))
            .map_err(|error| invalid_bundle(bundle, format!("could not hash `{file}`: {error}")))?;
        if actual_hash != *expected_hash {
            return Err(invalid_bundle(
                bundle,
                format!("checksum mismatch for `{file}`"),
            ));
        }
        verified_files.push(file.to_string());
    }

    Ok(PyannoteVadBundleVerification {
        kind: "pyannote-vad",
        source_model_id: manifest.source.model_id,
        source_revision: manifest.source.revision,
        verified_files,
    })
}

fn validate_manifest(bundle: &Path, manifest: &Manifest) -> Result<(), NativeWhisperxError> {
    if manifest.schema_version != 1 || manifest.kind != "pyannote-vad" {
        return Err(invalid_bundle(
            bundle,
            "manifest must declare schemaVersion 1 and kind `pyannote-vad`".to_string(),
        ));
    }
    if manifest.source.model_id != PYANNOTE_SEGMENTATION_MODEL_ID
        || manifest.source.revision != PYANNOTE_SEGMENTATION_REVISION
        || manifest.source.license != "MIT"
    {
        return Err(invalid_bundle(
            bundle,
            "manifest source model, revision, or license does not match the pinned contract"
                .to_string(),
        ));
    }
    let conversion = &manifest.conversion;
    if conversion.command.trim().is_empty()
        || conversion.python.trim().is_empty()
        || conversion.packages.is_empty()
        || conversion.onnx_opset == 0
        || !["pytorch_model.bin", "config.yaml", "README.md", "LICENSE"]
            .iter()
            .all(|file| {
                conversion
                    .input_hashes
                    .get(*file)
                    .is_some_and(|hash| is_sha256(hash))
            })
    {
        return Err(invalid_bundle(
            bundle,
            "manifest conversion provenance is incomplete".to_string(),
        ));
    }
    let tensor = &manifest.tensor_contract;
    if tensor.input_name != "waveform"
        || tensor.input_shape != [1, 1, 160_000]
        || tensor.output_name != "scores"
        || tensor.sample_rate != 16_000
        || tensor.window_seconds != 10.0
        || tensor.frame_count == 0
        || tensor.local_speaker_count != 3
    {
        return Err(invalid_bundle(
            bundle,
            "manifest tensor contract is incompatible with native pyannote VAD".to_string(),
        ));
    }
    let comparison = &manifest.numerical_comparison;
    if !comparison.tolerance.is_finite()
        || comparison.tolerance <= 0.0
        || comparison.fixture_seed == 0
        || !comparison.max_absolute_difference.is_finite()
        || comparison.max_absolute_difference > comparison.tolerance
    {
        return Err(invalid_bundle(
            bundle,
            "manifest numerical comparison is incomplete or exceeds its tolerance".to_string(),
        ));
    }
    Ok(())
}

fn validate_sha256(bundle: &Path, file: &str, hash: &str) -> Result<(), NativeWhisperxError> {
    if is_sha256(hash) {
        Ok(())
    } else {
        Err(invalid_bundle(
            bundle,
            format!("manifest checksum for `{file}` is not a SHA-256 digest"),
        ))
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn sha256_file(path: &Path) -> Result<String, std::io::Error> {
    Ok(format!("{:x}", Sha256::digest(fs::read(path)?)))
}

fn invalid_bundle(bundle: &Path, reason: String) -> NativeWhisperxError {
    NativeWhisperxError::InvalidConfig(format!(
        "invalid local pyannote VAD bundle at `{}`: {reason}",
        bundle.display()
    ))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn verifies_complete_bundle_and_rejects_corrupt_model() {
        let bundle = tempfile::tempdir().expect("bundle directory");
        write_bundle(bundle.path());
        verify_pyannote_vad_bundle(bundle.path()).expect("valid bundle");

        fs::write(bundle.path().join(MODEL_FILE), b"corrupt").expect("corrupt model");
        let error = verify_pyannote_vad_bundle(bundle.path())
            .expect_err("corrupt bundle should fail")
            .to_string();
        assert!(error.contains("checksum mismatch"));
    }

    #[test]
    fn rejects_wrong_pinned_revision_and_incomplete_provenance() {
        let bundle = tempfile::tempdir().expect("bundle directory");
        write_bundle(bundle.path());
        let manifest_path = bundle.path().join(MANIFEST_FILE);
        let mut manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest")).expect("JSON");
        manifest["source"]["revision"] = serde_json::json!("wrong");
        fs::write(&manifest_path, serde_json::to_vec(&manifest).expect("JSON")).expect("manifest");

        let error = verify_pyannote_vad_bundle(bundle.path())
            .expect_err("wrong source revision should fail")
            .to_string();
        assert!(error.contains("pinned contract"));

        write_bundle(bundle.path());
        let mut manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest")).expect("JSON");
        manifest["numericalComparison"]
            .as_object_mut()
            .expect("comparison")
            .remove("maxAbsoluteDifference");
        fs::write(&manifest_path, serde_json::to_vec(&manifest).expect("JSON")).expect("manifest");
        let error = verify_pyannote_vad_bundle(bundle.path())
            .expect_err("missing numerical result should fail")
            .to_string();
        assert!(error.contains("invalid pyannote_vad_manifest.json"));
    }

    #[test]
    fn rejects_missing_model_and_incomplete_tensor_contract() {
        let bundle = tempfile::tempdir().expect("bundle directory");
        write_bundle(bundle.path());
        fs::remove_file(bundle.path().join(MODEL_FILE)).expect("remove model");

        let error = verify_pyannote_vad_bundle(bundle.path())
            .expect_err("missing model should fail")
            .to_string();
        assert!(error.contains("could not hash `segmentation.onnx`"));

        write_bundle(bundle.path());
        let manifest_path = bundle.path().join(MANIFEST_FILE);
        let mut manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest")).expect("JSON");
        manifest["tensorContract"]["sampleRate"] = serde_json::json!(8_000);
        fs::write(&manifest_path, serde_json::to_vec(&manifest).expect("JSON")).expect("manifest");

        let error = verify_pyannote_vad_bundle(bundle.path())
            .expect_err("incomplete tensor contract should fail")
            .to_string();
        assert!(error.contains("tensor contract"));
    }

    fn write_bundle(bundle: &Path) {
        fs::write(bundle.join(MODEL_FILE), b"model").expect("model");
        fs::write(bundle.join(PROVENANCE_FILE), b"provenance").expect("provenance");
        let manifest = serde_json::json!({
            "schemaVersion": 1,
            "kind": "pyannote-vad",
            "source": {
                "modelId": PYANNOTE_SEGMENTATION_MODEL_ID,
                "revision": PYANNOTE_SEGMENTATION_REVISION,
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
                MODEL_FILE: sha256_file(&bundle.join(MODEL_FILE)).expect("model hash"),
                PROVENANCE_FILE: sha256_file(&bundle.join(PROVENANCE_FILE)).expect("provenance hash")
            }
        });
        fs::write(
            bundle.join(MANIFEST_FILE),
            serde_json::to_vec_pretty(&manifest).expect("JSON"),
        )
        .expect("manifest");
    }
}

//! Offline verification for immutable native pyannote VAD bundles.

use std::{collections::BTreeMap, fs, path::Path};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::NativeWhisperxError;

pub const PYANNOTE_SEGMENTATION_MODEL_ID: &str = "pyannote/segmentation-3.0";
pub const PYANNOTE_SEGMENTATION_REVISION: &str = "e66f3d3b9eb0873085418a7b813d3b369bf160bb";
pub const PYANNOTE_COMMUNITY_MODEL_ID: &str = "pyannote/speaker-diarization-community-1";
pub const PYANNOTE_COMMUNITY_REVISION: &str = "3533c8cf8e369892e6b79ff1bf80f7b0286a54ee";

const MANIFEST_FILE: &str = "pyannote_vad_manifest.json";
const MODEL_FILE: &str = "segmentation.onnx";
const PROVENANCE_FILE: &str = "MODEL_PROVENANCE.md";
const DIARIZATION_MANIFEST_FILE: &str = "pyannote_diarization_manifest.json";
const LICENSE_FILE: &str = "LICENSE.md";
const DIARIZATION_FILES: [&str; 7] = [
    "segmentation.onnx",
    "embedding.onnx",
    "plda_transform.json",
    "plda_model.json",
    "clustering.json",
    PROVENANCE_FILE,
    LICENSE_FILE,
];
const COMMUNITY_SOURCE_HASHES: [(&str, &str); 8] = [
    (
        "config.yaml",
        "5ce2bfa9a938dc132cec1172592d65173cbb8f444ea1e4133f10f9391de155be",
    ),
    (
        "README.md",
        "2db91f9265bd81f1653ff088b5bff22bf6aebebea03328513af65501643f8a31",
    ),
    (
        "segmentation/pytorch_model.bin",
        "7ad24338d844fb95985486eb1a464e32d229f6d7a03c9abe60f978bacf3f816e",
    ),
    (
        "embedding/pytorch_model.bin",
        "6f10ff60898a1d185fa22e1d11e0bfa8a92efec811f11bca48cb8cafebefd929",
    ),
    (
        "embedding/README.md",
        "fa9e5105ae95edb231d841476cdb91eef4be0621c372ed4f7d3421294b5f8ad7",
    ),
    (
        "plda/plda.npz",
        "9b77bcd840692710dd3496f62ecfeed8d8e5f002fd991b785079b244eab7d255",
    ),
    (
        "plda/xvec_transform.npz",
        "325f1ce8e48f7e55e9c8aa47e05d2766b7c48c4b25b8de8dd751e7a4cc5fbe8f",
    ),
    (
        "plda/README.md",
        "e1316dbbeb3261431478d48ceebbd4bba395c3587e7b80c254dbab00f1209d0a",
    ),
];

/// Sanitized result from validating a caller-provided native pyannote VAD bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PyannoteVadBundleVerification {
    pub kind: &'static str,
    pub source_model_id: String,
    pub source_revision: String,
    pub verified_files: Vec<String>,
}

/// Sanitized result from validating a caller-provided community diarization bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PyannoteDiarizationBundleVerification {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiarizationManifest {
    schema_version: u32,
    kind: String,
    source: Source,
    conversion: Conversion,
    model_id: String,
    sample_rate: u32,
    label_format: String,
    segmentation: DiarizationSegmentation,
    embedding: DiarizationEmbedding,
    clustering: VbxClustering,
    numerical_comparison: DiarizationNumericalComparison,
    end_to_end_comparison: DiarizationEndToEndComparison,
    artifact_set_sha256: String,
    files: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiarizationSegmentation {
    input_name: String,
    output_name: String,
    duration_seconds: f64,
    step_ratio: f64,
    powerset: bool,
    frames: usize,
    local_speakers: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiarizationEmbedding {
    waveform_input_name: String,
    mask_input_name: String,
    output_name: String,
    dimension: usize,
    mask_frames: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VbxClustering {
    kind: String,
    threshold: f64,
    fa: f64,
    fb: f64,
    max_iters: usize,
    min_active_ratio: f64,
    constrained_assignment: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiarizationNumericalComparison {
    tolerance: f64,
    fixture_seed: u64,
    segmentation_max_absolute_difference: f64,
    embedding_max_absolute_difference: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiarizationEndToEndComparison {
    fixture_sha256: String,
    requested_speakers: usize,
    assigned_speakers: usize,
    turn_count: usize,
    embeddings_finite: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PldaTransform {
    schema_version: u32,
    input_dimension: usize,
    output_dimension: usize,
    mean1: Vec<f64>,
    mean2: Vec<f64>,
    lda: Vec<Vec<f64>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PldaModel {
    schema_version: u32,
    dimension: usize,
    mean: Vec<f64>,
    transform: Vec<Vec<f64>>,
    psi: Vec<f64>,
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

/// Validate a local community diarization bundle without network access.
pub fn verify_pyannote_diarization_bundle(
    bundle: &Path,
) -> Result<PyannoteDiarizationBundleVerification, NativeWhisperxError> {
    let manifest_path = bundle.join(DIARIZATION_MANIFEST_FILE);
    let manifest: DiarizationManifest =
        serde_json::from_slice(&fs::read(&manifest_path).map_err(|error| {
            invalid_diarization_bundle(
                bundle,
                format!("missing or unreadable {DIARIZATION_MANIFEST_FILE}: {error}"),
            )
        })?)
        .map_err(|error| {
            invalid_diarization_bundle(
                bundle,
                format!("invalid {DIARIZATION_MANIFEST_FILE}: {error}"),
            )
        })?;

    validate_diarization_manifest(bundle, &manifest)?;
    let mut verified_files = Vec::with_capacity(DIARIZATION_FILES.len());
    for file in DIARIZATION_FILES {
        let expected_hash = manifest.files.get(file).ok_or_else(|| {
            invalid_diarization_bundle(
                bundle,
                format!("manifest does not checksum required file `{file}`"),
            )
        })?;
        validate_sha256(bundle, file, expected_hash)?;
        let actual_hash = sha256_file(&bundle.join(file)).map_err(|error| {
            invalid_diarization_bundle(bundle, format!("could not hash `{file}`: {error}"))
        })?;
        if actual_hash != *expected_hash {
            return Err(invalid_diarization_bundle(
                bundle,
                format!("checksum mismatch for `{file}`"),
            ));
        }
        verified_files.push(file.to_string());
    }
    let artifact_set_sha256 = format!(
        "{:x}",
        Sha256::digest(
            serde_json::to_vec(&manifest.files)
                .map_err(|error| invalid_diarization_bundle(bundle, error.to_string()))?
        )
    );
    if manifest.artifact_set_sha256 != artifact_set_sha256 {
        return Err(invalid_diarization_bundle(
            bundle,
            "artifactSetSha256 does not match the checksummed artifact set".to_string(),
        ));
    }
    if bundle
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            name.starts_with("sha256-") && name != format!("sha256-{artifact_set_sha256}")
        })
    {
        return Err(invalid_diarization_bundle(
            bundle,
            "checksum-addressed snapshot name does not match artifactSetSha256".to_string(),
        ));
    }

    validate_plda_transform(bundle)?;
    validate_plda_model(bundle)?;
    validate_vbx_file(bundle, &manifest.clustering)?;

    Ok(PyannoteDiarizationBundleVerification {
        kind: "pyannote-diarization",
        source_model_id: manifest.source.model_id,
        source_revision: manifest.source.revision,
        verified_files,
    })
}

fn validate_diarization_manifest(
    bundle: &Path,
    manifest: &DiarizationManifest,
) -> Result<(), NativeWhisperxError> {
    if manifest.schema_version != 1 || manifest.kind != "pyannote-diarization" {
        return Err(invalid_diarization_bundle(
            bundle,
            "manifest must declare schemaVersion 1 and kind `pyannote-diarization`".to_string(),
        ));
    }
    if manifest.source.model_id != PYANNOTE_COMMUNITY_MODEL_ID
        || manifest.source.revision != PYANNOTE_COMMUNITY_REVISION
        || manifest.source.license != "CC-BY-4.0"
        || manifest.model_id != PYANNOTE_COMMUNITY_MODEL_ID
    {
        return Err(invalid_diarization_bundle(
            bundle,
            "manifest source model, revision, or license does not match the pinned contract"
                .to_string(),
        ));
    }
    let conversion = &manifest.conversion;
    if conversion.command.trim().is_empty()
        || conversion.python.trim().is_empty()
        || conversion.packages.is_empty()
        || conversion.onnx_opset < 17
        || !COMMUNITY_SOURCE_HASHES.iter().all(|(file, hash)| {
            conversion
                .input_hashes
                .get(*file)
                .is_some_and(|actual| actual == hash)
        })
    {
        return Err(invalid_diarization_bundle(
            bundle,
            "manifest conversion provenance does not match the pinned source snapshot".to_string(),
        ));
    }
    let segmentation = &manifest.segmentation;
    if manifest.sample_rate != 16_000
        || manifest.label_format != "SPEAKER_{:02}"
        || segmentation.input_name != "waveform"
        || segmentation.output_name != "segmentations"
        || segmentation.duration_seconds != 10.0
        || segmentation.step_ratio != 0.1
        || !segmentation.powerset
        || segmentation.frames != 589
        || segmentation.local_speakers != 3
    {
        return Err(invalid_diarization_bundle(
            bundle,
            "manifest segmentation tensor contract is incompatible with native community diarization"
                .to_string(),
        ));
    }
    let embedding = &manifest.embedding;
    if embedding.waveform_input_name != "waveform"
        || embedding.mask_input_name != "masks"
        || embedding.output_name != "embeddings"
        || embedding.dimension != 256
        || embedding.mask_frames != 589
    {
        return Err(invalid_diarization_bundle(
            bundle,
            "manifest embedding tensor contract is incompatible with native community diarization"
                .to_string(),
        ));
    }
    validate_vbx(bundle, &manifest.clustering)?;
    let comparison = &manifest.numerical_comparison;
    if !comparison.tolerance.is_finite()
        || comparison.tolerance <= 0.0
        || comparison.fixture_seed != 218
        || !comparison.segmentation_max_absolute_difference.is_finite()
        || comparison.segmentation_max_absolute_difference > comparison.tolerance
        || !comparison.embedding_max_absolute_difference.is_finite()
        || comparison.embedding_max_absolute_difference > comparison.tolerance
    {
        return Err(invalid_diarization_bundle(
            bundle,
            "manifest numerical comparison is incomplete or exceeds its tolerance".to_string(),
        ));
    }
    let end_to_end = &manifest.end_to_end_comparison;
    if !is_sha256(&end_to_end.fixture_sha256)
        || end_to_end.requested_speakers != 2
        || end_to_end.assigned_speakers != 2
        || end_to_end.turn_count == 0
        || !end_to_end.embeddings_finite
    {
        return Err(invalid_diarization_bundle(
            bundle,
            "manifest end-to-end comparison does not prove a complete two-speaker assignment"
                .to_string(),
        ));
    }
    Ok(())
}

fn validate_plda_transform(bundle: &Path) -> Result<(), NativeWhisperxError> {
    let value: PldaTransform = read_diarization_json(bundle, "plda_transform.json")?;
    if value.schema_version != 1
        || value.input_dimension != 256
        || value.output_dimension != 128
        || value.mean1.len() != value.input_dimension
        || value.mean2.len() != value.output_dimension
        || !matrix_has_shape(&value.lda, value.input_dimension, value.output_dimension)
        || !all_finite(&value.mean1)
        || !all_finite(&value.mean2)
        || !value.lda.iter().all(|row| all_finite(row))
    {
        return Err(invalid_diarization_bundle(
            bundle,
            "PLDA transform dimensions or values are incompatible with the pinned VBx contract"
                .to_string(),
        ));
    }
    Ok(())
}

fn validate_plda_model(bundle: &Path) -> Result<(), NativeWhisperxError> {
    let value: PldaModel = read_diarization_json(bundle, "plda_model.json")?;
    if value.schema_version != 1
        || value.dimension != 128
        || value.mean.len() != value.dimension
        || value.psi.len() != value.dimension
        || !matrix_has_shape(&value.transform, value.dimension, value.dimension)
        || !all_finite(&value.mean)
        || !all_finite(&value.psi)
        || value.psi.iter().any(|value| *value <= 0.0)
        || !value.transform.iter().all(|row| all_finite(row))
    {
        return Err(invalid_diarization_bundle(
            bundle,
            "PLDA model dimensions or values are incompatible with the pinned VBx contract"
                .to_string(),
        ));
    }
    Ok(())
}

fn validate_vbx_file(
    bundle: &Path,
    manifest_value: &VbxClustering,
) -> Result<(), NativeWhisperxError> {
    let file_value: VbxClustering = read_diarization_json(bundle, "clustering.json")?;
    validate_vbx(bundle, &file_value)?;
    if &file_value != manifest_value {
        return Err(invalid_diarization_bundle(
            bundle,
            "VBx clustering configuration differs between manifest and clustering.json".to_string(),
        ));
    }
    Ok(())
}

fn validate_vbx(bundle: &Path, value: &VbxClustering) -> Result<(), NativeWhisperxError> {
    if value.kind != "vbx"
        || value.threshold != 0.6
        || value.fa != 0.07
        || value.fb != 0.8
        || value.max_iters != 20
        || value.min_active_ratio != 0.2
        || !value.constrained_assignment
    {
        return Err(invalid_diarization_bundle(
            bundle,
            "VBx clustering configuration does not match the pinned community pipeline".to_string(),
        ));
    }
    Ok(())
}

fn read_diarization_json<T: for<'de> Deserialize<'de>>(
    bundle: &Path,
    file: &str,
) -> Result<T, NativeWhisperxError> {
    serde_json::from_slice(&fs::read(bundle.join(file)).map_err(|error| {
        invalid_diarization_bundle(bundle, format!("missing or unreadable `{file}`: {error}"))
    })?)
    .map_err(|error| invalid_diarization_bundle(bundle, format!("invalid `{file}`: {error}")))
}

fn matrix_has_shape(matrix: &[Vec<f64>], rows: usize, columns: usize) -> bool {
    matrix.len() == rows && matrix.iter().all(|row| row.len() == columns)
}

fn all_finite(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite())
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

fn invalid_diarization_bundle(bundle: &Path, reason: String) -> NativeWhisperxError {
    NativeWhisperxError::InvalidConfig(format!(
        "invalid local pyannote diarization bundle at `{}`: {reason}",
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

    #[test]
    fn verifies_complete_community_diarization_bundle() {
        let bundle = tempfile::tempdir().expect("bundle directory");
        write_diarization_bundle(bundle.path());

        let report =
            verify_pyannote_diarization_bundle(bundle.path()).expect("valid diarization bundle");

        assert_eq!(report.kind, "pyannote-diarization");
        assert_eq!(report.source_model_id, PYANNOTE_COMMUNITY_MODEL_ID);
        assert_eq!(report.source_revision, PYANNOTE_COMMUNITY_REVISION);
        assert_eq!(
            report.verified_files,
            [
                "segmentation.onnx",
                "embedding.onnx",
                "plda_transform.json",
                "plda_model.json",
                "clustering.json",
                "MODEL_PROVENANCE.md",
                "LICENSE.md",
            ]
        );
    }

    #[test]
    fn rejects_partial_corrupt_and_revision_mismatched_diarization_bundles() {
        let bundle = tempfile::tempdir().expect("bundle directory");
        write_diarization_bundle(bundle.path());
        fs::remove_file(bundle.path().join("embedding.onnx")).expect("remove embedding");
        let error = verify_pyannote_diarization_bundle(bundle.path())
            .expect_err("partial bundle should fail")
            .to_string();
        assert!(error.contains("embedding.onnx"), "{error}");

        write_diarization_bundle(bundle.path());
        fs::write(bundle.path().join("plda_model.json"), b"corrupt").expect("corrupt PLDA");
        let error = verify_pyannote_diarization_bundle(bundle.path())
            .expect_err("corrupt bundle should fail")
            .to_string();
        assert!(error.contains("checksum mismatch for `plda_model.json`"));

        write_diarization_bundle(bundle.path());
        let manifest_path = bundle.path().join(DIARIZATION_MANIFEST_FILE);
        let mut manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest")).expect("JSON");
        manifest["source"]["revision"] = serde_json::json!("interrupted");
        fs::write(&manifest_path, serde_json::to_vec(&manifest).expect("JSON")).expect("manifest");
        let error = verify_pyannote_diarization_bundle(bundle.path())
            .expect_err("wrong revision should fail")
            .to_string();
        assert!(error.contains("pinned contract"), "{error}");

        write_diarization_bundle(bundle.path());
        let mut manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest")).expect("JSON");
        manifest["artifactSetSha256"] = serde_json::json!("0".repeat(64));
        fs::write(&manifest_path, serde_json::to_vec(&manifest).expect("JSON")).expect("manifest");
        let error = verify_pyannote_diarization_bundle(bundle.path())
            .expect_err("wrong artifact-set address should fail")
            .to_string();
        assert!(error.contains("artifactSetSha256"), "{error}");
    }

    #[test]
    fn rejects_invalid_diarization_tensor_and_vbx_contracts() {
        let bundle = tempfile::tempdir().expect("bundle directory");
        write_diarization_bundle(bundle.path());
        let manifest_path = bundle.path().join(DIARIZATION_MANIFEST_FILE);
        let mut manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest")).expect("JSON");
        manifest["embedding"]["dimension"] = serde_json::json!(128);
        fs::write(&manifest_path, serde_json::to_vec(&manifest).expect("JSON")).expect("manifest");
        let error = verify_pyannote_diarization_bundle(bundle.path())
            .expect_err("incompatible embedding contract should fail")
            .to_string();
        assert!(error.contains("embedding tensor contract"), "{error}");

        write_diarization_bundle(bundle.path());
        fs::write(
            bundle.path().join("clustering.json"),
            br#"{"kind":"vbx","threshold":0.6,"fa":0.07,"fb":0.8,"maxIters":0,"minActiveRatio":0.2,"constrainedAssignment":true}"#,
        )
        .expect("invalid clustering config");
        rewrite_diarization_file_hash(bundle.path(), "clustering.json");
        let error = verify_pyannote_diarization_bundle(bundle.path())
            .expect_err("invalid VBx config should fail")
            .to_string();
        assert!(error.contains("VBx clustering configuration"), "{error}");
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

    fn write_diarization_bundle(bundle: &Path) {
        let plda_transform = serde_json::json!({
            "schemaVersion": 1,
            "inputDimension": 256,
            "outputDimension": 128,
            "mean1": vec![0.0; 256],
            "mean2": vec![0.0; 128],
            "lda": vec![vec![0.0; 128]; 256],
        });
        let plda_model = serde_json::json!({
            "schemaVersion": 1,
            "dimension": 128,
            "mean": vec![0.0; 128],
            "transform": vec![vec![0.0; 128]; 128],
            "psi": vec![1.0; 128],
        });
        let plda_transform = serde_json::to_vec(&plda_transform).expect("PLDA transform JSON");
        let plda_model = serde_json::to_vec(&plda_model).expect("PLDA model JSON");
        let runtime_files = [
            ("segmentation.onnx", br#"segmentation"#.as_slice()),
            ("embedding.onnx", br#"embedding"#.as_slice()),
            ("plda_transform.json", plda_transform.as_slice()),
            ("plda_model.json", plda_model.as_slice()),
            (
                "clustering.json",
                br#"{"kind":"vbx","threshold":0.6,"fa":0.07,"fb":0.8,"maxIters":20,"minActiveRatio":0.2,"constrainedAssignment":true}"#
                    .as_slice(),
            ),
            ("MODEL_PROVENANCE.md", br#"provenance"#.as_slice()),
            ("LICENSE.md", br#"CC-BY-4.0"#.as_slice()),
        ];
        for (name, bytes) in &runtime_files {
            fs::write(bundle.join(name), bytes).expect("bundle file");
        }
        let files = runtime_files
            .iter()
            .map(|(name, _)| (*name, sha256_file(&bundle.join(name)).expect("bundle hash")))
            .collect::<BTreeMap<_, _>>();
        let artifact_set_sha256 = format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&files).expect("files JSON"))
        );
        let manifest = serde_json::json!({
            "schemaVersion": 1,
            "kind": "pyannote-diarization",
            "source": {
                "modelId": PYANNOTE_COMMUNITY_MODEL_ID,
                "revision": PYANNOTE_COMMUNITY_REVISION,
                "license": "CC-BY-4.0"
            },
            "conversion": {
                "command": "python scripts/convert_pyannote_community.py",
                "python": "3.11.15",
                "packages": {"torch": "2.8.0", "pyannote.audio": "4.0.4", "onnx": "1.22.0", "onnxruntime": "1.27.0"},
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
            "modelId": PYANNOTE_COMMUNITY_MODEL_ID,
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
            "artifactSetSha256": artifact_set_sha256,
            "files": files
        });
        fs::write(
            bundle.join(DIARIZATION_MANIFEST_FILE),
            serde_json::to_vec_pretty(&manifest).expect("JSON"),
        )
        .expect("manifest");
    }

    fn rewrite_diarization_file_hash(bundle: &Path, file: &str) {
        let manifest_path = bundle.join(DIARIZATION_MANIFEST_FILE);
        let mut manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest")).expect("JSON");
        manifest["files"][file] =
            serde_json::json!(sha256_file(&bundle.join(file)).expect("file hash"));
        manifest["artifactSetSha256"] = serde_json::json!(format!(
            "{:x}",
            Sha256::digest(
                serde_json::to_vec(&manifest["files"]).expect("artifact checksums JSON")
            )
        ));
        fs::write(&manifest_path, serde_json::to_vec(&manifest).expect("JSON")).expect("manifest");
    }
}

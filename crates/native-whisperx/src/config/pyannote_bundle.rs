//! Product-facing adapters for provider-owned pyannote bundle inspection.

use std::path::{Path, PathBuf};

use serde::Serialize;

use super::NativeWhisperxError;

#[cfg(feature = "pyannote-vad")]
const DEFAULT_SEGMENTATION_MODEL_FILE: &str = "segmentation.onnx";

/// Product-facing result from provider-owned pyannote VAD bundle inspection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PyannoteVadBundleVerification {
    pub kind: &'static str,
    pub model_path: PathBuf,
    pub manifest_path: PathBuf,
    pub input_name: String,
    pub output_name: Option<String>,
    pub window_samples: usize,
    pub supported_model_ids: Vec<String>,
}

/// Product-facing result from provider-owned pyannote diarization bundle inspection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PyannoteDiarizationBundleVerification {
    pub kind: &'static str,
    pub bundle_path: PathBuf,
    pub manifest_path: PathBuf,
    pub model_id: String,
    pub source_revision: String,
    pub artifact_set_sha256: String,
    pub required_files: Vec<PathBuf>,
}

/// Inspect a local pyannote VAD bundle using the consuming provider's contract.
pub fn verify_pyannote_vad_bundle(
    bundle: &Path,
) -> Result<PyannoteVadBundleVerification, NativeWhisperxError> {
    #[cfg(feature = "pyannote-vad")]
    {
        use audio_analysis_transcription::{inspect_pyannote_vad_bundle, PyannoteVadOptions};

        let report = inspect_pyannote_vad_bundle(&PyannoteVadOptions {
            model_path: bundle.join(DEFAULT_SEGMENTATION_MODEL_FILE),
            input_name: None,
            output_name: None,
            onset: 0.5,
            offset: 0.363,
            chunk_size: 30.0,
        })
        .map_err(|error| invalid_vad_bundle(bundle, error))?;
        Ok(PyannoteVadBundleVerification {
            kind: "pyannote-vad",
            model_path: report.model_path,
            manifest_path: report.manifest_path,
            input_name: report.input_name,
            output_name: report.output_name,
            window_samples: report.window_samples,
            supported_model_ids: report.supported_model_ids,
        })
    }
    #[cfg(not(feature = "pyannote-vad"))]
    {
        let _ = bundle;
        Err(NativeWhisperxError::InvalidConfig(
            "pyannote VAD bundle verification requires the pyannote-vad feature".to_string(),
        ))
    }
}

/// Inspect a local pyannote diarization bundle using the consuming provider's contract.
pub fn verify_pyannote_diarization_bundle(
    bundle: &Path,
) -> Result<PyannoteDiarizationBundleVerification, NativeWhisperxError> {
    #[cfg(feature = "pyannote-diarization")]
    {
        use audio_analysis_speakers::{
            inspect_pyannote_community_diarization_bundle, PyannoteCommunityDiarizationConfig,
        };

        let report =
            inspect_pyannote_community_diarization_bundle(PyannoteCommunityDiarizationConfig {
                bundle_path: bundle.to_path_buf(),
                manifest_file: None,
                segmentation_model_file: None,
                embedding_model_file: None,
                plda_transform_file: None,
                plda_model_file: None,
                clustering_config_file: None,
                min_speakers: None,
                max_speakers: None,
                return_speaker_embeddings: false,
            })
            .map_err(|error| invalid_diarization_bundle(bundle, error))?;
        Ok(PyannoteDiarizationBundleVerification {
            kind: "pyannote-diarization",
            bundle_path: report.bundle_path,
            manifest_path: report.manifest_path,
            model_id: report.model_id,
            source_revision: report.source_revision,
            artifact_set_sha256: report.artifact_set_sha256,
            required_files: report.required_files,
        })
    }
    #[cfg(not(feature = "pyannote-diarization"))]
    {
        let _ = bundle;
        Err(NativeWhisperxError::InvalidConfig(
            "pyannote diarization bundle verification requires the pyannote-diarization feature"
                .to_string(),
        ))
    }
}

#[cfg(feature = "pyannote-vad")]
fn invalid_vad_bundle(bundle: &Path, error: impl std::fmt::Display) -> NativeWhisperxError {
    NativeWhisperxError::InvalidConfig(format!(
        "invalid local pyannote VAD bundle at `{}`: {error}",
        bundle.display()
    ))
}

#[cfg(feature = "pyannote-diarization")]
fn invalid_diarization_bundle(bundle: &Path, error: impl std::fmt::Display) -> NativeWhisperxError {
    NativeWhisperxError::InvalidConfig(format!(
        "invalid local pyannote diarization bundle at `{}`: {error}",
        bundle.display()
    ))
}

#[cfg(all(test, feature = "pyannote-vad"))]
mod tests {
    use std::fs;

    use sha2::{Digest, Sha256};

    use super::*;

    #[test]
    fn verifies_complete_bundle_and_rejects_corrupt_model() {
        let bundle = tempfile::tempdir().expect("bundle directory");
        write_bundle(bundle.path());

        let report = verify_pyannote_vad_bundle(bundle.path()).expect("valid bundle");

        assert_eq!(report.kind, "pyannote-vad");
        assert_eq!(report.input_name, "waveform");
        assert_eq!(report.output_name.as_deref(), Some("scores"));
        fs::write(
            bundle.path().join(DEFAULT_SEGMENTATION_MODEL_FILE),
            b"corrupt",
        )
        .expect("corrupt model");
        let error = verify_pyannote_vad_bundle(bundle.path())
            .expect_err("corrupt bundle should fail")
            .to_string();
        assert!(error.contains("checksum mismatch"));
    }

    #[test]
    fn translates_provider_manifest_and_model_errors() {
        let bundle = tempfile::tempdir().expect("bundle directory");
        write_bundle(bundle.path());
        let manifest_path = bundle.path().join("pyannote_vad_manifest.json");
        let mut manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest")).expect("JSON");
        manifest["source"]["revision"] = serde_json::json!("wrong");
        fs::write(&manifest_path, serde_json::to_vec(&manifest).expect("JSON")).expect("manifest");

        let error = verify_pyannote_vad_bundle(bundle.path())
            .expect_err("wrong source revision should fail")
            .to_string();
        assert!(error.contains("pinned revision is unsupported"));

        write_bundle(bundle.path());
        fs::remove_file(bundle.path().join(DEFAULT_SEGMENTATION_MODEL_FILE)).expect("remove model");
        let error = verify_pyannote_vad_bundle(bundle.path())
            .expect_err("missing model should fail")
            .to_string();
        assert!(error.contains("does not exist or is not a file"));
    }

    fn write_bundle(bundle: &Path) {
        let model = pyannote_model();
        fs::write(bundle.join(DEFAULT_SEGMENTATION_MODEL_FILE), &model).expect("model");
        let manifest = serde_json::json!({
            "schemaVersion": 1,
            "kind": "pyannote-vad",
            "source": {
                "modelId": "pyannote/segmentation-3.0",
                "revision": "e66f3d3b9eb0873085418a7b813d3b369bf160bb"
            },
            "files": {
                DEFAULT_SEGMENTATION_MODEL_FILE: format!("{:x}", Sha256::digest(&model))
            },
            "sampleRate": 16000,
            "tensorContract": {
                "inputName": "waveform",
                "inputShape": [1, 1, 160000],
                "outputName": "scores",
                "windowSeconds": 10.0,
                "frameCount": 589,
                "localSpeakerCount": 3,
                "sampleRate": 16000
            }
        });
        fs::write(
            bundle.join("pyannote_vad_manifest.json"),
            serde_json::to_vec_pretty(&manifest).expect("JSON"),
        )
        .expect("manifest");
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
}

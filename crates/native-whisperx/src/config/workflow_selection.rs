//! Automatic Workflow Selection resolver for native finite transcription.

use std::path::{Path, PathBuf};

use super::{
    AsrProvider, AutomaticWorkflowSelection, AutomaticWorkflowSelectionDecision,
    AutomaticWorkflowSelectionResource, ConfigSelection, ModelResourceSource, NativeWhisperxConfig,
    NativeWhisperxError, VadMethod,
};

const PYANNOTE_COMMUNITY_DIARIZATION_MODEL: &str = "pyannote/speaker-diarization-community-1";
const PYANNOTE_VAD_MODEL: &str = "pyannote/segmentation-3.0";
const PYANNOTE_VAD_MODEL_FILE: &str = "segmentation.onnx";

pub fn resolve_automatic_workflow_selection(
    config: &NativeWhisperxConfig,
) -> Result<AutomaticWorkflowSelection, NativeWhisperxError> {
    let mut resolved = config.clone();
    let mut decisions = Vec::new();

    if config.asr.provider != AsrProvider::Native {
        return Ok(AutomaticWorkflowSelection {
            config: resolved,
            decisions,
        });
    }

    let automatic_vad = config.vad.selection.is_automatic() && config.vad.model_bundle.is_none();
    let automatic_diarization = config.diarization.enabled
        && config.diarization.model_selection.is_automatic()
        && config.diarization.model_bundle.is_none();

    if automatic_vad {
        if config.diarization.enabled {
            resolved.vad.method = VadMethod::Pyannote;
            resolved.vad.model_bundle = None;
            resolved
                .vad
                .model_file
                .get_or_insert_with(|| PYANNOTE_VAD_MODEL_FILE.to_string());
            decisions.push(AutomaticWorkflowSelectionDecision {
                target: AutomaticWorkflowSelectionResource::Vad,
                selection: ConfigSelection::Automatic,
                model_id: Some(PYANNOTE_VAD_MODEL.to_string()),
                source: ModelResourceSource::Unresolved,
                path: None,
            });
        } else {
            resolved.vad.method = VadMethod::Energy;
            decisions.push(AutomaticWorkflowSelectionDecision {
                target: AutomaticWorkflowSelectionResource::Vad,
                selection: ConfigSelection::Automatic,
                model_id: None,
                source: ModelResourceSource::ExistingEnergyVad,
                path: None,
            });
        }
    } else {
        decisions.push(AutomaticWorkflowSelectionDecision {
            target: AutomaticWorkflowSelectionResource::Vad,
            selection: ConfigSelection::Explicit,
            model_id: Some(resolved.vad.method.as_whisperx_arg().to_string()),
            source: ModelResourceSource::ExplicitConfig,
            path: resolved.vad.model_bundle.clone(),
        });
    }

    if automatic_diarization {
        resolved.diarization.model_id = PYANNOTE_COMMUNITY_DIARIZATION_MODEL.to_string();
        decisions.push(AutomaticWorkflowSelectionDecision {
            target: AutomaticWorkflowSelectionResource::Diarization,
            selection: ConfigSelection::Automatic,
            model_id: Some(PYANNOTE_COMMUNITY_DIARIZATION_MODEL.to_string()),
            source: ModelResourceSource::Unresolved,
            path: None,
        });
    } else if config.diarization.enabled {
        decisions.push(AutomaticWorkflowSelectionDecision {
            target: AutomaticWorkflowSelectionResource::Diarization,
            selection: ConfigSelection::Explicit,
            model_id: Some(resolved.diarization.model_id.clone()),
            source: ModelResourceSource::ExplicitConfig,
            path: resolved.diarization.model_bundle.clone(),
        });
    }

    resolve_automatic_resource_paths(&mut resolved, &mut decisions)?;

    Ok(AutomaticWorkflowSelection {
        config: resolved,
        decisions,
    })
}

fn resolve_automatic_resource_paths(
    config: &mut NativeWhisperxConfig,
    decisions: &mut [AutomaticWorkflowSelectionDecision],
) -> Result<(), NativeWhisperxError> {
    let cache_only = config.asr.model_cache_only || config.alignment.model_cache_only;
    let model_dir = config
        .asr
        .model_dir
        .as_deref()
        .or(config.alignment.model_dir.as_deref());
    let cache_roots = hugging_face_cache_roots(model_dir);
    let mut missing = Vec::new();

    for decision in decisions
        .iter_mut()
        .filter(|decision| decision.selection.is_automatic())
    {
        match decision.target {
            AutomaticWorkflowSelectionResource::Vad
                if decision.model_id.as_deref() == Some(PYANNOTE_VAD_MODEL) =>
            {
                if let Some((path, source)) =
                    resolve_cached_model_dir(&cache_roots, PYANNOTE_VAD_MODEL, pyannote_vad_ready)
                {
                    config.vad.model_bundle = Some(path.clone());
                    decision.source = source;
                    decision.path = Some(path);
                } else {
                    missing.push(format!("automatic pyannote VAD `{PYANNOTE_VAD_MODEL}`"));
                }
            }
            AutomaticWorkflowSelectionResource::Diarization
                if decision.model_id.as_deref() == Some(PYANNOTE_COMMUNITY_DIARIZATION_MODEL) =>
            {
                if let Some((path, source)) = resolve_cached_model_dir(
                    &cache_roots,
                    PYANNOTE_COMMUNITY_DIARIZATION_MODEL,
                    pyannote_diarization_ready,
                ) {
                    config.diarization.model_bundle = Some(path.clone());
                    decision.source = source;
                    decision.path = Some(path);
                } else {
                    missing.push(format!(
                        "automatic pyannote diarization `{PYANNOTE_COMMUNITY_DIARIZATION_MODEL}`"
                    ));
                }
            }
            _ => {}
        }
    }

    if missing.is_empty() {
        Ok(())
    } else {
        Err(NativeWhisperxError::InvalidConfig(format!(
            "failed to resolve automatic Workflow Composition resources before transcription: {}; checked --model-dir={}; standard Hugging Face cache roots; cache-only={cache_only}; automatic pyannote bundle downloads are intentionally unsupported, so provide verified local pyannote VAD and diarization bundles or pre-cache compatible resources",
            missing.join(", "),
            model_dir
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "<not set>".to_string())
        )))
    }
}

fn resolve_cached_model_dir(
    roots: &[CacheRoot],
    model_id: &str,
    ready: fn(&Path) -> bool,
) -> Option<(PathBuf, ModelResourceSource)> {
    for root in roots {
        for candidate in hf_cache_candidates(&root.path, model_id) {
            if ready(&candidate) {
                return Some((candidate, root.source));
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
struct CacheRoot {
    path: PathBuf,
    source: ModelResourceSource,
}

fn hugging_face_cache_roots(model_dir: Option<&Path>) -> Vec<CacheRoot> {
    let mut roots = Vec::new();
    if let Some(model_dir) = model_dir {
        roots.push(CacheRoot {
            path: model_dir.to_path_buf(),
            source: ModelResourceSource::ModelDir,
        });
    }
    if let Some(home) = std::env::var_os("HF_HOME") {
        roots.push(CacheRoot {
            path: PathBuf::from(home).join("hub"),
            source: ModelResourceSource::HuggingFaceCache,
        });
    } else if let Some(home) = std::env::var_os("HOME") {
        roots.push(CacheRoot {
            path: PathBuf::from(home).join(".cache/huggingface/hub"),
            source: ModelResourceSource::HuggingFaceCache,
        });
    }
    roots
}

fn hf_cache_candidates(root: &Path, model_id: &str) -> Vec<PathBuf> {
    let mut candidates = vec![root.to_path_buf(), root.join(model_id.replace('/', "--"))];
    let hf_repo_dir = root.join(format!("models--{}", model_id.replace('/', "--")));
    if let Ok(snapshot) = std::fs::read_to_string(hf_repo_dir.join("refs/main")) {
        candidates.push(hf_repo_dir.join("snapshots").join(snapshot.trim()));
    }
    if let Ok(entries) = std::fs::read_dir(hf_repo_dir.join("snapshots")) {
        for entry in entries.flatten() {
            candidates.push(entry.path());
        }
    }
    candidates
}

fn pyannote_vad_ready(path: &Path) -> bool {
    crate::verify_pyannote_vad_bundle(path).is_ok()
}

fn pyannote_diarization_ready(path: &Path) -> bool {
    crate::verify_pyannote_diarization_bundle(path).is_ok()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Mutex;

    use super::*;
    use crate::config::{
        AlignmentConfig, AsrConfig, DiarizationConfig, InputSource, OutputConfig,
        TranslationConfig, VadConfig,
    };

    static HF_HOME_TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn automatic_workflow_selection_resolves_non_diarized_vad_to_energy() {
        let selection = resolve_automatic_workflow_selection(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig {
                selection: ConfigSelection::Automatic,
                method: VadMethod::Pyannote,
                ..VadConfig::default()
            },
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("selection should resolve");

        assert_eq!(selection.config.vad.method, VadMethod::Energy);
        assert!(selection.decisions.iter().any(|decision| {
            decision.target == AutomaticWorkflowSelectionResource::Vad
                && decision.selection == ConfigSelection::Automatic
                && decision.source == ModelResourceSource::ExistingEnergyVad
        }));
    }

    #[test]
    fn automatic_workflow_selection_uses_model_dir_before_hugging_face_cache() {
        let _hf_home_lock = HF_HOME_TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let temp = tempfile::tempdir().expect("tempdir");
        let model_dir = temp.path().join("model-dir");
        let hf_home = temp.path().join("hf-home");
        let model_dir_vad = model_dir.join("models--pyannote--segmentation-3.0/snapshots/local");
        let model_dir_diarization =
            model_dir.join("models--pyannote--speaker-diarization-community-1/snapshots/local");
        let hf_vad = hf_home.join("hub/models--pyannote--segmentation-3.0/snapshots/cached");
        let hf_diarization =
            hf_home.join("hub/models--pyannote--speaker-diarization-community-1/snapshots/cached");
        write_ready_vad(&model_dir_vad);
        write_ready_diarization(&model_dir_diarization);
        write_ready_vad(&hf_vad);
        write_ready_diarization(&hf_diarization);
        fs::create_dir_all(model_dir.join("models--pyannote--segmentation-3.0/refs"))
            .expect("vad refs dir");
        fs::write(
            model_dir.join("models--pyannote--segmentation-3.0/refs/main"),
            "local",
        )
        .expect("vad ref");
        fs::create_dir_all(
            model_dir.join("models--pyannote--speaker-diarization-community-1/refs"),
        )
        .expect("diarization refs dir");
        fs::write(
            model_dir.join("models--pyannote--speaker-diarization-community-1/refs/main"),
            "local",
        )
        .expect("diarization ref");
        let _env = EnvVarGuard::set("HF_HOME", &hf_home);

        let selection = resolve_automatic_workflow_selection(&automatic_diarization_config(
            Some(model_dir.clone()),
            false,
        ))
        .expect("selection should resolve");

        assert_eq!(selection.config.vad.method, VadMethod::Pyannote);
        assert_eq!(
            selection.config.diarization.model_id,
            PYANNOTE_COMMUNITY_DIARIZATION_MODEL
        );
        assert_eq!(
            selection.config.vad.model_bundle.as_deref(),
            Some(model_dir_vad.as_path())
        );
        assert_eq!(
            selection.config.diarization.model_bundle.as_deref(),
            Some(model_dir_diarization.as_path())
        );
        assert!(selection.decisions.iter().any(|decision| {
            decision.target == AutomaticWorkflowSelectionResource::Vad
                && decision.source == ModelResourceSource::ModelDir
        }));
        assert!(selection.decisions.iter().any(|decision| {
            decision.target == AutomaticWorkflowSelectionResource::Diarization
                && decision.source == ModelResourceSource::ModelDir
        }));
    }

    #[test]
    fn automatic_workflow_selection_cache_only_names_all_missing_resources_without_tokens() {
        let _hf_home_lock = HF_HOME_TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let empty_cache = tempfile::tempdir().expect("empty Hugging Face cache");
        let _env = EnvVarGuard::set("HF_HOME", empty_cache.path());
        let secret = "hf_secret_token";
        let error = resolve_automatic_workflow_selection(&NativeWhisperxConfig {
            diarization: DiarizationConfig {
                enabled: true,
                model_selection: ConfigSelection::Automatic,
                hf_token: Some(secret.to_string()),
                ..DiarizationConfig::default()
            },
            ..automatic_diarization_config(None, true)
        })
        .expect_err("cache-only automatic resources should be required")
        .to_string();

        assert!(error.contains("automatic pyannote VAD"), "{error}");
        assert!(error.contains("automatic pyannote diarization"));
        assert!(error.contains("cache-only=true"));
        assert!(!error.contains(secret));
    }

    #[test]
    fn automatic_workflow_selection_download_allowed_missing_resources_fail_before_transcription() {
        let _hf_home_lock = HF_HOME_TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let empty_cache = tempfile::tempdir().expect("empty Hugging Face cache");
        let _env = EnvVarGuard::set("HF_HOME", empty_cache.path());
        let secret = "hf_secret_token";
        let error = crate::build_transcription_request(&NativeWhisperxConfig {
            diarization: DiarizationConfig {
                enabled: true,
                model_selection: ConfigSelection::Automatic,
                hf_token: Some(secret.to_string()),
                ..DiarizationConfig::default()
            },
            ..automatic_diarization_config(None, false)
        })
        .expect_err("missing automatic pyannote resources must fail before transcription")
        .to_string();

        assert!(error.contains("automatic pyannote VAD"), "{error}");
        assert!(error.contains("automatic pyannote diarization"));
        assert!(error.contains("cache-only=false"));
        assert!(!error.contains("hugging-face-download"));
        assert!(!error.contains(secret));
    }

    #[test]
    fn explicit_workflow_choices_override_automatic_selection() {
        let selection = resolve_automatic_workflow_selection(&NativeWhisperxConfig {
            vad: VadConfig {
                selection: ConfigSelection::Automatic,
                method: VadMethod::Silero,
                model_bundle: Some(PathBuf::from("/models/explicit-vad")),
                ..VadConfig::default()
            },
            diarization: DiarizationConfig {
                enabled: true,
                model_selection: ConfigSelection::Automatic,
                model_id: "pyannote/speaker-diarization-community-1".to_string(),
                model_bundle: Some(PathBuf::from("/models/explicit-diarization")),
                ..DiarizationConfig::default()
            },
            ..automatic_diarization_config(None, false)
        })
        .expect("explicit choices should resolve");

        assert_eq!(selection.config.vad.method, VadMethod::Silero);
        assert_eq!(
            selection.config.vad.model_bundle.as_deref(),
            Some(Path::new("/models/explicit-vad"))
        );
        assert_eq!(
            selection.config.diarization.model_id,
            "pyannote/speaker-diarization-community-1"
        );
        assert_eq!(
            selection.config.diarization.model_bundle.as_deref(),
            Some(Path::new("/models/explicit-diarization"))
        );
        assert!(selection.decisions.iter().any(|decision| {
            decision.target == AutomaticWorkflowSelectionResource::Vad
                && decision.selection == ConfigSelection::Explicit
        }));
        assert!(selection.decisions.iter().any(|decision| {
            decision.target == AutomaticWorkflowSelectionResource::Diarization
                && decision.selection == ConfigSelection::Explicit
        }));
    }

    fn automatic_diarization_config(
        model_dir: Option<PathBuf>,
        model_cache_only: bool,
    ) -> NativeWhisperxConfig {
        NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                model_dir,
                model_cache_only,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig {
                selection: ConfigSelection::Automatic,
                ..VadConfig::default()
            },
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig {
                enabled: true,
                model_selection: ConfigSelection::Automatic,
                ..DiarizationConfig::default()
            },
            output: OutputConfig::default(),
        }
    }

    fn write_ready_vad(path: &Path) {
        fs::create_dir_all(path).expect("vad dir");
        fs::write(path.join(PYANNOTE_VAD_MODEL_FILE), pyannote_model()).expect("vad model");
        fs::write(path.join("MODEL_PROVENANCE.md"), b"provenance").expect("provenance");
        let manifest = serde_json::json!({
            "schemaVersion": 1,
            "kind": "pyannote-vad",
            "source": {
                "modelId": "pyannote/segmentation-3.0",
                "revision": "e66f3d3b9eb0873085418a7b813d3b369bf160bb",
                "license": "MIT"
            },
            "conversion": {
                "command": "test conversion",
                "python": "3.12.0",
                "packages": {"torch": "2.8.0"},
                "onnxOpset": 17,
                "inputHashes": {
                    "pytorch_model.bin": "da85c29829d4002daedd676e012936488234d9255e65e86dfab9bec6b1729298",
                    "config.yaml": "fa65a47a751602f04cc570135007d76859b69e8f9f1bfdf5878a5145980d4263",
                    "README.md": "a37bc19811cc1a52a4c128c33207813b1558b4e49b050b03e814d0a96d14f05d",
                    "LICENSE": "63a777ad4b3c7aed4b260b084d8fb49ec781c46c70c6b599ca5d2402ef7ebe50"
                }
            },
            "tensorContract": {
                "inputName": "waveform", "inputShape": [1, 1, 160000],
                "outputName": "scores", "sampleRate": 16000, "windowSeconds": 10.0,
                "frameCount": 589, "localSpeakerCount": 3
            },
            "numericalComparison": {"tolerance": 0.0001, "fixtureSeed": 217, "maxAbsoluteDifference": 0.00001},
            "files": {
                "segmentation.onnx": sha256(&path.join("segmentation.onnx")),
                "MODEL_PROVENANCE.md": sha256(&path.join("MODEL_PROVENANCE.md"))
            }
        });
        fs::write(
            path.join("pyannote_vad_manifest.json"),
            serde_json::to_vec_pretty(&manifest).expect("manifest JSON"),
        )
        .expect("manifest");
    }

    fn sha256(path: &Path) -> String {
        use sha2::{Digest, Sha256};
        format!("{:x}", Sha256::digest(fs::read(path).expect("file bytes")))
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

    fn write_ready_diarization(path: &Path) {
        fs::create_dir_all(path).expect("diarization dir");
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
        let files = [
            ("segmentation.onnx", b"segmentation".to_vec()),
            ("embedding.onnx", b"embedding".to_vec()),
            (
                "plda_transform.json",
                serde_json::to_vec(&plda_transform).expect("PLDA transform"),
            ),
            (
                "plda_model.json",
                serde_json::to_vec(&plda_model).expect("PLDA model"),
            ),
            (
                "clustering.json",
                br#"{"kind":"vbx","threshold":0.6,"fa":0.07,"fb":0.8,"maxIters":20,"minActiveRatio":0.2,"constrainedAssignment":true}"#
                    .to_vec(),
            ),
            ("MODEL_PROVENANCE.md", b"provenance".to_vec()),
            ("LICENSE.md", b"CC-BY-4.0".to_vec()),
        ];
        for (name, bytes) in &files {
            fs::write(path.join(name), bytes).expect("diarization file");
        }
        let checksums = files
            .iter()
            .map(|(name, _)| ((*name).to_string(), sha256(&path.join(name))))
            .collect::<std::collections::BTreeMap<_, _>>();
        let artifact_set_sha256 = {
            use sha2::{Digest, Sha256};
            format!(
                "{:x}",
                Sha256::digest(serde_json::to_vec(&checksums).expect("checksums JSON"))
            )
        };
        let manifest = serde_json::json!({
            "schemaVersion": 1,
            "kind": "pyannote-diarization",
            "source": {
                "modelId": "pyannote/speaker-diarization-community-1",
                "revision": "3533c8cf8e369892e6b79ff1bf80f7b0286a54ee",
                "license": "CC-BY-4.0"
            },
            "conversion": {
                "command": "test conversion",
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
                "inputName": "waveform",
                "outputName": "segmentations",
                "durationSeconds": 10.0,
                "stepRatio": 0.1,
                "powerset": true,
                "frames": 589,
                "localSpeakers": 3
            },
            "embedding": {
                "waveformInputName": "waveform",
                "maskInputName": "masks",
                "outputName": "embeddings",
                "dimension": 256,
                "maskFrames": 589
            },
            "clustering": {
                "kind": "vbx",
                "threshold": 0.6,
                "fa": 0.07,
                "fb": 0.8,
                "maxIters": 20,
                "minActiveRatio": 0.2,
                "constrainedAssignment": true
            },
            "numericalComparison": {
                "tolerance": 0.0001,
                "fixtureSeed": 218,
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
            "files": checksums
        });
        fs::write(
            path.join("pyannote_diarization_manifest.json"),
            serde_json::to_vec_pretty(&manifest).expect("manifest"),
        )
        .expect("diarization manifest");
    }

    struct EnvVarGuard {
        key: &'static str,
        previous: Option<std::ffi::OsString>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: &Path) -> Self {
            let previous = std::env::var_os(key);
            std::env::set_var(key, value);
            Self { key, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(previous) = &self.previous {
                std::env::set_var(self.key, previous);
            } else {
                std::env::remove_var(self.key);
            }
        }
    }
}

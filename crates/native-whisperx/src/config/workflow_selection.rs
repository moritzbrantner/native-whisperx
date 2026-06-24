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
const PYANNOTE_DIARIZATION_MANIFEST_FILE: &str = "pyannote_diarization_manifest.json";

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
                source: ModelResourceSource::HuggingFaceDownload,
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
            source: ModelResourceSource::HuggingFaceDownload,
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
                } else if cache_only {
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
                } else if cache_only {
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
            "failed to resolve automatic Workflow Composition resources in cache-only mode: {}; checked --model-dir={}; standard Hugging Face cache roots; cache-only=true",
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
    path.join(PYANNOTE_VAD_MODEL_FILE).is_file()
}

fn pyannote_diarization_ready(path: &Path) -> bool {
    path.join(PYANNOTE_DIARIZATION_MANIFEST_FILE).is_file()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::config::{
        AlignmentConfig, AsrConfig, DiarizationConfig, InputSource, OutputConfig,
        TranslationConfig, VadConfig,
    };

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

        assert!(error.contains("automatic pyannote VAD"));
        assert!(error.contains("automatic pyannote diarization"));
        assert!(error.contains("cache-only=true"));
        assert!(!error.contains(secret));
    }

    #[test]
    fn automatic_workflow_selection_download_allowed_builds_request_without_token_argument() {
        let request = crate::build_transcription_request(&NativeWhisperxConfig {
            diarization: DiarizationConfig {
                enabled: true,
                model_selection: ConfigSelection::Automatic,
                hf_token: Some("hf_secret_token".to_string()),
                ..DiarizationConfig::default()
            },
            ..automatic_diarization_config(None, false)
        })
        .expect("download-allowed automatic resources should route through model resolution");

        assert_eq!(
            request.diarization.model_id,
            PYANNOTE_COMMUNITY_DIARIZATION_MODEL
        );
        assert!(request.diarization.pyannote_model_bundle.is_none());
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
        fs::write(path.join(PYANNOTE_VAD_MODEL_FILE), b"vad").expect("vad model");
    }

    fn write_ready_diarization(path: &Path) {
        fs::create_dir_all(path).expect("diarization dir");
        fs::write(
            path.join(PYANNOTE_DIARIZATION_MANIFEST_FILE),
            b"diarization",
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

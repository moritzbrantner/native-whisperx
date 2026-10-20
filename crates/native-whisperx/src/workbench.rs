//! Task selection for the local Native WhisperX workbench.
//!
//! This is a fixed product workflow, not an extensible execution graph. The
//! existing finite workflow owns inference, progress, cancellation and output.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    AlignmentConfig, AsrConfig, ConfigSelection, DevicePreference, DiarizationConfig, InputSource,
    NativeWhisperxConfig, NativeWhisperxError, OutputConfig, OutputFormat, TranscriptionTask,
    TranslationConfig, VadConfig, VadMethod,
};

/// Narrow input accepted by the local workbench. Paths are authorized by the
/// embedding server; this headless type does not grant filesystem authority.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkbenchRequest {
    pub input: PathBuf,
    #[serde(default)]
    pub align: bool,
    #[serde(default)]
    pub identify_speakers: bool,
    #[serde(default)]
    pub auto_update_speakers: bool,
    #[serde(default)]
    pub translation: Option<WorkbenchTranslation>,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub device: DevicePreference,
    #[serde(default)]
    pub model_dir: Option<PathBuf>,
    #[serde(default)]
    pub vad_bundle: Option<PathBuf>,
    #[serde(default)]
    pub diarization_bundle: Option<PathBuf>,
    #[serde(default)]
    pub output_dir: Option<PathBuf>,
    #[serde(default = "default_formats")]
    pub formats: Vec<OutputFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkbenchTranslation {
    pub model_id: String,
    pub source_language: String,
    pub target_language: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WorkbenchStage {
    Transcribe,
    Align,
    IdentifySpeakers,
    Translate,
    WriteOutputs,
}

/// One resolved product request. Stage order is informational; execution is one
/// call to the existing finite workflow, never a loop of per-stage ASR runs.
#[derive(Debug)]
pub struct PreparedWorkbench {
    config: NativeWhisperxConfig,
    stages: Vec<WorkbenchStage>,
}

impl PreparedWorkbench {
    pub fn config(&self) -> &NativeWhisperxConfig {
        &self.config
    }

    pub fn stages(&self) -> &[WorkbenchStage] {
        &self.stages
    }

    pub fn into_config(self) -> NativeWhisperxConfig {
        self.config
    }
}

/// Maps selected tasks into the existing Native WhisperX configuration.
/// Resource availability and authoritative provider/language validation remain
/// in the finite workflow and may fail before or during model initialization.
pub fn prepare_workbench(
    request: WorkbenchRequest,
) -> Result<PreparedWorkbench, NativeWhisperxError> {
    let invalid = |message: &str| NativeWhisperxError::InvalidConfig(message.to_string());
    if request.input.as_os_str().is_empty() {
        return Err(invalid("select an audio or video file"));
    }
    if request.model.trim().is_empty() {
        return Err(invalid("select a transcription model"));
    }
    if request.auto_update_speakers && !request.identify_speakers {
        return Err(invalid(
            "Auto-update with new speakers requires Identify speakers",
        ));
    }
    if request.formats.is_empty() {
        return Err(invalid("select at least one output format"));
    }
    if !cfg!(feature = "native") {
        return Err(NativeWhisperxError::FeatureDisabled {
            feature: "native",
            capability: "workbench transcription",
        });
    }
    if request.identify_speakers && !cfg!(feature = "pyannote-diarization") {
        return Err(NativeWhisperxError::FeatureDisabled {
            feature: "pyannote-diarization",
            capability: "workbench speaker identification",
        });
    }
    if request.identify_speakers && !cfg!(feature = "pyannote-vad") {
        return Err(NativeWhisperxError::FeatureDisabled {
            feature: "pyannote-vad",
            capability: "workbench speaker workflow VAD",
        });
    }
    if request.device == DevicePreference::Cuda && !cfg!(feature = "cuda") {
        return Err(NativeWhisperxError::FeatureDisabled {
            feature: "cuda",
            capability: "CUDA execution",
        });
    }
    let mut stages = vec![WorkbenchStage::Transcribe];
    if request.align {
        stages.push(WorkbenchStage::Align);
    }
    if request.identify_speakers {
        stages.push(WorkbenchStage::IdentifySpeakers);
    }
    let translation = match request.translation {
        Some(translation) => {
            if !cfg!(feature = "translation") {
                return Err(NativeWhisperxError::FeatureDisabled {
                    feature: "translation",
                    capability: "workbench translation",
                });
            }
            if translation.model_id.trim().is_empty()
                || translation.source_language.trim().is_empty()
                || translation.target_language.trim().is_empty()
            {
                return Err(invalid(
                    "translation requires a model, source language and target language",
                ));
            }
            if request.language.as_deref() != Some(translation.source_language.as_str()) {
                return Err(invalid(
                    "select the transcription language to match the translation source language",
                ));
            }
            stages.push(WorkbenchStage::Translate);
            TranslationConfig {
                enabled: true,
                model_id: Some(translation.model_id),
                model_dir: request.model_dir.clone(),
                model_cache_only: true,
                source_language: Some(translation.source_language),
                target_language: Some(translation.target_language),
                ..TranslationConfig::default()
            }
        }
        None => TranslationConfig::default(),
    };
    stages.push(WorkbenchStage::WriteOutputs);
    let output_dir = request.output_dir.or_else(|| {
        request.input.parent().map(|parent| {
            if parent.as_os_str().is_empty() {
                PathBuf::from(".")
            } else {
                parent.to_path_buf()
            }
        })
    });
    let config = NativeWhisperxConfig {
        input: InputSource::Path {
            path: request.input,
        },
        asr: AsrConfig {
            task: if translation.enabled {
                TranscriptionTask::Translate
            } else {
                TranscriptionTask::Transcribe
            },
            model_id: request.model,
            language: request.language,
            device: request.device,
            model_dir: request.model_dir.clone(),
            model_cache_only: true,
            compute_type: Some("float32".into()),
            max_batch_size: Some(1),
            ..AsrConfig::default()
        },
        alignment: AlignmentConfig {
            enabled: request.align,
            model_dir: request.model_dir,
            model_cache_only: true,
            ..AlignmentConfig::default()
        },
        diarization: DiarizationConfig {
            enabled: request.identify_speakers,
            save_draft_speakers: request.auto_update_speakers,
            model_id: "pyannote/speaker-diarization-community-1".into(),
            model_selection: ConfigSelection::Automatic,
            model_bundle: request.diarization_bundle,
            ..DiarizationConfig::default()
        },
        vad: VadConfig {
            method: if request.identify_speakers || request.vad_bundle.is_some() {
                VadMethod::Pyannote
            } else {
                VadMethod::Energy
            },
            selection: ConfigSelection::Automatic,
            model_bundle: request.vad_bundle,
            ..VadConfig::default()
        },
        translation,
        output: OutputConfig {
            output_dir,
            formats: request.formats,
            ..OutputConfig::default()
        },
    };
    Ok(PreparedWorkbench { config, stages })
}

fn default_model() -> String {
    "small".into()
}
fn default_formats() -> Vec<OutputFormat> {
    vec![OutputFormat::Json]
}

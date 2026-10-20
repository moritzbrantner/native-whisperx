//! Task selection for the local Native WhisperX workbench.
//!
//! This is a fixed product workflow, not an extensible execution graph. The
//! existing finite workflow owns inference, progress, cancellation and output.

mod preflight;
pub use preflight::{preflight_workbench, WorkbenchPreflight, WorkbenchResourceCheck};

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    AlignmentConfig, AsrConfig, ConfigSelection, DevicePreference, DiarizationConfig, InputSource,
    NativeWhisperxConfig, NativeWhisperxError, OutputConfig, OutputFormat,
    SpeakerDirectorySelection, TranscriptionTask, TranslationConfig, VadConfig, VadMethod,
};

/// Narrow input accepted by the local workbench. Paths are authorized by the
/// embedding server; this headless type does not grant filesystem authority.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkbenchRequest {
    pub input: PathBuf,
    #[serde(flatten)]
    pub options: WorkbenchSettings,
}

/// Reusable product settings. Input media and session/runtime data deliberately
/// do not belong to a saved configuration. The request keeps its flat wire shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkbenchSettings {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speaker_directory: Option<SpeakerDirectorySelection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkbenchTranslation {
    pub model_id: String,
    pub source_language: String,
    pub target_language: String,
}

impl Default for WorkbenchSettings {
    fn default() -> Self {
        Self {
            align: false,
            identify_speakers: false,
            auto_update_speakers: false,
            translation: None,
            model: default_model(),
            language: None,
            device: DevicePreference::Auto,
            model_dir: None,
            vad_bundle: None,
            diarization_bundle: None,
            output_dir: None,
            formats: default_formats(),
            speaker_directory: None,
        }
    }
}

impl WorkbenchSettings {
    fn validate(&self) -> Result<(), NativeWhisperxError> {
        let invalid = |message: &str| NativeWhisperxError::InvalidConfig(message.to_owned());
        if self.model.trim().is_empty() {
            return Err(invalid("select a transcription model"));
        }
        if self.auto_update_speakers && !self.identify_speakers {
            return Err(invalid(
                "Auto-update with new speakers requires Identify speakers",
            ));
        }
        if self.formats.is_empty() {
            return Err(invalid("select at least one output format"));
        }
        if let Some(translation) = &self.translation {
            if translation.model_id.trim().is_empty()
                || translation.source_language.trim().is_empty()
                || translation.target_language.trim().is_empty()
            {
                return Err(invalid(
                    "translation requires a model, source language and target language",
                ));
            }
            if self.language.as_deref() != Some(translation.source_language.as_str()) {
                return Err(invalid(
                    "select the transcription language to match the translation source language",
                ));
            }
        }
        Ok(())
    }

    pub fn for_input(self, input: impl Into<PathBuf>) -> WorkbenchRequest {
        WorkbenchRequest {
            input: input.into(),
            options: self,
        }
    }
}

/// A named product configuration; persisted by the local app, never a runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkbenchPreset {
    pub name: String,
    pub settings: WorkbenchSettings,
}

impl WorkbenchPreset {
    pub fn new(
        name: impl Into<String>,
        settings: WorkbenchSettings,
    ) -> Result<Self, NativeWhisperxError> {
        let preset = Self {
            name: name.into().trim().to_owned(),
            settings,
        };
        preset.validate()?;
        Ok(preset)
    }

    /// Checks product choices without reading resources or preparing any model.
    pub fn validate(&self) -> Result<(), NativeWhisperxError> {
        if self.name.is_empty()
            || self.name.trim() != self.name
            || self.name.chars().count() > 128
            || self.name.chars().any(char::is_control)
        {
            return Err(NativeWhisperxError::InvalidConfig("preset name must contain 1–128 characters without control characters or surrounding whitespace".into()));
        }
        self.settings.validate()?;
        if let Some(translation) = &self.settings.translation {
            preflight::validate_translation_choice(
                &translation.model_id,
                &translation.source_language,
                &translation.target_language,
            )
            .map_err(NativeWhisperxError::InvalidConfig)?;
        }
        Ok(())
    }
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
    let WorkbenchRequest {
        input,
        options: request,
    } = request;
    let invalid = |message: &str| NativeWhisperxError::InvalidConfig(message.to_string());
    if input.as_os_str().is_empty() {
        return Err(invalid("select an audio or video file"));
    }
    request.validate()?;
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
        input.parent().map(|parent| {
            if parent.as_os_str().is_empty() {
                PathBuf::from(".")
            } else {
                parent.to_path_buf()
            }
        })
    });
    let config = NativeWhisperxConfig {
        input: InputSource::Path { path: input },
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
            speaker_directory: request.speaker_directory.unwrap_or_default(),
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

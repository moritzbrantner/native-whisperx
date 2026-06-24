//! User-facing workflow report types returned after output writing.

use std::path::PathBuf;

use audio_analysis_transcription::TranscriptionPipelineResponse;
use serde::{Deserialize, Serialize};

use super::{
    AutomaticWorkflowSelection, AutomaticWorkflowSelectionResource, ConfigSelection,
    ModelResourceSource, OutputFormat,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWhisperxReport {
    pub response: TranscriptionPipelineResponse,
    #[serde(default)]
    pub output_files: Vec<OutputFile>,
    #[serde(
        default,
        skip_serializing_if = "NativeWorkflowSelectionReport::is_empty"
    )]
    pub workflow_selection: NativeWorkflowSelectionReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputFile {
    pub format: OutputFormat,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWorkflowSelectionReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_vad: Option<SelectedVadReport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_diarization_model: Option<SelectedDiarizationModelReport>,
}

impl NativeWorkflowSelectionReport {
    pub fn from_selection(selection: &AutomaticWorkflowSelection) -> Self {
        let selected_vad = selection
            .decisions
            .iter()
            .find(|decision| decision.target == AutomaticWorkflowSelectionResource::Vad)
            .map(|decision| SelectedVadReport {
                method: selection.config.vad.method.as_whisperx_arg().to_string(),
                selection: decision.selection,
                resource_source: decision.source,
            });
        let selected_diarization_model = selection
            .config
            .diarization
            .enabled
            .then(|| {
                selection
                    .decisions
                    .iter()
                    .find(|decision| {
                        decision.target == AutomaticWorkflowSelectionResource::Diarization
                    })
                    .map(|decision| SelectedDiarizationModelReport {
                        model_id: selection.config.diarization.model_id.clone(),
                        selection: decision.selection,
                        resource_source: decision.source,
                    })
            })
            .flatten();

        Self {
            selected_vad,
            selected_diarization_model,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.selected_vad.is_none() && self.selected_diarization_model.is_none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedVadReport {
    pub method: String,
    pub selection: ConfigSelection,
    pub resource_source: ModelResourceSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedDiarizationModelReport {
    pub model_id: String,
    pub selection: ConfigSelection,
    pub resource_source: ModelResourceSource,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::config::{
        AlignmentConfig, AsrConfig, DiarizationConfig, InputSource, NativeWhisperxConfig,
        OutputConfig, TranslationConfig, VadConfig, VadMethod,
    };

    #[test]
    fn automatic_selection_report_summarizes_automatic_choices_without_paths_or_tokens() {
        let secret = "hf_secret_token";
        let selection = AutomaticWorkflowSelection {
            config: NativeWhisperxConfig {
                input: InputSource::Path {
                    path: PathBuf::from("sample.wav"),
                },
                asr: AsrConfig::default(),
                translation: TranslationConfig::default(),
                vad: VadConfig {
                    method: VadMethod::Pyannote,
                    ..VadConfig::default()
                },
                alignment: AlignmentConfig::default(),
                diarization: DiarizationConfig {
                    enabled: true,
                    model_id: "pyannote/speaker-diarization-community-1".to_string(),
                    hf_token: Some(secret.to_string()),
                    ..DiarizationConfig::default()
                },
                output: OutputConfig::default(),
            },
            decisions: vec![
                automatic_decision(
                    AutomaticWorkflowSelectionResource::Vad,
                    Some("pyannote/segmentation-3.0"),
                ),
                automatic_decision(
                    AutomaticWorkflowSelectionResource::Diarization,
                    Some("pyannote/speaker-diarization-community-1"),
                ),
            ],
        };

        let report = NativeWorkflowSelectionReport::from_selection(&selection);

        assert_eq!(
            report.selected_vad,
            Some(SelectedVadReport {
                method: "pyannote".to_string(),
                selection: ConfigSelection::Automatic,
                resource_source: ModelResourceSource::ModelDir,
            })
        );
        assert_eq!(
            report.selected_diarization_model,
            Some(SelectedDiarizationModelReport {
                model_id: "pyannote/speaker-diarization-community-1".to_string(),
                selection: ConfigSelection::Automatic,
                resource_source: ModelResourceSource::ModelDir,
            })
        );
        let json = serde_json::to_string(&report).expect("selection report json");
        assert!(!json.contains("/models"));
        assert!(!json.contains(secret));
    }

    #[test]
    fn automatic_selection_report_summarizes_explicit_choices() {
        let selection = AutomaticWorkflowSelection {
            config: NativeWhisperxConfig {
                input: InputSource::Path {
                    path: PathBuf::from("sample.wav"),
                },
                asr: AsrConfig::default(),
                translation: TranslationConfig::default(),
                vad: VadConfig {
                    selection: ConfigSelection::Explicit,
                    method: VadMethod::Energy,
                    ..VadConfig::default()
                },
                alignment: AlignmentConfig::default(),
                diarization: DiarizationConfig {
                    enabled: true,
                    model_selection: ConfigSelection::Explicit,
                    model_id: "native-spectral-speaker-baseline".to_string(),
                    ..DiarizationConfig::default()
                },
                output: OutputConfig::default(),
            },
            decisions: vec![
                explicit_decision(AutomaticWorkflowSelectionResource::Vad, Some("energy")),
                explicit_decision(
                    AutomaticWorkflowSelectionResource::Diarization,
                    Some("native-spectral-speaker-baseline"),
                ),
            ],
        };

        let report = NativeWorkflowSelectionReport::from_selection(&selection);

        assert_eq!(
            report.selected_vad,
            Some(SelectedVadReport {
                method: "energy".to_string(),
                selection: ConfigSelection::Explicit,
                resource_source: ModelResourceSource::ExplicitConfig,
            })
        );
        assert_eq!(
            report.selected_diarization_model,
            Some(SelectedDiarizationModelReport {
                model_id: "native-spectral-speaker-baseline".to_string(),
                selection: ConfigSelection::Explicit,
                resource_source: ModelResourceSource::ExplicitConfig,
            })
        );
    }

    fn automatic_decision(
        target: AutomaticWorkflowSelectionResource,
        model_id: Option<&str>,
    ) -> super::super::AutomaticWorkflowSelectionDecision {
        super::super::AutomaticWorkflowSelectionDecision {
            target,
            selection: ConfigSelection::Automatic,
            model_id: model_id.map(str::to_string),
            source: ModelResourceSource::ModelDir,
            path: Some(PathBuf::from("/models/secret")),
        }
    }

    fn explicit_decision(
        target: AutomaticWorkflowSelectionResource,
        model_id: Option<&str>,
    ) -> super::super::AutomaticWorkflowSelectionDecision {
        super::super::AutomaticWorkflowSelectionDecision {
            target,
            selection: ConfigSelection::Explicit,
            model_id: model_id.map(str::to_string),
            source: ModelResourceSource::ExplicitConfig,
            path: Some(PathBuf::from("/models/secret")),
        }
    }
}

//! Shared configuration selection metadata for automatic Workflow Composition.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigSelection {
    Automatic,
    #[default]
    Explicit,
}

impl ConfigSelection {
    pub fn is_explicit(&self) -> bool {
        matches!(self, Self::Explicit)
    }

    pub fn is_automatic(&self) -> bool {
        matches!(self, Self::Automatic)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomaticWorkflowSelection {
    pub config: super::request::NativeWhisperxConfig,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub decisions: Vec<AutomaticWorkflowSelectionDecision>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomaticWorkflowSelectionDecision {
    pub target: AutomaticWorkflowSelectionResource,
    pub selection: ConfigSelection,
    pub model_id: Option<String>,
    pub source: ModelResourceSource,
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AutomaticWorkflowSelectionResource {
    Vad,
    Diarization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelResourceSource {
    ExistingEnergyVad,
    ExplicitConfig,
    ModelDir,
    HuggingFaceCache,
    Unresolved,
    HuggingFaceDownload,
}

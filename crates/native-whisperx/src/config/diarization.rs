use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::speaker_directory::SpeakerDirectorySelection;

use super::defaults::default_true;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiarizationConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_diarization_model_id")]
    pub model_id: String,
    #[serde(default)]
    pub hf_token: Option<String>,
    #[serde(default)]
    pub hf_token_env: Option<String>,
    #[serde(default)]
    pub return_speaker_embeddings: bool,
    #[serde(default)]
    pub model_bundle: Option<PathBuf>,
    #[serde(default)]
    pub manifest_file: Option<String>,
    #[serde(default)]
    pub segmentation_model_file: Option<String>,
    #[serde(default)]
    pub embedding_model_file: Option<String>,
    #[serde(default)]
    pub plda_transform_file: Option<String>,
    #[serde(default)]
    pub plda_model_file: Option<String>,
    #[serde(default)]
    pub clustering_config_file: Option<String>,
    #[serde(default)]
    pub speaker_embedding_model_bundle: Option<PathBuf>,
    #[serde(default)]
    pub speaker_embedding_model_file: Option<String>,
    #[serde(default)]
    pub speaker_embedding_dimension: Option<usize>,
    #[serde(default)]
    pub speaker_embedding_sample_rate: Option<u32>,
    #[serde(default)]
    pub min_speakers: Option<usize>,
    #[serde(default)]
    pub max_speakers: Option<usize>,
    #[serde(default)]
    pub assignment_policy: AssignmentPolicy,
    #[serde(default)]
    pub speaker_directory: SpeakerDirectorySelection,
    #[serde(default)]
    pub disable_speaker_library: bool,
    #[serde(default = "default_true")]
    pub save_draft_speakers: bool,
    #[serde(default = "default_true")]
    pub use_draft_speakers: bool,
}

impl Default for DiarizationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            model_id: default_diarization_model_id(),
            hf_token: None,
            hf_token_env: None,
            return_speaker_embeddings: false,
            model_bundle: None,
            manifest_file: None,
            segmentation_model_file: None,
            embedding_model_file: None,
            plda_transform_file: None,
            plda_model_file: None,
            clustering_config_file: None,
            speaker_embedding_model_bundle: None,
            speaker_embedding_model_file: None,
            speaker_embedding_dimension: None,
            speaker_embedding_sample_rate: None,
            min_speakers: None,
            max_speakers: None,
            assignment_policy: AssignmentPolicy::Majority,
            speaker_directory: SpeakerDirectorySelection::default(),
            disable_speaker_library: false,
            save_draft_speakers: true,
            use_draft_speakers: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssignmentPolicy {
    #[default]
    Majority,
    NearestStart,
    StrictContained,
}

fn default_diarization_model_id() -> String {
    "native-spectral-speaker-baseline".to_string()
}

pub(crate) fn is_pyannote_diarization_model(model_id: &str) -> bool {
    model_id
        .trim()
        .to_ascii_lowercase()
        .starts_with("pyannote/")
}

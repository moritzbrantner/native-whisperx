//! Alignment configuration mapped onto the native transcript alignment provider.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::defaults::default_alignment_model_id;

/// Timestamp interpolation behavior for missing alignment spans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlignmentInterpolationMethod {
    #[default]
    Nearest,
    Linear,
    Ignore,
}

impl AlignmentInterpolationMethod {
    pub(crate) fn as_upstream(self) -> audio_analysis_transcription::AlignmentInterpolationMethod {
        match self {
            Self::Nearest => audio_analysis_transcription::AlignmentInterpolationMethod::Nearest,
            Self::Linear => audio_analysis_transcription::AlignmentInterpolationMethod::Linear,
            Self::Ignore => audio_analysis_transcription::AlignmentInterpolationMethod::Ignore,
        }
    }

    pub fn as_whisperx_arg(self) -> &'static str {
        match self {
            Self::Nearest => "nearest",
            Self::Linear => "linear",
            Self::Ignore => "ignore",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlignmentConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_alignment_model_id")]
    pub model_id: String,
    #[serde(default)]
    pub model_bundle: Option<PathBuf>,
    #[serde(default)]
    pub model_dir: Option<PathBuf>,
    #[serde(default)]
    pub model_cache_only: bool,
    #[serde(default)]
    pub interpolate_method: AlignmentInterpolationMethod,
    #[serde(default)]
    pub return_char_alignments: bool,
}

impl Default for AlignmentConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            model_id: default_alignment_model_id(),
            model_bundle: None,
            model_dir: None,
            model_cache_only: false,
            interpolate_method: AlignmentInterpolationMethod::Nearest,
            return_char_alignments: false,
        }
    }
}

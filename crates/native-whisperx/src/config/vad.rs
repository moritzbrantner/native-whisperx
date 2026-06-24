//! VAD configuration for energy, Silero, pyannote, and delegated WhisperX paths.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::defaults::{
    default_vad_enabled, default_vad_frame_seconds, default_vad_hop_seconds,
    default_vad_max_chunk_seconds, default_vad_merge_gap_seconds, default_vad_min_speech_seconds,
    default_vad_padding_seconds, default_vad_rms_threshold,
};
use super::ConfigSelection;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VadConfig {
    #[serde(default = "default_vad_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub method: VadMethod,
    #[serde(default, skip_serializing_if = "ConfigSelection::is_explicit")]
    pub selection: ConfigSelection,
    #[serde(default)]
    pub onset: Option<f32>,
    #[serde(default)]
    pub offset: Option<f32>,
    #[serde(default)]
    pub chunk_size: Option<f64>,
    #[serde(default = "default_vad_rms_threshold")]
    pub rms_threshold: f32,
    #[serde(default = "default_vad_frame_seconds")]
    pub frame_seconds: f64,
    #[serde(default = "default_vad_hop_seconds")]
    pub hop_seconds: f64,
    #[serde(default = "default_vad_min_speech_seconds")]
    pub min_speech_seconds: f64,
    #[serde(default = "default_vad_padding_seconds")]
    pub padding_seconds: f64,
    #[serde(default = "default_vad_merge_gap_seconds")]
    pub merge_gap_seconds: f64,
    #[serde(default = "default_vad_max_chunk_seconds")]
    pub max_chunk_seconds: f64,
    #[serde(default)]
    pub model_bundle: Option<PathBuf>,
    #[serde(default)]
    pub model_file: Option<String>,
    #[serde(default)]
    pub input_name: Option<String>,
    #[serde(default)]
    pub output_name: Option<String>,
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            method: VadMethod::Energy,
            selection: ConfigSelection::Explicit,
            onset: None,
            offset: None,
            chunk_size: None,
            rms_threshold: 0.01,
            frame_seconds: 0.03,
            hop_seconds: 0.01,
            min_speech_seconds: 0.08,
            padding_seconds: 0.02,
            merge_gap_seconds: 0.05,
            max_chunk_seconds: 30.0,
            model_bundle: None,
            model_file: None,
            input_name: None,
            output_name: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VadMethod {
    #[default]
    Energy,
    Pyannote,
    Silero,
}

impl VadMethod {
    pub fn as_whisperx_arg(self) -> &'static str {
        match self {
            Self::Energy => "energy",
            Self::Pyannote => "pyannote",
            Self::Silero => "silero",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ConfigSelection;

    #[test]
    fn vad_config_serializes_automatic_selection_separately_from_explicit_method() {
        let automatic = VadConfig {
            selection: ConfigSelection::Automatic,
            method: VadMethod::Pyannote,
            ..VadConfig::default()
        };

        let json = serde_json::to_value(&automatic).expect("serialize VAD config");

        assert_eq!(json["selection"], "automatic");
        assert_eq!(json["method"], "pyannote");

        let explicit = VadConfig {
            method: VadMethod::Pyannote,
            ..VadConfig::default()
        };
        let json = serde_json::to_value(&explicit).expect("serialize explicit VAD config");

        assert!(json.get("selection").is_none());
        assert_eq!(json["method"], "pyannote");

        let decoded: VadConfig = serde_json::from_value(serde_json::json!({
            "method": "silero"
        }))
        .expect("deserialize existing VAD config shape");
        assert_eq!(decoded.selection, ConfigSelection::Explicit);
        assert_eq!(decoded.method, VadMethod::Silero);

        let decoded: VadConfig = serde_json::from_value(serde_json::json!({
            "selection": "automatic",
            "method": "energy"
        }))
        .expect("deserialize automatic VAD config");
        assert_eq!(decoded.selection, ConfigSelection::Automatic);
        assert_eq!(decoded.method, VadMethod::Energy);
    }
}

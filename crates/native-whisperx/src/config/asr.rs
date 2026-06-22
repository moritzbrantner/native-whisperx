//! ASR provider, decode, and external WhisperX compatibility configuration.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::defaults::{
    default_batch_chunks, default_external_whisperx_model, default_max_batch_size,
    default_whisper_model_id, default_whisperx_command,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsrConfig {
    #[serde(default)]
    pub provider: AsrProvider,
    #[serde(default)]
    pub task: TranscriptionTask,
    #[serde(default = "default_whisper_model_id")]
    pub model_id: String,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub whisper_bundle: Option<PathBuf>,
    #[serde(default)]
    pub model_dir: Option<PathBuf>,
    #[serde(default)]
    pub model_cache_only: bool,
    #[serde(default)]
    pub device: DevicePreference,
    #[serde(default)]
    pub device_index: Option<String>,
    #[serde(default)]
    pub compute_type: Option<String>,
    #[serde(default = "default_batch_chunks")]
    pub batch_chunks: bool,
    #[serde(default = "default_max_batch_size")]
    pub max_batch_size: Option<usize>,
    #[serde(default)]
    pub decode: WhisperxDecodeConfig,
    #[serde(default)]
    pub external_whisperx: ExternalWhisperxConfig,
}

impl Default for AsrConfig {
    fn default() -> Self {
        Self {
            provider: AsrProvider::Native,
            task: TranscriptionTask::Transcribe,
            model_id: default_whisper_model_id(),
            language: None,
            whisper_bundle: None,
            model_dir: None,
            model_cache_only: false,
            device: DevicePreference::Auto,
            device_index: None,
            compute_type: None,
            batch_chunks: true,
            max_batch_size: Some(4),
            decode: WhisperxDecodeConfig::default(),
            external_whisperx: ExternalWhisperxConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AsrProvider {
    #[default]
    Native,
    ExternalWhisperX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TranscriptionTask {
    #[default]
    Transcribe,
    Translate,
}

impl TranscriptionTask {
    pub fn as_whisperx_arg(self) -> &'static str {
        match self {
            Self::Transcribe => "transcribe",
            Self::Translate => "translate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DevicePreference {
    #[default]
    Auto,
    Cpu,
    Cuda,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhisperxDecodeConfig {
    #[serde(default)]
    pub temperature: Vec<f32>,
    #[serde(default)]
    pub best_of: Option<usize>,
    #[serde(default)]
    pub beam_size: Option<usize>,
    #[serde(default)]
    pub patience: Option<f32>,
    #[serde(default)]
    pub length_penalty: Option<f32>,
    #[serde(default)]
    pub suppress_tokens: Option<String>,
    #[serde(default)]
    pub suppress_numerals: bool,
    #[serde(default)]
    pub initial_prompt: Option<String>,
    #[serde(default)]
    pub hotwords: Option<String>,
    #[serde(default)]
    pub condition_on_previous_text: Option<bool>,
    #[serde(default)]
    pub fp16: Option<bool>,
    #[serde(default)]
    pub compression_ratio_threshold: Option<f32>,
    #[serde(default)]
    pub logprob_threshold: Option<f32>,
    #[serde(default)]
    pub no_speech_threshold: Option<f32>,
    #[serde(default)]
    pub threads: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalWhisperxConfig {
    #[serde(default = "default_whisperx_command")]
    pub command: PathBuf,
    #[serde(default = "default_external_whisperx_model")]
    pub model: String,
    #[serde(default)]
    pub compute_type: Option<String>,
    #[serde(default)]
    pub batch_size: Option<usize>,
    #[serde(default)]
    pub align_model: Option<String>,
    #[serde(default)]
    pub diarize: bool,
    #[serde(default)]
    pub min_speakers: Option<usize>,
    #[serde(default)]
    pub max_speakers: Option<usize>,
    #[serde(default)]
    pub hf_token_env: Option<String>,
    #[serde(default)]
    pub output_dir: Option<PathBuf>,
    #[serde(default)]
    pub timeout_seconds: Option<u64>,
    #[serde(default)]
    pub extra_args: Vec<String>,
}

impl Default for ExternalWhisperxConfig {
    fn default() -> Self {
        Self {
            command: default_whisperx_command(),
            model: default_external_whisperx_model(),
            compute_type: None,
            batch_size: None,
            align_model: None,
            diarize: false,
            min_speakers: None,
            max_speakers: None,
            hf_token_env: None,
            output_dir: None,
            timeout_seconds: None,
            extra_args: Vec::new(),
        }
    }
}

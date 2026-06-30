#![doc = include_str!("../README.md")]

pub mod surface;

#[cfg(feature = "alignment")]
mod ctc_alignment;
mod native_audio;
mod native_bundles;
mod native_device;
#[cfg(feature = "alignment")]
mod native_wav2vec2;
#[cfg(feature = "alignment")]
mod native_wav2vec2_model;
#[cfg(feature = "candle")]
mod native_whisper;
#[cfg(any(feature = "silero-vad", test))]
mod silero_vad;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use text_transcripts::{
    normalize_transcription_contract, TranscriptCharContract, TranscriptWordContract,
    TranscriptionContract,
};
use video_analysis_core::{DetectError, Result};

const CANDLE_WHISPER_AUTOREGRESSIVE_KV_CACHE_EXECUTION: &str =
    "candle-whisper-autoregressive-kv-cache";
const CANDLE_WHISPER_ACTIVE_ROW_TENSOR_BATCH_EXECUTION: &str =
    "candle-whisper-active-row-tensor-batch";

pub use audio_analysis_speakers::{
    AudioRuntime, SpeakerDiarizationOptions, SpeakerDiarizationResponse, SpeakerSegmentPrediction,
    SpeakerTranscriptAssignmentPolicy,
};
#[cfg(feature = "silero-vad")]
pub use silero_vad::{SileroVadOptions, SileroVadTranscriptionProvider};

/// Backward-compatible name for Transcript Speaker Assignment policy.
pub type SpeakerAssignmentPolicy = SpeakerTranscriptAssignmentPolicy;

/// Request for an audio/video-to-text transcription pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionPipelineRequest {
    pub source: TranscriptionSource,
    pub provider: TranscriptionProviderSelection,
    #[serde(default)]
    pub vad: VadOptions,
    #[serde(default)]
    pub alignment: AlignmentOptions,
    #[serde(default)]
    pub diarization: DiarizationOptions,
    #[serde(default)]
    pub output: TranscriptionOutputOptions,
}

/// Phase-level observer event emitted by the native transcription pipeline.
#[derive(Debug, Clone, PartialEq)]
pub enum TranscriptionPipelineEvent {
    ValidationStart,
    DecodeStart,
    DecodeEnd {
        duration_seconds: f64,
        samples: usize,
    },
    VadStart {
        provider: String,
    },
    VadEnd {
        segments: usize,
        windows: Option<usize>,
    },
    AsrStart {
        model_id: String,
    },
    AsrEnd {
        segments: usize,
    },
    ModelLoadStart {
        stage: String,
        provider: String,
        model_id: String,
    },
    ModelLoadEnd {
        stage: String,
        provider: String,
        model_id: String,
        duration_seconds: f64,
    },
    ModelReuse {
        stage: String,
        provider: String,
        model_id: String,
    },
    AlignmentStart {
        model_id: String,
    },
    AlignmentEnd {
        words: usize,
    },
    DiarizationStart {
        provider: String,
    },
    DiarizationEnd {
        speakers: usize,
        segments: usize,
    },
}

/// Observer for phase-level native transcription progress.
pub trait TranscriptionPipelineObserver {
    fn observe(&mut self, event: TranscriptionPipelineEvent);
}

/// Observer implementation that discards all events.
#[derive(Debug, Default)]
pub struct NoopTranscriptionPipelineObserver;

impl TranscriptionPipelineObserver for NoopTranscriptionPipelineObserver {
    fn observe(&mut self, _event: TranscriptionPipelineEvent) {}
}

/// Source accepted by transcription providers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum TranscriptionSource {
    Path {
        path: PathBuf,
    },
    Samples {
        samples: Vec<f32>,
        #[serde(rename = "sampleRate", alias = "sample_rate")]
        sample_rate: u32,
        channels: u16,
        #[serde(default)]
        source: Option<String>,
    },
}

impl TranscriptionSource {
    fn path(&self) -> Result<&Path> {
        match self {
            Self::Path { path } => Ok(path),
            Self::Samples { .. } => Err(invalid_request(
                "external command transcription requires a path source",
            )),
        }
    }
}

/// Provider selection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum TranscriptionProviderSelection {
    #[serde(rename = "candleWhisper", alias = "candle-whisper")]
    CandleWhisper(CandleWhisperOptions),
    #[serde(rename = "whisperCpp", alias = "whisper-cpp")]
    WhisperCpp(WhisperCppProviderOptions),
    #[serde(rename = "externalWhisperX", alias = "whisperx")]
    ExternalWhisperX(WhisperXCommandOptions),
}

impl TranscriptionProviderSelection {
    pub fn provider_id(&self) -> &'static str {
        match self {
            Self::CandleWhisper(_) => "candle-whisper",
            Self::WhisperCpp(_) => "whisper-cpp",
            Self::ExternalWhisperX(_) => "whisperx-command",
        }
    }

    pub fn model_id(&self) -> &str {
        match self {
            Self::CandleWhisper(options) => &options.model_id,
            Self::WhisperCpp(options) => &options.model_id,
            Self::ExternalWhisperX(options) => &options.model,
        }
    }

    pub fn task(&self) -> TranscriptionTask {
        match self {
            Self::CandleWhisper(options) => options.task,
            Self::WhisperCpp(options) => options.task,
            Self::ExternalWhisperX(options) => options.task,
        }
    }
}

/// Whisper speech task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TranscriptionTask {
    #[default]
    Transcribe,
    Translate,
}

impl TranscriptionTask {
    pub fn as_whisper_task(self) -> &'static str {
        match self {
            Self::Transcribe => "transcribe",
            Self::Translate => "translate",
        }
    }

    pub fn output_language_hint(self) -> Option<&'static str> {
        match self {
            Self::Transcribe => None,
            Self::Translate => Some("en"),
        }
    }
}

/// Options for the Candle Whisper native provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandleWhisperOptions {
    #[serde(default = "default_candle_whisper_model")]
    pub model_id: String,
    #[serde(default)]
    pub task: TranscriptionTask,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub device: NativeDevicePreference,
    #[serde(default)]
    pub compute_type: CandleWhisperComputeType,
    #[serde(default)]
    pub model_bundle: Option<PathBuf>,
    #[serde(default)]
    pub model_dir: Option<PathBuf>,
    #[serde(default)]
    pub model_cache_only: bool,
    #[serde(default)]
    pub batch_chunks: bool,
    #[serde(default)]
    pub max_batch_size: Option<usize>,
    #[serde(default)]
    pub decode_runtime: CandleWhisperDecodeRuntime,
}

impl Default for CandleWhisperOptions {
    fn default() -> Self {
        Self {
            model_id: default_candle_whisper_model(),
            task: TranscriptionTask::Transcribe,
            language: None,
            device: NativeDevicePreference::Auto,
            compute_type: CandleWhisperComputeType::Automatic,
            model_bundle: None,
            model_dir: None,
            model_cache_only: false,
            batch_chunks: true,
            max_batch_size: Some(4),
            decode_runtime: CandleWhisperDecodeRuntime::AutoregressiveKvCache,
        }
    }
}

fn default_candle_whisper_model() -> String {
    "openai/whisper-large-v3-turbo".to_string()
}

/// Native Candle Whisper compute-type preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CandleWhisperComputeType {
    #[default]
    #[serde(alias = "auto")]
    Automatic,
    #[serde(alias = "float16")]
    Fp16,
    #[serde(alias = "float32")]
    Fp32,
}

impl CandleWhisperComputeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Fp16 => "fp16",
            Self::Fp32 => "fp32",
        }
    }

    pub(crate) fn resolve_for_device(self, cuda_active: bool) -> Result<Self> {
        match (self, cuda_active) {
            (Self::Automatic, true) => Ok(Self::Fp16),
            (Self::Automatic, false) => Ok(Self::Fp32),
            (Self::Fp16, true) => Ok(Self::Fp16),
            (Self::Fp16, false) => Err(setup_error(
                "native Candle Whisper compute type fp16 requires a CUDA device; use automatic or fp32 for CPU execution",
            )),
            (Self::Fp32, _) => Ok(Self::Fp32),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn setup_fallback_eligible(self) -> bool {
        matches!(self, Self::Automatic)
    }
}

/// Native Candle Whisper chunk decode runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CandleWhisperDecodeRuntime {
    /// Existing safe per-window autoregressive decode with KV-cache reuse inside each window.
    #[default]
    AutoregressiveKvCache,
    /// Future true tensor-batched active-row decode path.
    ActiveRowTensorBatch,
}

impl CandleWhisperDecodeRuntime {
    pub fn execution_id(self) -> &'static str {
        match self {
            Self::AutoregressiveKvCache => CANDLE_WHISPER_AUTOREGRESSIVE_KV_CACHE_EXECUTION,
            Self::ActiveRowTensorBatch => CANDLE_WHISPER_ACTIVE_ROW_TENSOR_BATCH_EXECUTION,
        }
    }

    pub fn is_supported(self) -> bool {
        matches!(
            self,
            Self::AutoregressiveKvCache | Self::ActiveRowTensorBatch
        )
    }
}

/// Options for native whisper.cpp compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhisperCppProviderOptions {
    #[serde(default = "default_whisper_cpp_model")]
    pub model_id: String,
    #[serde(default)]
    pub task: TranscriptionTask,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub model_path: Option<PathBuf>,
}

impl Default for WhisperCppProviderOptions {
    fn default() -> Self {
        Self {
            model_id: default_whisper_cpp_model(),
            task: TranscriptionTask::Transcribe,
            language: None,
            model_path: None,
        }
    }
}

fn default_whisper_cpp_model() -> String {
    "large-v3-turbo".to_string()
}

/// Native execution device preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NativeDevicePreference {
    #[default]
    Auto,
    Cpu,
    Cuda,
}

/// Options for the external WhisperX command provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhisperXCommandOptions {
    pub command: PathBuf,
    pub model: String,
    #[serde(default)]
    pub task: TranscriptionTask,
    #[serde(default)]
    pub language: Option<String>,
    pub device: WhisperXDevice,
    #[serde(default)]
    pub compute_type: Option<String>,
    #[serde(default)]
    pub batch_size: Option<usize>,
    #[serde(default)]
    pub align_model: Option<String>,
    #[serde(default)]
    pub model_dir: Option<PathBuf>,
    #[serde(default)]
    pub model_cache_only: bool,
    #[serde(default)]
    pub no_align: bool,
    #[serde(default)]
    pub interpolate_method: AlignmentInterpolationMethod,
    #[serde(default)]
    pub return_char_alignments: bool,
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

/// Backward-compatible name for the external WhisperX provider options.
pub type WhisperXOptions = WhisperXCommandOptions;

impl Default for WhisperXCommandOptions {
    fn default() -> Self {
        Self {
            command: PathBuf::from("whisperx"),
            model: "large-v2".to_string(),
            task: TranscriptionTask::Transcribe,
            language: None,
            device: WhisperXDevice::Cpu,
            compute_type: None,
            batch_size: None,
            align_model: None,
            model_dir: None,
            model_cache_only: false,
            no_align: false,
            interpolate_method: AlignmentInterpolationMethod::Nearest,
            return_char_alignments: false,
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

/// WhisperX device selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WhisperXDevice {
    Cpu,
    Cuda,
}

impl WhisperXDevice {
    fn as_str(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Cuda => "cuda",
        }
    }
}

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
    pub fn as_whisperx_arg(self) -> &'static str {
        match self {
            Self::Nearest => "nearest",
            Self::Linear => "linear",
            Self::Ignore => "ignore",
        }
    }
}

/// VAD options used before ASR chunking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VadOptions {
    #[serde(default = "default_vad_enabled")]
    pub enabled: bool,
    #[serde(default = "default_vad_threshold")]
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
}

impl Default for VadOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            rms_threshold: default_vad_threshold(),
            frame_seconds: default_vad_frame_seconds(),
            hop_seconds: default_vad_hop_seconds(),
            min_speech_seconds: default_vad_min_speech_seconds(),
            padding_seconds: default_vad_padding_seconds(),
            merge_gap_seconds: default_vad_merge_gap_seconds(),
            max_chunk_seconds: default_vad_max_chunk_seconds(),
        }
    }
}

fn default_vad_enabled() -> bool {
    true
}
fn default_vad_threshold() -> f32 {
    0.01
}
fn default_vad_frame_seconds() -> f64 {
    0.03
}
fn default_vad_hop_seconds() -> f64 {
    0.01
}
fn default_vad_min_speech_seconds() -> f64 {
    0.08
}
fn default_vad_padding_seconds() -> f64 {
    0.02
}
fn default_vad_merge_gap_seconds() -> f64 {
    0.05
}
fn default_vad_max_chunk_seconds() -> f64 {
    30.0
}

/// Forced-alignment options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlignmentOptions {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_alignment_model")]
    pub model_id: String,
    #[serde(default = "default_alignment_device")]
    pub device: NativeDevicePreference,
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

impl Default for AlignmentOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            model_id: default_alignment_model(),
            device: default_alignment_device(),
            model_bundle: None,
            model_dir: None,
            model_cache_only: false,
            interpolate_method: AlignmentInterpolationMethod::Nearest,
            return_char_alignments: false,
        }
    }
}

fn default_alignment_model() -> String {
    "facebook/wav2vec2-base-960h".to_string()
}

fn default_alignment_device() -> NativeDevicePreference {
    NativeDevicePreference::Cpu
}

/// Diarization options.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiarizationOptions {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default, flatten)]
    pub speaker: SpeakerDiarizationOptions,
}

impl std::ops::Deref for DiarizationOptions {
    type Target = SpeakerDiarizationOptions;

    fn deref(&self) -> &Self::Target {
        &self.speaker
    }
}

impl std::ops::DerefMut for DiarizationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.speaker
    }
}

/// Output preferences for transcription.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionOutputOptions {
    #[serde(default = "default_output_formats")]
    pub formats: Vec<String>,
}

impl Default for TranscriptionOutputOptions {
    fn default() -> Self {
        Self {
            formats: default_output_formats(),
        }
    }
}

fn default_output_formats() -> Vec<String> {
    vec!["json".to_string()]
}

/// Artifact produced or discovered by a provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionArtifact {
    pub kind: String,
    pub path: PathBuf,
}

/// Response from a transcription pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionPipelineResponse {
    pub accepted: bool,
    pub operation: String,
    pub provider: String,
    pub model_id: String,
    pub transcript: TranscriptionContract,
    pub vad_segments: Vec<SpeechActivitySegment>,
    pub alignment: Option<AlignmentSummary>,
    pub diarization: Option<SpeakerDiarizationResponse>,
    pub artifacts: Vec<TranscriptionArtifact>,
    pub diagnostics: Vec<String>,
}

/// Metadata-only provider plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionProviderPlan {
    pub provider_id: String,
    pub external_runtime: bool,
    pub wasm_supported: bool,
    pub primary: bool,
    pub setup: Vec<String>,
    pub diagnostics: Vec<String>,
}

/// Loaded audio for native providers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadedAudio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    #[serde(default)]
    pub source: Option<String>,
}

impl LoadedAudio {
    pub fn mono_16khz_from_source(source: &TranscriptionSource) -> Result<Self> {
        native_audio::mono_16khz_from_source(source)
    }

    pub fn duration_seconds(&self) -> f64 {
        if self.sample_rate == 0 || self.channels == 0 {
            return 0.0;
        }
        self.samples.len() as f64 / self.channels as f64 / self.sample_rate as f64
    }
}

/// A speech activity span used for ASR chunking.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechActivitySegment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub score: f32,
}

impl SpeechActivitySegment {
    pub fn new(start_seconds: f64, end_seconds: f64, score: f32) -> Result<Self> {
        let segment = Self {
            start_seconds,
            end_seconds,
            score,
        };
        segment.validate()?;
        Ok(segment)
    }

    pub fn validate(&self) -> Result<()> {
        if !self.start_seconds.is_finite()
            || !self.end_seconds.is_finite()
            || !self.score.is_finite()
        {
            return Err(invalid_request(
                "speech activity segment values must be finite",
            ));
        }
        if self.start_seconds < 0.0 || self.end_seconds <= self.start_seconds {
            return Err(invalid_request(
                "speech activity segment must have non-negative start and positive duration",
            ));
        }
        Ok(())
    }
}

/// ASR provider request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsrRequest {
    pub audio: LoadedAudio,
    pub chunks: Vec<SpeechActivitySegment>,
    #[serde(default)]
    pub task: TranscriptionTask,
    pub language: Option<String>,
    pub model_id: String,
}

/// ASR provider response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsrResponse {
    pub model_id: String,
    pub language: Option<String>,
    pub transcript: TranscriptionContract,
    #[serde(default)]
    pub diagnostics: Vec<String>,
}

/// Forced-alignment request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlignmentRequest {
    pub audio: LoadedAudio,
    pub transcript: TranscriptionContract,
    pub language: Option<String>,
    pub model_id: String,
}

/// Forced-alignment response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlignmentResponse {
    pub model_id: String,
    pub words: Vec<AlignedWord>,
    #[serde(default)]
    pub chars: Vec<AlignedChar>,
    #[serde(default)]
    pub diagnostics: Vec<String>,
}

/// One aligned word timing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlignedWord {
    pub segment_index: u64,
    pub word_index: usize,
    pub text: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    #[serde(default)]
    pub confidence: Option<f32>,
}

/// One aligned character timing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlignedChar {
    pub segment_index: u64,
    pub char_index: usize,
    pub character: String,
    #[serde(default)]
    pub start_seconds: Option<f64>,
    #[serde(default)]
    pub end_seconds: Option<f64>,
    #[serde(default)]
    pub confidence: Option<f32>,
}

/// Alignment summary included in pipeline responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlignmentSummary {
    pub provider: String,
    pub model_id: String,
    pub word_count: usize,
}

/// VAD request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VadRequest {
    pub audio: LoadedAudio,
    pub options: VadOptions,
}

/// VAD response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VadResponse {
    pub segments: Vec<SpeechActivitySegment>,
    #[serde(default)]
    pub diagnostics: Vec<String>,
}

/// Trait for audio transcription providers.
pub trait AudioTranscriptionProvider {
    fn provider_id(&self) -> &str;
    fn transcribe(&mut self, request: AsrRequest) -> Result<AsrResponse>;

    fn transcribe_with_observer(
        &mut self,
        request: AsrRequest,
        _observer: &mut dyn TranscriptionPipelineObserver,
    ) -> Result<AsrResponse> {
        self.transcribe(request)
    }
}

/// Trait for forced alignment providers.
pub trait ForcedAlignmentProvider {
    fn provider_id(&self) -> &str;
    fn align(&mut self, request: AlignmentRequest) -> Result<AlignmentResponse>;

    fn align_with_observer(
        &mut self,
        request: AlignmentRequest,
        _observer: &mut dyn TranscriptionPipelineObserver,
    ) -> Result<AlignmentResponse> {
        self.align(request)
    }
}

/// Trait for transcription VAD providers.
pub trait TranscriptionVadProvider {
    fn provider_id(&self) -> &str;
    fn detect_speech(&mut self, request: VadRequest) -> Result<VadResponse>;
}

/// Trait for transcript diarization and speaker assignment providers.
pub trait TranscriptDiarizationProvider {
    fn provider_id(&self) -> &str;
    fn diarize(
        &mut self,
        audio: LoadedAudio,
        transcript: &TranscriptionContract,
        options: &DiarizationOptions,
    ) -> Result<SpeakerDiarizationResponse>;
}

/// Native deterministic speaker diarization adapter.
#[cfg(feature = "diarization")]
#[derive(Debug, Clone, Default)]
pub struct NativeSpeakerDiarizationProvider;

#[cfg(feature = "diarization")]
impl TranscriptDiarizationProvider for NativeSpeakerDiarizationProvider {
    fn provider_id(&self) -> &str {
        "native-speaker-diarization"
    }

    fn diarize(
        &mut self,
        audio: LoadedAudio,
        transcript: &TranscriptionContract,
        options: &DiarizationOptions,
    ) -> Result<SpeakerDiarizationResponse> {
        native_audio::validate_loaded_audio(&audio)?;
        if options.is_pyannote_model() {
            #[cfg(feature = "pyannote-diarization")]
            {
                let mut provider = PyannoteCommunityTranscriptDiarizationProvider;
                return provider.diarize(audio, transcript, options);
            }
            #[cfg(not(feature = "pyannote-diarization"))]
            {
                return Err(setup_error(
                    "native pyannote diarization requires the pyannote-diarization feature",
                ));
            }
        }
        if options.speaker_embedding_model_bundle.is_some() {
            return diarize_with_onnx_speaker_embeddings(audio, transcript, options);
        }
        let spans = speech_spans_from_transcript(transcript, audio.duration_seconds())?;
        if !spans.is_empty() {
            let speaker_audio =
                audio_analysis_speakers::SpeakerAudio::mono(&audio.samples, audio.sample_rate)?;
            let embedder = audio_analysis_speakers::SpectralSpeakerEmbedder::default();
            let vad = TranscriptSpeechSpanVad { spans };
            let mut diarizer = audio_analysis_speakers::WindowedSpeakerDiarizer::new(embedder, vad)
                .cluster_threshold(0.95)?
                .speaker_bounds(options.min_speakers, options.max_speakers)?;
            let result =
                audio_analysis_speakers::SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?;
            return Ok(SpeakerDiarizationResponse {
                accepted: true,
                operation: "audio.speakers.diarize".to_string(),
                model_id: options.model_id.clone(),
                runtime: audio_analysis_speakers::AudioRuntime::Heuristic,
                segments: stable_speaker_predictions_from_diarization(result.segments)?,
                speaker_embeddings: None,
                diagnostics: Vec::new(),
            });
        }

        let speaker_audio =
            audio_analysis_speakers::SpeakerAudio::mono(&audio.samples, audio.sample_rate)?;
        let embedder = audio_analysis_speakers::SpectralSpeakerEmbedder::default();
        let vad_config = audio_analysis_speakers::EnergyVadConfig::default();
        let vad = audio_analysis_speakers::EnergyVoiceActivityDetector::new(vad_config)?;
        let mut diarizer = audio_analysis_speakers::WindowedSpeakerDiarizer::new(embedder, vad)
            .cluster_threshold(0.95)?
            .speaker_bounds(options.min_speakers, options.max_speakers)?;
        let result =
            audio_analysis_speakers::SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?;
        Ok(SpeakerDiarizationResponse {
            accepted: true,
            operation: "audio.speakers.diarize".to_string(),
            model_id: options.model_id.clone(),
            runtime: audio_analysis_speakers::AudioRuntime::Heuristic,
            segments: stable_speaker_predictions_from_diarization(result.segments)?,
            speaker_embeddings: None,
            diagnostics: Vec::new(),
        })
    }
}

/// Native pyannote community diarization adapter.
#[cfg(all(feature = "diarization", feature = "pyannote-diarization"))]
#[derive(Debug, Clone, Default)]
pub struct PyannoteCommunityTranscriptDiarizationProvider;

#[cfg(all(feature = "diarization", feature = "pyannote-diarization"))]
impl TranscriptDiarizationProvider for PyannoteCommunityTranscriptDiarizationProvider {
    fn provider_id(&self) -> &str {
        "pyannote-community-diarization"
    }

    fn diarize(
        &mut self,
        audio: LoadedAudio,
        _transcript: &TranscriptionContract,
        options: &DiarizationOptions,
    ) -> Result<SpeakerDiarizationResponse> {
        native_audio::validate_loaded_audio(&audio)?;
        if !options.is_pyannote_model() {
            return Err(invalid_request(format!(
                "pyannote community diarization provider does not support model `{}`",
                options.model_id
            )));
        }
        let bundle_path = options.pyannote_model_bundle.clone().ok_or_else(|| {
            setup_error(
                "native pyannote diarization requires --diarization-model-bundle or DiarizationOptions.pyannote_model_bundle",
            )
        })?;
        let speaker_audio =
            audio_analysis_speakers::SpeakerAudio::mono(&audio.samples, audio.sample_rate)?;
        let mut diarizer = audio_analysis_speakers::PyannoteCommunityDiarizer::from_config(
            audio_analysis_speakers::PyannoteCommunityDiarizationConfig {
                bundle_path,
                manifest_file: options.pyannote_manifest_file.clone(),
                segmentation_model_file: options.pyannote_segmentation_model_file.clone(),
                embedding_model_file: options.pyannote_embedding_model_file.clone(),
                plda_transform_file: options.pyannote_plda_transform_file.clone(),
                plda_model_file: options.pyannote_plda_model_file.clone(),
                clustering_config_file: options.pyannote_clustering_config_file.clone(),
                min_speakers: options.min_speakers,
                max_speakers: options.max_speakers,
                return_speaker_embeddings: options.return_speaker_embeddings,
            },
        )?;
        let mut result = diarizer.diarize(&speaker_audio)?.response;
        result.model_id = options.model_id.clone();
        Ok(result)
    }
}

#[cfg(feature = "diarization")]
fn diarize_with_onnx_speaker_embeddings(
    audio: LoadedAudio,
    transcript: &TranscriptionContract,
    options: &DiarizationOptions,
) -> Result<SpeakerDiarizationResponse> {
    let config = options.onnx_speaker_embedding_config()?;
    let speaker_audio =
        audio_analysis_speakers::SpeakerAudio::mono(&audio.samples, audio.sample_rate)?;
    let embedder = audio_analysis_speakers::OnnxSpeakerEmbedder::from_config(config)?;
    let spans = speech_spans_from_transcript(transcript, audio.duration_seconds())?;
    let result = if spans.is_empty() {
        let vad = audio_analysis_speakers::EnergyVoiceActivityDetector::default();
        let mut diarizer = audio_analysis_speakers::WindowedSpeakerDiarizer::new(embedder, vad)
            .cluster_threshold(0.95)?
            .speaker_bounds(options.min_speakers, options.max_speakers)?;
        audio_analysis_speakers::SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?
    } else {
        let vad = TranscriptSpeechSpanVad { spans };
        let mut diarizer = audio_analysis_speakers::WindowedSpeakerDiarizer::new(embedder, vad)
            .cluster_threshold(0.95)?
            .speaker_bounds(options.min_speakers, options.max_speakers)?;
        audio_analysis_speakers::SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?
    };
    Ok(SpeakerDiarizationResponse {
        accepted: true,
        operation: "audio.speakers.diarize".to_string(),
        model_id: options.model_id.clone(),
        runtime: audio_analysis_speakers::AudioRuntime::Onnx,
        segments: stable_speaker_predictions_from_diarization(result.segments)?,
        speaker_embeddings: None,
        diagnostics: Vec::new(),
    })
}

/// Default pure-Rust energy VAD provider.
#[derive(Debug, Clone, Default)]
pub struct EnergyVadTranscriptionProvider;

impl TranscriptionVadProvider for EnergyVadTranscriptionProvider {
    fn provider_id(&self) -> &str {
        "energy-vad"
    }

    fn detect_speech(&mut self, request: VadRequest) -> Result<VadResponse> {
        let segments = energy_vad_segments(&request.audio, &request.options)?;
        Ok(VadResponse {
            segments,
            diagnostics: vec!["deterministic energy VAD completed".to_string()],
        })
    }
}

/// Feature-gated Candle Whisper provider.
#[derive(Debug, Clone, Default)]
pub struct CandleWhisperTranscriber {
    pub options: CandleWhisperOptions,
}

impl CandleWhisperTranscriber {
    pub fn new(options: CandleWhisperOptions) -> Self {
        Self { options }
    }
}

impl AudioTranscriptionProvider for CandleWhisperTranscriber {
    fn provider_id(&self) -> &str {
        "candle-whisper"
    }

    fn transcribe(&mut self, request: AsrRequest) -> Result<AsrResponse> {
        let mut observer = NoopTranscriptionPipelineObserver;
        self.transcribe_with_observer(request, &mut observer)
    }

    fn transcribe_with_observer(
        &mut self,
        request: AsrRequest,
        observer: &mut dyn TranscriptionPipelineObserver,
    ) -> Result<AsrResponse> {
        validate_asr_request(&request)?;
        validate_candle_setup(&self.options)?;
        let _ = &observer;
        #[cfg(feature = "candle")]
        {
            let chunk_count = request.chunks.len();
            let model_id = request.model_id.clone();
            observer.observe(TranscriptionPipelineEvent::ModelLoadStart {
                stage: "asr".to_string(),
                provider: self.provider_id().to_string(),
                model_id: model_id.clone(),
            });
            let mut response = native_whisper::transcribe_with_load_observer(
                &self.options,
                request,
                |duration| {
                    observer.observe(TranscriptionPipelineEvent::ModelLoadEnd {
                        stage: "asr".to_string(),
                        provider: "candle-whisper".to_string(),
                        model_id: model_id.clone(),
                        duration_seconds: duration,
                    });
                },
            )?;
            extend_missing_candle_batch_diagnostics(
                &mut response.diagnostics,
                &self.options,
                chunk_count,
            );
            Ok(response)
        }
        #[cfg(not(feature = "candle"))]
        {
            Err(unsupported_runtime(format!(
                "Candle Whisper requested for `{}` but the binary lacks the `candle` feature; {}; build with `candle` for native execution and `model-bundles` for Hugging Face cache resolution",
                request.model_id,
                candle_whisper_setup_context(&self.options)
            )))
        }
    }
}

/// Candle Whisper provider that keeps a compatible native model session loaded
/// across transcription requests.
#[derive(Default)]
pub struct ReusableCandleWhisperTranscriber {
    pub options: CandleWhisperOptions,
    #[cfg(feature = "candle")]
    session: Option<native_whisper::ReusableCandleWhisperSession>,
}

impl ReusableCandleWhisperTranscriber {
    pub fn new(options: CandleWhisperOptions) -> Self {
        Self {
            options,
            #[cfg(feature = "candle")]
            session: None,
        }
    }
}

impl std::fmt::Debug for ReusableCandleWhisperTranscriber {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReusableCandleWhisperTranscriber")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl AudioTranscriptionProvider for ReusableCandleWhisperTranscriber {
    fn provider_id(&self) -> &str {
        "candle-whisper"
    }

    fn transcribe(&mut self, request: AsrRequest) -> Result<AsrResponse> {
        let mut observer = NoopTranscriptionPipelineObserver;
        self.transcribe_with_observer(request, &mut observer)
    }

    fn transcribe_with_observer(
        &mut self,
        request: AsrRequest,
        observer: &mut dyn TranscriptionPipelineObserver,
    ) -> Result<AsrResponse> {
        validate_asr_request(&request)?;
        validate_candle_setup(&self.options)?;
        let _ = &observer;
        #[cfg(feature = "candle")]
        {
            let chunk_count = request.chunks.len();
            let model_id = request.model_id.clone();
            let mut response = native_whisper::ReusableCandleWhisperSession::transcribe(
                &mut self.session,
                &self.options,
                request,
                |event| match event {
                    native_whisper::ReusableCandleWhisperSessionEvent::LoadStart => {
                        observer.observe(TranscriptionPipelineEvent::ModelLoadStart {
                            stage: "asr".to_string(),
                            provider: "candle-whisper".to_string(),
                            model_id: model_id.clone(),
                        });
                    }
                    native_whisper::ReusableCandleWhisperSessionEvent::LoadEnd {
                        duration_seconds,
                    } => {
                        observer.observe(TranscriptionPipelineEvent::ModelLoadEnd {
                            stage: "asr".to_string(),
                            provider: "candle-whisper".to_string(),
                            model_id: model_id.clone(),
                            duration_seconds,
                        });
                    }
                    native_whisper::ReusableCandleWhisperSessionEvent::Reuse => {
                        observer.observe(TranscriptionPipelineEvent::ModelReuse {
                            stage: "asr".to_string(),
                            provider: "candle-whisper".to_string(),
                            model_id: model_id.clone(),
                        });
                    }
                },
            )?;
            extend_missing_candle_batch_diagnostics(
                &mut response.diagnostics,
                &self.options,
                chunk_count,
            );
            Ok(response)
        }
        #[cfg(not(feature = "candle"))]
        {
            Err(unsupported_runtime(format!(
                "Candle Whisper requested for `{}` but the binary lacks the `candle` feature; {}; build with `candle` for native execution and `model-bundles` for Hugging Face cache resolution",
                request.model_id,
                candle_whisper_setup_context(&self.options)
            )))
        }
    }
}

fn candle_whisper_setup_context(options: &CandleWhisperOptions) -> String {
    let model_location = options
        .model_bundle
        .as_ref()
        .map(|path| format!("--whisper-bundle={}", path.display()))
        .or_else(|| {
            options
                .model_dir
                .as_ref()
                .map(|path| format!("--model-dir={}", path.display()))
        })
        .unwrap_or_else(|| "--model-dir=<default huggingface cache>".to_string());
    format!("{model_location}; cache-only={}", options.model_cache_only)
}

/// Native whisper.cpp compatibility provider.
#[derive(Debug, Clone, Default)]
pub struct WhisperCppTranscriber {
    pub options: WhisperCppProviderOptions,
}

impl AudioTranscriptionProvider for WhisperCppTranscriber {
    fn provider_id(&self) -> &str {
        "whisper-cpp"
    }

    fn transcribe(&mut self, _request: AsrRequest) -> Result<AsrResponse> {
        let Some(model_path) = &self.options.model_path else {
            return Err(setup_error("required whisper.cpp model path is missing"));
        };
        if !model_path.exists() {
            return Err(setup_error(format!(
                "required whisper.cpp model `{}` is missing",
                model_path.display()
            )));
        }
        Err(unsupported_runtime(
            "whisper.cpp compatibility provider is not the primary transcription path",
        ))
    }
}

/// Feature-gated CTC forced aligner.
#[derive(Debug, Clone, Default)]
pub struct CtcForcedAligner {
    pub options: AlignmentOptions,
}

impl ForcedAlignmentProvider for CtcForcedAligner {
    fn provider_id(&self) -> &str {
        "ctc-forced-aligner"
    }

    fn align(&mut self, request: AlignmentRequest) -> Result<AlignmentResponse> {
        let mut observer = NoopTranscriptionPipelineObserver;
        self.align_with_observer(request, &mut observer)
    }

    fn align_with_observer(
        &mut self,
        request: AlignmentRequest,
        observer: &mut dyn TranscriptionPipelineObserver,
    ) -> Result<AlignmentResponse> {
        let _ = &observer;
        #[cfg(feature = "alignment")]
        {
            ctc_alignment::align_with_observer(&self.options, request, observer)
        }
        #[cfg(not(feature = "alignment"))]
        {
            validate_alignment_setup(&self.options)?;
            Err(unsupported_runtime(format!(
            "CTC alignment execution for `{}` is planned behind the alignment provider; default tests use mock alignment providers",
            request.model_id
        )))
        }
    }
}

/// External command provider for Python WhisperX.
#[derive(Debug, Clone, Default)]
pub struct WhisperXCommandTranscriber;

impl WhisperXCommandTranscriber {
    pub fn transcribe_pipeline(
        &mut self,
        request: TranscriptionPipelineRequest,
    ) -> Result<TranscriptionPipelineResponse> {
        match request.provider {
            TranscriptionProviderSelection::ExternalWhisperX(options) => {
                run_whisperx_command(request.source.path()?, options)
            }
            other => Err(invalid_request(format!(
                "whisperx-command cannot run provider `{}`",
                other.provider_id()
            ))),
        }
    }
}

fn run_native_transcription_pipeline(
    request: TranscriptionPipelineRequest,
    vad: &mut dyn TranscriptionVadProvider,
    asr: &mut dyn AudioTranscriptionProvider,
    diarization_provider: Option<&mut dyn TranscriptDiarizationProvider>,
) -> Result<TranscriptionPipelineResponse> {
    if !request.alignment.enabled {
        return run_transcription_pipeline(request, vad, asr, None, diarization_provider);
    }

    let mut aligner = CtcForcedAligner {
        options: request.alignment.clone(),
    };
    run_transcription_pipeline(
        request,
        vad,
        asr,
        Some(&mut aligner as &mut dyn ForcedAlignmentProvider),
        diarization_provider,
    )
}

/// Runs a transcription request with the selected primary provider.
pub fn transcribe(request: TranscriptionPipelineRequest) -> Result<TranscriptionPipelineResponse> {
    match &request.provider {
        TranscriptionProviderSelection::ExternalWhisperX(_) => {
            let mut provider = WhisperXCommandTranscriber;
            provider.transcribe_pipeline(request)
        }
        TranscriptionProviderSelection::CandleWhisper(options) => {
            let mut vad = EnergyVadTranscriptionProvider;
            let mut asr = CandleWhisperTranscriber::new(options.clone());
            #[cfg(feature = "diarization")]
            {
                let mut diarizer = NativeSpeakerDiarizationProvider;
                let diarization_provider = request
                    .diarization
                    .enabled
                    .then_some(&mut diarizer as &mut dyn TranscriptDiarizationProvider);
                run_native_transcription_pipeline(request, &mut vad, &mut asr, diarization_provider)
            }
            #[cfg(not(feature = "diarization"))]
            {
                run_native_transcription_pipeline(request, &mut vad, &mut asr, None)
            }
        }
        TranscriptionProviderSelection::WhisperCpp(options) => {
            let mut vad = EnergyVadTranscriptionProvider;
            let mut asr = WhisperCppTranscriber {
                options: options.clone(),
            };
            #[cfg(feature = "diarization")]
            {
                let mut diarizer = NativeSpeakerDiarizationProvider;
                let diarization_provider = request
                    .diarization
                    .enabled
                    .then_some(&mut diarizer as &mut dyn TranscriptDiarizationProvider);
                run_native_transcription_pipeline(request, &mut vad, &mut asr, diarization_provider)
            }
            #[cfg(not(feature = "diarization"))]
            {
                run_native_transcription_pipeline(request, &mut vad, &mut asr, None)
            }
        }
    }
}

/// Runs the provider-agnostic native transcription pipeline.
pub fn run_transcription_pipeline(
    request: TranscriptionPipelineRequest,
    vad_provider: &mut dyn TranscriptionVadProvider,
    asr_provider: &mut dyn AudioTranscriptionProvider,
    alignment_provider: Option<&mut dyn ForcedAlignmentProvider>,
    diarization_provider: Option<&mut dyn TranscriptDiarizationProvider>,
) -> Result<TranscriptionPipelineResponse> {
    let mut observer = NoopTranscriptionPipelineObserver;
    run_transcription_pipeline_with_observer(
        request,
        vad_provider,
        asr_provider,
        alignment_provider,
        diarization_provider,
        &mut observer,
    )
}

/// Runs the provider-agnostic native transcription pipeline and emits phase events.
pub fn run_transcription_pipeline_with_observer(
    request: TranscriptionPipelineRequest,
    vad_provider: &mut dyn TranscriptionVadProvider,
    asr_provider: &mut dyn AudioTranscriptionProvider,
    alignment_provider: Option<&mut dyn ForcedAlignmentProvider>,
    diarization_provider: Option<&mut dyn TranscriptDiarizationProvider>,
    observer: &mut dyn TranscriptionPipelineObserver,
) -> Result<TranscriptionPipelineResponse> {
    observer.observe(TranscriptionPipelineEvent::ValidationStart);
    validate_batch_options_for_provider(&request.provider)?;
    validate_task_options_for_request(&request)?;
    let provider = request.provider.provider_id().to_string();
    let model_id = request.provider.model_id().to_string();
    let task = request.provider.task();
    observer.observe(TranscriptionPipelineEvent::DecodeStart);
    let decode_started = Instant::now();
    let audio = LoadedAudio::mono_16khz_from_source(&request.source)?;
    observer.observe(TranscriptionPipelineEvent::DecodeEnd {
        duration_seconds: decode_started.elapsed().as_secs_f64(),
        samples: audio.samples.len(),
    });
    observer.observe(TranscriptionPipelineEvent::VadStart {
        provider: vad_provider.provider_id().to_string(),
    });
    let vad_response = if request.vad.enabled {
        vad_provider.detect_speech(VadRequest {
            audio: audio.clone(),
            options: request.vad.clone(),
        })?
    } else {
        VadResponse {
            segments: vec![SpeechActivitySegment::new(
                0.0,
                audio.duration_seconds().max(1.0 / audio.sample_rate as f64),
                1.0,
            )?],
            diagnostics: vec!["VAD disabled; using full source as one ASR chunk".to_string()],
        }
    };
    observer.observe(TranscriptionPipelineEvent::VadEnd {
        segments: vad_response.segments.len(),
        windows: diagnostic_usize(&vad_response.diagnostics, "pyannoteVadWindows")
            .or_else(|| diagnostic_usize(&vad_response.diagnostics, "sileroVadWindows")),
    });

    let language = provider_language(&request.provider);
    observer.observe(TranscriptionPipelineEvent::AsrStart {
        model_id: model_id.clone(),
    });
    let mut asr_response = asr_provider.transcribe_with_observer(
        AsrRequest {
            audio: audio.clone(),
            chunks: vad_response.segments.clone(),
            task,
            language: language.clone(),
            model_id: model_id.clone(),
        },
        observer,
    )?;
    observer.observe(TranscriptionPipelineEvent::AsrEnd {
        segments: asr_response.transcript.segments.len(),
    });
    if !asr_response
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.starts_with("batchChunks="))
    {
        if let TranscriptionProviderSelection::CandleWhisper(options) = &request.provider {
            extend_missing_candle_batch_diagnostics(
                &mut asr_response.diagnostics,
                options,
                vad_response.segments.len(),
            );
        }
    }
    let mut transcript = normalize_transcription_contract(asr_response.transcript)
        .map_err(|error| model_output_mismatch(error.to_string()))?;
    offset_chunk_local_segments(&mut transcript, &vad_response.segments)?;

    let mut diagnostics = vec![format!("asrTask={}", task.as_whisper_task())];
    if task == TranscriptionTask::Translate {
        diagnostics.push("translationRuntime=whisper-task".to_string());
        if let Some(language) = task.output_language_hint() {
            diagnostics.push(format!("translationTargetLanguage={language}"));
        }
    }
    diagnostics.extend(vad_response.diagnostics);
    diagnostics.extend(asr_response.diagnostics);
    let mut alignment_summary = None;
    if request.alignment.enabled {
        let provider = alignment_provider.ok_or_else(|| {
            setup_error("alignment requested but no alignment provider is available")
        })?;
        observer.observe(TranscriptionPipelineEvent::AlignmentStart {
            model_id: request.alignment.model_id.clone(),
        });
        let alignment_response = provider.align_with_observer(
            AlignmentRequest {
                audio: audio.clone(),
                transcript: transcript.clone(),
                language: language.clone(),
                model_id: request.alignment.model_id.clone(),
            },
            observer,
        )?;
        observer.observe(TranscriptionPipelineEvent::AlignmentEnd {
            words: alignment_response.words.len(),
        });
        apply_alignment_words(&mut transcript, &alignment_response.words)?;
        apply_alignment_chars(&mut transcript, &alignment_response.chars)?;
        alignment_summary = Some(AlignmentSummary {
            provider: provider.provider_id().to_string(),
            model_id: alignment_response.model_id,
            word_count: alignment_response.words.len(),
        });
        diagnostics.extend(alignment_response.diagnostics);
    }

    let mut diarization = None;
    if request.diarization.enabled {
        validate_diarization_options(&request.diarization)?;
        let provider = diarization_provider.ok_or_else(|| {
            setup_error("diarization requested but no diarization provider is available")
        })?;
        observer.observe(TranscriptionPipelineEvent::DiarizationStart {
            provider: diarization_progress_provider(provider.provider_id(), &request.diarization),
        });
        let response = provider.diarize(audio, &transcript, &request.diarization)?;
        observer.observe(TranscriptionPipelineEvent::DiarizationEnd {
            speakers: diarization_speaker_count(&response),
            segments: response.segments.len(),
        });
        diagnostics.extend(diarization_diagnostics(
            provider.provider_id(),
            &response,
            &request.diarization,
        ));
        diagnostics.extend(response.diagnostics.clone());
        transcript = audio_analysis_speakers::assign_speakers_to_transcript_with_policy(
            &transcript,
            &response,
            request.diarization.assignment_policy,
        )?;
        diarization = Some(response);
    }

    transcript = normalize_transcription_contract(transcript)
        .map_err(|error| model_output_mismatch(error.to_string()))?;
    transcript
        .validate_strict()
        .map_err(|error| model_output_mismatch(error.to_string()))?;

    Ok(TranscriptionPipelineResponse {
        accepted: true,
        operation: "audio.transcription.transcribe".to_string(),
        provider,
        model_id,
        transcript,
        vad_segments: vad_response.segments,
        alignment: alignment_summary,
        diarization,
        artifacts: Vec::new(),
        diagnostics,
    })
}

/// Parses existing WhisperX JSON without running external tools.
pub fn import_whisperx_json(bytes: &[u8]) -> Result<TranscriptionContract> {
    text_transcripts::parse_whisperx_json(bytes)
        .map_err(|error| DetectError::InvalidArgument(error.to_string()))
}

/// Returns provider plans.
pub fn transcription_provider_plans() -> Vec<TranscriptionProviderPlan> {
    vec![
        candle_whisper_provider_plan(),
        whisper_cpp_provider_plan(),
        whisperx_provider_plan(),
    ]
}

/// Returns the primary Candle Whisper provider plan.
pub fn candle_whisper_provider_plan() -> TranscriptionProviderPlan {
    TranscriptionProviderPlan {
        provider_id: "candle-whisper".to_string(),
        external_runtime: false,
        wasm_supported: false,
        primary: true,
        setup: vec![
            "Provide an offline model bundle with config.json, generation_config.json, tokenizer.json, preprocessor_config.json, and model.safetensors.".to_string(),
            "Build with feature `candle`; add `cuda` for CUDA device execution.".to_string(),
        ],
        diagnostics: vec![
            "Candle Whisper is the primary planned Rust-native ASR and translate-to-English provider.".to_string(),
            "Set task=translate for native Whisper translation; wav2vec2/CTC alignment is not supported for translated output.".to_string(),
            "Default tests do not download models or require CUDA.".to_string(),
            "Default decodeRuntime=autoregressiveKvCache preserves the safe per-window KV-cache path.".to_string(),
            "decodeRuntime=activeRowTensorBatch enables true tensor-batched active-row decode for eligible multi-window native Candle Whisper input.".to_string(),
        ],
    }
}

/// Returns the whisper.cpp compatibility provider plan.
pub fn whisper_cpp_provider_plan() -> TranscriptionProviderPlan {
    TranscriptionProviderPlan {
        provider_id: "whisper-cpp".to_string(),
        external_runtime: false,
        wasm_supported: false,
        primary: false,
        setup: vec!["Provide a local whisper.cpp model path explicitly.".to_string()],
        diagnostics: vec![
            "whisper.cpp is retained as a native compatibility provider, not the primary ASR path."
                .to_string(),
            "Whisper translate is not supported through this provider in this crate.".to_string(),
        ],
    }
}

/// Returns the external WhisperX provider plan.
pub fn whisperx_provider_plan() -> TranscriptionProviderPlan {
    TranscriptionProviderPlan {
        provider_id: "whisperx-command".to_string(),
        external_runtime: true,
        wasm_supported: false,
        primary: false,
        setup: vec![
            "Install whisperx in the active Python environment.".to_string(),
            "Ensure ffmpeg is available on PATH.".to_string(),
            "Set HF_TOKEN before diarization requests.".to_string(),
        ],
        diagnostics: vec![
            "WhisperX execution is opt-in and never required by default tests.".to_string(),
            "The compatibility command path forwards task=transcribe or task=translate to Python WhisperX.".to_string(),
            "Transcript normalization and WhisperX JSON import are delegated to text-transcripts."
                .to_string(),
        ],
    }
}

fn provider_language(provider: &TranscriptionProviderSelection) -> Option<String> {
    match provider {
        TranscriptionProviderSelection::CandleWhisper(options) => options.language.clone(),
        TranscriptionProviderSelection::WhisperCpp(options) => options.language.clone(),
        TranscriptionProviderSelection::ExternalWhisperX(options) => options.language.clone(),
    }
}

fn validate_batch_options_for_provider(provider: &TranscriptionProviderSelection) -> Result<()> {
    if let TranscriptionProviderSelection::CandleWhisper(options) = provider {
        validate_candle_batch_options(options)?;
    }
    Ok(())
}

fn validate_task_options_for_request(request: &TranscriptionPipelineRequest) -> Result<()> {
    let task = request.provider.task();
    if task == TranscriptionTask::Translate && request.alignment.enabled {
        return Err(invalid_request(
            "native Whisper translation output cannot be wav2vec2/CTC-aligned against source-language audio in this implementation",
        ));
    }
    if matches!(
        request.provider,
        TranscriptionProviderSelection::WhisperCpp(_)
    ) && task == TranscriptionTask::Translate
    {
        return Err(invalid_request(
            "Whisper translate is not supported by the whisper.cpp provider in this crate; use candleWhisper or externalWhisperX",
        ));
    }
    Ok(())
}

pub(crate) fn validate_candle_batch_options(options: &CandleWhisperOptions) -> Result<()> {
    if options.max_batch_size == Some(0) {
        return Err(invalid_request(
            "Candle Whisper max_batch_size must be greater than zero",
        ));
    }
    if matches!(
        options.decode_runtime,
        CandleWhisperDecodeRuntime::ActiveRowTensorBatch
    ) {
        if !options.batch_chunks {
            return Err(invalid_request(
                "Candle Whisper activeRowTensorBatch decodeRuntime requires batch_chunks=true",
            ));
        }
        if options.max_batch_size == Some(1) {
            return Err(invalid_request(
                "Candle Whisper activeRowTensorBatch decodeRuntime requires max_batch_size greater than one or unbounded batching",
            ));
        }
    }
    Ok(())
}

pub(crate) fn candle_batch_count(options: &CandleWhisperOptions, chunk_count: usize) -> usize {
    if chunk_count == 0 {
        return 0;
    }
    if !options.batch_chunks {
        return chunk_count;
    }
    match options.max_batch_size {
        Some(max_batch_size) => chunk_count.div_ceil(max_batch_size),
        None => 1,
    }
}

pub(crate) fn candle_batch_diagnostics(
    options: &CandleWhisperOptions,
    chunk_count: usize,
) -> Vec<String> {
    vec![
        format!("chunkCount={chunk_count}"),
        format!("batchChunks={}", options.batch_chunks),
        format!(
            "maxBatchSize={}",
            options
                .max_batch_size
                .map(|value| value.to_string())
                .unwrap_or_else(|| "unbounded".to_string())
        ),
        format!("batchCount={}", candle_batch_count(options, chunk_count)),
        format!("batchExecution={CANDLE_WHISPER_AUTOREGRESSIVE_KV_CACHE_EXECUTION}"),
    ]
}

fn extend_missing_candle_batch_diagnostics(
    diagnostics: &mut Vec<String>,
    options: &CandleWhisperOptions,
    chunk_count: usize,
) {
    for diagnostic in candle_batch_diagnostics(options, chunk_count) {
        let Some((key, _)) = diagnostic.split_once('=') else {
            diagnostics.push(diagnostic);
            continue;
        };
        let prefix = format!("{key}=");
        if diagnostics.iter().any(|item| item.starts_with(&prefix)) {
            continue;
        }
        diagnostics.push(diagnostic);
    }
}

pub(crate) fn validate_asr_request(request: &AsrRequest) -> Result<()> {
    native_audio::validate_loaded_audio(&request.audio)?;
    if request.chunks.is_empty() {
        return Err(invalid_request(
            "ASR request must contain at least one speech chunk",
        ));
    }
    let duration = request.audio.duration_seconds();
    let tolerance = 1.0 / request.audio.sample_rate as f64;
    for chunk in &request.chunks {
        chunk.validate()?;
        if chunk.end_seconds > duration + tolerance {
            return Err(invalid_request(format!(
                "speech chunk end {:.6} exceeds audio duration {:.6}",
                chunk.end_seconds, duration
            )));
        }
    }
    Ok(())
}

fn validate_diarization_options(options: &DiarizationOptions) -> Result<()> {
    options.speaker.validate()
}

#[cfg(feature = "diarization")]
fn speech_spans_from_transcript(
    transcript: &TranscriptionContract,
    audio_duration_seconds: f64,
) -> Result<Vec<audio_analysis_speakers::SpeechSpan>> {
    const AUDIO_DURATION_EPSILON: f64 = 1e-6;

    let has_timed_words = transcript.segments.iter().any(|segment| {
        segment.words.iter().any(|word| {
            !word.text.trim().is_empty()
                && word.start_seconds.is_some()
                && word.end_seconds.is_some()
        })
    });

    let mut spans = Vec::new();
    if has_timed_words {
        for word in transcript
            .segments
            .iter()
            .flat_map(|segment| &segment.words)
        {
            if word.text.trim().is_empty() {
                continue;
            }
            let Some((start, end)) = word.start_seconds.zip(word.end_seconds) else {
                continue;
            };
            spans.push(transcript_timing_span(
                start,
                end,
                audio_duration_seconds,
                AUDIO_DURATION_EPSILON,
            )?);
        }
    } else {
        for segment in &transcript.segments {
            if segment.text.trim().is_empty() {
                continue;
            }
            let Some((start, end)) = segment.start_seconds.zip(segment.end_seconds) else {
                continue;
            };
            spans.push(transcript_timing_span(
                start,
                end,
                audio_duration_seconds,
                AUDIO_DURATION_EPSILON,
            )?);
        }
    }

    merge_transcript_speech_spans(
        spans,
        audio_analysis_speakers::EnergyVadConfig::default().merge_gap_seconds,
    )
}

#[cfg(feature = "diarization")]
fn transcript_timing_span(
    start_seconds: f64,
    end_seconds: f64,
    audio_duration_seconds: f64,
    audio_duration_epsilon: f64,
) -> Result<audio_analysis_speakers::SpeechSpan> {
    if !start_seconds.is_finite() || !end_seconds.is_finite() || !audio_duration_seconds.is_finite()
    {
        return Err(invalid_request(
            "transcript diarization timing values must be finite",
        ));
    }
    if start_seconds < 0.0 || end_seconds <= start_seconds {
        return Err(invalid_request(
            "transcript diarization timing must be non-negative with positive duration",
        ));
    }
    if start_seconds > audio_duration_seconds + audio_duration_epsilon
        || end_seconds > audio_duration_seconds + audio_duration_epsilon
    {
        return Err(invalid_request(format!(
            "transcript diarization timing end {:.6} exceeds audio duration {:.6}",
            end_seconds, audio_duration_seconds
        )));
    }
    let end_seconds = if end_seconds > audio_duration_seconds {
        audio_duration_seconds
    } else {
        end_seconds
    };
    audio_analysis_speakers::SpeechSpan::new(start_seconds, end_seconds, 1.0)
}

#[cfg(feature = "diarization")]
fn merge_transcript_speech_spans(
    mut spans: Vec<audio_analysis_speakers::SpeechSpan>,
    merge_gap_seconds: f64,
) -> Result<Vec<audio_analysis_speakers::SpeechSpan>> {
    spans.sort_by(|left, right| left.start_seconds.total_cmp(&right.start_seconds));
    let mut merged: Vec<audio_analysis_speakers::SpeechSpan> = Vec::new();
    for span in spans {
        if let Some(last) = merged.last_mut() {
            if span.start_seconds - last.end_seconds <= merge_gap_seconds {
                let last_duration = last.duration_seconds();
                let span_duration = span.duration_seconds();
                let total_duration = last_duration + span_duration;
                last.end_seconds = last.end_seconds.max(span.end_seconds);
                last.score = if total_duration > f64::EPSILON {
                    (((last.score as f64 * last_duration) + (span.score as f64 * span_duration))
                        / total_duration) as f32
                } else {
                    last.score.max(span.score)
                };
                continue;
            }
        }
        merged.push(span);
    }
    Ok(merged)
}

#[cfg(feature = "diarization")]
#[derive(Debug, Clone)]
struct TranscriptSpeechSpanVad {
    spans: Vec<audio_analysis_speakers::SpeechSpan>,
}

#[cfg(feature = "diarization")]
impl audio_analysis_speakers::VoiceActivityDetector for TranscriptSpeechSpanVad {
    fn detect_speech(
        &mut self,
        _audio: &audio_analysis_speakers::SpeakerAudio<'_>,
    ) -> Result<Vec<audio_analysis_speakers::SpeechSpan>> {
        Ok(self.spans.clone())
    }
}

#[cfg(feature = "diarization")]
fn stable_speaker_predictions_from_diarization(
    segments: Vec<audio_analysis_speakers::DiarizationSegment>,
) -> Result<Vec<SpeakerSegmentPrediction>> {
    let mut unknown_labels: Vec<(String, String)> = Vec::new();
    let mut predictions = Vec::new();
    for segment in segments {
        let speaker = match segment.speaker {
            audio_analysis_speakers::DiarizedSpeaker::Known(id) => id.as_str().to_string(),
            audio_analysis_speakers::DiarizedSpeaker::Unknown(label) => {
                if let Some((_, stable)) = unknown_labels
                    .iter()
                    .find(|(existing, _)| existing == &label)
                {
                    stable.clone()
                } else {
                    let stable = format!("speaker_{}", unknown_labels.len());
                    unknown_labels.push((label, stable.clone()));
                    stable
                }
            }
        };
        predictions.push(normalize_speaker_prediction(SpeakerSegmentPrediction {
            speaker,
            start_seconds: segment.start_seconds as f32,
            end_seconds: segment.end_seconds as f32,
            score: Some(segment.score),
        })?);
    }
    merge_speaker_predictions(
        predictions,
        audio_analysis_speakers::EnergyVadConfig::default().merge_gap_seconds as f32,
    )
}

#[cfg(feature = "diarization")]
fn normalize_speaker_prediction(
    mut segment: SpeakerSegmentPrediction,
) -> Result<SpeakerSegmentPrediction> {
    segment.speaker = segment.speaker.trim().to_string();
    if segment.speaker.is_empty() {
        return Err(invalid_request("speaker label must not be empty"));
    }
    if !segment.start_seconds.is_finite() || !segment.end_seconds.is_finite() {
        return Err(invalid_request("speaker segment timestamps must be finite"));
    }
    if segment.end_seconds < segment.start_seconds {
        return Err(invalid_request(
            "speaker segment end_seconds must be greater than or equal to start_seconds",
        ));
    }
    segment.score = segment
        .score
        .and_then(|score| score.is_finite().then(|| score.clamp(0.0, 1.0)));
    Ok(segment)
}

#[cfg(feature = "diarization")]
fn merge_speaker_predictions(
    segments: Vec<SpeakerSegmentPrediction>,
    merge_gap_seconds: f32,
) -> Result<Vec<SpeakerSegmentPrediction>> {
    let mut merged: Vec<SpeakerSegmentPrediction> = Vec::new();
    for segment in segments {
        if let Some(last) = merged.last_mut() {
            if last.speaker == segment.speaker
                && segment.start_seconds - last.end_seconds <= merge_gap_seconds
            {
                let last_duration = (last.end_seconds - last.start_seconds).max(0.0);
                let segment_duration = (segment.end_seconds - segment.start_seconds).max(0.0);
                let total_duration = last_duration + segment_duration;
                last.end_seconds = segment.end_seconds;
                last.score = match (last.score, segment.score) {
                    (Some(left), Some(right)) if total_duration > f32::EPSILON => {
                        Some(((left * last_duration) + (right * segment_duration)) / total_duration)
                    }
                    (Some(left), Some(right)) => Some(left.max(right)),
                    (Some(left), None) => Some(left),
                    (None, Some(right)) => Some(right),
                    (None, None) => None,
                };
                continue;
            }
        }
        merged.push(segment);
    }
    Ok(merged)
}

fn diarization_speaker_count(response: &SpeakerDiarizationResponse) -> usize {
    response
        .segments
        .iter()
        .map(|segment| segment.speaker.as_str())
        .collect::<std::collections::BTreeSet<_>>()
        .len()
}

fn diagnostic_usize(diagnostics: &[String], key: &str) -> Option<usize> {
    let prefix = format!("{key}=");
    diagnostics
        .iter()
        .find_map(|diagnostic| diagnostic.strip_prefix(&prefix))
        .and_then(|value| value.parse().ok())
}

fn diarization_progress_provider(provider_id: &str, options: &DiarizationOptions) -> String {
    if options
        .model_id
        .trim()
        .to_ascii_lowercase()
        .starts_with("pyannote/")
    {
        "pyannote".to_string()
    } else {
        provider_id.to_string()
    }
}

fn diarization_diagnostics(
    provider_id: &str,
    response: &SpeakerDiarizationResponse,
    options: &DiarizationOptions,
) -> Vec<String> {
    let speaker_count = diarization_speaker_count(response);
    let mut diagnostics = vec![
        format!("diarizationProvider={provider_id}"),
        format!("diarizationRuntime={}", diarization_runtime_value(response)),
        format!("diarizationModelId={}", response.model_id),
        format!("diarizationSegmentCount={}", response.segments.len()),
        format!("diarizationSpeakerCount={speaker_count}"),
        format!(
            "diarizationAssignmentPolicy={}",
            speaker_assignment_policy_value(options.assignment_policy)
        ),
    ];
    if let Some(min) = options.min_speakers {
        diagnostics.push(format!("diarizationMinSpeakers={min}"));
        if speaker_count < min {
            diagnostics.push(format!(
                "diarizationSpeakerCountBelowRequestedMin={speaker_count}/{min}"
            ));
            if response.segments.len() < min {
                diagnostics.push("diarizationSpeakerBoundsSaturated=true".to_string());
            }
        }
    }
    if let Some(max) = options.max_speakers {
        diagnostics.push(format!("diarizationMaxSpeakers={max}"));
        if speaker_count > max {
            diagnostics.push(format!(
                "diarizationSpeakerCountAboveRequestedMax={speaker_count}/{max}"
            ));
        }
    }
    if options.min_speakers.is_some() || options.max_speakers.is_some() {
        diagnostics.push("diarizationSpeakerBoundsApplied=true".to_string());
    }
    if diarization_runtime_is_heuristic(response) {
        diagnostics.push("diarizationBaseline=heuristic-native".to_string());
    } else if diarization_runtime_value(response) == "onnx" {
        diagnostics.push("speakerEmbeddingProvider=onnx".to_string());
        if let Some(dimension) = options.speaker_embedding_dimension {
            diagnostics.push(format!("speakerEmbeddingDimension={dimension}"));
        }
        diagnostics.push("diarizationBaseline=false".to_string());
    }
    diagnostics
}

fn diarization_runtime_value(response: &SpeakerDiarizationResponse) -> &'static str {
    match response.runtime {
        audio_analysis_speakers::AudioRuntime::Onnx => "onnx",
        audio_analysis_speakers::AudioRuntime::Candle => "candle",
        audio_analysis_speakers::AudioRuntime::WhisperCpp => "whisper_cpp",
        audio_analysis_speakers::AudioRuntime::Demucs => "demucs",
        audio_analysis_speakers::AudioRuntime::External => "external",
        audio_analysis_speakers::AudioRuntime::Spectral => "spectral",
        audio_analysis_speakers::AudioRuntime::Heuristic => "heuristic",
        audio_analysis_speakers::AudioRuntime::Imported => "imported",
    }
}

fn diarization_runtime_is_heuristic(response: &SpeakerDiarizationResponse) -> bool {
    response.runtime == audio_analysis_speakers::AudioRuntime::Heuristic
}

fn speaker_assignment_policy_value(policy: SpeakerAssignmentPolicy) -> &'static str {
    match policy {
        SpeakerAssignmentPolicy::Majority => "majority",
        SpeakerAssignmentPolicy::NearestStart => "nearestStart",
        SpeakerAssignmentPolicy::StrictContained => "strictContained",
    }
}

pub(crate) fn normalize_samples_source(
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
    source: Option<String>,
) -> Result<LoadedAudio> {
    if sample_rate == 0 || channels == 0 {
        return Err(DetectError::InvalidAudioFormat {
            sample_rate,
            channels,
        });
    }
    if samples.is_empty() {
        return Err(invalid_request("empty audio"));
    }
    if !samples.len().is_multiple_of(channels as usize) {
        return Err(invalid_request(
            "sample count must contain complete interleaved frames",
        ));
    }
    if samples.iter().any(|sample| !sample.is_finite()) {
        return Err(invalid_request("audio samples must be finite"));
    }
    let mono = if channels == 1 {
        samples.to_vec()
    } else {
        samples
            .chunks_exact(channels as usize)
            .map(|frame| frame.iter().sum::<f32>() / channels as f32)
            .collect::<Vec<_>>()
    };
    let samples = if sample_rate == 16_000 {
        mono
    } else {
        resample_linear(&mono, sample_rate, 16_000)
    };
    Ok(LoadedAudio {
        samples,
        sample_rate: 16_000,
        channels: 1,
        source,
    })
}

pub(crate) fn resample_linear(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if samples.is_empty() || from_rate == to_rate {
        return samples.to_vec();
    }
    let output_len = ((samples.len() as f64 * to_rate as f64) / from_rate as f64)
        .round()
        .max(1.0) as usize;
    (0..output_len)
        .map(|index| {
            let position = index as f64 * from_rate as f64 / to_rate as f64;
            let left = position.floor() as usize;
            let right = (left + 1).min(samples.len() - 1);
            let frac = (position - left as f64) as f32;
            samples[left] * (1.0 - frac) + samples[right] * frac
        })
        .collect()
}

fn energy_vad_segments(
    audio: &LoadedAudio,
    options: &VadOptions,
) -> Result<Vec<SpeechActivitySegment>> {
    validate_vad_options(options)?;
    let frame_size = seconds_to_samples(options.frame_seconds, audio.sample_rate)?;
    let hop_size = seconds_to_samples(options.hop_seconds, audio.sample_rate)?;
    let mut active = Vec::new();
    let mut start = 0;
    while start < audio.samples.len() {
        let end = (start + frame_size).min(audio.samples.len());
        let score = rms(&audio.samples[start..end]);
        if score >= options.rms_threshold {
            active.push(SpeechActivitySegment::new(
                start as f64 / audio.sample_rate as f64,
                end as f64 / audio.sample_rate as f64,
                score,
            )?);
        }
        if start + hop_size >= audio.samples.len() {
            break;
        }
        start += hop_size;
    }
    let duration = audio.duration_seconds();
    let mut merged = merge_speech_segments(active, options.merge_gap_seconds)?;
    for segment in &mut merged {
        segment.start_seconds = (segment.start_seconds - options.padding_seconds).max(0.0);
        segment.end_seconds = (segment.end_seconds + options.padding_seconds).min(duration);
    }
    merged = merge_speech_segments(merged, options.merge_gap_seconds)?;
    let filtered = merged
        .into_iter()
        .filter(|segment| segment.end_seconds - segment.start_seconds >= options.min_speech_seconds)
        .flat_map(|segment| split_max_chunk(segment, options.max_chunk_seconds))
        .collect::<Vec<_>>();
    if filtered.is_empty() {
        return Ok(vec![SpeechActivitySegment::new(
            0.0,
            duration.max(1.0 / audio.sample_rate as f64),
            0.0,
        )?]);
    }
    Ok(filtered)
}

fn validate_vad_options(options: &VadOptions) -> Result<()> {
    if !options.rms_threshold.is_finite() || options.rms_threshold < 0.0 {
        return Err(invalid_request(
            "VAD RMS threshold must be finite and non-negative",
        ));
    }
    for (name, value, positive) in [
        ("frameSeconds", options.frame_seconds, true),
        ("hopSeconds", options.hop_seconds, true),
        ("minSpeechSeconds", options.min_speech_seconds, false),
        ("paddingSeconds", options.padding_seconds, false),
        ("mergeGapSeconds", options.merge_gap_seconds, false),
        ("maxChunkSeconds", options.max_chunk_seconds, true),
    ] {
        if !value.is_finite() || (positive && value <= 0.0) || (!positive && value < 0.0) {
            return Err(invalid_request(format!(
                "VAD option `{name}` has an invalid value"
            )));
        }
    }
    Ok(())
}

fn seconds_to_samples(seconds: f64, sample_rate: u32) -> Result<usize> {
    if !seconds.is_finite() || seconds <= 0.0 || sample_rate == 0 {
        return Err(invalid_request("invalid sample duration"));
    }
    Ok((seconds * sample_rate as f64).round().max(1.0) as usize)
}

fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    (samples.iter().map(|sample| sample * sample).sum::<f32>() / samples.len() as f32).sqrt()
}

fn merge_speech_segments(
    mut segments: Vec<SpeechActivitySegment>,
    merge_gap_seconds: f64,
) -> Result<Vec<SpeechActivitySegment>> {
    segments.sort_by(|left, right| left.start_seconds.total_cmp(&right.start_seconds));
    let mut merged: Vec<SpeechActivitySegment> = Vec::new();
    for segment in segments {
        if let Some(last) = merged.last_mut() {
            if segment.start_seconds - last.end_seconds <= merge_gap_seconds {
                last.end_seconds = last.end_seconds.max(segment.end_seconds);
                last.score = last.score.max(segment.score);
                continue;
            }
        }
        merged.push(segment);
    }
    for segment in &merged {
        segment.validate()?;
    }
    Ok(merged)
}

fn split_max_chunk(
    segment: SpeechActivitySegment,
    max_chunk_seconds: f64,
) -> Vec<SpeechActivitySegment> {
    if segment.end_seconds - segment.start_seconds <= max_chunk_seconds {
        return vec![segment];
    }
    let mut chunks = Vec::new();
    let mut start = segment.start_seconds;
    while start < segment.end_seconds {
        let end = (start + max_chunk_seconds).min(segment.end_seconds);
        if end > start {
            chunks.push(SpeechActivitySegment {
                start_seconds: start,
                end_seconds: end,
                score: segment.score,
            });
        }
        start = end;
    }
    chunks
}

fn offset_chunk_local_segments(
    transcript: &mut TranscriptionContract,
    chunks: &[SpeechActivitySegment],
) -> Result<()> {
    if chunks.is_empty() || transcript.segments.len() != chunks.len() {
        return Ok(());
    }
    for (segment, chunk) in transcript.segments.iter_mut().zip(chunks) {
        if segment
            .attributes
            .get("timing")
            .is_some_and(|value| value == "global")
        {
            continue;
        }
        if let Some(start) = &mut segment.start_seconds {
            *start += chunk.start_seconds;
        }
        if let Some(end) = &mut segment.end_seconds {
            *end += chunk.start_seconds;
        }
        for word in &mut segment.words {
            if let Some(start) = &mut word.start_seconds {
                *start += chunk.start_seconds;
            }
            if let Some(end) = &mut word.end_seconds {
                *end += chunk.start_seconds;
            }
        }
        segment
            .attributes
            .insert("timing".to_string(), "global".to_string());
    }
    Ok(())
}

fn apply_alignment_words(
    transcript: &mut TranscriptionContract,
    words: &[AlignedWord],
) -> Result<()> {
    for aligned in words {
        if !aligned.start_seconds.is_finite()
            || !aligned.end_seconds.is_finite()
            || aligned.end_seconds < aligned.start_seconds
        {
            return Err(model_output_mismatch(
                "alignment output contains invalid word timing",
            ));
        }
        let segment = transcript
            .segments
            .iter_mut()
            .find(|segment| segment.index == aligned.segment_index)
            .ok_or_else(|| model_output_mismatch("alignment output references unknown segment"))?;
        while segment.words.len() <= aligned.word_index {
            segment.words.push(TranscriptWordContract {
                text: String::new(),
                start_seconds: None,
                end_seconds: None,
                confidence: None,
                speaker: None,
                attributes: BTreeMap::new(),
            });
        }
        let word = &mut segment.words[aligned.word_index];
        if word.text.trim().is_empty() {
            word.text = aligned.text.clone();
        }
        word.start_seconds = Some(aligned.start_seconds);
        word.end_seconds = Some(aligned.end_seconds);
        word.confidence = aligned.confidence;
    }
    for segment in &mut transcript.segments {
        let timed_words = segment
            .words
            .iter()
            .filter_map(|word| word.start_seconds.zip(word.end_seconds))
            .collect::<Vec<_>>();
        if let (Some((start, _)), Some((_, end))) = (timed_words.first(), timed_words.last()) {
            segment.start_seconds = Some(*start);
            segment.end_seconds = Some(*end);
        }
    }
    Ok(())
}

fn apply_alignment_chars(
    transcript: &mut TranscriptionContract,
    chars: &[AlignedChar],
) -> Result<()> {
    for aligned in chars {
        if let Some((start, end)) = aligned.start_seconds.zip(aligned.end_seconds) {
            if !start.is_finite() || !end.is_finite() || end < start {
                return Err(model_output_mismatch(
                    "alignment output contains invalid char timing",
                ));
            }
        }
        let segment = transcript
            .segments
            .iter_mut()
            .find(|segment| segment.index == aligned.segment_index)
            .ok_or_else(|| model_output_mismatch("alignment output references unknown segment"))?;
        while segment.chars.len() <= aligned.char_index {
            segment.chars.push(TranscriptCharContract {
                character: String::new(),
                start_seconds: None,
                end_seconds: None,
                confidence: None,
                attributes: BTreeMap::new(),
            });
        }
        let character = &mut segment.chars[aligned.char_index];
        if character.character.is_empty() {
            character.character = aligned.character.clone();
        }
        character.start_seconds = aligned.start_seconds;
        character.end_seconds = aligned.end_seconds;
        character.confidence = aligned.confidence;
    }
    Ok(())
}

fn validate_candle_setup(options: &CandleWhisperOptions) -> Result<()> {
    validate_candle_batch_options(options)?;
    let resolved_device = native_device::resolve_native_device(options.device)?;
    options
        .compute_type
        .resolve_for_device(resolved_device.cuda_active())?;
    if !cfg!(feature = "candle") {
        return Err(unsupported_runtime(format!(
            "Candle Whisper requested but the binary lacks the `candle` feature; {}; build with `candle` for native execution and `model-bundles` for Hugging Face cache resolution",
            candle_whisper_setup_context(options)
        )));
    }
    if let Some(bundle) = &options.model_bundle {
        validate_model_bundle_files(
            bundle,
            &[
                "config.json",
                "generation_config.json",
                "tokenizer.json",
                "preprocessor_config.json",
                "model.safetensors",
            ],
        )
    } else {
        Ok(())
    }
}

#[cfg(not(feature = "alignment"))]
fn validate_alignment_setup(options: &AlignmentOptions) -> Result<()> {
    if !cfg!(feature = "alignment") {
        return Err(unsupported_runtime(
            "CTC alignment requested but the binary lacks the `alignment` feature",
        ));
    }
    if let Some(bundle) = &options.model_bundle {
        validate_model_bundle_files(
            bundle,
            &[
                "config.json",
                "tokenizer.json",
                "preprocessor_config.json",
                "model.safetensors",
            ],
        )
    } else {
        Err(setup_error(
            "required CTC alignment model bundle is missing",
        ))
    }
}

fn validate_model_bundle_files(bundle: &Path, files: &[&str]) -> Result<()> {
    native_bundles::validate_required_bundle_files(bundle, files)
}

fn run_whisperx_command(
    source_path: &Path,
    options: WhisperXCommandOptions,
) -> Result<TranscriptionPipelineResponse> {
    let task = options.task;
    let output_dir = options
        .output_dir
        .clone()
        .unwrap_or_else(default_whisperx_output_dir);
    fs::create_dir_all(&output_dir)?;

    let hf_token = if options.diarize {
        let env_name = options
            .hf_token_env
            .clone()
            .unwrap_or_else(|| "HF_TOKEN".to_string());
        Some(std::env::var(&env_name).map_err(|_| {
            setup_error(format!(
                "diarization requires `{env_name}` to be set before running WhisperX"
            ))
        })?)
    } else {
        None
    };

    let args = whisperx_args(source_path, &output_dir, &options, hf_token.as_deref());
    let child = Command::new(&options.command)
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                setup_error(format!(
                    "WhisperX command `{}` was not found; install whisperx or pass provider.command",
                    options.command.display()
                ))
            } else {
                DetectError::Io(error)
            }
        })?;
    let output = wait_with_optional_timeout(child, &options.command, options.timeout_seconds)?;
    if !output.status.success() {
        return Err(setup_error(format!(
            "WhisperX command `{}` failed: {}",
            options.command.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }

    let (transcript_path, transcript_bytes) =
        whisperx_json_bytes(source_path, &output_dir, &output.stdout).ok_or_else(|| {
            model_output_mismatch(format!(
                "WhisperX completed but no JSON transcript was found in `{}`",
                output_dir.display()
            ))
        })?;
    let mut transcript = import_whisperx_json(&transcript_bytes)?;
    if transcript.source.is_none() {
        transcript.source = Some(source_path.to_string_lossy().into_owned());
    }
    let vad_segments = whisperx_stdout_vad_segments(&output.stdout);
    let artifacts = transcript_path
        .map(|path| TranscriptionArtifact {
            kind: "whisperx-json".to_string(),
            path,
        })
        .into_iter()
        .collect();
    let mut diagnostics = vec![
        format!("asrTask={}", task.as_whisper_task()),
        format!("ran WhisperX output in `{}`", output_dir.display()),
        "parsed WhisperX JSON through text-transcripts".to_string(),
    ];
    if task == TranscriptionTask::Translate {
        diagnostics.push("translationRuntime=whisperx-command".to_string());
        if let Some(language) = task.output_language_hint() {
            diagnostics.push(format!("translationTargetLanguage={language}"));
        }
    }
    if !vad_segments.is_empty() {
        diagnostics.push(format!(
            "whisperxVadSegmentsFromStdout={}",
            vad_segments.len()
        ));
    }
    Ok(TranscriptionPipelineResponse {
        accepted: true,
        operation: "audio.transcription.transcribe".to_string(),
        provider: "whisperx-command".to_string(),
        model_id: options.model,
        transcript,
        vad_segments,
        alignment: None,
        diarization: None,
        artifacts,
        diagnostics,
    })
}

fn whisperx_args(
    source_path: &Path,
    output_dir: &Path,
    options: &WhisperXCommandOptions,
    hf_token: Option<&str>,
) -> Vec<String> {
    let mut args = vec![
        source_path.to_string_lossy().into_owned(),
        "--model".to_string(),
        options.model.clone(),
        "--task".to_string(),
        options.task.as_whisper_task().to_string(),
        "--device".to_string(),
        options.device.as_str().to_string(),
        "--output_format".to_string(),
        "json".to_string(),
        "--output_dir".to_string(),
        output_dir.to_string_lossy().into_owned(),
    ];
    if let Some(language) = &options.language {
        args.extend(["--language".to_string(), language.clone()]);
    }
    if let Some(compute_type) = &options.compute_type {
        args.extend(["--compute_type".to_string(), compute_type.clone()]);
    }
    if let Some(batch_size) = options.batch_size {
        args.extend(["--batch_size".to_string(), batch_size.to_string()]);
    }
    if options.no_align {
        args.push("--no_align".to_string());
    }
    if let Some(align_model) = &options.align_model {
        args.extend(["--align_model".to_string(), align_model.clone()]);
    }
    if let Some(model_dir) = &options.model_dir {
        args.extend([
            "--model_dir".to_string(),
            model_dir.to_string_lossy().into_owned(),
        ]);
    }
    if options.model_cache_only {
        args.push("--model_cache_only".to_string());
    }
    args.extend([
        "--interpolate_method".to_string(),
        options.interpolate_method.as_whisperx_arg().to_string(),
    ]);
    if options.return_char_alignments {
        args.push("--return_char_alignments".to_string());
    }
    if options.diarize {
        args.push("--diarize".to_string());
        if let Some(min_speakers) = options.min_speakers {
            args.extend(["--min_speakers".to_string(), min_speakers.to_string()]);
        }
        if let Some(max_speakers) = options.max_speakers {
            args.extend(["--max_speakers".to_string(), max_speakers.to_string()]);
        }
        if let Some(hf_token) = hf_token {
            args.extend(["--hf_token".to_string(), hf_token.to_string()]);
        }
    }
    args.extend(options.extra_args.clone());
    args
}

fn whisperx_json_bytes(
    source_path: &Path,
    output_dir: &Path,
    stdout: &[u8],
) -> Option<(Option<PathBuf>, Vec<u8>)> {
    let path = find_json_artifact_for_source(output_dir, source_path);
    if let Some(path) = path {
        return fs::read(&path).ok().map(|bytes| (Some(path), bytes));
    }
    serde_json::from_slice::<serde_json::Value>(stdout)
        .ok()
        .map(|_| (None, stdout.to_vec()))
}

fn whisperx_stdout_vad_segments(stdout: &[u8]) -> Vec<SpeechActivitySegment> {
    let stdout = String::from_utf8_lossy(stdout);
    stdout
        .lines()
        .filter_map(|line| {
            let start_marker = "Transcript: [";
            let start = line.find(start_marker)? + start_marker.len();
            let range = line[start..].split_once(']')?.0;
            let (start_seconds, end_seconds) = range.split_once("-->")?;
            let start_seconds = start_seconds.trim().parse::<f64>().ok()?;
            let end_seconds = end_seconds.trim().parse::<f64>().ok()?;
            SpeechActivitySegment::new(start_seconds, end_seconds, 1.0).ok()
        })
        .collect()
}

fn find_json_artifact_for_source(output_dir: &Path, source_path: &Path) -> Option<PathBuf> {
    let expected = source_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .map(|stem| output_dir.join(format!("{stem}.json")));
    if let Some(expected) = expected.filter(|path| path.is_file()) {
        return Some(expected);
    }

    let mut candidates = fs::read_dir(output_dir)
        .ok()?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    candidates.sort();
    candidates.into_iter().next()
}

fn default_whisperx_output_dir() -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    std::env::temp_dir().join(format!("video-analysis-whisperx-{millis}"))
}

fn wait_with_optional_timeout(
    mut child: Child,
    command: &Path,
    timeout_seconds: Option<u64>,
) -> Result<Output> {
    let Some(seconds) = timeout_seconds else {
        return child.wait_with_output().map_err(DetectError::Io);
    };
    let started = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output().map_err(DetectError::Io);
        }
        if started.elapsed() >= Duration::from_secs(seconds) {
            let _ = child.kill();
            let _ = child.wait();
            return Err(timeout_error(format!(
                "WhisperX command `{}` timed out after {seconds} seconds",
                command.display()
            )));
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}

pub(crate) fn setup_error(message: impl Into<String>) -> DetectError {
    DetectError::InvalidArgument(format!("setup_error: {}", message.into()))
}

pub(crate) fn invalid_request(message: impl Into<String>) -> DetectError {
    DetectError::InvalidArgument(format!("invalid_request: {}", message.into()))
}

pub(crate) fn model_output_mismatch(message: impl Into<String>) -> DetectError {
    DetectError::InvalidArgument(format!("model_output_mismatch: {}", message.into()))
}

fn timeout_error(message: impl Into<String>) -> DetectError {
    DetectError::InvalidArgument(format!("timeout: {}", message.into()))
}

pub(crate) fn unsupported_runtime(message: impl Into<String>) -> DetectError {
    DetectError::InvalidArgument(format!("unsupported_runtime: {}", message.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use text_transcripts::TranscriptSegmentContract;

    #[derive(Default)]
    struct MockAsrProvider;

    impl AudioTranscriptionProvider for MockAsrProvider {
        fn provider_id(&self) -> &str {
            "mock-asr"
        }

        fn transcribe(&mut self, request: AsrRequest) -> Result<AsrResponse> {
            let segments = request
                .chunks
                .iter()
                .enumerate()
                .map(|(index, chunk)| {
                    let mut segment = TranscriptSegmentContract::new(index as u64, " hello ");
                    segment.start_seconds = Some(0.0);
                    segment.end_seconds = Some(chunk.end_seconds - chunk.start_seconds);
                    segment
                })
                .collect::<Vec<_>>();
            Ok(AsrResponse {
                model_id: request.model_id,
                language: request
                    .task
                    .output_language_hint()
                    .map(str::to_string)
                    .or(request.language),
                transcript: TranscriptionContract::from_segments(
                    request.audio.source,
                    Some("en".to_string()),
                    segments,
                )
                .map_err(|error| DetectError::InvalidArgument(error.to_string()))?,
                diagnostics: vec!["mock ASR completed".to_string()],
            })
        }
    }

    #[derive(Clone)]
    struct FixedVadProvider {
        segments: Vec<SpeechActivitySegment>,
    }

    impl TranscriptionVadProvider for FixedVadProvider {
        fn provider_id(&self) -> &str {
            "fixed-vad"
        }

        fn detect_speech(&mut self, _request: VadRequest) -> Result<VadResponse> {
            Ok(VadResponse {
                segments: self.segments.clone(),
                diagnostics: vec!["fixed VAD completed".to_string()],
            })
        }
    }

    #[derive(Default)]
    struct MockAlignmentProvider;

    impl ForcedAlignmentProvider for MockAlignmentProvider {
        fn provider_id(&self) -> &str {
            "mock-aligner"
        }

        fn align(&mut self, request: AlignmentRequest) -> Result<AlignmentResponse> {
            Ok(AlignmentResponse {
                model_id: request.model_id,
                words: vec![AlignedWord {
                    segment_index: 0,
                    word_index: 0,
                    text: "hello".to_string(),
                    start_seconds: 0.05,
                    end_seconds: 0.35,
                    confidence: Some(0.91),
                }],
                chars: Vec::new(),
                diagnostics: vec!["mock alignment completed".to_string()],
            })
        }
    }

    #[derive(Default)]
    struct RecordingObserver {
        events: Vec<TranscriptionPipelineEvent>,
    }

    impl TranscriptionPipelineObserver for RecordingObserver {
        fn observe(&mut self, event: TranscriptionPipelineEvent) {
            self.events.push(event);
        }
    }

    struct ObservingAsrProvider;

    impl AudioTranscriptionProvider for ObservingAsrProvider {
        fn provider_id(&self) -> &str {
            "observing-asr"
        }

        fn transcribe(&mut self, request: AsrRequest) -> Result<AsrResponse> {
            MockAsrProvider.transcribe(request)
        }

        fn transcribe_with_observer(
            &mut self,
            request: AsrRequest,
            observer: &mut dyn TranscriptionPipelineObserver,
        ) -> Result<AsrResponse> {
            observer.observe(TranscriptionPipelineEvent::ModelLoadStart {
                stage: "asr".to_string(),
                provider: self.provider_id().to_string(),
                model_id: request.model_id.clone(),
            });
            observer.observe(TranscriptionPipelineEvent::ModelLoadEnd {
                stage: "asr".to_string(),
                provider: self.provider_id().to_string(),
                model_id: request.model_id.clone(),
                duration_seconds: 0.125,
            });
            observer.observe(TranscriptionPipelineEvent::ModelReuse {
                stage: "asr".to_string(),
                provider: self.provider_id().to_string(),
                model_id: request.model_id.clone(),
            });
            self.transcribe(request)
        }
    }

    struct ObservingAlignmentProvider;

    impl ForcedAlignmentProvider for ObservingAlignmentProvider {
        fn provider_id(&self) -> &str {
            "observing-aligner"
        }

        fn align(&mut self, request: AlignmentRequest) -> Result<AlignmentResponse> {
            MockAlignmentProvider.align(request)
        }

        fn align_with_observer(
            &mut self,
            request: AlignmentRequest,
            observer: &mut dyn TranscriptionPipelineObserver,
        ) -> Result<AlignmentResponse> {
            observer.observe(TranscriptionPipelineEvent::ModelLoadStart {
                stage: "alignment".to_string(),
                provider: self.provider_id().to_string(),
                model_id: request.model_id.clone(),
            });
            observer.observe(TranscriptionPipelineEvent::ModelLoadEnd {
                stage: "alignment".to_string(),
                provider: self.provider_id().to_string(),
                model_id: request.model_id.clone(),
                duration_seconds: 0.25,
            });
            self.align(request)
        }
    }

    struct MockDiarizationProvider;

    impl TranscriptDiarizationProvider for MockDiarizationProvider {
        fn provider_id(&self) -> &str {
            "mock-diarization"
        }

        fn diarize(
            &mut self,
            _audio: LoadedAudio,
            _transcript: &TranscriptionContract,
            options: &DiarizationOptions,
        ) -> Result<SpeakerDiarizationResponse> {
            Ok(SpeakerDiarizationResponse {
                accepted: true,
                operation: "audio.speakers.diarize".to_string(),
                model_id: options.model_id.clone(),
                runtime: audio_analysis_speakers::AudioRuntime::Imported,
                segments: vec![SpeakerSegmentPrediction {
                    speaker: "SPEAKER_00".to_string(),
                    start_seconds: 0.0,
                    end_seconds: 1.0,
                    score: Some(0.9),
                }],
                speaker_embeddings: None,
                diagnostics: Vec::new(),
            })
        }
    }

    struct PanickingDiarizationProvider {
        called: bool,
    }

    #[cfg(feature = "diarization")]
    struct MockOnnxDiarizationProvider;

    #[cfg(feature = "diarization")]
    impl TranscriptDiarizationProvider for MockOnnxDiarizationProvider {
        fn provider_id(&self) -> &str {
            "mock-onnx-diarization"
        }

        fn diarize(
            &mut self,
            _audio: LoadedAudio,
            _transcript: &TranscriptionContract,
            options: &DiarizationOptions,
        ) -> Result<SpeakerDiarizationResponse> {
            Ok(SpeakerDiarizationResponse {
                accepted: true,
                operation: "audio.speakers.diarize".to_string(),
                model_id: options.model_id.clone(),
                runtime: audio_analysis_speakers::AudioRuntime::Onnx,
                segments: vec![audio_analysis_speakers::SpeakerSegmentPrediction {
                    speaker: "SPEAKER_ONNX".to_string(),
                    start_seconds: 0.0,
                    end_seconds: 1.0,
                    score: Some(0.9),
                }],
                speaker_embeddings: None,
                diagnostics: Vec::new(),
            })
        }
    }

    impl TranscriptDiarizationProvider for PanickingDiarizationProvider {
        fn provider_id(&self) -> &str {
            "panicking-diarization"
        }

        fn diarize(
            &mut self,
            _audio: LoadedAudio,
            _transcript: &TranscriptionContract,
            _options: &DiarizationOptions,
        ) -> Result<SpeakerDiarizationResponse> {
            self.called = true;
            panic!("diarization provider should not be called for invalid options");
        }
    }

    fn sample_request() -> TranscriptionPipelineRequest {
        let mut samples = vec![0.0; 16_000];
        for sample in &mut samples[1_000..5_000] {
            *sample = 0.1;
        }
        TranscriptionPipelineRequest {
            source: TranscriptionSource::Samples {
                samples,
                sample_rate: 16_000,
                channels: 1,
                source: Some("synthetic".to_string()),
            },
            provider: TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions::default()),
            vad: VadOptions {
                min_speech_seconds: 0.01,
                ..VadOptions::default()
            },
            alignment: AlignmentOptions::default(),
            diarization: DiarizationOptions::default(),
            output: TranscriptionOutputOptions::default(),
        }
    }

    fn batch_test_chunks() -> Vec<SpeechActivitySegment> {
        vec![
            SpeechActivitySegment::new(0.0, 0.20, 1.0).unwrap(),
            SpeechActivitySegment::new(0.20, 0.40, 1.0).unwrap(),
            SpeechActivitySegment::new(0.40, 0.60, 1.0).unwrap(),
        ]
    }

    fn batch_test_request(options: CandleWhisperOptions) -> TranscriptionPipelineRequest {
        TranscriptionPipelineRequest {
            provider: TranscriptionProviderSelection::CandleWhisper(options),
            ..sample_request()
        }
    }

    #[test]
    fn pipeline_observer_receives_model_load_and_reuse_events() {
        let mut request = sample_request();
        request.vad = VadOptions {
            enabled: false,
            ..VadOptions::default()
        };
        request.alignment = AlignmentOptions {
            enabled: true,
            model_id: "facebook/wav2vec2-base-960h".to_string(),
            ..AlignmentOptions::default()
        };
        let mut vad = FixedVadProvider {
            segments: vec![SpeechActivitySegment::new(0.0, 0.5, 1.0).unwrap()],
        };
        let mut asr = ObservingAsrProvider;
        let mut aligner = ObservingAlignmentProvider;
        let mut observer = RecordingObserver::default();

        let response = run_transcription_pipeline_with_observer(
            request,
            &mut vad,
            &mut asr,
            Some(&mut aligner),
            None,
            &mut observer,
        )
        .expect("pipeline should run with observing providers");

        assert!(response.accepted);
        assert!(observer
            .events
            .contains(&TranscriptionPipelineEvent::ModelLoadStart {
                stage: "asr".to_string(),
                provider: "observing-asr".to_string(),
                model_id: "openai/whisper-large-v3-turbo".to_string(),
            }));
        assert!(observer
            .events
            .contains(&TranscriptionPipelineEvent::ModelLoadEnd {
                stage: "asr".to_string(),
                provider: "observing-asr".to_string(),
                model_id: "openai/whisper-large-v3-turbo".to_string(),
                duration_seconds: 0.125,
            }));
        assert!(observer
            .events
            .contains(&TranscriptionPipelineEvent::ModelReuse {
                stage: "asr".to_string(),
                provider: "observing-asr".to_string(),
                model_id: "openai/whisper-large-v3-turbo".to_string(),
            }));
        assert!(observer
            .events
            .contains(&TranscriptionPipelineEvent::ModelLoadStart {
                stage: "alignment".to_string(),
                provider: "observing-aligner".to_string(),
                model_id: "facebook/wav2vec2-base-960h".to_string(),
            }));
        assert!(observer
            .events
            .contains(&TranscriptionPipelineEvent::ModelLoadEnd {
                stage: "alignment".to_string(),
                provider: "observing-aligner".to_string(),
                model_id: "facebook/wav2vec2-base-960h".to_string(),
                duration_seconds: 0.25,
            }));
    }

    #[test]
    fn candle_whisper_options_default_to_automatic_compute_type() {
        assert_eq!(
            CandleWhisperOptions::default().compute_type,
            CandleWhisperComputeType::Automatic
        );
    }

    #[test]
    fn candle_whisper_compute_type_serializes_public_values_and_aliases() {
        let options = CandleWhisperOptions {
            compute_type: CandleWhisperComputeType::Fp16,
            ..CandleWhisperOptions::default()
        };
        let encoded = serde_json::to_value(&options).unwrap();
        assert_eq!(encoded["computeType"], "fp16");

        let decoded: CandleWhisperOptions =
            serde_json::from_value(serde_json::json!({"computeType": "float32"})).unwrap();
        assert_eq!(decoded.compute_type, CandleWhisperComputeType::Fp32);

        let decoded: CandleWhisperOptions =
            serde_json::from_value(serde_json::json!({"computeType": "auto"})).unwrap();
        assert_eq!(decoded.compute_type, CandleWhisperComputeType::Automatic);
    }

    #[test]
    fn candle_whisper_compute_type_resolves_by_device() {
        assert_eq!(
            CandleWhisperComputeType::Automatic
                .resolve_for_device(true)
                .unwrap(),
            CandleWhisperComputeType::Fp16
        );
        assert_eq!(
            CandleWhisperComputeType::Fp16
                .resolve_for_device(true)
                .unwrap(),
            CandleWhisperComputeType::Fp16
        );
        assert_eq!(
            CandleWhisperComputeType::Fp32
                .resolve_for_device(true)
                .unwrap(),
            CandleWhisperComputeType::Fp32
        );
        assert_eq!(
            CandleWhisperComputeType::Automatic
                .resolve_for_device(false)
                .unwrap(),
            CandleWhisperComputeType::Fp32
        );
        assert_eq!(
            CandleWhisperComputeType::Fp32
                .resolve_for_device(false)
                .unwrap(),
            CandleWhisperComputeType::Fp32
        );
    }

    #[test]
    fn candle_whisper_cpu_fp16_is_rejected_clearly() {
        let error = CandleWhisperComputeType::Fp16
            .resolve_for_device(false)
            .unwrap_err()
            .to_string();

        assert!(error.contains("setup_error"));
        assert!(error.contains("fp16 requires a CUDA device"));
    }

    #[test]
    fn only_automatic_candle_compute_type_is_setup_fallback_eligible() {
        assert!(CandleWhisperComputeType::Automatic.setup_fallback_eligible());
        assert!(!CandleWhisperComputeType::Fp16.setup_fallback_eligible());
        assert!(!CandleWhisperComputeType::Fp32.setup_fallback_eligible());
    }

    fn diarization_response_for_tests(
        segments: Vec<SpeakerSegmentPrediction>,
    ) -> SpeakerDiarizationResponse {
        SpeakerDiarizationResponse {
            accepted: true,
            operation: "audio.speakers.diarize".to_string(),
            model_id: "test-speakers".to_string(),
            runtime: audio_analysis_speakers::AudioRuntime::Imported,
            segments,
            speaker_embeddings: None,
            diagnostics: Vec::new(),
        }
    }

    fn transcript_with_words(
        words: Vec<(&str, f64, f64)>,
    ) -> std::result::Result<TranscriptionContract, Box<dyn std::error::Error>> {
        let mut segment = TranscriptSegmentContract::new(
            0,
            words
                .iter()
                .map(|(word, _, _)| *word)
                .collect::<Vec<_>>()
                .join(" "),
        );
        segment.start_seconds = Some(0.0);
        segment.end_seconds = Some(2.0);
        segment.words = words
            .into_iter()
            .map(
                |(text, start_seconds, end_seconds)| TranscriptWordContract {
                    text: text.to_string(),
                    start_seconds: Some(start_seconds),
                    end_seconds: Some(end_seconds),
                    confidence: None,
                    speaker: None,
                    attributes: BTreeMap::new(),
                },
            )
            .collect();
        Ok(TranscriptionContract::from_segments(
            None,
            Some("en".to_string()),
            vec![segment],
        )?)
    }

    fn assign_speakers_from_diarization(
        transcript: &mut TranscriptionContract,
        diarization: &SpeakerDiarizationResponse,
        policy: SpeakerAssignmentPolicy,
    ) -> Result<()> {
        *transcript = audio_analysis_speakers::assign_speakers_to_transcript_with_policy(
            transcript,
            diarization,
            policy,
        )?;
        Ok(())
    }

    #[cfg(feature = "diarization")]
    fn sine_into(samples: &mut [f32], sample_rate: u32, start_seconds: f32, freq_hz: f32) {
        for (offset, sample) in samples.iter_mut().enumerate() {
            let t = start_seconds + offset as f32 / sample_rate as f32;
            *sample = (2.0 * std::f32::consts::PI * freq_hz * t).sin() * 0.5;
        }
    }

    #[cfg(feature = "diarization")]
    fn two_profile_loaded_audio() -> LoadedAudio {
        let sample_rate = 16_000;
        let mut samples = vec![0.0_f32; sample_rate as usize * 2];
        let first_start = (0.20 * sample_rate as f32) as usize;
        let first_end = (0.50 * sample_rate as f32) as usize;
        let second_start = (1.00 * sample_rate as f32) as usize;
        let second_end = (1.40 * sample_rate as f32) as usize;
        sine_into(
            &mut samples[first_start..first_end],
            sample_rate,
            0.20,
            220.0,
        );
        sine_into(
            &mut samples[second_start..second_end],
            sample_rate,
            1.00,
            1_200.0,
        );
        LoadedAudio {
            samples,
            sample_rate,
            channels: 1,
            source: Some("synthetic-two-speaker".to_string()),
        }
    }

    #[cfg(all(feature = "diarization", feature = "onnx"))]
    fn local_16khz_wav_samples(
        path: &Path,
    ) -> std::result::Result<Vec<f32>, Box<dyn std::error::Error>> {
        let mut reader = hound::WavReader::open(path)?;
        let spec = reader.spec();
        if spec.sample_rate != 16_000 {
            return Err(format!(
                "DIARIZATION_AUDIO_PATH must be 16 kHz WAV, got {} Hz",
                spec.sample_rate
            )
            .into());
        }
        if spec.channels == 0 {
            return Err("DIARIZATION_AUDIO_PATH WAV channel count must be non-zero".into());
        }
        let interleaved = match spec.sample_format {
            hound::SampleFormat::Float => {
                if spec.bits_per_sample != 32 {
                    return Err("float DIARIZATION_AUDIO_PATH WAV must be 32-bit".into());
                }
                reader
                    .samples::<f32>()
                    .collect::<std::result::Result<Vec<_>, _>>()?
            }
            hound::SampleFormat::Int if spec.bits_per_sample <= 16 => reader
                .samples::<i16>()
                .map(|sample| sample.map(|value| value as f32 / 32_768.0))
                .collect::<std::result::Result<Vec<_>, _>>()?,
            hound::SampleFormat::Int => {
                let scale = 2_f32.powi(spec.bits_per_sample as i32 - 1);
                reader
                    .samples::<i32>()
                    .map(|sample| sample.map(|value| value as f32 / scale))
                    .collect::<std::result::Result<Vec<_>, _>>()?
            }
        };
        let channels = spec.channels as usize;
        let samples = if channels == 1 {
            interleaved
        } else {
            interleaved
                .chunks_exact(channels)
                .map(|frame| frame.iter().copied().sum::<f32>() / channels as f32)
                .collect()
        };
        if samples.is_empty() {
            return Err("DIARIZATION_AUDIO_PATH WAV must contain samples".into());
        }
        if !samples.iter().all(|sample| sample.is_finite()) {
            return Err("DIARIZATION_AUDIO_PATH WAV samples must be finite".into());
        }
        Ok(samples)
    }

    #[test]
    fn diarization_options_reject_invalid_speaker_bounds() {
        let mut options = DiarizationOptions {
            speaker: SpeakerDiarizationOptions {
                min_speakers: Some(0),
                ..SpeakerDiarizationOptions::default()
            },
            ..DiarizationOptions::default()
        };
        assert!(validate_diarization_options(&options)
            .unwrap_err()
            .to_string()
            .contains("invalid_request"));

        options = DiarizationOptions {
            speaker: SpeakerDiarizationOptions {
                max_speakers: Some(0),
                ..SpeakerDiarizationOptions::default()
            },
            ..DiarizationOptions::default()
        };
        assert!(validate_diarization_options(&options)
            .unwrap_err()
            .to_string()
            .contains("invalid_request"));

        options = DiarizationOptions {
            speaker: SpeakerDiarizationOptions {
                min_speakers: Some(3),
                max_speakers: Some(2),
                ..SpeakerDiarizationOptions::default()
            },
            ..DiarizationOptions::default()
        };
        assert!(validate_diarization_options(&options)
            .unwrap_err()
            .to_string()
            .contains("invalid_request"));

        options = DiarizationOptions {
            speaker: SpeakerDiarizationOptions {
                min_speakers: Some(1),
                max_speakers: Some(2),
                ..SpeakerDiarizationOptions::default()
            },
            ..DiarizationOptions::default()
        };
        validate_diarization_options(&options).unwrap();
    }

    #[test]
    fn diarization_options_keep_flat_json_shape_with_speaker_owned_contract() {
        let options: DiarizationOptions = serde_json::from_value(serde_json::json!({
            "enabled": true,
            "modelId": "native-speakers",
            "speakerEmbeddingModelBundle": "/models/speaker",
            "speakerEmbeddingModelFile": "speaker.onnx",
            "speakerEmbeddingInputName": "waveform",
            "speakerEmbeddingOutputName": "embedding",
            "speakerEmbeddingDimension": 192,
            "speakerEmbeddingSampleRate": 16000,
            "returnSpeakerEmbeddings": true,
            "minSpeakers": 1,
            "maxSpeakers": 2,
            "assignmentPolicy": "strictContained"
        }))
        .unwrap();

        assert!(options.enabled);
        assert_eq!(options.model_id, "native-speakers");
        assert_eq!(
            options.speaker_embedding_model_bundle.as_deref(),
            Some(Path::new("/models/speaker"))
        );
        assert_eq!(
            options.speaker_embedding_model_file.as_deref(),
            Some("speaker.onnx")
        );
        assert_eq!(
            options.speaker_embedding_input_name.as_deref(),
            Some("waveform")
        );
        assert_eq!(
            options.speaker_embedding_output_name.as_deref(),
            Some("embedding")
        );
        assert_eq!(options.speaker_embedding_dimension, Some(192));
        assert_eq!(options.speaker_embedding_sample_rate, Some(16_000));
        assert!(options.return_speaker_embeddings);
        assert_eq!(options.min_speakers, Some(1));
        assert_eq!(options.max_speakers, Some(2));
        assert_eq!(
            options.assignment_policy,
            SpeakerAssignmentPolicy::StrictContained
        );

        let value = serde_json::to_value(&options).unwrap();
        assert_eq!(value["enabled"], true);
        assert_eq!(value["modelId"], "native-speakers");
        assert_eq!(value["assignmentPolicy"], "strictContained");
        assert!(value.get("speaker").is_none());
    }

    #[test]
    fn default_build_exposes_speakers_owned_diarization_response_types() {
        fn accepts_speakers_response(_: audio_analysis_speakers::SpeakerDiarizationResponse) {}

        let response = SpeakerDiarizationResponse {
            accepted: true,
            operation: "audio.speakers.diarize".to_string(),
            model_id: "fixture".to_string(),
            runtime: audio_analysis_speakers::AudioRuntime::Imported,
            segments: vec![SpeakerSegmentPrediction {
                speaker: "speaker_0".to_string(),
                start_seconds: 0.0,
                end_seconds: 1.0,
                score: None,
            }],
            speaker_embeddings: None,
            diagnostics: Vec::new(),
        };

        accepts_speakers_response(response);
    }

    #[test]
    fn batch_options_reject_zero_max_batch_size() {
        let mut vad = FixedVadProvider {
            segments: batch_test_chunks(),
        };
        let mut asr = MockAsrProvider;
        let result = run_transcription_pipeline(
            batch_test_request(CandleWhisperOptions {
                max_batch_size: Some(0),
                ..CandleWhisperOptions::default()
            }),
            &mut vad,
            &mut asr,
            None,
            None,
        );

        let error = result.unwrap_err().to_string();
        assert!(error.contains("invalid_request"));
        assert!(error.contains("max_batch_size"));
    }

    #[test]
    fn candle_decode_runtime_defaults_to_autoregressive_kv_cache() {
        let options = CandleWhisperOptions::default();

        assert_eq!(
            options.decode_runtime,
            CandleWhisperDecodeRuntime::AutoregressiveKvCache
        );
        assert_eq!(
            options.decode_runtime.execution_id(),
            "candle-whisper-autoregressive-kv-cache"
        );
        validate_candle_batch_options(&options).unwrap();
    }

    #[test]
    fn candle_decode_runtime_deserializes_active_row_tensor_batch() {
        let options: CandleWhisperOptions = serde_json::from_value(serde_json::json!({
            "decodeRuntime": "activeRowTensorBatch",
            "batchChunks": true,
            "maxBatchSize": 4
        }))
        .unwrap();

        assert_eq!(
            options.decode_runtime,
            CandleWhisperDecodeRuntime::ActiveRowTensorBatch
        );
        assert_eq!(
            options.decode_runtime.execution_id(),
            "candle-whisper-active-row-tensor-batch"
        );
    }

    #[test]
    fn candle_active_row_decode_runtime_is_supported_when_batching_is_enabled() {
        let options = CandleWhisperOptions {
            decode_runtime: CandleWhisperDecodeRuntime::ActiveRowTensorBatch,
            batch_chunks: true,
            max_batch_size: Some(4),
            ..CandleWhisperOptions::default()
        };

        validate_candle_batch_options(&options).unwrap();
        assert!(options.decode_runtime.is_supported());
    }

    #[test]
    fn candle_active_row_decode_runtime_requires_chunk_batching() {
        let options = CandleWhisperOptions {
            decode_runtime: CandleWhisperDecodeRuntime::ActiveRowTensorBatch,
            batch_chunks: false,
            max_batch_size: Some(4),
            ..CandleWhisperOptions::default()
        };

        let error = validate_candle_batch_options(&options)
            .unwrap_err()
            .to_string();
        assert!(error.contains("invalid_request"));
        assert!(error.contains("requires batch_chunks=true"));
    }

    #[test]
    fn candle_active_row_decode_runtime_rejects_single_row_batching() {
        let options = CandleWhisperOptions {
            decode_runtime: CandleWhisperDecodeRuntime::ActiveRowTensorBatch,
            batch_chunks: true,
            max_batch_size: Some(1),
            ..CandleWhisperOptions::default()
        };

        let error = validate_candle_batch_options(&options)
            .unwrap_err()
            .to_string();
        assert!(error.contains("invalid_request"));
        assert!(error.contains("max_batch_size greater than one"));
    }

    #[test]
    fn batch_chunking_preserves_transcript_order() {
        let mut vad = FixedVadProvider {
            segments: batch_test_chunks(),
        };
        let mut asr = MockAsrProvider;
        let response = run_transcription_pipeline(
            batch_test_request(CandleWhisperOptions {
                batch_chunks: true,
                max_batch_size: Some(2),
                ..CandleWhisperOptions::default()
            }),
            &mut vad,
            &mut asr,
            None,
            None,
        )
        .unwrap();

        let starts = response
            .transcript
            .segments
            .iter()
            .map(|segment| segment.start_seconds.unwrap())
            .collect::<Vec<_>>();
        assert_eq!(starts, vec![0.0, 0.20, 0.40]);
    }

    #[test]
    fn batch_chunking_reports_batch_diagnostics() {
        let mut vad = FixedVadProvider {
            segments: batch_test_chunks(),
        };
        let mut asr = MockAsrProvider;
        let response = run_transcription_pipeline(
            batch_test_request(CandleWhisperOptions {
                batch_chunks: true,
                max_batch_size: Some(2),
                ..CandleWhisperOptions::default()
            }),
            &mut vad,
            &mut asr,
            None,
            None,
        )
        .unwrap();

        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "chunkCount=3"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "batchChunks=true"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "maxBatchSize=2"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "batchCount=2"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "batchExecution=candle-whisper-autoregressive-kv-cache"));
    }

    #[test]
    fn requested_active_row_runtime_reports_fallback_execution_without_native_proof() {
        let mut vad = FixedVadProvider {
            segments: batch_test_chunks(),
        };
        let mut asr = MockAsrProvider;
        let response = run_transcription_pipeline(
            batch_test_request(CandleWhisperOptions {
                decode_runtime: CandleWhisperDecodeRuntime::ActiveRowTensorBatch,
                batch_chunks: true,
                max_batch_size: Some(3),
                ..CandleWhisperOptions::default()
            }),
            &mut vad,
            &mut asr,
            None,
            None,
        )
        .unwrap();

        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "batchExecution=candle-whisper-autoregressive-kv-cache"));
        assert!(!response
            .diagnostics
            .iter()
            .any(|item| item == "batchExecution=candle-whisper-active-row-tensor-batch"));
    }

    #[test]
    fn public_batch_diagnostics_do_not_claim_active_row_execution_by_request() {
        let diagnostics = candle_batch_diagnostics(
            &CandleWhisperOptions {
                decode_runtime: CandleWhisperDecodeRuntime::ActiveRowTensorBatch,
                batch_chunks: true,
                max_batch_size: Some(4),
                ..CandleWhisperOptions::default()
            },
            3,
        );

        assert!(diagnostics
            .iter()
            .any(|item| item == "batchExecution=candle-whisper-autoregressive-kv-cache"));
        assert!(!diagnostics
            .iter()
            .any(|item| item == "batchExecution=candle-whisper-active-row-tensor-batch"));
    }

    #[test]
    fn batch_chunking_reports_unbounded_batch_diagnostics() {
        let mut vad = FixedVadProvider {
            segments: batch_test_chunks(),
        };
        let mut asr = MockAsrProvider;
        let response = run_transcription_pipeline(
            batch_test_request(CandleWhisperOptions {
                batch_chunks: true,
                max_batch_size: None,
                ..CandleWhisperOptions::default()
            }),
            &mut vad,
            &mut asr,
            None,
            None,
        )
        .unwrap();

        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "maxBatchSize=unbounded"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "batchCount=1"));
    }

    #[test]
    fn batch_disabled_reports_sequential_diagnostics() {
        let mut vad = FixedVadProvider {
            segments: batch_test_chunks(),
        };
        let mut asr = MockAsrProvider;
        let response = run_transcription_pipeline(
            batch_test_request(CandleWhisperOptions {
                batch_chunks: false,
                max_batch_size: Some(2),
                ..CandleWhisperOptions::default()
            }),
            &mut vad,
            &mut asr,
            None,
            None,
        )
        .unwrap();

        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "batchChunks=false"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "batchCount=3"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "batchExecution=candle-whisper-autoregressive-kv-cache"));
    }

    #[test]
    #[cfg(feature = "diarization")]
    fn speech_spans_from_transcript_prefers_aligned_words(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let transcript = transcript_with_words(vec![("hello", 0.2, 0.5), ("world", 1.0, 1.4)])?;

        let spans = speech_spans_from_transcript(&transcript, 2.0)?;

        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].start_seconds, 0.2);
        assert_eq!(spans[0].end_seconds, 0.5);
        assert_eq!(spans[1].start_seconds, 1.0);
        assert_eq!(spans[1].end_seconds, 1.4);
        assert!(spans[0].start_seconds < spans[1].start_seconds);
        assert!(spans
            .iter()
            .all(|span| span.score.is_finite() && span.score > 0.0));
        Ok(())
    }

    #[test]
    #[cfg(feature = "diarization")]
    fn speech_spans_from_transcript_falls_back_to_segments(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut first = TranscriptSegmentContract::new(0, "hello");
        first.start_seconds = Some(0.2);
        first.end_seconds = Some(0.5);
        let mut second = TranscriptSegmentContract::new(1, "world");
        second.start_seconds = Some(1.0);
        second.end_seconds = Some(1.4);
        let transcript = TranscriptionContract::from_segments(
            None,
            Some("en".to_string()),
            vec![second, first],
        )?;

        let spans = speech_spans_from_transcript(&transcript, 2.0)?;

        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].start_seconds, 0.2);
        assert_eq!(spans[0].end_seconds, 0.5);
        assert_eq!(spans[1].start_seconds, 1.0);
        assert_eq!(spans[1].end_seconds, 1.4);
        Ok(())
    }

    #[test]
    #[cfg(feature = "diarization")]
    fn speech_spans_from_transcript_rejects_out_of_range_timing(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let transcript = transcript_with_words(vec![("hello", 0.2, 2.000_01)])?;

        let error = speech_spans_from_transcript(&transcript, 2.0)
            .unwrap_err()
            .to_string();

        assert!(error.contains("invalid_request"));
        Ok(())
    }

    #[cfg(feature = "alignment")]
    struct AlignmentAwareDiarizationProvider {
        saw_aligned_words: bool,
    }

    #[cfg(feature = "alignment")]
    impl TranscriptDiarizationProvider for AlignmentAwareDiarizationProvider {
        fn provider_id(&self) -> &str {
            "alignment-aware-diarization"
        }

        fn diarize(
            &mut self,
            _audio: LoadedAudio,
            transcript: &TranscriptionContract,
            options: &DiarizationOptions,
        ) -> Result<SpeakerDiarizationResponse> {
            self.saw_aligned_words = transcript.segments.iter().any(|segment| {
                segment.words.iter().any(|word| {
                    word.start_seconds.is_some()
                        && word.end_seconds.is_some()
                        && word.confidence.is_some()
                })
            });
            assert!(
                self.saw_aligned_words,
                "diarization should receive transcript word timings from alignment"
            );
            let mut response = diarization_response_for_tests(vec![SpeakerSegmentPrediction {
                speaker: "SPEAKER_ALIGNED".to_string(),
                start_seconds: 0.0,
                end_seconds: 1.0,
                score: Some(0.95),
            }]);
            response.model_id = options.model_id.clone();
            Ok(response)
        }
    }

    #[cfg(all(feature = "alignment", feature = "candle"))]
    fn write_tiny_wav2vec2_bundle(root: &Path) {
        use candle_core::{Device, Tensor};
        use std::collections::HashMap;

        fs::write(
            root.join("config.json"),
            serde_json::json!({
                "model_type": "wav2vec2",
                "architectures": ["Wav2Vec2ForCTC"],
                "vocab_size": 10,
                "word_delimiter_token": "|",
                "hidden_size": 1,
                "num_hidden_layers": 0,
                "num_attention_heads": 1,
                "intermediate_size": 1,
                "hidden_act": "gelu",
                "layer_norm_eps": 1e-5,
                "feat_extract_activation": "gelu",
                "conv_dim": [1],
                "conv_stride": [1],
                "conv_kernel": [1],
                "conv_bias": false,
                "num_conv_pos_embeddings": 0,
                "num_conv_pos_embedding_groups": 1
            })
            .to_string(),
        )
        .unwrap();
        fs::write(
            root.join("tokenizer.json"),
            serde_json::json!({
                "version": "1.0",
                "word_delimiter_token": "|",
                "model": {
                    "type": "WordLevel",
                    "vocab": {
                        "[PAD]": 0,
                        "H": 1,
                        "E": 2,
                        "L": 3,
                        "O": 4,
                        "|": 5,
                        "W": 6,
                        "R": 7,
                        "D": 8,
                        "<unk>": 9
                    },
                    "unk_token": "<unk>"
                }
            })
            .to_string(),
        )
        .unwrap();
        fs::write(
            root.join("preprocessor_config.json"),
            serde_json::json!({
                "sampling_rate": 16000,
                "do_normalize": false,
                "return_attention_mask": false
            })
            .to_string(),
        )
        .unwrap();

        let device = Device::Cpu;
        let mut tensors = HashMap::new();
        tensors.insert(
            "wav2vec2.feature_extractor.conv_layers.0.conv.weight".to_string(),
            Tensor::new(&[1.0f32], &device)
                .unwrap()
                .reshape((1, 1, 1))
                .unwrap(),
        );
        tensors.insert(
            "wav2vec2.feature_projection.layer_norm.weight".to_string(),
            Tensor::new(&[1.0f32], &device).unwrap(),
        );
        tensors.insert(
            "wav2vec2.feature_projection.layer_norm.bias".to_string(),
            Tensor::new(&[0.0f32], &device).unwrap(),
        );
        tensors.insert(
            "wav2vec2.feature_projection.projection.weight".to_string(),
            Tensor::new(&[1.0f32], &device)
                .unwrap()
                .reshape((1, 1))
                .unwrap(),
        );
        tensors.insert(
            "wav2vec2.feature_projection.projection.bias".to_string(),
            Tensor::new(&[0.0f32], &device).unwrap(),
        );
        tensors.insert(
            "lm_head.weight".to_string(),
            Tensor::new(&[0.0f32; 10], &device)
                .unwrap()
                .reshape((10, 1))
                .unwrap(),
        );
        tensors.insert(
            "lm_head.bias".to_string(),
            Tensor::new(&[0.0f32; 10], &device).unwrap(),
        );
        candle_core::safetensors::save(&tensors, root.join("model.safetensors")).unwrap();
    }

    #[cfg(all(feature = "alignment", feature = "candle", feature = "model-bundles"))]
    fn env_path(name: &str) -> Option<PathBuf> {
        std::env::var_os(name).map(PathBuf::from)
    }

    #[cfg(all(feature = "alignment", feature = "candle", feature = "model-bundles"))]
    fn write_default_alignment_smoke_wav(path: &Path) -> std::result::Result<(), hound::Error> {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(path, spec)?;
        for index in 0..16_000 {
            let sample = if (1_000..8_000).contains(&index) {
                16_384i16
            } else {
                0i16
            };
            writer.write_sample(sample)?;
        }
        writer.finalize()
    }

    #[cfg(all(feature = "alignment", feature = "candle", feature = "model-bundles"))]
    fn default_alignment_smoke_audio_path(
        temp: &Path,
    ) -> std::result::Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(path) =
            env_path("ALIGNMENT_AUDIO_PATH").or_else(|| env_path("TRANSCRIPTION_AUDIO_PATH"))
        {
            return Ok(path);
        }

        let repo_sample = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../vendor/whisper.cpp/samples/jfk.wav");
        if repo_sample.exists() {
            return Ok(repo_sample);
        }

        let generated = temp.join("default-alignment-smoke.wav");
        write_default_alignment_smoke_wav(&generated)?;
        Ok(generated)
    }

    #[cfg(all(feature = "alignment", feature = "candle", feature = "model-bundles"))]
    fn resolve_alignment_smoke_bundle_candidate(path: &Path) -> Option<PathBuf> {
        let candidates = [
            path.to_path_buf(),
            path.join("main"),
            path.join("wav2vec2-base-960h"),
            path.join("wav2vec2-base-960h/main"),
            path.join("facebook--wav2vec2-base-960h"),
            path.join("facebook--wav2vec2-base-960h/main"),
            path.join("models--facebook--wav2vec2-base-960h"),
        ];
        candidates
            .into_iter()
            .find(|candidate| native_wav2vec2::resolve_wav2vec2_bundle_paths(candidate).is_ok())
    }

    #[cfg(all(feature = "alignment", feature = "candle", feature = "model-bundles"))]
    fn default_alignment_smoke_bundle(
        temp: &Path,
    ) -> std::result::Result<(PathBuf, &'static str), Box<dyn std::error::Error>> {
        if let Some(bundle) = env_path("ALIGNMENT_MODEL_BUNDLE") {
            return Ok((bundle, "ALIGNMENT_MODEL_BUNDLE"));
        }

        if let Some(model_dir) = env_path("ALIGNMENT_MODEL_DIR") {
            if let Some(bundle) = resolve_alignment_smoke_bundle_candidate(&model_dir) {
                return Ok((bundle, "ALIGNMENT_MODEL_DIR"));
            }
            return Err(format!(
                "ALIGNMENT_MODEL_DIR did not contain a supported wav2vec2 bundle: {}",
                model_dir.display()
            )
            .into());
        }

        if let Some(xdg_data_home) = env_path("XDG_DATA_HOME") {
            let smoke_models = xdg_data_home.join("video-analysis-smoke/models");
            if let Some(bundle) = resolve_alignment_smoke_bundle_candidate(&smoke_models) {
                return Ok((bundle, "default-xdg-data-home"));
            }
        }

        if let Some(home) = env_path("HOME") {
            let smoke_models = home.join(".local/share/video-analysis-smoke/models");
            if let Some(bundle) = resolve_alignment_smoke_bundle_candidate(&smoke_models) {
                return Ok((bundle, "default-home-local-share"));
            }
        }

        let generated = temp.join("default-wav2vec2-bundle");
        fs::create_dir_all(&generated)?;
        write_tiny_wav2vec2_bundle(&generated);
        Ok((generated, "generated-tiny-bundle"))
    }

    #[test]
    fn majority_overlap_assigns_word_and_segment_speaker(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut transcript = transcript_with_words(vec![("hello", 0.1, 0.7), ("world", 0.8, 1.2)])?;
        let diarization = diarization_response_for_tests(vec![
            SpeakerSegmentPrediction {
                speaker: "speaker_0".to_string(),
                start_seconds: 0.0,
                end_seconds: 0.75,
                score: Some(0.9),
            },
            SpeakerSegmentPrediction {
                speaker: "speaker_1".to_string(),
                start_seconds: 0.75,
                end_seconds: 1.5,
                score: Some(0.8),
            },
        ]);

        assign_speakers_from_diarization(
            &mut transcript,
            &diarization,
            SpeakerAssignmentPolicy::Majority,
        )?;

        assert_eq!(
            transcript.segments[0].words[0].speaker.as_deref(),
            Some("speaker_0")
        );
        assert_eq!(
            transcript.segments[0].words[1].speaker.as_deref(),
            Some("speaker_1")
        );
        assert_eq!(transcript.segments[0].speaker.as_deref(), Some("speaker_0"));
        Ok(())
    }

    #[test]
    fn nearest_start_policy_assigns_nearest_speaker(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut transcript = transcript_with_words(vec![("hello", 0.72, 0.9)])?;
        let diarization = diarization_response_for_tests(vec![
            SpeakerSegmentPrediction {
                speaker: "speaker_0".to_string(),
                start_seconds: 0.0,
                end_seconds: 0.4,
                score: Some(0.9),
            },
            SpeakerSegmentPrediction {
                speaker: "speaker_1".to_string(),
                start_seconds: 0.7,
                end_seconds: 1.0,
                score: Some(0.9),
            },
        ]);

        assign_speakers_from_diarization(
            &mut transcript,
            &diarization,
            SpeakerAssignmentPolicy::NearestStart,
        )?;

        assert_eq!(
            transcript.segments[0].words[0].speaker.as_deref(),
            Some("speaker_1")
        );
        Ok(())
    }

    #[test]
    fn strict_contained_policy_leaves_uncontained_words_unassigned(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut transcript = transcript_with_words(vec![("hello", 0.2, 0.8)])?;
        let diarization = diarization_response_for_tests(vec![SpeakerSegmentPrediction {
            speaker: "speaker_0".to_string(),
            start_seconds: 0.3,
            end_seconds: 0.7,
            score: Some(0.9),
        }]);

        assign_speakers_from_diarization(
            &mut transcript,
            &diarization,
            SpeakerAssignmentPolicy::StrictContained,
        )?;

        assert!(transcript.segments[0].words[0].speaker.is_none());
        assert!(transcript.segments[0].speaker.is_none());
        Ok(())
    }

    #[test]
    fn existing_segment_speaker_is_not_overwritten(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut transcript = transcript_with_words(vec![("hello", 0.1, 0.4)])?;
        transcript.segments[0].speaker = Some("manual".to_string());
        let diarization = diarization_response_for_tests(vec![SpeakerSegmentPrediction {
            speaker: "speaker_0".to_string(),
            start_seconds: 0.0,
            end_seconds: 1.0,
            score: Some(0.9),
        }]);

        assign_speakers_from_diarization(
            &mut transcript,
            &diarization,
            SpeakerAssignmentPolicy::Majority,
        )?;

        assert_eq!(transcript.segments[0].speaker.as_deref(), Some("manual"));
        assert_eq!(
            transcript.segments[0].words[0].speaker.as_deref(),
            Some("speaker_0")
        );
        Ok(())
    }

    #[test]
    fn segment_speaker_uses_majority_word_speaker_when_words_are_present(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut transcript = transcript_with_words(vec![
            ("one", 0.0, 0.3),
            ("two", 0.3, 0.6),
            ("three", 0.6, 0.9),
        ])?;
        let diarization = diarization_response_for_tests(vec![
            SpeakerSegmentPrediction {
                speaker: "speaker_0".to_string(),
                start_seconds: 0.0,
                end_seconds: 0.65,
                score: Some(0.9),
            },
            SpeakerSegmentPrediction {
                speaker: "speaker_1".to_string(),
                start_seconds: 0.65,
                end_seconds: 1.0,
                score: Some(0.9),
            },
        ]);

        assign_speakers_from_diarization(
            &mut transcript,
            &diarization,
            SpeakerAssignmentPolicy::Majority,
        )?;

        assert_eq!(transcript.segments[0].speaker.as_deref(), Some("speaker_0"));
        Ok(())
    }

    #[test]
    fn segment_without_words_uses_policy_fallback(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut segment = TranscriptSegmentContract::new(0, "hello");
        segment.start_seconds = Some(0.5);
        segment.end_seconds = Some(0.8);
        let mut transcript =
            TranscriptionContract::from_segments(None, Some("en".to_string()), vec![segment])?;
        let diarization = diarization_response_for_tests(vec![
            SpeakerSegmentPrediction {
                speaker: "speaker_0".to_string(),
                start_seconds: 0.0,
                end_seconds: 0.2,
                score: Some(0.9),
            },
            SpeakerSegmentPrediction {
                speaker: "speaker_1".to_string(),
                start_seconds: 0.45,
                end_seconds: 1.0,
                score: Some(0.9),
            },
        ]);

        assign_speakers_from_diarization(
            &mut transcript,
            &diarization,
            SpeakerAssignmentPolicy::StrictContained,
        )?;

        assert_eq!(transcript.segments[0].speaker.as_deref(), Some("speaker_1"));
        Ok(())
    }

    #[test]
    fn offset_chunk_local_segments_skips_global_timing() -> Result<()> {
        let mut segment = TranscriptSegmentContract::new(0, "hello");
        segment.start_seconds = Some(10.0);
        segment.end_seconds = Some(10.5);
        segment
            .attributes
            .insert("timing".to_string(), "global".to_string());
        segment.words.push(TranscriptWordContract {
            text: "hello".to_string(),
            start_seconds: Some(10.0),
            end_seconds: Some(10.5),
            confidence: None,
            speaker: None,
            attributes: BTreeMap::new(),
        });
        let mut transcript =
            TranscriptionContract::from_segments(None, Some("en".to_string()), vec![segment])
                .map_err(|error| DetectError::InvalidArgument(error.to_string()))?;
        let chunks = vec![SpeechActivitySegment::new(5.0, 6.0, 0.8)?];

        offset_chunk_local_segments(&mut transcript, &chunks)?;

        assert_eq!(transcript.segments[0].start_seconds, Some(10.0));
        assert_eq!(transcript.segments[0].end_seconds, Some(10.5));
        assert_eq!(transcript.segments[0].words[0].start_seconds, Some(10.0));
        assert_eq!(transcript.segments[0].words[0].end_seconds, Some(10.5));
        Ok(())
    }

    #[test]
    fn offset_chunk_local_segments_offsets_local_timing() -> Result<()> {
        let mut segment = TranscriptSegmentContract::new(0, "hello");
        segment.start_seconds = Some(0.0);
        segment.end_seconds = Some(0.5);
        let mut transcript =
            TranscriptionContract::from_segments(None, Some("en".to_string()), vec![segment])
                .map_err(|error| DetectError::InvalidArgument(error.to_string()))?;
        let chunks = vec![SpeechActivitySegment::new(5.0, 6.0, 0.8)?];

        offset_chunk_local_segments(&mut transcript, &chunks)?;

        assert_eq!(transcript.segments[0].start_seconds, Some(5.0));
        assert_eq!(transcript.segments[0].end_seconds, Some(5.5));
        assert_eq!(
            transcript.segments[0]
                .attributes
                .get("timing")
                .map(String::as_str),
            Some("global")
        );
        Ok(())
    }

    #[test]
    fn alignment_overwrites_projected_word_timings() -> Result<()> {
        let mut segment = TranscriptSegmentContract::new(0, "hello");
        segment.start_seconds = Some(0.0);
        segment.end_seconds = Some(1.0);
        segment.words.push(TranscriptWordContract {
            text: "hello".to_string(),
            start_seconds: Some(0.0),
            end_seconds: Some(0.5),
            confidence: None,
            speaker: None,
            attributes: BTreeMap::from([(
                "timing".to_string(),
                "whisperTimestampProjection".to_string(),
            )]),
        });
        let mut transcript =
            TranscriptionContract::from_segments(None, Some("en".to_string()), vec![segment])
                .map_err(|error| DetectError::InvalidArgument(error.to_string()))?;

        apply_alignment_words(
            &mut transcript,
            &[AlignedWord {
                segment_index: 0,
                word_index: 0,
                text: "hello".to_string(),
                start_seconds: 0.1,
                end_seconds: 0.4,
                confidence: Some(0.9),
            }],
        )?;

        let word = &transcript.segments[0].words[0];
        assert_eq!(word.text, "hello");
        assert_eq!(word.start_seconds, Some(0.1));
        assert_eq!(word.end_seconds, Some(0.4));
        assert_eq!(word.confidence, Some(0.9));
        assert_eq!(
            word.attributes.get("timing").map(String::as_str),
            Some("whisperTimestampProjection")
        );
        assert_eq!(transcript.segments[0].start_seconds, Some(0.1));
        assert_eq!(transcript.segments[0].end_seconds, Some(0.4));
        Ok(())
    }

    #[test]
    fn provider_plan_reports_candle_primary_native_provider() {
        let plans = transcription_provider_plans();
        let candle = plans
            .iter()
            .find(|plan| plan.provider_id == "candle-whisper")
            .unwrap();
        assert!(candle.primary);
        assert!(!candle.external_runtime);
        assert!(plans
            .iter()
            .any(|plan| plan.provider_id == "whisperx-command" && !plan.primary));
    }

    #[test]
    fn cuda_request_without_feature_returns_setup_error() {
        let mut provider = CandleWhisperTranscriber::new(CandleWhisperOptions {
            device: NativeDevicePreference::Cuda,
            ..CandleWhisperOptions::default()
        });
        let result = provider.transcribe(AsrRequest {
            audio: LoadedAudio {
                samples: vec![0.0; 16],
                sample_rate: 16_000,
                channels: 1,
                source: None,
            },
            chunks: vec![SpeechActivitySegment::new(0.0, 0.001, 0.0).unwrap()],
            task: TranscriptionTask::Transcribe,
            language: None,
            model_id: "openai/whisper-large-v3".to_string(),
        });
        let error = result.unwrap_err().to_string();
        assert!(error.contains("setup_error") || cfg!(feature = "cuda"));
    }

    #[test]
    fn missing_model_bundle_returns_setup_error() {
        let temp = tempfile::tempdir().unwrap();
        let mut provider = CandleWhisperTranscriber::new(CandleWhisperOptions {
            model_dir: Some(temp.path().to_path_buf()),
            model_cache_only: true,
            ..CandleWhisperOptions::default()
        });
        let result = provider.transcribe(AsrRequest {
            audio: LoadedAudio {
                samples: vec![0.0; 16],
                sample_rate: 16_000,
                channels: 1,
                source: None,
            },
            chunks: vec![SpeechActivitySegment::new(0.0, 0.001, 0.0).unwrap()],
            task: TranscriptionTask::Transcribe,
            language: None,
            model_id: "openai/whisper-large-v3".to_string(),
        });
        let error = result.unwrap_err().to_string();
        assert!(error.contains("setup_error") || error.contains("unsupported_runtime"));
        assert!(error.contains("cache-only=true") || error.contains("model-bundles"));
    }

    #[test]
    fn empty_audio_returns_invalid_request() {
        let mut provider = CandleWhisperTranscriber::default();
        let result = provider.transcribe(AsrRequest {
            audio: LoadedAudio {
                samples: Vec::new(),
                sample_rate: 16_000,
                channels: 1,
                source: None,
            },
            chunks: vec![SpeechActivitySegment::new(0.0, 0.001, 0.0).unwrap()],
            task: TranscriptionTask::Transcribe,
            language: None,
            model_id: "openai/whisper-large-v3".to_string(),
        });
        let error = result.unwrap_err().to_string();
        assert!(error.contains("invalid_request"));
        assert!(error.contains("empty audio"));
    }

    #[test]
    fn non_finite_audio_returns_invalid_request() {
        let mut provider = CandleWhisperTranscriber::default();
        let result = provider.transcribe(AsrRequest {
            audio: LoadedAudio {
                samples: vec![0.0, f32::NAN],
                sample_rate: 16_000,
                channels: 1,
                source: None,
            },
            chunks: vec![SpeechActivitySegment::new(0.0, 0.001, 0.0).unwrap()],
            task: TranscriptionTask::Transcribe,
            language: None,
            model_id: "openai/whisper-large-v3".to_string(),
        });
        let error = result.unwrap_err().to_string();
        assert!(error.contains("invalid_request"));
        assert!(error.contains("finite"));
    }

    #[cfg(not(feature = "audio-io"))]
    #[test]
    fn path_non_wav_returns_unsupported_runtime() {
        let result = LoadedAudio::mono_16khz_from_source(&TranscriptionSource::Path {
            path: PathBuf::from("clip.mp4"),
        });
        let error = result.unwrap_err().to_string();
        assert!(error.contains("unsupported_runtime"));
        assert!(error.contains("WAV"));
    }

    #[test]
    fn wav_path_decodes_to_mono_16khz() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("stereo-8khz.wav");
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 8_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&path, spec)?;
        for _ in 0..8_000 {
            writer.write_sample::<i16>(16_384)?;
            writer.write_sample::<i16>(0)?;
        }
        writer.finalize()?;

        let audio = LoadedAudio::mono_16khz_from_source(&TranscriptionSource::Path { path })?;
        assert_eq!(audio.sample_rate, 16_000);
        assert_eq!(audio.channels, 1);
        assert_eq!(audio.samples.len(), 16_000);
        assert!(audio.samples.iter().all(|sample| sample.is_finite()));
        assert!(audio.samples.iter().any(|sample| *sample > 0.20));
        Ok(())
    }

    #[test]
    fn decode_diagnostics_report_direct_samples_route() {
        let (audio, diagnostics) =
            native_audio::mono_16khz_from_source_with_diagnostics(&TranscriptionSource::Samples {
                samples: vec![0.0, 1.0],
                sample_rate: 8_000,
                channels: 1,
                source: Some("inline".to_string()),
            })
            .unwrap();

        assert_eq!(diagnostics.decode_route, "direct-samples");
        assert_eq!(diagnostics.source_path_extension, None);
        assert_eq!(diagnostics.input_sample_rate, Some(8_000));
        assert_eq!(diagnostics.output_sample_rate, 16_000);
        assert_eq!(diagnostics.output_channels, 1);
        assert_eq!(audio.sample_rate, 16_000);
        assert_eq!(audio.channels, 1);
    }

    #[test]
    fn decode_diagnostics_report_wav_route() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("diagnostic.wav");
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 8_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&path, spec)?;
        writer.write_sample::<i16>(16_384)?;
        writer.finalize()?;

        let (_audio, diagnostics) =
            native_audio::mono_16khz_from_source_with_diagnostics(&TranscriptionSource::Path {
                path,
            })?;

        assert_eq!(diagnostics.decode_route, "native-wav-reader");
        assert_eq!(diagnostics.source_path_extension.as_deref(), Some("wav"));
        assert_eq!(diagnostics.input_sample_rate, Some(8_000));
        assert_eq!(diagnostics.output_sample_rate, 16_000);
        assert_eq!(diagnostics.output_channels, 1);
        Ok(())
    }

    #[test]
    fn wav_path_still_uses_native_reader_without_audio_io(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("native-reader.wav");
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&path, spec)?;
        writer.write_sample::<i16>(16_384)?;
        writer.write_sample::<i16>(-16_384)?;
        writer.finalize()?;

        let audio =
            LoadedAudio::mono_16khz_from_source(&TranscriptionSource::Path { path: path.clone() })?;

        assert_eq!(audio.sample_rate, 16_000);
        assert_eq!(audio.channels, 1);
        assert_eq!(
            audio.source.as_deref(),
            Some(path.to_string_lossy().as_ref())
        );
        assert_eq!(audio.samples.len(), 2);
        assert!(audio.samples[0] > 0.49);
        assert!(audio.samples[1] < -0.49);
        Ok(())
    }

    #[cfg(feature = "audio-io")]
    #[test]
    #[ignore = "requires RUN_NATIVE_MEDIA_DECODE_TESTS=1 and a local FFmpeg-decodable media file"]
    fn native_media_decode_when_requested() -> std::result::Result<(), Box<dyn std::error::Error>> {
        if std::env::var("RUN_NATIVE_MEDIA_DECODE_TESTS")
            .ok()
            .as_deref()
            != Some("1")
        {
            eprintln!("set RUN_NATIVE_MEDIA_DECODE_TESTS=1 to run native media decode smoke");
            return Ok(());
        }
        let path = std::env::var_os("TRANSCRIPTION_MEDIA_PATH")
            .map(PathBuf::from)
            .map(resolve_smoke_path)
            .ok_or("TRANSCRIPTION_MEDIA_PATH must point to a local media file")?;

        let (audio, diagnostics) =
            native_audio::mono_16khz_from_source_with_diagnostics(&TranscriptionSource::Path {
                path: path.clone(),
            })?;

        assert_eq!(audio.sample_rate, 16_000);
        assert_eq!(audio.channels, 1);
        assert!(!audio.samples.is_empty());
        assert!(audio.samples.iter().all(|sample| sample.is_finite()));
        assert_eq!(diagnostics.decode_route, "audio-io-media-decode");
        assert_eq!(diagnostics.output_sample_rate, 16_000);
        assert_eq!(diagnostics.output_channels, 1);
        assert!(diagnostics.input_sample_rate.is_some());
        assert_eq!(
            audio.source.as_deref(),
            Some(path.to_string_lossy().as_ref())
        );
        Ok(())
    }

    #[cfg(feature = "audio-io")]
    fn resolve_smoke_path(path: PathBuf) -> PathBuf {
        if path.is_absolute() || path.exists() {
            return path;
        }
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let Some(workspace_root) = manifest_dir.ancestors().nth(3) else {
            return path;
        };
        let workspace_path = workspace_root.join(&path);
        if workspace_path.exists() {
            workspace_path
        } else {
            path
        }
    }

    #[test]
    fn vad_splits_deterministic_synthetic_speech() {
        let request = sample_request();
        let audio = LoadedAudio::mono_16khz_from_source(&request.source).unwrap();
        let mut vad = EnergyVadTranscriptionProvider;
        let response = vad
            .detect_speech(VadRequest {
                audio,
                options: request.vad,
            })
            .unwrap();
        assert_eq!(response.segments.len(), 1);
        assert!(response.segments[0].start_seconds < 0.07);
        assert!(response.segments[0].end_seconds > 0.30);
    }

    #[test]
    fn mock_pipeline_normalizes_offsets_alignment_and_diarization() {
        let mut request = sample_request();
        request.alignment.enabled = true;
        request.diarization.enabled = true;
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;
        let mut aligner = MockAlignmentProvider;
        let mut diarizer = MockDiarizationProvider;
        let response = run_transcription_pipeline(
            request,
            &mut vad,
            &mut asr,
            Some(&mut aligner),
            Some(&mut diarizer),
        )
        .unwrap();
        assert!(response.accepted);
        assert_eq!(response.provider, "candle-whisper");
        assert_eq!(response.alignment.as_ref().unwrap().word_count, 1);
        assert_eq!(
            response.transcript.segments[0].words[0].speaker.as_deref(),
            Some("SPEAKER_00")
        );
        assert_eq!(
            response.transcript.segments[0].speaker.as_deref(),
            Some("SPEAKER_00")
        );
    }

    #[test]
    fn native_pipeline_reports_diarization_diagnostics() {
        let mut request = sample_request();
        request.diarization = DiarizationOptions {
            enabled: true,
            speaker: SpeakerDiarizationOptions {
                min_speakers: Some(2),
                max_speakers: Some(3),
                ..SpeakerDiarizationOptions::default()
            },
        };
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;
        let mut diarizer = MockDiarizationProvider;

        let response =
            run_transcription_pipeline(request, &mut vad, &mut asr, None, Some(&mut diarizer))
                .unwrap();

        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "diarizationProvider=mock-diarization"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "diarizationSegmentCount=1"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "diarizationSpeakerCount=1"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "diarizationMinSpeakers=2"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "diarizationSpeakerBoundsApplied=true"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "diarizationSpeakerCountBelowRequestedMin=1/2"));
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn native_pipeline_reports_onnx_diarization_diagnostics() {
        let mut request = sample_request();
        request.diarization = DiarizationOptions {
            enabled: true,
            speaker: SpeakerDiarizationOptions {
                speaker_embedding_model_bundle: Some(PathBuf::from("speaker-model")),
                speaker_embedding_dimension: Some(2),
                ..SpeakerDiarizationOptions::default()
            },
            ..DiarizationOptions::default()
        };
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;
        let mut diarizer = MockOnnxDiarizationProvider;

        let response =
            run_transcription_pipeline(request, &mut vad, &mut asr, None, Some(&mut diarizer))
                .unwrap();

        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "diarizationRuntime=onnx"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "speakerEmbeddingProvider=onnx"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "speakerEmbeddingDimension=2"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "diarizationBaseline=false"));
        assert!(!response
            .diagnostics
            .iter()
            .any(|item| item == "diarizationBaseline=heuristic-native"));
        assert_eq!(
            response.transcript.segments[0].speaker.as_deref(),
            Some("SPEAKER_ONNX")
        );
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn native_onnx_diarization_missing_bundle_returns_setup_error() {
        let mut request = sample_request();
        request.diarization = DiarizationOptions {
            enabled: true,
            speaker: SpeakerDiarizationOptions {
                speaker_embedding_model_bundle: Some(PathBuf::from(
                    "/definitely/missing/onnx-speaker-model",
                )),
                speaker_embedding_dimension: Some(2),
                ..SpeakerDiarizationOptions::default()
            },
            ..DiarizationOptions::default()
        };
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;
        let mut diarizer = NativeSpeakerDiarizationProvider;

        let error =
            run_native_transcription_pipeline(request, &mut vad, &mut asr, Some(&mut diarizer))
                .unwrap_err()
                .to_string();

        assert!(error.contains("setup_error"));
        assert!(error.contains("ONNX speaker embedding"));
    }

    #[cfg(all(feature = "diarization", feature = "onnx"))]
    #[test]
    #[ignore = "requires RUN_NATIVE_SPEAKER_MODEL_TESTS=1 and caller-owned local ONNX speaker model/audio"]
    fn native_onnx_diarization_smoke_when_requested(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if std::env::var("RUN_NATIVE_SPEAKER_MODEL_TESTS").as_deref() != Ok("1") {
            eprintln!(
                "skipping native ONNX diarization smoke; set RUN_NATIVE_SPEAKER_MODEL_TESTS=1"
            );
            return Ok(());
        }

        let bundle_path = std::env::var_os("SPEAKER_EMBEDDING_MODEL_BUNDLE")
            .map(PathBuf::from)
            .ok_or("SPEAKER_EMBEDDING_MODEL_BUNDLE is required")?;
        let audio_path = std::env::var_os("DIARIZATION_AUDIO_PATH")
            .map(PathBuf::from)
            .ok_or("DIARIZATION_AUDIO_PATH is required")?;
        let embedding_dimension = std::env::var("SPEAKER_EMBEDDING_DIMENSION")
            .ok()
            .map(|value| value.parse::<usize>())
            .transpose()?
            .unwrap_or(192);
        let model_file =
            optional_smoke_env_value(std::env::var("SPEAKER_EMBEDDING_MODEL_FILE").ok());
        let input_name =
            optional_smoke_env_value(std::env::var("SPEAKER_EMBEDDING_INPUT_NAME").ok());
        let output_name =
            optional_smoke_env_value(std::env::var("SPEAKER_EMBEDDING_OUTPUT_NAME").ok());
        let model_path = resolve_onnx_smoke_model_path(&bundle_path, model_file.as_deref())?;
        eprintln!("speakerEmbeddingResolvedModelPath={}", model_path.display());
        eprintln!("speakerEmbeddingExpectedDimension={embedding_dimension}");
        eprintln!(
            "speakerEmbeddingConfiguredInputName={}",
            input_name.as_deref().unwrap_or("<auto>")
        );
        eprintln!(
            "speakerEmbeddingConfiguredOutputName={}",
            output_name.as_deref().unwrap_or("<auto>")
        );
        let static_metadata = runtime_onnx::inspect_model_metadata(&model_path)?;
        eprintln!("onnxStaticMetadata=ok");
        for diagnostic in runtime_onnx::inspect_model_graph_diagnostics(&model_path)? {
            eprintln!("{diagnostic}");
        }
        eprintln!(
            "onnxLoadMode={}",
            std::env::var("ONNX_RUNTIME_LOAD_MODE").unwrap_or_else(|_| "file".to_string())
        );
        eprintln!(
            "onnxRuntimeDylib={}",
            if std::env::var_os("ORT_DYLIB_PATH").is_some() {
                "set"
            } else {
                "unset"
            }
        );
        if let Some(input) = static_metadata.inputs.first() {
            eprintln!("speakerEmbeddingStaticInputName={}", input.name);
            eprintln!(
                "speakerEmbeddingStaticInputDimensions={}",
                format_onnx_smoke_dimensions(&input.dimensions)
            );
        }
        if let Some(output) = static_metadata.outputs.first() {
            eprintln!("speakerEmbeddingStaticOutputName={}", output.name);
            eprintln!(
                "speakerEmbeddingStaticOutputDimensions={}",
                format_onnx_smoke_dimensions(&output.dimensions)
            );
        }
        eprintln!(
            "onnxSessionOptions=cpu-single-threaded,no-memory-pattern,graph-optimization-disabled"
        );
        let samples = local_16khz_wav_samples(&audio_path)?;
        let duration_seconds = samples.len() as f64 / 16_000.0;
        let midpoint = (duration_seconds / 2.0).clamp(1.0 / 16_000.0, duration_seconds);
        let first_end = midpoint.min(duration_seconds);
        let second_start = first_end;
        let second_end = duration_seconds;

        let mut request = TranscriptionPipelineRequest {
            source: TranscriptionSource::Samples {
                samples,
                sample_rate: 16_000,
                channels: 1,
                source: Some(audio_path.to_string_lossy().into_owned()),
            },
            provider: TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions::default()),
            vad: VadOptions::default(),
            alignment: AlignmentOptions::default(),
            diarization: DiarizationOptions {
                enabled: true,
                speaker: SpeakerDiarizationOptions {
                    speaker_embedding_model_bundle: Some(bundle_path),
                    speaker_embedding_model_file: model_file,
                    speaker_embedding_input_name: input_name,
                    speaker_embedding_output_name: output_name,
                    speaker_embedding_dimension: Some(embedding_dimension),
                    speaker_embedding_sample_rate: Some(16_000),
                    assignment_policy: SpeakerAssignmentPolicy::StrictContained,
                    ..SpeakerDiarizationOptions::default()
                },
                ..DiarizationOptions::default()
            },
            output: TranscriptionOutputOptions::default(),
        };
        if second_end > second_start {
            request.vad.enabled = true;
        }

        let segments = if second_end > second_start {
            vec![
                SpeechActivitySegment::new(0.0, first_end, 1.0)?,
                SpeechActivitySegment::new(second_start, second_end, 1.0)?,
            ]
        } else {
            vec![SpeechActivitySegment::new(0.0, first_end, 1.0)?]
        };
        let mut vad = FixedVadProvider { segments };
        let mut asr = MockAsrProvider;
        let mut diarizer = NativeSpeakerDiarizationProvider;

        let response =
            run_native_transcription_pipeline(request, &mut vad, &mut asr, Some(&mut diarizer))?;
        eprintln!("{}", response.diagnostics.join("\n"));

        assert!(response.accepted);
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "diarizationRuntime=onnx"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "speakerEmbeddingProvider=onnx"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == &format!("speakerEmbeddingDimension={embedding_dimension}")));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "diarizationBaseline=false"));
        assert!(response.transcript.segments.iter().all(|segment| segment
            .speaker
            .as_deref()
            .is_some_and(|speaker| { !speaker.trim().is_empty() })));
        normalize_transcription_contract(response.transcript.clone())
            .map_err(|error| format!("transcript speaker assignment must validate: {error}"))?;
        Ok(())
    }

    #[cfg(all(feature = "diarization", feature = "onnx"))]
    fn optional_smoke_env_value(value: Option<String>) -> Option<String> {
        value.filter(|value| !value.trim().is_empty())
    }

    #[cfg(all(feature = "diarization", feature = "onnx"))]
    fn resolve_onnx_smoke_model_path(
        bundle_path: &Path,
        model_file: Option<&str>,
    ) -> std::result::Result<PathBuf, Box<dyn std::error::Error>> {
        let model_file = model_file.unwrap_or("model.onnx");
        if bundle_path.is_file() {
            return Ok(bundle_path.to_path_buf());
        }
        #[cfg(feature = "model-bundles")]
        {
            let manifest_path = bundle_path.join("manifest.json");
            if manifest_path.is_file() {
                let bundle = model_runtime::ModelBundle::load(&manifest_path)?;
                for file in bundle.manifest.files.values() {
                    if file.remote_path == model_file
                        || file.remote_path.ends_with(model_file)
                        || file.local_path.ends_with(model_file)
                    {
                        if let Some(path) = bundle.file_path(&file.remote_path) {
                            return Ok(path);
                        }
                    }
                }
            }
        }
        Ok(bundle_path.join(model_file))
    }

    #[cfg(all(feature = "diarization", feature = "onnx"))]
    fn format_onnx_smoke_dimensions(dimensions: &[runtime_onnx::OnnxDimension]) -> String {
        let values = dimensions
            .iter()
            .map(|dimension| match dimension {
                runtime_onnx::OnnxDimension::Fixed(value) => value.to_string(),
                runtime_onnx::OnnxDimension::Symbolic(value) => value.clone(),
                runtime_onnx::OnnxDimension::Unknown => "unknown".to_string(),
            })
            .collect::<Vec<_>>()
            .join(",");
        format!("[{values}]")
    }

    #[test]
    fn native_pipeline_diarization_invalid_bounds_errors_before_provider() {
        let mut request = sample_request();
        request.diarization = DiarizationOptions {
            enabled: true,
            speaker: SpeakerDiarizationOptions {
                min_speakers: Some(3),
                max_speakers: Some(2),
                ..SpeakerDiarizationOptions::default()
            },
        };
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;
        let mut diarizer = PanickingDiarizationProvider { called: false };

        let error =
            run_transcription_pipeline(request, &mut vad, &mut asr, None, Some(&mut diarizer))
                .unwrap_err()
                .to_string();

        assert!(error.contains("invalid_request"));
        assert!(!diarizer.called);
    }

    #[test]
    #[cfg(not(feature = "diarization"))]
    fn native_diarization_without_feature_still_reports_no_provider() {
        let mut request = sample_request();
        request.diarization.enabled = true;
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;

        let error = run_transcription_pipeline(request, &mut vad, &mut asr, None, None)
            .unwrap_err()
            .to_string();

        assert!(error.contains("setup_error"));
        assert!(error.contains("no diarization provider is available"));
    }

    #[test]
    #[cfg(feature = "diarization")]
    fn native_speaker_diarization_provider_uses_transcript_spans_when_available(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let samples = (0..32_000)
            .map(|index| if index % 80 < 40 { 0.2 } else { -0.2 })
            .collect::<Vec<_>>();
        let audio = LoadedAudio {
            samples,
            sample_rate: 16_000,
            channels: 1,
            source: Some("synthetic".to_string()),
        };
        let transcript = transcript_with_words(vec![("hello", 0.20, 0.50), ("world", 1.00, 1.40)])?;
        let options = DiarizationOptions {
            enabled: true,
            speaker: SpeakerDiarizationOptions {
                model_id: "requested-native-speakers".to_string(),
                ..SpeakerDiarizationOptions::default()
            },
            ..DiarizationOptions::default()
        };
        let mut provider = NativeSpeakerDiarizationProvider;

        let response = provider.diarize(audio, &transcript, &options)?;

        assert_eq!(response.model_id, "requested-native-speakers");
        assert_eq!(
            response.runtime,
            audio_analysis_speakers::AudioRuntime::Heuristic
        );
        assert_eq!(response.segments.len(), 2);
        assert!((response.segments[0].start_seconds - 0.20).abs() < 0.001);
        assert!((response.segments[0].end_seconds - 0.50).abs() < 0.001);
        assert!((response.segments[1].start_seconds - 1.00).abs() < 0.001);
        assert!((response.segments[1].end_seconds - 1.40).abs() < 0.001);
        assert!(response
            .segments
            .iter()
            .all(|segment| segment.speaker.starts_with("speaker_")));
        Ok(())
    }

    #[test]
    #[cfg(feature = "diarization")]
    fn native_speaker_diarization_provider_applies_exact_speaker_bounds(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let audio = two_profile_loaded_audio();
        let transcript = transcript_with_words(vec![("hello", 0.20, 0.50), ("world", 1.00, 1.40)])?;
        let options = DiarizationOptions {
            enabled: true,
            speaker: SpeakerDiarizationOptions {
                min_speakers: Some(2),
                max_speakers: Some(2),
                ..SpeakerDiarizationOptions::default()
            },
            ..DiarizationOptions::default()
        };
        let mut provider = NativeSpeakerDiarizationProvider;

        let response = provider.diarize(audio, &transcript, &options)?;
        let speakers = response
            .segments
            .iter()
            .map(|segment| segment.speaker.clone())
            .collect::<std::collections::BTreeSet<_>>();

        assert_eq!(speakers.len(), 2, "{:?}", response.segments);
        assert!(speakers.contains("speaker_0"));
        assert!(speakers.contains("speaker_1"));
        Ok(())
    }

    #[test]
    #[cfg(feature = "diarization")]
    fn native_speaker_diarization_provider_applies_max_one_bound(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let audio = two_profile_loaded_audio();
        let transcript = transcript_with_words(vec![("hello", 0.20, 0.50), ("world", 1.00, 1.40)])?;
        let options = DiarizationOptions {
            enabled: true,
            speaker: SpeakerDiarizationOptions {
                max_speakers: Some(1),
                ..SpeakerDiarizationOptions::default()
            },
            ..DiarizationOptions::default()
        };
        let mut provider = NativeSpeakerDiarizationProvider;

        let response = provider.diarize(audio, &transcript, &options)?;
        let speakers = response
            .segments
            .iter()
            .map(|segment| segment.speaker.clone())
            .collect::<std::collections::BTreeSet<_>>();

        assert_eq!(speakers.len(), 1, "{:?}", response.segments);
        assert!(speakers.contains("speaker_0"));
        Ok(())
    }

    #[test]
    #[cfg(feature = "diarization")]
    fn native_speaker_diarization_provider_falls_back_without_transcript_timing(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let samples = (0..16_000)
            .map(|index| {
                if (1_000..8_000).contains(&index) {
                    0.2
                } else {
                    0.0
                }
            })
            .collect::<Vec<_>>();
        let audio = LoadedAudio {
            samples,
            sample_rate: 16_000,
            channels: 1,
            source: Some("synthetic".to_string()),
        };
        let segment = TranscriptSegmentContract::new(0, "hello without timing");
        let transcript =
            TranscriptionContract::from_segments(None, Some("en".to_string()), vec![segment])?;
        let options = DiarizationOptions {
            enabled: true,
            speaker: SpeakerDiarizationOptions {
                model_id: "fallback-native-speakers".to_string(),
                ..SpeakerDiarizationOptions::default()
            },
            ..DiarizationOptions::default()
        };
        let mut provider = NativeSpeakerDiarizationProvider;

        let response = provider.diarize(audio, &transcript, &options)?;

        assert_eq!(response.model_id, "fallback-native-speakers");
        assert_eq!(response.operation, "audio.speakers.diarize");
        assert_eq!(
            response.runtime,
            audio_analysis_speakers::AudioRuntime::Heuristic
        );
        assert!(!response.segments.is_empty());
        Ok(())
    }

    #[test]
    fn alignment_options_default_device_is_cpu() {
        assert_eq!(
            AlignmentOptions::default().device,
            NativeDevicePreference::Cpu
        );
    }

    #[test]
    fn alignment_options_deserializes_cuda_device() {
        let options: AlignmentOptions =
            serde_json::from_str(r#"{"device":"cuda"}"#).expect("alignment options should parse");

        assert_eq!(options.device, NativeDevicePreference::Cuda);
    }

    #[test]
    #[cfg(feature = "alignment")]
    fn native_pipeline_supplies_alignment_provider_when_enabled(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp = tempfile::tempdir()?;
        write_tiny_wav2vec2_bundle(temp.path());
        let mut request = sample_request();
        request.alignment = AlignmentOptions {
            enabled: true,
            model_bundle: Some(temp.path().to_path_buf()),
            ..AlignmentOptions::default()
        };
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;

        let response = run_native_transcription_pipeline(request, &mut vad, &mut asr, None)?;

        let alignment = response.alignment.as_ref().unwrap();
        assert_eq!(alignment.provider, "ctc-forced-aligner");
        assert_eq!(alignment.model_id, default_alignment_model());
        assert_eq!(alignment.word_count, 1);
        let word = &response.transcript.segments[0].words[0];
        assert_eq!(word.text, "hello");
        assert!(word.start_seconds.is_some());
        assert!(word.end_seconds.is_some());
        assert!(word
            .confidence
            .is_some_and(|confidence| confidence.is_finite()));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "alignmentModelSource=explicit-bundle"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "alignmentDevice=cpu"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "alignmentCuda=false"));
        response.transcript.validate_strict()?;
        Ok(())
    }

    #[test]
    fn native_pipeline_leaves_alignment_absent_when_disabled() {
        let request = sample_request();
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;

        let response =
            run_native_transcription_pipeline(request, &mut vad, &mut asr, None).unwrap();

        assert!(response.alignment.is_none());
        assert!(response
            .diagnostics
            .iter()
            .all(|item| !item.to_lowercase().contains("alignment")));
        response
            .transcript
            .validate_strict()
            .expect("native pipeline response should be strictly valid");
    }

    #[test]
    fn native_pipeline_passes_translate_task_to_asr_provider() {
        let mut request = sample_request();
        request.provider = TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
            task: TranscriptionTask::Translate,
            ..CandleWhisperOptions::default()
        });
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;

        let response =
            run_native_transcription_pipeline(request, &mut vad, &mut asr, None).unwrap();

        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "asrTask=translate"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "translationRuntime=whisper-task"));
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "translationTargetLanguage=en"));
        assert_eq!(response.transcript.language.as_deref(), Some("en"));
        assert!(response.alignment.is_none());
    }

    #[test]
    fn native_translate_rejects_ctc_alignment() {
        let mut request = sample_request();
        request.provider = TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
            task: TranscriptionTask::Translate,
            ..CandleWhisperOptions::default()
        });
        request.alignment.enabled = true;
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;

        let error = run_native_transcription_pipeline(request, &mut vad, &mut asr, None)
            .unwrap_err()
            .to_string();

        assert!(error.contains("invalid_request"));
        assert!(error.contains("translation output cannot be wav2vec2/CTC-aligned"));
    }

    #[test]
    fn native_translate_allows_diarization_with_segment_timings() {
        let mut request = sample_request();
        request.provider = TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
            task: TranscriptionTask::Translate,
            ..CandleWhisperOptions::default()
        });
        request.diarization.enabled = true;
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;
        let mut diarizer = MockDiarizationProvider;

        let response =
            run_native_transcription_pipeline(request, &mut vad, &mut asr, Some(&mut diarizer))
                .unwrap();

        assert!(response.diarization.is_some());
        assert_eq!(
            response.transcript.segments[0].speaker.as_deref(),
            Some("SPEAKER_00")
        );
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "asrTask=translate"));
    }

    #[test]
    #[cfg(feature = "alignment")]
    fn native_pipeline_runs_alignment_before_diarization() {
        let mut request = sample_request();
        request.alignment.enabled = true;
        request.diarization.enabled = true;
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;
        let mut diarizer = AlignmentAwareDiarizationProvider {
            saw_aligned_words: false,
        };

        let response =
            run_native_transcription_pipeline(request, &mut vad, &mut asr, Some(&mut diarizer))
                .unwrap();

        assert!(diarizer.saw_aligned_words);
        assert!(response.diarization.is_some());
        let word = &response.transcript.segments[0].words[0];
        assert!(word.start_seconds.is_some());
        assert!(word.end_seconds.is_some());
        assert_eq!(word.speaker.as_deref(), Some("SPEAKER_ALIGNED"));
        assert_eq!(
            response.transcript.segments[0].speaker.as_deref(),
            Some("SPEAKER_ALIGNED")
        );
    }

    #[test]
    #[cfg(not(feature = "alignment"))]
    fn native_pipeline_alignment_without_feature_reports_alignment_unsupported_runtime() {
        let mut request = sample_request();
        request.alignment.enabled = true;
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;

        let error = run_native_transcription_pipeline(request, &mut vad, &mut asr, None)
            .unwrap_err()
            .to_string();

        assert!(error.contains("unsupported_runtime"));
        assert!(error.contains("CTC alignment"));
        assert!(error.contains("alignment"));
        assert!(!error.contains("no alignment provider is available"));
    }

    #[test]
    #[cfg(all(feature = "alignment", feature = "candle"))]
    fn native_pipeline_with_tiny_wav2vec2_bundle_runs_model_backed_alignment(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp = tempfile::tempdir()?;
        write_tiny_wav2vec2_bundle(temp.path());
        let mut request = sample_request();
        request.alignment = AlignmentOptions {
            enabled: true,
            model_bundle: Some(temp.path().to_path_buf()),
            ..AlignmentOptions::default()
        };
        let mut vad = EnergyVadTranscriptionProvider;
        let mut asr = MockAsrProvider;

        let response = run_native_transcription_pipeline(request, &mut vad, &mut asr, None)?;

        let alignment = response.alignment.as_ref().unwrap();
        assert_eq!(alignment.provider, "ctc-forced-aligner");
        assert_eq!(alignment.word_count, 1);
        assert!(response
            .diagnostics
            .iter()
            .any(|item| item == "alignmentModelExecution=candle-wav2vec2"));
        let word = &response.transcript.segments[0].words[0];
        assert_eq!(word.text, "hello");
        assert!(word.start_seconds.is_some());
        assert!(word.end_seconds.is_some());
        assert!(word.confidence.is_some());
        response.transcript.validate_strict()?;
        Ok(())
    }

    #[test]
    fn missing_command_returns_setup_error() {
        let result = transcribe(TranscriptionPipelineRequest {
            source: TranscriptionSource::Path {
                path: PathBuf::from("missing.wav"),
            },
            provider: TranscriptionProviderSelection::ExternalWhisperX(WhisperXCommandOptions {
                command: PathBuf::from("definitely-missing-whisperx-command"),
                ..WhisperXCommandOptions::default()
            }),
            vad: VadOptions::default(),
            alignment: AlignmentOptions::default(),
            diarization: DiarizationOptions::default(),
            output: TranscriptionOutputOptions::default(),
        });

        let error = result.unwrap_err().to_string();
        assert!(error.contains("setup_error"));
        assert!(error.contains("not found"));
    }

    #[test]
    fn whisperx_json_bytes_prefers_current_source_stem() {
        let temp = tempfile::tempdir().expect("tempdir");
        fs::write(temp.path().join("a.json"), br#"{"text":"first"}"#).expect("a json");
        fs::write(temp.path().join("b.json"), br#"{"text":"second"}"#).expect("b json");

        let (path, bytes) =
            whisperx_json_bytes(Path::new("audio/b.wav"), temp.path(), b"{}").expect("json");

        assert_eq!(path, Some(temp.path().join("b.json")));
        assert_eq!(bytes, br#"{"text":"second"}"#);
    }

    #[test]
    fn reusable_candle_provider_reports_unsupported_without_candle_feature() {
        #[cfg(not(feature = "candle"))]
        {
            let mut provider = ReusableCandleWhisperTranscriber::new(CandleWhisperOptions {
                model_bundle: Some(PathBuf::from("bundle")),
                ..CandleWhisperOptions::default()
            });
            let error = provider
                .transcribe(AsrRequest {
                    audio: LoadedAudio {
                        samples: vec![0.0; 16],
                        sample_rate: 16_000,
                        channels: 1,
                        source: None,
                    },
                    chunks: vec![SpeechActivitySegment::new(0.0, 0.001, 1.0).unwrap()],
                    task: TranscriptionTask::Transcribe,
                    language: Some("en".to_string()),
                    model_id: "tiny.en".to_string(),
                })
                .unwrap_err()
                .to_string();

            assert!(
                error.contains("unsupported_runtime") || error.contains("setup_error"),
                "{error}"
            );
            assert!(
                error.contains("candle") || error.contains("bundle"),
                "{error}"
            );
        }
    }

    #[test]
    fn diarization_requires_token_before_spawn() {
        let result = transcribe(TranscriptionPipelineRequest {
            source: TranscriptionSource::Path {
                path: PathBuf::from("missing.wav"),
            },
            provider: TranscriptionProviderSelection::ExternalWhisperX(WhisperXCommandOptions {
                command: PathBuf::from("definitely-missing-whisperx-command"),
                diarize: true,
                hf_token_env: Some("VIDEO_ANALYSIS_TEST_MISSING_HF_TOKEN".to_string()),
                ..WhisperXCommandOptions::default()
            }),
            vad: VadOptions::default(),
            alignment: AlignmentOptions::default(),
            diarization: DiarizationOptions::default(),
            output: TranscriptionOutputOptions::default(),
        });

        let error = result.unwrap_err().to_string();
        assert!(error.contains("diarization requires"));
        assert!(!error.contains("not found"));
    }

    #[test]
    fn mock_command_output_round_trips() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp = tempfile::tempdir()?;
        let command = temp.path().join("mock-whisperx.sh");
        let output_dir = temp.path().join("out");
        fs::write(
            &command,
            format!(
                "#!/usr/bin/env bash\nmkdir -p \"{}\"\nprintf 'Transcript: [0.29 --> 1.47]  hello\\n'\ncat > \"{}/sample.json\" <<'JSON'\n{{\"segments\":[{{\"start\":0.0,\"end\":1.0,\"text\":\" hello \",\"speaker\":\"SPEAKER_00\",\"words\":[{{\"word\":\"hello\",\"start\":0.0,\"end\":0.8,\"score\":0.9,\"speaker\":\"SPEAKER_00\"}}]}}]}}\nJSON\n",
                output_dir.display(),
                output_dir.display()
            ),
        )?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(&command)?.permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&command, permissions)?;
        }

        let response = transcribe(TranscriptionPipelineRequest {
            source: TranscriptionSource::Path {
                path: PathBuf::from("speech.wav"),
            },
            provider: TranscriptionProviderSelection::ExternalWhisperX(WhisperXCommandOptions {
                command,
                output_dir: Some(output_dir),
                ..WhisperXCommandOptions::default()
            }),
            vad: VadOptions::default(),
            alignment: AlignmentOptions::default(),
            diarization: DiarizationOptions::default(),
            output: TranscriptionOutputOptions::default(),
        })?;

        assert!(response.accepted);
        assert_eq!(response.transcript.text.as_deref(), Some("hello"));
        assert_eq!(
            response.transcript.segments[0].words[0].speaker.as_deref(),
            Some("SPEAKER_00")
        );
        assert_eq!(response.vad_segments.len(), 1);
        assert_eq!(response.vad_segments[0].start_seconds, 0.29);
        assert_eq!(response.vad_segments[0].end_seconds, 1.47);
        Ok(())
    }

    #[test]
    fn whisperx_stdout_vad_segments_parses_transcript_lines() {
        let segments = whisperx_stdout_vad_segments(
            b"noise\nTranscript: [0.29 --> 1.47]  This is a test.\nTranscript: [2.00 --> 3.25]  More speech.\n",
        );

        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].start_seconds, 0.29);
        assert_eq!(segments[0].end_seconds, 1.47);
        assert_eq!(segments[1].start_seconds, 2.0);
        assert_eq!(segments[1].end_seconds, 3.25);
    }

    #[test]
    fn whisperx_args_include_alignment_parity_flags() {
        let args = whisperx_args(
            Path::new("speech.wav"),
            Path::new("out"),
            &WhisperXCommandOptions {
                align_model: Some("facebook/wav2vec2-base-960h".to_string()),
                model_dir: Some(PathBuf::from("models")),
                model_cache_only: true,
                no_align: true,
                interpolate_method: AlignmentInterpolationMethod::Linear,
                return_char_alignments: true,
                ..WhisperXCommandOptions::default()
            },
            None,
        );

        assert!(args.iter().any(|arg| arg == "--no_align"));
        assert!(args
            .windows(2)
            .any(|pair| pair[0] == "--align_model" && pair[1] == "facebook/wav2vec2-base-960h"));
        assert!(args
            .windows(2)
            .any(|pair| pair[0] == "--model_dir" && pair[1] == "models"));
        assert!(args.iter().any(|arg| arg == "--model_cache_only"));
        assert!(args
            .windows(2)
            .any(|pair| pair[0] == "--interpolate_method" && pair[1] == "linear"));
        assert!(args.iter().any(|arg| arg == "--return_char_alignments"));
    }

    #[test]
    fn whisperx_args_include_task_translate() {
        let args = whisperx_args(
            Path::new("speech.wav"),
            Path::new("out"),
            &WhisperXCommandOptions {
                task: TranscriptionTask::Translate,
                ..WhisperXCommandOptions::default()
            },
            None,
        );

        assert!(args
            .windows(2)
            .any(|pair| pair[0] == "--task" && pair[1] == "translate"));
    }

    #[test]
    fn timeout_returns_typed_error() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp = tempfile::tempdir()?;
        let command = temp.path().join("slow-whisperx.sh");
        fs::write(&command, "#!/usr/bin/env bash\nsleep 2\n")?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(&command)?.permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&command, permissions)?;
        }

        let result = transcribe(TranscriptionPipelineRequest {
            source: TranscriptionSource::Path {
                path: PathBuf::from("speech.wav"),
            },
            provider: TranscriptionProviderSelection::ExternalWhisperX(WhisperXCommandOptions {
                command,
                timeout_seconds: Some(1),
                ..WhisperXCommandOptions::default()
            }),
            vad: VadOptions::default(),
            alignment: AlignmentOptions::default(),
            diarization: DiarizationOptions::default(),
            output: TranscriptionOutputOptions::default(),
        });
        assert!(result.unwrap_err().to_string().contains("timeout"));
        Ok(())
    }

    #[test]
    #[ignore]
    fn native_whisper_hf_cache_smoke() {
        if std::env::var("RUN_NATIVE_WHISPER_MODEL_CACHE_TESTS").as_deref() != Ok("1") {
            eprintln!(
                "skipping native Whisper HF cache smoke; set RUN_NATIVE_WHISPER_MODEL_CACHE_TESTS=1"
            );
            return;
        }
        #[cfg(not(all(feature = "candle", feature = "model-bundles")))]
        panic!("native Whisper HF cache smoke requires candle,model-bundles features");

        #[cfg(all(feature = "candle", feature = "model-bundles"))]
        {
            let audio_path = std::env::var_os("TRANSCRIPTION_AUDIO_PATH")
                .map(PathBuf::from)
                .expect("TRANSCRIPTION_AUDIO_PATH is required");
            let model_dir = std::env::var_os("TRANSCRIPTION_MODEL_DIR")
                .map(PathBuf::from)
                .expect("TRANSCRIPTION_MODEL_DIR is required");
            let model_id =
                std::env::var("TRANSCRIPTION_MODEL_ID").unwrap_or_else(|_| "tiny.en".to_string());
            let response = transcribe(TranscriptionPipelineRequest {
                source: TranscriptionSource::Path { path: audio_path },
                provider: TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
                    model_id,
                    device: NativeDevicePreference::Cpu,
                    language: Some("en".to_string()),
                    model_dir: Some(model_dir),
                    model_cache_only: true,
                    ..CandleWhisperOptions::default()
                }),
                vad: VadOptions::default(),
                alignment: AlignmentOptions {
                    enabled: false,
                    ..AlignmentOptions::default()
                },
                diarization: DiarizationOptions::default(),
                output: TranscriptionOutputOptions::default(),
            })
            .expect("native Candle Whisper HF cache transcription should run");
            eprintln!("{}", response.diagnostics.join("\n"));
            assert!(response.accepted);
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "asrModelSource=hugging-face-cache"));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item.starts_with("asrModelResolved=")));
        }
    }

    #[test]
    #[ignore]
    fn candle_whisper_cuda_smoke_when_requested() {
        if std::env::var("RUN_NATIVE_TRANSCRIPTION_TESTS").as_deref() != Ok("1") {
            eprintln!("skipping native transcription smoke; set RUN_NATIVE_TRANSCRIPTION_TESTS=1");
            return;
        }
        #[cfg(not(all(feature = "candle", feature = "cuda", feature = "model-bundles")))]
        panic!("native transcription smoke requires candle,cuda,model-bundles features");

        #[cfg(all(feature = "candle", feature = "cuda", feature = "model-bundles"))]
        {
            let bundle = std::env::var_os("TRANSCRIPTION_MODEL_BUNDLE")
                .map(PathBuf::from)
                .expect("TRANSCRIPTION_MODEL_BUNDLE is required");
            let audio_path = std::env::var_os("TRANSCRIPTION_AUDIO_PATH")
                .map(PathBuf::from)
                .expect("TRANSCRIPTION_AUDIO_PATH is required");
            let response = transcribe(TranscriptionPipelineRequest {
                source: TranscriptionSource::Path { path: audio_path },
                provider: TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
                    model_id: "openai/whisper-tiny".to_string(),
                    device: NativeDevicePreference::Cuda,
                    language: Some("en".to_string()),
                    model_bundle: Some(bundle),
                    ..CandleWhisperOptions::default()
                }),
                vad: VadOptions::default(),
                alignment: AlignmentOptions::default(),
                diarization: DiarizationOptions::default(),
                output: TranscriptionOutputOptions::default(),
            })
            .expect("native Candle Whisper CUDA transcription should run");
            eprintln!("{}", response.diagnostics.join("\n"));
            assert!(response.accepted);
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "provider=candle-whisper"));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "device=cuda:0"));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item.starts_with("modelId=")));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item.starts_with("bundle=")));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item.starts_with("chunkCount=")));
            assert!(response.diagnostics.iter().any(|item| item == "cuda=true"));
            assert!(
                response
                    .transcript
                    .text
                    .as_deref()
                    .is_some_and(|text| !text.trim().is_empty())
                    || !response.transcript.segments.is_empty()
            );
        }
    }

    #[test]
    #[ignore]
    fn candle_whisper_cuda_translate_smoke_when_requested() {
        if std::env::var("RUN_NATIVE_TRANSLATION_TESTS").as_deref() != Ok("1") {
            eprintln!("skipping native translation smoke; set RUN_NATIVE_TRANSLATION_TESTS=1");
            return;
        }
        #[cfg(not(all(feature = "candle", feature = "cuda", feature = "model-bundles")))]
        panic!("native translation smoke requires candle,cuda,model-bundles features");

        #[cfg(all(feature = "candle", feature = "cuda", feature = "model-bundles"))]
        {
            let bundle = std::env::var_os("TRANSCRIPTION_MODEL_BUNDLE")
                .map(PathBuf::from)
                .expect("TRANSCRIPTION_MODEL_BUNDLE is required");
            let audio_path = std::env::var_os("TRANSCRIPTION_AUDIO_PATH")
                .map(PathBuf::from)
                .expect("TRANSCRIPTION_AUDIO_PATH is required");
            let response = transcribe(TranscriptionPipelineRequest {
                source: TranscriptionSource::Path { path: audio_path },
                provider: TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
                    model_id: "openai/whisper-tiny".to_string(),
                    task: TranscriptionTask::Translate,
                    device: NativeDevicePreference::Cuda,
                    model_bundle: Some(bundle),
                    ..CandleWhisperOptions::default()
                }),
                vad: VadOptions::default(),
                alignment: AlignmentOptions {
                    enabled: false,
                    ..AlignmentOptions::default()
                },
                diarization: DiarizationOptions::default(),
                output: TranscriptionOutputOptions::default(),
            })
            .expect("native Candle Whisper CUDA translation should run");
            eprintln!("{}", response.diagnostics.join("\n"));
            assert!(response.accepted);
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "provider=candle-whisper"));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "device=cuda:0"));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "asrTask=translate"));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "translationRuntime=whisper-task"));
            assert_eq!(response.transcript.language.as_deref(), Some("en"));
            assert!(
                response
                    .transcript
                    .text
                    .as_deref()
                    .is_some_and(|text| !text.trim().is_empty())
                    || !response.transcript.segments.is_empty()
            );
        }
    }

    #[test]
    #[ignore]
    fn ctc_alignment_wav2vec2_smoke_when_requested() {
        if matches!(
            std::env::var("RUN_NATIVE_ALIGNMENT_TESTS").as_deref(),
            Ok("0" | "false" | "FALSE")
        ) {
            eprintln!("skipping native alignment smoke; RUN_NATIVE_ALIGNMENT_TESTS disables it");
            return;
        }
        #[cfg(not(all(feature = "candle", feature = "alignment", feature = "model-bundles")))]
        panic!("native alignment smoke requires candle,alignment,model-bundles features");

        #[cfg(all(feature = "candle", feature = "alignment", feature = "model-bundles"))]
        {
            let temp = tempfile::tempdir().expect("alignment smoke tempdir should be created");
            let (bundle, bundle_source) = default_alignment_smoke_bundle(temp.path())
                .expect("alignment smoke should resolve a default wav2vec2 bundle");
            let audio_path = default_alignment_smoke_audio_path(temp.path())
                .expect("alignment smoke should resolve a default WAV path");
            let layout = native_wav2vec2::inspect_wav2vec2_bundle_layout(&bundle)
                .expect("alignment smoke should inspect wav2vec2 bundle layout");
            eprintln!(
                "alignment smoke defaults: bundleSource={bundle_source} bundle={} audio={}",
                bundle.display(),
                audio_path.display()
            );
            eprintln!("wav2vec2 layout report: {layout:#?}");
            let transcript_text = std::env::var("ALIGNMENT_TRANSCRIPT_TEXT")
                .unwrap_or_else(|_| "hello world".to_string());
            let audio = LoadedAudio::mono_16khz_from_source(&TranscriptionSource::Path {
                path: audio_path,
            })
            .expect("alignment smoke requires readable WAV audio");
            let mut segment = TranscriptSegmentContract::new(0, transcript_text.clone());
            segment.start_seconds = Some(0.0);
            segment.end_seconds = Some(audio.duration_seconds().clamp(1.0 / 16_000.0, 1.0));
            let transcript =
                TranscriptionContract::from_segments(None, Some("en".to_string()), vec![segment])
                    .expect("alignment smoke transcript should validate");
            let request = AlignmentRequest {
                audio,
                transcript,
                language: Some("en".to_string()),
                model_id: default_alignment_model(),
            };
            let mut aligner = CtcForcedAligner {
                options: AlignmentOptions {
                    enabled: true,
                    model_bundle: Some(bundle),
                    return_char_alignments: true,
                    ..AlignmentOptions::default()
                },
            };
            let response = aligner
                .align(request)
                .expect("native wav2vec2/CTC alignment should run");
            eprintln!("{}", response.diagnostics.join("\n"));
            assert!(!response.words.is_empty());
            assert!(response
                .words
                .iter()
                .all(|word| word.end_seconds >= word.start_seconds));
            assert!(response.words.iter().all(|word| word.confidence.is_some()));
            assert!(!response.chars.is_empty());
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "alignmentProvider=ctc-forced-aligner"));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "alignmentModelExecution=candle-wav2vec2"));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "alignmentModelSource=explicit-bundle"));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "alignmentInterpolateMethod=nearest"));
            assert!(response
                .diagnostics
                .iter()
                .any(|item| item == "returnCharAlignments=true"));
        }
    }
}

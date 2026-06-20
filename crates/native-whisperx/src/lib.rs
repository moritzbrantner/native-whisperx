#![doc = include_str!("../README.md")]

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

mod silero_vad;
mod speaker_directory;

#[cfg(feature = "diarization")]
use audio_analysis_speakers::{
    AudioRuntime, DiarizationSegment, DiarizedSpeaker, EnergyVadConfig,
    EnergyVoiceActivityDetector, SpeakerDiarizer, SpeakerIdentificationOptions, SpeakerLibrary,
    SpeakerSegmentPrediction, SpeechSpan, WindowedSpeakerDiarizer,
};
use audio_analysis_speakers::{
    SpeakerAudio, SpeakerEmbedding, SpeakerEmbeddingExtractor, SpectralSpeakerEmbedder,
};
use audio_analysis_transcription::LoadedAudio;
#[cfg(feature = "diarization")]
use audio_analysis_transcription::SpeakerDiarizationResponse;
pub use audio_analysis_transcription::{
    AlignmentInterpolationMethod, TranscriptionPipelineRequest, TranscriptionPipelineResponse,
};
#[cfg(feature = "translation")]
use candle_core::IndexOp;
pub use speaker_directory::{
    delete_speaker_profile, global_speaker_directory, list_speaker_profiles,
    local_speaker_directory, read_speaker_directory_state, rebuild_speaker_trace,
    reject_draft_speaker_profile_creation, resolve_speaker_directory, speaker_library_path,
    speaker_trace_path, update_speaker_profile, validate_speaker_library,
    validate_speaker_library_file, ResolvedSpeakerDirectory, ResolvedSpeakerDirectoryScope,
    SpeakerCorrectionRange, SpeakerDirectoryScope, SpeakerDirectorySelection,
    SpeakerDirectoryState, SpeakerDirectoryStateScope, SpeakerLibraryState,
    SpeakerLibraryValidation, SpeakerLibraryValidationStatus, SpeakerProfileEdit,
    SpeakerProfileState, SpeakerProfileSummary, SpeakerTrace, SpeakerTraceError, SpeakerTraceFile,
    SpeakerTraceRebuildReport, SpeakerTraceRebuildStats, SpeakerTraceSpan, SpeakerTraceSpeaker,
    SpeakerTraceSpeakerKind, SpeakerTraceState, SpeakerTraceStateStatus,
    GLOBAL_SPEAKER_DIRECTORY_APP, GLOBAL_SPEAKER_DIRECTORY_NAME, LOCAL_SPEAKER_DIRECTORY,
    SPEAKER_LIBRARY_FILE, SPEAKER_TRACE_FILE,
};
pub use text_transcripts::TranscriptionContract;

#[cfg(feature = "diarization")]
use audio_analysis_transcription::NativeSpeakerDiarizationProvider;
use audio_analysis_transcription::{
    run_transcription_pipeline_with_observer, transcribe, AlignmentOptions, CandleWhisperOptions,
    CandleWhisperTranscriber, CtcForcedAligner, DiarizationOptions, EnergyVadTranscriptionProvider,
    ForcedAlignmentProvider, NativeDevicePreference, SpeakerAssignmentPolicy,
    SpeakerDiarizationOptions, SpeechActivitySegment, TranscriptDiarizationProvider,
    TranscriptionOutputOptions, TranscriptionPipelineEvent, TranscriptionPipelineObserver,
    TranscriptionProviderSelection, TranscriptionSource,
    TranscriptionTask as UpstreamTranscriptionTask, TranscriptionVadProvider, VadOptions,
    WhisperXCommandOptions, WhisperXDevice,
};
#[cfg(feature = "pyannote-vad")]
use silero_vad::{PyannoteVadOptions, PyannoteVadTranscriptionProvider};
#[cfg(feature = "silero-vad")]
use silero_vad::{SileroVadOptions, SileroVadTranscriptionProvider};
use text_transcripts::parse_whisperx_json;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWhisperxConfig {
    pub input: InputSource,
    #[serde(default)]
    pub asr: AsrConfig,
    #[serde(default)]
    pub translation: TranslationConfig,
    #[serde(default)]
    pub vad: VadConfig,
    #[serde(default)]
    pub alignment: AlignmentConfig,
    #[serde(default)]
    pub diarization: DiarizationConfig,
    #[serde(default)]
    pub output: OutputConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum InputSource {
    Path {
        path: PathBuf,
    },
    Samples {
        samples: Vec<f32>,
        sample_rate: u32,
        channels: u16,
        #[serde(default)]
        source: Option<String>,
    },
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub model_bundle: Option<PathBuf>,
    #[serde(default)]
    pub model_dir: Option<PathBuf>,
    #[serde(default)]
    pub model_cache_only: bool,
    #[serde(default)]
    pub source_language: Option<String>,
    #[serde(default)]
    pub target_language: Option<String>,
    #[serde(default = "default_translation_max_new_tokens")]
    pub max_new_tokens: usize,
}

impl Default for TranslationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            model_id: None,
            model_bundle: None,
            model_dir: None,
            model_cache_only: false,
            source_language: None,
            target_language: None,
            max_new_tokens: default_translation_max_new_tokens(),
        }
    }
}

fn default_translation_max_new_tokens() -> usize {
    256
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VadConfig {
    #[serde(default = "default_vad_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub method: VadMethod,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputConfig {
    #[serde(default)]
    pub output_dir: Option<PathBuf>,
    #[serde(default = "default_output_formats")]
    pub formats: Vec<OutputFormat>,
    #[serde(default)]
    pub basename: Option<String>,
    #[serde(default = "default_pretty_json")]
    pub pretty_json: bool,
    #[serde(default)]
    pub subtitles: SubtitleConfig,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            formats: default_output_formats(),
            basename: None,
            pretty_json: true,
            subtitles: SubtitleConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    All,
    Json,
    #[serde(rename = "native-json", alias = "nativeJson")]
    NativeJson,
    Srt,
    Vtt,
    Txt,
    Tsv,
    #[serde(rename = "aud", alias = "audacity")]
    Audacity,
}

impl OutputFormat {
    pub fn extension(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Json => "json",
            Self::NativeJson => "native.json",
            Self::Srt => "srt",
            Self::Vtt => "vtt",
            Self::Txt => "txt",
            Self::Tsv => "tsv",
            Self::Audacity => "aud",
        }
    }

    pub fn as_transcription_format(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Json => "json",
            Self::NativeJson => "native-json",
            Self::Srt => "srt",
            Self::Vtt => "vtt",
            Self::Txt => "txt",
            Self::Tsv => "tsv",
            Self::Audacity => "aud",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtitleConfig {
    #[serde(default)]
    pub max_line_width: Option<usize>,
    #[serde(default)]
    pub max_line_count: Option<usize>,
    #[serde(default)]
    pub highlight_words: bool,
    #[serde(default)]
    pub segment_resolution: SegmentResolution,
}

impl Default for SubtitleConfig {
    fn default() -> Self {
        Self {
            max_line_width: None,
            max_line_count: None,
            highlight_words: false,
            segment_resolution: SegmentResolution::Sentence,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SegmentResolution {
    #[default]
    #[serde(alias = "segment")]
    Sentence,
    Chunk,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWhisperxReport {
    pub response: TranscriptionPipelineResponse,
    #[serde(default)]
    pub output_files: Vec<OutputFile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputFile {
    pub format: OutputFormat,
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerCorrectionReport {
    pub transcript: TranscriptionContract,
    pub speaker_directory_path: PathBuf,
    pub profile_id: String,
    pub label: String,
    pub corrected_from: String,
    pub enrolled_seconds: f64,
    pub updated_existing_profile: bool,
    pub output_files: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpeakerCorrectionRequest {
    pub transcript: TranscriptionContract,
    pub audio: InputSource,
    pub from_speaker: String,
    pub to_label: String,
    pub speaker_id: Option<String>,
    pub ranges: Vec<SpeakerCorrectionRange>,
    pub speaker_directory: SpeakerDirectorySelection,
    pub output: OutputConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpectedOutputFile {
    pub format: OutputFormat,
    pub path: PathBuf,
    #[serde(default)]
    pub comparison: OutputComparisonMode,
    #[serde(default = "default_true")]
    pub gating: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputComparisonMode {
    #[default]
    Exact,
    JsonSemantic,
    SubtitleSemantic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpectedOutputComparison {
    pub format: OutputFormat,
    #[serde(default)]
    pub comparison: OutputComparisonMode,
    #[serde(default = "default_true")]
    pub gating: bool,
    pub expected_path: PathBuf,
    pub actual_path: Option<PathBuf>,
    pub passed: bool,
    pub difference: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExpectedTranscriptTarget {
    #[default]
    Native,
    Whisperx,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityConfig {
    pub input: PathBuf,
    #[serde(default)]
    pub expected_json: Option<PathBuf>,
    #[serde(default)]
    pub expected_target: ExpectedTranscriptTarget,
    #[serde(default)]
    pub comparison: ParityComparisonConfig,
    #[serde(default)]
    pub native_asr: AsrConfig,
    #[serde(default)]
    pub translation: TranslationConfig,
    #[serde(default)]
    pub vad: VadConfig,
    #[serde(default)]
    pub alignment: AlignmentConfig,
    #[serde(default)]
    pub diarization: DiarizationConfig,
    #[serde(default)]
    pub whisperx_diarization: Option<DiarizationConfig>,
    #[serde(default)]
    pub whisperx: ExternalWhisperxConfig,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub output: OutputConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityFixtureSuite {
    pub fixtures: Vec<ParityFixtureCase>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityFixtureCase {
    pub name: String,
    #[serde(default = "default_gating")]
    pub gating: bool,
    pub input: PathBuf,
    #[serde(default)]
    pub clip_seconds: Option<f64>,
    #[serde(default)]
    pub timeout_seconds: Option<u64>,
    #[serde(default)]
    pub expected_json: Option<PathBuf>,
    #[serde(default)]
    pub expected_target: ExpectedTranscriptTarget,
    #[serde(default)]
    pub comparison: ParityComparisonConfig,
    #[serde(default)]
    pub expected_outputs: Vec<ExpectedOutputFile>,
    #[serde(default)]
    pub native_asr: AsrConfig,
    #[serde(default)]
    pub translation: TranslationConfig,
    #[serde(default)]
    pub vad: VadConfig,
    #[serde(default)]
    pub alignment: AlignmentConfig,
    #[serde(default)]
    pub diarization: DiarizationConfig,
    #[serde(default)]
    pub whisperx_diarization: Option<DiarizationConfig>,
    #[serde(default)]
    pub whisperx: ExternalWhisperxConfig,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub output: OutputConfig,
    #[serde(default)]
    pub required_diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityComparisonConfig {
    #[serde(default = "default_true")]
    pub text: bool,
    #[serde(default = "default_true")]
    pub language: bool,
    #[serde(default = "default_true")]
    pub segment_text: bool,
    #[serde(default = "default_true")]
    pub word_text: bool,
    #[serde(default = "default_true")]
    pub char_count: bool,
    #[serde(default = "default_true")]
    pub char_content: bool,
    #[serde(default = "default_true")]
    pub segment_count: bool,
    #[serde(default = "default_true")]
    pub word_count: bool,
    #[serde(default = "default_true")]
    pub segment_timing: bool,
    #[serde(default = "default_true")]
    pub word_timing: bool,
    #[serde(default = "default_true")]
    pub speaker_turns: bool,
    #[serde(default = "default_true")]
    pub vad_segments: bool,
    #[serde(default = "default_true")]
    pub vad_segment_timing: bool,
    #[serde(default = "default_true")]
    pub vad_segment_count: bool,
}

impl Default for ParityComparisonConfig {
    fn default() -> Self {
        Self {
            text: true,
            language: true,
            segment_text: true,
            word_text: true,
            char_count: true,
            char_content: true,
            segment_count: true,
            word_count: true,
            segment_timing: true,
            word_timing: true,
            speaker_turns: true,
            vad_segments: true,
            vad_segment_timing: true,
            vad_segment_count: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityFixtureSuiteReport {
    pub passed: bool,
    pub cases: Vec<ParityFixtureCaseReport>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityFixtureCaseReport {
    pub name: String,
    #[serde(default)]
    pub gating: bool,
    pub passed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elapsed_seconds: Option<f64>,
    #[serde(default)]
    pub timed_out: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<ParityReport>,
    #[serde(default)]
    pub missing_required_diagnostics: Vec<String>,
    #[serde(default)]
    pub expected_output_matches: Vec<ExpectedOutputComparison>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub failure_summary: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityPreflightReport {
    pub passed: bool,
    pub manifest: PathBuf,
    pub root: PathBuf,
    pub whisperx_command: PathBuf,
    pub model_dir: PathBuf,
    pub source_checkout_tag: Option<String>,
    pub cases: Vec<ParityPreflightCaseReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityPreflightCaseReport {
    pub name: String,
    pub gating: bool,
    pub passed: bool,
    pub missing: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityReport {
    pub native_report: NativeWhisperxReport,
    pub whisperx_report: NativeWhisperxReport,
    #[serde(default)]
    pub expected: Option<TranscriptionContract>,
    #[serde(default)]
    pub expected_target: ExpectedTranscriptTarget,
    pub comparison: ParityComparison,
    pub expected_segment_count_matches: Option<bool>,
    pub expected_text_matches: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityComparison {
    pub text_matches: bool,
    #[serde(default)]
    pub language_matches: Option<bool>,
    #[serde(default)]
    pub segment_text_matches: Option<bool>,
    #[serde(default)]
    pub word_text_matches: Option<bool>,
    #[serde(default)]
    pub char_count_matches: Option<bool>,
    #[serde(default)]
    pub char_content_matches: Option<bool>,
    pub segment_count_matches: bool,
    pub word_count_matches: bool,
    pub segment_timing_matches: bool,
    pub word_timing_matches: bool,
    pub speaker_turns_match: bool,
    #[serde(default)]
    pub vad_segment_count_matches: Option<bool>,
    #[serde(default)]
    pub vad_segment_timing_matches: Option<bool>,
    pub confidence_compared: bool,
    pub passed: bool,
    pub tolerance: ParityTolerance,
    #[serde(default)]
    pub differences: Vec<String>,
    #[serde(default)]
    pub diagnostic_differences: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityTolerance {
    pub segment_seconds: f64,
    pub word_seconds: f64,
}

impl Default for ParityTolerance {
    fn default() -> Self {
        Self {
            segment_seconds: 0.100,
            word_seconds: 0.050,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NativeWhisperxError {
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("transcription failed: {0}")]
    Transcription(String),
    #[error("transcript import failed: {0}")]
    Import(String),
    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),
}

pub fn run(config: NativeWhisperxConfig) -> Result<NativeWhisperxReport, NativeWhisperxError> {
    let run_started = Instant::now();
    let request = build_transcription_request(&config)?;
    let mut response = if config.asr.provider == AsrProvider::Native && config.translation.enabled {
        run_native_with_translation(request, &config)?
    } else if config.asr.provider == AsrProvider::Native
        && matches!(config.vad.method, VadMethod::Silero | VadMethod::Pyannote)
    {
        run_native_with_selected_vad(request, &config)?
    } else {
        run_with_phase_observer(request, &config)?
    };
    append_native_alignment_diagnostics(&mut response, &config);
    append_native_diarization_diagnostics(&mut response, &config);
    save_draft_speakers_from_response(&mut response, &config)?;
    let output_started = Instant::now();
    let output_files = write_outputs_with_options(
        &response,
        &config.output,
        config.alignment.return_char_alignments,
    )?;
    response.diagnostics.push(format!(
        "phaseOutputSeconds={:.6}",
        output_started.elapsed().as_secs_f64()
    ));
    response.diagnostics.push(format!(
        "phaseNativeTotalSeconds={:.6}",
        run_started.elapsed().as_secs_f64()
    ));
    Ok(NativeWhisperxReport {
        response,
        output_files,
    })
}

fn run_with_phase_observer(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    if config.asr.provider != AsrProvider::Native {
        return transcribe(request)
            .map_err(|error| NativeWhisperxError::Transcription(error.to_string()));
    }

    let TranscriptionProviderSelection::CandleWhisper(options) = &request.provider else {
        return transcribe(request)
            .map_err(|error| NativeWhisperxError::Transcription(error.to_string()));
    };
    let mut vad = EnergyVadTranscriptionProvider;
    let mut asr_provider = CandleWhisperTranscriber::new(options.clone());

    #[cfg(feature = "diarization")]
    {
        let mut diarizer = native_diarization_provider(config)?;
        let diarization_provider = request
            .diarization
            .enabled
            .then_some(&mut diarizer as &mut dyn TranscriptDiarizationProvider);
        return run_native_with_optional_alignment(
            request,
            &mut vad,
            &mut asr_provider,
            diarization_provider,
        );
    }

    #[cfg(not(feature = "diarization"))]
    {
        run_native_with_optional_alignment(request, &mut vad, &mut asr_provider, None)
    }
}

fn append_native_diarization_diagnostics(
    response: &mut TranscriptionPipelineResponse,
    config: &NativeWhisperxConfig,
) {
    if config.asr.provider != AsrProvider::Native
        || !config.diarization.enabled
        || !is_pyannote_diarization_model(&config.diarization.model_id)
    {
        return;
    }

    for diagnostic in [
        "diarizationPhase=segmentation",
        "diarizationPhase=embedding",
        "diarizationPhase=plda",
        "diarizationPhase=vbx",
        "diarizationPhase=clustering",
    ] {
        if !response
            .diagnostics
            .iter()
            .any(|existing| existing == diagnostic)
        {
            response.diagnostics.push(diagnostic.to_string());
        }
    }
}

#[cfg(feature = "diarization")]
fn save_draft_speakers_from_response(
    response: &mut TranscriptionPipelineResponse,
    config: &NativeWhisperxConfig,
) -> Result<(), NativeWhisperxError> {
    if config.asr.provider != AsrProvider::Native
        || !config.diarization.enabled
        || config.diarization.disable_speaker_library
        || is_pyannote_diarization_model(&config.diarization.model_id)
    {
        return Ok(());
    }
    if !config.diarization.save_draft_speakers {
        response
            .diagnostics
            .push("speakerLibraryDraftProfilesSaved=0".to_string());
        return Ok(());
    }

    let Some(diarization) = &response.diarization else {
        response
            .diagnostics
            .push("speakerLibraryDraftProfilesSaved=0".to_string());
        return Ok(());
    };
    let current_dir = std::env::current_dir()?;
    let resolved = resolve_speaker_directory(&config.diarization.speaker_directory, &current_dir)?;
    let mut library =
        speaker_directory::load_speaker_library_if_present(&resolved.path)?.unwrap_or_default();
    let existing_labels = library
        .profiles()
        .map(|profile| profile.id().as_str().to_string())
        .collect::<std::collections::BTreeSet<_>>();
    let labels = diarization
        .segments
        .iter()
        .filter(|segment| is_transient_speaker_label(&segment.speaker))
        .map(|segment| segment.speaker.clone())
        .collect::<std::collections::BTreeSet<_>>();
    if labels.is_empty() {
        response
            .diagnostics
            .push("speakerLibraryDraftProfilesSaved=0".to_string());
        return Ok(());
    }

    let audio = LoadedAudio::mono_16khz_from_source(&map_input_source(&config.input))
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))?;
    let mut saved = 0usize;
    for label in labels {
        if existing_labels.contains(&label) {
            continue;
        }
        let ranges = diarization
            .segments
            .iter()
            .filter(|segment| segment.speaker == label)
            .map(|segment| SpeakerCorrectionRange {
                start_seconds: segment.start_seconds as f64,
                end_seconds: segment.end_seconds as f64,
            })
            .collect::<Vec<_>>();
        let embedding = speaker_embedding_for_ranges(&audio, &ranges)?;
        let draft_id = format!(
            "draft-{}-{}",
            slug_speaker_id(&label),
            current_unix_seconds()
        );
        let draft_label = format!("draft_{}", slug_speaker_id(&label));
        let mut metadata = BTreeMap::new();
        metadata.insert("status".to_string(), "draft".to_string());
        metadata.insert("detectedLabel".to_string(), label);
        let now = current_unix_seconds_string();
        metadata.insert("createdAt".to_string(), now.clone());
        metadata.insert("updatedAt".to_string(), now);
        let (updated, _) = speaker_directory::upsert_speaker_profile_embedding(
            &library,
            &draft_id,
            &draft_label,
            embedding,
            metadata,
        )?;
        library = updated;
        saved += 1;
    }
    if saved > 0 {
        speaker_directory::save_speaker_library(&resolved.path, &library)?;
    }
    response
        .diagnostics
        .push(format!("speakerLibraryDraftProfilesSaved={saved}"));
    Ok(())
}

#[cfg(not(feature = "diarization"))]
fn save_draft_speakers_from_response(
    _response: &mut TranscriptionPipelineResponse,
    _config: &NativeWhisperxConfig,
) -> Result<(), NativeWhisperxError> {
    Ok(())
}

#[cfg(feature = "diarization")]
fn is_transient_speaker_label(label: &str) -> bool {
    label
        .strip_prefix("speaker_")
        .is_some_and(|suffix| !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit()))
}

fn append_native_alignment_diagnostics(
    response: &mut TranscriptionPipelineResponse,
    config: &NativeWhisperxConfig,
) {
    if config.asr.provider != AsrProvider::Native || !config.alignment.enabled {
        return;
    }
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentModelId",
        format!(
            "alignmentModelId={}",
            canonical_alignment_model_id(&config.alignment.model_id)
        ),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentFallbackCount",
        "alignmentFallbackCount=0".to_string(),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentRetryCount",
        "alignmentRetryCount=0".to_string(),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentWordTimingMissingCount",
        format!(
            "alignmentWordTimingMissingCount={}",
            alignment_word_timing_missing_count(&response.transcript)
        ),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentCharTimingMissingCount",
        format!(
            "alignmentCharTimingMissingCount={}",
            if config.alignment.return_char_alignments {
                alignment_char_timing_missing_count(&response.transcript)
            } else {
                0
            }
        ),
    );
}

fn canonical_alignment_model_id(model_id: &str) -> &str {
    if model_id.eq_ignore_ascii_case("WAV2VEC2_ASR_BASE_960H") {
        "facebook/wav2vec2-base-960h"
    } else {
        model_id
    }
}

fn push_diagnostic_if_missing(diagnostics: &mut Vec<String>, key: &str, diagnostic: String) {
    let prefix = format!("{key}=");
    if diagnostics
        .iter()
        .any(|existing| existing.starts_with(&prefix))
    {
        return;
    }
    diagnostics.push(diagnostic);
}

#[cfg(feature = "diarization")]
#[derive(Debug, Clone)]
struct RuntimeSpeakerLibrary {
    path: PathBuf,
    profile_count: usize,
    filtered_draft_profiles: usize,
    use_draft_profiles: bool,
    library: SpeakerLibrary,
}

#[cfg(feature = "diarization")]
#[derive(Debug, Clone)]
enum RuntimeSpeakerLibraryStatus {
    NotRequested,
    Disabled,
    ExternalWhisperX,
    Missing(PathBuf),
    Loaded(RuntimeSpeakerLibrary),
}

#[cfg(feature = "diarization")]
impl RuntimeSpeakerLibraryStatus {
    fn library(&self) -> Option<SpeakerLibrary> {
        match self {
            Self::Loaded(library) => Some(library.library.clone()),
            _ => None,
        }
    }

    fn diagnostics(&self) -> Vec<String> {
        match self {
            Self::NotRequested => Vec::new(),
            Self::Disabled => vec!["speakerLibraryStatus=disabled".to_string()],
            Self::ExternalWhisperX => {
                vec!["speakerLibraryStatus=ignored-external-whisperx".to_string()]
            }
            Self::Missing(path) => vec![
                "speakerLibraryStatus=missing".to_string(),
                format!("speakerLibraryPath={}", path.display()),
            ],
            Self::Loaded(library) => vec![
                "speakerLibraryStatus=loaded".to_string(),
                format!("speakerLibraryPath={}", library.path.display()),
                format!("speakerLibraryProfiles={}", library.profile_count),
                format!(
                    "speakerLibraryDraftProfilesUsed={}",
                    library.use_draft_profiles
                ),
                format!(
                    "speakerLibraryDraftProfilesFiltered={}",
                    library.filtered_draft_profiles
                ),
            ],
        }
    }
}

#[cfg(feature = "diarization")]
fn runtime_speaker_library_status(
    config: &NativeWhisperxConfig,
) -> Result<RuntimeSpeakerLibraryStatus, NativeWhisperxError> {
    if !config.diarization.enabled {
        return Ok(RuntimeSpeakerLibraryStatus::NotRequested);
    }
    if config.asr.provider != AsrProvider::Native {
        return Ok(RuntimeSpeakerLibraryStatus::ExternalWhisperX);
    }
    if config.diarization.disable_speaker_library {
        return Ok(RuntimeSpeakerLibraryStatus::Disabled);
    }

    let current_dir = std::env::current_dir()?;
    let resolved = resolve_speaker_directory(&config.diarization.speaker_directory, &current_dir)?;
    let path = speaker_library_path(&resolved.path);
    match fs::read_to_string(&path) {
        Ok(json) => {
            let library = SpeakerLibrary::from_json_str(&json).map_err(|error| {
                NativeWhisperxError::InvalidConfig(format!(
                    "Speaker Library `{}` is malformed or incompatible: {error}",
                    path.display()
                ))
            })?;
            let (library, filtered_draft_profiles) =
                speaker_directory::filter_speaker_library_drafts(
                    &library,
                    config.diarization.use_draft_speakers,
                )?;
            Ok(RuntimeSpeakerLibraryStatus::Loaded(RuntimeSpeakerLibrary {
                path,
                profile_count: library.len(),
                filtered_draft_profiles,
                use_draft_profiles: config.diarization.use_draft_speakers,
                library,
            }))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(RuntimeSpeakerLibraryStatus::Missing(path))
        }
        Err(error) => Err(error.into()),
    }
}

#[cfg(feature = "diarization")]
#[derive(Debug, Clone)]
struct ConfiguredNativeDiarizationProvider {
    speaker_library: RuntimeSpeakerLibraryStatus,
}

#[cfg(feature = "diarization")]
fn native_diarization_provider(
    config: &NativeWhisperxConfig,
) -> Result<ConfiguredNativeDiarizationProvider, NativeWhisperxError> {
    Ok(ConfiguredNativeDiarizationProvider {
        speaker_library: runtime_speaker_library_status(config)?,
    })
}

#[cfg(feature = "diarization")]
impl TranscriptDiarizationProvider for ConfiguredNativeDiarizationProvider {
    fn provider_id(&self) -> &str {
        "native-speaker-diarization"
    }

    fn diarize(
        &mut self,
        audio: LoadedAudio,
        transcript: &TranscriptionContract,
        options: &DiarizationOptions,
    ) -> video_analysis_core::Result<SpeakerDiarizationResponse> {
        let mut response = if options.is_pyannote_model() {
            let mut provider = NativeSpeakerDiarizationProvider;
            provider.diarize(audio, transcript, options)?
        } else if let Some(library) = self.speaker_library.library() {
            diarize_with_speaker_library(audio, transcript, options, library)?
        } else {
            let mut provider = NativeSpeakerDiarizationProvider;
            provider.diarize(audio, transcript, options)?
        };
        response
            .diagnostics
            .extend(self.speaker_library.diagnostics());
        if options.is_pyannote_model()
            && matches!(self.speaker_library, RuntimeSpeakerLibraryStatus::Loaded(_))
        {
            response.diagnostics.push(
                "speakerLibraryStatus=loaded-but-pyannote-provider-does-not-expose-known-speaker-identification".to_string(),
            );
        }
        Ok(response)
    }
}

#[cfg(feature = "diarization")]
fn diarize_with_speaker_library(
    audio: LoadedAudio,
    transcript: &TranscriptionContract,
    options: &DiarizationOptions,
    library: SpeakerLibrary,
) -> video_analysis_core::Result<SpeakerDiarizationResponse> {
    validate_loaded_audio_for_diarization(&audio)?;
    if options.speaker_embedding_model_bundle.is_some() {
        return diarize_with_speaker_library_and_onnx_embeddings(
            audio, transcript, options, library,
        );
    }

    let speaker_audio = SpeakerAudio::mono(&audio.samples, audio.sample_rate)?;
    let embedder = SpectralSpeakerEmbedder::default();
    let spans = speech_spans_from_transcript_for_diarization(transcript, audio.duration_seconds())?;
    let result = if spans.is_empty() {
        let vad = EnergyVoiceActivityDetector::new(EnergyVadConfig::default())?;
        let mut diarizer = runtime_library_diarizer(embedder, vad, library)
            .cluster_threshold(0.95)?
            .speaker_bounds(options.min_speakers, options.max_speakers)?;
        SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?
    } else {
        let vad = TranscriptSpeechSpanVad { spans };
        let mut diarizer = runtime_library_diarizer(embedder, vad, library)
            .cluster_threshold(0.95)?
            .speaker_bounds(options.min_speakers, options.max_speakers)?;
        SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?
    };

    Ok(SpeakerDiarizationResponse {
        accepted: true,
        operation: "audio.speakers.diarize".to_string(),
        model_id: options.model_id.clone(),
        runtime: AudioRuntime::Heuristic,
        segments: stable_speaker_predictions_from_diarization(result.segments)?,
        speaker_embeddings: None,
        diagnostics: Vec::new(),
    })
}

#[cfg(all(feature = "diarization", feature = "onnx-diarization"))]
fn diarize_with_speaker_library_and_onnx_embeddings(
    audio: LoadedAudio,
    transcript: &TranscriptionContract,
    options: &DiarizationOptions,
    library: SpeakerLibrary,
) -> video_analysis_core::Result<SpeakerDiarizationResponse> {
    let config = options.onnx_speaker_embedding_config()?;
    let speaker_audio = SpeakerAudio::mono(&audio.samples, audio.sample_rate)?;
    let embedder = audio_analysis_speakers::OnnxSpeakerEmbedder::from_config(config)?;
    let spans = speech_spans_from_transcript_for_diarization(transcript, audio.duration_seconds())?;
    let result = if spans.is_empty() {
        let vad = EnergyVoiceActivityDetector::default();
        let mut diarizer = runtime_library_diarizer(embedder, vad, library)
            .cluster_threshold(0.95)?
            .speaker_bounds(options.min_speakers, options.max_speakers)?;
        SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?
    } else {
        let vad = TranscriptSpeechSpanVad { spans };
        let mut diarizer = runtime_library_diarizer(embedder, vad, library)
            .cluster_threshold(0.95)?
            .speaker_bounds(options.min_speakers, options.max_speakers)?;
        SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?
    };
    Ok(SpeakerDiarizationResponse {
        accepted: true,
        operation: "audio.speakers.diarize".to_string(),
        model_id: options.model_id.clone(),
        runtime: AudioRuntime::Onnx,
        segments: stable_speaker_predictions_from_diarization(result.segments)?,
        speaker_embeddings: None,
        diagnostics: Vec::new(),
    })
}

#[cfg(feature = "diarization")]
fn runtime_library_diarizer<E, V>(
    embedder: E,
    vad: V,
    library: SpeakerLibrary,
) -> WindowedSpeakerDiarizer<E, V> {
    let mut identification_options = SpeakerIdentificationOptions::default();
    identification_options.min_margin = None;
    let mut diarizer = WindowedSpeakerDiarizer::new(embedder, vad).library(library);
    diarizer.identification_options = identification_options;
    diarizer
}

#[cfg(all(feature = "diarization", not(feature = "onnx-diarization")))]
fn diarize_with_speaker_library_and_onnx_embeddings(
    _audio: LoadedAudio,
    _transcript: &TranscriptionContract,
    _options: &DiarizationOptions,
    _library: SpeakerLibrary,
) -> video_analysis_core::Result<SpeakerDiarizationResponse> {
    let mut provider = NativeSpeakerDiarizationProvider;
    provider.diarize(_audio, _transcript, _options)
}

#[cfg(feature = "diarization")]
#[derive(Debug, Clone)]
struct TranscriptSpeechSpanVad {
    spans: Vec<SpeechSpan>,
}

#[cfg(feature = "diarization")]
impl audio_analysis_speakers::VoiceActivityDetector for TranscriptSpeechSpanVad {
    fn detect_speech(
        &mut self,
        _audio: &SpeakerAudio<'_>,
    ) -> video_analysis_core::Result<Vec<SpeechSpan>> {
        Ok(self.spans.clone())
    }
}

#[cfg(feature = "diarization")]
fn validate_loaded_audio_for_diarization(audio: &LoadedAudio) -> video_analysis_core::Result<()> {
    if audio.sample_rate == 0 || audio.channels == 0 {
        return Err(video_analysis_core::DetectError::InvalidAudioFormat {
            sample_rate: audio.sample_rate,
            channels: audio.channels,
        });
    }
    if audio.samples.is_empty() {
        return Err(video_analysis_core::DetectError::InvalidArgument(
            "diarization audio samples must not be empty".to_string(),
        ));
    }
    if audio.samples.iter().any(|sample| !sample.is_finite()) {
        return Err(video_analysis_core::DetectError::InvalidArgument(
            "diarization audio samples must be finite".to_string(),
        ));
    }
    Ok(())
}

#[cfg(feature = "diarization")]
fn speech_spans_from_transcript_for_diarization(
    transcript: &TranscriptionContract,
    audio_duration_seconds: f64,
) -> video_analysis_core::Result<Vec<SpeechSpan>> {
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
        for segment in &transcript.segments {
            for word in &segment.words {
                if word.text.trim().is_empty() {
                    continue;
                }
                let Some((start, end)) = word.start_seconds.zip(word.end_seconds) else {
                    continue;
                };
                if end > audio_duration_seconds + AUDIO_DURATION_EPSILON {
                    return Err(video_analysis_core::DetectError::InvalidArgument(format!(
                        "diarization word end {:.6} exceeds audio duration {:.6}",
                        end, audio_duration_seconds
                    )));
                }
                spans.push(SpeechSpan::new(start, end, word.confidence.unwrap_or(1.0))?);
            }
        }
        return Ok(spans);
    }

    for segment in &transcript.segments {
        if segment.text.trim().is_empty() {
            continue;
        }
        let Some((start, end)) = segment.start_seconds.zip(segment.end_seconds) else {
            continue;
        };
        if end > audio_duration_seconds + AUDIO_DURATION_EPSILON {
            return Err(video_analysis_core::DetectError::InvalidArgument(format!(
                "diarization segment end {:.6} exceeds audio duration {:.6}",
                end, audio_duration_seconds
            )));
        }
        spans.push(SpeechSpan::new(start, end, 1.0)?);
    }
    Ok(spans)
}

#[cfg(feature = "diarization")]
fn stable_speaker_predictions_from_diarization(
    segments: Vec<DiarizationSegment>,
) -> video_analysis_core::Result<Vec<SpeakerSegmentPrediction>> {
    let mut unknown_labels: Vec<(String, String)> = Vec::new();
    let mut predictions = Vec::new();
    for segment in segments {
        let speaker = match segment.speaker {
            DiarizedSpeaker::Known(id) => id.as_str().to_string(),
            DiarizedSpeaker::Unknown(label) => {
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
        predictions.push(SpeakerSegmentPrediction {
            speaker,
            start_seconds: segment.start_seconds as f32,
            end_seconds: segment.end_seconds as f32,
            score: Some(segment.score),
        });
    }
    Ok(predictions)
}

fn alignment_word_timing_missing_count(transcript: &TranscriptionContract) -> usize {
    transcript
        .segments
        .iter()
        .flat_map(|segment| segment.words.iter())
        .filter(|word| word.start_seconds.zip(word.end_seconds).is_none())
        .count()
}

fn alignment_char_timing_missing_count(transcript: &TranscriptionContract) -> usize {
    transcript
        .segments
        .iter()
        .flat_map(|segment| segment.chars.iter())
        .filter(|character| character.start_seconds.zip(character.end_seconds).is_none())
        .count()
}

#[cfg(feature = "translation")]
fn run_native_with_translation(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    let mut response = run_with_phase_observer(request, config)?;
    let translation_started = Instant::now();
    let mut translator = MarianSegmentTranslator::from_config(config)?;
    translate_response_segments(&mut response, config, &mut translator)?;
    response.diagnostics.push(format!(
        "phaseTranslationSeconds={:.6}",
        translation_started.elapsed().as_secs_f64()
    ));
    Ok(response)
}

#[cfg(not(feature = "translation"))]
fn run_native_with_translation(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    let _ = (request, config);
    Err(NativeWhisperxError::InvalidConfig(
        "native post-ASR translation requires the `translation` feature".to_string(),
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TranslationRunOptions {
    source_language: Option<String>,
    target_language: String,
    max_new_tokens: usize,
}

trait SegmentTranslator {
    fn model_id(&self) -> &str;
    fn model_source(&self) -> &'static str;
    fn translate_segment(
        &mut self,
        text: &str,
        options: &TranslationRunOptions,
    ) -> Result<String, NativeWhisperxError>;
}

fn translate_response_segments(
    response: &mut TranscriptionPipelineResponse,
    config: &NativeWhisperxConfig,
    translator: &mut dyn SegmentTranslator,
) -> Result<(), NativeWhisperxError> {
    let options = translation_run_options(config)?;
    for segment in &mut response.transcript.segments {
        let source_text = segment.text.trim();
        if source_text.is_empty() {
            continue;
        }
        segment.text = translator.translate_segment(source_text, &options)?;
        segment.language = Some(options.target_language.clone());
        segment.words.clear();
        segment.chars.clear();
    }
    response.transcript.language = Some(options.target_language.clone());
    response.transcript.text = Some(response.transcript.joined_text());
    response
        .diagnostics
        .push(format!("translationModelId={}", translator.model_id()));
    response.diagnostics.push(format!(
        "translationModelSource={}",
        translator.model_source()
    ));
    if let Some(source_language) = &options.source_language {
        response
            .diagnostics
            .push(format!("translationSourceLanguage={source_language}"));
    }
    response.diagnostics.push(format!(
        "translationTargetLanguage={}",
        options.target_language
    ));
    response.diagnostics.push(format!(
        "translationMaxNewTokens={}",
        options.max_new_tokens
    ));
    Ok(())
}

fn translation_run_options(
    config: &NativeWhisperxConfig,
) -> Result<TranslationRunOptions, NativeWhisperxError> {
    let model_pair = config
        .translation
        .model_id
        .as_deref()
        .and_then(opus_mt_language_pair);
    let source_language = config
        .translation
        .source_language
        .clone()
        .or_else(|| config.asr.language.clone())
        .or_else(|| model_pair.as_ref().map(|(source, _)| (*source).to_string()));
    let target_language = config
        .translation
        .target_language
        .clone()
        .or_else(|| model_pair.as_ref().map(|(_, target)| (*target).to_string()))
        .unwrap_or_else(|| "en".to_string());

    if let (Some((expected_source, expected_target)), Some(source_language)) =
        (model_pair, source_language.as_deref())
    {
        if source_language != expected_source || target_language != expected_target {
            return Err(NativeWhisperxError::InvalidConfig(format!(
                "translation model expects {expected_source}->{expected_target}, got {source_language}->{target_language}"
            )));
        }
    }

    Ok(TranslationRunOptions {
        source_language,
        target_language,
        max_new_tokens: config.translation.max_new_tokens,
    })
}

fn opus_mt_language_pair(model_id: &str) -> Option<(&'static str, &'static str)> {
    let suffix = model_id.rsplit('/').next().unwrap_or(model_id);
    match suffix {
        "opus-mt-de-en" => Some(("de", "en")),
        _ => None,
    }
}

#[cfg(feature = "translation")]
const REQUIRED_TRANSLATION_FILES: &[&str] = &[
    "config.json",
    "generation_config.json",
    "source.spm",
    "target.spm",
    "vocab.json",
];

#[cfg(feature = "translation")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TranslationWeightFormat {
    Safetensors,
    Pytorch,
}

#[cfg(feature = "translation")]
#[derive(Debug, Clone, PartialEq, Eq)]
struct TranslationBundlePaths {
    root: PathBuf,
    config_json: PathBuf,
    generation_config_json: PathBuf,
    source_spm: PathBuf,
    target_spm: PathBuf,
    vocab_json: PathBuf,
    model_weights: PathBuf,
    weight_format: TranslationWeightFormat,
    source: &'static str,
}

#[cfg(feature = "translation")]
struct MarianSegmentTranslator {
    model_id: String,
    model_source: &'static str,
    source_tokenizer: sentencepiece_rs::SentencePieceProcessor,
    target_tokenizer: sentencepiece_rs::SentencePieceProcessor,
    config: candle_transformers::models::marian::Config,
    model: candle_transformers::models::marian::MTModel,
    device: candle_core::Device,
}

#[cfg(feature = "translation")]
impl MarianSegmentTranslator {
    fn from_config(config: &NativeWhisperxConfig) -> Result<Self, NativeWhisperxError> {
        let model_id = config
            .translation
            .model_id
            .clone()
            .unwrap_or_else(|| "Helsinki-NLP/opus-mt-de-en".to_string());
        let bundle = resolve_translation_bundle(&config.translation)?;
        let device = translation_device(config.asr.device)?;
        let marian_config: candle_transformers::models::marian::Config =
            read_json_file(&bundle.config_json)?;
        let _generation_config: serde_json::Value = read_json_file(&bundle.generation_config_json)?;
        let _vocab: serde_json::Value = read_json_file(&bundle.vocab_json)?;
        let source_tokenizer = sentencepiece_rs::SentencePieceProcessor::open(&bundle.source_spm)
            .map_err(|error| {
            NativeWhisperxError::Transcription(format!(
                "failed to load source SentencePiece model `{}`: {error}",
                bundle.source_spm.display()
            ))
        })?;
        let target_tokenizer = sentencepiece_rs::SentencePieceProcessor::open(&bundle.target_spm)
            .map_err(|error| {
            NativeWhisperxError::Transcription(format!(
                "failed to load target SentencePiece model `{}`: {error}",
                bundle.target_spm.display()
            ))
        })?;
        let vb = match bundle.weight_format {
            TranslationWeightFormat::Safetensors => unsafe {
                candle_nn::VarBuilder::from_mmaped_safetensors(
                    &[bundle.model_weights.as_path()],
                    candle_core::DType::F32,
                    &device,
                )
            },
            TranslationWeightFormat::Pytorch => candle_nn::VarBuilder::from_pth(
                &bundle.model_weights,
                candle_core::DType::F32,
                &device,
            ),
        }
        .map_err(|error| {
            NativeWhisperxError::Transcription(format!(
                "failed to load Marian weights `{}`: {error}",
                bundle.model_weights.display()
            ))
        })?;
        let model = candle_transformers::models::marian::MTModel::new(&marian_config, vb).map_err(
            |error| {
                NativeWhisperxError::Transcription(format!(
                    "failed to construct Marian translation model from `{}`: {error}",
                    bundle.root.display()
                ))
            },
        )?;
        Ok(Self {
            model_id,
            model_source: bundle.source,
            source_tokenizer,
            target_tokenizer,
            config: marian_config,
            model,
            device,
        })
    }
}

#[cfg(feature = "translation")]
impl SegmentTranslator for MarianSegmentTranslator {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn model_source(&self) -> &'static str {
        self.model_source
    }

    fn translate_segment(
        &mut self,
        text: &str,
        options: &TranslationRunOptions,
    ) -> Result<String, NativeWhisperxError> {
        let mut input_ids: Vec<u32> = self
            .source_tokenizer
            .encode_to_ids(text)
            .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))?
            .into_iter()
            .map(|id| id as u32)
            .collect();
        if input_ids.last().copied() != Some(self.config.eos_token_id) {
            input_ids.push(self.config.eos_token_id);
        }
        input_ids.truncate(self.config.max_position_embeddings);
        let input = candle_core::Tensor::new(input_ids.as_slice(), &self.device)
            .and_then(|tensor| tensor.unsqueeze(0))
            .map_err(candle_translation_error)?;
        self.model.reset_kv_cache();
        let encoder_xs = self
            .model
            .encoder()
            .forward(&input, 0)
            .map_err(candle_translation_error)?;
        let mut generated = vec![self.config.decoder_start_token_id];
        for _ in 0..options.max_new_tokens {
            self.model.reset_kv_cache();
            let decoder_input = candle_core::Tensor::new(generated.as_slice(), &self.device)
                .and_then(|tensor| tensor.unsqueeze(0))
                .map_err(candle_translation_error)?;
            let logits = self
                .model
                .decode(&decoder_input, &encoder_xs, 0)
                .map_err(candle_translation_error)?;
            let next = logits
                .i((0, generated.len() - 1))
                .and_then(|logits| logits.argmax(candle_core::D::Minus1))
                .and_then(|token| token.to_scalar::<u32>())
                .map_err(candle_translation_error)?;
            if next == self.config.eos_token_id || next == self.config.forced_eos_token_id {
                break;
            }
            generated.push(next);
        }
        let decoded_ids: Vec<usize> = generated
            .into_iter()
            .skip(1)
            .map(|id| id as usize)
            .collect();
        self.target_tokenizer
            .decode_ids(&decoded_ids)
            .map(|text| text.trim().to_string())
            .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
    }
}

#[cfg(feature = "translation")]
fn candle_translation_error(error: candle_core::Error) -> NativeWhisperxError {
    NativeWhisperxError::Transcription(format!("Marian translation failed: {error}"))
}

#[cfg(not(feature = "translation"))]
struct MarianSegmentTranslator;

#[cfg(not(feature = "translation"))]
impl MarianSegmentTranslator {
    fn from_config(_config: &NativeWhisperxConfig) -> Result<Self, NativeWhisperxError> {
        Err(NativeWhisperxError::InvalidConfig(
            "native post-ASR translation requires the `translation` feature".to_string(),
        ))
    }
}

#[cfg(feature = "translation")]
fn read_json_file<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, NativeWhisperxError> {
    let bytes = fs::read(path)?;
    serde_json::from_slice(&bytes).map_err(NativeWhisperxError::Json)
}

#[cfg(feature = "translation")]
fn translation_device(
    preference: DevicePreference,
) -> Result<candle_core::Device, NativeWhisperxError> {
    match preference {
        DevicePreference::Auto | DevicePreference::Cpu => Ok(candle_core::Device::Cpu),
        DevicePreference::Cuda => {
            #[cfg(feature = "cuda")]
            {
                candle_core::Device::new_cuda(0).map_err(|error| {
                    NativeWhisperxError::Transcription(format!(
                        "failed to initialize Candle CUDA device 0 for translation: {error}"
                    ))
                })
            }
            #[cfg(not(feature = "cuda"))]
            {
                Err(NativeWhisperxError::InvalidConfig(
                    "translation requested CUDA but this binary was built without the `cuda` feature"
                        .to_string(),
                ))
            }
        }
    }
}

#[cfg(feature = "translation")]
fn resolve_translation_bundle(
    translation: &TranslationConfig,
) -> Result<TranslationBundlePaths, NativeWhisperxError> {
    if let Some(bundle) = &translation.model_bundle {
        return resolve_translation_bundle_paths(bundle, "explicit-bundle");
    }

    let model_id = translation
        .model_id
        .as_deref()
        .unwrap_or("Helsinki-NLP/opus-mt-de-en");
    if translation.model_cache_only {
        return resolve_cached_translation_model(model_id, translation.model_dir.as_deref())
            .ok_or_else(|| missing_translation_model_error(model_id, translation));
    }

    let mut downloader = model_runtime::HuggingFaceDownloader::new().progress(false);
    if let Some(model_dir) = &translation.model_dir {
        downloader = downloader.cache_dir(model_dir.clone());
    }
    let downloaded = downloader
        .download(&translation_model_spec(model_id))
        .map_err(|error| {
            NativeWhisperxError::Transcription(format!(
                "failed to download translation model `{model_id}`: {error}"
            ))
        })?;
    let model_dir = downloaded.model_dir().ok_or_else(|| {
        NativeWhisperxError::Transcription(format!(
            "translation model `{model_id}` resolved without a local model directory"
        ))
    })?;
    resolve_translation_bundle_paths(model_dir, "hugging-face-cache")
}

#[cfg(feature = "translation")]
fn resolve_cached_translation_model(
    model_id: &str,
    model_dir: Option<&Path>,
) -> Option<TranslationBundlePaths> {
    for root in hugging_face_cache_roots(model_dir) {
        for candidate in hf_cache_candidates(&root, model_id) {
            if let Ok(paths) = resolve_translation_bundle_paths(&candidate, "hugging-face-cache") {
                return Some(paths);
            }
        }
    }
    None
}

#[cfg(feature = "translation")]
fn hugging_face_cache_roots(model_dir: Option<&Path>) -> Vec<PathBuf> {
    if let Some(model_dir) = model_dir {
        return vec![model_dir.to_path_buf()];
    }
    if let Some(home) = std::env::var_os("HF_HOME") {
        return vec![PathBuf::from(home).join("hub")];
    }
    std::env::var_os("HOME")
        .map(|home| vec![PathBuf::from(home).join(".cache/huggingface/hub")])
        .unwrap_or_default()
}

#[cfg(feature = "translation")]
fn hf_cache_candidates(root: &Path, model_id: &str) -> Vec<PathBuf> {
    let mut candidates = vec![root.to_path_buf(), root.join(model_id.replace('/', "--"))];
    let hf_repo_dir = root.join(format!("models--{}", model_id.replace('/', "--")));
    if let Ok(snapshot) = fs::read_to_string(hf_repo_dir.join("refs/main")) {
        candidates.push(hf_repo_dir.join("snapshots").join(snapshot.trim()));
    }
    if let Ok(entries) = fs::read_dir(hf_repo_dir.join("snapshots")) {
        for entry in entries.flatten() {
            candidates.push(entry.path());
        }
    }
    candidates
}

#[cfg(feature = "translation")]
fn resolve_translation_bundle_paths(
    root: &Path,
    source: &'static str,
) -> Result<TranslationBundlePaths, NativeWhisperxError> {
    let (model_weights, weight_format) = resolve_translation_weights(root)?;
    Ok(TranslationBundlePaths {
        root: root.to_path_buf(),
        config_json: resolve_translation_file(root, "config.json")?,
        generation_config_json: resolve_translation_file(root, "generation_config.json")?,
        source_spm: resolve_translation_file(root, "source.spm")?,
        target_spm: resolve_translation_file(root, "target.spm")?,
        vocab_json: resolve_translation_file(root, "vocab.json")?,
        model_weights,
        weight_format,
        source,
    })
}

#[cfg(feature = "translation")]
fn resolve_translation_file(root: &Path, file: &str) -> Result<PathBuf, NativeWhisperxError> {
    if let Ok(bundle) = model_runtime::ModelBundle::load(root) {
        if let Some(path) = bundle.file_path(file).filter(|path| path.exists()) {
            return Ok(path);
        }
    }
    let direct = root.join(file);
    if direct.exists() {
        return Ok(direct);
    }
    Err(NativeWhisperxError::Transcription(format!(
        "translation bundle `{}` is missing required file `{file}`",
        root.display()
    )))
}

#[cfg(feature = "translation")]
fn resolve_translation_weights(
    root: &Path,
) -> Result<(PathBuf, TranslationWeightFormat), NativeWhisperxError> {
    if let Ok(path) = resolve_translation_file(root, "model.safetensors") {
        return Ok((path, TranslationWeightFormat::Safetensors));
    }
    if let Some(path) = first_file_with_extension(root, "safetensors") {
        return Ok((path, TranslationWeightFormat::Safetensors));
    }
    if let Ok(path) = resolve_translation_file(root, "pytorch_model.bin") {
        return Ok((path, TranslationWeightFormat::Pytorch));
    }
    Err(NativeWhisperxError::Transcription(format!(
        "translation bundle `{}` is missing supported Marian weights: model.safetensors, *.safetensors, or pytorch_model.bin",
        root.display()
    )))
}

#[cfg(feature = "translation")]
fn first_file_with_extension(root: &Path, extension: &str) -> Option<PathBuf> {
    fs::read_dir(root)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| {
            path.extension()
                .and_then(|value| value.to_str())
                .is_some_and(|value| value == extension)
        })
}

#[cfg(feature = "translation")]
fn translation_model_spec(model_id: &str) -> model_runtime::HuggingFaceModelSpec {
    let mut spec = model_runtime::HuggingFaceModelSpec::new(
        model_id.to_string(),
        model_runtime::ModelTask::Custom("translation".to_string()),
    );
    spec.files = REQUIRED_TRANSLATION_FILES
        .iter()
        .copied()
        .map(model_runtime::ModelFileRequest::required)
        .chain(
            ["model.safetensors", "pytorch_model.bin"]
                .into_iter()
                .map(model_runtime::ModelFileRequest::optional),
        )
        .collect();
    spec
}

#[cfg(feature = "translation")]
fn missing_translation_model_error(
    model_id: &str,
    translation: &TranslationConfig,
) -> NativeWhisperxError {
    NativeWhisperxError::Transcription(format!(
        "failed to resolve translation model `{model_id}`; required files: {}; --model-dir={}; cache-only={}",
        REQUIRED_TRANSLATION_FILES.join(", "),
        translation
            .model_dir
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<default huggingface cache>".to_string()),
        translation.model_cache_only
    ))
}

pub fn run_many(
    configs: Vec<NativeWhisperxConfig>,
) -> Result<Vec<NativeWhisperxReport>, NativeWhisperxError> {
    configs.into_iter().map(run).collect()
}

pub fn build_transcription_request(
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineRequest, NativeWhisperxError> {
    if config.output.formats.is_empty() {
        return Err(NativeWhisperxError::InvalidConfig(
            "at least one output format is required".to_string(),
        ));
    }

    validate_native_support(config)?;

    Ok(TranscriptionPipelineRequest {
        source: map_input_source(&config.input),
        provider: map_provider(config),
        vad: map_vad(&config.vad),
        alignment: map_alignment(&config.alignment, config.asr.device),
        diarization: map_diarization(&config.diarization),
        output: TranscriptionOutputOptions {
            formats: config
                .output
                .formats
                .iter()
                .copied()
                .flat_map(expand_output_format)
                .map(|format| format.as_transcription_format().to_string())
                .collect(),
        },
    })
}

pub fn import_whisperx_json(bytes: &[u8]) -> Result<TranscriptionContract, NativeWhisperxError> {
    parse_whisperx_json(bytes).map_err(|error| NativeWhisperxError::Import(error.to_string()))
}

pub fn correct_speaker(
    request: SpeakerCorrectionRequest,
) -> Result<SpeakerCorrectionReport, NativeWhisperxError> {
    let current_dir = std::env::current_dir()?;
    let resolved = resolve_speaker_directory(&request.speaker_directory, &current_dir)?;
    let ranges =
        speaker_correction_ranges(&request.transcript, &request.from_speaker, &request.ranges)?;
    let audio = LoadedAudio::mono_16khz_from_source(&map_input_source(&request.audio))
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))?;
    let embedding = speaker_embedding_for_ranges(&audio, &ranges)?;
    let library =
        speaker_directory::load_speaker_library_if_present(&resolved.path)?.unwrap_or_default();
    let profile_id = request
        .speaker_id
        .clone()
        .unwrap_or_else(|| slug_speaker_id(&request.to_label));
    let mut metadata = BTreeMap::new();
    metadata.insert("status".to_string(), "confirmed".to_string());
    metadata.insert("correctedFrom".to_string(), request.from_speaker.clone());
    metadata.insert("updatedAt".to_string(), current_unix_seconds_string());
    let (library, updated_existing_profile) = speaker_directory::upsert_speaker_profile_embedding(
        &library,
        &profile_id,
        &request.to_label,
        embedding,
        metadata,
    )?;
    speaker_directory::save_speaker_library(&resolved.path, &library)?;

    let mut transcript = request.transcript;
    replace_speaker_labels(
        &mut transcript,
        &request.from_speaker,
        &request.to_label,
        &request.ranges,
    );
    let response = speaker_correction_response(transcript.clone());
    let output_files = write_outputs(&response, &request.output)?
        .into_iter()
        .map(|file| file.path)
        .collect();

    Ok(SpeakerCorrectionReport {
        transcript,
        speaker_directory_path: resolved.path,
        profile_id,
        label: request.to_label,
        corrected_from: request.from_speaker,
        enrolled_seconds: ranges
            .iter()
            .map(|range| range.end_seconds - range.start_seconds)
            .sum(),
        updated_existing_profile,
        output_files,
    })
}

pub fn write_outputs(
    response: &TranscriptionPipelineResponse,
    output: &OutputConfig,
) -> Result<Vec<OutputFile>, NativeWhisperxError> {
    write_outputs_with_options(response, output, false)
}

fn write_outputs_with_options(
    response: &TranscriptionPipelineResponse,
    output: &OutputConfig,
    return_char_alignments: bool,
) -> Result<Vec<OutputFile>, NativeWhisperxError> {
    let Some(output_dir) = &output.output_dir else {
        return Ok(Vec::new());
    };
    fs::create_dir_all(output_dir)?;
    let basename = output
        .basename
        .clone()
        .or_else(|| {
            response
                .transcript
                .source
                .as_ref()
                .and_then(source_basename)
        })
        .unwrap_or_else(|| "transcript".to_string());

    output
        .formats
        .iter()
        .copied()
        .flat_map(expand_output_format)
        .map(|format| {
            let path = output_dir.join(format!("{basename}.{}", format.extension()));
            let contents = render_output(response, format, output, return_char_alignments)?;
            fs::write(&path, contents)?;
            Ok(OutputFile { format, path })
        })
        .collect()
}

fn speaker_correction_response(transcript: TranscriptionContract) -> TranscriptionPipelineResponse {
    TranscriptionPipelineResponse {
        accepted: true,
        operation: "audio.transcription.correctSpeakers".to_string(),
        provider: "native-whisperx".to_string(),
        model_id: "speaker-correction".to_string(),
        transcript,
        vad_segments: Vec::new(),
        alignment: None,
        diarization: None,
        artifacts: Vec::new(),
        diagnostics: Vec::new(),
    }
}

fn speaker_correction_ranges(
    transcript: &TranscriptionContract,
    from_speaker: &str,
    filters: &[SpeakerCorrectionRange],
) -> Result<Vec<SpeakerCorrectionRange>, NativeWhisperxError> {
    let mut ranges = Vec::new();
    for filter in filters {
        validate_speaker_correction_range(*filter)?;
    }
    for segment in &transcript.segments {
        if segment.speaker.as_deref() != Some(from_speaker) {
            continue;
        }
        let Some((start_seconds, end_seconds)) = segment.start_seconds.zip(segment.end_seconds)
        else {
            continue;
        };
        let segment_range = SpeakerCorrectionRange {
            start_seconds,
            end_seconds,
        };
        validate_speaker_correction_range(segment_range)?;
        if filters.is_empty()
            || filters
                .iter()
                .any(|filter| filter.overlaps(start_seconds, end_seconds))
        {
            ranges.push(segment_range);
        }
    }
    if ranges.is_empty() {
        return Err(NativeWhisperxError::InvalidConfig(format!(
            "speaker correction found no timed transcript segments for speaker `{from_speaker}`"
        )));
    }
    Ok(ranges)
}

fn validate_speaker_correction_range(
    range: SpeakerCorrectionRange,
) -> Result<(), NativeWhisperxError> {
    if !range.start_seconds.is_finite()
        || !range.end_seconds.is_finite()
        || range.start_seconds < 0.0
        || range.end_seconds <= range.start_seconds
    {
        return Err(NativeWhisperxError::InvalidConfig(
            "speaker correction ranges must be finite, non-negative, and have positive duration"
                .to_string(),
        ));
    }
    Ok(())
}

fn speaker_embedding_for_ranges(
    audio: &LoadedAudio,
    ranges: &[SpeakerCorrectionRange],
) -> Result<SpeakerEmbedding, NativeWhisperxError> {
    let mut samples = Vec::new();
    for range in ranges {
        validate_speaker_correction_range(*range)?;
        let start = ((range.start_seconds * audio.sample_rate as f64).floor() as usize)
            .min(audio.samples.len());
        let end = ((range.end_seconds * audio.sample_rate as f64).ceil() as usize)
            .min(audio.samples.len());
        if end > start {
            samples.extend_from_slice(&audio.samples[start..end]);
        }
    }
    if samples.is_empty() {
        return Err(NativeWhisperxError::InvalidConfig(
            "speaker correction did not select any audio samples".to_string(),
        ));
    }
    let speaker_audio = SpeakerAudio::mono(&samples, audio.sample_rate).map_err(|error| {
        NativeWhisperxError::Transcription(format!("speaker correction audio invalid: {error}"))
    })?;
    let mut embedder = SpectralSpeakerEmbedder::default();
    embedder
        .embed_speaker(&speaker_audio)
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
}

fn replace_speaker_labels(
    transcript: &mut TranscriptionContract,
    from_speaker: &str,
    to_label: &str,
    filters: &[SpeakerCorrectionRange],
) {
    for segment in &mut transcript.segments {
        if segment.speaker.as_deref() != Some(from_speaker) {
            continue;
        }
        let selected = if filters.is_empty() {
            true
        } else {
            segment
                .start_seconds
                .zip(segment.end_seconds)
                .is_some_and(|(start, end)| {
                    filters.iter().any(|filter| filter.overlaps(start, end))
                })
        };
        if !selected {
            continue;
        }
        segment.speaker = Some(to_label.to_string());
        for word in &mut segment.words {
            if word.speaker.as_deref() == Some(from_speaker) {
                word.speaker = Some(to_label.to_string());
            }
        }
    }
}

fn slug_speaker_id(value: &str) -> String {
    let slug = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if slug.is_empty() {
        "speaker".to_string()
    } else {
        slug
    }
}

fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

fn current_unix_seconds_string() -> String {
    current_unix_seconds().to_string()
}

pub fn compare_with_whisperx(config: ParityConfig) -> Result<ParityReport, NativeWhisperxError> {
    let mut native_asr = config.native_asr;
    native_asr.provider = AsrProvider::Native;
    native_asr.language = config.language.clone();
    let external_task = native_asr.task;
    let translation = config.translation;
    let alignment = config.alignment;
    let vad = config.vad;
    let diarization = config.diarization;
    let whisperx_diarization = config
        .whisperx_diarization
        .unwrap_or_else(|| diarization.clone());

    let native_report = run(NativeWhisperxConfig {
        input: InputSource::Path {
            path: config.input.clone(),
        },
        asr: native_asr,
        translation,
        vad: vad.clone(),
        alignment: alignment.clone(),
        diarization: diarization.clone(),
        output: config.output.clone(),
    })?;

    let whisperx_report = run(NativeWhisperxConfig {
        input: InputSource::Path { path: config.input },
        asr: AsrConfig {
            provider: AsrProvider::ExternalWhisperX,
            task: external_task,
            language: config.language,
            external_whisperx: config.whisperx,
            ..AsrConfig::default()
        },
        translation: TranslationConfig::default(),
        vad,
        alignment,
        diarization: whisperx_diarization,
        output: config.output,
    })?;

    let expected = config
        .expected_json
        .map(|path| fs::read(path).map_err(NativeWhisperxError::Io))
        .transpose()?
        .map(|bytes| import_whisperx_json(&bytes))
        .transpose()?;

    let mut comparison = compare_transcripts(
        &native_report.response.transcript,
        &whisperx_report.response.transcript,
        ParityTolerance::default(),
        &config.comparison,
    );
    comparison.diagnostic_differences = compare_diagnostics(
        &native_report.response.diagnostics,
        &whisperx_report.response.diagnostics,
    );
    compare_vad_segments(
        &native_report.response.vad_segments,
        &whisperx_report.response.vad_segments,
        ParityTolerance::default(),
        &config.comparison,
        &mut comparison,
    );

    let (expected_segment_count_matches, expected_text_matches) = expected_transcript_matches(
        expected.as_ref(),
        config.expected_target,
        &native_report.response.transcript,
        &whisperx_report.response.transcript,
    );

    Ok(ParityReport {
        native_report,
        whisperx_report,
        expected,
        expected_target: config.expected_target,
        comparison,
        expected_segment_count_matches,
        expected_text_matches,
    })
}

fn expected_transcript_matches(
    expected: Option<&TranscriptionContract>,
    expected_target: ExpectedTranscriptTarget,
    native_transcript: &TranscriptionContract,
    whisperx_transcript: &TranscriptionContract,
) -> (Option<bool>, Option<bool>) {
    let Some(expected) = expected else {
        return (None, None);
    };
    let comparison_transcript = match expected_target {
        ExpectedTranscriptTarget::Native => native_transcript,
        ExpectedTranscriptTarget::Whisperx => whisperx_transcript,
    };
    (
        Some(expected.segments.len() == comparison_transcript.segments.len()),
        Some(
            normalize_space(&expected.text_or_joined())
                == normalize_space(&comparison_transcript.text_or_joined()),
        ),
    )
}

pub fn run_parity_fixture_suite(
    suite: ParityFixtureSuite,
    root: Option<&Path>,
) -> Result<ParityFixtureSuiteReport, NativeWhisperxError> {
    run_parity_fixture_suite_with_runner(suite, root, compare_with_whisperx)
}

fn run_parity_fixture_suite_with_runner<F>(
    suite: ParityFixtureSuite,
    root: Option<&Path>,
    mut runner: F,
) -> Result<ParityFixtureSuiteReport, NativeWhisperxError>
where
    F: FnMut(ParityConfig) -> Result<ParityReport, NativeWhisperxError>,
{
    let mut cases = Vec::with_capacity(suite.fixtures.len());

    for fixture in suite.fixtures {
        let fixture = resolve_fixture_case_paths(fixture, root);
        let name = fixture.name;
        let gating = fixture.gating;
        let required_diagnostics = fixture.required_diagnostics;
        let expected_outputs = fixture.expected_outputs;
        let case_result = runner(ParityConfig {
            input: fixture.input,
            expected_json: fixture.expected_json,
            expected_target: fixture.expected_target,
            comparison: fixture.comparison,
            native_asr: fixture.native_asr,
            translation: fixture.translation,
            vad: fixture.vad,
            alignment: fixture.alignment,
            diarization: fixture.diarization,
            whisperx_diarization: fixture.whisperx_diarization,
            whisperx: fixture.whisperx,
            language: fixture.language,
            output: fixture.output,
        })
        .and_then(|report| {
            let missing_required_diagnostics =
                missing_required_diagnostics(&report, &required_diagnostics);
            let expected_output_matches =
                compare_expected_outputs(&report.native_report.output_files, &expected_outputs)?;
            let passed = parity_fixture_case_passed(
                &report,
                &missing_required_diagnostics,
                &expected_output_matches,
            );
            let failure_summary = parity_fixture_failure_summary(
                Some(&report),
                &missing_required_diagnostics,
                &expected_output_matches,
                None,
            );
            Ok(ParityFixtureCaseReport {
                name: name.clone(),
                gating,
                passed,
                started_at: None,
                elapsed_seconds: None,
                timed_out: false,
                report: Some(report),
                missing_required_diagnostics,
                expected_output_matches,
                error: None,
                failure_summary,
            })
        });

        match case_result {
            Ok(case) => cases.push(case),
            Err(error) => {
                let error = error.to_string();
                cases.push(ParityFixtureCaseReport {
                    name,
                    gating,
                    passed: false,
                    started_at: None,
                    elapsed_seconds: None,
                    timed_out: false,
                    report: None,
                    missing_required_diagnostics: Vec::new(),
                    expected_output_matches: Vec::new(),
                    failure_summary: parity_fixture_failure_summary(None, &[], &[], Some(&error)),
                    error: Some(error),
                });
            }
        }
    }

    let passed = cases
        .iter()
        .filter(|case| case.gating)
        .all(|case| case.passed);
    Ok(ParityFixtureSuiteReport { passed, cases })
}

fn parity_fixture_failure_summary(
    report: Option<&ParityReport>,
    missing_required_diagnostics: &[String],
    expected_output_matches: &[ExpectedOutputComparison],
    error: Option<&str>,
) -> Vec<String> {
    let mut summary = Vec::new();
    if let Some(report) = report {
        summary.extend(report.comparison.differences.iter().cloned());
        summary.extend(report.comparison.diagnostic_differences.iter().cloned());
        if report.expected_text_matches == Some(false) {
            summary.push("expected transcript text differs".to_string());
        }
        if report.expected_segment_count_matches == Some(false) {
            summary.push("expected transcript segment count differs".to_string());
        }
    }
    summary.extend(
        missing_required_diagnostics
            .iter()
            .map(|diagnostic| format!("missing required diagnostic: {diagnostic}")),
    );
    summary.extend(
        expected_output_matches
            .iter()
            .filter(|output| !output.passed)
            .filter_map(|output| {
                output
                    .difference
                    .as_ref()
                    .map(|difference| format!("{:?} output: {difference}", output.format))
            }),
    );
    if let Some(error) = error {
        summary.push(error.to_string());
    }
    summary
}

pub fn run_parity_preflight(
    suite: ParityFixtureSuite,
    manifest: PathBuf,
    root: PathBuf,
    whisperx_command: PathBuf,
    model_dir: PathBuf,
    require_expected: bool,
    include_non_gating: bool,
) -> ParityPreflightReport {
    let source_checkout_tag = whisperx_source_checkout_tag();
    let source_checkout_ok = source_checkout_tag.as_deref() == Some("v3.8.6");
    let whisperx_version_result = check_whisperx_version(&whisperx_command);
    let model_dir_ok = model_dir.exists();

    let mut cases = Vec::with_capacity(suite.fixtures.len());
    for fixture in suite.fixtures {
        let fixture = resolve_fixture_case_paths(fixture, Some(&root));
        let enforce = fixture.gating || include_non_gating;
        let mut missing = Vec::new();
        let mut warnings = Vec::new();

        push_preflight_check(
            enforce,
            &mut missing,
            &mut warnings,
            source_checkout_ok,
            || match source_checkout_tag.as_deref() {
                Some(tag) => {
                    format!(".audio-tools/whisperx-src is not exact tag v3.8.6 (found {tag})")
                }
                None => ".audio-tools/whisperx-src is missing or not at an exact tag".to_string(),
            },
        );
        push_preflight_check(
            enforce,
            &mut missing,
            &mut warnings,
            whisperx_version_result.is_ok(),
            || {
                whisperx_version_result
                    .as_ref()
                    .err()
                    .cloned()
                    .unwrap_or_else(|| "whisperx command failed --version".to_string())
            },
        );
        push_preflight_check(enforce, &mut missing, &mut warnings, model_dir_ok, || {
            format!("model directory {} does not exist", model_dir.display())
        });
        push_preflight_check(
            enforce,
            &mut missing,
            &mut warnings,
            fixture.input.exists(),
            || format!("input {} does not exist", fixture.input.display()),
        );

        if require_expected {
            if let Some(expected_json) = &fixture.expected_json {
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    expected_json.exists(),
                    || format!("expected JSON {} does not exist", expected_json.display()),
                );
            }
            for expected_output in &fixture.expected_outputs {
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    expected_output.path.exists(),
                    || {
                        format!(
                            "expected {:?} output {} does not exist",
                            expected_output.format,
                            expected_output.path.display()
                        )
                    },
                );
            }
        }

        for env_name in preflight_required_hf_token_envs(&fixture) {
            push_preflight_check(
                enforce,
                &mut missing,
                &mut warnings,
                std::env::var_os(env_name).is_some(),
                || format!("environment variable {env_name} is not set"),
            );
        }

        if fixture.translation.enabled {
            if let Some(model_bundle) = &fixture.translation.model_bundle {
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    model_bundle.exists(),
                    || {
                        format!(
                            "translation bundle {} does not exist",
                            model_bundle.display()
                        )
                    },
                );
            }
        }

        if preflight_case_needs_onnx_runtime(&fixture) {
            push_preflight_check(
                enforce,
                &mut missing,
                &mut warnings,
                env_path_exists("ORT_DYLIB_PATH"),
                || "ORT_DYLIB_PATH is not set to an existing file".to_string(),
            );
        }

        if fixture.vad.enabled
            && matches!(fixture.vad.method, VadMethod::Silero | VadMethod::Pyannote)
        {
            if let Some(model_bundle) = &fixture.vad.model_bundle {
                let vad_label = match fixture.vad.method {
                    VadMethod::Silero => "Silero",
                    VadMethod::Pyannote => "pyannote",
                    VadMethod::Energy => "energy",
                };
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    model_bundle.exists(),
                    || {
                        format!(
                            "{vad_label} VAD bundle {} does not exist",
                            model_bundle.display()
                        )
                    },
                );
                let model_file =
                    fixture
                        .vad
                        .model_file
                        .as_deref()
                        .unwrap_or(match fixture.vad.method {
                            VadMethod::Silero => "silero_vad.onnx",
                            VadMethod::Pyannote => "segmentation.onnx",
                            VadMethod::Energy => "",
                        });
                if model_bundle.is_dir() || fixture.vad.model_file.is_some() {
                    let model_path = model_bundle.join(model_file);
                    push_preflight_check(
                        enforce,
                        &mut missing,
                        &mut warnings,
                        model_path.exists(),
                        || {
                            format!(
                                "{vad_label} VAD model {} does not exist",
                                model_path.display()
                            )
                        },
                    );
                }
            } else {
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    false,
                    || match fixture.vad.method {
                        VadMethod::Silero => "Silero VAD modelBundle is not set".to_string(),
                        VadMethod::Pyannote => "pyannote VAD modelBundle is not set".to_string(),
                        VadMethod::Energy => "energy VAD modelBundle is not set".to_string(),
                    },
                );
            }
        }

        if let Some(model_bundle) = &fixture.diarization.speaker_embedding_model_bundle {
            push_preflight_check(
                enforce,
                &mut missing,
                &mut warnings,
                model_bundle.exists(),
                || {
                    format!(
                        "speaker embedding bundle {} does not exist",
                        model_bundle.display()
                    )
                },
            );
            if let Some(model_file) = &fixture.diarization.speaker_embedding_model_file {
                let model_path = model_bundle.join(model_file);
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    model_path.exists(),
                    || {
                        format!(
                            "speaker embedding model {} does not exist",
                            model_path.display()
                        )
                    },
                );
            }
        }
        if let Some(model_bundle) = &fixture.diarization.model_bundle {
            push_preflight_check(
                enforce,
                &mut missing,
                &mut warnings,
                model_bundle.exists(),
                || {
                    format!(
                        "diarization model bundle {} does not exist",
                        model_bundle.display()
                    )
                },
            );
            for (label, file) in [
                (
                    "diarization manifest",
                    fixture
                        .diarization
                        .manifest_file
                        .as_deref()
                        .unwrap_or("pyannote_diarization_manifest.json"),
                ),
                (
                    "diarization segmentation model",
                    fixture
                        .diarization
                        .segmentation_model_file
                        .as_deref()
                        .unwrap_or("segmentation.onnx"),
                ),
                (
                    "diarization embedding model",
                    fixture
                        .diarization
                        .embedding_model_file
                        .as_deref()
                        .unwrap_or("embedding.onnx"),
                ),
                (
                    "diarization PLDA transform",
                    fixture
                        .diarization
                        .plda_transform_file
                        .as_deref()
                        .unwrap_or("plda_transform.json"),
                ),
                (
                    "diarization PLDA model",
                    fixture
                        .diarization
                        .plda_model_file
                        .as_deref()
                        .unwrap_or("plda_model.json"),
                ),
                (
                    "diarization clustering config",
                    fixture
                        .diarization
                        .clustering_config_file
                        .as_deref()
                        .unwrap_or("clustering.json"),
                ),
            ] {
                let artifact = model_bundle.join(file);
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    artifact.exists(),
                    || format!("{label} {} does not exist", artifact.display()),
                );
            }
        }

        cases.push(ParityPreflightCaseReport {
            name: fixture.name,
            gating: fixture.gating,
            passed: missing.is_empty(),
            missing,
            warnings,
        });
    }

    let passed = cases.iter().all(|case| case.passed);
    ParityPreflightReport {
        passed,
        manifest,
        root,
        whisperx_command,
        model_dir,
        source_checkout_tag,
        cases,
    }
}

fn preflight_required_hf_token_envs(fixture: &ParityFixtureCase) -> Vec<&str> {
    let mut envs = Vec::new();
    push_diarization_hf_token_env(&mut envs, &fixture.diarization);
    if let Some(diarization) = &fixture.whisperx_diarization {
        push_diarization_hf_token_env(&mut envs, diarization);
        if diarization.enabled && diarization.hf_token.is_none() {
            if let Some(env) = fixture.whisperx.hf_token_env.as_deref() {
                envs.push(env);
            }
        }
    } else if fixture.whisperx.diarize {
        if let Some(env) = fixture.whisperx.hf_token_env.as_deref() {
            envs.push(env);
        }
    }
    envs.sort_unstable();
    envs.dedup();
    envs
}

fn push_diarization_hf_token_env<'a>(envs: &mut Vec<&'a str>, diarization: &'a DiarizationConfig) {
    if diarization.enabled && diarization.hf_token.is_none() {
        if let Some(env) = diarization.hf_token_env.as_deref() {
            envs.push(env);
        }
    }
}

fn preflight_case_needs_onnx_runtime(fixture: &ParityFixtureCase) -> bool {
    let vad_uses_onnx = fixture.vad.enabled
        && matches!(fixture.vad.method, VadMethod::Silero | VadMethod::Pyannote);
    let diarization_uses_onnx = fixture.diarization.enabled
        && (fixture.diarization.model_bundle.is_some()
            || fixture.diarization.speaker_embedding_model_bundle.is_some());
    vad_uses_onnx || diarization_uses_onnx
}

fn push_preflight_check<F>(
    enforce: bool,
    missing: &mut Vec<String>,
    warnings: &mut Vec<String>,
    passed: bool,
    message: F,
) where
    F: FnOnce() -> String,
{
    if passed {
        return;
    }
    if enforce {
        missing.push(message());
    } else {
        warnings.push(message());
    }
}

fn whisperx_source_checkout_tag() -> Option<String> {
    let output = Command::new("git")
        .args([
            "-C",
            ".audio-tools/whisperx-src",
            "describe",
            "--tags",
            "--exact-match",
            "HEAD",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn check_whisperx_version(command: &Path) -> Result<(), String> {
    if !command.exists() {
        return Err(format!(
            "whisperx command {} does not exist",
            command.display()
        ));
    }
    let output = Command::new(command)
        .arg("--version")
        .output()
        .map_err(|error| format!("failed to run {} --version: {error}", command.display()))?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(format!(
            "{} --version exited with status {}{}",
            command.display(),
            output.status,
            if stderr.is_empty() {
                String::new()
            } else {
                format!(": {stderr}")
            }
        ))
    }
}

fn env_path_exists(name: &str) -> bool {
    std::env::var_os(name)
        .map(PathBuf::from)
        .is_some_and(|path| path.exists())
}

fn parity_fixture_case_passed(
    report: &ParityReport,
    missing_required_diagnostics: &[String],
    expected_output_matches: &[ExpectedOutputComparison],
) -> bool {
    report.comparison.passed
        && report.expected_text_matches != Some(false)
        && report.expected_segment_count_matches != Some(false)
        && missing_required_diagnostics.is_empty()
        && expected_output_matches
            .iter()
            .filter(|output| output.gating)
            .all(|output| output.passed)
}

fn compare_expected_outputs(
    actual_outputs: &[OutputFile],
    expected_outputs: &[ExpectedOutputFile],
) -> Result<Vec<ExpectedOutputComparison>, NativeWhisperxError> {
    expected_outputs
        .iter()
        .map(|expected| {
            let actual_path = actual_outputs
                .iter()
                .find(|actual| actual.format == expected.format)
                .map(|actual| actual.path.clone());
            let Some(actual_path_ref) = actual_path.as_ref() else {
                return Ok(ExpectedOutputComparison {
                    format: expected.format,
                    comparison: expected.comparison,
                    gating: expected.gating,
                    expected_path: expected.path.clone(),
                    actual_path,
                    passed: false,
                    difference: Some(format!("missing actual {:?} output", expected.format)),
                });
            };

            let comparison = match expected.comparison {
                OutputComparisonMode::Exact => {
                    compare_output_bytes(&expected.path, actual_path_ref)
                }
                OutputComparisonMode::JsonSemantic => {
                    compare_output_json(&expected.path, actual_path_ref)
                }
                OutputComparisonMode::SubtitleSemantic => {
                    compare_output_subtitles(&expected.path, actual_path_ref)
                }
            }?;

            Ok(ExpectedOutputComparison {
                format: expected.format,
                comparison: expected.comparison,
                gating: expected.gating,
                expected_path: expected.path.clone(),
                actual_path,
                passed: comparison.is_none(),
                difference: comparison,
            })
        })
        .collect()
}

fn compare_output_bytes(
    expected_path: &Path,
    actual_path: &Path,
) -> Result<Option<String>, NativeWhisperxError> {
    let expected = match fs::read(expected_path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Some(format!(
                "missing expected output {}",
                expected_path.display()
            )));
        }
        Err(error) => return Err(NativeWhisperxError::Io(error)),
    };
    let actual = fs::read(actual_path)?;
    if expected == actual {
        return Ok(None);
    }
    Ok(Some(first_output_difference(
        expected_path,
        actual_path,
        &expected,
        &actual,
    )))
}

fn compare_output_json(
    expected_path: &Path,
    actual_path: &Path,
) -> Result<Option<String>, NativeWhisperxError> {
    let expected = match fs::read(expected_path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Some(format!(
                "missing expected output {}",
                expected_path.display()
            )));
        }
        Err(error) => return Err(NativeWhisperxError::Io(error)),
    };
    let actual = fs::read(actual_path)?;
    let expected_json: serde_json::Value = serde_json::from_slice(&expected)?;
    let actual_json: serde_json::Value = serde_json::from_slice(&actual)?;
    if expected_json == actual_json {
        return Ok(None);
    }
    if looks_like_whisperx_transcript_json(&expected_json)
        && looks_like_whisperx_transcript_json(&actual_json)
    {
        return Ok(compare_whisperx_transcript_json(
            &expected_json,
            &actual_json,
            ParityTolerance::default(),
        ));
    }
    Ok(Some(format!(
        "JSON output differs: expected={} actual={}",
        expected_path.display(),
        actual_path.display()
    )))
}

#[derive(Debug, Clone, PartialEq)]
struct ParsedSubtitleCue {
    start: f64,
    end: f64,
    text: String,
}

fn compare_output_subtitles(
    expected_path: &Path,
    actual_path: &Path,
) -> Result<Option<String>, NativeWhisperxError> {
    let expected = match fs::read_to_string(expected_path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Some(format!(
                "missing expected output {}",
                expected_path.display()
            )));
        }
        Err(error) => return Err(NativeWhisperxError::Io(error)),
    };
    let actual = fs::read_to_string(actual_path)?;
    let expected_cues = parse_subtitle_cues(&expected);
    let actual_cues = parse_subtitle_cues(&actual);
    if expected_cues.len() != actual_cues.len() {
        return Ok(Some(format!(
            "subtitle cue count differs: expected={} actual={}",
            expected_cues.len(),
            actual_cues.len()
        )));
    }
    let tolerance = ParityTolerance::default().word_seconds;
    for (index, (expected, actual)) in expected_cues.iter().zip(actual_cues.iter()).enumerate() {
        if let Some(difference) =
            compare_subtitle_seconds(index, "start", expected.start, actual.start, tolerance)
        {
            return Ok(Some(difference));
        }
        if let Some(difference) =
            compare_subtitle_seconds(index, "end", expected.end, actual.end, tolerance)
        {
            return Ok(Some(difference));
        }
        if expected.text != actual.text {
            return Ok(Some(format!(
                "subtitle cue {index} text differs: expected {:?} actual {:?}",
                expected.text, actual.text
            )));
        }
    }
    Ok(None)
}

fn compare_subtitle_seconds(
    index: usize,
    field: &str,
    expected: f64,
    actual: f64,
    tolerance: f64,
) -> Option<String> {
    let delta = (expected - actual).abs();
    if delta <= tolerance {
        None
    } else {
        Some(format!(
            "subtitle cue {index} {field} differs: expected={expected:.3} actual={actual:.3} delta={delta:.3} tolerance={tolerance:.3}"
        ))
    }
}

fn parse_subtitle_cues(text: &str) -> Vec<ParsedSubtitleCue> {
    let normalized = text.replace("\r\n", "\n");
    normalized
        .split("\n\n")
        .filter_map(parse_subtitle_block)
        .collect()
}

fn parse_subtitle_block(block: &str) -> Option<ParsedSubtitleCue> {
    let mut lines = block
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && *line != "WEBVTT");
    let timing_line = lines.find(|line| line.contains("-->"))?;
    let (start, end) = parse_subtitle_timing_line(timing_line)?;
    let text = normalize_subtitle_text(&lines.collect::<Vec<_>>().join(" "));
    Some(ParsedSubtitleCue { start, end, text })
}

fn parse_subtitle_timing_line(line: &str) -> Option<(f64, f64)> {
    let (start, rest) = line.split_once("-->")?;
    let end = rest.split_whitespace().next()?;
    Some((
        timestamp_to_seconds(start.trim()),
        timestamp_to_seconds(end.trim()),
    ))
}

fn normalize_subtitle_text(text: &str) -> String {
    normalize_space(&text.replace("<u>", "").replace("</u>", ""))
}

fn looks_like_whisperx_transcript_json(value: &serde_json::Value) -> bool {
    value.as_object().is_some_and(|object| {
        object.contains_key("segments")
            || object.contains_key("word_segments")
            || (object.contains_key("language") && object.contains_key("text"))
    })
}

fn compare_whisperx_transcript_json(
    expected: &serde_json::Value,
    actual: &serde_json::Value,
    tolerance: ParityTolerance,
) -> Option<String> {
    let expected_object = match expected.as_object() {
        Some(object) => object,
        None => return Some("JSON transcript malformed: expected top-level object".to_string()),
    };
    let actual_object = match actual.as_object() {
        Some(object) => object,
        None => return Some("JSON transcript malformed: actual top-level object".to_string()),
    };

    if let Some(difference) = compare_json_language(expected_object, actual_object) {
        return Some(difference);
    }
    if let Some(difference) = compare_json_segments(expected_object, actual_object, tolerance) {
        return Some(difference);
    }
    if let Some(difference) = compare_json_words(expected_object, actual_object, tolerance) {
        return Some(difference);
    }
    if json_contains_chars(expected_object) || json_contains_chars(actual_object) {
        if let Some(difference) = compare_json_chars(expected_object, actual_object, tolerance) {
            return Some(difference);
        }
    }
    None
}

fn compare_json_language(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
) -> Option<String> {
    let expected_language = match optional_json_string(expected, "language", "expected language") {
        Ok(language) => language,
        Err(error) => return Some(error),
    };
    let actual_language = match optional_json_string(actual, "language", "actual language") {
        Ok(language) => language,
        Err(error) => return Some(error),
    };
    if expected_language != actual_language {
        return Some(format!(
            "JSON transcript language differs: expected={expected_language:?} actual={actual_language:?}"
        ));
    }
    None
}

fn compare_json_segments(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    tolerance: ParityTolerance,
) -> Option<String> {
    let expected_segments = match json_array_field(expected, "segments", "expected segments") {
        Ok(segments) => segments,
        Err(error) => return Some(error),
    };
    let actual_segments = match json_array_field(actual, "segments", "actual segments") {
        Ok(segments) => segments,
        Err(error) => return Some(error),
    };
    if expected_segments.len() != actual_segments.len() {
        return Some(format!(
            "JSON transcript segment count differs: expected={} actual={}",
            expected_segments.len(),
            actual_segments.len()
        ));
    }

    for (index, (expected_segment, actual_segment)) in expected_segments
        .iter()
        .zip(actual_segments.iter())
        .enumerate()
    {
        let expected_segment = match expected_segment.as_object() {
            Some(segment) => segment,
            None => {
                return Some(format!(
                    "JSON transcript segment {index} malformed: expected object"
                ));
            }
        };
        let actual_segment = match actual_segment.as_object() {
            Some(segment) => segment,
            None => {
                return Some(format!(
                    "JSON transcript segment {index} malformed: actual object"
                ));
            }
        };

        if let Some(difference) = compare_required_json_seconds(
            expected_segment,
            actual_segment,
            "start",
            &format!("segment {index} start"),
            tolerance.segment_seconds,
        ) {
            return Some(difference);
        }
        if let Some(difference) = compare_required_json_seconds(
            expected_segment,
            actual_segment,
            "end",
            &format!("segment {index} end"),
            tolerance.segment_seconds,
        ) {
            return Some(difference);
        }

        let expected_text = match required_json_string(
            expected_segment,
            "text",
            &format!("segment {index} expected text"),
        ) {
            Ok(text) => text,
            Err(error) => return Some(error),
        };
        let actual_text = match required_json_string(
            actual_segment,
            "text",
            &format!("segment {index} actual text"),
        ) {
            Ok(text) => text,
            Err(error) => return Some(error),
        };
        if normalize_space(expected_text) != normalize_space(actual_text) {
            return Some(format!(
                "JSON transcript segment {index} text differs: expected={expected_text:?} actual={actual_text:?}"
            ));
        }

        if expected_segment.contains_key("speaker") || actual_segment.contains_key("speaker") {
            let expected_speaker = match optional_json_string(
                expected_segment,
                "speaker",
                &format!("segment {index} expected speaker"),
            ) {
                Ok(speaker) => speaker,
                Err(error) => return Some(error),
            };
            let actual_speaker = match optional_json_string(
                actual_segment,
                "speaker",
                &format!("segment {index} actual speaker"),
            ) {
                Ok(speaker) => speaker,
                Err(error) => return Some(error),
            };
            if expected_speaker != actual_speaker {
                return Some(format!(
                    "JSON transcript segment {index} speaker differs: expected={expected_speaker:?} actual={actual_speaker:?}"
                ));
            }
        }
    }

    None
}

fn compare_json_words(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    tolerance: ParityTolerance,
) -> Option<String> {
    let expected_words = match flattened_json_words(expected, "expected") {
        Ok(words) => words,
        Err(error) => return Some(error),
    };
    let actual_words = match flattened_json_words(actual, "actual") {
        Ok(words) => words,
        Err(error) => return Some(error),
    };
    if expected_words.len() != actual_words.len() {
        return Some(format!(
            "JSON transcript word count differs: expected={} actual={}",
            expected_words.len(),
            actual_words.len()
        ));
    }

    for (index, (expected_word, actual_word)) in
        expected_words.iter().zip(actual_words.iter()).enumerate()
    {
        if let Some(difference) = compare_json_word(index, expected_word, actual_word, tolerance) {
            return Some(difference);
        }
    }

    None
}

fn compare_json_word(
    index: usize,
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    tolerance: ParityTolerance,
) -> Option<String> {
    let expected_text =
        match required_json_string(expected, "word", &format!("word {index} expected word")) {
            Ok(text) => text,
            Err(error) => return Some(error),
        };
    let actual_text =
        match required_json_string(actual, "word", &format!("word {index} actual word")) {
            Ok(text) => text,
            Err(error) => return Some(error),
        };
    if normalize_space(expected_text) != normalize_space(actual_text) {
        return Some(format!(
            "JSON transcript word {index} text differs: expected={expected_text:?} actual={actual_text:?}"
        ));
    }
    if let Some(difference) = compare_required_json_seconds(
        expected,
        actual,
        "start",
        &format!("word {index} start"),
        tolerance.word_seconds,
    ) {
        return Some(difference);
    }
    if let Some(difference) = compare_required_json_seconds(
        expected,
        actual,
        "end",
        &format!("word {index} end"),
        tolerance.word_seconds,
    ) {
        return Some(difference);
    }

    if expected.contains_key("score") && actual.contains_key("score") {
        let expected_score = match optional_json_number(
            expected,
            "score",
            &format!("word {index} expected score"),
        ) {
            Ok(Some(score)) => score,
            Ok(None) => return None,
            Err(error) => return Some(error),
        };
        let actual_score =
            match optional_json_number(actual, "score", &format!("word {index} actual score")) {
                Ok(Some(score)) => score,
                Ok(None) => return None,
                Err(error) => return Some(error),
            };
        if (expected_score - actual_score).abs() > 0.001 {
            return Some(format!(
                "JSON transcript word {index} score differs: expected={expected_score:.3} actual={actual_score:.3} tolerance=0.001"
            ));
        }
    }

    None
}

fn compare_json_chars(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    tolerance: ParityTolerance,
) -> Option<String> {
    let expected_chars = match flattened_json_chars(expected, "expected") {
        Ok(chars) => chars,
        Err(error) => return Some(error),
    };
    let actual_chars = match flattened_json_chars(actual, "actual") {
        Ok(chars) => chars,
        Err(error) => return Some(error),
    };
    if expected_chars.len() != actual_chars.len() {
        return Some(format!(
            "JSON transcript char count differs: expected={} actual={}",
            expected_chars.len(),
            actual_chars.len()
        ));
    }

    for (index, (expected_char, actual_char)) in
        expected_chars.iter().zip(actual_chars.iter()).enumerate()
    {
        let expected_text = match required_json_string(
            expected_char,
            "char",
            &format!("char {index} expected char"),
        ) {
            Ok(text) => text,
            Err(error) => return Some(error),
        };
        let actual_text =
            match required_json_string(actual_char, "char", &format!("char {index} actual char")) {
                Ok(text) => text,
                Err(error) => return Some(error),
            };
        if expected_text != actual_text {
            return Some(format!(
                "JSON transcript char {index} text differs: expected={expected_text:?} actual={actual_text:?}"
            ));
        }
        if let Some(difference) = compare_optional_json_seconds(
            expected_char,
            actual_char,
            "start",
            &format!("char {index} start"),
            tolerance.word_seconds,
        ) {
            return Some(difference);
        }
        if let Some(difference) = compare_optional_json_seconds(
            expected_char,
            actual_char,
            "end",
            &format!("char {index} end"),
            tolerance.word_seconds,
        ) {
            return Some(difference);
        }
    }

    None
}

fn json_contains_chars(object: &serde_json::Map<String, serde_json::Value>) -> bool {
    object
        .get("segments")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|segments| {
            segments.iter().any(|segment| {
                segment
                    .as_object()
                    .is_some_and(|segment| segment.contains_key("chars"))
            })
        })
}

fn flattened_json_words<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    side: &str,
) -> Result<Vec<&'a serde_json::Map<String, serde_json::Value>>, String> {
    if let Some(words) = object.get("word_segments") {
        return json_value_array(words, &format!("{side} word_segments"))?
            .iter()
            .enumerate()
            .map(|(index, word)| {
                word.as_object().ok_or_else(|| {
                    format!(
                        "JSON transcript {side} word_segments[{index}] malformed: object expected"
                    )
                })
            })
            .collect();
    }

    let segments = json_array_field(object, "segments", &format!("{side} segments"))?;
    let mut words = Vec::new();
    for (segment_index, segment) in segments.iter().enumerate() {
        let Some(segment) = segment.as_object() else {
            return Err(format!(
                "JSON transcript {side} segment {segment_index} malformed: object expected"
            ));
        };
        if let Some(segment_words) = segment.get("words") {
            for (word_index, word) in json_value_array(
                segment_words,
                &format!("{side} segment {segment_index} words"),
            )?
            .iter()
            .enumerate()
            {
                words.push(word.as_object().ok_or_else(|| {
                    format!("JSON transcript {side} segment {segment_index} words[{word_index}] malformed: object expected")
                })?);
            }
        }
    }
    Ok(words)
}

fn flattened_json_chars<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    side: &str,
) -> Result<Vec<&'a serde_json::Map<String, serde_json::Value>>, String> {
    let segments = json_array_field(object, "segments", &format!("{side} segments"))?;
    let mut chars = Vec::new();
    for (segment_index, segment) in segments.iter().enumerate() {
        let Some(segment) = segment.as_object() else {
            return Err(format!(
                "JSON transcript {side} segment {segment_index} malformed: object expected"
            ));
        };
        if let Some(segment_chars) = segment.get("chars") {
            for (char_index, character) in json_value_array(
                segment_chars,
                &format!("{side} segment {segment_index} chars"),
            )?
            .iter()
            .enumerate()
            {
                chars.push(character.as_object().ok_or_else(|| {
                    format!("JSON transcript {side} segment {segment_index} chars[{char_index}] malformed: object expected")
                })?);
            }
        }
    }
    Ok(chars)
}

fn json_array_field<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
) -> Result<&'a Vec<serde_json::Value>, String> {
    let value = object
        .get(key)
        .ok_or_else(|| format!("JSON transcript missing array: {label}"))?;
    json_value_array(value, label)
}

fn json_value_array<'a>(
    value: &'a serde_json::Value,
    label: &str,
) -> Result<&'a Vec<serde_json::Value>, String> {
    value
        .as_array()
        .ok_or_else(|| format!("JSON transcript malformed field: {label} must be an array"))
}

fn required_json_string<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
) -> Result<&'a str, String> {
    let value = object
        .get(key)
        .ok_or_else(|| format!("JSON transcript malformed field: {label} missing"))?;
    value
        .as_str()
        .ok_or_else(|| format!("JSON transcript malformed field: {label} must be a string"))
}

fn optional_json_string<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
) -> Result<Option<&'a str>, String> {
    match object.get(key) {
        Some(serde_json::Value::Null) | None => Ok(None),
        Some(value) => value
            .as_str()
            .map(Some)
            .ok_or_else(|| format!("JSON transcript malformed field: {label} must be a string")),
    }
}

fn optional_json_number(
    object: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
) -> Result<Option<f64>, String> {
    match object.get(key) {
        Some(serde_json::Value::Null) | None => Ok(None),
        Some(value) => value
            .as_f64()
            .map(Some)
            .ok_or_else(|| format!("JSON transcript malformed field: {label} must be a number")),
    }
}

fn compare_required_json_seconds(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
    tolerance: f64,
) -> Option<String> {
    let expected_seconds = match optional_json_number(expected, key, &format!("{label} expected")) {
        Ok(Some(seconds)) => seconds,
        Ok(None) => {
            return Some(format!(
                "JSON transcript malformed field: {label} expected missing"
            ));
        }
        Err(error) => return Some(error),
    };
    let actual_seconds = match optional_json_number(actual, key, &format!("{label} actual")) {
        Ok(Some(seconds)) => seconds,
        Ok(None) => {
            return Some(format!(
                "JSON transcript malformed field: {label} actual missing"
            ));
        }
        Err(error) => return Some(error),
    };
    if (expected_seconds - actual_seconds).abs() > tolerance {
        return Some(format!(
            "JSON transcript {label} timing differs: expected={expected_seconds:.3}s actual={actual_seconds:.3}s delta={:.3}s tolerance={tolerance:.3}s",
            (expected_seconds - actual_seconds).abs()
        ));
    }
    None
}

fn compare_optional_json_seconds(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
    tolerance: f64,
) -> Option<String> {
    let expected_seconds = match optional_json_number(expected, key, &format!("{label} expected")) {
        Ok(seconds) => seconds,
        Err(error) => return Some(error),
    };
    let actual_seconds = match optional_json_number(actual, key, &format!("{label} actual")) {
        Ok(seconds) => seconds,
        Err(error) => return Some(error),
    };
    match (expected_seconds, actual_seconds) {
        (Some(expected_seconds), Some(actual_seconds)) => {
            if (expected_seconds - actual_seconds).abs() > tolerance {
                Some(format!(
                    "JSON transcript {label} timing differs: expected={expected_seconds:.3}s actual={actual_seconds:.3}s delta={:.3}s tolerance={tolerance:.3}s",
                    (expected_seconds - actual_seconds).abs()
                ))
            } else {
                None
            }
        }
        (None, None) => None,
        (Some(_), None) | (None, Some(_)) => Some(format!(
            "JSON transcript {label} timing shape differs: expected={} actual={}",
            timing_shape(expected_seconds),
            timing_shape(actual_seconds)
        )),
    }
}

fn timing_shape(value: Option<f64>) -> &'static str {
    if value.is_some() {
        "present"
    } else {
        "null"
    }
}

fn first_output_difference(
    expected_path: &Path,
    actual_path: &Path,
    expected: &[u8],
    actual: &[u8],
) -> String {
    let expected_text = std::str::from_utf8(expected);
    let actual_text = std::str::from_utf8(actual);
    if let (Ok(expected_text), Ok(actual_text)) = (expected_text, actual_text) {
        for (index, (expected_line, actual_line)) in
            expected_text.lines().zip(actual_text.lines()).enumerate()
        {
            if expected_line != actual_line {
                return format!(
                    "line {} differs: expected {:?}, actual {:?}",
                    index + 1,
                    expected_line,
                    actual_line
                );
            }
        }
    }
    format!(
        "output bytes differ: expected={} ({} bytes) actual={} ({} bytes)",
        expected_path.display(),
        expected.len(),
        actual_path.display(),
        actual.len()
    )
}

fn missing_required_diagnostics(report: &ParityReport, required: &[String]) -> Vec<String> {
    required
        .iter()
        .filter(|required| {
            !report
                .native_report
                .response
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic == *required)
        })
        .cloned()
        .collect()
}

fn resolve_fixture_case_paths(
    mut fixture: ParityFixtureCase,
    root: Option<&Path>,
) -> ParityFixtureCase {
    fixture.input = resolve_path_with_root(fixture.input, root);
    fixture.expected_json = resolve_optional_path_with_root(fixture.expected_json, root);
    for expected_output in &mut fixture.expected_outputs {
        expected_output.path = resolve_path_with_root(expected_output.path.clone(), root);
    }
    resolve_asr_paths(&mut fixture.native_asr, root);
    resolve_translation_paths(&mut fixture.translation, root);
    resolve_vad_paths(&mut fixture.vad, root);
    resolve_alignment_paths(&mut fixture.alignment, root);
    resolve_diarization_paths(&mut fixture.diarization, root);
    if let Some(diarization) = &mut fixture.whisperx_diarization {
        resolve_diarization_paths(diarization, root);
    }
    resolve_external_whisperx_paths(&mut fixture.whisperx, root);
    resolve_output_paths(&mut fixture.output, root);
    fixture
}

fn resolve_asr_paths(asr: &mut AsrConfig, root: Option<&Path>) {
    asr.whisper_bundle = resolve_optional_path_with_root(asr.whisper_bundle.take(), root);
    asr.model_dir = resolve_optional_path_with_root(asr.model_dir.take(), root);
    resolve_external_whisperx_paths(&mut asr.external_whisperx, root);
}

fn resolve_translation_paths(translation: &mut TranslationConfig, root: Option<&Path>) {
    translation.model_bundle =
        resolve_optional_path_with_root(translation.model_bundle.take(), root);
    translation.model_dir = resolve_optional_path_with_root(translation.model_dir.take(), root);
}

fn resolve_vad_paths(vad: &mut VadConfig, root: Option<&Path>) {
    vad.model_bundle = resolve_optional_path_with_root(vad.model_bundle.take(), root);
}

fn resolve_alignment_paths(alignment: &mut AlignmentConfig, root: Option<&Path>) {
    alignment.model_bundle = resolve_optional_path_with_root(alignment.model_bundle.take(), root);
    alignment.model_dir = resolve_optional_path_with_root(alignment.model_dir.take(), root);
}

fn resolve_diarization_paths(diarization: &mut DiarizationConfig, root: Option<&Path>) {
    diarization.model_bundle =
        resolve_optional_path_with_root(diarization.model_bundle.take(), root);
    diarization.speaker_embedding_model_bundle =
        resolve_optional_path_with_root(diarization.speaker_embedding_model_bundle.take(), root);
}

fn resolve_external_whisperx_paths(whisperx: &mut ExternalWhisperxConfig, root: Option<&Path>) {
    if whisperx.command != default_whisperx_command() {
        whisperx.command = resolve_path_with_root(whisperx.command.clone(), root);
    }
    whisperx.output_dir = resolve_optional_path_with_root(whisperx.output_dir.take(), root);
}

fn resolve_output_paths(output: &mut OutputConfig, root: Option<&Path>) {
    output.output_dir = resolve_optional_path_with_root(output.output_dir.take(), root);
}

fn resolve_optional_path_with_root(path: Option<PathBuf>, root: Option<&Path>) -> Option<PathBuf> {
    path.map(|path| resolve_path_with_root(path, root))
}

fn resolve_path_with_root(path: PathBuf, root: Option<&Path>) -> PathBuf {
    match root {
        Some(root) if path.is_relative() => root.join(path),
        _ => path,
    }
}

fn map_input_source(input: &InputSource) -> TranscriptionSource {
    match input {
        InputSource::Path { path } => TranscriptionSource::Path { path: path.clone() },
        InputSource::Samples {
            samples,
            sample_rate,
            channels,
            source,
        } => TranscriptionSource::Samples {
            samples: samples.clone(),
            sample_rate: *sample_rate,
            channels: *channels,
            source: source.clone(),
        },
    }
}

fn validate_native_support(config: &NativeWhisperxConfig) -> Result<(), NativeWhisperxError> {
    if config.asr.provider != AsrProvider::Native {
        return Ok(());
    }
    if config.asr.task == TranscriptionTask::Translate && !config.translation.enabled {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --task translate requires --translation-model or --translation-bundle; use --provider external-whisperx for WhisperX built-in translation".to_string(),
        ));
    }
    if config.translation.enabled {
        validate_translation_support(config)?;
    }
    validate_native_vad_support(config)?;
    validate_native_diarization_support(&config.diarization)?;
    validate_native_decode_support(&config.asr)?;
    Ok(())
}

fn validate_native_diarization_support(
    diarization: &DiarizationConfig,
) -> Result<(), NativeWhisperxError> {
    if !diarization.enabled {
        return Ok(());
    }
    let is_pyannote = is_pyannote_diarization_model(&diarization.model_id);
    if diarization.model_bundle.is_some() && !is_pyannote {
        return Err(NativeWhisperxError::InvalidConfig(
            "native diarization modelBundle is only supported for pyannote diarization models"
                .to_string(),
        ));
    }
    if diarization.return_speaker_embeddings && !(is_pyannote && diarization.model_bundle.is_some())
    {
        return Err(NativeWhisperxError::InvalidConfig(
            "native speaker embeddings require a pyannote diarization model with an explicit modelBundle".to_string(),
        ));
    }
    if is_pyannote && diarization.model_bundle.is_none() {
        return Err(NativeWhisperxError::InvalidConfig(
            "native pyannote diarization requires an explicit modelBundle".to_string(),
        ));
    }
    #[cfg(not(feature = "pyannote-diarization"))]
    if is_pyannote {
        return Err(NativeWhisperxError::InvalidConfig(
            "native pyannote diarization requires the pyannote-diarization feature".to_string(),
        ));
    }
    Ok(())
}

fn validate_translation_support(config: &NativeWhisperxConfig) -> Result<(), NativeWhisperxError> {
    if config.asr.task != TranscriptionTask::Translate {
        return Err(NativeWhisperxError::InvalidConfig(
            "--translation-model requires --task translate".to_string(),
        ));
    }
    if config.translation.model_id.is_none() && config.translation.model_bundle.is_none() {
        return Err(NativeWhisperxError::InvalidConfig(
            "--translation-model or --translation-bundle is required for post-ASR translation"
                .to_string(),
        ));
    }
    if config.translation.max_new_tokens == 0 {
        return Err(NativeWhisperxError::InvalidConfig(
            "--translation-max-new-tokens must be greater than zero".to_string(),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct UnsupportedNativeControl {
    flag: &'static str,
    reason: &'static str,
}

fn validate_native_decode_support(asr: &AsrConfig) -> Result<(), NativeWhisperxError> {
    let mut unsupported = Vec::new();
    if asr.compute_type.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--compute_type",
            reason: "Candle Whisper does not expose a compute type or quantization selector",
        });
    }
    if asr.device_index.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--device_index",
            reason: "native device resolution currently selects the default device for the requested backend",
        });
    }

    let decode = &asr.decode;
    if !decode.temperature.is_empty() && !is_native_greedy_temperature(&decode.temperature) {
        unsupported.push(UnsupportedNativeControl {
            flag: "--temperature",
            reason: "native decode currently supports deterministic greedy temperature 0 only; sampling and fallback schedules require upstream decode APIs",
        });
    }
    if decode.best_of.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--best_of",
            reason: "best-of requires sampling candidate generation that the native backend does not expose",
        });
    }
    if decode.beam_size.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--beam_size",
            reason: "beam search is not exposed by the native Candle Whisper backend",
        });
    }
    if decode.patience.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--patience",
            reason: "beam patience only applies to beam search, which is not exposed by the native backend",
        });
    }
    if decode.length_penalty.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--length_penalty",
            reason: "length penalty only applies to beam ranking, which is not exposed by the native backend",
        });
    }
    if decode.suppress_tokens.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--suppress_tokens",
            reason: "token suppression requires tokenizer-aware logit filtering before each decode step",
        });
    }
    if decode.suppress_numerals {
        unsupported.push(UnsupportedNativeControl {
            flag: "--suppress_numerals",
            reason: "numeral suppression requires tokenizer-aware logit filtering before each decode step",
        });
    }
    if decode.initial_prompt.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--initial_prompt",
            reason: "prompt-prefilled decoder context is not exposed by the native backend",
        });
    }
    if decode.hotwords.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--hotwords",
            reason: "hotwords are a faster-whisper prompt biasing feature without a native backend equivalent",
        });
    }
    if decode.condition_on_previous_text == Some(true) {
        unsupported.push(UnsupportedNativeControl {
            flag: "--condition_on_previous_text",
            reason:
                "previous-text conditioning requires carrying decoder prompt tokens across chunks",
        });
    }
    if decode.fp16.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--fp16",
            reason: "native precision is selected by the Candle model/device path rather than WhisperX fp16",
        });
    }
    if decode.compression_ratio_threshold.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--compression_ratio_threshold",
            reason:
                "fallback thresholds require per-candidate compression scoring from the decoder",
        });
    }
    if decode.logprob_threshold.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--logprob_threshold",
            reason: "fallback thresholds require token log probability summaries from the decoder",
        });
    }
    if decode.no_speech_threshold.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--no_speech_threshold",
            reason: "no-speech thresholding requires native no-speech probability output",
        });
    }
    if decode.threads.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--threads",
            reason: "the native backend does not expose a per-request decoder thread-count control",
        });
    }

    if unsupported.is_empty() {
        return Ok(());
    }

    let details = unsupported
        .iter()
        .map(|control| format!("{} ({})", control.flag, control.reason))
        .collect::<Vec<_>>()
        .join("; ");
    Err(NativeWhisperxError::InvalidConfig(format!(
        "native provider cannot apply decode controls: {details}; use --provider external-whisperx for WhisperX decode-control parity"
    )))
}

fn is_native_greedy_temperature(temperature: &[f32]) -> bool {
    temperature
        .iter()
        .all(|value| value.is_finite() && *value == 0.0)
}

fn validate_native_vad_support(config: &NativeWhisperxConfig) -> Result<(), NativeWhisperxError> {
    match config.vad.method {
        VadMethod::Energy => Ok(()),
        VadMethod::Silero => validate_native_silero_config(&config.vad),
        VadMethod::Pyannote => validate_native_pyannote_config(&config.vad),
    }
}

fn validate_native_silero_config(vad: &VadConfig) -> Result<(), NativeWhisperxError> {
    #[cfg(not(feature = "silero-vad"))]
    {
        let _ = vad;
        Err(NativeWhisperxError::InvalidConfig(
            "native Silero VAD requires the silero-vad feature".to_string(),
        ))
    }
    #[cfg(feature = "silero-vad")]
    {
        validate_silero_threshold(vad.onset)?;
        validate_silero_chunk_size(vad.chunk_size)?;
        resolve_silero_model_path(vad).map(|_| ())
    }
}

#[cfg(feature = "silero-vad")]
fn validate_silero_threshold(threshold: Option<f32>) -> Result<(), NativeWhisperxError> {
    if let Some(threshold) = threshold {
        if !threshold.is_finite() || threshold <= 0.0 || threshold >= 1.0 {
            return Err(NativeWhisperxError::InvalidConfig(
                "native Silero VAD requires vad_onset to be finite and between 0 and 1".to_string(),
            ));
        }
    }
    Ok(())
}

#[cfg(feature = "silero-vad")]
fn validate_silero_chunk_size(chunk_size: Option<f64>) -> Result<(), NativeWhisperxError> {
    if let Some(chunk_size) = chunk_size {
        if !chunk_size.is_finite() || chunk_size <= 0.0 {
            return Err(NativeWhisperxError::InvalidConfig(
                "native Silero VAD requires chunk_size to be finite and greater than 0".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_native_pyannote_config(vad: &VadConfig) -> Result<(), NativeWhisperxError> {
    #[cfg(not(feature = "pyannote-vad"))]
    {
        let _ = vad;
        Err(NativeWhisperxError::InvalidConfig(
            "native pyannote VAD requires the pyannote-vad feature".to_string(),
        ))
    }
    #[cfg(feature = "pyannote-vad")]
    {
        validate_pyannote_threshold("vad_onset", vad.onset)?;
        validate_pyannote_threshold("vad_offset", vad.offset)?;
        validate_pyannote_chunk_size(vad.chunk_size)?;
        resolve_pyannote_vad_model_path(vad).map(|_| ())
    }
}

#[cfg(feature = "pyannote-vad")]
fn validate_pyannote_threshold(
    name: &str,
    threshold: Option<f32>,
) -> Result<(), NativeWhisperxError> {
    if let Some(threshold) = threshold {
        if !threshold.is_finite() || threshold <= 0.0 || threshold >= 1.0 {
            return Err(NativeWhisperxError::InvalidConfig(format!(
                "native pyannote VAD requires {name} to be finite and between 0 and 1"
            )));
        }
    }
    Ok(())
}

#[cfg(feature = "pyannote-vad")]
fn validate_pyannote_chunk_size(chunk_size: Option<f64>) -> Result<(), NativeWhisperxError> {
    if let Some(chunk_size) = chunk_size {
        if !chunk_size.is_finite() || chunk_size <= 0.0 {
            return Err(NativeWhisperxError::InvalidConfig(
                "native pyannote VAD requires chunk_size to be finite and greater than 0"
                    .to_string(),
            ));
        }
    }
    Ok(())
}

#[cfg(feature = "silero-vad")]
fn build_silero_vad_provider(
    vad: &VadConfig,
) -> Result<SileroVadTranscriptionProvider, NativeWhisperxError> {
    let model_path = resolve_silero_model_path(vad)?;
    let threshold = vad.onset.unwrap_or(0.5);
    let max_speech_duration_seconds = vad.chunk_size.unwrap_or(30.0);
    validate_silero_threshold(Some(threshold))?;
    validate_silero_chunk_size(Some(max_speech_duration_seconds))?;
    let options = SileroVadOptions {
        model_path: model_path.clone(),
        input_name: vad.input_name.clone(),
        output_name: vad.output_name.clone(),
        threshold,
        max_speech_duration_seconds,
        min_speech_duration_ms: 250,
        min_silence_duration_ms: 100,
        speech_pad_ms: 30,
    };
    let mut diagnostics = vec![
        format!("sileroVadThreshold={threshold}"),
        format!("sileroVadChunkSizeSeconds={max_speech_duration_seconds}"),
        format!("sileroVadModel={}", model_path.display()),
    ];
    if vad.offset.is_some() {
        diagnostics.push(
            "native Silero VAD accepts vad_offset for WhisperX CLI parity; WhisperX Silero merge does not use vad_offset".to_string(),
        );
    }
    SileroVadTranscriptionProvider::from_options(options, diagnostics)
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
}

fn run_native_with_selected_vad(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    match config.vad.method {
        VadMethod::Silero => {
            #[cfg(feature = "silero-vad")]
            {
                let mut vad_provider = build_silero_vad_provider(&config.vad)?;
                run_native_with_custom_vad(request, config, &mut vad_provider)
            }
            #[cfg(not(feature = "silero-vad"))]
            {
                let _ = (request, config);
                Err(NativeWhisperxError::InvalidConfig(
                    "native Silero VAD requires the silero-vad feature".to_string(),
                ))
            }
        }
        VadMethod::Pyannote => {
            #[cfg(feature = "pyannote-vad")]
            {
                let mut vad_provider = build_pyannote_vad_provider(&config.vad)?;
                run_native_with_custom_vad(request, config, &mut vad_provider)
            }
            #[cfg(not(feature = "pyannote-vad"))]
            {
                let _ = (request, config);
                Err(NativeWhisperxError::InvalidConfig(
                    "native pyannote VAD requires the pyannote-vad feature".to_string(),
                ))
            }
        }
        VadMethod::Energy => {
            let _ = request;
            Err(NativeWhisperxError::InvalidConfig(
                "custom native VAD was requested for energy VAD".to_string(),
            ))
        }
    }
}

#[cfg(feature = "pyannote-vad")]
fn build_pyannote_vad_provider(
    vad: &VadConfig,
) -> Result<PyannoteVadTranscriptionProvider, NativeWhisperxError> {
    let model_path = resolve_pyannote_vad_model_path(vad)?;
    let onset = vad.onset.unwrap_or(0.5);
    let offset = vad.offset.unwrap_or(0.363);
    let chunk_size = vad.chunk_size.unwrap_or(30.0);
    validate_pyannote_threshold("vad_onset", Some(onset))?;
    validate_pyannote_threshold("vad_offset", Some(offset))?;
    validate_pyannote_chunk_size(Some(chunk_size))?;
    let options = PyannoteVadOptions {
        model_path: model_path.clone(),
        input_name: vad.input_name.clone(),
        output_name: vad.output_name.clone(),
        onset,
        offset,
        chunk_size,
    };
    let diagnostics = vec![
        format!("pyannoteVadOnset={onset}"),
        format!("pyannoteVadOffset={offset}"),
        format!("pyannoteVadChunkSizeSeconds={chunk_size}"),
        format!("pyannoteVadModel={}", model_path.display()),
    ];
    PyannoteVadTranscriptionProvider::from_options(options, diagnostics)
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
}

#[cfg(any(feature = "silero-vad", feature = "pyannote-vad"))]
fn run_native_with_custom_vad(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
    vad_provider: &mut dyn TranscriptionVadProvider,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    let TranscriptionProviderSelection::CandleWhisper(options) = &request.provider else {
        return Err(NativeWhisperxError::InvalidConfig(
            "custom native VAD requires the Candle Whisper native provider".to_string(),
        ));
    };
    let mut asr_provider = CandleWhisperTranscriber::new(options.clone());

    #[cfg(feature = "diarization")]
    {
        if request.diarization.enabled {
            let mut diarizer = native_diarization_provider(config)?;
            return run_native_with_optional_alignment(
                request,
                vad_provider,
                &mut asr_provider,
                Some(&mut diarizer as &mut dyn TranscriptDiarizationProvider),
            );
        }
    }

    run_native_with_optional_alignment(request, vad_provider, &mut asr_provider, None)
}

fn run_native_with_optional_alignment(
    request: TranscriptionPipelineRequest,
    vad_provider: &mut dyn TranscriptionVadProvider,
    asr_provider: &mut CandleWhisperTranscriber,
    #[cfg_attr(not(feature = "diarization"), allow(unused_variables))] diarization_provider: Option<
        &mut dyn TranscriptDiarizationProvider,
    >,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    let mut observer = PhaseTimingObserver::default();
    if request.alignment.enabled {
        let mut aligner = CtcForcedAligner {
            options: request.alignment.clone(),
        };
        return run_transcription_pipeline_with_observer(
            request,
            vad_provider,
            asr_provider,
            Some(&mut aligner as &mut dyn ForcedAlignmentProvider),
            diarization_provider,
            &mut observer,
        )
        .map(|mut response| {
            observer.append_diagnostics(&mut response.diagnostics);
            response
        })
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()));
    }

    run_transcription_pipeline_with_observer(
        request,
        vad_provider,
        asr_provider,
        None,
        diarization_provider,
        &mut observer,
    )
    .map(|mut response| {
        observer.append_diagnostics(&mut response.diagnostics);
        response
    })
    .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
}

#[derive(Debug, Default)]
struct PhaseTimingObserver {
    decode_seconds: Option<f64>,
    decode_samples: Option<usize>,
    vad_started: Option<Instant>,
    vad_seconds: Option<f64>,
    vad_segments: Option<usize>,
    vad_windows: Option<usize>,
    asr_started: Option<Instant>,
    asr_seconds: Option<f64>,
    asr_segments: Option<usize>,
    alignment_started: Option<Instant>,
    alignment_seconds: Option<f64>,
    alignment_words: Option<usize>,
    diarization_started: Option<Instant>,
    diarization_seconds: Option<f64>,
    diarization_speakers: Option<usize>,
    diarization_segments: Option<usize>,
}

impl PhaseTimingObserver {
    fn append_diagnostics(&self, diagnostics: &mut Vec<String>) {
        push_optional_seconds(diagnostics, "phaseDecodeSeconds", self.decode_seconds);
        push_optional_usize(diagnostics, "phaseDecodeSamples", self.decode_samples);
        push_optional_seconds(diagnostics, "phaseVadSeconds", self.vad_seconds);
        push_optional_usize(diagnostics, "phaseVadSegments", self.vad_segments);
        push_optional_usize(diagnostics, "phaseVadWindows", self.vad_windows);
        push_optional_seconds(diagnostics, "phaseAsrSeconds", self.asr_seconds);
        push_optional_usize(diagnostics, "phaseAsrSegments", self.asr_segments);
        push_optional_seconds(diagnostics, "phaseAlignmentSeconds", self.alignment_seconds);
        push_optional_usize(diagnostics, "phaseAlignmentWords", self.alignment_words);
        push_optional_seconds(
            diagnostics,
            "phaseDiarizationSeconds",
            self.diarization_seconds,
        );
        push_optional_usize(
            diagnostics,
            "phaseDiarizationSpeakers",
            self.diarization_speakers,
        );
        push_optional_usize(
            diagnostics,
            "phaseDiarizationSegments",
            self.diarization_segments,
        );
    }
}

impl TranscriptionPipelineObserver for PhaseTimingObserver {
    fn observe(&mut self, event: TranscriptionPipelineEvent) {
        match event {
            TranscriptionPipelineEvent::ValidationStart => {}
            TranscriptionPipelineEvent::DecodeStart => {}
            TranscriptionPipelineEvent::DecodeEnd {
                duration_seconds,
                samples,
            } => {
                self.decode_seconds = Some(duration_seconds);
                self.decode_samples = Some(samples);
            }
            TranscriptionPipelineEvent::VadStart { .. } => {
                self.vad_started = Some(Instant::now());
            }
            TranscriptionPipelineEvent::VadEnd { segments, windows } => {
                self.vad_seconds = self
                    .vad_started
                    .map(|started| started.elapsed().as_secs_f64());
                self.vad_segments = Some(segments);
                self.vad_windows = windows;
            }
            TranscriptionPipelineEvent::AsrStart { .. } => {
                self.asr_started = Some(Instant::now());
            }
            TranscriptionPipelineEvent::AsrEnd { segments } => {
                self.asr_seconds = self
                    .asr_started
                    .map(|started| started.elapsed().as_secs_f64());
                self.asr_segments = Some(segments);
            }
            TranscriptionPipelineEvent::AlignmentStart { .. } => {
                self.alignment_started = Some(Instant::now());
            }
            TranscriptionPipelineEvent::AlignmentEnd { words } => {
                self.alignment_seconds = self
                    .alignment_started
                    .map(|started| started.elapsed().as_secs_f64());
                self.alignment_words = Some(words);
            }
            TranscriptionPipelineEvent::DiarizationStart { .. } => {
                self.diarization_started = Some(Instant::now());
            }
            TranscriptionPipelineEvent::DiarizationEnd { speakers, segments } => {
                self.diarization_seconds = self
                    .diarization_started
                    .map(|started| started.elapsed().as_secs_f64());
                self.diarization_speakers = Some(speakers);
                self.diarization_segments = Some(segments);
            }
        }
    }
}

fn push_optional_seconds(diagnostics: &mut Vec<String>, key: &str, value: Option<f64>) {
    if let Some(value) = value {
        diagnostics.push(format!("{key}={value:.6}"));
    }
}

fn push_optional_usize(diagnostics: &mut Vec<String>, key: &str, value: Option<usize>) {
    if let Some(value) = value {
        diagnostics.push(format!("{key}={value}"));
    }
}

fn map_provider(config: &NativeWhisperxConfig) -> TranscriptionProviderSelection {
    let asr = &config.asr;
    match asr.provider {
        AsrProvider::Native => {
            TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
                model_id: asr.model_id.clone(),
                task: map_transcription_task(native_asr_task(config)),
                language: native_language_hint(asr),
                device: map_device(asr.device),
                model_bundle: asr.whisper_bundle.clone(),
                model_dir: asr.model_dir.clone(),
                model_cache_only: asr.model_cache_only,
                batch_chunks: asr.batch_chunks,
                max_batch_size: asr.max_batch_size,
            })
        }
        AsrProvider::ExternalWhisperX => {
            let mut extra_args = external_whisperx_extra_args(config);
            let builtin_diarize =
                config.diarization.enabled && config.diarization.hf_token.is_none();
            let model_cache_only = asr.model_cache_only || config.alignment.model_cache_only;
            if model_cache_only {
                extra_args.extend(["--model_cache_only".to_string(), "True".to_string()]);
            }
            TranscriptionProviderSelection::ExternalWhisperX(WhisperXCommandOptions {
                command: asr.external_whisperx.command.clone(),
                model: asr.external_whisperx.model.clone(),
                task: map_transcription_task(asr.task),
                language: asr.language.clone(),
                device: match asr.device {
                    DevicePreference::Cuda => WhisperXDevice::Cuda,
                    DevicePreference::Auto | DevicePreference::Cpu => WhisperXDevice::Cpu,
                },
                compute_type: asr
                    .compute_type
                    .clone()
                    .or_else(|| asr.external_whisperx.compute_type.clone()),
                batch_size: asr.max_batch_size.or(asr.external_whisperx.batch_size),
                diarize: builtin_diarize,
                min_speakers: builtin_diarize
                    .then_some(config.diarization.min_speakers)
                    .flatten()
                    .or(asr.external_whisperx.min_speakers),
                max_speakers: builtin_diarize
                    .then_some(config.diarization.max_speakers)
                    .flatten()
                    .or(asr.external_whisperx.max_speakers),
                hf_token_env: config
                    .diarization
                    .hf_token_env
                    .clone()
                    .or_else(|| asr.external_whisperx.hf_token_env.clone()),
                output_dir: config
                    .output
                    .output_dir
                    .clone()
                    .or_else(|| asr.external_whisperx.output_dir.clone()),
                timeout_seconds: asr.external_whisperx.timeout_seconds,
                model_dir: asr
                    .model_dir
                    .clone()
                    .or_else(|| config.alignment.model_dir.clone()),
                model_cache_only: false,
                no_align: !config.alignment.enabled,
                interpolate_method: config.alignment.interpolate_method,
                return_char_alignments: config.alignment.return_char_alignments,
                align_model: asr
                    .external_whisperx
                    .align_model
                    .clone()
                    .or_else(|| Some(config.alignment.model_id.clone())),
                extra_args,
            })
        }
    }
}

fn native_asr_task(config: &NativeWhisperxConfig) -> TranscriptionTask {
    if config.asr.task == TranscriptionTask::Translate && config.translation.enabled {
        TranscriptionTask::Transcribe
    } else {
        config.asr.task
    }
}

fn map_transcription_task(task: TranscriptionTask) -> UpstreamTranscriptionTask {
    match task {
        TranscriptionTask::Transcribe => UpstreamTranscriptionTask::Transcribe,
        TranscriptionTask::Translate => UpstreamTranscriptionTask::Translate,
    }
}

fn external_whisperx_extra_args(config: &NativeWhisperxConfig) -> Vec<String> {
    let mut args = config.asr.external_whisperx.extra_args.clone();
    push_arg(
        &mut args,
        "--device_index",
        config.asr.device_index.as_deref(),
    );
    if config.vad.method != VadMethod::Energy {
        push_arg(
            &mut args,
            "--vad_method",
            Some(config.vad.method.as_whisperx_arg()),
        );
    }
    push_arg_display(&mut args, "--vad_onset", config.vad.onset);
    push_arg_display(&mut args, "--vad_offset", config.vad.offset);
    push_arg_display(&mut args, "--chunk_size", config.vad.chunk_size);

    let decode = &config.asr.decode;
    if !decode.temperature.is_empty() {
        let value = decode
            .temperature
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",");
        push_arg(&mut args, "--temperature", Some(value));
    }
    push_arg_display(&mut args, "--best_of", decode.best_of);
    push_arg_display(&mut args, "--beam_size", decode.beam_size);
    push_arg_display(&mut args, "--patience", decode.patience);
    push_arg_display(&mut args, "--length_penalty", decode.length_penalty);
    push_arg(
        &mut args,
        "--suppress_tokens",
        decode.suppress_tokens.as_deref(),
    );
    if decode.suppress_numerals {
        args.push("--suppress_numerals".to_string());
    }
    push_arg(
        &mut args,
        "--initial_prompt",
        decode.initial_prompt.as_deref(),
    );
    push_arg(&mut args, "--hotwords", decode.hotwords.as_deref());
    push_arg_bool(
        &mut args,
        "--condition_on_previous_text",
        decode.condition_on_previous_text,
    );
    push_arg_bool(&mut args, "--fp16", decode.fp16);
    push_arg_display(
        &mut args,
        "--compression_ratio_threshold",
        decode.compression_ratio_threshold,
    );
    push_arg_display(&mut args, "--logprob_threshold", decode.logprob_threshold);
    push_arg_display(
        &mut args,
        "--no_speech_threshold",
        decode.no_speech_threshold,
    );
    push_arg_display(&mut args, "--threads", decode.threads);

    if config.diarization.enabled && config.diarization.hf_token.is_some() {
        args.push("--diarize".to_string());
        push_arg_display(&mut args, "--min_speakers", config.diarization.min_speakers);
        push_arg_display(&mut args, "--max_speakers", config.diarization.max_speakers);
        push_arg(
            &mut args,
            "--hf_token",
            config.diarization.hf_token.as_deref(),
        );
    }
    if config.diarization.enabled {
        push_arg(
            &mut args,
            "--diarize_model",
            Some(config.diarization.model_id.as_str()),
        );
    }
    if config.diarization.return_speaker_embeddings {
        args.push("--speaker_embeddings".to_string());
    }
    push_arg_display(
        &mut args,
        "--max_line_width",
        config.output.subtitles.max_line_width,
    );
    push_arg_display(
        &mut args,
        "--max_line_count",
        config.output.subtitles.max_line_count,
    );
    if config.output.subtitles.highlight_words {
        args.extend(["--highlight_words".to_string(), "True".to_string()]);
    }
    push_arg(
        &mut args,
        "--segment_resolution",
        Some(match config.output.subtitles.segment_resolution {
            SegmentResolution::Sentence => "sentence",
            SegmentResolution::Chunk => "chunk",
        }),
    );
    args
}

fn push_arg<T: Into<String>>(args: &mut Vec<String>, flag: &str, value: Option<T>) {
    if let Some(value) = value {
        args.extend([flag.to_string(), value.into()]);
    }
}

fn push_arg_display<T: std::fmt::Display>(args: &mut Vec<String>, flag: &str, value: Option<T>) {
    if let Some(value) = value {
        args.extend([flag.to_string(), value.to_string()]);
    }
}

fn push_arg_bool(args: &mut Vec<String>, flag: &str, value: Option<bool>) {
    if let Some(value) = value {
        args.extend([flag.to_string(), value.to_string()]);
    }
}

fn native_language_hint(asr: &AsrConfig) -> Option<String> {
    asr.language
        .clone()
        .or_else(|| english_only_whisper_model(&asr.model_id).then(|| "en".to_string()))
}

fn english_only_whisper_model(model_id: &str) -> bool {
    let normalized = model_id
        .rsplit('/')
        .next()
        .unwrap_or(model_id)
        .strip_prefix("whisper-")
        .unwrap_or_else(|| model_id.rsplit('/').next().unwrap_or(model_id));
    matches!(normalized, "tiny.en" | "base.en" | "small.en" | "medium.en")
}

fn map_device(device: DevicePreference) -> NativeDevicePreference {
    match device {
        DevicePreference::Auto => NativeDevicePreference::Auto,
        DevicePreference::Cpu => NativeDevicePreference::Cpu,
        DevicePreference::Cuda => NativeDevicePreference::Cuda,
    }
}

fn map_vad(vad: &VadConfig) -> VadOptions {
    VadOptions {
        enabled: vad.enabled,
        rms_threshold: vad.onset.unwrap_or(vad.rms_threshold),
        frame_seconds: vad.frame_seconds,
        hop_seconds: vad.hop_seconds,
        min_speech_seconds: vad.min_speech_seconds,
        padding_seconds: vad.padding_seconds,
        merge_gap_seconds: vad.merge_gap_seconds,
        max_chunk_seconds: vad.chunk_size.unwrap_or(vad.max_chunk_seconds),
    }
}

fn map_alignment(
    alignment: &AlignmentConfig,
    native_asr_device: DevicePreference,
) -> AlignmentOptions {
    AlignmentOptions {
        enabled: alignment.enabled,
        model_id: alignment.model_id.clone(),
        device: map_device(native_asr_device),
        model_bundle: alignment.model_bundle.clone(),
        model_dir: alignment.model_dir.clone(),
        model_cache_only: alignment.model_cache_only,
        interpolate_method: alignment.interpolate_method,
        return_char_alignments: alignment.return_char_alignments,
    }
}

fn map_diarization(diarization: &DiarizationConfig) -> DiarizationOptions {
    DiarizationOptions {
        enabled: diarization.enabled,
        speaker: SpeakerDiarizationOptions {
            model_id: diarization.model_id.clone(),
            pyannote_model_bundle: diarization.model_bundle.clone(),
            pyannote_manifest_file: diarization.manifest_file.clone(),
            pyannote_segmentation_model_file: diarization.segmentation_model_file.clone(),
            pyannote_embedding_model_file: diarization.embedding_model_file.clone(),
            pyannote_plda_transform_file: diarization.plda_transform_file.clone(),
            pyannote_plda_model_file: diarization.plda_model_file.clone(),
            pyannote_clustering_config_file: diarization.clustering_config_file.clone(),
            speaker_embedding_model_bundle: diarization.speaker_embedding_model_bundle.clone(),
            speaker_embedding_model_file: diarization.speaker_embedding_model_file.clone(),
            speaker_embedding_input_name: None,
            speaker_embedding_output_name: None,
            speaker_embedding_dimension: diarization.speaker_embedding_dimension,
            speaker_embedding_sample_rate: diarization.speaker_embedding_sample_rate,
            return_speaker_embeddings: diarization.return_speaker_embeddings,
            min_speakers: diarization.min_speakers,
            max_speakers: diarization.max_speakers,
            assignment_policy: match diarization.assignment_policy {
                AssignmentPolicy::Majority => SpeakerAssignmentPolicy::Majority,
                AssignmentPolicy::NearestStart => SpeakerAssignmentPolicy::NearestStart,
                AssignmentPolicy::StrictContained => SpeakerAssignmentPolicy::StrictContained,
            },
            ..SpeakerDiarizationOptions::default()
        },
    }
}

fn render_output(
    response: &TranscriptionPipelineResponse,
    format: OutputFormat,
    output: &OutputConfig,
    return_char_alignments: bool,
) -> Result<String, NativeWhisperxError> {
    match format {
        OutputFormat::All => Err(NativeWhisperxError::InvalidConfig(
            "internal error: all output format must be expanded before rendering".to_string(),
        )),
        OutputFormat::Json if output.pretty_json => Ok(serde_json::to_string_pretty(
            &whisperx_json_value(&response.transcript, return_char_alignments),
        )?),
        OutputFormat::Json => Ok(serde_json::to_string(&whisperx_json_value(
            &response.transcript,
            return_char_alignments,
        ))?),
        OutputFormat::NativeJson if output.pretty_json => {
            Ok(serde_json::to_string_pretty(&response.transcript)?)
        }
        OutputFormat::NativeJson => Ok(serde_json::to_string(&response.transcript)?),
        OutputFormat::Srt => Ok(format_srt_with_options(
            &response.transcript,
            &output.subtitles,
        )),
        OutputFormat::Vtt => Ok(format_webvtt_with_options(
            &response.transcript,
            &output.subtitles,
        )),
        OutputFormat::Txt => Ok(format_txt(&response.transcript)),
        OutputFormat::Tsv => Ok(format_tsv(&response.transcript)),
        OutputFormat::Audacity => Ok(format_audacity_labels(&response.transcript)),
    }
}

fn whisperx_json_value(
    transcript: &TranscriptionContract,
    return_char_alignments: bool,
) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert(
        "text".to_string(),
        serde_json::Value::String(transcript.text_or_joined()),
    );
    if let Some(language) = &transcript.language {
        object.insert(
            "language".to_string(),
            serde_json::Value::String(language.clone()),
        );
    }
    if let Some(source) = &transcript.source {
        object.insert(
            "source".to_string(),
            serde_json::Value::String(source.clone()),
        );
    }

    let segments = transcript
        .segments
        .iter()
        .map(|segment| whisperx_segment_value(segment, return_char_alignments))
        .collect::<Vec<_>>();
    let words = transcript
        .segments
        .iter()
        .flat_map(|segment| segment.words.iter())
        .map(whisperx_word_value)
        .collect::<Vec<_>>();

    object.insert("segments".to_string(), serde_json::Value::Array(segments));
    object.insert("word_segments".to_string(), serde_json::Value::Array(words));
    serde_json::Value::Object(object)
}

fn whisperx_segment_value(
    segment: &text_transcripts::TranscriptSegmentContract,
    return_char_alignments: bool,
) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert("id".to_string(), serde_json::Value::from(segment.index));
    insert_seconds(&mut object, "start", segment.start_seconds);
    insert_seconds(&mut object, "end", segment.end_seconds);
    object.insert(
        "text".to_string(),
        serde_json::Value::String(segment.text.clone()),
    );
    if let Some(speaker) = &segment.speaker {
        object.insert(
            "speaker".to_string(),
            serde_json::Value::String(speaker.clone()),
        );
    }
    if let Some(confidence) = segment.confidence {
        object.insert("score".to_string(), serde_json::Value::from(confidence));
    }
    if !segment.words.is_empty() {
        object.insert(
            "words".to_string(),
            serde_json::Value::Array(segment.words.iter().map(whisperx_word_value).collect()),
        );
    }
    if return_char_alignments && !segment.chars.is_empty() {
        object.insert(
            "chars".to_string(),
            serde_json::Value::Array(segment.chars.iter().map(whisperx_char_value).collect()),
        );
    }
    serde_json::Value::Object(object)
}

fn whisperx_word_value(word: &text_transcripts::TranscriptWordContract) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert(
        "word".to_string(),
        serde_json::Value::String(word.text.clone()),
    );
    insert_seconds(&mut object, "start", word.start_seconds);
    insert_seconds(&mut object, "end", word.end_seconds);
    if let Some(confidence) = word.confidence {
        object.insert("score".to_string(), serde_json::Value::from(confidence));
    }
    if let Some(speaker) = &word.speaker {
        object.insert(
            "speaker".to_string(),
            serde_json::Value::String(speaker.clone()),
        );
    }
    serde_json::Value::Object(object)
}

fn whisperx_char_value(character: &text_transcripts::TranscriptCharContract) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert(
        "char".to_string(),
        serde_json::Value::String(character.character.clone()),
    );
    insert_seconds(&mut object, "start", character.start_seconds);
    insert_seconds(&mut object, "end", character.end_seconds);
    if let Some(confidence) = character.confidence {
        object.insert("score".to_string(), serde_json::Value::from(confidence));
    }
    serde_json::Value::Object(object)
}

fn insert_seconds(
    object: &mut serde_json::Map<String, serde_json::Value>,
    key: &str,
    value: Option<f64>,
) {
    if let Some(value) = value {
        object.insert(key.to_string(), serde_json::Value::from(value));
    }
}

fn expand_output_format(format: OutputFormat) -> Vec<OutputFormat> {
    match format {
        OutputFormat::All => vec![
            OutputFormat::Txt,
            OutputFormat::Vtt,
            OutputFormat::Srt,
            OutputFormat::Tsv,
            OutputFormat::Audacity,
            OutputFormat::Json,
        ],
        other => vec![other],
    }
}

fn format_txt(transcript: &TranscriptionContract) -> String {
    let text = transcript
        .segments
        .iter()
        .map(|segment| match &segment.speaker {
            Some(speaker) => format!("[{speaker}]: {}", segment.text.trim()),
            None => segment.text.trim().to_string(),
        })
        .collect::<Vec<_>>()
        .join("\n");
    if text.is_empty() {
        text
    } else {
        format!("{text}\n")
    }
}

fn format_tsv(transcript: &TranscriptionContract) -> String {
    let mut output = String::from("start\tend\ttext\n");
    for segment in &transcript.segments {
        let start = seconds_to_millis(segment.start_seconds);
        let end = seconds_to_millis(segment.end_seconds);
        output.push_str(&format!(
            "{start}\t{end}\t{}\n",
            segment.text.trim().replace('\t', " ")
        ));
    }
    output
}

fn format_audacity_labels(transcript: &TranscriptionContract) -> String {
    let mut output = String::new();
    for segment in &transcript.segments {
        let start = segment.start_seconds.unwrap_or(0.0);
        let end = segment.end_seconds.unwrap_or(start).max(start);
        let text = match &segment.speaker {
            Some(speaker) => format!("[[{speaker}]]{}", segment.text.trim().replace('\t', " ")),
            None => segment.text.trim().replace('\t', " "),
        };
        output.push_str(&format!("{start}\t{end}\t{text}\n"));
    }
    output
}

fn seconds_to_millis(seconds: Option<f64>) -> u64 {
    seconds.unwrap_or(0.0).max(0.0).mul_add(1000.0, 0.0).round() as u64
}

fn format_srt_with_options(
    transcript: &TranscriptionContract,
    subtitles: &SubtitleConfig,
) -> String {
    let mut output = String::new();
    for (index, cue) in subtitle_cues(transcript, subtitles).into_iter().enumerate() {
        output.push_str(&(index + 1).to_string());
        output.push('\n');
        output.push_str(&format_subtitle_timestamp(cue.start, true, ','));
        output.push_str(" --> ");
        output.push_str(&format_subtitle_timestamp(cue.end, true, ','));
        output.push('\n');
        output.push_str(&cue.text);
        output.push_str("\n\n");
    }
    output
}

fn format_webvtt_with_options(
    transcript: &TranscriptionContract,
    subtitles: &SubtitleConfig,
) -> String {
    let mut output = String::from("WEBVTT\n\n");
    for cue in subtitle_cues(transcript, subtitles) {
        output.push_str(&format_subtitle_timestamp(cue.start, false, '.'));
        output.push_str(" --> ");
        output.push_str(&format_subtitle_timestamp(cue.end, false, '.'));
        output.push('\n');
        output.push_str(&cue.text);
        output.push_str("\n\n");
    }
    output
}

#[derive(Debug, Clone)]
struct SubtitleCue {
    start: f64,
    end: f64,
    text: String,
}

#[derive(Debug, Clone)]
struct SubtitleTiming {
    word: String,
    start: Option<f64>,
    end: Option<f64>,
}

fn subtitle_cues(
    transcript: &TranscriptionContract,
    subtitles: &SubtitleConfig,
) -> Vec<SubtitleCue> {
    let Some(first_segment) = transcript.segments.first() else {
        return Vec::new();
    };
    if !first_segment.words.is_empty() {
        return subtitle_word_cues(transcript, subtitles);
    }
    transcript
        .segments
        .iter()
        .map(|segment| {
            let start = segment.start_seconds.unwrap_or(0.0);
            let end = segment.end_seconds.unwrap_or(start).max(start);
            let mut text = segment.text.trim().replace("-->", "->");
            if let Some(speaker) = &segment.speaker {
                text = format!("[{speaker}]: {text}");
            }
            SubtitleCue { start, end, text }
        })
        .collect()
}

fn subtitle_word_cues(
    transcript: &TranscriptionContract,
    subtitles: &SubtitleConfig,
) -> Vec<SubtitleCue> {
    let mut cues = Vec::new();
    let raw_max_line_width = subtitles.max_line_width;
    let max_line_count = subtitles.max_line_count;
    let max_line_width = raw_max_line_width.unwrap_or(1000);
    let preserve_segments = max_line_count.is_none() || raw_max_line_width.is_none();

    let mut line_len = 0usize;
    let mut line_count = 1usize;
    let mut subtitle = Vec::<SubtitleTiming>::new();
    let mut times = Vec::<(f64, f64, Option<String>)>::new();
    let mut last = transcript
        .segments
        .first()
        .and_then(|segment| segment.start_seconds)
        .unwrap_or(0.0);

    for segment in &transcript.segments {
        for (word_index, original_timing) in segment.words.iter().enumerate() {
            let mut timing = SubtitleTiming {
                word: original_timing.text.clone(),
                start: original_timing.start_seconds,
                end: original_timing.end_seconds,
            };
            let long_pause = if preserve_segments {
                false
            } else {
                timing.start.is_some_and(|start| start - last > 3.0)
            };
            let has_room = line_len + timing.word.chars().count() <= max_line_width;
            let seg_break = word_index == 0 && !subtitle.is_empty() && preserve_segments;
            if line_len > 0 && has_room && !long_pause && !seg_break {
                line_len += timing.word.chars().count();
            } else {
                timing.word = timing.word.trim().to_string();
                if (!subtitle.is_empty()
                    && max_line_count.is_some()
                    && (long_pause || line_count >= max_line_count.unwrap_or(0)))
                    || seg_break
                {
                    push_subtitle_cues(transcript, subtitles, &subtitle, &times, &mut cues);
                    subtitle.clear();
                    times.clear();
                    line_count = 1;
                } else if line_len > 0 {
                    line_count += 1;
                    timing.word = format!("\n{}", timing.word);
                }
                line_len = timing.word.trim().chars().count();
            }
            subtitle.push(timing);
            times.push((
                segment.start_seconds.unwrap_or(0.0),
                segment
                    .end_seconds
                    .unwrap_or_else(|| segment.start_seconds.unwrap_or(0.0)),
                segment.speaker.clone(),
            ));
            if let Some(start) = original_timing.start_seconds {
                last = start;
            }
        }
    }
    if !subtitle.is_empty() {
        push_subtitle_cues(transcript, subtitles, &subtitle, &times, &mut cues);
    }
    cues
}

fn push_subtitle_cues(
    transcript: &TranscriptionContract,
    subtitles: &SubtitleConfig,
    subtitle: &[SubtitleTiming],
    times: &[(f64, f64, Option<String>)],
    cues: &mut Vec<SubtitleCue>,
) {
    let Some((fallback_start, fallback_end, speaker)) = times.first() else {
        return;
    };
    let word_starts = subtitle.iter().filter_map(|word| word.start);
    let word_ends = subtitle.iter().filter_map(|word| word.end);
    let start = word_starts.reduce(f64::min).unwrap_or(*fallback_start);
    let end = word_ends.reduce(f64::max).unwrap_or(*fallback_end);
    let prefix = speaker
        .as_ref()
        .map(|speaker| format!("[{speaker}]: "))
        .unwrap_or_default();
    let subtitle_text = subtitle_text_for_language(transcript, subtitle);
    let has_timing = subtitle.iter().any(|word| word.start.is_some());

    if subtitles.highlight_words && has_timing {
        let mut last = format_subtitle_timestamp(start, true, ',');
        let all_words = subtitle
            .iter()
            .map(|timing| timing.word.clone())
            .collect::<Vec<_>>();
        for (index, timing) in subtitle.iter().enumerate() {
            let (Some(word_start), Some(word_end)) = (timing.start, timing.end) else {
                continue;
            };
            let start_text = format_subtitle_timestamp(word_start, true, ',');
            let end_text = format_subtitle_timestamp(word_end, true, ',');
            if last != start_text {
                cues.push(SubtitleCue {
                    start: timestamp_to_seconds(&last),
                    end: word_start,
                    text: format!("{prefix}{subtitle_text}"),
                });
            }
            cues.push(SubtitleCue {
                start: word_start,
                end: word_end,
                text: format!(
                    "{prefix}{}",
                    all_words
                        .iter()
                        .enumerate()
                        .map(|(word_index, word)| {
                            if word_index == index {
                                underline_word_preserving_leading_space(word)
                            } else {
                                word.clone()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
            });
            last = end_text;
        }
    } else {
        cues.push(SubtitleCue {
            start,
            end,
            text: format!("{prefix}{subtitle_text}"),
        });
    }
}

fn subtitle_text_for_language(
    transcript: &TranscriptionContract,
    subtitle: &[SubtitleTiming],
) -> String {
    let words = subtitle
        .iter()
        .map(|timing| timing.word.clone())
        .collect::<Vec<_>>();
    if transcript
        .language
        .as_deref()
        .is_some_and(|language| matches!(language, "ja" | "zh"))
    {
        words.join("")
    } else {
        words.join(" ")
    }
}

fn underline_word_preserving_leading_space(word: &str) -> String {
    let leading_bytes = word
        .char_indices()
        .find(|(_, character)| !character.is_whitespace())
        .map(|(index, _)| index)
        .unwrap_or(word.len());
    let (leading, rest) = word.split_at(leading_bytes);
    format!("{leading}<u>{rest}</u>")
}

fn format_subtitle_timestamp(
    seconds: f64,
    always_include_hours: bool,
    decimal_marker: char,
) -> String {
    let total_millis = (seconds.max(0.0) * 1_000.0).round() as u64;
    let millis = total_millis % 1_000;
    let total_seconds = total_millis / 1_000;
    let secs = total_seconds % 60;
    let total_minutes = total_seconds / 60;
    let minutes = total_minutes % 60;
    let hours = total_minutes / 60;
    if always_include_hours || hours > 0 {
        format!("{hours:02}:{minutes:02}:{secs:02}{decimal_marker}{millis:03}")
    } else {
        format!("{minutes:02}:{secs:02}{decimal_marker}{millis:03}")
    }
}

fn timestamp_to_seconds(timestamp: &str) -> f64 {
    let normalized = timestamp.replace(',', ".");
    let parts = normalized.split(':').collect::<Vec<_>>();
    match parts.as_slice() {
        [hours, minutes, seconds] => {
            hours.parse::<f64>().unwrap_or(0.0) * 3600.0
                + minutes.parse::<f64>().unwrap_or(0.0) * 60.0
                + seconds.parse::<f64>().unwrap_or(0.0)
        }
        [minutes, seconds] => {
            minutes.parse::<f64>().unwrap_or(0.0) * 60.0 + seconds.parse::<f64>().unwrap_or(0.0)
        }
        _ => 0.0,
    }
}

fn source_basename(source: &String) -> Option<String> {
    Path::new(source)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.to_string())
        .filter(|stem| !stem.trim().is_empty())
}

fn normalize_space(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn compare_transcripts(
    native: &TranscriptionContract,
    whisperx: &TranscriptionContract,
    tolerance: ParityTolerance,
    config: &ParityComparisonConfig,
) -> ParityComparison {
    let mut differences = Vec::new();
    let text_matches =
        normalize_space(&native.text_or_joined()) == normalize_space(&whisperx.text_or_joined());
    if !text_matches {
        push_comparison_difference(
            &mut differences,
            config.text,
            "normalized transcript text differs".to_string(),
        );
    }

    let language_matches = native.language == whisperx.language;
    if !language_matches {
        push_comparison_difference(
            &mut differences,
            config.language,
            format!(
                "language differs: native={:?} reference={:?}",
                native.language, whisperx.language
            ),
        );
    }

    let native_segment_text = segment_text_signature(native);
    let reference_segment_text = segment_text_signature(whisperx);
    let segment_text_matches = native_segment_text == reference_segment_text;
    if !segment_text_matches {
        push_comparison_difference(
            &mut differences,
            config.segment_text,
            format!(
                "segment text sequence differs: native={native_segment_text:?} reference={reference_segment_text:?}"
            ),
        );
    }

    let native_word_text = word_text_signature(native);
    let reference_word_text = word_text_signature(whisperx);
    let word_text_matches = native_word_text == reference_word_text;
    if !word_text_matches {
        push_comparison_difference(
            &mut differences,
            config.word_text,
            format!(
                "word text sequence differs: native={native_word_text:?} reference={reference_word_text:?}"
            ),
        );
    }

    let native_char_count = char_count(native);
    let whisperx_char_count = char_count(whisperx);
    let char_count_matches = native_char_count == whisperx_char_count;
    if !char_count_matches {
        push_comparison_difference(
            &mut differences,
            config.char_count,
            format!(
                "char alignment count differs: native={native_char_count} reference={whisperx_char_count}"
            ),
        );
    }

    let native_char_content = char_content_signature(native);
    let whisperx_char_content = char_content_signature(whisperx);
    let char_content_matches = native_char_content == whisperx_char_content;
    if !char_content_matches {
        push_comparison_difference(
            &mut differences,
            config.char_content,
            char_content_difference(&native_char_content, &whisperx_char_content),
        );
    }

    let segment_count_matches = native.segments.len() == whisperx.segments.len();
    if !segment_count_matches {
        push_comparison_difference(
            &mut differences,
            config.segment_count,
            format!(
                "segment count differs: native={} reference={}",
                native.segments.len(),
                whisperx.segments.len()
            ),
        );
    }

    let native_word_count = word_count(native);
    let whisperx_word_count = word_count(whisperx);
    let word_count_matches = native_word_count == whisperx_word_count;
    if !word_count_matches {
        push_comparison_difference(
            &mut differences,
            config.word_count,
            format!(
                "word count differs: native={native_word_count} reference={whisperx_word_count}"
            ),
        );
    }

    let segment_timing_matches = timings_match(
        native.segments.iter().map(|segment| {
            (
                segment.start_seconds,
                segment.end_seconds,
                format!("segment {}", segment.index),
            )
        }),
        whisperx.segments.iter().map(|segment| {
            (
                segment.start_seconds,
                segment.end_seconds,
                format!("segment {}", segment.index),
            )
        }),
        tolerance.segment_seconds,
        "segment",
        config.segment_timing,
        &mut differences,
    );

    let native_words = native
        .segments
        .iter()
        .flat_map(|segment| segment.words.iter())
        .collect::<Vec<_>>();
    let whisperx_words = whisperx
        .segments
        .iter()
        .flat_map(|segment| segment.words.iter())
        .collect::<Vec<_>>();
    let word_timing_matches = timings_match(
        native_words.iter().enumerate().map(|(index, word)| {
            (
                word.start_seconds,
                word.end_seconds,
                format!("word {index}"),
            )
        }),
        whisperx_words.iter().enumerate().map(|(index, word)| {
            (
                word.start_seconds,
                word.end_seconds,
                format!("word {index}"),
            )
        }),
        tolerance.word_seconds,
        "word",
        config.word_timing,
        &mut differences,
    );

    let speaker_turns_match = speaker_turn_signature(native) == speaker_turn_signature(whisperx);
    if !speaker_turns_match {
        push_comparison_difference(
            &mut differences,
            config.speaker_turns,
            "speaker turn structure differs".to_string(),
        );
    }

    let passed = comparison_field_passed(config.text, text_matches)
        && comparison_field_passed(config.language, language_matches)
        && comparison_field_passed(config.segment_text, segment_text_matches)
        && comparison_field_passed(config.word_text, word_text_matches)
        && comparison_field_passed(config.char_count, char_count_matches)
        && comparison_field_passed(config.char_content, char_content_matches)
        && comparison_field_passed(config.segment_count, segment_count_matches)
        && comparison_field_passed(config.word_count, word_count_matches)
        && comparison_field_passed(config.segment_timing, segment_timing_matches)
        && comparison_field_passed(config.word_timing, word_timing_matches)
        && comparison_field_passed(config.speaker_turns, speaker_turns_match);

    ParityComparison {
        text_matches,
        language_matches: Some(language_matches),
        segment_text_matches: Some(segment_text_matches),
        word_text_matches: Some(word_text_matches),
        char_count_matches: Some(char_count_matches),
        char_content_matches: Some(char_content_matches),
        segment_count_matches,
        word_count_matches,
        segment_timing_matches,
        word_timing_matches,
        speaker_turns_match,
        vad_segment_count_matches: None,
        vad_segment_timing_matches: None,
        confidence_compared: true,
        passed,
        tolerance,
        differences,
        diagnostic_differences: Vec::new(),
    }
}

fn comparison_field_passed(enabled: bool, matches: bool) -> bool {
    !enabled || matches
}

fn push_comparison_difference(differences: &mut Vec<String>, enabled: bool, difference: String) {
    if enabled {
        differences.push(difference);
    } else {
        differences.push(format!("report-only: {difference}"));
    }
}

fn word_count(transcript: &TranscriptionContract) -> usize {
    transcript
        .segments
        .iter()
        .map(|segment| segment.words.len())
        .sum()
}

fn char_count(transcript: &TranscriptionContract) -> usize {
    transcript
        .segments
        .iter()
        .map(|segment| segment.chars.len())
        .sum()
}

fn char_content_signature(transcript: &TranscriptionContract) -> Vec<String> {
    transcript
        .segments
        .iter()
        .flat_map(|segment| segment.chars.iter())
        .map(|character| character.character.clone())
        .collect()
}

fn char_content_difference(native: &[String], whisperx: &[String]) -> String {
    let mismatch = native
        .iter()
        .zip(whisperx.iter())
        .enumerate()
        .find(|(_, (native, whisperx))| native != whisperx);
    if let Some((index, (native, whisperx))) = mismatch {
        return format!(
            "char alignment content differs at char {index}: native={native:?} reference={whisperx:?}"
        );
    }
    format!(
        "char alignment content differs: native_count={} reference_count={}",
        native.len(),
        whisperx.len()
    )
}

fn segment_text_signature(transcript: &TranscriptionContract) -> Vec<String> {
    transcript
        .segments
        .iter()
        .map(|segment| normalize_space(&segment.text))
        .collect()
}

fn word_text_signature(transcript: &TranscriptionContract) -> Vec<String> {
    transcript
        .segments
        .iter()
        .flat_map(|segment| segment.words.iter())
        .map(|word| normalize_space(&word.text))
        .collect()
}

fn compare_diagnostics(native: &[String], whisperx: &[String]) -> Vec<String> {
    let native_set = native.iter().collect::<std::collections::BTreeSet<_>>();
    let whisperx_set = whisperx.iter().collect::<std::collections::BTreeSet<_>>();
    let mut differences = Vec::new();

    for diagnostic in native_set.difference(&whisperx_set) {
        differences.push(format!("native diagnostic only: {diagnostic}"));
    }
    for diagnostic in whisperx_set.difference(&native_set) {
        differences.push(format!("whisperx diagnostic only: {diagnostic}"));
    }

    differences
}

fn compare_vad_segments(
    native: &[SpeechActivitySegment],
    whisperx: &[SpeechActivitySegment],
    tolerance: ParityTolerance,
    config: &ParityComparisonConfig,
    comparison: &mut ParityComparison,
) {
    if !config.vad_segments {
        comparison.vad_segment_count_matches = None;
        comparison.vad_segment_timing_matches = None;
        return;
    }

    let count_matches = native.len() == whisperx.len();
    if !count_matches {
        push_comparison_difference(
            &mut comparison.differences,
            config.vad_segment_count,
            format!(
                "VAD segment count differs: native={} reference={}",
                native.len(),
                whisperx.len()
            ),
        );
    }

    let timing_matches = timings_match(
        native.iter().enumerate().map(|(index, segment)| {
            (
                Some(segment.start_seconds),
                Some(segment.end_seconds),
                format!("VAD segment {index}"),
            )
        }),
        whisperx.iter().enumerate().map(|(index, segment)| {
            (
                Some(segment.start_seconds),
                Some(segment.end_seconds),
                format!("VAD segment {index}"),
            )
        }),
        tolerance.segment_seconds,
        "VAD segment",
        config.vad_segment_timing,
        &mut comparison.differences,
    );

    comparison.vad_segment_count_matches = Some(count_matches);
    comparison.vad_segment_timing_matches = Some(timing_matches);
    comparison.passed = comparison.passed
        && comparison_field_passed(config.vad_segment_count, count_matches)
        && comparison_field_passed(config.vad_segment_timing, timing_matches);
}

fn timings_match<N, W>(
    native: N,
    whisperx: W,
    tolerance: f64,
    label: &str,
    enabled: bool,
    differences: &mut Vec<String>,
) -> bool
where
    N: Iterator<Item = (Option<f64>, Option<f64>, String)>,
    W: Iterator<Item = (Option<f64>, Option<f64>, String)>,
{
    let native = native.collect::<Vec<_>>();
    let whisperx = whisperx.collect::<Vec<_>>();
    if native.len() != whisperx.len() {
        return false;
    }

    let mut matches = true;
    for ((native_start, native_end, name), (whisperx_start, whisperx_end, _)) in
        native.into_iter().zip(whisperx)
    {
        if !optional_seconds_match(native_start, whisperx_start, tolerance)
            || !optional_seconds_match(native_end, whisperx_end, tolerance)
        {
            push_comparison_difference(
                differences,
                enabled,
                format_timing_difference(
                    label,
                    &name,
                    native_start,
                    native_end,
                    whisperx_start,
                    whisperx_end,
                    tolerance,
                ),
            );
            matches = false;
        }
    }
    matches
}

fn format_timing_difference(
    label: &str,
    name: &str,
    native_start: Option<f64>,
    native_end: Option<f64>,
    whisperx_start: Option<f64>,
    whisperx_end: Option<f64>,
    tolerance: f64,
) -> String {
    format!(
        "{label} timing differs at {name}: native start={} native end={}, reference start={} reference end={}, start_delta={} end_delta={} tolerance={:.3}s",
        format_optional_seconds(native_start),
        format_optional_seconds(native_end),
        format_optional_seconds(whisperx_start),
        format_optional_seconds(whisperx_end),
        format_optional_delta(native_start, whisperx_start),
        format_optional_delta(native_end, whisperx_end),
        tolerance,
    )
}

fn format_optional_seconds(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.3}s"))
        .unwrap_or_else(|| "missing".to_string())
}

fn format_optional_delta(left: Option<f64>, right: Option<f64>) -> String {
    match (left, right) {
        (Some(left), Some(right)) => format!("{:.3}s", (left - right).abs()),
        _ => "missing".to_string(),
    }
}

fn optional_seconds_match(left: Option<f64>, right: Option<f64>, tolerance: f64) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => (left - right).abs() <= tolerance,
        (None, None) => true,
        _ => false,
    }
}

fn speaker_turn_signature(transcript: &TranscriptionContract) -> Vec<Option<usize>> {
    let mut speakers = Vec::<String>::new();
    transcript
        .segments
        .iter()
        .map(|segment| {
            segment.speaker.as_ref().map(|speaker| {
                speakers
                    .iter()
                    .position(|known| known == speaker)
                    .unwrap_or_else(|| {
                        speakers.push(speaker.clone());
                        speakers.len() - 1
                    })
            })
        })
        .collect()
}

fn default_whisper_model_id() -> String {
    "small".to_string()
}

fn default_external_whisperx_model() -> String {
    "small".to_string()
}

fn default_whisperx_command() -> PathBuf {
    PathBuf::from("whisperx")
}

fn default_alignment_model_id() -> String {
    "facebook/wav2vec2-base-960h".to_string()
}

fn default_diarization_model_id() -> String {
    "native-spectral-speaker-baseline".to_string()
}

fn is_pyannote_diarization_model(model_id: &str) -> bool {
    model_id
        .trim()
        .to_ascii_lowercase()
        .starts_with("pyannote/")
}

fn default_batch_chunks() -> bool {
    true
}

fn default_max_batch_size() -> Option<usize> {
    Some(4)
}

fn default_gating() -> bool {
    true
}

fn default_true() -> bool {
    true
}

fn default_vad_enabled() -> bool {
    true
}

fn default_vad_rms_threshold() -> f32 {
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

fn default_output_formats() -> Vec<OutputFormat> {
    vec![OutputFormat::Json]
}

fn default_pretty_json() -> bool {
    true
}

#[cfg(feature = "silero-vad")]
fn resolve_silero_model_path(vad: &VadConfig) -> Result<PathBuf, NativeWhisperxError> {
    let Some(model_bundle) = &vad.model_bundle else {
        return Err(NativeWhisperxError::InvalidConfig(
            "native Silero VAD requires --vad-model-bundle or VadConfig.model_bundle".to_string(),
        ));
    };
    let path = if model_bundle.is_dir() {
        model_bundle.join(vad.model_file.as_deref().unwrap_or("silero_vad.onnx"))
    } else if model_bundle
        .extension()
        .and_then(|extension| extension.to_str())
        == Some("onnx")
    {
        model_bundle.clone()
    } else {
        model_bundle.join(vad.model_file.as_deref().unwrap_or("silero_vad.onnx"))
    };
    if !path.is_file() {
        return Err(NativeWhisperxError::InvalidConfig(format!(
            "silero VAD model path `{}` does not exist or is not a file",
            path.display()
        )));
    }
    Ok(path)
}

#[cfg(feature = "pyannote-vad")]
fn resolve_pyannote_vad_model_path(vad: &VadConfig) -> Result<PathBuf, NativeWhisperxError> {
    let Some(model_bundle) = &vad.model_bundle else {
        return Err(NativeWhisperxError::InvalidConfig(
            "native pyannote VAD requires --vad-model-bundle or VadConfig.model_bundle".to_string(),
        ));
    };
    let path = if model_bundle.is_dir() {
        model_bundle.join(vad.model_file.as_deref().unwrap_or("segmentation.onnx"))
    } else if model_bundle
        .extension()
        .and_then(|extension| extension.to_str())
        == Some("onnx")
    {
        model_bundle.clone()
    } else {
        model_bundle.join(vad.model_file.as_deref().unwrap_or("segmentation.onnx"))
    };
    if !path.is_file() {
        return Err(NativeWhisperxError::InvalidConfig(format!(
            "pyannote VAD model path `{}` does not exist or is not a file",
            path.display()
        )));
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    const WHISPERX_SAMPLE: &[u8] =
        include_bytes!("../../../tests/fixtures/whisperx-parity-sample.json");

    #[test]
    fn map_diarization_maps_all_assignment_policy_variants() {
        for (input, expected) in [
            (
                AssignmentPolicy::Majority,
                SpeakerAssignmentPolicy::Majority,
            ),
            (
                AssignmentPolicy::NearestStart,
                SpeakerAssignmentPolicy::NearestStart,
            ),
            (
                AssignmentPolicy::StrictContained,
                SpeakerAssignmentPolicy::StrictContained,
            ),
        ] {
            let mapped = map_diarization(&DiarizationConfig {
                enabled: true,
                assignment_policy: input,
                ..DiarizationConfig::default()
            });
            assert_eq!(mapped.assignment_policy, expected);
        }
    }

    #[test]
    fn map_diarization_maps_pyannote_bundle_and_phase_artifacts() {
        let mapped = map_diarization(&DiarizationConfig {
            enabled: true,
            model_id: "pyannote/speaker-diarization-community-1".to_string(),
            model_bundle: Some(PathBuf::from("/models/pyannote-diarization")),
            manifest_file: Some("manifest.json".to_string()),
            segmentation_model_file: Some("segmentation.onnx".to_string()),
            embedding_model_file: Some("embedding.onnx".to_string()),
            plda_transform_file: Some("plda_transform.json".to_string()),
            plda_model_file: Some("plda_model.json".to_string()),
            clustering_config_file: Some("clustering.json".to_string()),
            return_speaker_embeddings: true,
            min_speakers: Some(2),
            max_speakers: Some(2),
            ..DiarizationConfig::default()
        });

        assert_eq!(mapped.model_id, "pyannote/speaker-diarization-community-1");
        assert_eq!(
            mapped.pyannote_model_bundle.as_deref(),
            Some(Path::new("/models/pyannote-diarization"))
        );
        assert_eq!(
            mapped.pyannote_manifest_file.as_deref(),
            Some("manifest.json")
        );
        assert_eq!(
            mapped.pyannote_segmentation_model_file.as_deref(),
            Some("segmentation.onnx")
        );
        assert_eq!(
            mapped.pyannote_embedding_model_file.as_deref(),
            Some("embedding.onnx")
        );
        assert_eq!(
            mapped.pyannote_plda_transform_file.as_deref(),
            Some("plda_transform.json")
        );
        assert_eq!(
            mapped.pyannote_plda_model_file.as_deref(),
            Some("plda_model.json")
        );
        assert_eq!(
            mapped.pyannote_clustering_config_file.as_deref(),
            Some("clustering.json")
        );
        assert!(mapped.return_speaker_embeddings);
        assert_eq!(mapped.min_speakers, Some(2));
        assert_eq!(mapped.max_speakers, Some(2));
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn native_diarization_with_runtime_speaker_library_labels_known_speaker(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let audio = two_speaker_loaded_audio();
        let library = speaker_library_matching_first_span(&audio)?;
        let transcript = timed_transcript(vec![("hello", 0.20, 0.50), ("world", 1.00, 1.40)])?;
        let mut provider = ConfiguredNativeDiarizationProvider {
            speaker_library: RuntimeSpeakerLibraryStatus::Loaded(RuntimeSpeakerLibrary {
                path: PathBuf::from("/project/.native-whisperx/speakers/library.json"),
                profile_count: 1,
                filtered_draft_profiles: 0,
                use_draft_profiles: true,
                library,
            }),
        };

        let response = provider.diarize(
            audio,
            &transcript,
            &DiarizationOptions {
                enabled: true,
                speaker: SpeakerDiarizationOptions {
                    model_id: "native-spectral-speaker-baseline".to_string(),
                    ..SpeakerDiarizationOptions::default()
                },
            },
        )?;

        assert_eq!(response.segments[0].speaker, "known-speaker");
        assert!(response
            .diagnostics
            .contains(&"speakerLibraryStatus=loaded".to_string()));
        assert!(response
            .diagnostics
            .contains(&"speakerLibraryProfiles=1".to_string()));
        Ok(())
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn native_diarization_missing_runtime_speaker_library_keeps_anonymous_labels(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let audio = two_speaker_loaded_audio();
        let transcript = timed_transcript(vec![("hello", 0.20, 0.50), ("world", 1.00, 1.40)])?;
        let mut provider = ConfiguredNativeDiarizationProvider {
            speaker_library: RuntimeSpeakerLibraryStatus::Missing(PathBuf::from(
                "/missing/library.json",
            )),
        };

        let response = provider.diarize(
            audio,
            &transcript,
            &DiarizationOptions {
                enabled: true,
                speaker: SpeakerDiarizationOptions {
                    model_id: "native-spectral-speaker-baseline".to_string(),
                    ..SpeakerDiarizationOptions::default()
                },
            },
        )?;

        assert!(response
            .segments
            .iter()
            .all(|segment| segment.speaker.starts_with("speaker_")));
        assert!(response
            .diagnostics
            .contains(&"speakerLibraryStatus=missing".to_string()));
        Ok(())
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn runtime_speaker_library_can_be_disabled_explicitly() {
        let config = NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig {
                enabled: true,
                disable_speaker_library: true,
                speaker_directory: SpeakerDirectorySelection {
                    scope: SpeakerDirectoryScope::Local,
                    explicit_path: Some(PathBuf::from("/ignored")),
                },
                ..DiarizationConfig::default()
            },
            output: OutputConfig::default(),
        };

        assert!(matches!(
            runtime_speaker_library_status(&config).expect("status"),
            RuntimeSpeakerLibraryStatus::Disabled
        ));
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn external_whisperx_ignores_runtime_speaker_library_selection() {
        let config = NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                provider: AsrProvider::ExternalWhisperX,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig {
                enabled: true,
                speaker_directory: SpeakerDirectorySelection {
                    scope: SpeakerDirectoryScope::Auto,
                    explicit_path: Some(PathBuf::from("/ignored")),
                },
                ..DiarizationConfig::default()
            },
            output: OutputConfig::default(),
        };

        assert!(matches!(
            runtime_speaker_library_status(&config).expect("status"),
            RuntimeSpeakerLibraryStatus::ExternalWhisperX
        ));
        let request = build_transcription_request(&config).expect("external request should build");
        match request.provider {
            TranscriptionProviderSelection::ExternalWhisperX(options) => {
                assert!(!options
                    .extra_args
                    .iter()
                    .any(|arg| arg.contains("speaker-library")
                        || arg.contains("speakerLibrary")
                        || arg.contains("speaker_directory")));
            }
            other => panic!("expected external provider, got {other:?}"),
        }
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn transcription_request_json_does_not_serialize_runtime_speaker_library() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig {
                enabled: true,
                speaker_directory: SpeakerDirectorySelection {
                    scope: SpeakerDirectoryScope::Auto,
                    explicit_path: Some(PathBuf::from("/project/speakers")),
                },
                ..DiarizationConfig::default()
            },
            output: OutputConfig::default(),
        })
        .expect("request should build");

        let json = serde_json::to_string(&request).expect("request JSON");
        assert!(!json.contains("Speaker A"));
        assert!(!json.contains("profiles"));
        assert!(!json.contains("speakerDirectory"));
        assert!(!json.contains("speakerLibrary"));
    }

    #[test]
    fn native_speaker_embeddings_require_pyannote_bundle() {
        let error = validate_native_diarization_support(&DiarizationConfig {
            enabled: true,
            return_speaker_embeddings: true,
            ..DiarizationConfig::default()
        })
        .expect_err("non-pyannote embeddings should be rejected")
        .to_string();

        assert!(error.contains("native speaker embeddings require"));

        let error = validate_native_diarization_support(&DiarizationConfig {
            enabled: true,
            model_id: "pyannote/speaker-diarization-community-1".to_string(),
            return_speaker_embeddings: true,
            ..DiarizationConfig::default()
        })
        .expect_err("pyannote embeddings without a bundle should be rejected")
        .to_string();

        assert!(error.contains("native speaker embeddings require"));
    }

    #[test]
    fn native_diarization_bundle_requires_pyannote_model() {
        let error = validate_native_diarization_support(&DiarizationConfig {
            enabled: true,
            model_bundle: Some(PathBuf::from("/models/pyannote-diarization")),
            ..DiarizationConfig::default()
        })
        .expect_err("bundle without pyannote model should be rejected")
        .to_string();

        assert!(error.contains("modelBundle is only supported for pyannote"));
    }

    #[test]
    fn native_pyannote_diarization_diagnostics_identify_phases() {
        let mut response = fixture_response_with_chars();
        let config = NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig {
                enabled: true,
                model_id: "pyannote/speaker-diarization-community-1".to_string(),
                model_bundle: Some(PathBuf::from("/models/pyannote-diarization")),
                ..DiarizationConfig::default()
            },
            output: OutputConfig::default(),
        };

        append_native_diarization_diagnostics(&mut response, &config);

        for expected in [
            "diarizationPhase=segmentation",
            "diarizationPhase=embedding",
            "diarizationPhase=plda",
            "diarizationPhase=vbx",
            "diarizationPhase=clustering",
        ] {
            assert!(
                response
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic == expected),
                "missing {expected}: {:?}",
                response.diagnostics
            );
        }
    }

    #[test]
    fn maps_native_surface_defaults() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        assert!(matches!(request.source, TranscriptionSource::Path { .. }));
        assert!(request.vad.enabled);
        assert!(request.alignment.enabled);
        assert_eq!(request.output.formats, vec!["json"]);
        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.model_id, "small");
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn maps_native_english_only_whisper_alias_to_language_hint() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                model_id: "tiny.en".to_string(),
                language: None,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.language.as_deref(), Some("en"));
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn maps_native_multilingual_whisper_model_without_language_hint() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                model_id: "small".to_string(),
                language: None,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.language, None);
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn explicit_native_language_overrides_english_only_model_hint() {
        let asr = AsrConfig {
            model_id: "openai/whisper-tiny.en".to_string(),
            language: Some("de".to_string()),
            ..AsrConfig::default()
        };

        assert_eq!(native_language_hint(&asr).as_deref(), Some("de"));
    }

    #[test]
    fn maps_config_to_transcription_request() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                language: Some("en".to_string()),
                whisper_bundle: Some(PathBuf::from("models/whisper")),
                device: DevicePreference::Cpu,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig {
                enabled: true,
                model_id: "facebook/wav2vec2-base-960h".to_string(),
                model_bundle: Some(PathBuf::from("models/wav2vec2")),
                model_dir: Some(PathBuf::from("models/cache")),
                model_cache_only: true,
                interpolate_method: AlignmentInterpolationMethod::Linear,
                return_char_alignments: true,
            },
            diarization: DiarizationConfig::default(),
            output: OutputConfig {
                formats: vec![OutputFormat::Json, OutputFormat::Srt],
                ..OutputConfig::default()
            },
        })
        .expect("request should build");

        assert!(matches!(request.source, TranscriptionSource::Path { .. }));
        assert!(request.alignment.enabled);
        assert_eq!(
            request.alignment.model_dir,
            Some(PathBuf::from("models/cache"))
        );
        assert!(request.alignment.model_cache_only);
        assert_eq!(
            request.alignment.interpolate_method,
            AlignmentInterpolationMethod::Linear
        );
        assert_eq!(request.alignment.device, NativeDevicePreference::Cpu);
        assert!(request.alignment.return_char_alignments);
        assert_eq!(request.output.formats, vec!["json", "srt"]);
        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.language.as_deref(), Some("en"));
                assert_eq!(options.device, NativeDevicePreference::Cpu);
                assert_eq!(options.model_bundle, Some(PathBuf::from("models/whisper")));
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn maps_native_asr_cuda_device_to_alignment_options() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                device: DevicePreference::Cuda,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig {
                enabled: true,
                ..AlignmentConfig::default()
            },
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        assert_eq!(request.alignment.device, NativeDevicePreference::Cuda);
    }

    #[test]
    fn native_alignment_diagnostics_include_fallback_and_retry_counts() {
        let mut response = fixture_response_with_chars();

        append_native_alignment_diagnostics(
            &mut response,
            &NativeWhisperxConfig {
                input: InputSource::Path {
                    path: PathBuf::from("sample.wav"),
                },
                asr: AsrConfig::default(),
                translation: TranslationConfig::default(),
                vad: VadConfig::default(),
                alignment: AlignmentConfig {
                    enabled: true,
                    return_char_alignments: true,
                    ..AlignmentConfig::default()
                },
                diarization: DiarizationConfig::default(),
                output: OutputConfig::default(),
            },
        );

        for expected in [
            "alignmentFallbackCount=0",
            "alignmentRetryCount=0",
            "alignmentWordTimingMissingCount=0",
            "alignmentCharTimingMissingCount=0",
        ] {
            assert!(
                response
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic == expected),
                "diagnostics should include `{expected}`: {:?}",
                response.diagnostics
            );
        }
    }

    #[test]
    fn maps_native_asr_model_cache_options() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                model_dir: Some(PathBuf::from("models")),
                model_cache_only: true,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.model_dir, Some(PathBuf::from("models")));
                assert!(options.model_cache_only);
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn accepts_native_decode_controls_that_match_greedy_defaults() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                decode: WhisperxDecodeConfig {
                    temperature: vec![0.0],
                    condition_on_previous_text: Some(false),
                    ..WhisperxDecodeConfig::default()
                },
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("greedy native decode defaults should build");

        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(_) => {}
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn rejects_native_decode_controls_with_specific_reasons() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                decode: WhisperxDecodeConfig {
                    beam_size: Some(5),
                    initial_prompt: Some("context".to_string()),
                    logprob_threshold: Some(-1.0),
                    ..WhisperxDecodeConfig::default()
                },
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native decode controls should be rejected");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        let message = error.to_string();
        assert!(message.contains("--beam_size (beam search is not exposed"));
        assert!(message.contains("--initial_prompt (prompt-prefilled decoder context"));
        assert!(message
            .contains("--logprob_threshold (fallback thresholds require token log probability"));
        assert!(message.contains("external-whisperx"));
    }

    #[test]
    fn reports_each_unsupported_native_decode_control() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                compute_type: Some("int8".to_string()),
                device_index: Some("0".to_string()),
                decode: WhisperxDecodeConfig {
                    temperature: vec![0.0, 0.2],
                    best_of: Some(3),
                    patience: Some(1.2),
                    length_penalty: Some(1.1),
                    suppress_tokens: Some("-1".to_string()),
                    suppress_numerals: true,
                    initial_prompt: Some("domain prompt".to_string()),
                    hotwords: Some("proper nouns".to_string()),
                    condition_on_previous_text: Some(true),
                    fp16: Some(false),
                    compression_ratio_threshold: Some(2.4),
                    logprob_threshold: Some(-1.0),
                    no_speech_threshold: Some(0.6),
                    threads: Some(4),
                    ..WhisperxDecodeConfig::default()
                },
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native unsupported controls should be rejected");

        let message = error.to_string();
        for expected in [
            "--compute_type",
            "--device_index",
            "--temperature",
            "--best_of",
            "--patience",
            "--length_penalty",
            "--suppress_tokens",
            "--suppress_numerals",
            "--initial_prompt",
            "--hotwords",
            "--condition_on_previous_text",
            "--fp16",
            "--compression_ratio_threshold",
            "--logprob_threshold",
            "--no_speech_threshold",
            "--threads",
        ] {
            assert!(
                message.contains(expected),
                "error should mention `{expected}`: {message}"
            );
        }
    }

    #[cfg(feature = "pyannote-vad")]
    #[test]
    fn rejects_native_pyannote_vad_without_model_bundle() {
        let error = build_transcription_request(&NativeWhisperxConfig {
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
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native pyannote VAD should be rejected");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains("--vad-model-bundle"));
    }

    #[cfg(not(feature = "pyannote-vad"))]
    #[test]
    fn rejects_native_pyannote_vad_without_feature() {
        let error = build_transcription_request(&NativeWhisperxConfig {
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
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native pyannote VAD should require a feature");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains("pyannote-vad feature"));
    }

    #[cfg(feature = "pyannote-vad")]
    #[test]
    fn accepts_native_pyannote_vad_with_local_onnx_bundle() {
        let temp = tempfile::tempdir().expect("tempdir");
        let model = temp.path().join("pyannote_vad.onnx");
        fs::write(&model, b"fixture").expect("model file");

        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig {
                method: VadMethod::Pyannote,
                model_bundle: Some(temp.path().to_path_buf()),
                model_file: Some("pyannote_vad.onnx".to_string()),
                ..VadConfig::default()
            },
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("native pyannote VAD should accept an explicit local ONNX bundle");

        assert!(request.vad.enabled);
    }

    #[cfg(not(feature = "silero-vad"))]
    #[test]
    fn rejects_native_silero_without_feature() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig {
                method: VadMethod::Silero,
                ..VadConfig::default()
            },
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native silero VAD should be rejected");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains("silero-vad feature"));
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn silero_requires_model_bundle_with_feature() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig {
                method: VadMethod::Silero,
                ..VadConfig::default()
            },
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native silero VAD should require a model bundle");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains("--vad-model-bundle"));
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn resolves_silero_direct_onnx_path() {
        let temp = tempfile::tempdir().expect("tempdir");
        let model = temp.path().join("silero.onnx");
        fs::write(&model, b"fixture").expect("model file");
        let vad = VadConfig {
            model_bundle: Some(model.clone()),
            ..VadConfig::default()
        };

        assert_eq!(resolve_silero_model_path(&vad).expect("path"), model);
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn resolves_silero_bundle_directory() {
        let temp = tempfile::tempdir().expect("tempdir");
        let model = temp.path().join("silero_vad.onnx");
        fs::write(&model, b"fixture").expect("model file");
        let vad = VadConfig {
            model_bundle: Some(temp.path().to_path_buf()),
            ..VadConfig::default()
        };

        assert_eq!(resolve_silero_model_path(&vad).expect("path"), model);
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn resolves_silero_custom_model_file() {
        let temp = tempfile::tempdir().expect("tempdir");
        let model = temp.path().join("model.onnx");
        fs::write(&model, b"fixture").expect("model file");
        let vad = VadConfig {
            model_bundle: Some(temp.path().to_path_buf()),
            model_file: Some("model.onnx".to_string()),
            ..VadConfig::default()
        };

        assert_eq!(resolve_silero_model_path(&vad).expect("path"), model);
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn rejects_invalid_silero_onset_before_model_resolution() {
        let error = validate_native_silero_config(&VadConfig {
            method: VadMethod::Silero,
            onset: Some(0.0),
            ..VadConfig::default()
        })
        .expect_err("invalid onset should fail");

        assert!(error.to_string().contains("vad_onset"));
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn rejects_invalid_silero_chunk_size_before_model_resolution() {
        let error = validate_native_silero_config(&VadConfig {
            method: VadMethod::Silero,
            chunk_size: Some(0.0),
            ..VadConfig::default()
        })
        .expect_err("invalid chunk size should fail");

        assert!(error.to_string().contains("chunk_size"));
    }

    #[test]
    fn rejects_native_translate_with_alignment() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                task: TranscriptionTask::Translate,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native translate should be rejected");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains(
            "native --task translate requires --translation-model or --translation-bundle"
        ));
    }

    #[test]
    fn rejects_native_translate_without_alignment() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                task: TranscriptionTask::Translate,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig {
                enabled: false,
                ..AlignmentConfig::default()
            },
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native translate should require a translation model");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains(
            "native --task translate requires --translation-model or --translation-bundle"
        ));
    }

    #[test]
    fn maps_native_translate_with_translation_model_to_post_asr_transcribe_request() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                task: TranscriptionTask::Translate,
                language: Some("de".to_string()),
                ..AsrConfig::default()
            },
            translation: TranslationConfig {
                enabled: true,
                model_id: Some("Helsinki-NLP/opus-mt-de-en".to_string()),
                target_language: Some("en".to_string()),
                ..TranslationConfig::default()
            },
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("native post-ASR translation should build with alignment");

        assert!(request.alignment.enabled);
        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.language.as_deref(), Some("de"));
                assert_eq!(options.task, UpstreamTranscriptionTask::Transcribe);
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn translates_segments_with_configured_languages_and_max_tokens() {
        #[derive(Default)]
        struct FakeTranslator {
            seen: Vec<TranslationRunOptions>,
        }

        impl SegmentTranslator for FakeTranslator {
            fn model_id(&self) -> &str {
                "Helsinki-NLP/opus-mt-de-en"
            }

            fn model_source(&self) -> &'static str {
                "hugging-face-cache"
            }

            fn translate_segment(
                &mut self,
                text: &str,
                options: &TranslationRunOptions,
            ) -> Result<String, NativeWhisperxError> {
                self.seen.push(options.clone());
                Ok(format!("{text} translated"))
            }
        }

        let config = NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                task: TranscriptionTask::Translate,
                language: Some("de".to_string()),
                ..AsrConfig::default()
            },
            translation: TranslationConfig {
                enabled: true,
                model_id: Some("Helsinki-NLP/opus-mt-de-en".to_string()),
                source_language: Some("de".to_string()),
                target_language: Some("en".to_string()),
                max_new_tokens: 7,
                ..TranslationConfig::default()
            },
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        };
        let mut segment = text_transcripts::TranscriptSegmentContract::new(0, "Guten Tag");
        segment
            .words
            .push(text_transcripts::TranscriptWordContract {
                text: "Guten".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.2),
                confidence: None,
                speaker: None,
                attributes: Default::default(),
            });
        let mut response = TranscriptionPipelineResponse {
            accepted: true,
            operation: "transcribe".to_string(),
            provider: "native".to_string(),
            model_id: "small".to_string(),
            transcript: TranscriptionContract::new(vec![segment]),
            vad_segments: Vec::new(),
            alignment: None,
            diarization: None,
            artifacts: Vec::new(),
            diagnostics: Vec::new(),
        };
        let mut translator = FakeTranslator::default();

        translate_response_segments(&mut response, &config, &mut translator)
            .expect("translation should update transcript");

        assert_eq!(response.transcript.language.as_deref(), Some("en"));
        assert_eq!(
            response.transcript.text.as_deref(),
            Some("Guten Tag translated")
        );
        assert_eq!(response.transcript.segments[0].text, "Guten Tag translated");
        assert_eq!(
            response.transcript.segments[0].language.as_deref(),
            Some("en")
        );
        assert!(response.transcript.segments[0].words.is_empty());
        assert_eq!(
            translator.seen,
            vec![TranslationRunOptions {
                source_language: Some("de".to_string()),
                target_language: "en".to_string(),
                max_new_tokens: 7,
            }]
        );
        assert!(response
            .diagnostics
            .contains(&"translationModelSource=hugging-face-cache".to_string()));
        assert!(response
            .diagnostics
            .contains(&"translationMaxNewTokens=7".to_string()));
    }

    #[cfg(feature = "translation")]
    #[test]
    fn translation_cache_only_resolves_fake_hugging_face_snapshot() {
        let temp = tempfile::tempdir().unwrap();
        let snapshot = temp
            .path()
            .join("models--Helsinki-NLP--opus-mt-de-en/snapshots/abc123");
        fs::create_dir_all(&snapshot).unwrap();
        for file in REQUIRED_TRANSLATION_FILES {
            fs::write(snapshot.join(file), "{}").unwrap();
        }
        fs::write(snapshot.join("model.safetensors"), "").unwrap();

        let resolved = resolve_translation_bundle(&TranslationConfig {
            enabled: true,
            model_id: Some("Helsinki-NLP/opus-mt-de-en".to_string()),
            model_dir: Some(temp.path().to_path_buf()),
            model_cache_only: true,
            ..TranslationConfig::default()
        })
        .expect("cache snapshot should resolve");

        assert_eq!(resolved.root, snapshot);
        assert_eq!(resolved.source, "hugging-face-cache");
        assert_eq!(resolved.weight_format, TranslationWeightFormat::Safetensors);
    }

    #[test]
    fn maps_external_whisperx_all_surface_args() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                provider: AsrProvider::ExternalWhisperX,
                task: TranscriptionTask::Translate,
                model_id: "small".to_string(),
                language: Some("en".to_string()),
                device: DevicePreference::Cuda,
                device_index: Some("0".to_string()),
                compute_type: Some("int8".to_string()),
                max_batch_size: Some(8),
                decode: WhisperxDecodeConfig {
                    temperature: vec![0.0, 0.2],
                    best_of: Some(3),
                    beam_size: Some(5),
                    patience: Some(1.2),
                    length_penalty: Some(1.1),
                    suppress_tokens: Some("-1".to_string()),
                    suppress_numerals: true,
                    initial_prompt: Some("domain prompt".to_string()),
                    hotwords: Some("proper nouns".to_string()),
                    condition_on_previous_text: Some(false),
                    fp16: Some(false),
                    compression_ratio_threshold: Some(2.4),
                    logprob_threshold: Some(-1.0),
                    no_speech_threshold: Some(0.6),
                    threads: Some(4),
                },
                external_whisperx: ExternalWhisperxConfig {
                    model: "small".to_string(),
                    align_model: Some("external-align".to_string()),
                    ..ExternalWhisperxConfig::default()
                },
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig {
                method: VadMethod::Silero,
                onset: Some(0.5),
                offset: Some(0.363),
                chunk_size: Some(20.0),
                ..VadConfig::default()
            },
            alignment: AlignmentConfig {
                enabled: false,
                model_id: "fallback-align".to_string(),
                model_dir: Some(PathBuf::from("model-cache")),
                model_cache_only: true,
                return_char_alignments: true,
                ..AlignmentConfig::default()
            },
            diarization: DiarizationConfig {
                enabled: true,
                model_id: "pyannote/speaker-diarization-community-1".to_string(),
                hf_token: Some("token".to_string()),
                return_speaker_embeddings: true,
                min_speakers: Some(1),
                max_speakers: Some(2),
                ..DiarizationConfig::default()
            },
            output: OutputConfig {
                formats: vec![OutputFormat::All],
                subtitles: SubtitleConfig {
                    max_line_width: Some(42),
                    max_line_count: Some(2),
                    highlight_words: true,
                    segment_resolution: SegmentResolution::Chunk,
                },
                ..OutputConfig::default()
            },
        })
        .expect("request should build");

        assert_eq!(
            request.output.formats,
            vec!["txt", "vtt", "srt", "tsv", "aud", "json"]
        );
        match request.provider {
            TranscriptionProviderSelection::ExternalWhisperX(options) => {
                assert_eq!(options.model, "small");
                assert_eq!(options.task, UpstreamTranscriptionTask::Translate);
                assert_eq!(options.language.as_deref(), Some("en"));
                assert_eq!(options.device, WhisperXDevice::Cuda);
                assert_eq!(options.compute_type.as_deref(), Some("int8"));
                assert_eq!(options.batch_size, Some(8));
                assert!(options.no_align);
                assert_eq!(options.align_model.as_deref(), Some("external-align"));
                assert_eq!(options.model_dir, Some(PathBuf::from("model-cache")));
                assert!(!options.model_cache_only);
                assert!(options.return_char_alignments);
                assert!(!options.diarize);
                assert!(contains_pair(
                    &options.extra_args,
                    "--model_cache_only",
                    "True"
                ));
                assert!(contains_pair(&options.extra_args, "--device_index", "0"));
                assert!(contains_pair(&options.extra_args, "--vad_method", "silero"));
                assert!(contains_pair(&options.extra_args, "--vad_onset", "0.5"));
                assert!(contains_pair(&options.extra_args, "--vad_offset", "0.363"));
                assert!(contains_pair(&options.extra_args, "--chunk_size", "20"));
                assert!(contains_pair(&options.extra_args, "--temperature", "0,0.2"));
                assert!(contains_pair(&options.extra_args, "--best_of", "3"));
                assert!(contains_pair(&options.extra_args, "--beam_size", "5"));
                assert!(contains_pair(&options.extra_args, "--patience", "1.2"));
                assert!(contains_pair(
                    &options.extra_args,
                    "--length_penalty",
                    "1.1"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--suppress_tokens",
                    "-1"
                ));
                assert!(options
                    .extra_args
                    .contains(&"--suppress_numerals".to_string()));
                assert!(contains_pair(
                    &options.extra_args,
                    "--initial_prompt",
                    "domain prompt"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--hotwords",
                    "proper nouns"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--condition_on_previous_text",
                    "false"
                ));
                assert!(contains_pair(&options.extra_args, "--fp16", "false"));
                assert!(contains_pair(
                    &options.extra_args,
                    "--compression_ratio_threshold",
                    "2.4"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--logprob_threshold",
                    "-1"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--no_speech_threshold",
                    "0.6"
                ));
                assert!(contains_pair(&options.extra_args, "--threads", "4"));
                assert!(options.extra_args.contains(&"--diarize".to_string()));
                assert!(contains_pair(
                    &options.extra_args,
                    "--diarize_model",
                    "pyannote/speaker-diarization-community-1"
                ));
                assert!(contains_pair(&options.extra_args, "--hf_token", "token"));
                assert!(options
                    .extra_args
                    .contains(&"--speaker_embeddings".to_string()));
                assert!(contains_pair(&options.extra_args, "--max_line_width", "42"));
                assert!(contains_pair(&options.extra_args, "--max_line_count", "2"));
                assert!(contains_pair(
                    &options.extra_args,
                    "--highlight_words",
                    "True"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--segment_resolution",
                    "chunk"
                ));
            }
            other => panic!("expected external provider, got {other:?}"),
        }
    }

    #[test]
    fn maps_external_silero_still_delegated() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                provider: AsrProvider::ExternalWhisperX,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig {
                method: VadMethod::Silero,
                model_bundle: Some(PathBuf::from("native-only/silero_vad.onnx")),
                model_file: Some("ignored.onnx".to_string()),
                ..VadConfig::default()
            },
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("external silero should build");

        match request.provider {
            TranscriptionProviderSelection::ExternalWhisperX(options) => {
                assert!(contains_pair(&options.extra_args, "--vad_method", "silero"));
                assert!(!options
                    .extra_args
                    .iter()
                    .any(|arg| arg.contains("vad_model")));
            }
            other => panic!("expected external provider, got {other:?}"),
        }
    }

    #[test]
    fn imports_whisperx_fixture() {
        let transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        assert_eq!(transcript.language.as_deref(), Some("en"));
        assert_eq!(transcript.segments.len(), 2);
        assert_eq!(transcript.text_or_joined(), "hello world second speaker");
    }

    #[test]
    fn correct_speaker_persists_confirmed_profile_and_writes_corrected_output() {
        let temp = tempfile::tempdir().expect("tempdir");
        let speaker_directory = temp.path().join("speakers");
        let output_dir = temp.path().join("out");

        let report = correct_speaker(SpeakerCorrectionRequest {
            transcript: correction_transcript(),
            audio: InputSource::Samples {
                samples: correction_samples(),
                sample_rate: 16_000,
                channels: 1,
                source: Some("sample.wav".to_string()),
            },
            from_speaker: "speaker_0".to_string(),
            to_label: "Alice".to_string(),
            speaker_id: Some("alice".to_string()),
            ranges: Vec::new(),
            speaker_directory: SpeakerDirectorySelection {
                scope: SpeakerDirectoryScope::Auto,
                explicit_path: Some(speaker_directory.clone()),
            },
            output: OutputConfig {
                output_dir: Some(output_dir.clone()),
                basename: Some("sample.corrected".to_string()),
                formats: vec![OutputFormat::Json],
                ..OutputConfig::default()
            },
        })
        .expect("correction should succeed");

        assert_eq!(report.profile_id, "alice");
        assert_eq!(report.label, "Alice");
        assert_eq!(report.corrected_from, "speaker_0");
        assert!(!report.updated_existing_profile);
        assert!(report.enrolled_seconds > 0.9);
        assert_eq!(
            report.transcript.segments[0].speaker.as_deref(),
            Some("Alice")
        );
        assert_eq!(
            report.transcript.segments[1].speaker.as_deref(),
            Some("speaker_1")
        );
        assert!(speaker_library_path(&speaker_directory).is_file());
        let library =
            fs::read_to_string(speaker_library_path(&speaker_directory)).expect("library");
        assert!(library.contains(r#""id": "alice""#));
        assert!(library.contains(r#""status": "confirmed""#));
        let corrected = output_dir.join("sample.corrected.json");
        assert!(corrected.is_file());
        assert!(fs::read_to_string(corrected)
            .expect("corrected")
            .contains("Alice"));
    }

    #[test]
    fn correct_speaker_range_limits_relabeling() {
        let temp = tempfile::tempdir().expect("tempdir");
        let speaker_directory = temp.path().join("speakers");
        let mut transcript = correction_transcript();
        transcript.segments[1].speaker = Some("speaker_0".to_string());

        let report = correct_speaker(SpeakerCorrectionRequest {
            transcript,
            audio: InputSource::Samples {
                samples: correction_samples(),
                sample_rate: 16_000,
                channels: 1,
                source: Some("sample.wav".to_string()),
            },
            from_speaker: "speaker_0".to_string(),
            to_label: "Alice".to_string(),
            speaker_id: Some("alice".to_string()),
            ranges: vec![SpeakerCorrectionRange {
                start_seconds: 0.0,
                end_seconds: 1.0,
            }],
            speaker_directory: SpeakerDirectorySelection {
                scope: SpeakerDirectoryScope::Auto,
                explicit_path: Some(speaker_directory),
            },
            output: OutputConfig::default(),
        })
        .expect("correction should succeed");

        assert_eq!(
            report.transcript.segments[0].speaker.as_deref(),
            Some("Alice")
        );
        assert_eq!(
            report.transcript.segments[1].speaker.as_deref(),
            Some("speaker_0")
        );
    }

    #[test]
    fn correct_speaker_rejects_empty_selected_audio() {
        let temp = tempfile::tempdir().expect("tempdir");
        let error = correct_speaker(SpeakerCorrectionRequest {
            transcript: correction_transcript(),
            audio: InputSource::Samples {
                samples: correction_samples(),
                sample_rate: 16_000,
                channels: 1,
                source: Some("sample.wav".to_string()),
            },
            from_speaker: "missing".to_string(),
            to_label: "Alice".to_string(),
            speaker_id: Some("alice".to_string()),
            ranges: Vec::new(),
            speaker_directory: SpeakerDirectorySelection {
                scope: SpeakerDirectoryScope::Auto,
                explicit_path: Some(temp.path().join("speakers")),
            },
            output: OutputConfig::default(),
        })
        .expect_err("missing source speaker should fail");

        assert!(error
            .to_string()
            .contains("found no timed transcript segments"));
    }

    #[test]
    fn writes_requested_outputs() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        let files = write_outputs_with_options(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![
                    OutputFormat::Json,
                    OutputFormat::NativeJson,
                    OutputFormat::Srt,
                    OutputFormat::Vtt,
                    OutputFormat::Txt,
                    OutputFormat::Tsv,
                    OutputFormat::Audacity,
                ],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig::default(),
            },
            true,
        )
        .expect("outputs should write");

        assert_eq!(files.len(), 7);
        let json_path = temp.path().join("sample.json");
        let native_json_path = temp.path().join("sample.native.json");
        let srt_path = temp.path().join("sample.srt");
        let vtt_path = temp.path().join("sample.vtt");
        let txt_path = temp.path().join("sample.txt");
        let tsv_path = temp.path().join("sample.tsv");
        let aud_path = temp.path().join("sample.aud");
        assert!(json_path.is_file());
        assert!(native_json_path.is_file());
        assert!(srt_path.is_file());
        assert!(vtt_path.is_file());
        assert!(txt_path.is_file());
        assert!(tsv_path.is_file());
        assert!(aud_path.is_file());

        let whisperx_json: serde_json::Value =
            serde_json::from_slice(&fs::read(json_path).expect("json"))
                .expect("valid whisperx json");
        assert!(whisperx_json.get("segments").is_some());
        assert!(whisperx_json.get("word_segments").is_some());
        assert!(whisperx_json["segments"][0].get("start").is_some());
        assert!(whisperx_json["segments"][0].get("end").is_some());
        assert!(whisperx_json["segments"][0].get("startSeconds").is_none());
        assert_eq!(whisperx_json["segments"][0]["chars"][0]["char"], "h");

        let native_json: serde_json::Value =
            serde_json::from_slice(&fs::read(native_json_path).expect("native json"))
                .expect("valid native json");
        assert!(native_json["segments"][0].get("startSeconds").is_some());
        assert!(native_json["segments"][0].get("chars").is_some());

        let txt = fs::read_to_string(txt_path).expect("txt");
        assert_eq!(
            txt,
            "[SPEAKER_00]: hello world\n[SPEAKER_01]: second speaker\n"
        );
        let srt = fs::read_to_string(srt_path).expect("srt");
        assert!(srt.contains("00:00:00,000 --> 00:00:01,100"));
        assert!(srt.contains("[SPEAKER_00]: hello world"));
        let vtt = fs::read_to_string(vtt_path).expect("vtt");
        assert!(vtt.starts_with("WEBVTT\n\n"));
        assert!(vtt.contains("00:01.350 --> 00:02.350"));
        assert!(vtt.contains("[SPEAKER_01]: second speaker"));
        let tsv = fs::read_to_string(tsv_path).expect("tsv");
        assert!(tsv.starts_with("start\tend\ttext\n"));
        assert!(tsv.contains("0\t1200\thello world"));
        assert!(tsv.contains("1350\t2400\tsecond speaker"));
        let aud = fs::read_to_string(aud_path).expect("aud");
        assert!(aud.contains("0\t1.2\t[[SPEAKER_00]]hello world"));
        assert!(aud.contains("1.35\t2.4\t[[SPEAKER_01]]second speaker"));
    }

    #[test]
    fn all_format_writes_whisperx_compatible_set_without_native_json() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs_with_options(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::All],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig::default(),
            },
            true,
        )
        .expect("outputs should write");

        let mut names = fs::read_dir(temp.path())
            .expect("read output dir")
            .map(|entry| {
                entry
                    .expect("dir entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<_>>();
        names.sort();
        assert_eq!(
            names,
            vec![
                "sample.aud",
                "sample.json",
                "sample.srt",
                "sample.tsv",
                "sample.txt",
                "sample.vtt",
            ]
        );
    }

    #[test]
    fn output_stems_keep_multi_input_writes_collision_safe() {
        let temp = tempfile::tempdir().expect("tempdir");
        let mut first = fixture_response_with_chars();
        first.transcript.source = Some("audio/first-input.wav".to_string());
        let mut second = fixture_response_with_chars();
        second.transcript.source = Some("audio/second-input.wav".to_string());
        let output = OutputConfig {
            output_dir: Some(temp.path().to_path_buf()),
            formats: vec![OutputFormat::All],
            basename: None,
            pretty_json: true,
            subtitles: SubtitleConfig::default(),
        };

        write_outputs_with_options(&first, &output, true).expect("first outputs should write");
        write_outputs_with_options(&second, &output, true).expect("second outputs should write");

        let mut names = fs::read_dir(temp.path())
            .expect("read output dir")
            .map(|entry| {
                entry
                    .expect("dir entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<_>>();
        names.sort();

        assert_eq!(
            names,
            vec![
                "first-input.aud",
                "first-input.json",
                "first-input.srt",
                "first-input.tsv",
                "first-input.txt",
                "first-input.vtt",
                "second-input.aud",
                "second-input.json",
                "second-input.srt",
                "second-input.tsv",
                "second-input.txt",
                "second-input.vtt",
            ]
        );
    }

    #[test]
    fn txt_writes_each_segment_without_speakers() {
        let mut response = fixture_response_with_chars();
        for segment in &mut response.transcript.segments {
            segment.speaker = None;
        }
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Txt],
                basename: Some("sample".to_string()),
                ..OutputConfig::default()
            },
        )
        .expect("txt should write");

        let txt = fs::read_to_string(temp.path().join("sample.txt")).expect("txt");
        assert_eq!(txt, "hello world\nsecond speaker\n");
    }

    #[test]
    fn tsv_includes_header_and_replaces_tabs() {
        let mut response = fixture_response_with_chars();
        response.transcript.segments[0].text = " hello\tworld ".to_string();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Tsv],
                basename: Some("sample".to_string()),
                ..OutputConfig::default()
            },
        )
        .expect("tsv should write");

        let tsv = fs::read_to_string(temp.path().join("sample.tsv")).expect("tsv");
        assert!(tsv.starts_with("start\tend\ttext\n"));
        assert!(tsv.contains("0\t1200\thello world\n"));
    }

    #[test]
    fn subtitle_options_highlight_and_wrap_text() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Srt, OutputFormat::Vtt],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig {
                    max_line_width: Some(8),
                    max_line_count: None,
                    highlight_words: true,
                    segment_resolution: SegmentResolution::Sentence,
                },
            },
        )
        .expect("subtitles should write");

        let srt = fs::read_to_string(temp.path().join("sample.srt")).expect("srt");
        assert!(srt.contains("<u>hello</u>"));
        assert!(srt.contains("[SPEAKER_00]: <u>hello</u> \nworld"));
        assert!(srt.contains("[SPEAKER_00]: hello \n<u>world</u>"));
        let vtt = fs::read_to_string(temp.path().join("sample.vtt")).expect("vtt");
        assert!(vtt.contains("<u>hello</u>"));
    }

    #[test]
    fn subtitle_max_line_count_merges_overflow() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Srt],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig {
                    max_line_width: Some(8),
                    max_line_count: Some(1),
                    highlight_words: false,
                    segment_resolution: SegmentResolution::Sentence,
                },
            },
        )
        .expect("subtitles should write");

        let srt = fs::read_to_string(temp.path().join("sample.srt")).expect("srt");
        assert!(srt.contains("[SPEAKER_00]: hello\n\n2"));
        assert!(srt.contains("[SPEAKER_00]: world\n\n3"));
        assert!(srt.contains("[SPEAKER_01]: second\n\n4"));
        assert!(srt.contains("[SPEAKER_01]: speaker\n\n"));
    }

    #[test]
    fn subtitle_word_cues_join_languages_without_spaces() {
        let mut response = fixture_response_with_chars();
        response.transcript.language = Some("ja".to_string());
        response.transcript.segments[0].speaker = None;
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Srt],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig::default(),
            },
        )
        .expect("subtitles should write");

        let srt = fs::read_to_string(temp.path().join("sample.srt")).expect("srt");
        assert!(srt.contains("helloworld"));
    }

    #[test]
    fn whisperx_json_omits_chars_when_not_requested() {
        let mut transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        transcript.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });

        let without_chars = whisperx_json_value(&transcript, false);
        let with_chars = whisperx_json_value(&transcript, true);

        assert!(without_chars["segments"][0].get("chars").is_none());
        assert!(with_chars["segments"][0].get("chars").is_some());
    }

    #[test]
    fn parity_comparison_accepts_permutation_equivalent_speaker_turns() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.segments[0].speaker = Some("reference-speaker-b".to_string());
        whisperx.segments[1].speaker = Some("reference-speaker-a".to_string());

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig {
                text: false,
                language: false,
                segment_text: false,
                word_text: false,
                char_count: false,
                char_content: false,
                segment_count: false,
                word_count: false,
                segment_timing: false,
                word_timing: false,
                speaker_turns: true,
                vad_segments: false,
                vad_segment_timing: false,
                vad_segment_count: false,
            },
        );

        assert!(comparison.speaker_turns_match);
        assert!(comparison.passed);
    }

    #[test]
    fn parity_comparison_reports_text_language_word_and_char_categories() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.language = Some("de".to_string());
        whisperx.segments[0].text = "hello changed".to_string();
        whisperx.segments[0].words[0].text = "changed".to_string();
        whisperx.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig::default(),
        );

        assert_eq!(comparison.language_matches, Some(false));
        assert_eq!(comparison.segment_text_matches, Some(false));
        assert_eq!(comparison.word_text_matches, Some(false));
        assert_eq!(comparison.char_count_matches, Some(false));
        assert_eq!(comparison.char_content_matches, Some(false));
        assert!(!comparison.passed);
        for expected in [
            "language differs: native=Some(\"en\") reference=Some(\"de\")",
            "segment text sequence differs: native=[\"hello world\", \"second speaker\"] reference=[\"hello changed\", \"second speaker\"]",
            "word text sequence differs: native=[\"hello\", \"world\", \"second\", \"speaker\"] reference=[\"changed\", \"world\", \"second\", \"speaker\"]",
            "char alignment count differs",
            "char alignment content differs",
        ] {
            assert!(
                comparison
                    .differences
                    .iter()
                    .any(|difference| difference.contains(expected)),
                "comparison should report `{expected}`: {:?}",
                comparison.differences
            );
        }
    }

    #[test]
    fn parity_comparison_fails_character_content_mismatches() {
        let mut native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        native.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });
        let mut whisperx = native.clone();
        whisperx.segments[0].chars[0].character = "x".to_string();

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig::default(),
        );

        assert_eq!(comparison.char_count_matches, Some(true));
        assert_eq!(comparison.char_content_matches, Some(false));
        assert!(!comparison.passed);
        assert!(comparison
            .differences
            .iter()
            .any(|difference| { difference.contains("char alignment content differs at char 0") }));
    }

    #[test]
    fn parity_comparison_config_defaults_to_strict() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav"
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");
        let parity_config: ParityConfig =
            serde_json::from_str(r#"{"input":"audio/input.wav"}"#).expect("config should parse");

        assert_eq!(
            fixture_suite.fixtures[0].comparison,
            ParityComparisonConfig::default()
        );
        assert_eq!(parity_config.comparison, ParityComparisonConfig::default());
    }

    #[test]
    fn parity_comparison_config_can_make_timing_report_only() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.segments[0].start_seconds = Some(4.0);
        whisperx.segments[0].words[0].start_seconds = Some(4.0);
        let config = ParityComparisonConfig {
            segment_timing: false,
            word_timing: false,
            ..ParityComparisonConfig::default()
        };

        let comparison =
            compare_transcripts(&native, &whisperx, ParityTolerance::default(), &config);

        assert!(!comparison.segment_timing_matches);
        assert!(!comparison.word_timing_matches);
        assert!(comparison.passed);
        let segment_difference = comparison
            .differences
            .iter()
            .find(|difference| {
                difference.starts_with("report-only: segment timing differs at segment 0")
            })
            .expect("segment timing difference should be reported");
        assert!(segment_difference.contains("native start="));
        assert!(segment_difference.contains("reference start=4.000s"));
        assert!(segment_difference.contains("start_delta="));
        assert!(segment_difference.contains("tolerance=0.100s"));

        let word_difference = comparison
            .differences
            .iter()
            .find(|difference| difference.starts_with("report-only: word timing differs at word 0"))
            .expect("word timing difference should be reported");
        assert!(word_difference.contains("native start="));
        assert!(word_difference.contains("reference start=4.000s"));
        assert!(word_difference.contains("start_delta="));
        assert!(word_difference.contains("tolerance=0.050s"));
    }

    #[test]
    fn parity_comparison_strict_timing_differences_fail_with_numeric_deltas() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.segments[0].start_seconds = Some(4.0);
        whisperx.segments[0].end_seconds = Some(5.0);
        whisperx.segments[0].words[0].start_seconds = Some(4.0);
        whisperx.segments[0].words[0].end_seconds = Some(4.5);

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig::default(),
        );

        assert!(!comparison.segment_timing_matches);
        assert!(!comparison.word_timing_matches);
        assert!(!comparison.passed);
        let segment_difference = comparison
            .differences
            .iter()
            .find(|difference| difference.starts_with("segment timing differs at segment 0"))
            .expect("segment timing difference should be reported");
        assert!(segment_difference.contains("native start="));
        assert!(segment_difference.contains("native end="));
        assert!(segment_difference.contains("reference start=4.000s"));
        assert!(segment_difference.contains("reference end=5.000s"));
        assert!(segment_difference.contains("start_delta="));
        assert!(segment_difference.contains("end_delta="));
        assert!(segment_difference.contains("tolerance=0.100s"));

        let word_difference = comparison
            .differences
            .iter()
            .find(|difference| difference.starts_with("word timing differs at word 0"))
            .expect("word timing difference should be reported");
        assert!(word_difference.contains("native start="));
        assert!(word_difference.contains("native end="));
        assert!(word_difference.contains("reference start=4.000s"));
        assert!(word_difference.contains("reference end=4.500s"));
        assert!(word_difference.contains("start_delta="));
        assert!(word_difference.contains("end_delta="));
        assert!(word_difference.contains("tolerance=0.050s"));
    }

    #[test]
    fn fixture_suite_keeps_report_only_differences_visible() {
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", true, "audio/input.wav")],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |_| {
            let mut report = fixture_parity_report();
            report.comparison.segment_timing_matches = false;
            report.comparison.differences =
                vec!["report-only: segment timing differs at segment 0".to_string()];
            Ok(report)
        })
        .expect("suite should run");

        assert!(report.passed);
        assert!(report.cases[0].passed);
        assert!(report.cases[0]
            .failure_summary
            .iter()
            .any(|difference| difference == "report-only: segment timing differs at segment 0"));
    }

    #[test]
    fn parity_fixture_manifest_accepts_comparison_config() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "comparison": {
                    "segmentTiming": false,
                    "charContent": false
                  }
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        assert!(!fixture_suite.fixtures[0].comparison.segment_timing);
        assert!(!fixture_suite.fixtures[0].comparison.char_content);
        assert!(fixture_suite.fixtures[0].comparison.word_timing);
        assert!(fixture_suite.fixtures[0].comparison.char_count);
    }

    #[test]
    fn parity_fixture_manifest_accepts_expected_target() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "expectedTarget": "whisperx"
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        assert_eq!(
            fixture_suite.fixtures[0].expected_target,
            ExpectedTranscriptTarget::Whisperx
        );
    }

    #[test]
    fn legacy_fixture_expected_target_defaults_to_native() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav"
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        assert_eq!(
            fixture_suite.fixtures[0].expected_target,
            ExpectedTranscriptTarget::Native
        );
    }

    #[test]
    fn compare_with_whisperx_expected_target_uses_whisperx_transcript() {
        let expected = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let whisperx = expected.clone();
        let mut native = expected.clone();
        native.segments[0].text = "native transcript mismatch".to_string();
        native.segments.pop();

        let (expected_segment_count_matches, expected_text_matches) = expected_transcript_matches(
            Some(&expected),
            ExpectedTranscriptTarget::Whisperx,
            &native,
            &whisperx,
        );

        assert_eq!(expected_text_matches, Some(true));
        assert_eq!(expected_segment_count_matches, Some(true));

        let mut report = fixture_parity_report();
        report.expected_target = ExpectedTranscriptTarget::Whisperx;
        report.expected_text_matches = expected_text_matches;
        report.expected_segment_count_matches = expected_segment_count_matches;
        report.comparison.differences =
            vec!["report-only: native transcript differs from WhisperX transcript".to_string()];

        assert!(parity_fixture_case_passed(&report, &[], &[]));
    }

    #[test]
    fn parity_fixture_manifest_accepts_whisperx_diarization_config() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "diarization": {
                    "enabled": true,
                    "modelId": "native-spectral-speaker-baseline"
                  },
                  "whisperxDiarization": {
                    "enabled": true,
                    "modelId": "pyannote/speaker-diarization-community-1",
                    "hfTokenEnv": "HF_TOKEN",
                    "returnSpeakerEmbeddings": true
                  }
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        let fixture = &fixture_suite.fixtures[0];
        assert_eq!(
            fixture.diarization.model_id,
            "native-spectral-speaker-baseline"
        );
        let whisperx_diarization = fixture
            .whisperx_diarization
            .as_ref()
            .expect("whisperx diarization config");
        assert_eq!(
            whisperx_diarization.model_id,
            "pyannote/speaker-diarization-community-1"
        );
        assert_eq!(
            whisperx_diarization.hf_token_env.as_deref(),
            Some("HF_TOKEN")
        );
        assert!(whisperx_diarization.return_speaker_embeddings);
    }

    #[test]
    fn parity_fixture_manifest_without_whisperx_diarization_keeps_shared_behavior() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "diarization": {
                    "enabled": true,
                    "modelId": "legacy-shared-model"
                  }
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        let fixture = &fixture_suite.fixtures[0];
        assert_eq!(fixture.diarization.model_id, "legacy-shared-model");
        assert!(fixture.whisperx_diarization.is_none());
    }

    #[test]
    fn diagnostic_comparison_reports_provider_specific_entries() {
        let differences = compare_diagnostics(
            &["shared".to_string(), "native-only".to_string()],
            &["shared".to_string(), "whisperx-only".to_string()],
        );

        assert_eq!(
            differences,
            vec![
                "native diagnostic only: native-only".to_string(),
                "whisperx diagnostic only: whisperx-only".to_string()
            ]
        );
    }

    #[test]
    fn output_comparison_reports_exact_json_and_missing_outputs() {
        let temp = tempfile::tempdir().expect("tempdir");
        let expected_txt = temp.path().join("expected.txt");
        let actual_txt = temp.path().join("actual.txt");
        let expected_json = temp.path().join("expected.json");
        let actual_json = temp.path().join("actual.json");
        let missing_expected = temp.path().join("missing.srt");
        let actual_srt = temp.path().join("actual.srt");
        fs::write(&expected_txt, "hello\n").expect("expected txt");
        fs::write(&actual_txt, "hello changed\n").expect("actual txt");
        fs::write(&expected_json, "{\n  \"a\": 1\n}\n").expect("expected json");
        fs::write(&actual_json, "{\"a\":1}").expect("actual json");
        fs::write(&actual_srt, "1\n").expect("actual srt");

        let actual_outputs = vec![
            OutputFile {
                format: OutputFormat::Txt,
                path: actual_txt,
            },
            OutputFile {
                format: OutputFormat::Json,
                path: actual_json,
            },
            OutputFile {
                format: OutputFormat::Srt,
                path: actual_srt,
            },
        ];
        let comparisons = compare_expected_outputs(
            &actual_outputs,
            &[
                ExpectedOutputFile {
                    format: OutputFormat::Txt,
                    path: expected_txt,
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                },
                ExpectedOutputFile {
                    format: OutputFormat::Json,
                    path: expected_json,
                    comparison: OutputComparisonMode::JsonSemantic,
                    gating: true,
                },
                ExpectedOutputFile {
                    format: OutputFormat::Vtt,
                    path: temp.path().join("expected.vtt"),
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                },
                ExpectedOutputFile {
                    format: OutputFormat::Srt,
                    path: missing_expected,
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                },
            ],
        )
        .expect("comparison should run");

        assert!(!comparisons[0].passed);
        assert!(comparisons[0]
            .difference
            .as_deref()
            .is_some_and(|difference| difference.contains("line 1 differs")));
        assert!(comparisons[1].passed);
        assert!(!comparisons[2].passed);
        assert!(comparisons[2]
            .difference
            .as_deref()
            .is_some_and(|difference| difference.contains("missing actual")));
        assert!(!comparisons[3].passed);
        assert!(comparisons[3]
            .difference
            .as_deref()
            .is_some_and(|difference| difference.contains("missing expected")));
    }

    #[test]
    fn output_json_semantic_compares_whisperx_transcript_contract() {
        let difference =
            compare_json_output_values(semantic_expected_whisperx_json(), semantic_actual_json());

        assert_eq!(difference, None);
    }

    #[test]
    fn output_json_semantic_fails_changed_word_text() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["word_segments"][1]["word"] = serde_json::json!("planet");

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript word 1 text differs"));
    }

    #[test]
    fn output_json_semantic_fails_word_timing_beyond_tolerance() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["word_segments"][0]["start"] = serde_json::json!(0.200);

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript word 0 start timing differs"));
        assert!(difference.contains("tolerance=0.050s"));
    }

    #[test]
    fn output_json_semantic_fails_segment_timing_beyond_tolerance() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["segments"][0]["end"] = serde_json::json!(1.500);

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript segment 0 end timing differs"));
        assert!(difference.contains("tolerance=0.100s"));
    }

    #[test]
    fn output_json_semantic_fails_char_count_mismatch_when_chars_present() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["segments"][0]["chars"] = serde_json::json!([
            {
                "char": "h",
                "start": 0.002,
                "end": 0.098
            }
        ]);

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript char count differs"));
    }

    #[test]
    fn parity_fixture_fails_failed_output_comparison() {
        let report = fixture_parity_report();
        let failed_outputs = vec![ExpectedOutputComparison {
            format: OutputFormat::Txt,
            comparison: OutputComparisonMode::Exact,
            gating: true,
            expected_path: PathBuf::from("expected.txt"),
            actual_path: Some(PathBuf::from("actual.txt")),
            passed: false,
            difference: Some("line 1 differs".to_string()),
        }];

        assert!(!parity_fixture_case_passed(&report, &[], &failed_outputs));
    }

    #[test]
    fn preflight_resolves_relative_manifest_paths_under_root() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path();
        fs::create_dir_all(root.join("audio")).expect("audio");
        fs::create_dir_all(root.join("models")).expect("models");
        fs::write(root.join("audio/input.wav"), b"audio").expect("input");

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![minimal_fixture("case", true, "audio/input.wav")],
            },
            root.join("fixtures.json"),
            root.to_path_buf(),
            PathBuf::from("/bin/true"),
            root.join("models"),
            false,
            false,
        );

        assert!(!report.cases[0]
            .missing
            .iter()
            .any(|missing| missing.contains("input")));
    }

    #[test]
    fn preflight_reports_missing_input() {
        let temp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(temp.path().join("models")).expect("models");

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![minimal_fixture("case", true, "audio/missing.wav")],
            },
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            false,
            false,
        );

        assert!(!report.cases[0].passed);
        assert!(report.cases[0]
            .missing
            .iter()
            .any(|missing| missing.contains("audio/missing.wav")));
    }

    #[test]
    fn preflight_reports_missing_expected_output_when_required() {
        let temp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(temp.path().join("audio")).expect("audio");
        fs::create_dir_all(temp.path().join("models")).expect("models");
        fs::write(temp.path().join("audio/input.wav"), b"audio").expect("input");
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.expected_outputs.push(ExpectedOutputFile {
            format: OutputFormat::Srt,
            path: PathBuf::from("expected/missing.srt"),
            comparison: OutputComparisonMode::Exact,
            gating: true,
        });

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![fixture],
            },
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            true,
            false,
        );

        assert!(report.cases[0]
            .missing
            .iter()
            .any(|missing| missing.contains("expected/missing.srt")));
    }

    #[test]
    fn preflight_ignores_missing_non_gating_resources_unless_included() {
        let temp = tempfile::tempdir().expect("tempdir");
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", false, "audio/missing.wav")],
        };

        let ignored = run_parity_preflight(
            suite.clone(),
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            false,
            false,
        );
        assert!(ignored.passed);
        assert!(ignored.cases[0].missing.is_empty());
        assert!(!ignored.cases[0].warnings.is_empty());

        let included = run_parity_preflight(
            suite,
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            false,
            true,
        );
        assert!(!included.passed);
        assert!(!included.cases[0].missing.is_empty());
    }

    #[test]
    fn preflight_reports_missing_onnx_runtime_for_onnx_diarization() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path();
        fs::create_dir_all(root.join("audio")).expect("audio");
        fs::create_dir_all(root.join("models/diarization")).expect("diarization model bundle");
        fs::create_dir_all(root.join("models")).expect("models");
        fs::write(root.join("audio/input.wav"), b"audio").expect("input");
        for file in [
            "pyannote_diarization_manifest.json",
            "segmentation.onnx",
            "embedding.onnx",
            "plda_transform.json",
            "plda_model.json",
            "clustering.json",
        ] {
            fs::write(root.join("models/diarization").join(file), b"model").expect(file);
        }
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.diarization = DiarizationConfig {
            enabled: true,
            model_bundle: Some(PathBuf::from("models/diarization")),
            ..DiarizationConfig::default()
        };
        let previous_ort = std::env::var_os("ORT_DYLIB_PATH");
        std::env::set_var("ORT_DYLIB_PATH", root.join("missing-onnxruntime.so"));

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![fixture],
            },
            root.join("fixtures.json"),
            root.to_path_buf(),
            PathBuf::from("/bin/true"),
            root.join("models"),
            false,
            false,
        );

        if let Some(previous_ort) = previous_ort {
            std::env::set_var("ORT_DYLIB_PATH", previous_ort);
        } else {
            std::env::remove_var("ORT_DYLIB_PATH");
        }
        assert!(!report.cases[0].passed);
        assert!(report.cases[0]
            .missing
            .iter()
            .any(|missing| missing == "ORT_DYLIB_PATH is not set to an existing file"));
    }

    #[test]
    fn preflight_skips_hf_token_env_when_diarization_is_disabled() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path();
        fs::create_dir_all(root.join("audio")).expect("audio");
        fs::create_dir_all(root.join("models")).expect("models");
        fs::write(root.join("audio/input.wav"), b"audio").expect("input");
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.diarization = DiarizationConfig {
            enabled: false,
            model_id: "pyannote/speaker-diarization-community-1".to_string(),
            hf_token_env: Some("NATIVE_WHISPERX_TEST_MISSING_HF_TOKEN".to_string()),
            ..DiarizationConfig::default()
        };
        std::env::remove_var("NATIVE_WHISPERX_TEST_MISSING_HF_TOKEN");

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![fixture],
            },
            root.join("fixtures.json"),
            root.to_path_buf(),
            PathBuf::from("/bin/true"),
            root.join("models"),
            false,
            false,
        );

        assert!(!report.cases[0]
            .missing
            .iter()
            .any(|missing| { missing.contains("NATIVE_WHISPERX_TEST_MISSING_HF_TOKEN") }));
    }

    #[test]
    fn fixture_suite_records_gating_case_error_and_fails_suite() {
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", true, "audio/input.wav")],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |_| {
            Err(NativeWhisperxError::InvalidConfig(
                "setup failed".to_string(),
            ))
        })
        .expect("suite should not abort");

        assert!(!report.passed);
        assert!(!report.cases[0].passed);
        assert_eq!(
            report.cases[0].error.as_deref(),
            Some("invalid configuration: setup failed")
        );
    }

    #[test]
    fn fixture_suite_passes_separate_whisperx_diarization_config() {
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.diarization = DiarizationConfig {
            enabled: true,
            model_id: "native-spectral-speaker-baseline".to_string(),
            min_speakers: Some(2),
            max_speakers: Some(2),
            ..DiarizationConfig::default()
        };
        fixture.whisperx_diarization = Some(DiarizationConfig {
            enabled: true,
            model_id: "pyannote/speaker-diarization-community-1".to_string(),
            hf_token_env: Some("HF_TOKEN".to_string()),
            return_speaker_embeddings: true,
            min_speakers: Some(2),
            max_speakers: Some(2),
            ..DiarizationConfig::default()
        });
        let suite = ParityFixtureSuite {
            fixtures: vec![fixture],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |config| {
            assert_eq!(
                config.diarization.model_id,
                "native-spectral-speaker-baseline"
            );
            let whisperx_diarization = config
                .whisperx_diarization
                .expect("whisperx diarization config");
            assert_eq!(
                whisperx_diarization.model_id,
                "pyannote/speaker-diarization-community-1"
            );
            assert!(whisperx_diarization.return_speaker_embeddings);
            Ok(fixture_parity_report())
        })
        .expect("suite should run");

        assert!(report.passed);
    }

    #[test]
    fn fixture_suite_records_non_gating_case_error_and_keeps_suite_passed() {
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", false, "audio/input.wav")],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |_| {
            Err(NativeWhisperxError::InvalidConfig(
                "setup failed".to_string(),
            ))
        })
        .expect("suite should not abort");

        assert!(report.passed);
        assert!(!report.cases[0].passed);
        assert!(report.cases[0].error.is_some());
    }

    #[test]
    fn failure_summary_includes_output_diff_and_missing_diagnostics() {
        let report = fixture_parity_report();
        let summary = parity_fixture_failure_summary(
            Some(&report),
            &["asrModelSource=hugging-face-cache".to_string()],
            &[ExpectedOutputComparison {
                format: OutputFormat::Txt,
                comparison: OutputComparisonMode::Exact,
                gating: true,
                expected_path: PathBuf::from("expected.txt"),
                actual_path: Some(PathBuf::from("actual.txt")),
                passed: false,
                difference: Some("line 1 differs: expected \"a\", actual \"b\"".to_string()),
            }],
            None,
        );

        assert!(summary
            .iter()
            .any(|line| line.contains("missing required diagnostic")));
        assert!(summary.iter().any(|line| line.contains("line 1 differs")));
    }

    #[test]
    fn parity_fixture_resolves_relative_paths_against_root() {
        let fixture = resolve_fixture_case_paths(
            ParityFixtureCase {
                name: "case".to_string(),
                gating: true,
                input: PathBuf::from("audio/input.wav"),
                clip_seconds: None,
                timeout_seconds: None,
                expected_json: Some(PathBuf::from("expected/input.json")),
                expected_target: ExpectedTranscriptTarget::Native,
                comparison: ParityComparisonConfig::default(),
                expected_outputs: vec![ExpectedOutputFile {
                    format: OutputFormat::Srt,
                    path: PathBuf::from("expected/input.srt"),
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                }],
                native_asr: AsrConfig {
                    whisper_bundle: Some(PathBuf::from("models/whisper")),
                    model_dir: Some(PathBuf::from("models")),
                    external_whisperx: ExternalWhisperxConfig {
                        command: PathBuf::from("bin/whisperx"),
                        output_dir: Some(PathBuf::from("external-out")),
                        ..ExternalWhisperxConfig::default()
                    },
                    ..AsrConfig::default()
                },
                translation: TranslationConfig {
                    model_bundle: Some(PathBuf::from("models/translation")),
                    model_dir: Some(PathBuf::from("models")),
                    ..TranslationConfig::default()
                },
                vad: VadConfig {
                    model_bundle: Some(PathBuf::from("models/silero")),
                    ..VadConfig::default()
                },
                alignment: AlignmentConfig {
                    model_bundle: Some(PathBuf::from("models/wav2vec2")),
                    model_dir: Some(PathBuf::from("models")),
                    ..AlignmentConfig::default()
                },
                diarization: DiarizationConfig {
                    speaker_embedding_model_bundle: Some(PathBuf::from("models/speakers")),
                    ..DiarizationConfig::default()
                },
                whisperx_diarization: None,
                whisperx: ExternalWhisperxConfig {
                    command: PathBuf::from("bin/whisperx"),
                    output_dir: Some(PathBuf::from("whisperx-out")),
                    ..ExternalWhisperxConfig::default()
                },
                language: Some("en".to_string()),
                output: OutputConfig {
                    output_dir: Some(PathBuf::from("out")),
                    ..OutputConfig::default()
                },
                required_diagnostics: Vec::new(),
            },
            Some(Path::new("/smoke")),
        );

        assert_eq!(fixture.input, PathBuf::from("/smoke/audio/input.wav"));
        assert_eq!(
            fixture.expected_json,
            Some(PathBuf::from("/smoke/expected/input.json"))
        );
        assert_eq!(
            fixture.expected_outputs[0].path,
            PathBuf::from("/smoke/expected/input.srt")
        );
        assert_eq!(
            fixture.native_asr.whisper_bundle,
            Some(PathBuf::from("/smoke/models/whisper"))
        );
        assert_eq!(
            fixture.native_asr.external_whisperx.command,
            PathBuf::from("/smoke/bin/whisperx")
        );
        assert_eq!(
            fixture.translation.model_bundle,
            Some(PathBuf::from("/smoke/models/translation"))
        );
        assert_eq!(
            fixture.translation.model_dir,
            Some(PathBuf::from("/smoke/models"))
        );
        assert_eq!(
            fixture.vad.model_bundle,
            Some(PathBuf::from("/smoke/models/silero"))
        );
        assert_eq!(
            fixture.alignment.model_bundle,
            Some(PathBuf::from("/smoke/models/wav2vec2"))
        );
        assert_eq!(
            fixture.diarization.speaker_embedding_model_bundle,
            Some(PathBuf::from("/smoke/models/speakers"))
        );
        assert_eq!(
            fixture.whisperx.command,
            PathBuf::from("/smoke/bin/whisperx")
        );
        assert_eq!(fixture.output.output_dir, Some(PathBuf::from("/smoke/out")));
    }

    #[test]
    fn parity_fixture_reports_required_diagnostics() {
        let mut report = fixture_parity_report();
        report
            .native_report
            .response
            .diagnostics
            .push("asrModelSource=hugging-face-cache".to_string());

        let missing = missing_required_diagnostics(
            &report,
            &[
                "asrModelSource=hugging-face-cache".to_string(),
                "asrModelId=openai/whisper-tiny.en".to_string(),
            ],
        );

        assert_eq!(
            missing,
            vec!["asrModelId=openai/whisper-tiny.en".to_string()]
        );
        assert!(!parity_fixture_case_passed(&report, &missing, &[]));
    }

    #[test]
    fn parity_fixture_passes_when_comparison_expected_and_diagnostics_pass() {
        let mut report = fixture_parity_report();
        report.expected_text_matches = Some(true);
        report.expected_segment_count_matches = Some(true);
        report
            .native_report
            .response
            .diagnostics
            .push("asrModelSource=hugging-face-cache".to_string());

        let missing = missing_required_diagnostics(
            &report,
            &["asrModelSource=hugging-face-cache".to_string()],
        );

        assert!(missing.is_empty());
        assert!(parity_fixture_case_passed(&report, &missing, &[]));
    }

    #[test]
    fn parity_fixture_fails_expected_json_mismatches() {
        let mut report = fixture_parity_report();
        report.expected_text_matches = Some(false);
        report.expected_segment_count_matches = Some(true);

        assert!(!parity_fixture_case_passed(&report, &[], &[]));

        report.expected_text_matches = Some(true);
        report.expected_segment_count_matches = Some(false);

        assert!(!parity_fixture_case_passed(&report, &[], &[]));
    }

    #[test]
    fn parity_fixture_fails_failed_comparison() {
        let mut report = fixture_parity_report();
        report.comparison.passed = false;

        assert!(!parity_fixture_case_passed(&report, &[], &[]));
    }

    #[test]
    fn vad_segment_comparison_fails_count_mismatch() {
        let transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let config = ParityComparisonConfig::default();
        let mut comparison = compare_transcripts(
            &transcript,
            &transcript,
            ParityTolerance::default(),
            &config,
        );
        let native = vec![
            SpeechActivitySegment::new(0.0, 1.0, 0.9).unwrap(),
            SpeechActivitySegment::new(2.0, 3.0, 0.8).unwrap(),
        ];
        let whisperx = vec![SpeechActivitySegment::new(0.0, 1.0, 0.7).unwrap()];

        compare_vad_segments(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &config,
            &mut comparison,
        );

        assert_eq!(comparison.vad_segment_count_matches, Some(false));
        assert_eq!(comparison.vad_segment_timing_matches, Some(false));
        assert!(!comparison.passed);
        assert!(comparison
            .differences
            .iter()
            .any(|difference| difference.contains("VAD segment count differs")));
    }

    #[test]
    fn vad_segment_timing_can_be_report_only() {
        let transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let config = ParityComparisonConfig {
            vad_segment_timing: false,
            ..ParityComparisonConfig::default()
        };
        let mut comparison = compare_transcripts(
            &transcript,
            &transcript,
            ParityTolerance::default(),
            &config,
        );
        let native = vec![SpeechActivitySegment::new(0.0, 1.0, 0.9).unwrap()];
        let whisperx = vec![SpeechActivitySegment::new(0.25, 1.0, 0.7).unwrap()];

        compare_vad_segments(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &config,
            &mut comparison,
        );

        assert_eq!(comparison.vad_segment_count_matches, Some(true));
        assert_eq!(comparison.vad_segment_timing_matches, Some(false));
        assert!(comparison.passed);
        assert!(comparison.differences.iter().any(|difference| {
            difference.starts_with("report-only: VAD segment timing differs")
        }));
    }

    fn minimal_fixture(name: &str, gating: bool, input: &str) -> ParityFixtureCase {
        ParityFixtureCase {
            name: name.to_string(),
            gating,
            input: PathBuf::from(input),
            clip_seconds: None,
            timeout_seconds: None,
            expected_json: None,
            expected_target: ExpectedTranscriptTarget::Native,
            comparison: ParityComparisonConfig::default(),
            expected_outputs: Vec::new(),
            native_asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            whisperx_diarization: None,
            whisperx: ExternalWhisperxConfig::default(),
            language: None,
            output: OutputConfig::default(),
            required_diagnostics: Vec::new(),
        }
    }

    fn fixture_parity_report() -> ParityReport {
        let native_report = NativeWhisperxReport {
            response: fixture_response_with_chars(),
            output_files: Vec::new(),
        };
        let whisperx_report = native_report.clone();
        ParityReport {
            native_report,
            whisperx_report,
            expected: None,
            expected_target: ExpectedTranscriptTarget::Native,
            comparison: ParityComparison {
                text_matches: true,
                language_matches: Some(true),
                segment_text_matches: Some(true),
                word_text_matches: Some(true),
                char_count_matches: Some(true),
                char_content_matches: Some(true),
                segment_count_matches: true,
                word_count_matches: true,
                segment_timing_matches: true,
                word_timing_matches: true,
                speaker_turns_match: true,
                vad_segment_count_matches: None,
                vad_segment_timing_matches: None,
                confidence_compared: true,
                passed: true,
                tolerance: ParityTolerance::default(),
                differences: Vec::new(),
                diagnostic_differences: Vec::new(),
            },
            expected_segment_count_matches: None,
            expected_text_matches: None,
        }
    }

    fn fixture_response_with_chars() -> TranscriptionPipelineResponse {
        let mut transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        transcript.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });
        TranscriptionPipelineResponse {
            accepted: true,
            operation: "audio.transcription.transcribe".to_string(),
            provider: "fixture".to_string(),
            model_id: "fixture".to_string(),
            transcript,
            vad_segments: Vec::new(),
            alignment: None,
            diarization: None,
            artifacts: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    fn correction_transcript() -> TranscriptionContract {
        let mut first = text_transcripts::TranscriptSegmentContract::new(0, "hello");
        first.start_seconds = Some(0.0);
        first.end_seconds = Some(1.0);
        first.speaker = Some("speaker_0".to_string());
        first.words.push(text_transcripts::TranscriptWordContract {
            text: "hello".to_string(),
            start_seconds: Some(0.1),
            end_seconds: Some(0.9),
            confidence: Some(0.9),
            speaker: Some("speaker_0".to_string()),
            attributes: Default::default(),
        });
        let mut second = text_transcripts::TranscriptSegmentContract::new(1, "world");
        second.start_seconds = Some(1.0);
        second.end_seconds = Some(2.0);
        second.speaker = Some("speaker_1".to_string());
        TranscriptionContract::new(vec![first, second])
    }

    fn correction_samples() -> Vec<f32> {
        let sample_rate = 16_000;
        let mut samples = vec![0.0_f32; sample_rate as usize * 2];
        sine_into(
            &mut samples[0..sample_rate as usize],
            sample_rate,
            0.0,
            220.0,
        );
        sine_into(
            &mut samples[sample_rate as usize..sample_rate as usize * 2],
            sample_rate,
            1.0,
            440.0,
        );
        samples
    }

    #[cfg(feature = "diarization")]
    fn two_speaker_loaded_audio() -> LoadedAudio {
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

    fn sine_into(samples: &mut [f32], sample_rate: u32, start_seconds: f32, freq_hz: f32) {
        for (offset, sample) in samples.iter_mut().enumerate() {
            let t = start_seconds + offset as f32 / sample_rate as f32;
            *sample = (2.0 * std::f32::consts::PI * freq_hz * t).sin() * 0.5;
        }
    }

    #[cfg(feature = "diarization")]
    fn speaker_library_matching_first_span(
        audio: &LoadedAudio,
    ) -> std::result::Result<SpeakerLibrary, Box<dyn std::error::Error>> {
        use audio_analysis_speakers::{
            SpeakerEmbeddingExtractor, SpeakerId, SpeakerLabel, SpeakerProfile,
        };

        let start = (0.20 * audio.sample_rate as f32) as usize;
        let end = (0.50 * audio.sample_rate as f32) as usize;
        let speaker_audio = SpeakerAudio::mono(&audio.samples[start..end], audio.sample_rate)?;
        let mut embedder = SpectralSpeakerEmbedder::default();
        let embedding = embedder.embed_speaker(&speaker_audio)?;
        let mut library = SpeakerLibrary::new();
        library.add_profile(
            SpeakerProfile::new(
                SpeakerId::new("known-speaker")?,
                SpeakerLabel::new("Known Speaker")?,
            )
            .with_embedding(embedding)?,
        )?;
        Ok(library)
    }

    #[cfg(feature = "diarization")]
    fn timed_transcript(
        words: Vec<(&str, f64, f64)>,
    ) -> std::result::Result<TranscriptionContract, Box<dyn std::error::Error>> {
        let mut segment = text_transcripts::TranscriptSegmentContract::new(
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
                |(text, start_seconds, end_seconds)| text_transcripts::TranscriptWordContract {
                    text: text.to_string(),
                    start_seconds: Some(start_seconds),
                    end_seconds: Some(end_seconds),
                    confidence: None,
                    speaker: None,
                    attributes: Default::default(),
                },
            )
            .collect();
        Ok(TranscriptionContract::from_segments(
            None,
            Some("en".to_string()),
            vec![segment],
        )?)
    }

    fn compare_json_output_values(
        expected: serde_json::Value,
        actual: serde_json::Value,
    ) -> Option<String> {
        let temp = tempfile::tempdir().expect("tempdir");
        let expected_path = temp.path().join("expected.json");
        let actual_path = temp.path().join("actual.json");
        fs::write(
            &expected_path,
            serde_json::to_string(&expected).expect("expected json"),
        )
        .expect("write expected json");
        fs::write(
            &actual_path,
            serde_json::to_string_pretty(&actual).expect("actual json"),
        )
        .expect("write actual json");
        compare_output_json(&expected_path, &actual_path).expect("json comparison")
    }

    fn semantic_expected_whisperx_json() -> serde_json::Value {
        serde_json::json!({
            "language": "en",
            "segments": [
                {
                    "start": 0.0,
                    "end": 1.2,
                    "text": " hello world",
                    "avg_logprob": -0.1,
                    "no_speech_prob": 0.01,
                    "words": [
                        {
                            "word": " hello",
                            "start": 0.0,
                            "end": 0.5,
                            "score": 0.9876
                        },
                        {
                            "word": "world",
                            "start": 0.55,
                            "end": 1.2,
                            "score": 0.902
                        }
                    ],
                    "chars": [
                        {
                            "char": "h",
                            "start": 0.0,
                            "end": 0.1
                        },
                        {
                            "char": "i",
                            "start": null,
                            "end": null
                        }
                    ]
                }
            ],
            "word_segments": [
                {
                    "word": " hello",
                    "start": 0.0,
                    "end": 0.5,
                    "score": 0.9876
                },
                {
                    "word": "world",
                    "start": 0.55,
                    "end": 1.2,
                    "score": 0.902
                }
            ]
        })
    }

    fn semantic_actual_json() -> serde_json::Value {
        serde_json::json!({
            "text": "hello world",
            "source": "sample.wav",
            "language": "en",
            "segments": [
                {
                    "id": 0,
                    "start": 0.004,
                    "end": 1.196,
                    "text": "hello world",
                    "score": 0.95,
                    "words": [
                        {
                            "word": "hello",
                            "start": 0.002,
                            "end": 0.501,
                            "score": 0.987
                        },
                        {
                            "word": " world",
                            "start": 0.552,
                            "end": 1.198,
                            "score": 0.9025
                        }
                    ],
                    "chars": [
                        {
                            "char": "h",
                            "start": 0.002,
                            "end": 0.098
                        },
                        {
                            "char": "i"
                        }
                    ]
                }
            ],
            "word_segments": [
                {
                    "word": "hello",
                    "start": 0.002,
                    "end": 0.501,
                    "score": 0.987
                },
                {
                    "word": " world",
                    "start": 0.552,
                    "end": 1.198,
                    "score": 0.9025
                }
            ]
        })
    }

    fn contains_pair(args: &[String], flag: &str, value: &str) -> bool {
        args.windows(2)
            .any(|pair| pair[0] == flag && pair[1] == value)
    }
}

use std::path::PathBuf;

use audio_analysis_transcription::{AlignmentInterpolationMethod, TranscriptionPipelineResponse};
use serde::{Deserialize, Serialize};
use text_transcripts::TranscriptionContract;

use crate::speaker_directory::{SpeakerCorrectionRange, SpeakerDirectorySelection};

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

fn default_whisper_model_id() -> String {
    "small".to_string()
}

fn default_external_whisperx_model() -> String {
    "small".to_string()
}

pub(crate) fn default_whisperx_command() -> PathBuf {
    PathBuf::from("whisperx")
}

fn default_alignment_model_id() -> String {
    "facebook/wav2vec2-base-960h".to_string()
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

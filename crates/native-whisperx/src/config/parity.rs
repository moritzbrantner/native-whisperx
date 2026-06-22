use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use text_transcripts::TranscriptionContract;

use super::defaults::{default_gating, default_true};
use super::{
    AlignmentConfig, AsrConfig, DiarizationConfig, ExternalWhisperxConfig, NativeWhisperxReport,
    OutputConfig, OutputFormat, TranslationConfig, VadConfig,
};

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
    #[serde(default)]
    pub fixtures: Vec<ParityFixtureCase>,
    #[serde(default)]
    pub multi_input_fixtures: Vec<ParityMultiInputFixtureCase>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityMultiInputFixtureCase {
    pub name: String,
    #[serde(default = "default_gating")]
    pub gating: bool,
    pub inputs: Vec<PathBuf>,
    #[serde(default)]
    pub clip_seconds_per_input: Option<f64>,
    #[serde(default)]
    pub timeout_seconds: Option<u64>,
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

//! Output format configuration for WhisperX JSON, Native JSON, and transcript files.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::defaults::{default_output_formats, default_pretty_json};

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

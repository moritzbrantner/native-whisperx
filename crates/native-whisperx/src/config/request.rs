//! Top-level workflow request configuration and input source selection.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{
    AlignmentConfig, AsrConfig, DiarizationConfig, OutputConfig, TranslationConfig, VadConfig,
};

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
/// The stable finite-input representation used by legacy workflow configuration.
///
/// This enum intentionally remains limited to `Path` and `Samples`. Explicit
/// audio-stream selection uses [`SelectedMediaInput`] plus the selected-media
/// workflow entrypoints because adding an enum variant would break exhaustive
/// matches in downstream crates and change serialized configuration schemas.
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

/// An explicit zero-based audio-stream ordinal for a finite path input.
///
/// The media path remains in [`NativeWhisperxConfig::input`] as the existing
/// [`InputSource::Path`] variant. Pass this additive type to the selected-media
/// workflow entrypoints without changing exhaustive matches on [`InputSource`].
///
/// `audio_track` counts audio streams only, matching FFmpeg `0:a:N` semantics.
/// It is not a global container stream index, so a video stream at global index
/// zero is never selected by `audio_track = 0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedMediaInput {
    /// The zero-based ordinal among audio streams, not a global stream index.
    pub audio_track: usize,
}

impl SelectedMediaInput {
    /// Selects the audio stream at `audio_track` for a selected-media workflow.
    pub fn new(audio_track: usize) -> Self {
        Self { audio_track }
    }
}

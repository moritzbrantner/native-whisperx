//! Error type shared by library workflow, parity, output, and speaker operations.

#[derive(Debug, thiserror::Error)]
pub enum NativeWhisperxError {
    #[error("{capability} is unavailable because the `{feature}` feature is disabled; rebuild with `--features {feature}`")]
    FeatureDisabled {
        feature: &'static str,
        capability: &'static str,
    },
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("transcription failed: {0}")]
    Transcription(String),
    #[error("transcript import failed: {0}")]
    Import(String),
    #[cfg(feature = "translation")]
    #[error("legacy PyTorch weight loading failed: {0}")]
    LegacyPytorchWeights(#[from] crate::translation::LegacyPytorchError),
    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),
}

/// Error returned only by the additive selected-media workflow entrypoints.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SelectedMediaError {
    /// A non-selection Workflow Composition error.
    #[error(transparent)]
    Workflow(#[from] NativeWhisperxError),
    #[cfg(feature = "media-decode")]
    #[error(
        "native selected-media decode failed for `{path}` before model loading: invalid zero-based audio track {audio_track}: {reason:?}; available streams: {available_streams_summary}"
    )]
    /// Typed selected-media stream-selection failure, including the probed inventory.
    StreamSelection {
        /// The media path whose stream inventory was probed.
        path: std::path::PathBuf,
        /// The requested zero-based audio-stream ordinal.
        audio_track: usize,
        /// Why the requested audio ordinal could not be selected.
        reason: video_analysis_ffmpeg::AudioStreamSelectionErrorReason,
        /// Every container stream reported by FFprobe.
        available_streams: video_analysis_ffmpeg::MediaStreamInventory,
        #[doc(hidden)]
        available_streams_summary: String,
    },
}

impl SelectedMediaError {
    pub(crate) fn into_native(self) -> NativeWhisperxError {
        match self {
            Self::Workflow(error) => error,
            #[cfg(feature = "media-decode")]
            error @ Self::StreamSelection { .. } => {
                NativeWhisperxError::Transcription(error.to_string())
            }
        }
    }
}

pub(crate) fn ensure_whisperx_compat_enabled(
    capability: &'static str,
) -> Result<(), NativeWhisperxError> {
    if cfg!(feature = "whisperx-compat") {
        Ok(())
    } else {
        Err(NativeWhisperxError::FeatureDisabled {
            feature: "whisperx-compat",
            capability,
        })
    }
}

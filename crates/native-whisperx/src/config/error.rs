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
    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),
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

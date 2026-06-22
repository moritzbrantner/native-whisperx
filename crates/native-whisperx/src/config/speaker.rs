//! Speaker correction request and report types for Speaker Directory workflows.

use std::path::PathBuf;

use serde::Serialize;
use text_transcripts::TranscriptionContract;

use crate::speaker_directory::{SpeakerCorrectionRange, SpeakerDirectorySelection};

use super::{InputSource, OutputConfig};

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

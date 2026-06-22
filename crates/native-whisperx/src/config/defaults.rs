use std::path::PathBuf;

use super::OutputFormat;

pub(super) fn default_whisper_model_id() -> String {
    "small".to_string()
}

pub(super) fn default_external_whisperx_model() -> String {
    "small".to_string()
}

pub(crate) fn default_whisperx_command() -> PathBuf {
    PathBuf::from("whisperx")
}

pub(super) fn default_alignment_model_id() -> String {
    "facebook/wav2vec2-base-960h".to_string()
}

pub(super) fn default_batch_chunks() -> bool {
    true
}

pub(super) fn default_max_batch_size() -> Option<usize> {
    Some(4)
}

pub(super) fn default_gating() -> bool {
    true
}

pub(super) fn default_true() -> bool {
    true
}

pub(super) fn default_vad_enabled() -> bool {
    true
}

pub(super) fn default_vad_rms_threshold() -> f32 {
    0.01
}

pub(super) fn default_vad_frame_seconds() -> f64 {
    0.03
}

pub(super) fn default_vad_hop_seconds() -> f64 {
    0.01
}

pub(super) fn default_vad_min_speech_seconds() -> f64 {
    0.08
}

pub(super) fn default_vad_padding_seconds() -> f64 {
    0.02
}

pub(super) fn default_vad_merge_gap_seconds() -> f64 {
    0.05
}

pub(super) fn default_vad_max_chunk_seconds() -> f64 {
    30.0
}

pub(super) fn default_output_formats() -> Vec<OutputFormat> {
    vec![OutputFormat::Json]
}

pub(super) fn default_pretty_json() -> bool {
    true
}

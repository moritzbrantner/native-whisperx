//! Shared CLI argument enums and transcribe option parsing.

use std::path::PathBuf;

use clap::{ArgAction, Args, ValueEnum};
use native_whisperx::{AssignmentPolicy, SegmentResolution, VadMethod};

use crate::SpeakerDirectoryArgs;

#[derive(Debug, Args)]
pub(crate) struct LiveTranscribeArgs {
    pub(crate) source: String,
    #[arg(long, default_value = "small")]
    pub(crate) model: String,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    pub(crate) model_cache_only: bool,
    #[arg(long)]
    pub(crate) language: Option<String>,
    #[arg(long = "ffmpeg-bin", default_value = "ffmpeg")]
    pub(crate) ffmpeg_bin: String,
    #[arg(long = "ffmpeg-input-option", action = ArgAction::Append, allow_hyphen_values = true)]
    pub(crate) ffmpeg_input_options: Vec<String>,
    #[arg(long = "ffmpeg-output-option", action = ArgAction::Append, allow_hyphen_values = true)]
    pub(crate) ffmpeg_output_options: Vec<String>,
    #[arg(long = "window-seconds", default_value_t = 5.0)]
    pub(crate) window_seconds: f64,
    #[arg(long = "hop-seconds", default_value_t = 2.5)]
    pub(crate) hop_seconds: f64,
    #[arg(long = "finalize-lag-seconds", default_value_t = 5.0)]
    pub(crate) finalize_lag_seconds: f64,
    #[arg(long = "max-buffer-lag-seconds", default_value_t = 30.0)]
    pub(crate) max_buffer_lag_seconds: f64,
    #[arg(long = "print-ffmpeg-plan", hide = true)]
    pub(crate) print_ffmpeg_plan: bool,
    #[arg(long = "__fake-live-asr-text", hide = true)]
    pub(crate) fake_live_asr_text: Option<String>,
}

#[derive(Debug, Args)]
pub(crate) struct TranscribeArgs {
    #[arg(required = true)]
    pub(crate) input: Vec<PathBuf>,
    #[arg(long, value_enum, default_value_t = CliProvider::Native)]
    pub(crate) provider: CliProvider,
    #[arg(long, visible_alias = "whisper_bundle")]
    pub(crate) whisper_bundle: Option<PathBuf>,
    #[arg(long, default_value = "small")]
    pub(crate) model: String,
    #[arg(long, value_enum, default_value_t = CliTask::Transcribe)]
    pub(crate) task: CliTask,
    #[arg(long)]
    pub(crate) language: Option<String>,
    #[arg(long, value_enum, default_value_t = CliDevicePreference::Auto)]
    pub(crate) device: CliDevicePreference,
    #[arg(long, visible_alias = "device_index")]
    pub(crate) device_index: Option<String>,
    #[arg(long, visible_alias = "batch_size")]
    pub(crate) batch_size: Option<usize>,
    #[arg(long, visible_alias = "compute_type")]
    pub(crate) compute_type: Option<String>,
    #[arg(long, num_args = 0..=1, default_missing_value = "true")]
    pub(crate) verbose: Option<String>,
    #[arg(long = "log-level", visible_alias = "log_level")]
    pub(crate) log_level: Option<String>,
    #[arg(long = "print-progress", visible_alias = "print_progress", action = ArgAction::SetTrue)]
    pub(crate) print_progress: bool,
    #[arg(long = "no-align", visible_alias = "no_align")]
    pub(crate) no_align: bool,
    #[arg(long, visible_alias = "alignment_bundle")]
    pub(crate) alignment_bundle: Option<PathBuf>,
    #[arg(
        long = "align-model",
        visible_alias = "align_model",
        default_value = "facebook/wav2vec2-base-960h"
    )]
    pub(crate) alignment_model: String,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    pub(crate) model_cache_only: bool,
    #[arg(long = "translation-model", visible_alias = "translation_model")]
    pub(crate) translation_model: Option<String>,
    #[arg(long = "translation-bundle", visible_alias = "translation_bundle")]
    pub(crate) translation_bundle: Option<PathBuf>,
    #[arg(
        long = "translation-source-language",
        visible_alias = "translation_source_language"
    )]
    pub(crate) translation_source_language: Option<String>,
    #[arg(
        long = "translation-target-language",
        visible_alias = "translation_target_language"
    )]
    pub(crate) translation_target_language: Option<String>,
    #[arg(
        long = "translation-max-new-tokens",
        visible_alias = "translation_max_new_tokens",
        default_value_t = 256
    )]
    pub(crate) translation_max_new_tokens: usize,
    #[arg(long = "interpolate-method", visible_alias = "interpolate_method", value_enum, default_value_t = CliAlignmentInterpolationMethod::Nearest)]
    pub(crate) interpolate_method: CliAlignmentInterpolationMethod,
    #[arg(
        long = "return-char-alignments",
        visible_alias = "return_char_alignments"
    )]
    pub(crate) return_char_alignments: bool,
    #[arg(long, visible_alias = "speaker_embedding_bundle")]
    pub(crate) speaker_embedding_bundle: Option<PathBuf>,
    #[arg(long, visible_alias = "speaker_embedding_model_file")]
    pub(crate) speaker_embedding_model_file: Option<String>,
    #[arg(long, visible_alias = "speaker_embedding_dim")]
    pub(crate) speaker_embedding_dim: Option<usize>,
    #[arg(long, visible_alias = "speaker_embedding_sample_rate")]
    pub(crate) speaker_embedding_sample_rate: Option<u32>,
    #[arg(long, action = ArgAction::SetTrue)]
    pub(crate) diarize: bool,
    #[arg(long, visible_alias = "diarize_model")]
    pub(crate) diarize_model: Option<String>,
    #[arg(
        long = "diarization-model-bundle",
        visible_alias = "diarization_model_bundle"
    )]
    pub(crate) diarization_model_bundle: Option<PathBuf>,
    #[arg(
        long = "diarization-manifest-file",
        visible_alias = "diarization_manifest_file"
    )]
    pub(crate) diarization_manifest_file: Option<String>,
    #[arg(
        long = "diarization-segmentation-model-file",
        visible_alias = "diarization_segmentation_model_file"
    )]
    pub(crate) diarization_segmentation_model_file: Option<String>,
    #[arg(
        long = "diarization-embedding-model-file",
        visible_alias = "diarization_embedding_model_file"
    )]
    pub(crate) diarization_embedding_model_file: Option<String>,
    #[arg(
        long = "diarization-plda-transform-file",
        visible_alias = "diarization_plda_transform_file"
    )]
    pub(crate) diarization_plda_transform_file: Option<String>,
    #[arg(
        long = "diarization-plda-model-file",
        visible_alias = "diarization_plda_model_file"
    )]
    pub(crate) diarization_plda_model_file: Option<String>,
    #[arg(
        long = "diarization-clustering-config-file",
        visible_alias = "diarization_clustering_config_file"
    )]
    pub(crate) diarization_clustering_config_file: Option<String>,
    #[arg(long, visible_alias = "speaker_embeddings", action = ArgAction::SetTrue)]
    pub(crate) speaker_embeddings: bool,
    #[arg(long, visible_alias = "hf_token")]
    pub(crate) hf_token: Option<String>,
    #[arg(long, visible_alias = "min_speakers")]
    pub(crate) min_speakers: Option<usize>,
    #[arg(long, visible_alias = "max_speakers")]
    pub(crate) max_speakers: Option<usize>,
    #[arg(
        long = "speaker-assignment-policy",
        visible_alias = "speaker_assignment_policy",
        value_enum,
        default_value_t = CliAssignmentPolicy::Majority
    )]
    pub(crate) speaker_assignment_policy: CliAssignmentPolicy,
    #[command(flatten)]
    pub(crate) speaker_directory: SpeakerDirectoryArgs,
    #[arg(long = "no-speaker-library", visible_alias = "no_speaker_library", action = ArgAction::SetTrue)]
    pub(crate) no_speaker_library: bool,
    #[arg(long = "no-speaker-store", visible_alias = "no_speaker_store", action = ArgAction::SetTrue)]
    pub(crate) no_speaker_store: bool,
    #[arg(long = "no-save-draft-speakers", visible_alias = "no_save_draft_speakers", action = ArgAction::SetTrue)]
    pub(crate) no_save_draft_speakers: bool,
    #[arg(long = "no-use-draft-speakers", visible_alias = "no_use_draft_speakers", action = ArgAction::SetTrue)]
    pub(crate) no_use_draft_speakers: bool,
    #[arg(long, short = 'o', visible_alias = "output_dir")]
    pub(crate) output_dir: Option<PathBuf>,
    #[arg(long)]
    pub(crate) basename: Option<String>,
    #[arg(
        long = "format",
        short = 'f',
        alias = "output-format",
        visible_alias = "output_format",
        value_enum,
        default_values_t = [CliOutputFormat::Json]
    )]
    pub(crate) formats: Vec<CliOutputFormat>,
    #[arg(long, visible_alias = "vad_method", value_enum, default_value_t = CliVadMethod::Energy)]
    pub(crate) vad_method: CliVadMethod,
    #[arg(long, visible_alias = "vad_onset")]
    pub(crate) vad_onset: Option<f32>,
    #[arg(long, visible_alias = "vad_offset")]
    pub(crate) vad_offset: Option<f32>,
    #[arg(long, visible_alias = "chunk_size")]
    pub(crate) chunk_size: Option<f64>,
    #[arg(long = "vad-model-bundle", visible_alias = "vad_model_bundle")]
    pub(crate) vad_model_bundle: Option<PathBuf>,
    #[arg(long = "vad-model-file", visible_alias = "vad_model_file")]
    pub(crate) vad_model_file: Option<String>,
    #[arg(long = "vad-input-name", visible_alias = "vad_input_name")]
    pub(crate) vad_input_name: Option<String>,
    #[arg(long = "vad-output-name", visible_alias = "vad_output_name")]
    pub(crate) vad_output_name: Option<String>,
    #[arg(long, value_delimiter = ',')]
    pub(crate) temperature: Vec<f32>,
    #[arg(long, visible_alias = "best_of")]
    pub(crate) best_of: Option<usize>,
    #[arg(long, visible_alias = "beam_size")]
    pub(crate) beam_size: Option<usize>,
    #[arg(long)]
    pub(crate) patience: Option<f32>,
    #[arg(long, visible_alias = "length_penalty")]
    pub(crate) length_penalty: Option<f32>,
    #[arg(long, visible_alias = "suppress_tokens")]
    pub(crate) suppress_tokens: Option<String>,
    #[arg(long, visible_alias = "suppress_numerals", action = ArgAction::SetTrue)]
    pub(crate) suppress_numerals: bool,
    #[arg(long, visible_alias = "initial_prompt")]
    pub(crate) initial_prompt: Option<String>,
    #[arg(long)]
    pub(crate) hotwords: Option<String>,
    #[arg(long, visible_alias = "condition_on_previous_text")]
    pub(crate) condition_on_previous_text: Option<bool>,
    #[arg(long)]
    pub(crate) fp16: Option<bool>,
    #[arg(long, visible_alias = "compression_ratio_threshold")]
    pub(crate) compression_ratio_threshold: Option<f32>,
    #[arg(long, visible_alias = "logprob_threshold")]
    pub(crate) logprob_threshold: Option<f32>,
    #[arg(long, visible_alias = "no_speech_threshold")]
    pub(crate) no_speech_threshold: Option<f32>,
    #[arg(long)]
    pub(crate) threads: Option<usize>,
    #[arg(long, visible_alias = "max_line_width")]
    pub(crate) max_line_width: Option<usize>,
    #[arg(long, visible_alias = "max_line_count")]
    pub(crate) max_line_count: Option<usize>,
    #[arg(long, visible_alias = "highlight_words", action = ArgAction::SetTrue)]
    pub(crate) highlight_words: bool,
    #[arg(long, visible_alias = "segment_resolution", value_enum, default_value_t = CliSegmentResolution::Sentence)]
    pub(crate) segment_resolution: CliSegmentResolution,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliOutputFormat {
    All,
    Json,
    NativeJson,
    Srt,
    Vtt,
    Txt,
    Tsv,
    Aud,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub(crate) enum CliProvider {
    Native,
    ExternalWhisperx,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub(crate) enum CliTask {
    Transcribe,
    Translate,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliDevicePreference {
    Auto,
    Cpu,
    Cuda,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliAlignmentInterpolationMethod {
    Nearest,
    Linear,
    Ignore,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub(crate) enum CliVadMethod {
    Energy,
    Pyannote,
    Silero,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliAssignmentPolicy {
    Majority,
    NearestStart,
    StrictContained,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliSegmentResolution {
    #[value(alias = "segment")]
    Sentence,
    Chunk,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliSpeakerDirectoryScope {
    Auto,
    Local,
    Global,
}

impl From<CliVadMethod> for VadMethod {
    fn from(value: CliVadMethod) -> Self {
        match value {
            CliVadMethod::Energy => Self::Energy,
            CliVadMethod::Pyannote => Self::Pyannote,
            CliVadMethod::Silero => Self::Silero,
        }
    }
}

impl From<CliAssignmentPolicy> for AssignmentPolicy {
    fn from(value: CliAssignmentPolicy) -> Self {
        match value {
            CliAssignmentPolicy::Majority => Self::Majority,
            CliAssignmentPolicy::NearestStart => Self::NearestStart,
            CliAssignmentPolicy::StrictContained => Self::StrictContained,
        }
    }
}

impl From<CliSegmentResolution> for SegmentResolution {
    fn from(value: CliSegmentResolution) -> Self {
        match value {
            CliSegmentResolution::Sentence => Self::Sentence,
            CliSegmentResolution::Chunk => Self::Chunk,
        }
    }
}

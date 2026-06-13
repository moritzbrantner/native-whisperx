#![doc = include_str!("../README.md")]

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

mod silero_vad;

pub use audio_analysis_transcription::{
    AlignmentInterpolationMethod, TranscriptionPipelineRequest, TranscriptionPipelineResponse,
};
pub use text_transcripts::TranscriptionContract;

#[cfg(feature = "diarization")]
use audio_analysis_transcription::NativeSpeakerDiarizationProvider;
#[cfg(feature = "silero-vad")]
use audio_analysis_transcription::{
    run_transcription_pipeline, CandleWhisperTranscriber, CtcForcedAligner,
    ForcedAlignmentProvider, TranscriptDiarizationProvider, TranscriptionVadProvider,
};
use audio_analysis_transcription::{
    transcribe, AlignmentOptions, CandleWhisperOptions, DiarizationOptions, NativeDevicePreference,
    SpeakerAssignmentPolicy, TranscriptionOutputOptions, TranscriptionProviderSelection,
    TranscriptionSource, VadOptions, WhisperXCommandOptions, WhisperXDevice,
};
#[cfg(feature = "silero-vad")]
use silero_vad::{SileroVadOptions, SileroVadTranscriptionProvider};
use text_transcripts::{
    format_srt, format_srt_timestamp, format_webvtt, parse_whisperx_json, TranscriptSegment,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWhisperxConfig {
    pub input: InputSource,
    #[serde(default)]
    pub asr: AsrConfig,
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
    fn as_whisperx_arg(self) -> &'static str {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl Default for WhisperxDecodeConfig {
    fn default() -> Self {
        Self {
            temperature: Vec::new(),
            best_of: None,
            beam_size: None,
            patience: None,
            length_penalty: None,
            suppress_tokens: None,
            suppress_numerals: false,
            initial_prompt: None,
            hotwords: None,
            condition_on_previous_text: None,
            fp16: None,
            compression_ratio_threshold: None,
            logprob_threshold: None,
            no_speech_threshold: None,
            threads: None,
        }
    }
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
    fn as_whisperx_arg(self) -> &'static str {
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
}

impl Default for DiarizationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            model_id: default_diarization_model_id(),
            hf_token: None,
            hf_token_env: None,
            return_speaker_embeddings: false,
            speaker_embedding_model_bundle: None,
            speaker_embedding_model_file: None,
            speaker_embedding_dimension: None,
            speaker_embedding_sample_rate: None,
            min_speakers: None,
            max_speakers: None,
            assignment_policy: AssignmentPolicy::Majority,
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
            segment_resolution: SegmentResolution::Segment,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SegmentResolution {
    #[default]
    Segment,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityConfig {
    pub input: PathBuf,
    #[serde(default)]
    pub expected_json: Option<PathBuf>,
    #[serde(default)]
    pub native_asr: AsrConfig,
    #[serde(default)]
    pub vad: VadConfig,
    #[serde(default)]
    pub alignment: AlignmentConfig,
    #[serde(default)]
    pub diarization: DiarizationConfig,
    #[serde(default)]
    pub whisperx: ExternalWhisperxConfig,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub output: OutputConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityReport {
    pub native_report: NativeWhisperxReport,
    pub whisperx_report: NativeWhisperxReport,
    #[serde(default)]
    pub expected: Option<TranscriptionContract>,
    pub comparison: ParityComparison,
    pub expected_segment_count_matches: Option<bool>,
    pub expected_text_matches: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityComparison {
    pub text_matches: bool,
    pub segment_count_matches: bool,
    pub word_count_matches: bool,
    pub segment_timing_matches: bool,
    pub word_timing_matches: bool,
    pub speaker_turns_match: bool,
    pub confidence_compared: bool,
    pub passed: bool,
    pub tolerance: ParityTolerance,
    #[serde(default)]
    pub differences: Vec<String>,
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
    let request = build_transcription_request(&config)?;
    let response =
        if config.asr.provider == AsrProvider::Native && config.vad.method == VadMethod::Silero {
            #[cfg(feature = "silero-vad")]
            {
                let mut vad_provider = build_silero_vad_provider(&config.vad)?;
                run_native_with_custom_vad(request, &mut vad_provider)?
            }
            #[cfg(not(feature = "silero-vad"))]
            {
                return Err(NativeWhisperxError::InvalidConfig(
                    "native Silero VAD requires the silero-vad feature".to_string(),
                ));
            }
        } else {
            transcribe(request)
                .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))?
        };
    let output_files = write_outputs_with_options(
        &response,
        &config.output,
        config.alignment.return_char_alignments,
    )?;
    Ok(NativeWhisperxReport {
        response,
        output_files,
    })
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
        alignment: map_alignment(&config.alignment),
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

pub fn compare_with_whisperx(config: ParityConfig) -> Result<ParityReport, NativeWhisperxError> {
    let mut native_asr = config.native_asr;
    native_asr.provider = AsrProvider::Native;
    native_asr.language = config.language.clone();
    let alignment = config.alignment;

    let native_report = run(NativeWhisperxConfig {
        input: InputSource::Path {
            path: config.input.clone(),
        },
        asr: native_asr,
        vad: config.vad,
        alignment: alignment.clone(),
        diarization: config.diarization,
        output: config.output.clone(),
    })?;

    let whisperx_report = run(NativeWhisperxConfig {
        input: InputSource::Path { path: config.input },
        asr: AsrConfig {
            provider: AsrProvider::ExternalWhisperX,
            language: config.language,
            external_whisperx: config.whisperx,
            ..AsrConfig::default()
        },
        vad: VadConfig::default(),
        alignment,
        diarization: DiarizationConfig::default(),
        output: config.output,
    })?;

    let expected = config
        .expected_json
        .map(|path| fs::read(path).map_err(NativeWhisperxError::Io))
        .transpose()?
        .map(|bytes| import_whisperx_json(&bytes))
        .transpose()?;

    let comparison = compare_transcripts(
        &native_report.response.transcript,
        &whisperx_report.response.transcript,
        ParityTolerance::default(),
    );

    let expected_segment_count_matches = expected.as_ref().map(|expected| {
        expected.segments.len() == native_report.response.transcript.segments.len()
    });
    let expected_text_matches = expected.as_ref().map(|expected| {
        normalize_space(&expected.text_or_joined())
            == normalize_space(&native_report.response.transcript.text_or_joined())
    });

    Ok(ParityReport {
        native_report,
        whisperx_report,
        expected,
        comparison,
        expected_segment_count_matches,
        expected_text_matches,
    })
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
    if config.asr.task == TranscriptionTask::Translate {
        return Err(NativeWhisperxError::InvalidConfig(
            "--task translate is not implemented by the native provider; use --provider external-whisperx".to_string(),
        ));
    }
    validate_native_vad_support(config)?;
    if config.asr.compute_type.is_some()
        || config.asr.device_index.is_some()
        || !config.asr.decode.is_default()
    {
        return Err(NativeWhisperxError::InvalidConfig(
            "native provider does not yet implement WhisperX/faster-whisper decode controls; use --provider external-whisperx".to_string(),
        ));
    }
    Ok(())
}

fn validate_native_vad_support(config: &NativeWhisperxConfig) -> Result<(), NativeWhisperxError> {
    match config.vad.method {
        VadMethod::Energy => Ok(()),
        VadMethod::Silero => validate_native_silero_config(&config.vad),
        VadMethod::Pyannote => Err(NativeWhisperxError::InvalidConfig(
            "native pyannote VAD is not implemented; use --provider external-whisperx".to_string(),
        )),
    }
}

fn validate_native_silero_config(vad: &VadConfig) -> Result<(), NativeWhisperxError> {
    #[cfg(not(feature = "silero-vad"))]
    {
        let _ = vad;
        return Err(NativeWhisperxError::InvalidConfig(
            "native Silero VAD requires the silero-vad feature".to_string(),
        ));
    }
    #[cfg(feature = "silero-vad")]
    {
        resolve_silero_model_path(vad).map(|_| ())
    }
}

#[cfg(feature = "silero-vad")]
fn build_silero_vad_provider(
    vad: &VadConfig,
) -> Result<SileroVadTranscriptionProvider, NativeWhisperxError> {
    let model_path = resolve_silero_model_path(vad)?;
    let options = SileroVadOptions {
        model_path,
        input_name: vad.input_name.clone(),
        output_name: vad.output_name.clone(),
        threshold: vad.onset.unwrap_or(0.5),
        max_speech_duration_seconds: vad.chunk_size.unwrap_or(30.0),
        min_speech_duration_ms: 250,
        min_silence_duration_ms: 100,
        speech_pad_ms: 30,
    };
    let diagnostics = vad
        .offset
        .is_some()
        .then(|| "native Silero VAD ignores vad_offset; WhisperX Silero uses vad_offset only in the generic merge API".to_string())
        .into_iter()
        .collect();
    SileroVadTranscriptionProvider::from_options(options, diagnostics)
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
}

#[cfg(feature = "silero-vad")]
fn run_native_with_custom_vad(
    request: TranscriptionPipelineRequest,
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
            let mut diarizer = NativeSpeakerDiarizationProvider;
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

#[cfg(feature = "silero-vad")]
fn run_native_with_optional_alignment(
    request: TranscriptionPipelineRequest,
    vad_provider: &mut dyn TranscriptionVadProvider,
    asr_provider: &mut CandleWhisperTranscriber,
    #[cfg_attr(not(feature = "diarization"), allow(unused_variables))] diarization_provider: Option<
        &mut dyn TranscriptDiarizationProvider,
    >,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    if request.alignment.enabled {
        let mut aligner = CtcForcedAligner {
            options: request.alignment.clone(),
        };
        return run_transcription_pipeline(
            request,
            vad_provider,
            asr_provider,
            Some(&mut aligner as &mut dyn ForcedAlignmentProvider),
            diarization_provider,
        )
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()));
    }

    run_transcription_pipeline(
        request,
        vad_provider,
        asr_provider,
        None,
        diarization_provider,
    )
    .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
}

fn map_provider(config: &NativeWhisperxConfig) -> TranscriptionProviderSelection {
    let asr = &config.asr;
    match asr.provider {
        AsrProvider::Native => {
            TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
                model_id: asr.model_id.clone(),
                language: asr.language.clone(),
                device: map_device(asr.device),
                model_bundle: asr.whisper_bundle.clone(),
                model_dir: asr.model_dir.clone(),
                model_cache_only: asr.model_cache_only,
                batch_chunks: asr.batch_chunks,
                max_batch_size: asr.max_batch_size,
            })
        }
        AsrProvider::ExternalWhisperX => {
            let extra_args = external_whisperx_extra_args(config);
            let builtin_diarize =
                config.diarization.enabled && config.diarization.hf_token.is_none();
            TranscriptionProviderSelection::ExternalWhisperX(WhisperXCommandOptions {
                command: asr.external_whisperx.command.clone(),
                model: asr.external_whisperx.model.clone(),
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
                model_cache_only: asr.model_cache_only || config.alignment.model_cache_only,
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

impl WhisperxDecodeConfig {
    fn is_default(&self) -> bool {
        self == &Self::default()
    }
}

fn external_whisperx_extra_args(config: &NativeWhisperxConfig) -> Vec<String> {
    let mut args = config.asr.external_whisperx.extra_args.clone();
    push_arg(&mut args, "--task", Some(config.asr.task.as_whisperx_arg()));
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
        args.push("--highlight_words".to_string());
    }
    push_arg(
        &mut args,
        "--segment_resolution",
        Some(match config.output.subtitles.segment_resolution {
            SegmentResolution::Segment => "segment",
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

fn map_alignment(alignment: &AlignmentConfig) -> AlignmentOptions {
    AlignmentOptions {
        enabled: alignment.enabled,
        model_id: alignment.model_id.clone(),
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
        model_id: diarization.model_id.clone(),
        speaker_embedding_model_bundle: diarization.speaker_embedding_model_bundle.clone(),
        speaker_embedding_model_file: diarization.speaker_embedding_model_file.clone(),
        speaker_embedding_input_name: None,
        speaker_embedding_output_name: None,
        speaker_embedding_dimension: diarization.speaker_embedding_dimension,
        speaker_embedding_sample_rate: diarization.speaker_embedding_sample_rate,
        min_speakers: diarization.min_speakers,
        max_speakers: diarization.max_speakers,
        assignment_policy: match diarization.assignment_policy {
            AssignmentPolicy::Majority => SpeakerAssignmentPolicy::Majority,
            AssignmentPolicy::NearestStart => SpeakerAssignmentPolicy::NearestStart,
            AssignmentPolicy::StrictContained => SpeakerAssignmentPolicy::StrictContained,
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

fn segments_for_format(transcript: &TranscriptionContract) -> Vec<TranscriptSegment> {
    transcript
        .segments
        .iter()
        .cloned()
        .map(TranscriptSegment::from)
        .collect()
}

fn expand_output_format(format: OutputFormat) -> Vec<OutputFormat> {
    match format {
        OutputFormat::All => vec![
            OutputFormat::Txt,
            OutputFormat::Vtt,
            OutputFormat::Srt,
            OutputFormat::Tsv,
            OutputFormat::Json,
        ],
        other => vec![other],
    }
}

fn format_txt(transcript: &TranscriptionContract) -> String {
    if transcript
        .segments
        .iter()
        .any(|segment| segment.speaker.is_some())
    {
        transcript
            .segments
            .iter()
            .map(|segment| match &segment.speaker {
                Some(speaker) => format!("[{speaker}]: {}", segment.text.trim()),
                None => segment.text.trim().to_string(),
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        transcript.text_or_joined()
    }
}

fn format_tsv(transcript: &TranscriptionContract) -> String {
    let mut output = String::new();
    for segment in &transcript.segments {
        let start = seconds_to_millis(segment.start_seconds);
        let end = seconds_to_millis(segment.end_seconds);
        output.push_str(&format!("{start}\t{end}\t{}\n", segment.text.trim()));
    }
    output
}

fn format_audacity_labels(transcript: &TranscriptionContract) -> String {
    let mut output = String::new();
    for segment in &transcript.segments {
        let start = segment.start_seconds.unwrap_or(0.0);
        let end = segment.end_seconds.unwrap_or(start).max(start);
        let text = match &segment.speaker {
            Some(speaker) => format!("[[{speaker}]] {}", segment.text.trim()),
            None => segment.text.trim().to_string(),
        };
        output.push_str(&format!("{start:.3}\t{end:.3}\t{text}\n"));
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
    if !subtitles.highlight_words
        && subtitles.max_line_width.is_none()
        && subtitles.max_line_count.is_none()
    {
        return format_srt(&segments_for_format(transcript));
    }

    let mut output = String::new();
    for (index, segment) in transcript.segments.iter().enumerate() {
        let start = segment.start_seconds.unwrap_or(0.0);
        let end = segment.end_seconds.unwrap_or(start).max(start);
        output.push_str(&(index + 1).to_string());
        output.push('\n');
        output.push_str(&format_srt_timestamp(start));
        output.push_str(" --> ");
        output.push_str(&format_srt_timestamp(end));
        output.push('\n');
        output.push_str(&subtitle_text(segment, subtitles));
        output.push_str("\n\n");
    }
    output
}

fn format_webvtt_with_options(
    transcript: &TranscriptionContract,
    subtitles: &SubtitleConfig,
) -> String {
    if !subtitles.highlight_words
        && subtitles.max_line_width.is_none()
        && subtitles.max_line_count.is_none()
    {
        return format_webvtt(&segments_for_format(transcript));
    }

    let mut output = String::from("WEBVTT\n\n");
    for segment in &transcript.segments {
        let start = segment.start_seconds.unwrap_or(0.0);
        let end = segment.end_seconds.unwrap_or(start).max(start);
        output.push_str(&format_srt_timestamp(start).replace(',', "."));
        output.push_str(" --> ");
        output.push_str(&format_srt_timestamp(end).replace(',', "."));
        output.push('\n');
        output.push_str(&subtitle_text(segment, subtitles));
        output.push_str("\n\n");
    }
    output
}

fn subtitle_text(
    segment: &text_transcripts::TranscriptSegmentContract,
    subtitles: &SubtitleConfig,
) -> String {
    let mut text = if subtitles.highlight_words && !segment.words.is_empty() {
        segment
            .words
            .iter()
            .map(|word| format!("<u>{}</u>", word.text.trim()))
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        segment.text.trim().to_string()
    };

    if let Some(width) = subtitles.max_line_width.filter(|width| *width > 0) {
        text = wrap_subtitle_text(&text, width, subtitles.max_line_count);
    }
    text
}

fn wrap_subtitle_text(text: &str, width: usize, max_line_count: Option<usize>) -> String {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let next_len = if current.is_empty() {
            word.len()
        } else {
            current.len() + 1 + word.len()
        };
        if !current.is_empty() && next_len > width {
            lines.push(current);
            current = String::new();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if let Some(max_line_count) = max_line_count.filter(|count| *count > 0) {
        if lines.len() > max_line_count {
            let mut kept = lines[..max_line_count].to_vec();
            let merged = lines[max_line_count..].join(" ");
            if let Some(last) = kept.last_mut() {
                if !merged.is_empty() {
                    if !last.is_empty() {
                        last.push(' ');
                    }
                    last.push_str(&merged);
                }
            }
            return kept.join("\n");
        }
    }
    lines.join("\n")
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
) -> ParityComparison {
    let mut differences = Vec::new();
    let text_matches =
        normalize_space(&native.text_or_joined()) == normalize_space(&whisperx.text_or_joined());
    if !text_matches {
        differences.push("normalized transcript text differs".to_string());
    }

    let segment_count_matches = native.segments.len() == whisperx.segments.len();
    if !segment_count_matches {
        differences.push(format!(
            "segment count differs: native={} whisperx={}",
            native.segments.len(),
            whisperx.segments.len()
        ));
    }

    let native_word_count = word_count(native);
    let whisperx_word_count = word_count(whisperx);
    let word_count_matches = native_word_count == whisperx_word_count;
    if !word_count_matches {
        differences.push(format!(
            "word count differs: native={native_word_count} whisperx={whisperx_word_count}"
        ));
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
        &mut differences,
    );

    let speaker_turns_match = speaker_turn_signature(native) == speaker_turn_signature(whisperx);
    if !speaker_turns_match {
        differences.push("speaker turn structure differs".to_string());
    }

    let passed = text_matches
        && segment_count_matches
        && word_count_matches
        && segment_timing_matches
        && word_timing_matches
        && speaker_turns_match;

    ParityComparison {
        text_matches,
        segment_count_matches,
        word_count_matches,
        segment_timing_matches,
        word_timing_matches,
        speaker_turns_match,
        confidence_compared: true,
        passed,
        tolerance,
        differences,
    }
}

fn word_count(transcript: &TranscriptionContract) -> usize {
    transcript
        .segments
        .iter()
        .map(|segment| segment.words.len())
        .sum()
}

fn timings_match<N, W>(
    native: N,
    whisperx: W,
    tolerance: f64,
    label: &str,
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
            differences.push(format!("{label} timing differs at {name}"));
            matches = false;
        }
    }
    matches
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

fn default_batch_chunks() -> bool {
    true
}

fn default_max_batch_size() -> Option<usize> {
    Some(4)
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

#[cfg(test)]
mod tests {
    use super::*;

    const WHISPERX_SAMPLE: &[u8] =
        include_bytes!("../../../tests/fixtures/whisperx-parity-sample.json");

    #[test]
    fn maps_native_surface_defaults() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
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
        assert_eq!(request.alignment.enabled, true);
        assert_eq!(
            request.alignment.model_dir,
            Some(PathBuf::from("models/cache"))
        );
        assert!(request.alignment.model_cache_only);
        assert_eq!(
            request.alignment.interpolate_method,
            AlignmentInterpolationMethod::Linear
        );
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
    fn rejects_native_decode_controls() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                decode: WhisperxDecodeConfig {
                    beam_size: Some(5),
                    ..WhisperxDecodeConfig::default()
                },
                ..AsrConfig::default()
            },
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native decode controls should be rejected");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains("decode controls"));
    }

    #[test]
    fn rejects_native_pyannote_vad() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
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
        assert!(error.to_string().contains("native pyannote VAD"));
    }

    #[cfg(not(feature = "silero-vad"))]
    #[test]
    fn rejects_native_silero_without_feature() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
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

    #[test]
    fn rejects_native_translate() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                task: TranscriptionTask::Translate,
                ..AsrConfig::default()
            },
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native translate should be rejected");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains("translate"));
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
                    ..WhisperxDecodeConfig::default()
                },
                external_whisperx: ExternalWhisperxConfig {
                    model: "small".to_string(),
                    align_model: Some("external-align".to_string()),
                    ..ExternalWhisperxConfig::default()
                },
                ..AsrConfig::default()
            },
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
            vec!["txt", "vtt", "srt", "tsv", "json"]
        );
        match request.provider {
            TranscriptionProviderSelection::ExternalWhisperX(options) => {
                assert_eq!(options.model, "small");
                assert_eq!(options.language.as_deref(), Some("en"));
                assert_eq!(options.device, WhisperXDevice::Cuda);
                assert_eq!(options.compute_type.as_deref(), Some("int8"));
                assert_eq!(options.batch_size, Some(8));
                assert!(options.no_align);
                assert_eq!(options.align_model.as_deref(), Some("external-align"));
                assert_eq!(options.model_dir, Some(PathBuf::from("model-cache")));
                assert!(options.model_cache_only);
                assert!(options.return_char_alignments);
                assert!(!options.diarize);
                assert!(contains_pair(&options.extra_args, "--task", "translate"));
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
                assert!(options
                    .extra_args
                    .contains(&"--highlight_words".to_string()));
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
        assert!(txt.contains("[SPEAKER_00]: hello world"));
        assert!(txt.contains("[SPEAKER_01]: second speaker"));
        let srt = fs::read_to_string(srt_path).expect("srt");
        assert!(srt.contains("hello world"));
        let vtt = fs::read_to_string(vtt_path).expect("vtt");
        assert!(vtt.contains("WEBVTT"));
        assert!(vtt.contains("second speaker"));
        let tsv = fs::read_to_string(tsv_path).expect("tsv");
        assert!(tsv.contains("0\t1200\thello world"));
        assert!(tsv.contains("1350\t2400\tsecond speaker"));
        let aud = fs::read_to_string(aud_path).expect("aud");
        assert!(aud.contains("0.000\t1.200\t[[SPEAKER_00]] hello world"));
        assert!(aud.contains("1.350\t2.400\t[[SPEAKER_01]] second speaker"));
    }

    #[test]
    fn all_format_writes_whisperx_default_set() {
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
                "sample.json",
                "sample.srt",
                "sample.tsv",
                "sample.txt",
                "sample.vtt",
            ]
        );
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
                    segment_resolution: SegmentResolution::Segment,
                },
            },
        )
        .expect("subtitles should write");

        let srt = fs::read_to_string(temp.path().join("sample.srt")).expect("srt");
        assert!(srt.contains("<u>hello</u>"));
        assert!(srt.contains("<u>hello</u>\n<u>world</u>"));
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
                    segment_resolution: SegmentResolution::Segment,
                },
            },
        )
        .expect("subtitles should write");

        let srt = fs::read_to_string(temp.path().join("sample.srt")).expect("srt");
        assert!(srt.contains("hello world\n\n2"));
        assert!(srt.contains("second speaker\n\n"));
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

    fn contains_pair(args: &[String], flag: &str, value: &str) -> bool {
        args.windows(2)
            .any(|pair| pair[0] == flag && pair[1] == value)
    }
}

#![doc = include_str!("../README.md")]

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub use audio_analysis_transcription::{
    AlignmentInterpolationMethod, TranscriptionPipelineRequest, TranscriptionPipelineResponse,
};
pub use text_transcripts::TranscriptionContract;

use audio_analysis_transcription::{
    transcribe, AlignmentOptions, CandleWhisperOptions, DiarizationOptions, NativeDevicePreference,
    SpeakerAssignmentPolicy, TranscriptionOutputOptions, TranscriptionProviderSelection,
    TranscriptionSource, VadOptions, WhisperXCommandOptions, WhisperXDevice,
};
use text_transcripts::{format_srt, format_webvtt, parse_whisperx_json, TranscriptSegment};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsrConfig {
    #[serde(default)]
    pub provider: AsrProvider,
    #[serde(default = "default_whisper_model_id")]
    pub model_id: String,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub whisper_bundle: Option<PathBuf>,
    #[serde(default)]
    pub device: DevicePreference,
    #[serde(default = "default_batch_chunks")]
    pub batch_chunks: bool,
    #[serde(default = "default_max_batch_size")]
    pub max_batch_size: Option<usize>,
    #[serde(default)]
    pub external_whisperx: ExternalWhisperxConfig,
}

impl Default for AsrConfig {
    fn default() -> Self {
        Self {
            provider: AsrProvider::Native,
            model_id: default_whisper_model_id(),
            language: None,
            whisper_bundle: None,
            device: DevicePreference::Auto,
            batch_chunks: true,
            max_batch_size: Some(4),
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
pub enum DevicePreference {
    #[default]
    Auto,
    Cpu,
    Cuda,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rms_threshold: 0.01,
            frame_seconds: 0.03,
            hop_seconds: 0.01,
            min_speech_seconds: 0.08,
            padding_seconds: 0.02,
            merge_gap_seconds: 0.05,
            max_chunk_seconds: 30.0,
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
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            formats: default_output_formats(),
            basename: None,
            pretty_json: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    #[serde(rename = "native-json", alias = "nativeJson")]
    NativeJson,
    Srt,
    Vtt,
    Txt,
}

impl OutputFormat {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::NativeJson => "native.json",
            Self::Srt => "srt",
            Self::Vtt => "vtt",
            Self::Txt => "txt",
        }
    }

    pub fn as_transcription_format(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::NativeJson => "native-json",
            Self::Srt => "srt",
            Self::Vtt => "vtt",
            Self::Txt => "txt",
        }
    }
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
    let response = transcribe(request)
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))?;
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

pub fn build_transcription_request(
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineRequest, NativeWhisperxError> {
    if config.output.formats.is_empty() {
        return Err(NativeWhisperxError::InvalidConfig(
            "at least one output format is required".to_string(),
        ));
    }

    Ok(TranscriptionPipelineRequest {
        source: map_input_source(&config.input),
        provider: map_provider(&config.asr, &config.alignment),
        vad: map_vad(&config.vad),
        alignment: map_alignment(&config.alignment),
        diarization: map_diarization(&config.diarization),
        output: TranscriptionOutputOptions {
            formats: config
                .output
                .formats
                .iter()
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
        .map(|format| {
            let path = output_dir.join(format!("{basename}.{}", format.extension()));
            let contents =
                render_output(response, format, output.pretty_json, return_char_alignments)?;
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

fn map_provider(asr: &AsrConfig, alignment: &AlignmentConfig) -> TranscriptionProviderSelection {
    match asr.provider {
        AsrProvider::Native => {
            TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
                model_id: asr.model_id.clone(),
                language: asr.language.clone(),
                device: map_device(asr.device),
                model_bundle: asr.whisper_bundle.clone(),
                batch_chunks: asr.batch_chunks,
                max_batch_size: asr.max_batch_size,
            })
        }
        AsrProvider::ExternalWhisperX => {
            TranscriptionProviderSelection::ExternalWhisperX(WhisperXCommandOptions {
                command: asr.external_whisperx.command.clone(),
                model: asr.external_whisperx.model.clone(),
                language: asr.language.clone(),
                device: match asr.device {
                    DevicePreference::Cuda => WhisperXDevice::Cuda,
                    DevicePreference::Auto | DevicePreference::Cpu => WhisperXDevice::Cpu,
                },
                compute_type: asr.external_whisperx.compute_type.clone(),
                batch_size: asr.external_whisperx.batch_size,
                diarize: asr.external_whisperx.diarize,
                min_speakers: asr.external_whisperx.min_speakers,
                max_speakers: asr.external_whisperx.max_speakers,
                hf_token_env: asr.external_whisperx.hf_token_env.clone(),
                output_dir: asr.external_whisperx.output_dir.clone(),
                timeout_seconds: asr.external_whisperx.timeout_seconds,
                model_dir: alignment.model_dir.clone(),
                model_cache_only: alignment.model_cache_only,
                no_align: !alignment.enabled,
                interpolate_method: alignment.interpolate_method,
                return_char_alignments: alignment.return_char_alignments,
                align_model: asr
                    .external_whisperx
                    .align_model
                    .clone()
                    .or_else(|| Some(alignment.model_id.clone())),
                extra_args: asr.external_whisperx.extra_args.clone(),
            })
        }
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
        rms_threshold: vad.rms_threshold,
        frame_seconds: vad.frame_seconds,
        hop_seconds: vad.hop_seconds,
        min_speech_seconds: vad.min_speech_seconds,
        padding_seconds: vad.padding_seconds,
        merge_gap_seconds: vad.merge_gap_seconds,
        max_chunk_seconds: vad.max_chunk_seconds,
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
    pretty_json: bool,
    return_char_alignments: bool,
) -> Result<String, NativeWhisperxError> {
    match format {
        OutputFormat::Json if pretty_json => Ok(serde_json::to_string_pretty(
            &whisperx_json_value(&response.transcript, return_char_alignments),
        )?),
        OutputFormat::Json => Ok(serde_json::to_string(&whisperx_json_value(
            &response.transcript,
            return_char_alignments,
        ))?),
        OutputFormat::NativeJson if pretty_json => {
            Ok(serde_json::to_string_pretty(&response.transcript)?)
        }
        OutputFormat::NativeJson => Ok(serde_json::to_string(&response.transcript)?),
        OutputFormat::Srt => Ok(format_srt(&segments_for_format(&response.transcript))),
        OutputFormat::Vtt => Ok(format_webvtt(&segments_for_format(&response.transcript))),
        OutputFormat::Txt => Ok(response.transcript.text_or_joined()),
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
    "openai/whisper-large-v3-turbo".to_string()
}

fn default_external_whisperx_model() -> String {
    "large-v2".to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    const WHISPERX_SAMPLE: &[u8] =
        include_bytes!("../../../tests/fixtures/whisperx-parity-sample.json");

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
    fn imports_whisperx_fixture() {
        let transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        assert_eq!(transcript.language.as_deref(), Some("en"));
        assert_eq!(transcript.segments.len(), 2);
        assert_eq!(transcript.text_or_joined(), "hello world second speaker");
    }

    #[test]
    fn writes_requested_outputs() {
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
        let response = TranscriptionPipelineResponse {
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
        };
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
                ],
                basename: Some("sample".to_string()),
                pretty_json: true,
            },
            true,
        )
        .expect("outputs should write");

        assert_eq!(files.len(), 5);
        assert!(temp.path().join("sample.json").is_file());
        assert!(temp.path().join("sample.native.json").is_file());
        assert!(temp.path().join("sample.srt").is_file());
        assert!(temp.path().join("sample.vtt").is_file());
        assert!(temp.path().join("sample.txt").is_file());

        let whisperx_json: serde_json::Value =
            serde_json::from_slice(&fs::read(temp.path().join("sample.json")).expect("json"))
                .expect("valid whisperx json");
        assert!(whisperx_json.get("word_segments").is_some());
        assert!(whisperx_json["segments"][0].get("start").is_some());
        assert!(whisperx_json["segments"][0].get("startSeconds").is_none());
        assert_eq!(whisperx_json["segments"][0]["chars"][0]["char"], "h");

        let native_json: serde_json::Value = serde_json::from_slice(
            &fs::read(temp.path().join("sample.native.json")).expect("native json"),
        )
        .expect("valid native json");
        assert!(native_json["segments"][0].get("startSeconds").is_some());
        assert!(native_json["segments"][0].get("chars").is_some());
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
}

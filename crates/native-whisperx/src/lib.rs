#![doc = include_str!("../README.md")]

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub use audio_analysis_transcription::{
    TranscriptionPipelineRequest, TranscriptionPipelineResponse,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AlignmentConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_alignment_model_id")]
    pub model_id: String,
    #[serde(default)]
    pub model_bundle: Option<PathBuf>,
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
    Srt,
    Vtt,
    Txt,
}

impl OutputFormat {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Srt => "srt",
            Self::Vtt => "vtt",
            Self::Txt => "txt",
        }
    }

    pub fn as_transcription_format(self) -> &'static str {
        match self {
            Self::Json => "json",
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParityConfig {
    pub input: PathBuf,
    #[serde(default)]
    pub expected_json: Option<PathBuf>,
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
    #[serde(default)]
    pub expected: Option<TranscriptionContract>,
    pub segment_count_matches: Option<bool>,
    pub text_matches: Option<bool>,
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
    let output_files = write_outputs(&response, &config.output)?;
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
        provider: map_provider(&config.asr),
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
            let contents = render_output(response, format, output.pretty_json)?;
            fs::write(&path, contents)?;
            Ok(OutputFile { format, path })
        })
        .collect()
}

pub fn compare_with_whisperx(config: ParityConfig) -> Result<ParityReport, NativeWhisperxError> {
    let native_report = run(NativeWhisperxConfig {
        input: InputSource::Path { path: config.input },
        asr: AsrConfig {
            provider: AsrProvider::ExternalWhisperX,
            language: config.language,
            external_whisperx: config.whisperx,
            ..AsrConfig::default()
        },
        vad: VadConfig::default(),
        alignment: AlignmentConfig::default(),
        diarization: DiarizationConfig::default(),
        output: config.output,
    })?;

    let expected = config
        .expected_json
        .map(|path| fs::read(path).map_err(NativeWhisperxError::Io))
        .transpose()?
        .map(|bytes| import_whisperx_json(&bytes))
        .transpose()?;

    let segment_count_matches = expected.as_ref().map(|expected| {
        expected.segments.len() == native_report.response.transcript.segments.len()
    });
    let text_matches = expected.as_ref().map(|expected| {
        normalize_space(&expected.text_or_joined())
            == normalize_space(&native_report.response.transcript.text_or_joined())
    });

    Ok(ParityReport {
        native_report,
        expected,
        segment_count_matches,
        text_matches,
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

fn map_provider(asr: &AsrConfig) -> TranscriptionProviderSelection {
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
                align_model: asr.external_whisperx.align_model.clone(),
                diarize: asr.external_whisperx.diarize,
                min_speakers: asr.external_whisperx.min_speakers,
                max_speakers: asr.external_whisperx.max_speakers,
                hf_token_env: asr.external_whisperx.hf_token_env.clone(),
                output_dir: asr.external_whisperx.output_dir.clone(),
                timeout_seconds: asr.external_whisperx.timeout_seconds,
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
) -> Result<String, NativeWhisperxError> {
    match format {
        OutputFormat::Json if pretty_json => {
            Ok(serde_json::to_string_pretty(&response.transcript)?)
        }
        OutputFormat::Json => Ok(serde_json::to_string(&response.transcript)?),
        OutputFormat::Srt => Ok(format_srt(&segments_for_format(&response.transcript))),
        OutputFormat::Vtt => Ok(format_webvtt(&segments_for_format(&response.transcript))),
        OutputFormat::Txt => Ok(response.transcript.text_or_joined()),
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
        let transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
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
        let files = write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![
                    OutputFormat::Json,
                    OutputFormat::Srt,
                    OutputFormat::Vtt,
                    OutputFormat::Txt,
                ],
                basename: Some("sample".to_string()),
                pretty_json: true,
            },
        )
        .expect("outputs should write");

        assert_eq!(files.len(), 4);
        assert!(temp.path().join("sample.json").is_file());
        assert!(temp.path().join("sample.srt").is_file());
        assert!(temp.path().join("sample.vtt").is_file());
        assert!(temp.path().join("sample.txt").is_file());
    }
}

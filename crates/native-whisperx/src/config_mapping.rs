//! Mapping from native-whisperx configuration to upstream transcription requests.

use std::path::Path;
#[cfg(any(feature = "pyannote-vad", feature = "silero-vad"))]
use std::path::PathBuf;
#[cfg(feature = "media-decode")]
use std::process::Command;
use std::time::Instant;

#[cfg(feature = "media-decode")]
use audio_analysis_io::{
    AudioIoError, AudioStreamSelectionErrorReason, FfmpegError, MediaStream, MediaStreamInventory,
    MediaType,
};
#[cfg(any(feature = "pyannote-vad", feature = "silero-vad"))]
use audio_analysis_transcription::RequestConfiguredCandleWhisperTranscriber;
use audio_analysis_transcription::{
    run_transcription_pipeline_with_observer, AlignmentOptions, AudioTranscriptionProvider,
    CandleWhisperComputeType, CandleWhisperDecodeConfig, CandleWhisperDecodeRequestConfig,
    CandleWhisperDecodeRuntime, CandleWhisperOptions, CandleWhisperRuntimeControls,
    CandleWhisperTranscriptionRequestConfig, CtcForcedAligner, DiarizationOptions,
    ForcedAlignmentProvider, LoadedAudio, NativeDevicePreference, SpeakerAssignmentPolicy,
    SpeakerDiarizationOptions, TranscriptDiarizationProvider, TranscriptionOutputOptions,
    TranscriptionPipelineEvent, TranscriptionPipelineObserver, TranscriptionPipelineRequest,
    TranscriptionPipelineResponse, TranscriptionProviderSelection, TranscriptionSource,
    TranscriptionTask as UpstreamTranscriptionTask, TranscriptionVadProvider, VadOptions,
    WhisperXCommandOptions, WhisperXDevice,
};
#[cfg(feature = "pyannote-vad")]
use audio_analysis_transcription::{PyannoteVadOptions, PyannoteVadTranscriptionProvider};
#[cfg(feature = "silero-vad")]
use audio_analysis_transcription::{SileroVadOptions, SileroVadTranscriptionProvider};

use crate::config::{
    ensure_whisperx_compat_enabled, is_pyannote_diarization_model,
    resolve_automatic_workflow_selection, AlignmentConfig, AsrConfig, AsrProvider,
    AssignmentPolicy, ConfigSelection, DevicePreference, DiarizationConfig, InputSource,
    NativeWhisperxConfig, NativeWhisperxError, SegmentResolution, SelectedMediaError,
    SelectedMediaInput, TranscriptionTask, VadConfig, VadMethod,
};
#[cfg(feature = "media-decode")]
use crate::config::{
    SelectedMediaErrorReason, SelectedMediaStream, SelectedMediaStreamInventory, SelectedMediaType,
};
#[cfg(all(
    feature = "diarization",
    any(feature = "silero-vad", feature = "pyannote-vad")
))]
use crate::native_diarization_provider;
use crate::output::expand_output_format;
use crate::workflow::{
    NativeProgressContext, TranscriptionProgressEvent, TranscriptionProgressTask,
};

pub(crate) fn build_transcription_request(
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineRequest, NativeWhisperxError> {
    validate_pre_resolution_support(config)?;
    let resolved = resolve_automatic_workflow_selection(config)?;
    build_transcription_request_from_resolved_config(&resolved.config)
}

/// Serializes the internal execution mapping for CLI inspection without exposing its DTO type.
pub fn inspect_workflow_mapping(
    config: &NativeWhisperxConfig,
) -> Result<serde_json::Value, NativeWhisperxError> {
    serde_json::to_value(build_transcription_request(config)?).map_err(NativeWhisperxError::Json)
}

/// Returns the product-required filenames for an explicit Q8 Whisper bundle.
pub fn whisper_q8_required_bundle_files() -> &'static [&'static str] {
    CandleWhisperComputeType::Int8.required_bundle_files()
}

pub(crate) fn validate_pre_resolution_support(
    config: &NativeWhisperxConfig,
) -> Result<(), NativeWhisperxError> {
    if config.asr.provider == AsrProvider::Native
        && config
            .asr
            .compute_type
            .as_deref()
            .is_some_and(|value| value.trim().eq_ignore_ascii_case("int8"))
    {
        validate_native_q8_support(config)?;
    }
    Ok(())
}

pub(crate) fn build_transcription_request_from_resolved_config(
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineRequest, NativeWhisperxError> {
    validate_request_config(config)?;

    Ok(TranscriptionPipelineRequest {
        source: map_input_source(&config.input),
        provider: map_provider(config),
        vad: map_vad(&config.vad),
        alignment: map_alignment(&config.alignment, config.asr.device),
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

pub(crate) fn validate_request_config(
    config: &NativeWhisperxConfig,
) -> Result<(), NativeWhisperxError> {
    if config.asr.provider == AsrProvider::ExternalWhisperX {
        ensure_whisperx_compat_enabled("external WhisperX provider")?;
    }
    if config.output.formats.is_empty() {
        return Err(NativeWhisperxError::InvalidConfig(
            "at least one output format is required".to_string(),
        ));
    }

    validate_native_support(config)?;
    Ok(())
}

pub(crate) fn map_input_source(input: &InputSource) -> TranscriptionSource {
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
    let compute_type = map_native_compute_type(config.asr.compute_type.as_deref())?;
    if compute_type == CandleWhisperComputeType::Int8 {
        validate_native_q8_support(config)?;
    }
    if config.asr.task == TranscriptionTask::Translate && !config.translation.enabled {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --task translate requires --translation-model or --translation-bundle; use --provider external-whisperx for WhisperX built-in translation".to_string(),
        ));
    }
    if config.translation.enabled {
        validate_translation_support(config)?;
    }
    validate_native_vad_support(config)?;
    validate_native_diarization_support(&config.diarization)?;
    validate_native_decode_support(&config.asr)?;
    Ok(())
}

fn validate_native_q8_support(config: &NativeWhisperxConfig) -> Result<(), NativeWhisperxError> {
    let mut incompatible = Vec::new();
    match config.asr.device {
        DevicePreference::Cpu => {}
        DevicePreference::Auto => incompatible
            .push("--device cpu is required; --device auto is not supported".to_string()),
        DevicePreference::Cuda => incompatible
            .push("--device cpu is required; --device cuda is not supported".to_string()),
    }

    match &config.asr.whisper_bundle {
        Some(bundle) => {
            let missing = CandleWhisperComputeType::Int8
                .required_bundle_files()
                .iter()
                .copied()
                .filter(|name| !bundle.join(name).is_file())
                .collect::<Vec<_>>();
            if !missing.is_empty() {
                incompatible.push(format!(
                    "--whisper-bundle `{}` is missing required regular files: {}",
                    bundle.display(),
                    missing.join(", ")
                ));
            }
        }
        None => incompatible.push(format!(
            "--whisper-bundle is required and must contain these regular files: {}",
            CandleWhisperComputeType::Int8
                .required_bundle_files()
                .join(", ")
        )),
    }

    if config.alignment.enabled {
        incompatible.push("alignment must be disabled with --no-align".to_string());
    }
    if config.asr.task != TranscriptionTask::Transcribe {
        incompatible.push("--task transcribe is required".to_string());
    }
    if config.translation.enabled {
        incompatible.push(
            "post-ASR translation must be disabled; remove --translation-model and --translation-bundle"
                .to_string(),
        );
    }
    if config.diarization.enabled {
        incompatible.push("diarization must be disabled; remove --diarize".to_string());
    }

    if incompatible.is_empty() {
        return Ok(());
    }

    Err(NativeWhisperxError::InvalidConfig(format!(
        "native Q8 (--compute-type int8) configuration is incompatible: {}",
        incompatible.join("; ")
    )))
}

pub(crate) fn validate_native_diarization_support(
    diarization: &DiarizationConfig,
) -> Result<(), NativeWhisperxError> {
    if !diarization.enabled {
        return Ok(());
    }
    let is_pyannote = is_pyannote_diarization_model(&diarization.model_id);
    if diarization.model_bundle.is_some() && !is_pyannote {
        return Err(NativeWhisperxError::InvalidConfig(
            "native diarization modelBundle is only supported for pyannote diarization models"
                .to_string(),
        ));
    }
    if diarization.return_speaker_embeddings && !(is_pyannote && diarization.model_bundle.is_some())
    {
        return Err(NativeWhisperxError::InvalidConfig(
            "native speaker embeddings require a pyannote diarization model with an explicit modelBundle".to_string(),
        ));
    }
    if is_pyannote
        && diarization.model_bundle.is_none()
        && diarization.model_selection != ConfigSelection::Automatic
    {
        return Err(NativeWhisperxError::InvalidConfig(
            "native pyannote diarization requires an explicit modelBundle".to_string(),
        ));
    }
    #[cfg(not(feature = "pyannote-diarization"))]
    if is_pyannote && diarization.model_selection != ConfigSelection::Automatic {
        return Err(NativeWhisperxError::InvalidConfig(
            "native pyannote diarization requires the pyannote-diarization feature".to_string(),
        ));
    }
    Ok(())
}

fn validate_translation_support(config: &NativeWhisperxConfig) -> Result<(), NativeWhisperxError> {
    if config.asr.task != TranscriptionTask::Translate {
        return Err(NativeWhisperxError::InvalidConfig(
            "--translation-model requires --task translate".to_string(),
        ));
    }
    if config.translation.model_id.is_none() && config.translation.model_bundle.is_none() {
        return Err(NativeWhisperxError::InvalidConfig(
            "--translation-model or --translation-bundle is required for post-ASR translation"
                .to_string(),
        ));
    }
    if config.translation.max_new_tokens == 0 {
        return Err(NativeWhisperxError::InvalidConfig(
            "--translation-max-new-tokens must be greater than zero".to_string(),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct UnsupportedNativeControl {
    flag: &'static str,
    reason: &'static str,
}

fn validate_native_decode_support(asr: &AsrConfig) -> Result<(), NativeWhisperxError> {
    build_native_request_config(asr)?;
    let mut unsupported = Vec::new();

    let decode = &asr.decode;
    if decode.suppress_tokens.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--suppress_tokens",
            reason: "token suppression requires tokenizer-aware logit filtering before each decode step",
        });
    }
    if decode.suppress_numerals {
        unsupported.push(UnsupportedNativeControl {
            flag: "--suppress_numerals",
            reason: "numeral suppression requires tokenizer-aware logit filtering before each decode step",
        });
    }
    if decode.initial_prompt.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--initial_prompt",
            reason: "prompt-prefilled decoder context is not exposed by the native backend",
        });
    }
    if decode.hotwords.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--hotwords",
            reason: "hotwords are a faster-whisper prompt biasing feature without a native backend equivalent",
        });
    }
    if decode.condition_on_previous_text == Some(true) {
        unsupported.push(UnsupportedNativeControl {
            flag: "--condition_on_previous_text",
            reason:
                "previous-text conditioning requires carrying decoder prompt tokens across chunks",
        });
    }
    if decode.fp16.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--fp16",
            reason: "native precision is selected by the Candle model/device path rather than WhisperX fp16",
        });
    }
    if unsupported.is_empty() {
        return Ok(());
    }

    let details = unsupported
        .iter()
        .map(|control| format!("{} ({})", control.flag, control.reason))
        .collect::<Vec<_>>()
        .join("; ");
    Err(NativeWhisperxError::InvalidConfig(format!(
        "native provider cannot apply decode controls: {details}; use --provider external-whisperx for WhisperX decode-control parity"
    )))
}

pub(crate) fn build_native_runtime_controls(
    asr: &AsrConfig,
) -> Result<CandleWhisperRuntimeControls, NativeWhisperxError> {
    if asr.decode.threads == Some(0) {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --threads must be greater than zero".to_string(),
        ));
    }
    let cuda_device_index = match asr.device_index.as_deref() {
        Some(raw) if raw.contains(',') => {
            return Err(NativeWhisperxError::InvalidConfig(
                "native --device-index accepts one non-negative integer; WhisperX accepts comma-separated device lists, so use one native-whisperx process per CUDA device"
                    .to_string(),
            ));
        }
        Some(raw) => raw.trim().parse::<usize>().map_err(|_| {
            NativeWhisperxError::InvalidConfig(format!(
                "native --device-index must be one non-negative integer, got `{raw}`"
            ))
        })?,
        None => 0,
    };
    if asr.device == DevicePreference::Cpu && cuda_device_index != 0 {
        return Err(NativeWhisperxError::InvalidConfig(format!(
            "native --device-index {cuda_device_index} cannot be used with --device cpu; use --device-index 0 for CPU execution or select --device cuda"
        )));
    }

    Ok(CandleWhisperRuntimeControls {
        cuda_device_index,
        decoder_threads: asr.decode.threads,
    })
}

fn build_native_decode_config(
    asr: &AsrConfig,
) -> Result<CandleWhisperDecodeConfig, NativeWhisperxError> {
    let temperature_schedule = if asr.decode.temperature.is_empty() {
        CandleWhisperDecodeConfig::default().temperature_schedule
    } else {
        asr.decode
            .temperature
            .iter()
            .copied()
            .map(f64::from)
            .collect()
    };

    if temperature_schedule
        .iter()
        .any(|temperature| !temperature.is_finite() || *temperature < 0.0)
    {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --temperature values must be finite and greater than or equal to zero"
                .to_string(),
        ));
    }

    let best_of = asr.decode.best_of.unwrap_or(1);
    if best_of == 0 {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --best_of must be greater than zero".to_string(),
        ));
    }

    let beam_size = asr.decode.beam_size.unwrap_or(1);
    if beam_size == 0 {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --beam_size must be greater than zero".to_string(),
        ));
    }
    if beam_size > 1
        && temperature_schedule
            .iter()
            .any(|temperature| *temperature > 0.0)
    {
        return Err(NativeWhisperxError::InvalidConfig(
            "native beam search requires an all-zero --temperature schedule".to_string(),
        ));
    }
    if beam_size > 1 && best_of != 1 {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --best_of must be 1 when --beam_size is greater than 1".to_string(),
        ));
    }
    if beam_size == 1
        && best_of > 1
        && !temperature_schedule
            .iter()
            .any(|temperature| *temperature > 0.0)
    {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --best_of greater than 1 requires a positive --temperature".to_string(),
        ));
    }

    let patience = asr.decode.patience.map(f64::from).unwrap_or(1.0);
    if !patience.is_finite() || patience <= 0.0 {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --patience must be finite and greater than zero".to_string(),
        ));
    }
    let length_penalty = asr.decode.length_penalty.map(f64::from).unwrap_or(1.0);
    if !length_penalty.is_finite() || length_penalty < 0.0 {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --length_penalty must be finite and greater than or equal to zero".to_string(),
        ));
    }
    if beam_size == 1 && patience != 1.0 {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --patience only applies when --beam_size is greater than 1".to_string(),
        ));
    }
    if beam_size == 1 && length_penalty != 1.0 {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --length_penalty only applies when --beam_size is greater than 1".to_string(),
        ));
    }

    Ok(CandleWhisperDecodeConfig {
        temperature_schedule,
        best_of,
        beam_size,
        patience,
        length_penalty,
        seed: 0,
    })
}

pub(crate) fn build_native_request_config(
    asr: &AsrConfig,
) -> Result<CandleWhisperTranscriptionRequestConfig, NativeWhisperxError> {
    const WHISPERX_LOGPROB_THRESHOLD: f64 = -1.0;
    const WHISPERX_NO_SPEECH_THRESHOLD: f64 = 0.6;
    const WHISPERX_COMPRESSION_RATIO_THRESHOLD: f64 = 2.4;

    let search = build_native_decode_config(asr)?;
    let uses_fallback_schedule = search.temperature_schedule.len() > 1;
    let min_average_log_probability = asr
        .decode
        .logprob_threshold
        .map(f64::from)
        .or(uses_fallback_schedule.then_some(WHISPERX_LOGPROB_THRESHOLD));
    if min_average_log_probability.is_some_and(|value| !value.is_finite()) {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --logprob_threshold must be finite".to_string(),
        ));
    }

    let max_no_speech_probability = asr
        .decode
        .no_speech_threshold
        .map(f64::from)
        .or(uses_fallback_schedule.then_some(WHISPERX_NO_SPEECH_THRESHOLD));
    if max_no_speech_probability
        .is_some_and(|value| !value.is_finite() || !(0.0..=1.0).contains(&value))
    {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --no_speech_threshold must be finite and between zero and one".to_string(),
        ));
    }

    let max_compression_ratio = asr
        .decode
        .compression_ratio_threshold
        .map(f64::from)
        .or(uses_fallback_schedule.then_some(WHISPERX_COMPRESSION_RATIO_THRESHOLD));
    if max_compression_ratio.is_some_and(|value| !value.is_finite() || value <= 0.0) {
        return Err(NativeWhisperxError::InvalidConfig(
            "native --compression_ratio_threshold must be finite and greater than zero".to_string(),
        ));
    }

    Ok(CandleWhisperTranscriptionRequestConfig {
        runtime: build_native_runtime_controls(asr)?,
        decode: CandleWhisperDecodeRequestConfig {
            search,
            min_average_log_probability,
            max_no_speech_probability,
            max_compression_ratio,
            ..CandleWhisperDecodeRequestConfig::default()
        },
        ..CandleWhisperTranscriptionRequestConfig::default()
    })
}

fn map_native_compute_type(
    compute_type: Option<&str>,
) -> Result<CandleWhisperComputeType, NativeWhisperxError> {
    let Some(raw) = compute_type else {
        return Ok(CandleWhisperComputeType::Automatic);
    };
    let normalized = raw.trim().to_ascii_lowercase().replace('-', "_");
    match normalized.as_str() {
        "" | "auto" | "automatic" => Ok(CandleWhisperComputeType::Automatic),
        "float16" | "fp16" => Ok(CandleWhisperComputeType::Fp16),
        "float32" | "fp32" => Ok(CandleWhisperComputeType::Fp32),
        "int8" => Ok(CandleWhisperComputeType::Int8),
        "int8_float16" | "float16_int8" => Err(
            NativeWhisperxError::InvalidConfig(format!(
                "native provider does not support quantized alias --compute_type `{raw}`; use exact --compute-type int8 for the native CPU Q8 workflow or --provider external-whisperx for WhisperX compute-type parity"
            )),
        ),
        _ => Err(NativeWhisperxError::InvalidConfig(format!(
            "native provider supports --compute_type auto, float16/fp16, float32/fp32, or exact int8, got `{raw}`; use --provider external-whisperx for WhisperX compute-type parity"
        ))),
    }
}

fn validate_native_vad_support(config: &NativeWhisperxConfig) -> Result<(), NativeWhisperxError> {
    match config.vad.method {
        VadMethod::Energy => Ok(()),
        VadMethod::Silero => validate_native_silero_config(&config.vad),
        VadMethod::Pyannote => validate_native_pyannote_config(&config.vad),
    }
}

pub(crate) fn validate_native_silero_config(vad: &VadConfig) -> Result<(), NativeWhisperxError> {
    #[cfg(not(feature = "silero-vad"))]
    {
        let _ = vad;
        Err(NativeWhisperxError::InvalidConfig(
            "native Silero VAD requires the silero-vad feature".to_string(),
        ))
    }
    #[cfg(feature = "silero-vad")]
    {
        validate_silero_threshold(vad.onset)?;
        validate_silero_chunk_size(vad.chunk_size)?;
        resolve_silero_model_path(vad).map(|_| ())
    }
}

#[cfg(feature = "silero-vad")]
fn validate_silero_threshold(threshold: Option<f32>) -> Result<(), NativeWhisperxError> {
    if let Some(threshold) = threshold {
        if !threshold.is_finite() || threshold <= 0.0 || threshold >= 1.0 {
            return Err(NativeWhisperxError::InvalidConfig(
                "native Silero VAD requires vad_onset to be finite and between 0 and 1".to_string(),
            ));
        }
    }
    Ok(())
}

#[cfg(feature = "silero-vad")]
fn validate_silero_chunk_size(chunk_size: Option<f64>) -> Result<(), NativeWhisperxError> {
    if let Some(chunk_size) = chunk_size {
        if !chunk_size.is_finite() || chunk_size <= 0.0 {
            return Err(NativeWhisperxError::InvalidConfig(
                "native Silero VAD requires chunk_size to be finite and greater than 0".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_native_pyannote_config(vad: &VadConfig) -> Result<(), NativeWhisperxError> {
    if vad.selection == ConfigSelection::Automatic {
        return Ok(());
    }
    #[cfg(not(feature = "pyannote-vad"))]
    {
        let _ = vad;
        Err(NativeWhisperxError::InvalidConfig(
            "native pyannote VAD requires the pyannote-vad feature".to_string(),
        ))
    }
    #[cfg(feature = "pyannote-vad")]
    {
        validate_pyannote_threshold("vad_onset", vad.onset)?;
        validate_pyannote_threshold("vad_offset", vad.offset)?;
        validate_pyannote_chunk_size(vad.chunk_size)?;
        resolve_pyannote_vad_model_path(vad).map(|_| ())
    }
}

#[cfg(feature = "pyannote-vad")]
fn validate_pyannote_threshold(
    name: &str,
    threshold: Option<f32>,
) -> Result<(), NativeWhisperxError> {
    if let Some(threshold) = threshold {
        if !threshold.is_finite() || threshold <= 0.0 || threshold >= 1.0 {
            return Err(NativeWhisperxError::InvalidConfig(format!(
                "native pyannote VAD requires {name} to be finite and between 0 and 1"
            )));
        }
    }
    Ok(())
}

#[cfg(feature = "pyannote-vad")]
fn validate_pyannote_chunk_size(chunk_size: Option<f64>) -> Result<(), NativeWhisperxError> {
    if let Some(chunk_size) = chunk_size {
        if !chunk_size.is_finite() || chunk_size <= 0.0 {
            return Err(NativeWhisperxError::InvalidConfig(
                "native pyannote VAD requires chunk_size to be finite and greater than 0"
                    .to_string(),
            ));
        }
    }
    Ok(())
}

#[cfg(feature = "silero-vad")]
fn build_silero_vad_provider(
    vad: &VadConfig,
) -> Result<SileroVadTranscriptionProvider, NativeWhisperxError> {
    let model_path = resolve_silero_model_path(vad)?;
    let threshold = vad.onset.unwrap_or(0.5);
    let max_speech_duration_seconds = vad.chunk_size.unwrap_or(30.0);
    validate_silero_threshold(Some(threshold))?;
    validate_silero_chunk_size(Some(max_speech_duration_seconds))?;
    let options = SileroVadOptions {
        model_path: model_path.clone(),
        input_name: vad.input_name.clone(),
        output_name: vad.output_name.clone(),
        threshold,
        max_speech_duration_seconds,
        min_speech_duration_ms: 250,
        min_silence_duration_ms: 100,
        speech_pad_ms: 30,
    };
    let mut diagnostics = vec![
        format!("sileroVadThreshold={threshold}"),
        format!("sileroVadChunkSizeSeconds={max_speech_duration_seconds}"),
        format!("sileroVadModel={}", model_path.display()),
    ];
    if vad.offset.is_some() {
        diagnostics.push(
            "native Silero VAD accepts vad_offset for WhisperX CLI parity; WhisperX Silero merge does not use vad_offset".to_string(),
        );
    }
    SileroVadTranscriptionProvider::from_options(options, diagnostics)
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
}

#[allow(dead_code)]
pub(crate) fn run_native_with_selected_vad(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    run_native_with_selected_vad_and_progress(request, config, None)
}

pub(crate) fn run_native_with_selected_vad_and_progress(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
    progress: Option<NativeProgressContext<'_>>,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    match config.vad.method {
        VadMethod::Silero => {
            #[cfg(feature = "silero-vad")]
            {
                let mut vad_provider = build_silero_vad_provider(&config.vad)?;
                run_native_with_custom_vad(request, config, &mut vad_provider, progress)
            }
            #[cfg(not(feature = "silero-vad"))]
            {
                let _ = progress;
                let _ = (request, config);
                Err(NativeWhisperxError::InvalidConfig(
                    "native Silero VAD requires the silero-vad feature".to_string(),
                ))
            }
        }
        VadMethod::Pyannote => {
            #[cfg(feature = "pyannote-vad")]
            {
                let mut vad_provider = build_pyannote_vad_provider(&config.vad)?;
                run_native_with_custom_vad(request, config, &mut vad_provider, progress)
            }
            #[cfg(not(feature = "pyannote-vad"))]
            {
                let _ = progress;
                let _ = (request, config);
                Err(NativeWhisperxError::InvalidConfig(
                    "native pyannote VAD requires the pyannote-vad feature".to_string(),
                ))
            }
        }
        VadMethod::Energy => {
            let _ = (request, progress);
            Err(NativeWhisperxError::InvalidConfig(
                "custom native VAD was requested for energy VAD".to_string(),
            ))
        }
    }
}

#[cfg(feature = "pyannote-vad")]
fn build_pyannote_vad_provider(
    vad: &VadConfig,
) -> Result<PyannoteVadTranscriptionProvider, NativeWhisperxError> {
    let model_path = resolve_pyannote_vad_model_path(vad)?;
    let onset = vad.onset.unwrap_or(0.5);
    let offset = vad.offset.unwrap_or(0.363);
    let chunk_size = vad.chunk_size.unwrap_or(30.0);
    validate_pyannote_threshold("vad_onset", Some(onset))?;
    validate_pyannote_threshold("vad_offset", Some(offset))?;
    validate_pyannote_chunk_size(Some(chunk_size))?;
    let options = PyannoteVadOptions {
        model_path: model_path.clone(),
        input_name: vad.input_name.clone(),
        output_name: vad.output_name.clone(),
        onset,
        offset,
        chunk_size,
    };
    let diagnostics = vec![
        format!("pyannoteVadOnset={onset}"),
        format!("pyannoteVadOffset={offset}"),
        format!("pyannoteVadChunkSizeSeconds={chunk_size}"),
        format!("pyannoteVadModel={}", model_path.display()),
    ];
    PyannoteVadTranscriptionProvider::from_options(options, diagnostics)
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
}

#[cfg(any(feature = "silero-vad", feature = "pyannote-vad"))]
fn run_native_with_custom_vad(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
    vad_provider: &mut dyn TranscriptionVadProvider,
    progress: Option<NativeProgressContext<'_>>,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    #[cfg(not(feature = "diarization"))]
    let _ = config;

    let TranscriptionProviderSelection::CandleWhisper(options) = &request.provider else {
        return Err(NativeWhisperxError::InvalidConfig(
            "custom native VAD requires the Candle Whisper native provider".to_string(),
        ));
    };
    let request_config = build_native_request_config(&config.asr)?;
    let mut asr_provider =
        RequestConfiguredCandleWhisperTranscriber::new(options.clone(), request_config);

    #[cfg(feature = "diarization")]
    {
        if request.diarization.enabled {
            let mut diarizer = native_diarization_provider(config)?;
            return run_native_with_optional_alignment_and_progress(
                request,
                vad_provider,
                &mut asr_provider,
                Some(&mut diarizer as &mut dyn TranscriptDiarizationProvider),
                progress,
            );
        }
    }

    run_native_with_optional_alignment_and_progress(
        request,
        vad_provider,
        &mut asr_provider,
        None,
        progress,
    )
}

#[allow(dead_code)]
pub(crate) fn run_native_with_optional_alignment(
    request: TranscriptionPipelineRequest,
    vad_provider: &mut dyn TranscriptionVadProvider,
    asr_provider: &mut dyn AudioTranscriptionProvider,
    #[cfg_attr(not(feature = "diarization"), allow(unused_variables))] diarization_provider: Option<
        &mut dyn TranscriptDiarizationProvider,
    >,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    run_native_with_optional_alignment_and_progress(
        request,
        vad_provider,
        asr_provider,
        diarization_provider,
        None,
    )
}

pub(crate) fn run_native_with_optional_alignment_and_progress(
    request: TranscriptionPipelineRequest,
    vad_provider: &mut dyn TranscriptionVadProvider,
    asr_provider: &mut dyn AudioTranscriptionProvider,
    #[cfg_attr(not(feature = "diarization"), allow(unused_variables))] diarization_provider: Option<
        &mut dyn TranscriptDiarizationProvider,
    >,
    progress: Option<NativeProgressContext<'_>>,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    let (request, mut decode_diagnostics) = predecode_native_request_input(request)?;
    let mut phase_observer = PhaseTimingObserver::default();
    let result = {
        let mut observer = NativePipelineProgressObserver::new(&mut phase_observer, progress);
        if request.alignment.enabled {
            let mut aligner = CtcForcedAligner {
                options: request.alignment.clone(),
            };
            run_transcription_pipeline_with_observer(
                request,
                vad_provider,
                asr_provider,
                Some(&mut aligner as &mut dyn ForcedAlignmentProvider),
                diarization_provider,
                &mut observer,
            )
        } else {
            run_transcription_pipeline_with_observer(
                request,
                vad_provider,
                asr_provider,
                None,
                diarization_provider,
                &mut observer,
            )
        }
    };

    result
        .map(|mut response| {
            response.diagnostics.append(&mut decode_diagnostics);
            phase_observer.append_diagnostics(&mut response.diagnostics);
            response
        })
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
}

pub(crate) fn predecode_native_config_input(
    mut config: NativeWhisperxConfig,
    selected_media: Option<SelectedMediaInput>,
) -> Result<(NativeWhisperxConfig, Vec<String>), SelectedMediaError> {
    let Some(selected_media) = selected_media else {
        return Ok((config, Vec::new()));
    };
    if config.asr.provider != AsrProvider::Native {
        return Ok((config, Vec::new()));
    }
    let (source, diagnostics) =
        predecode_selected_media_source(map_input_source(&config.input), selected_media)?;
    config.input = match source {
        TranscriptionSource::Samples {
            samples,
            sample_rate,
            channels,
            source,
        } => InputSource::Samples {
            samples,
            sample_rate,
            channels,
            source,
        },
        TranscriptionSource::Path { .. } | TranscriptionSource::Media { .. } => config.input,
    };
    Ok((config, diagnostics))
}

pub(crate) fn predecode_native_request_input(
    mut request: TranscriptionPipelineRequest,
) -> Result<(TranscriptionPipelineRequest, Vec<String>), NativeWhisperxError> {
    let (source, diagnostics) = predecode_native_source(request.source)?;
    request.source = source;
    Ok((request, diagnostics))
}

fn predecode_native_source(
    source: TranscriptionSource,
) -> Result<(TranscriptionSource, Vec<String>), NativeWhisperxError> {
    let TranscriptionSource::Path { path } = &source else {
        return Ok((source, Vec::new()));
    };
    let route = native_path_decode_route(path);
    #[cfg(not(feature = "media-decode"))]
    if route != "native-wav-reader" {
        return Err(NativeWhisperxError::InvalidConfig(format!(
            "native non-WAV media input `{}` requires the media-decode feature for FFmpeg-backed container/video input; enable media-decode, pass WAV or Samples, or use --provider external-whisperx",
            path.display()
        )));
    }

    #[cfg(feature = "media-decode")]
    if route != "native-wav-reader" {
        ensure_media_decode_runtime(path)?;
    }

    let decode_started = Instant::now();
    #[cfg(feature = "media-decode")]
    let audio = LoadedAudio::mono_16khz_from_source(&source)
        .map_err(|error| native_path_decode_error(path, route, error))?;
    #[cfg(not(feature = "media-decode"))]
    let audio = LoadedAudio::mono_16khz_from_source(&source)
        .map_err(|error| native_path_decode_error(path, route, error))?;
    Ok(decoded_source_with_diagnostics(
        audio,
        path,
        route,
        None,
        decode_started,
    ))
}

fn predecode_selected_media_source(
    source: TranscriptionSource,
    selected_media: SelectedMediaInput,
) -> Result<(TranscriptionSource, Vec<String>), SelectedMediaError> {
    let TranscriptionSource::Path { path } = &source else {
        return Ok((source, Vec::new()));
    };
    #[cfg(not(feature = "media-decode"))]
    {
        let _ = selected_media;
        return Err(NativeWhisperxError::InvalidConfig(format!(
            "native non-WAV media input `{}` requires the media-decode feature for FFmpeg-backed container/video input; enable media-decode, pass WAV or Samples, or use --provider external-whisperx",
            path.display()
        ))
        .into());
    }
    #[cfg(feature = "media-decode")]
    {
        ensure_media_decode_runtime(path)?;
        let decode_started = Instant::now();
        let audio =
            LoadedAudio::mono_16khz_from_selected_media(path, Some(selected_media.audio_track))
                .map_err(|error| {
                    selected_media_decode_error(path, selected_media.audio_track, error)
                })?;
        Ok(decoded_source_with_diagnostics(
            audio,
            path,
            "audio-io-selected-media-decode",
            Some(selected_media.audio_track),
            decode_started,
        ))
    }
}

fn decoded_source_with_diagnostics(
    audio: LoadedAudio,
    path: &Path,
    route: &'static str,
    audio_track: Option<usize>,
    decode_started: Instant,
) -> (TranscriptionSource, Vec<String>) {
    let mut diagnostics = vec![
        format!("nativeDecodeRoute={route}"),
        format!("nativeDecodeInput={}", path.display()),
        format!("nativeDecodeOutputSampleRate={}", audio.sample_rate),
        format!("nativeDecodeOutputChannels={}", audio.channels),
        format!(
            "phaseNativePredecodeSeconds={:.6}",
            decode_started.elapsed().as_secs_f64()
        ),
    ];
    if let Some(audio_track) = audio_track {
        diagnostics.push(format!("nativeDecodeAudioTrack={audio_track}"));
    }
    (
        TranscriptionSource::Samples {
            samples: audio.samples,
            sample_rate: audio.sample_rate,
            channels: audio.channels,
            source: audio.source,
        },
        diagnostics,
    )
}

fn native_path_decode_route(path: &Path) -> &'static str {
    if path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("wav"))
    {
        "native-wav-reader"
    } else {
        "audio-io-media-decode"
    }
}

#[cfg(feature = "media-decode")]
fn selected_media_decode_error(
    path: &Path,
    audio_track: usize,
    error: AudioIoError,
) -> SelectedMediaError {
    match error {
        AudioIoError::Ffmpeg(FfmpegError::InvalidAudioStreamSelection {
            reason,
            available_streams,
            ..
        }) => SelectedMediaError::StreamSelection {
            path: path.to_path_buf(),
            audio_track,
            available_streams_summary: available_stream_context(&available_streams),
            reason: selected_media_error_reason(reason),
            available_streams: selected_media_stream_inventory(available_streams),
        },
        error => SelectedMediaError::Workflow(NativeWhisperxError::Transcription(format!(
            "native selected-media decode failed for `{}` before model loading: {error}",
            path.display()
        ))),
    }
}

#[cfg(feature = "media-decode")]
fn selected_media_error_reason(
    reason: AudioStreamSelectionErrorReason,
) -> SelectedMediaErrorReason {
    match reason {
        AudioStreamSelectionErrorReason::NoAudioStreams => SelectedMediaErrorReason::NoAudioStreams,
        AudioStreamSelectionErrorReason::OutOfRange => SelectedMediaErrorReason::OutOfRange,
        AudioStreamSelectionErrorReason::NotAudio => SelectedMediaErrorReason::NotAudio,
    }
}

#[cfg(feature = "media-decode")]
fn selected_media_stream_inventory(
    inventory: MediaStreamInventory,
) -> SelectedMediaStreamInventory {
    SelectedMediaStreamInventory {
        streams: inventory
            .streams
            .into_iter()
            .map(selected_media_stream)
            .collect(),
    }
}

#[cfg(feature = "media-decode")]
fn selected_media_stream(stream: MediaStream) -> SelectedMediaStream {
    SelectedMediaStream {
        index: stream.index,
        media_type: match stream.media_type {
            MediaType::Video => SelectedMediaType::Video,
            MediaType::Audio => SelectedMediaType::Audio,
            MediaType::Subtitle => SelectedMediaType::Subtitle,
            MediaType::Data => SelectedMediaType::Data,
            MediaType::Attachment => SelectedMediaType::Attachment,
            MediaType::Unknown(value) => SelectedMediaType::Unknown(value),
        },
        audio_stream_ordinal: stream.audio_stream_ordinal,
        codec: stream.codec,
        channels: stream.channels,
        sample_rate: stream.sample_rate,
        language: stream.language,
        default_disposition: stream.default_disposition,
    }
}

#[cfg(feature = "media-decode")]
fn available_stream_context(inventory: &MediaStreamInventory) -> String {
    if inventory.streams.is_empty() {
        return "none".to_string();
    }
    inventory
        .streams
        .iter()
        .map(|stream| {
            let audio_track = stream
                .audio_stream_ordinal
                .map(|ordinal| ordinal.to_string())
                .unwrap_or_else(|| "n/a".to_string());
            format!(
                "global-index={} type={:?} audio-track={} codec={} channels={} sample-rate={} language={} default={}",
                stream.index,
                stream.media_type,
                audio_track,
                stream.codec.as_deref().unwrap_or("unknown"),
                stream
                    .channels
                    .map(|channels| channels.to_string())
                    .unwrap_or_else(|| "n/a".to_string()),
                stream
                    .sample_rate
                    .map(|sample_rate| sample_rate.to_string())
                    .unwrap_or_else(|| "n/a".to_string()),
                stream.language.as_deref().unwrap_or("unknown"),
                stream
                    .default_disposition
                    .map(|default| default.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

#[cfg(feature = "media-decode")]
fn ensure_media_decode_runtime(path: &Path) -> Result<(), NativeWhisperxError> {
    let missing = ["ffmpeg", "ffprobe"]
        .into_iter()
        .filter(|command| !command_is_available(command))
        .collect::<Vec<_>>();
    if missing.is_empty() {
        return Ok(());
    }

    Err(NativeWhisperxError::Transcription(format!(
        "native media decode for non-WAV input `{}` requires FFmpeg runtime tools on PATH; missing {}; install ffmpeg and ffprobe or use --provider external-whisperx",
        path.display(),
        missing.join(" and ")
    )))
}

#[cfg(feature = "media-decode")]
fn command_is_available(command: &str) -> bool {
    Command::new(command)
        .arg("-version")
        .output()
        .is_ok_and(|output| output.status.success())
}

fn native_path_decode_error(
    path: &Path,
    route: &'static str,
    error: media_core::DetectError,
) -> NativeWhisperxError {
    let hint = if route == "audio-io-media-decode" {
        "FFmpeg-backed media decode failed before ASR, alignment, diarization, translation, or output writing"
    } else {
        "native WAV decode failed before ASR, alignment, diarization, translation, or output writing"
    };
    NativeWhisperxError::Transcription(format!(
        "native decode failed for `{}` via {route}: {error}; {hint}",
        path.display()
    ))
}

struct NativePipelineProgressObserver<'phase, 'progress> {
    phase: &'phase mut PhaseTimingObserver,
    progress: Option<NativeProgressContext<'progress>>,
}

impl<'phase, 'progress> NativePipelineProgressObserver<'phase, 'progress> {
    fn new(
        phase: &'phase mut PhaseTimingObserver,
        progress: Option<NativeProgressContext<'progress>>,
    ) -> Self {
        Self { phase, progress }
    }

    fn progress_task(stage: &str) -> Option<TranscriptionProgressTask> {
        match stage {
            "decode" => Some(TranscriptionProgressTask::Decode),
            "vad" => Some(TranscriptionProgressTask::Vad),
            "asr" => Some(TranscriptionProgressTask::Asr),
            "alignment" => Some(TranscriptionProgressTask::Alignment),
            "diarization" => Some(TranscriptionProgressTask::Diarization),
            _ => None,
        }
    }

    fn start_task(&mut self, task: TranscriptionProgressTask) {
        if let Some(progress) = &mut self.progress {
            progress.task_tracker.set_current(Some(task));
            progress
                .observer
                .observe(TranscriptionProgressEvent::TaskStart {
                    file_index: progress.file_index,
                    task,
                });
        }
    }

    fn end_task(&mut self, task: TranscriptionProgressTask, duration_seconds: f64) {
        if let Some(progress) = &mut self.progress {
            progress
                .observer
                .observe(TranscriptionProgressEvent::TaskEnd {
                    file_index: progress.file_index,
                    task,
                    duration_seconds,
                });
            progress.task_tracker.set_current(None);
        }
    }
}

impl TranscriptionPipelineObserver for NativePipelineProgressObserver<'_, '_> {
    fn observe(&mut self, event: TranscriptionPipelineEvent) {
        self.phase.observe(event.clone());
        match event {
            TranscriptionPipelineEvent::ValidationStart => {}
            TranscriptionPipelineEvent::DecodeStart => {
                self.start_task(TranscriptionProgressTask::Decode);
            }
            TranscriptionPipelineEvent::DecodeEnd {
                duration_seconds, ..
            } => {
                self.end_task(TranscriptionProgressTask::Decode, duration_seconds);
            }
            TranscriptionPipelineEvent::VadStart { .. } => {
                self.start_task(TranscriptionProgressTask::Vad);
            }
            TranscriptionPipelineEvent::VadEnd { .. } => {
                let duration_seconds = self.phase.vad_seconds.unwrap_or_default();
                self.end_task(TranscriptionProgressTask::Vad, duration_seconds);
            }
            TranscriptionPipelineEvent::AsrStart { .. } => {
                self.start_task(TranscriptionProgressTask::Asr);
            }
            TranscriptionPipelineEvent::AsrEnd { .. } => {
                let duration_seconds = self.phase.asr_seconds.unwrap_or_default();
                self.end_task(TranscriptionProgressTask::Asr, duration_seconds);
            }
            TranscriptionPipelineEvent::AlignmentStart { .. } => {
                self.start_task(TranscriptionProgressTask::Alignment);
            }
            TranscriptionPipelineEvent::AlignmentEnd { .. } => {
                let duration_seconds = self.phase.alignment_seconds.unwrap_or_default();
                self.end_task(TranscriptionProgressTask::Alignment, duration_seconds);
            }
            TranscriptionPipelineEvent::DiarizationStart { .. } => {
                self.start_task(TranscriptionProgressTask::Diarization);
            }
            TranscriptionPipelineEvent::DiarizationEnd { .. } => {
                let duration_seconds = self.phase.diarization_seconds.unwrap_or_default();
                self.end_task(TranscriptionProgressTask::Diarization, duration_seconds);
            }
            TranscriptionPipelineEvent::ModelLoadStart {
                stage,
                provider,
                model_id,
            } => {
                if let (Some(progress), Some(task)) =
                    (&mut self.progress, Self::progress_task(&stage))
                {
                    progress
                        .observer
                        .observe(TranscriptionProgressEvent::ModelLoadStart {
                            file_index: progress.file_index,
                            task,
                            provider,
                            model_id,
                        });
                }
            }
            TranscriptionPipelineEvent::ModelLoadEnd {
                stage,
                provider,
                model_id,
                duration_seconds,
            } => {
                if let (Some(progress), Some(task)) =
                    (&mut self.progress, Self::progress_task(&stage))
                {
                    progress
                        .observer
                        .observe(TranscriptionProgressEvent::ModelLoadEnd {
                            file_index: progress.file_index,
                            task,
                            provider,
                            model_id,
                            duration_seconds,
                        });
                }
            }
            TranscriptionPipelineEvent::ModelReuse {
                stage,
                provider,
                model_id,
            } => {
                if let (Some(progress), Some(task)) =
                    (&mut self.progress, Self::progress_task(&stage))
                {
                    progress
                        .observer
                        .observe(TranscriptionProgressEvent::ModelReuse {
                            file_index: progress.file_index,
                            task,
                            provider,
                            model_id,
                        });
                }
            }
        }
    }

    fn cancellation_requested(&self) -> bool {
        self.progress
            .as_ref()
            .is_some_and(|progress| progress.cancellation.is_cancelled())
    }

    fn model_resolution_start(&mut self, stage: &str, provider: &str, model_id: &str) {
        if let (Some(progress), Some(task)) = (&mut self.progress, Self::progress_task(stage)) {
            progress
                .observer
                .observe(TranscriptionProgressEvent::ModelResolutionStart {
                    file_index: progress.file_index,
                    task,
                    provider: provider.to_string(),
                    model_id: model_id.to_string(),
                });
        }
    }

    fn model_resolution_end(&mut self, stage: &str, provider: &str, model_id: &str, source: &str) {
        if let (Some(progress), Some(task)) = (&mut self.progress, Self::progress_task(stage)) {
            progress
                .observer
                .observe(TranscriptionProgressEvent::ModelResolutionEnd {
                    file_index: progress.file_index,
                    task,
                    provider: provider.to_string(),
                    model_id: model_id.to_string(),
                    source: source.to_string(),
                });
        }
    }

    fn model_download_start(&mut self, stage: &str, provider: &str, model_id: &str) {
        if let (Some(progress), Some(task)) = (&mut self.progress, Self::progress_task(stage)) {
            progress
                .observer
                .observe(TranscriptionProgressEvent::ModelDownloadStart {
                    file_index: progress.file_index,
                    task,
                    provider: provider.to_string(),
                    model_id: model_id.to_string(),
                });
        }
    }

    fn model_download_end(
        &mut self,
        stage: &str,
        provider: &str,
        model_id: &str,
        duration_seconds: f64,
    ) {
        if let (Some(progress), Some(task)) = (&mut self.progress, Self::progress_task(stage)) {
            progress
                .observer
                .observe(TranscriptionProgressEvent::ModelDownloadEnd {
                    file_index: progress.file_index,
                    task,
                    provider: provider.to_string(),
                    model_id: model_id.to_string(),
                    duration_seconds,
                });
        }
    }
}

#[derive(Debug, Default)]
struct PhaseTimingObserver {
    decode_seconds: Option<f64>,
    decode_samples: Option<usize>,
    vad_started: Option<Instant>,
    vad_seconds: Option<f64>,
    vad_segments: Option<usize>,
    vad_windows: Option<usize>,
    asr_started: Option<Instant>,
    asr_seconds: Option<f64>,
    asr_model_load_seconds: Option<f64>,
    asr_segments: Option<usize>,
    alignment_started: Option<Instant>,
    alignment_seconds: Option<f64>,
    alignment_words: Option<usize>,
    diarization_started: Option<Instant>,
    diarization_seconds: Option<f64>,
    diarization_speakers: Option<usize>,
    diarization_segments: Option<usize>,
}

impl PhaseTimingObserver {
    fn append_diagnostics(&self, diagnostics: &mut Vec<String>) {
        push_optional_seconds(diagnostics, "phaseDecodeSeconds", self.decode_seconds);
        push_optional_usize(diagnostics, "phaseDecodeSamples", self.decode_samples);
        push_optional_seconds(diagnostics, "phaseVadSeconds", self.vad_seconds);
        push_optional_usize(diagnostics, "phaseVadSegments", self.vad_segments);
        push_optional_usize(diagnostics, "phaseVadWindows", self.vad_windows);
        push_optional_seconds(diagnostics, "phaseAsrSeconds", self.asr_seconds);
        push_optional_seconds(
            diagnostics,
            "phaseAsrModelLoadSeconds",
            self.asr_model_load_seconds,
        );
        push_optional_usize(diagnostics, "phaseAsrSegments", self.asr_segments);
        push_optional_seconds(diagnostics, "phaseAlignmentSeconds", self.alignment_seconds);
        push_optional_usize(diagnostics, "phaseAlignmentWords", self.alignment_words);
        push_optional_seconds(
            diagnostics,
            "phaseDiarizationSeconds",
            self.diarization_seconds,
        );
        push_optional_usize(
            diagnostics,
            "phaseDiarizationSpeakers",
            self.diarization_speakers,
        );
        push_optional_usize(
            diagnostics,
            "phaseDiarizationSegments",
            self.diarization_segments,
        );
    }
}

impl TranscriptionPipelineObserver for PhaseTimingObserver {
    fn observe(&mut self, event: TranscriptionPipelineEvent) {
        match event {
            TranscriptionPipelineEvent::ValidationStart => {}
            TranscriptionPipelineEvent::ModelLoadStart { .. }
            | TranscriptionPipelineEvent::ModelReuse { .. } => {}
            TranscriptionPipelineEvent::ModelLoadEnd {
                stage,
                duration_seconds,
                ..
            } => {
                if stage == "asr" {
                    self.asr_model_load_seconds = Some(duration_seconds);
                }
            }
            TranscriptionPipelineEvent::DecodeStart => {}
            TranscriptionPipelineEvent::DecodeEnd {
                duration_seconds,
                samples,
            } => {
                self.decode_seconds = Some(duration_seconds);
                self.decode_samples = Some(samples);
            }
            TranscriptionPipelineEvent::VadStart { .. } => {
                self.vad_started = Some(Instant::now());
            }
            TranscriptionPipelineEvent::VadEnd { segments, windows } => {
                self.vad_seconds = self
                    .vad_started
                    .map(|started| started.elapsed().as_secs_f64());
                self.vad_segments = Some(segments);
                self.vad_windows = windows;
            }
            TranscriptionPipelineEvent::AsrStart { .. } => {
                self.asr_started = Some(Instant::now());
            }
            TranscriptionPipelineEvent::AsrEnd { segments } => {
                self.asr_seconds = self
                    .asr_started
                    .map(|started| started.elapsed().as_secs_f64());
                self.asr_segments = Some(segments);
            }
            TranscriptionPipelineEvent::AlignmentStart { .. } => {
                self.alignment_started = Some(Instant::now());
            }
            TranscriptionPipelineEvent::AlignmentEnd { words } => {
                self.alignment_seconds = self
                    .alignment_started
                    .map(|started| started.elapsed().as_secs_f64());
                self.alignment_words = Some(words);
            }
            TranscriptionPipelineEvent::DiarizationStart { .. } => {
                self.diarization_started = Some(Instant::now());
            }
            TranscriptionPipelineEvent::DiarizationEnd { speakers, segments } => {
                self.diarization_seconds = self
                    .diarization_started
                    .map(|started| started.elapsed().as_secs_f64());
                self.diarization_speakers = Some(speakers);
                self.diarization_segments = Some(segments);
            }
        }
    }
}

fn push_optional_seconds(diagnostics: &mut Vec<String>, key: &str, value: Option<f64>) {
    if let Some(value) = value {
        diagnostics.push(format!("{key}={value:.6}"));
    }
}

fn push_optional_usize(diagnostics: &mut Vec<String>, key: &str, value: Option<usize>) {
    if let Some(value) = value {
        diagnostics.push(format!("{key}={value}"));
    }
}

fn map_provider(config: &NativeWhisperxConfig) -> TranscriptionProviderSelection {
    let asr = &config.asr;
    match asr.provider {
        AsrProvider::Native => {
            TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
                model_id: asr.model_id.clone(),
                task: map_transcription_task(native_asr_task(config)),
                language: native_language_hint(asr),
                device: map_device(asr.device),
                compute_type: map_native_compute_type(asr.compute_type.as_deref())
                    .expect("native compute type should be validated before provider mapping"),
                model_bundle: asr.whisper_bundle.clone(),
                model_dir: asr.model_dir.clone(),
                model_cache_only: asr.model_cache_only,
                batch_chunks: asr.batch_chunks,
                max_batch_size: asr.max_batch_size,
                decode_runtime: map_candle_decode_runtime(asr),
            })
        }
        AsrProvider::ExternalWhisperX => {
            let mut extra_args = external_whisperx_extra_args(config);
            let builtin_diarize =
                config.diarization.enabled && config.diarization.hf_token.is_none();
            let model_cache_only = asr.model_cache_only || config.alignment.model_cache_only;
            if model_cache_only {
                extra_args.extend(["--model_cache_only".to_string(), "True".to_string()]);
            }
            TranscriptionProviderSelection::ExternalWhisperX(WhisperXCommandOptions {
                command: asr.external_whisperx.command.clone(),
                model: asr.external_whisperx.model.clone(),
                task: map_transcription_task(asr.task),
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
                output_dir: asr
                    .external_whisperx
                    .output_dir
                    .clone()
                    .or_else(|| config.output.output_dir.clone()),
                timeout_seconds: asr.external_whisperx.timeout_seconds,
                model_dir: asr
                    .model_dir
                    .clone()
                    .or_else(|| config.alignment.model_dir.clone()),
                model_cache_only: false,
                no_align: !config.alignment.enabled,
                interpolate_method: config.alignment.interpolate_method.as_upstream(),
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

fn map_candle_decode_runtime(asr: &AsrConfig) -> CandleWhisperDecodeRuntime {
    if asr.batch_chunks && asr.max_batch_size != Some(1) {
        return CandleWhisperDecodeRuntime::ActiveRowTensorBatch;
    }
    CandleWhisperDecodeRuntime::AutoregressiveKvCache
}

fn native_asr_task(config: &NativeWhisperxConfig) -> TranscriptionTask {
    if config.asr.task == TranscriptionTask::Translate && config.translation.enabled {
        TranscriptionTask::Transcribe
    } else {
        config.asr.task
    }
}

fn map_transcription_task(task: TranscriptionTask) -> UpstreamTranscriptionTask {
    match task {
        TranscriptionTask::Transcribe => UpstreamTranscriptionTask::Transcribe,
        TranscriptionTask::Translate => UpstreamTranscriptionTask::Translate,
    }
}

fn external_whisperx_extra_args(config: &NativeWhisperxConfig) -> Vec<String> {
    let mut args = config.asr.external_whisperx.extra_args.clone();
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
        args.extend(["--highlight_words".to_string(), "True".to_string()]);
    }
    push_arg(
        &mut args,
        "--segment_resolution",
        Some(match config.output.subtitles.segment_resolution {
            SegmentResolution::Sentence => "sentence",
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

pub(crate) fn native_language_hint(asr: &AsrConfig) -> Option<String> {
    asr.language
        .clone()
        .or_else(|| english_only_whisper_model(&asr.model_id).then(|| "en".to_string()))
}

fn english_only_whisper_model(model_id: &str) -> bool {
    let normalized = model_id
        .rsplit('/')
        .next()
        .unwrap_or(model_id)
        .strip_prefix("whisper-")
        .unwrap_or_else(|| model_id.rsplit('/').next().unwrap_or(model_id));
    matches!(normalized, "tiny.en" | "base.en" | "small.en" | "medium.en")
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

fn map_alignment(
    alignment: &AlignmentConfig,
    native_asr_device: DevicePreference,
) -> AlignmentOptions {
    AlignmentOptions {
        enabled: alignment.enabled,
        model_id: alignment.model_id.clone(),
        device: map_device(native_asr_device),
        model_bundle: alignment.model_bundle.clone(),
        model_dir: alignment.model_dir.clone(),
        model_cache_only: alignment.model_cache_only,
        interpolate_method: alignment.interpolate_method.as_upstream(),
        return_char_alignments: alignment.return_char_alignments,
    }
}

pub(crate) fn map_diarization(diarization: &DiarizationConfig) -> DiarizationOptions {
    DiarizationOptions {
        enabled: diarization.enabled,
        speaker: SpeakerDiarizationOptions {
            model_id: diarization.model_id.clone(),
            pyannote_model_bundle: diarization.model_bundle.clone(),
            pyannote_manifest_file: diarization.manifest_file.clone(),
            pyannote_segmentation_model_file: diarization.segmentation_model_file.clone(),
            pyannote_embedding_model_file: diarization.embedding_model_file.clone(),
            pyannote_plda_transform_file: diarization.plda_transform_file.clone(),
            pyannote_plda_model_file: diarization.plda_model_file.clone(),
            pyannote_clustering_config_file: diarization.clustering_config_file.clone(),
            speaker_embedding_model_bundle: diarization.speaker_embedding_model_bundle.clone(),
            speaker_embedding_model_file: diarization.speaker_embedding_model_file.clone(),
            speaker_embedding_input_name: None,
            speaker_embedding_output_name: None,
            speaker_embedding_dimension: diarization.speaker_embedding_dimension,
            speaker_embedding_sample_rate: diarization.speaker_embedding_sample_rate,
            return_speaker_embeddings: diarization.return_speaker_embeddings,
            min_speakers: diarization.min_speakers,
            max_speakers: diarization.max_speakers,
            assignment_policy: match diarization.assignment_policy {
                AssignmentPolicy::Majority => SpeakerAssignmentPolicy::Majority,
                AssignmentPolicy::NearestStart => SpeakerAssignmentPolicy::NearestStart,
                AssignmentPolicy::StrictContained => SpeakerAssignmentPolicy::StrictContained,
            },
        },
    }
}

#[cfg(feature = "silero-vad")]
pub(crate) fn resolve_silero_model_path(vad: &VadConfig) -> Result<PathBuf, NativeWhisperxError> {
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

#[cfg(feature = "pyannote-vad")]
pub(crate) fn resolve_pyannote_vad_model_path(
    vad: &VadConfig,
) -> Result<PathBuf, NativeWhisperxError> {
    let Some(model_bundle) = &vad.model_bundle else {
        return Err(NativeWhisperxError::InvalidConfig(
            "native pyannote VAD requires --vad-model-bundle or VadConfig.model_bundle".to_string(),
        ));
    };
    let path = if model_bundle.is_dir() {
        model_bundle.join(vad.model_file.as_deref().unwrap_or("segmentation.onnx"))
    } else if model_bundle
        .extension()
        .and_then(|extension| extension.to_str())
        == Some("onnx")
    {
        model_bundle.clone()
    } else {
        model_bundle.join(vad.model_file.as_deref().unwrap_or("segmentation.onnx"))
    };
    if !path.is_file() {
        return Err(NativeWhisperxError::InvalidConfig(format!(
            "pyannote VAD model path `{}` does not exist or is not a file",
            path.display()
        )));
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::WhisperxDecodeConfig;

    #[test]
    fn native_temperature_fallback_schedule_uses_whisperx_threshold_defaults() {
        let request = build_native_request_config(&AsrConfig {
            decode: WhisperxDecodeConfig {
                temperature: vec![0.0, 0.2],
                ..WhisperxDecodeConfig::default()
            },
            ..AsrConfig::default()
        })
        .expect("temperature fallback schedule should map");

        assert_eq!(
            request.decode.search.temperature_schedule,
            [0.0, f64::from(0.2_f32)]
        );
        assert_eq!(request.decode.min_average_log_probability, Some(-1.0));
        assert_eq!(request.decode.max_no_speech_probability, Some(0.6));
        assert_eq!(request.decode.max_compression_ratio, Some(2.4));
    }

    #[test]
    fn native_decode_thresholds_map_to_request_scoped_fallback_controls() {
        let request = build_native_request_config(&AsrConfig {
            decode: WhisperxDecodeConfig {
                compression_ratio_threshold: Some(2.4),
                logprob_threshold: Some(-1.0),
                no_speech_threshold: Some(0.6),
                ..WhisperxDecodeConfig::default()
            },
            ..AsrConfig::default()
        })
        .expect("fallback thresholds should map");

        assert_eq!(
            request.decode.min_average_log_probability,
            Some(f64::from(-1.0_f32))
        );
        assert_eq!(
            request.decode.max_no_speech_probability,
            Some(f64::from(0.6_f32))
        );
        assert_eq!(
            request.decode.max_compression_ratio,
            Some(f64::from(2.4_f32))
        );
    }

    #[test]
    fn default_native_decode_request_preserves_unset_fallback_controls() {
        let request = build_native_request_config(&AsrConfig::default())
            .expect("default native decode request should map");

        assert_eq!(request.decode.search.temperature_schedule, [0.0]);
        assert_eq!(request.decode.min_average_log_probability, None);
        assert_eq!(request.decode.max_no_speech_probability, None);
        assert_eq!(request.decode.max_compression_ratio, None);
    }

    #[test]
    fn native_decode_thresholds_are_validated_before_model_loading() {
        for (decode, expected) in [
            (
                WhisperxDecodeConfig {
                    compression_ratio_threshold: Some(0.0),
                    ..WhisperxDecodeConfig::default()
                },
                "--compression_ratio_threshold",
            ),
            (
                WhisperxDecodeConfig {
                    logprob_threshold: Some(f32::NAN),
                    ..WhisperxDecodeConfig::default()
                },
                "--logprob_threshold",
            ),
            (
                WhisperxDecodeConfig {
                    no_speech_threshold: Some(1.1),
                    ..WhisperxDecodeConfig::default()
                },
                "--no_speech_threshold",
            ),
        ] {
            let error = build_native_request_config(&AsrConfig {
                decode,
                ..AsrConfig::default()
            })
            .expect_err("invalid fallback threshold should be rejected");

            assert!(error.to_string().contains(expected));
        }
    }

    #[test]
    fn model_reuse_does_not_invent_asr_model_load_duration() {
        let mut observer = PhaseTimingObserver::default();
        observer.observe(TranscriptionPipelineEvent::ModelReuse {
            stage: "asr".to_string(),
            provider: "candle-whisper".to_string(),
            model_id: "small".to_string(),
        });
        let mut diagnostics = Vec::new();

        observer.append_diagnostics(&mut diagnostics);

        assert!(!diagnostics
            .iter()
            .any(|value| value.starts_with("phaseAsrModelLoadSeconds=")));
    }
}

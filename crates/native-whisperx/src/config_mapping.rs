#[cfg(any(feature = "pyannote-vad", feature = "silero-vad"))]
use std::path::PathBuf;
use std::time::Instant;

#[cfg(feature = "pyannote-vad")]
use crate::silero_vad::{PyannoteVadOptions, PyannoteVadTranscriptionProvider};
#[cfg(feature = "silero-vad")]
use crate::silero_vad::{SileroVadOptions, SileroVadTranscriptionProvider};
use audio_analysis_transcription::{
    run_transcription_pipeline_with_observer, AlignmentOptions, CandleWhisperOptions,
    CandleWhisperTranscriber, CtcForcedAligner, DiarizationOptions, ForcedAlignmentProvider,
    NativeDevicePreference, SpeakerAssignmentPolicy, SpeakerDiarizationOptions,
    TranscriptDiarizationProvider, TranscriptionOutputOptions, TranscriptionPipelineEvent,
    TranscriptionPipelineObserver, TranscriptionPipelineRequest, TranscriptionPipelineResponse,
    TranscriptionProviderSelection, TranscriptionSource,
    TranscriptionTask as UpstreamTranscriptionTask, TranscriptionVadProvider, VadOptions,
    WhisperXCommandOptions, WhisperXDevice,
};

use crate::config::{
    is_pyannote_diarization_model, AlignmentConfig, AsrConfig, AsrProvider, AssignmentPolicy,
    DevicePreference, DiarizationConfig, InputSource, NativeWhisperxConfig, NativeWhisperxError,
    SegmentResolution, TranscriptionTask, VadConfig, VadMethod,
};
#[cfg(feature = "diarization")]
use crate::native_diarization_provider;
use crate::output::expand_output_format;

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
    if is_pyannote && diarization.model_bundle.is_none() {
        return Err(NativeWhisperxError::InvalidConfig(
            "native pyannote diarization requires an explicit modelBundle".to_string(),
        ));
    }
    #[cfg(not(feature = "pyannote-diarization"))]
    if is_pyannote {
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
    let mut unsupported = Vec::new();
    if asr.compute_type.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--compute_type",
            reason: "Candle Whisper does not expose a compute type or quantization selector",
        });
    }
    if asr.device_index.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--device_index",
            reason: "native device resolution currently selects the default device for the requested backend",
        });
    }

    let decode = &asr.decode;
    if !decode.temperature.is_empty() && !is_native_greedy_temperature(&decode.temperature) {
        unsupported.push(UnsupportedNativeControl {
            flag: "--temperature",
            reason: "native decode currently supports deterministic greedy temperature 0 only; sampling and fallback schedules require upstream decode APIs",
        });
    }
    if decode.best_of.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--best_of",
            reason: "best-of requires sampling candidate generation that the native backend does not expose",
        });
    }
    if decode.beam_size.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--beam_size",
            reason: "beam search is not exposed by the native Candle Whisper backend",
        });
    }
    if decode.patience.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--patience",
            reason: "beam patience only applies to beam search, which is not exposed by the native backend",
        });
    }
    if decode.length_penalty.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--length_penalty",
            reason: "length penalty only applies to beam ranking, which is not exposed by the native backend",
        });
    }
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
    if decode.compression_ratio_threshold.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--compression_ratio_threshold",
            reason:
                "fallback thresholds require per-candidate compression scoring from the decoder",
        });
    }
    if decode.logprob_threshold.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--logprob_threshold",
            reason: "fallback thresholds require token log probability summaries from the decoder",
        });
    }
    if decode.no_speech_threshold.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--no_speech_threshold",
            reason: "no-speech thresholding requires native no-speech probability output",
        });
    }
    if decode.threads.is_some() {
        unsupported.push(UnsupportedNativeControl {
            flag: "--threads",
            reason: "the native backend does not expose a per-request decoder thread-count control",
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

fn is_native_greedy_temperature(temperature: &[f32]) -> bool {
    temperature
        .iter()
        .all(|value| value.is_finite() && *value == 0.0)
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

pub(crate) fn run_native_with_selected_vad(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    match config.vad.method {
        VadMethod::Silero => {
            #[cfg(feature = "silero-vad")]
            {
                let mut vad_provider = build_silero_vad_provider(&config.vad)?;
                run_native_with_custom_vad(request, config, &mut vad_provider)
            }
            #[cfg(not(feature = "silero-vad"))]
            {
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
                run_native_with_custom_vad(request, config, &mut vad_provider)
            }
            #[cfg(not(feature = "pyannote-vad"))]
            {
                let _ = (request, config);
                Err(NativeWhisperxError::InvalidConfig(
                    "native pyannote VAD requires the pyannote-vad feature".to_string(),
                ))
            }
        }
        VadMethod::Energy => {
            let _ = request;
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
            let mut diarizer = native_diarization_provider(config)?;
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

pub(crate) fn run_native_with_optional_alignment(
    request: TranscriptionPipelineRequest,
    vad_provider: &mut dyn TranscriptionVadProvider,
    asr_provider: &mut CandleWhisperTranscriber,
    #[cfg_attr(not(feature = "diarization"), allow(unused_variables))] diarization_provider: Option<
        &mut dyn TranscriptDiarizationProvider,
    >,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    let mut observer = PhaseTimingObserver::default();
    if request.alignment.enabled {
        let mut aligner = CtcForcedAligner {
            options: request.alignment.clone(),
        };
        return run_transcription_pipeline_with_observer(
            request,
            vad_provider,
            asr_provider,
            Some(&mut aligner as &mut dyn ForcedAlignmentProvider),
            diarization_provider,
            &mut observer,
        )
        .map(|mut response| {
            observer.append_diagnostics(&mut response.diagnostics);
            response
        })
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()));
    }

    run_transcription_pipeline_with_observer(
        request,
        vad_provider,
        asr_provider,
        None,
        diarization_provider,
        &mut observer,
    )
    .map(|mut response| {
        observer.append_diagnostics(&mut response.diagnostics);
        response
    })
    .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
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
                model_bundle: asr.whisper_bundle.clone(),
                model_dir: asr.model_dir.clone(),
                model_cache_only: asr.model_cache_only,
                batch_chunks: asr.batch_chunks,
                max_batch_size: asr.max_batch_size,
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
                model_cache_only: false,
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
        interpolate_method: alignment.interpolate_method,
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

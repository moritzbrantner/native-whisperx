//! Single-input transcription execution against native or delegated providers.

#[cfg(feature = "diarization")]
use audio_analysis_transcription::TranscriptDiarizationProvider;
use audio_analysis_transcription::{
    transcribe, CandleWhisperTranscriber, EnergyVadTranscriptionProvider,
    ReusableCandleWhisperTranscriber, TranscriptionPipelineRequest, TranscriptionPipelineResponse,
    TranscriptionProviderSelection,
};

use crate::config::{AsrProvider, NativeWhisperxConfig, NativeWhisperxError};
use crate::config_mapping::run_native_with_optional_alignment_and_progress;
use crate::workflow::NativeProgressContext;

#[allow(dead_code)]
pub(crate) fn run_with_reusable_asr(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
    vad_provider: &mut EnergyVadTranscriptionProvider,
    asr_provider: &mut ReusableCandleWhisperTranscriber,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    run_with_reusable_asr_and_progress(request, config, vad_provider, asr_provider, None)
}

pub(crate) fn run_with_reusable_asr_and_progress(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
    vad_provider: &mut EnergyVadTranscriptionProvider,
    asr_provider: &mut ReusableCandleWhisperTranscriber,
    progress: Option<NativeProgressContext<'_>>,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    #[cfg(feature = "diarization")]
    {
        let mut diarizer = crate::native_diarization_provider(config)?;
        let diarization_provider = request
            .diarization
            .enabled
            .then_some(&mut diarizer as &mut dyn TranscriptDiarizationProvider);
        run_native_with_optional_alignment_and_progress(
            request,
            vad_provider,
            asr_provider,
            diarization_provider,
            progress,
        )
    }

    #[cfg(not(feature = "diarization"))]
    {
        let _ = config;
        run_native_with_optional_alignment_and_progress(
            request,
            vad_provider,
            asr_provider,
            None,
            progress,
        )
    }
}

pub(crate) fn run_with_phase_observer(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    run_with_progress_observer(request, config, None)
}

pub(crate) fn run_with_progress_observer(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
    progress: Option<NativeProgressContext<'_>>,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    if config.asr.provider != AsrProvider::Native {
        return transcribe(request)
            .map_err(|error| NativeWhisperxError::Transcription(error.to_string()));
    }

    let TranscriptionProviderSelection::CandleWhisper(options) = &request.provider else {
        return transcribe(request)
            .map_err(|error| NativeWhisperxError::Transcription(error.to_string()));
    };
    let mut vad = EnergyVadTranscriptionProvider;
    let mut asr_provider = CandleWhisperTranscriber::new(options.clone());

    #[cfg(feature = "diarization")]
    {
        let mut diarizer = crate::native_diarization_provider(config)?;
        let diarization_provider = request
            .diarization
            .enabled
            .then_some(&mut diarizer as &mut dyn TranscriptDiarizationProvider);
        run_native_with_optional_alignment_and_progress(
            request,
            &mut vad,
            &mut asr_provider,
            diarization_provider,
            progress,
        )
    }

    #[cfg(not(feature = "diarization"))]
    {
        run_native_with_optional_alignment_and_progress(
            request,
            &mut vad,
            &mut asr_provider,
            None,
            progress,
        )
    }
}

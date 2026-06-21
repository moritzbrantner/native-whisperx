use std::time::Instant;

#[cfg(feature = "diarization")]
use audio_analysis_transcription::TranscriptDiarizationProvider;
use audio_analysis_transcription::{
    transcribe, CandleWhisperTranscriber, EnergyVadTranscriptionProvider,
    TranscriptionPipelineRequest, TranscriptionPipelineResponse, TranscriptionProviderSelection,
};

use crate::config::{
    AsrProvider, NativeWhisperxConfig, NativeWhisperxError, NativeWhisperxReport, VadMethod,
};
use crate::config_mapping::{
    build_transcription_request, run_native_with_optional_alignment, run_native_with_selected_vad,
};
use crate::output::write_outputs_with_options;
use crate::report::{append_native_alignment_diagnostics, append_native_diarization_diagnostics};

pub fn run(config: NativeWhisperxConfig) -> Result<NativeWhisperxReport, NativeWhisperxError> {
    let run_started = Instant::now();
    let request = build_transcription_request(&config)?;
    let mut response = if config.asr.provider == AsrProvider::Native && config.translation.enabled {
        crate::run_native_with_translation(request, &config)?
    } else if config.asr.provider == AsrProvider::Native
        && matches!(config.vad.method, VadMethod::Silero | VadMethod::Pyannote)
    {
        run_native_with_selected_vad(request, &config)?
    } else {
        run_with_phase_observer(request, &config)?
    };
    append_native_alignment_diagnostics(&mut response, &config);
    append_native_diarization_diagnostics(&mut response, &config);
    crate::save_draft_speakers_from_response(&mut response, &config)?;
    let output_started = Instant::now();
    let output_files = write_outputs_with_options(
        &response,
        &config.output,
        config.alignment.return_char_alignments,
    )?;
    response.diagnostics.push(format!(
        "phaseOutputSeconds={:.6}",
        output_started.elapsed().as_secs_f64()
    ));
    response.diagnostics.push(format!(
        "phaseNativeTotalSeconds={:.6}",
        run_started.elapsed().as_secs_f64()
    ));
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

pub(crate) fn run_with_phase_observer(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
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
        run_native_with_optional_alignment(
            request,
            &mut vad,
            &mut asr_provider,
            diarization_provider,
        )
    }

    #[cfg(not(feature = "diarization"))]
    {
        run_native_with_optional_alignment(request, &mut vad, &mut asr_provider, None)
    }
}

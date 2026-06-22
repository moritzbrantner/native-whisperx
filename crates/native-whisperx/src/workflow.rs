use std::time::Instant;

#[cfg(feature = "diarization")]
use audio_analysis_transcription::TranscriptDiarizationProvider;
use audio_analysis_transcription::{
    transcribe, CandleWhisperTranscriber, EnergyVadTranscriptionProvider,
    ReusableCandleWhisperTranscriber, TranscriptionPipelineRequest, TranscriptionPipelineResponse,
    TranscriptionProviderSelection,
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
    if should_reuse_native_asr_provider(&configs) {
        return run_many_reusing_native_provider(configs);
    }
    configs.into_iter().map(run).collect()
}

pub fn run_many_reusing_native_provider(
    configs: Vec<NativeWhisperxConfig>,
) -> Result<Vec<NativeWhisperxReport>, NativeWhisperxError> {
    let mut reports = Vec::with_capacity(configs.len());
    let mut reusable_asr: Option<ReusableCandleWhisperTranscriber> = None;

    for config in configs {
        let run_started = Instant::now();
        let request = build_transcription_request(&config)?;
        let TranscriptionProviderSelection::CandleWhisper(options) = &request.provider else {
            return Err(NativeWhisperxError::InvalidConfig(
                "native multi-input reuse requires the Candle Whisper native provider".to_string(),
            ));
        };

        let reused_provider = reusable_asr
            .as_ref()
            .is_some_and(|provider| provider.options == *options);
        if !reused_provider {
            reusable_asr = Some(ReusableCandleWhisperTranscriber::new(options.clone()));
        }
        let asr_provider = reusable_asr
            .as_mut()
            .expect("native ASR provider should be initialized");
        let mut vad = EnergyVadTranscriptionProvider;
        let mut response = run_with_reusable_asr(request, &config, &mut vad, asr_provider)?;
        response.diagnostics.push(if reused_provider {
            "nativeMultiInputAsrProvider=reused".to_string()
        } else {
            "nativeMultiInputAsrProvider=loaded".to_string()
        });
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
        reports.push(NativeWhisperxReport {
            response,
            output_files,
        });
    }

    Ok(reports)
}

fn should_reuse_native_asr_provider(configs: &[NativeWhisperxConfig]) -> bool {
    configs.len() > 1
        && configs.iter().all(|config| {
            config.asr.provider == AsrProvider::Native
                && !config.translation.enabled
                && matches!(config.vad.method, VadMethod::Energy)
        })
}

fn run_with_reusable_asr(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
    vad_provider: &mut EnergyVadTranscriptionProvider,
    asr_provider: &mut ReusableCandleWhisperTranscriber,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    #[cfg(feature = "diarization")]
    {
        let mut diarizer = crate::native_diarization_provider(config)?;
        let diarization_provider = request
            .diarization
            .enabled
            .then_some(&mut diarizer as &mut dyn TranscriptDiarizationProvider);
        return run_native_with_optional_alignment(
            request,
            vad_provider,
            asr_provider,
            diarization_provider,
        );
    }

    #[cfg(not(feature = "diarization"))]
    {
        let _ = config;
        run_native_with_optional_alignment(request, vad_provider, asr_provider, None)
    }
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

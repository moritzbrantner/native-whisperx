//! A transcription session that keeps loaded native models across runs.

#[cfg(feature = "diarization")]
use audio_analysis_transcription::TranscriptDiarizationProvider;
use audio_analysis_transcription::{
    RequestConfiguredCandleWhisperTranscriber, TranscriptionPipelineRequest,
    TranscriptionPipelineResponse, TranscriptionProviderSelection, VoiceActivityDetector,
};

use crate::config::{
    AsrProvider, NativeWhisperxConfig, NativeWhisperxError, NativeWhisperxReport,
    SelectedMediaError, VadConfig,
};
use crate::config_mapping::{
    build_native_request_config_for_workflow, build_native_vad_provider,
    run_native_with_optional_alignment_and_progress,
};

use super::{
    run_one_with_control_selected, validate_product_provider, CancellationHandle,
    FiniteTranscriptionOutcome, NativeProgressContext, NoopTranscriptionProgressObserver,
    TranscriptionProgressObserver,
};

/// Keeps the native ASR model and VAD model loaded across finite runs.
///
/// Each [`TranscriptionSession::run`] call takes a complete
/// [`NativeWhisperxConfig`]. Native runs without translation reuse the loaded
/// Whisper model while the ASR provider setup (model, device, compute type,
/// bundle) stays the same, and the loaded VAD model while the VAD config stays
/// the same; a change rebuilds only the provider that changed. Other runs
/// (translation) use the ordinary one-shot workflow. Decode options, language,
/// alignment, diarization and outputs may differ per run.
#[derive(Default)]
pub struct TranscriptionSession {
    asr: Option<RequestConfiguredCandleWhisperTranscriber>,
    vad: Option<(VadConfig, Box<dyn VoiceActivityDetector>)>,
}

impl std::fmt::Debug for TranscriptionSession {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TranscriptionSession")
            .field("asr", &self.asr)
            .field("vad", &self.vad.as_ref().map(|(config, _)| config))
            .finish()
    }
}

impl TranscriptionSession {
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs one finite workflow, reusing loaded models from earlier runs.
    pub fn run(
        &mut self,
        config: NativeWhisperxConfig,
    ) -> Result<NativeWhisperxReport, NativeWhisperxError> {
        let mut observer = NoopTranscriptionProgressObserver;
        self.run_with_observer(config, &mut observer)
    }

    /// Observer-enabled form of [`TranscriptionSession::run`].
    pub fn run_with_observer(
        &mut self,
        config: NativeWhisperxConfig,
        observer: &mut dyn TranscriptionProgressObserver,
    ) -> Result<NativeWhisperxReport, NativeWhisperxError> {
        validate_product_provider(&config)?;
        let cancellation = CancellationHandle::new();
        match run_one_with_control_selected(
            config,
            None,
            0,
            1,
            observer,
            true,
            &cancellation,
            Some(self),
        )
        .map_err(SelectedMediaError::into_native)?
        {
            FiniteTranscriptionOutcome::Completed(report) => Ok(*report),
            FiniteTranscriptionOutcome::Cancelled(_) => {
                unreachable!("a session run uses an uncancelled handle")
            }
        }
    }

    /// Whether this session's providers can run the resolved request.
    pub(crate) fn handles(
        config: &NativeWhisperxConfig,
        request: &TranscriptionPipelineRequest,
    ) -> bool {
        config.asr.provider == AsrProvider::Native
            && !config.translation.enabled
            && matches!(
                request.provider,
                TranscriptionProviderSelection::CandleWhisper(_)
            )
    }

    /// Runs a resolved native request through the session-owned providers.
    pub(crate) fn execute(
        &mut self,
        request: TranscriptionPipelineRequest,
        config: &NativeWhisperxConfig,
        progress: Option<NativeProgressContext<'_>>,
    ) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
        let TranscriptionProviderSelection::CandleWhisper(options) = &request.provider else {
            return Err(NativeWhisperxError::InvalidConfig(
                "transcription session requires the Candle Whisper native provider".to_string(),
            ));
        };
        let request_config = build_native_request_config_for_workflow(config)?;
        match self.asr.as_mut() {
            Some(asr) if asr.options() == options => asr.set_request_config(request_config),
            _ => {
                self.asr = Some(RequestConfiguredCandleWhisperTranscriber::reusable(
                    options.clone(),
                    request_config,
                ))
            }
        }
        if self
            .vad
            .as_ref()
            .is_none_or(|(vad_config, _)| vad_config != &config.vad)
        {
            self.vad = Some((config.vad.clone(), build_native_vad_provider(&config.vad)?));
        }
        let asr = self
            .asr
            .as_mut()
            .expect("session ASR provider was just set");
        let (_, vad) = self
            .vad
            .as_mut()
            .expect("session VAD provider was just set");

        #[cfg(feature = "diarization")]
        {
            let mut diarizer = crate::native_diarization_provider(config)?;
            let diarization_provider = request
                .diarization
                .enabled
                .then_some(&mut diarizer as &mut dyn TranscriptDiarizationProvider);
            run_native_with_optional_alignment_and_progress(
                request,
                vad.as_mut(),
                asr,
                diarization_provider,
                progress,
            )
        }

        #[cfg(not(feature = "diarization"))]
        {
            run_native_with_optional_alignment_and_progress(
                request,
                vad.as_mut(),
                asr,
                None,
                progress,
            )
        }
    }
}

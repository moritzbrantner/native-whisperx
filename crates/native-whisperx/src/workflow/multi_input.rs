//! Multi-input transcription runs that reuse native provider state where possible.

use std::time::Instant;

use audio_analysis_transcription::{
    EnergyVadTranscriptionProvider, RequestConfiguredCandleWhisperTranscriber,
    TranscriptionProviderSelection,
};

use crate::config::{
    resolve_automatic_workflow_selection, AsrProvider, NativeWhisperxConfig, NativeWhisperxError,
    NativeWhisperxReport, NativeWorkflowSelectionReport, SelectedMediaError, SelectedMediaInput,
    VadMethod,
};
use crate::config_mapping::{
    build_native_request_config, build_transcription_request_from_resolved_config_with_selected_media,
    validate_pre_resolution_support, validate_request_config, validate_selected_media_source,
};
use crate::report::{
    append_automatic_workflow_selection_diagnostics, append_native_alignment_diagnostics,
    append_native_diarization_diagnostics,
};

use super::execution::run_with_reusable_asr_and_progress;
use super::{
    ensure_active, progress_input_path, run_one_with_control_selected,
    validate_selected_media_config, write_outputs_with_control, CancellationHandle,
    FiniteCancellation, FiniteTranscriptionOutcome, MultiInputTranscriptionOutcome,
    NativeProgressContext, NoopTranscriptionProgressObserver, ProgressTaskTracker,
    TranscriptionProgressEvent, TranscriptionProgressObserver, UnfinishedTranscription,
};

pub fn run_many(
    configs: Vec<NativeWhisperxConfig>,
) -> Result<Vec<NativeWhisperxReport>, NativeWhisperxError> {
    let mut observer = NoopTranscriptionProgressObserver;
    run_many_with_observer(configs, &mut observer)
}

pub fn run_many_with_observer(
    configs: Vec<NativeWhisperxConfig>,
    observer: &mut dyn TranscriptionProgressObserver,
) -> Result<Vec<NativeWhisperxReport>, NativeWhisperxError> {
    run_many_with_optional_selected_media(configs, None, observer)
        .map_err(SelectedMediaError::into_native)
}

/// Transcribes path inputs using the same explicit zero-based audio-stream ordinal.
///
/// Every config must use [`crate::InputSource::Path`]. Existing [`run_many`]
/// callers retain default-stream selection and an unchanged input enum.
pub fn run_many_selected_media(
    configs: Vec<NativeWhisperxConfig>,
    selected_media: SelectedMediaInput,
) -> Result<Vec<NativeWhisperxReport>, SelectedMediaError> {
    let mut observer = NoopTranscriptionProgressObserver;
    run_many_selected_media_with_observer(configs, selected_media, &mut observer)
}

/// Observer-enabled form of [`run_many_selected_media`].
pub fn run_many_selected_media_with_observer(
    configs: Vec<NativeWhisperxConfig>,
    selected_media: SelectedMediaInput,
    observer: &mut dyn TranscriptionProgressObserver,
) -> Result<Vec<NativeWhisperxReport>, SelectedMediaError> {
    run_many_with_optional_selected_media(configs, Some(selected_media), observer)
}

fn run_many_with_optional_selected_media(
    configs: Vec<NativeWhisperxConfig>,
    selected_media: Option<SelectedMediaInput>,
    observer: &mut dyn TranscriptionProgressObserver,
) -> Result<Vec<NativeWhisperxReport>, SelectedMediaError> {
    let cancellation = CancellationHandle::new();
    match run_many_with_control_selected(configs, selected_media, observer, &cancellation)? {
        MultiInputTranscriptionOutcome::Completed(reports) => Ok(reports),
        MultiInputTranscriptionOutcome::Cancelled { .. } => {
            unreachable!("the compatibility multi-input entry point uses an uncancelled handle")
        }
    }
}

/// Runs a Multi-Input Transcription Run with progress and cooperative control.
pub fn run_many_with_control(
    configs: Vec<NativeWhisperxConfig>,
    observer: &mut dyn TranscriptionProgressObserver,
    cancellation: &CancellationHandle,
) -> Result<MultiInputTranscriptionOutcome, NativeWhisperxError> {
    run_many_with_control_selected(configs, None, observer, cancellation)
        .map_err(SelectedMediaError::into_native)
}

/// Runs selected-media Multi-Input Transcription with cooperative control.
///
/// The shared audio ordinal follows the same early-decode route as
/// [`run_many_selected_media`], while cancellation retains completed and
/// unfinished input reporting from [`run_many_with_control`].
pub fn run_many_selected_media_with_control(
    configs: Vec<NativeWhisperxConfig>,
    selected_media: SelectedMediaInput,
    observer: &mut dyn TranscriptionProgressObserver,
    cancellation: &CancellationHandle,
) -> Result<MultiInputTranscriptionOutcome, SelectedMediaError> {
    run_many_with_control_selected(configs, Some(selected_media), observer, cancellation)
}

fn run_many_with_control_selected(
    configs: Vec<NativeWhisperxConfig>,
    selected_media: Option<SelectedMediaInput>,
    observer: &mut dyn TranscriptionProgressObserver,
    cancellation: &CancellationHandle,
) -> Result<MultiInputTranscriptionOutcome, SelectedMediaError> {
    let total_files = configs.len();
    let run_started = Instant::now();
    observer.observe(TranscriptionProgressEvent::RunStart { total_files });
    if should_reuse_native_asr_provider(&configs) {
        let outcome = run_many_reusing_native_provider_with_control(
            configs,
            selected_media,
            observer,
            cancellation,
        )?;
        if matches!(outcome, MultiInputTranscriptionOutcome::Completed(_)) {
            observer.observe(TranscriptionProgressEvent::RunEnd {
                total_files,
                duration_seconds: run_started.elapsed().as_secs_f64(),
            });
        }
        return Ok(outcome);
    }
    let mut reports = Vec::with_capacity(total_files);
    let inputs = configs.iter().map(progress_input_path).collect::<Vec<_>>();
    for (file_index, config) in configs.into_iter().enumerate() {
        if cancellation.is_cancelled() {
            let input = inputs[file_index].clone();
            let cancellation = FiniteCancellation::new(file_index, input.clone(), None);
            observer.observe(TranscriptionProgressEvent::Cancelled {
                file_index,
                input,
                task: None,
                duration_seconds: run_started.elapsed().as_secs_f64(),
            });
            return Ok(MultiInputTranscriptionOutcome::Cancelled {
                completed: reports,
                cancellation,
                unfinished: unfinished_inputs(&inputs, file_index),
            });
        }
        match run_one_with_control_selected(
            config,
            selected_media,
            file_index,
            total_files,
            observer,
            false,
            cancellation,
        )? {
            FiniteTranscriptionOutcome::Completed(report) => reports.push(*report),
            FiniteTranscriptionOutcome::Cancelled(cancellation) => {
                return Ok(MultiInputTranscriptionOutcome::Cancelled {
                    completed: reports,
                    cancellation,
                    unfinished: unfinished_inputs(&inputs, file_index),
                });
            }
        }
    }
    observer.observe(TranscriptionProgressEvent::RunEnd {
        total_files,
        duration_seconds: run_started.elapsed().as_secs_f64(),
    });
    Ok(MultiInputTranscriptionOutcome::Completed(reports))
}

pub fn run_many_reusing_native_provider(
    configs: Vec<NativeWhisperxConfig>,
) -> Result<Vec<NativeWhisperxReport>, NativeWhisperxError> {
    let mut observer = NoopTranscriptionProgressObserver;
    run_many_reusing_native_provider_with_observer(configs, &mut observer)
}

pub fn run_many_reusing_native_provider_with_observer(
    configs: Vec<NativeWhisperxConfig>,
    observer: &mut dyn TranscriptionProgressObserver,
) -> Result<Vec<NativeWhisperxReport>, NativeWhisperxError> {
    let cancellation = CancellationHandle::new();
    match run_many_reusing_native_provider_with_control(configs, None, observer, &cancellation)
        .map_err(SelectedMediaError::into_native)?
    {
        MultiInputTranscriptionOutcome::Completed(reports) => Ok(reports),
        MultiInputTranscriptionOutcome::Cancelled { .. } => {
            unreachable!("the compatibility reusable entry point uses an uncancelled handle")
        }
    }
}

fn run_many_reusing_native_provider_with_control(
    configs: Vec<NativeWhisperxConfig>,
    selected_media: Option<SelectedMediaInput>,
    observer: &mut dyn TranscriptionProgressObserver,
    cancellation: &CancellationHandle,
) -> Result<MultiInputTranscriptionOutcome, SelectedMediaError> {
    let total_files = configs.len();
    let mut reports = Vec::with_capacity(configs.len());
    let mut reusable_asr: Option<RequestConfiguredCandleWhisperTranscriber> = None;
    let inputs = configs.iter().map(progress_input_path).collect::<Vec<_>>();

    for (file_index, config) in configs.into_iter().enumerate() {
        let run_started = Instant::now();
        let input = progress_input_path(&config);
        if cancellation.is_cancelled() {
            let cancellation = FiniteCancellation::new(file_index, input.clone(), None);
            observer.observe(TranscriptionProgressEvent::Cancelled {
                file_index,
                input,
                task: None,
                duration_seconds: 0.0,
            });
            return Ok(MultiInputTranscriptionOutcome::Cancelled {
                completed: reports,
                cancellation,
                unfinished: unfinished_inputs(&inputs, file_index),
            });
        }
        observer.observe(TranscriptionProgressEvent::FileStart {
            file_index,
            total_files,
            input: input.clone(),
        });
        let mut task_tracker = ProgressTaskTracker::default();
        let result: Result<NativeWhisperxReport, SelectedMediaError> = (|| {
            ensure_active(cancellation)?;
            validate_pre_resolution_support(&config)?;
            validate_selected_media_config(&config, selected_media)?;
            validate_request_config(&config)?;
            validate_selected_media_source(&config, selected_media)?;
            let selection = resolve_automatic_workflow_selection(&config)?;
            let resolved_config = selection.config.clone();
            ensure_active(cancellation)?;
            let request = build_transcription_request_from_resolved_config_with_selected_media(
                &resolved_config,
                selected_media,
            )?;
            let TranscriptionProviderSelection::CandleWhisper(options) = &request.provider else {
                return Err(NativeWhisperxError::InvalidConfig(
                    "native multi-input reuse requires the Candle Whisper native provider"
                        .to_string(),
                )
                .into());
            };

            let reused_provider = reusable_asr
                .as_ref()
                .is_some_and(|provider| provider.options() == options);
            let request_config = build_native_request_config(&resolved_config.asr)?;
            let asr_provider = if reused_provider {
                let provider = reusable_asr
                    .as_mut()
                    .expect("reused native ASR provider should be initialized");
                provider.set_request_config(request_config);
                provider
            } else {
                super::mark_provider_setup();
                reusable_asr.insert(RequestConfiguredCandleWhisperTranscriber::reusable(
                    options.clone(),
                    request_config,
                ))
            };
            let mut vad = EnergyVadTranscriptionProvider;
            let mut response = run_with_reusable_asr_and_progress(
                request,
                &resolved_config,
                &mut vad,
                asr_provider,
                Some(NativeProgressContext {
                    observer,
                    file_index,
                    task_tracker: &mut task_tracker,
                    cancellation,
                }),
            )?;
            response.diagnostics.push(if reused_provider {
                "nativeMultiInputAsrProvider=reused".to_string()
            } else {
                "nativeMultiInputAsrProvider=loaded".to_string()
            });
            append_automatic_workflow_selection_diagnostics(&mut response, &selection);
            append_native_alignment_diagnostics(&mut response, &resolved_config);
            append_native_diarization_diagnostics(&mut response, &resolved_config);
            ensure_active(cancellation)?;
            crate::save_draft_speakers_from_response(&mut response, &resolved_config)?;
            ensure_active(cancellation)?;
            let (output_files, output_seconds) = write_outputs_with_control(
                &response,
                &resolved_config.output,
                resolved_config.alignment.return_char_alignments,
                file_index,
                observer,
                cancellation,
                &mut task_tracker,
            )?;
            response
                .diagnostics
                .push(format!("phaseOutputSeconds={:.6}", output_seconds));
            let total_seconds = run_started.elapsed().as_secs_f64();
            response
                .diagnostics
                .push(format!("phaseNativeTotalSeconds={:.6}", total_seconds));
            observer.observe(TranscriptionProgressEvent::FileEnd {
                file_index,
                total_files,
                input: input.clone(),
                duration_seconds: total_seconds,
            });
            Ok(NativeWhisperxReport::from_pipeline_response(
                response,
                output_files,
                NativeWorkflowSelectionReport::from_selection(&selection),
            ))
        })();

        if result.is_err() && cancellation.is_cancelled() {
            let cancelled =
                FiniteCancellation::new(file_index, input.clone(), task_tracker.current());
            observer.observe(TranscriptionProgressEvent::Cancelled {
                file_index,
                input,
                task: cancelled.task(),
                duration_seconds: run_started.elapsed().as_secs_f64(),
            });
            return Ok(MultiInputTranscriptionOutcome::Cancelled {
                completed: reports,
                cancellation: cancelled,
                unfinished: unfinished_inputs(&inputs, file_index),
            });
        }

        match result {
            Ok(report) => reports.push(report),
            Err(error) => {
                observer.observe(TranscriptionProgressEvent::Failure {
                    file_index,
                    input,
                    task: task_tracker.current(),
                    duration_seconds: run_started.elapsed().as_secs_f64(),
                    message: error.to_string(),
                });
                return Err(error);
            }
        }
    }

    Ok(MultiInputTranscriptionOutcome::Completed(reports))
}

fn unfinished_inputs(inputs: &[std::path::PathBuf], from: usize) -> Vec<UnfinishedTranscription> {
    inputs
        .iter()
        .enumerate()
        .skip(from)
        .map(|(file_index, input)| UnfinishedTranscription::new(file_index, input.clone()))
        .collect()
}

fn should_reuse_native_asr_provider(configs: &[NativeWhisperxConfig]) -> bool {
    configs.len() > 1
        && configs.iter().all(|config| {
            validate_pre_resolution_support(config).is_ok()
                && config.asr.provider == AsrProvider::Native
                && !config.translation.enabled
                && matches!(config.vad.method, VadMethod::Energy)
                && (!config.vad.selection.is_automatic() || !config.diarization.enabled)
        })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::{
        AlignmentConfig, AsrConfig, AsrProvider, DiarizationConfig, InputSource,
        NativeWhisperxConfig, OutputConfig, TranslationConfig, VadConfig, VadMethod,
    };

    use super::should_reuse_native_asr_provider;

    #[test]
    fn native_multi_input_reuse_is_limited_to_energy_vad_without_translation() {
        let first = native_config("first.wav");
        let second = native_config("second.wav");

        assert!(should_reuse_native_asr_provider(&[
            first.clone(),
            second.clone()
        ]));
        assert!(!should_reuse_native_asr_provider(std::slice::from_ref(
            &first
        )));
        assert!(!should_reuse_native_asr_provider(&[
            first.clone(),
            NativeWhisperxConfig {
                vad: VadConfig {
                    method: VadMethod::Silero,
                    ..VadConfig::default()
                },
                ..second.clone()
            }
        ]));
        assert!(!should_reuse_native_asr_provider(&[
            first.clone(),
            NativeWhisperxConfig {
                asr: AsrConfig {
                    provider: AsrProvider::ExternalWhisperX,
                    ..AsrConfig::default()
                },
                ..second.clone()
            }
        ]));
        assert!(!should_reuse_native_asr_provider(&[
            first,
            NativeWhisperxConfig {
                translation: TranslationConfig {
                    enabled: true,
                    model_id: Some("Helsinki-NLP/opus-mt-de-en".to_string()),
                    ..TranslationConfig::default()
                },
                ..second
            }
        ]));

        let mut invalid_q8 = native_config("q8.wav");
        invalid_q8.asr.compute_type = Some("int8".to_string());
        invalid_q8.alignment.enabled = false;
        assert!(!should_reuse_native_asr_provider(&[
            invalid_q8.clone(),
            invalid_q8,
        ]));
    }

    fn native_config(input: &str) -> NativeWhisperxConfig {
        NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from(input),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        }
    }
}

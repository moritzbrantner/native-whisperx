//! Multi-input transcription runs that reuse native provider state where possible.

use std::time::Instant;

use audio_analysis_transcription::{
    EnergyVadTranscriptionProvider, ReusableCandleWhisperTranscriber,
    TranscriptionProviderSelection,
};

use crate::config::{
    resolve_automatic_workflow_selection, AsrProvider, NativeWhisperxConfig, NativeWhisperxError,
    NativeWhisperxReport, NativeWorkflowSelectionReport, VadMethod,
};
use crate::config_mapping::build_transcription_request_from_resolved_config;
use crate::report::{
    append_automatic_workflow_selection_diagnostics, append_native_alignment_diagnostics,
    append_native_diarization_diagnostics,
};

use super::execution::run_with_reusable_asr_and_progress;
use super::{
    ensure_active, progress_input_path, run_one_with_control, write_outputs_with_control,
    CancellationHandle, FiniteCancellation, FiniteTranscriptionOutcome,
    MultiInputTranscriptionOutcome, NativeProgressContext, NoopTranscriptionProgressObserver,
    ProgressTaskTracker, TranscriptionProgressEvent, TranscriptionProgressObserver,
    UnfinishedTranscription,
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
    let cancellation = CancellationHandle::new();
    match run_many_with_control(configs, observer, &cancellation)? {
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
    let total_files = configs.len();
    let run_started = Instant::now();
    observer.observe(TranscriptionProgressEvent::RunStart { total_files });
    if should_reuse_native_asr_provider(&configs) {
        let outcome =
            run_many_reusing_native_provider_with_control(configs, observer, cancellation)?;
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
        match run_one_with_control(
            config,
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
    match run_many_reusing_native_provider_with_control(configs, observer, &cancellation)? {
        MultiInputTranscriptionOutcome::Completed(reports) => Ok(reports),
        MultiInputTranscriptionOutcome::Cancelled { .. } => {
            unreachable!("the compatibility reusable entry point uses an uncancelled handle")
        }
    }
}

fn run_many_reusing_native_provider_with_control(
    configs: Vec<NativeWhisperxConfig>,
    observer: &mut dyn TranscriptionProgressObserver,
    cancellation: &CancellationHandle,
) -> Result<MultiInputTranscriptionOutcome, NativeWhisperxError> {
    let total_files = configs.len();
    let mut reports = Vec::with_capacity(configs.len());
    let mut reusable_asr: Option<ReusableCandleWhisperTranscriber> = None;
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
        let result: Result<NativeWhisperxReport, NativeWhisperxError> = (|| {
            ensure_active(cancellation)?;
            let selection = resolve_automatic_workflow_selection(&config)?;
            let resolved_config = selection.config.clone();
            ensure_active(cancellation)?;
            let request = build_transcription_request_from_resolved_config(&resolved_config)?;
            let TranscriptionProviderSelection::CandleWhisper(options) = &request.provider else {
                return Err(NativeWhisperxError::InvalidConfig(
                    "native multi-input reuse requires the Candle Whisper native provider"
                        .to_string(),
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
            Ok(NativeWhisperxReport {
                response,
                output_files,
                workflow_selection: NativeWorkflowSelectionReport::from_selection(&selection),
            })
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
            resolve_automatic_workflow_selection(config)
                .map(|selection| {
                    let config = selection.config;
                    config.asr.provider == AsrProvider::Native
                        && !config.translation.enabled
                        && matches!(config.vad.method, VadMethod::Energy)
                })
                .unwrap_or(false)
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

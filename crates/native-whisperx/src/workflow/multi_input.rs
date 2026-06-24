//! Multi-input transcription runs that reuse native provider state where possible.

use std::time::Instant;

use audio_analysis_transcription::{
    EnergyVadTranscriptionProvider, ReusableCandleWhisperTranscriber,
    TranscriptionProviderSelection,
};

use crate::config::{
    resolve_automatic_workflow_selection, AsrProvider, NativeWhisperxConfig, NativeWhisperxError,
    NativeWhisperxReport, VadMethod,
};
use crate::config_mapping::build_transcription_request_from_resolved_config;
use crate::output::write_outputs_with_options;
use crate::report::{
    append_automatic_workflow_selection_diagnostics, append_native_alignment_diagnostics,
    append_native_diarization_diagnostics,
};

use super::execution::run_with_reusable_asr_and_progress;
use super::{
    progress_input_path, run_one_with_observer, NativeProgressContext,
    NoopTranscriptionProgressObserver, ProgressTaskTracker, TranscriptionProgressEvent,
    TranscriptionProgressObserver, TranscriptionProgressTask,
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
    let total_files = configs.len();
    let run_started = Instant::now();
    observer.observe(TranscriptionProgressEvent::RunStart { total_files });
    if should_reuse_native_asr_provider(&configs) {
        let reports = run_many_reusing_native_provider_with_observer(configs, observer)?;
        observer.observe(TranscriptionProgressEvent::RunEnd {
            total_files,
            duration_seconds: run_started.elapsed().as_secs_f64(),
        });
        return Ok(reports);
    }
    let mut reports = Vec::with_capacity(total_files);
    for (file_index, config) in configs.into_iter().enumerate() {
        reports.push(run_one_with_observer(
            config,
            file_index,
            total_files,
            observer,
            false,
        )?);
    }
    observer.observe(TranscriptionProgressEvent::RunEnd {
        total_files,
        duration_seconds: run_started.elapsed().as_secs_f64(),
    });
    Ok(reports)
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
    let total_files = configs.len();
    let mut reports = Vec::with_capacity(configs.len());
    let mut reusable_asr: Option<ReusableCandleWhisperTranscriber> = None;

    for (file_index, config) in configs.into_iter().enumerate() {
        let run_started = Instant::now();
        let input = progress_input_path(&config);
        observer.observe(TranscriptionProgressEvent::FileStart {
            file_index,
            total_files,
            input: input.clone(),
        });
        let mut task_tracker = ProgressTaskTracker::default();
        let result: Result<NativeWhisperxReport, NativeWhisperxError> = (|| {
            let selection = resolve_automatic_workflow_selection(&config)?;
            let resolved_config = selection.config.clone();
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
            crate::save_draft_speakers_from_response(&mut response, &resolved_config)?;
            let output_started = Instant::now();
            task_tracker.set_current(Some(TranscriptionProgressTask::Output));
            observer.observe(TranscriptionProgressEvent::TaskStart {
                file_index,
                task: TranscriptionProgressTask::Output,
            });
            let output_files = write_outputs_with_options(
                &response,
                &resolved_config.output,
                resolved_config.alignment.return_char_alignments,
            )?;
            let output_seconds = output_started.elapsed().as_secs_f64();
            observer.observe(TranscriptionProgressEvent::TaskEnd {
                file_index,
                task: TranscriptionProgressTask::Output,
                duration_seconds: output_seconds,
            });
            task_tracker.set_current(None);
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
            })
        })();

        if let Err(error) = &result {
            observer.observe(TranscriptionProgressEvent::Failure {
                file_index,
                input,
                task: task_tracker.current(),
                duration_seconds: run_started.elapsed().as_secs_f64(),
                message: error.to_string(),
            });
        }
        reports.push(result?);
    }

    Ok(reports)
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

//! User-facing transcription workflow orchestration and output reporting.

use std::time::Instant;

mod execution;
mod multi_input;
mod progress;

use crate::config::{
    resolve_automatic_workflow_selection, AsrProvider, NativeWhisperxConfig, NativeWhisperxError,
    NativeWhisperxReport, VadMethod,
};
use crate::config_mapping::{
    build_transcription_request, build_transcription_request_from_resolved_config,
    run_native_with_selected_vad_and_progress,
};
use crate::output::write_outputs_with_options;
use crate::report::{
    append_automatic_workflow_selection_diagnostics, append_native_alignment_diagnostics,
    append_native_diarization_diagnostics,
};

pub(crate) use execution::{run_with_phase_observer, run_with_progress_observer};
pub use multi_input::{run_many, run_many_reusing_native_provider, run_many_with_observer};
pub(crate) use progress::{NativeProgressContext, ProgressTaskTracker};
pub use progress::{
    NoopTranscriptionProgressObserver, TranscriptionProgressEvent, TranscriptionProgressObserver,
    TranscriptionProgressTask,
};

pub fn run_live_asr_window(
    config: NativeWhisperxConfig,
) -> Result<crate::TranscriptionPipelineResponse, NativeWhisperxError> {
    if config.asr.provider != AsrProvider::Native {
        return Err(NativeWhisperxError::InvalidConfig(
            "live-transcribe supports native ASR only".to_string(),
        ));
    }
    if config.translation.enabled {
        return Err(NativeWhisperxError::InvalidConfig(
            "live-transcribe does not support translation in the first live workflow".to_string(),
        ));
    }
    if config.alignment.enabled {
        return Err(NativeWhisperxError::InvalidConfig(
            "live-transcribe does not support alignment in the first live workflow".to_string(),
        ));
    }
    if config.diarization.enabled {
        return Err(NativeWhisperxError::InvalidConfig(
            "live-transcribe does not support diarization in the first live workflow".to_string(),
        ));
    }

    let request = build_transcription_request(&config)?;
    run_with_phase_observer(request, &config)
}

pub fn run(config: NativeWhisperxConfig) -> Result<NativeWhisperxReport, NativeWhisperxError> {
    let mut observer = NoopTranscriptionProgressObserver;
    run_with_observer(config, &mut observer)
}

pub fn run_with_observer(
    config: NativeWhisperxConfig,
    observer: &mut dyn TranscriptionProgressObserver,
) -> Result<NativeWhisperxReport, NativeWhisperxError> {
    run_one_with_observer(config, 0, 1, observer, true)
}

pub(crate) fn run_one_with_observer(
    config: NativeWhisperxConfig,
    file_index: usize,
    total_files: usize,
    observer: &mut dyn TranscriptionProgressObserver,
    emit_run_events: bool,
) -> Result<NativeWhisperxReport, NativeWhisperxError> {
    let run_started = Instant::now();
    let input = progress_input_path(&config);
    if emit_run_events {
        observer.observe(TranscriptionProgressEvent::RunStart { total_files });
    }
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
        let mut response = if resolved_config.asr.provider == AsrProvider::Native
            && resolved_config.translation.enabled
        {
            crate::run_native_with_translation_with_progress(
                request,
                &resolved_config,
                Some(NativeProgressContext {
                    observer,
                    file_index,
                    task_tracker: &mut task_tracker,
                }),
            )?
        } else if resolved_config.asr.provider == AsrProvider::Native
            && matches!(
                resolved_config.vad.method,
                VadMethod::Silero | VadMethod::Pyannote
            )
        {
            run_native_with_selected_vad_and_progress(
                request,
                &resolved_config,
                Some(NativeProgressContext {
                    observer,
                    file_index,
                    task_tracker: &mut task_tracker,
                }),
            )?
        } else {
            run_with_progress_observer(
                request,
                &resolved_config,
                Some(NativeProgressContext {
                    observer,
                    file_index,
                    task_tracker: &mut task_tracker,
                }),
            )?
        };
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
        if emit_run_events {
            observer.observe(TranscriptionProgressEvent::RunEnd {
                total_files,
                duration_seconds: total_seconds,
            });
        }
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

    result
}

pub(crate) fn progress_input_path(config: &NativeWhisperxConfig) -> std::path::PathBuf {
    match &config.input {
        crate::config::InputSource::Path { path } => path.clone(),
        crate::config::InputSource::Samples { source, .. } => source
            .as_ref()
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::path::PathBuf::from("<samples>")),
    }
}

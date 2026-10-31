//! Multi-input transcription runs that share one [`TranscriptionSession`].

use std::time::Instant;

use crate::config::{
    NativeWhisperxConfig, NativeWhisperxError, NativeWhisperxReport, SelectedMediaError,
    SelectedMediaInput,
};

use super::{
    progress_input_path, run_one_with_control_selected, CancellationHandle, FiniteCancellation,
    FiniteTranscriptionOutcome, MultiInputTranscriptionOutcome, NoopTranscriptionProgressObserver,
    TranscriptionProgressEvent, TranscriptionProgressObserver, TranscriptionSession,
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
    for config in &configs {
        super::validate_product_provider(config)?;
    }
    let total_files = configs.len();
    let run_started = Instant::now();
    observer.observe(TranscriptionProgressEvent::RunStart { total_files });
    // One input has nothing to share; several inputs keep loaded models.
    let mut session = (total_files > 1).then(TranscriptionSession::new);
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
            session.as_mut(),
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

fn unfinished_inputs(inputs: &[std::path::PathBuf], from: usize) -> Vec<UnfinishedTranscription> {
    inputs
        .iter()
        .enumerate()
        .skip(from)
        .map(|(file_index, input)| UnfinishedTranscription::new(file_index, input.clone()))
        .collect()
}

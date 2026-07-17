//! Observer types for the native Transcription Progress Stream.

use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::config::NativeWhisperxReport;
use crate::translation::{CuratedLanguage, TranslationPlanProvenance};

/// Cloneable cooperative cancellation shared by finite and live workflows.
///
/// Cancellation is sticky: after [`cancel`](Self::cancel) is called, every
/// clone observes the request. Workflows stop at the next safe composition
/// boundary rather than interrupting a model invocation or output write.
#[derive(Debug, Clone, Default)]
pub struct CancellationHandle {
    requested: Arc<AtomicBool>,
}

impl CancellationHandle {
    /// Creates an uncancelled handle.
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests cooperative cancellation from this or another thread.
    pub fn cancel(&self) {
        self.requested.store(true, Ordering::Release);
    }

    /// Returns whether cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.requested.load(Ordering::Acquire)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscriptionProgressTask {
    Decode,
    Vad,
    Asr,
    Alignment,
    Diarization,
    Translation,
    Output,
}

/// Details about the safe finite-workflow boundary where cancellation won.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteCancellation {
    file_index: usize,
    input: PathBuf,
    task: Option<TranscriptionProgressTask>,
}

impl FiniteCancellation {
    pub(crate) fn new(
        file_index: usize,
        input: PathBuf,
        task: Option<TranscriptionProgressTask>,
    ) -> Self {
        Self {
            file_index,
            input,
            task,
        }
    }

    /// Zero-based index of the file active when cancellation was observed.
    pub const fn file_index(&self) -> usize {
        self.file_index
    }

    /// Input associated with the cancelled file.
    pub fn input(&self) -> &std::path::Path {
        &self.input
    }

    /// Phase that was active, or `None` when cancellation preceded all phases.
    pub const fn task(&self) -> Option<TranscriptionProgressTask> {
        self.task
    }
}

/// Typed result of a cancellable single-input finite workflow.
#[derive(Debug)]
pub enum FiniteTranscriptionOutcome {
    /// The workflow and output writing completed.
    Completed(Box<NativeWhisperxReport>),
    /// The workflow stopped at a safe boundary without reporting a failure.
    Cancelled(FiniteCancellation),
}

/// Input not completed by a cancelled Multi-Input Transcription Run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnfinishedTranscription {
    file_index: usize,
    input: PathBuf,
}

impl UnfinishedTranscription {
    pub(crate) fn new(file_index: usize, input: PathBuf) -> Self {
        Self { file_index, input }
    }

    /// Zero-based position in the original Multi-Input Transcription Run.
    pub const fn file_index(&self) -> usize {
        self.file_index
    }

    /// Input that did not complete.
    pub fn input(&self) -> &std::path::Path {
        &self.input
    }
}

/// Typed result of a cancellable Multi-Input Transcription Run.
#[derive(Debug)]
pub enum MultiInputTranscriptionOutcome {
    /// Every file completed successfully.
    Completed(Vec<NativeWhisperxReport>),
    /// Cancellation retained completed files and identified all unfinished work.
    Cancelled {
        completed: Vec<NativeWhisperxReport>,
        cancellation: FiniteCancellation,
        unfinished: Vec<UnfinishedTranscription>,
    },
}

/// One ordered observation in the finite Transcription Progress Stream.
///
/// This enum is non-exhaustive so embedding applications can remain source
/// compatible as new model/provider facts are added. Consumers should retain
/// a wildcard match arm when formatting events.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum TranscriptionProgressEvent {
    RunStart {
        total_files: usize,
    },
    RunEnd {
        total_files: usize,
        duration_seconds: f64,
    },
    FileStart {
        file_index: usize,
        total_files: usize,
        input: PathBuf,
    },
    FileEnd {
        file_index: usize,
        total_files: usize,
        input: PathBuf,
        duration_seconds: f64,
    },
    TaskStart {
        file_index: usize,
        task: TranscriptionProgressTask,
    },
    TaskEnd {
        file_index: usize,
        task: TranscriptionProgressTask,
        duration_seconds: f64,
    },
    ModelResolutionStart {
        file_index: usize,
        task: TranscriptionProgressTask,
        provider: String,
        model_id: String,
    },
    ModelResolutionEnd {
        file_index: usize,
        task: TranscriptionProgressTask,
        provider: String,
        model_id: String,
        source: String,
    },
    ModelDownloadStart {
        file_index: usize,
        task: TranscriptionProgressTask,
        provider: String,
        model_id: String,
    },
    ModelDownloadEnd {
        file_index: usize,
        task: TranscriptionProgressTask,
        provider: String,
        model_id: String,
        duration_seconds: f64,
    },
    ModelLoadStart {
        file_index: usize,
        task: TranscriptionProgressTask,
        provider: String,
        model_id: String,
    },
    ModelLoadEnd {
        file_index: usize,
        task: TranscriptionProgressTask,
        provider: String,
        model_id: String,
        duration_seconds: f64,
    },
    ModelReuse {
        file_index: usize,
        task: TranscriptionProgressTask,
        provider: String,
        model_id: String,
    },
    TranslationLegStart {
        file_index: usize,
        leg_index: usize,
        total_legs: usize,
        provenance: TranslationPlanProvenance,
        source: CuratedLanguage,
        target: CuratedLanguage,
        provider: String,
        model_id: String,
    },
    TranslationLegEnd {
        file_index: usize,
        leg_index: usize,
        total_legs: usize,
        provenance: TranslationPlanProvenance,
        source: CuratedLanguage,
        target: CuratedLanguage,
        provider: String,
        model_id: String,
        duration_seconds: f64,
    },
    Failure {
        file_index: usize,
        input: PathBuf,
        task: Option<TranscriptionProgressTask>,
        duration_seconds: f64,
        message: String,
    },
    Cancelled {
        file_index: usize,
        input: PathBuf,
        task: Option<TranscriptionProgressTask>,
        duration_seconds: f64,
    },
}

pub trait TranscriptionProgressObserver {
    fn observe(&mut self, event: TranscriptionProgressEvent);
}

#[derive(Debug, Default)]
pub struct NoopTranscriptionProgressObserver;

impl TranscriptionProgressObserver for NoopTranscriptionProgressObserver {
    fn observe(&mut self, _event: TranscriptionProgressEvent) {}
}

#[derive(Debug, Default)]
pub(crate) struct ProgressTaskTracker {
    current: Option<TranscriptionProgressTask>,
}

impl ProgressTaskTracker {
    pub(crate) fn set_current(&mut self, task: Option<TranscriptionProgressTask>) {
        self.current = task;
    }

    pub(crate) fn current(&self) -> Option<TranscriptionProgressTask> {
        self.current
    }
}

pub(crate) struct NativeProgressContext<'a> {
    pub(crate) observer: &'a mut dyn TranscriptionProgressObserver,
    pub(crate) file_index: usize,
    pub(crate) task_tracker: &'a mut ProgressTaskTracker,
    pub(crate) cancellation: &'a CancellationHandle,
}

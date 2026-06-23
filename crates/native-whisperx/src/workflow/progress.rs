//! Observer types for the native Transcription Progress Stream.

use std::path::PathBuf;

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
    Failure {
        file_index: usize,
        input: PathBuf,
        task: Option<TranscriptionProgressTask>,
        duration_seconds: f64,
        message: String,
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
}

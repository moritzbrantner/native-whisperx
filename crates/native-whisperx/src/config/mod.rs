//! Public configuration types for native-whisperx workflows.

mod alignment;
mod asr;
mod defaults;
mod diarization;
mod error;
mod output;
mod parity;
mod report;
mod request;
mod selection;
mod speaker;
mod translation;
mod vad;
mod workflow_selection;

pub use alignment::AlignmentConfig;
pub use asr::{
    AsrConfig, AsrProvider, DevicePreference, ExternalWhisperxConfig, TranscriptionTask,
    WhisperxDecodeConfig,
};
pub(crate) use defaults::default_whisperx_command;
pub(crate) use diarization::is_pyannote_diarization_model;
pub use diarization::{AssignmentPolicy, DiarizationConfig};
pub use error::NativeWhisperxError;
pub use output::{OutputConfig, OutputFormat, SegmentResolution, SubtitleConfig};
pub use parity::{
    ExpectedOutputComparison, ExpectedOutputFile, ExpectedTranscriptTarget, OutputComparisonMode,
    ParityComparison, ParityComparisonConfig, ParityConfig, ParityFixtureCase,
    ParityFixtureCaseReport, ParityFixtureSuite, ParityFixtureSuiteReport,
    ParityMultiInputFixtureCase, ParityPreflightCaseReport, ParityPreflightReport, ParityReport,
    ParityTolerance,
};
pub use report::{
    NativeWhisperxReport, NativeWorkflowSelectionReport, OutputFile,
    SelectedDiarizationModelReport, SelectedVadReport,
};
pub use request::{InputSource, NativeWhisperxConfig};
pub use selection::{
    AutomaticWorkflowSelection, AutomaticWorkflowSelectionDecision,
    AutomaticWorkflowSelectionResource, ConfigSelection, ModelResourceSource,
};
pub use speaker::{SpeakerCorrectionReport, SpeakerCorrectionRequest};
#[cfg(feature = "translation")]
pub use translation::NativeOpusMtTranslationProviderConfig;
pub use translation::TranslationConfig;
pub use vad::{VadConfig, VadMethod};
pub use workflow_selection::resolve_automatic_workflow_selection;

//! Public configuration types for native-whisperx workflows.

mod alignment;
mod asr;
mod defaults;
mod diarization;
mod error;
mod output;
mod parity;
mod pyannote_bundle;
mod report;
mod request;
mod selection;
mod speaker;
mod translation;
mod vad;
mod workflow_selection;

pub use alignment::{AlignmentConfig, AlignmentInterpolationMethod};
pub use asr::{
    AsrConfig, AsrProvider, DevicePreference, ExternalWhisperxConfig, TranscriptionTask,
    WhisperxDecodeConfig,
};
pub(crate) use defaults::default_whisperx_command;
pub(crate) use diarization::is_pyannote_diarization_model;
pub use diarization::{AssignmentPolicy, DiarizationConfig};
pub(crate) use error::ensure_whisperx_compat_enabled;
pub use error::{NativeWhisperxError, SelectedMediaError};
#[cfg(feature = "media-decode")]
pub use error::{
    SelectedMediaErrorReason, SelectedMediaStream, SelectedMediaStreamInventory, SelectedMediaType,
};
pub use output::{OutputConfig, OutputFormat, SegmentResolution, SubtitleConfig};
pub use parity::{
    ExpectedOutputComparison, ExpectedOutputFile, ExpectedTranscriptTarget, OutputComparisonMode,
    ParityBenchmarkGate, ParityComparison, ParityComparisonConfig, ParityConfig, ParityFixtureCase,
    ParityFixtureCaseReport, ParityFixtureSuite, ParityFixtureSuiteReport,
    ParityMultiInputFixtureCase, ParityPreflightCaseReport, ParityPreflightReport, ParityReport,
    ParityTolerance,
};
pub use pyannote_bundle::{
    verify_pyannote_diarization_bundle, verify_pyannote_vad_bundle,
    PyannoteDiarizationBundleVerification, PyannoteVadBundleVerification,
};
pub use report::{
    NativePerformanceReport, NativeTranscriptionProvenance, NativeVadSegment, NativeWhisperxReport,
    NativeWorkflowSelectionReport, OutputFile, SelectedDiarizationModelReport, SelectedVadReport,
};
pub use request::{InputSource, NativeWhisperxConfig, SelectedMediaInput};
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

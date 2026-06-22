mod alignment;
mod asr;
mod defaults;
mod diarization;
mod error;
mod output;
mod parity;
mod report;
mod request;
mod speaker;
mod translation;
mod vad;

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
pub use report::{NativeWhisperxReport, OutputFile};
pub use request::{InputSource, NativeWhisperxConfig};
pub use speaker::{SpeakerCorrectionReport, SpeakerCorrectionRequest};
pub use translation::TranslationConfig;
pub use vad::{VadConfig, VadMethod};

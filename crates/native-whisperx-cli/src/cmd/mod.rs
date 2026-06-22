//! CLI command modules and shared command-layer imports.

pub(crate) use std::collections::HashSet;
pub(crate) use std::ffi::OsString;
pub(crate) use std::fs;
pub(crate) use std::path::{Path, PathBuf};
pub(crate) use std::process::{Command as ProcessCommand, ExitStatus, Stdio};
pub(crate) use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub(crate) use anyhow::Context;
#[cfg(test)]
pub(crate) use native_whisperx::AlignmentConfig;
pub(crate) use native_whisperx::{
    build_transcription_request, compare_with_whisperx, import_whisperx_json, run, run_many,
    run_parity_fixture_suite, AsrConfig, AsrProvider, DevicePreference, DiarizationConfig,
    ExpectedOutputFile, ExpectedTranscriptTarget, ExternalWhisperxConfig, InputSource,
    NativeWhisperxConfig, OutputComparisonMode, OutputConfig, OutputFormat, ParityComparisonConfig,
    ParityConfig, ParityFixtureCase, ParityFixtureCaseReport, ParityFixtureSuite,
    ParityFixtureSuiteReport, ParityMultiInputFixtureCase, SegmentResolution, SubtitleConfig,
    TranscriptionTask, TranslationConfig, VadConfig, VadMethod, WhisperxDecodeConfig,
};

pub(crate) use crate::{
    CliAlignmentInterpolationMethod, CliAssignmentPolicy, CliDevicePreference, CliProvider,
    CliTask, LiveTranscribeArgs, TranscribeArgs,
};
pub(crate) use support::{
    absolute_from_cwd, alignment_config, resolve_cli_path_with_root, translation_config,
    validate_speaker_directory_args,
};

pub(crate) mod import;
pub(crate) mod inspect;
pub(crate) mod live;
pub(crate) mod parity;
pub(crate) mod speaker;
pub(crate) mod support;
pub(crate) mod transcribe;

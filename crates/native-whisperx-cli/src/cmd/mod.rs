pub(crate) use std::ffi::OsString;
pub(crate) use std::collections::HashSet;
pub(crate) use std::fs;
pub(crate) use std::io::Write;
pub(crate) use std::net::{Ipv4Addr, SocketAddr, TcpListener};
pub(crate) use std::path::{Path, PathBuf};
pub(crate) use std::process::{Command as ProcessCommand, ExitStatus, Stdio};
pub(crate) use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub(crate) use anyhow::Context;
pub(crate) use native_whisperx::{
    build_transcription_request, compare_with_whisperx, correct_speaker, import_whisperx_json,
    list_speaker_profiles, rebuild_speaker_trace, resolve_speaker_directory, run, run_many,
    run_parity_fixture_suite, run_parity_preflight, validate_speaker_library, AsrConfig,
    AsrProvider, DevicePreference, DiarizationConfig, ExpectedOutputFile,
    ExpectedTranscriptTarget, ExternalWhisperxConfig, InputSource, NativeWhisperxConfig,
    OutputComparisonMode, OutputConfig, OutputFormat, ParityComparisonConfig, ParityConfig,
    ParityFixtureCase, ParityFixtureCaseReport, ParityFixtureSuite, ParityFixtureSuiteReport,
    ResolvedSpeakerDirectoryScope, SegmentResolution, SpeakerCorrectionRange,
    SpeakerCorrectionRequest, SubtitleConfig, TranscriptionTask, TranslationConfig, VadConfig,
    VadMethod, WhisperxDecodeConfig,
};
#[cfg(test)]
pub(crate) use native_whisperx::AlignmentConfig;

pub(crate) use crate::{
    CliOutputFormat, CliProvider, CliTask, ImportWhisperxArgs, InspectModelsArgs, ParityArgs,
    ParityBenchArgs, ParityBenchCaseArgs, ParityFixtureCaseArgs, ParityFixturesArgs,
    ParityGoldensArgs, ParityPreflightArgs, ParitySummaryArgs, SpeakerDirectoryArgs,
    SpeakersArgs, SpeakersCommand, SpeakersCorrectArgs, SpeakersListArgs, SpeakersOpenArgs,
    SpeakersPathArgs, SpeakersRebuildTraceArgs, SpeakersValidateArgs, TranscribeArgs,
};
pub(crate) use support::{
    absolute_from_cwd, alignment_config, resolve_cli_path_with_root, translation_config,
    validate_speaker_directory_args,
};

pub(crate) mod inspect;
pub(crate) mod parity;
pub(crate) mod speaker;
pub(crate) mod support;
pub(crate) mod transcribe;

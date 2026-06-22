use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use audio_analysis_speakers::{
    SpeakerEmbedding, SpeakerId, SpeakerLabel, SpeakerLibrary, SpeakerProfile,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use text_transcripts::TranscriptionContract;

use crate::{import_whisperx_json, NativeWhisperxError};

pub const LOCAL_SPEAKER_DIRECTORY: &str = ".native-whisperx/speakers";
pub const GLOBAL_SPEAKER_DIRECTORY_APP: &str = "native-whisperx";
pub const GLOBAL_SPEAKER_DIRECTORY_NAME: &str = "speakers";
pub const SPEAKER_LIBRARY_FILE: &str = "library.json";
pub const SPEAKER_TRACE_FILE: &str = "speaker-trace.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerDirectorySelection {
    #[serde(default)]
    pub scope: SpeakerDirectoryScope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_path: Option<PathBuf>,
}

impl Default for SpeakerDirectorySelection {
    fn default() -> Self {
        Self {
            scope: SpeakerDirectoryScope::Auto,
            explicit_path: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpeakerDirectoryScope {
    #[default]
    Auto,
    Local,
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedSpeakerDirectoryScope {
    Local,
    Global,
    Explicit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSpeakerDirectory {
    pub path: PathBuf,
    pub scope: ResolvedSpeakerDirectoryScope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeakerLibraryValidation {
    pub path: PathBuf,
    pub profile_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerProfileSummary {
    pub speaker_id: String,
    pub label: String,
    pub status: String,
    pub embedding_count: usize,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerCorrectionRange {
    pub start_seconds: f64,
    pub end_seconds: f64,
}

impl SpeakerCorrectionRange {
    pub fn overlaps(self, start_seconds: f64, end_seconds: f64) -> bool {
        start_seconds < self.end_seconds && end_seconds > self.start_seconds
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerTrace {
    pub version: u32,
    pub scan_root: PathBuf,
    pub speakers: Vec<SpeakerTraceSpeaker>,
    #[serde(default)]
    pub errors: Vec<SpeakerTraceError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerTraceSpeaker {
    pub kind: SpeakerTraceSpeakerKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_label: Option<String>,
    pub files: Vec<SpeakerTraceFile>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpeakerTraceSpeakerKind {
    Enrolled,
    Anonymous,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerTraceFile {
    pub source_file: PathBuf,
    pub segment_count: usize,
    pub total_duration: f64,
    pub spans: Vec<SpeakerTraceSpan>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerTraceSpan {
    pub start_seconds: Option<f64>,
    pub end_seconds: Option<f64>,
    pub snippet: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerTraceError {
    pub source_file: PathBuf,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerTraceRebuildReport {
    pub trace_path: PathBuf,
    pub trace: SpeakerTrace,
    pub stats: SpeakerTraceRebuildStats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerTraceRebuildStats {
    pub scanned_files: usize,
    pub accepted_entries: usize,
    pub ignored_non_json_files: usize,
    pub malformed_json_errors: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerDirectoryState {
    pub scope: SpeakerDirectoryStateScope,
    pub path: PathBuf,
    pub library: SpeakerLibraryState,
    pub profiles: Vec<SpeakerProfileState>,
    pub trace: SpeakerTraceState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SpeakerDirectoryStateScope {
    Local,
    Global,
    Explicit,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerLibraryState {
    pub path: PathBuf,
    pub status: SpeakerLibraryValidationStatus,
    pub profile_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SpeakerLibraryValidationStatus {
    Valid,
    Missing,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerProfileState {
    pub id: String,
    pub label: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SpeakerProfileEdit {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub metadata: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerTraceState {
    pub path: PathBuf,
    pub status: SpeakerTraceStateStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_root: Option<PathBuf>,
    pub speakers: Vec<SpeakerTraceSpeaker>,
    pub errors: Vec<SpeakerTraceError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SpeakerTraceStateStatus {
    Valid,
    Missing,
    Invalid,
}

pub fn resolve_speaker_directory(
    selection: &SpeakerDirectorySelection,
    current_dir: &Path,
) -> Result<ResolvedSpeakerDirectory, NativeWhisperxError> {
    if let Some(explicit_path) = &selection.explicit_path {
        return Ok(ResolvedSpeakerDirectory {
            path: absolute_from_base(current_dir, explicit_path),
            scope: ResolvedSpeakerDirectoryScope::Explicit,
        });
    }

    match selection.scope {
        SpeakerDirectoryScope::Auto => {
            let local = local_speaker_directory(current_dir);
            if local.is_dir() {
                Ok(ResolvedSpeakerDirectory {
                    path: local,
                    scope: ResolvedSpeakerDirectoryScope::Local,
                })
            } else {
                Ok(ResolvedSpeakerDirectory {
                    path: global_speaker_directory()?,
                    scope: ResolvedSpeakerDirectoryScope::Global,
                })
            }
        }
        SpeakerDirectoryScope::Local => Ok(ResolvedSpeakerDirectory {
            path: local_speaker_directory(current_dir),
            scope: ResolvedSpeakerDirectoryScope::Local,
        }),
        SpeakerDirectoryScope::Global => Ok(ResolvedSpeakerDirectory {
            path: global_speaker_directory()?,
            scope: ResolvedSpeakerDirectoryScope::Global,
        }),
    }
}

pub fn local_speaker_directory(current_dir: &Path) -> PathBuf {
    current_dir.join(LOCAL_SPEAKER_DIRECTORY)
}

pub fn global_speaker_directory() -> Result<PathBuf, NativeWhisperxError> {
    let data_dir = dirs::data_dir().ok_or_else(|| {
        NativeWhisperxError::InvalidConfig(
            "could not resolve a platform data directory for the global Speaker Directory"
                .to_string(),
        )
    })?;
    Ok(data_dir
        .join(GLOBAL_SPEAKER_DIRECTORY_APP)
        .join(GLOBAL_SPEAKER_DIRECTORY_NAME))
}

pub fn speaker_library_path(speaker_directory: &Path) -> PathBuf {
    speaker_directory.join(SPEAKER_LIBRARY_FILE)
}

pub fn speaker_trace_path(speaker_directory: &Path) -> PathBuf {
    speaker_directory.join(SPEAKER_TRACE_FILE)
}

pub fn validate_speaker_library(
    speaker_directory: &Path,
) -> Result<SpeakerLibraryValidation, NativeWhisperxError> {
    validate_speaker_library_file(&speaker_library_path(speaker_directory))
}

pub fn validate_speaker_library_file(
    library_path: &Path,
) -> Result<SpeakerLibraryValidation, NativeWhisperxError> {
    let bytes = fs::read(library_path)?;
    let (validation, _) = parse_canonical_speaker_library_json(library_path, &bytes)?;
    Ok(validation)
}

pub fn list_speaker_profiles(
    selection: SpeakerDirectorySelection,
    include_drafts: bool,
) -> Result<Vec<SpeakerProfileSummary>, NativeWhisperxError> {
    let current_dir = std::env::current_dir()?;
    let resolved = resolve_speaker_directory(&selection, &current_dir)?;
    let Some(library) = load_speaker_library_if_present(&resolved.path)? else {
        return Ok(Vec::new());
    };
    Ok(library
        .profiles()
        .filter(|profile| include_drafts || speaker_profile_status(profile) != "draft")
        .map(|profile| SpeakerProfileSummary {
            speaker_id: profile.id().as_str().to_string(),
            label: profile.label().as_str().to_string(),
            status: speaker_profile_status(profile),
            embedding_count: profile.embeddings().len(),
            metadata: profile.metadata_map().clone(),
        })
        .collect())
}

pub fn update_speaker_profile(
    speaker_directory: &Path,
    profile_id: &str,
    edit: SpeakerProfileEdit,
) -> Result<SpeakerLibraryValidation, NativeWhisperxError> {
    if profile_id.trim().is_empty() {
        return Err(NativeWhisperxError::InvalidConfig(
            "Speaker Library profile id must not be empty".to_string(),
        ));
    }
    if let Some(request_id) = &edit.id {
        if request_id != profile_id {
            return Err(NativeWhisperxError::InvalidConfig(format!(
                "Speaker Library profile ids are immutable: `{profile_id}` cannot be changed to `{request_id}`"
            )));
        }
    }
    if edit.label.is_none() && edit.metadata.is_none() {
        return Err(NativeWhisperxError::InvalidConfig(
            "Speaker Library profile edit must include a label or metadata".to_string(),
        ));
    }

    let library_path = speaker_library_path(speaker_directory);
    let mut value = read_canonical_speaker_library_value(&library_path)?;
    let profile = speaker_profile_value_mut(&library_path, &mut value, profile_id)?;

    if let Some(label) = edit.label {
        if label.trim().is_empty() {
            return Err(NativeWhisperxError::InvalidConfig(
                "Speaker Library profile label must not be empty".to_string(),
            ));
        }
        profile.insert("label".to_string(), Value::String(label));
    }
    if let Some(metadata) = edit.metadata {
        let metadata = metadata
            .into_iter()
            .map(|(key, value)| (key, Value::String(value)))
            .collect::<Map<String, Value>>();
        profile.insert("metadata".to_string(), Value::Object(metadata));
    }

    write_validated_speaker_library_value(&library_path, &value)
}

pub fn delete_speaker_profile(
    speaker_directory: &Path,
    profile_id: &str,
) -> Result<SpeakerLibraryValidation, NativeWhisperxError> {
    if profile_id.trim().is_empty() {
        return Err(NativeWhisperxError::InvalidConfig(
            "Speaker Library profile id must not be empty".to_string(),
        ));
    }

    let library_path = speaker_library_path(speaker_directory);
    let mut value = read_canonical_speaker_library_value(&library_path)?;
    let profiles = speaker_profiles_array_mut(&library_path, &mut value)?;
    let Some(index) = profiles.iter().position(|profile| {
        profile
            .as_object()
            .and_then(|object| object.get("id"))
            .and_then(Value::as_str)
            == Some(profile_id)
    }) else {
        return Err(NativeWhisperxError::InvalidConfig(format!(
            "Speaker Library profile `{profile_id}` was not found"
        )));
    };
    profiles.remove(index);
    if profiles.is_empty() {
        fs::remove_file(&library_path)?;
        return Ok(SpeakerLibraryValidation {
            path: library_path,
            profile_count: 0,
        });
    }

    write_validated_speaker_library_value(&library_path, &value)
}

pub fn reject_draft_speaker_profile_creation() -> Result<(), NativeWhisperxError> {
    Err(NativeWhisperxError::InvalidConfig(
        "creating draft Speaker Library profiles without embeddings is not supported".to_string(),
    ))
}

pub(crate) fn load_speaker_library_if_present(
    speaker_directory: &Path,
) -> Result<Option<SpeakerLibrary>, NativeWhisperxError> {
    let path = speaker_library_path(speaker_directory);
    match fs::read_to_string(&path) {
        Ok(json) => SpeakerLibrary::from_json_str(&json)
            .map(Some)
            .map_err(|error| {
                NativeWhisperxError::InvalidConfig(format!(
                    "Speaker Library `{}` is malformed or incompatible: {error}",
                    path.display()
                ))
            }),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

pub(crate) fn save_speaker_library(
    speaker_directory: &Path,
    library: &SpeakerLibrary,
) -> Result<SpeakerLibraryValidation, NativeWhisperxError> {
    let library_path = speaker_library_path(speaker_directory);
    let json = library.to_json_string().map_err(|error| {
        NativeWhisperxError::InvalidConfig(format!(
            "Speaker Library `{}` could not be serialized: {error}",
            library_path.display()
        ))
    })?;
    let value = serde_json::from_str::<Value>(&json)?;
    write_validated_speaker_library_value(&library_path, &value)
}

#[cfg(feature = "diarization")]
pub(crate) fn filter_speaker_library_drafts(
    library: &SpeakerLibrary,
    include_drafts: bool,
) -> Result<(SpeakerLibrary, usize), NativeWhisperxError> {
    if include_drafts {
        return Ok((library.clone(), 0));
    }

    let mut filtered = SpeakerLibrary::new();
    let mut filtered_count = 0usize;
    for profile in library.profiles() {
        if speaker_profile_status(profile) == "draft" {
            filtered_count += 1;
            continue;
        }
        filtered
            .add_profile(profile.clone())
            .map_err(speaker_library_error)?;
    }
    Ok((filtered, filtered_count))
}

pub(crate) fn upsert_speaker_profile_embedding(
    library: &SpeakerLibrary,
    profile_id: &str,
    label: &str,
    embedding: SpeakerEmbedding,
    metadata: BTreeMap<String, String>,
) -> Result<(SpeakerLibrary, bool), NativeWhisperxError> {
    let speaker_id = SpeakerId::new(profile_id.to_string()).map_err(speaker_library_error)?;
    let speaker_label = SpeakerLabel::new(label.to_string()).map_err(speaker_library_error)?;
    let mut updated = SpeakerLibrary::new();
    let mut matched = false;

    for profile in library.profiles() {
        if profile.id().as_str() == profile_id {
            if profile.label().as_str() != label {
                return Err(NativeWhisperxError::InvalidConfig(format!(
                    "Speaker Library profile `{profile_id}` already has label `{}`; refusing to relabel it to `{label}`",
                    profile.label().as_str()
                )));
            }
            let mut replacement = SpeakerProfile::new(speaker_id.clone(), speaker_label.clone());
            for existing_embedding in profile.embeddings() {
                replacement
                    .add_embedding(existing_embedding.clone())
                    .map_err(speaker_library_error)?;
            }
            replacement
                .add_embedding(embedding.clone())
                .map_err(speaker_library_error)?;
            for (key, value) in profile.metadata_map().iter().chain(metadata.iter()) {
                replacement = replacement.metadata(key.clone(), value.clone());
            }
            updated
                .add_profile(replacement)
                .map_err(speaker_library_error)?;
            matched = true;
        } else {
            updated
                .add_profile(profile.clone())
                .map_err(speaker_library_error)?;
        }
    }

    if !matched {
        let mut profile = SpeakerProfile::new(speaker_id, speaker_label)
            .with_embedding(embedding)
            .map_err(speaker_library_error)?;
        for (key, value) in metadata {
            profile = profile.metadata(key, value);
        }
        updated
            .add_profile(profile)
            .map_err(speaker_library_error)?;
    }

    Ok((updated, matched))
}

pub(crate) fn speaker_profile_status(profile: &SpeakerProfile) -> String {
    profile
        .metadata_map()
        .get("status")
        .cloned()
        .filter(|status| !status.trim().is_empty())
        .unwrap_or_else(|| "confirmed".to_string())
}

fn speaker_library_error(error: impl std::fmt::Display) -> NativeWhisperxError {
    NativeWhisperxError::InvalidConfig(error.to_string())
}

fn read_canonical_speaker_library_value(library_path: &Path) -> Result<Value, NativeWhisperxError> {
    let bytes = fs::read(library_path)?;
    let value = serde_json::from_slice::<Value>(&bytes)?;
    validate_canonical_snapshot_shape(library_path, &value)?;
    parse_canonical_speaker_library_json(library_path, &bytes)?;
    Ok(value)
}

fn speaker_profiles_array_mut<'a>(
    library_path: &Path,
    value: &'a mut Value,
) -> Result<&'a mut Vec<Value>, NativeWhisperxError> {
    value
        .as_object_mut()
        .and_then(|object| object.get_mut("profiles"))
        .and_then(Value::as_array_mut)
        .ok_or_else(|| {
            NativeWhisperxError::InvalidConfig(format!(
                "Speaker Library `{}` must contain a profiles array",
                library_path.display()
            ))
        })
}

fn speaker_profile_value_mut<'a>(
    library_path: &Path,
    value: &'a mut Value,
    profile_id: &str,
) -> Result<&'a mut Map<String, Value>, NativeWhisperxError> {
    let profiles = speaker_profiles_array_mut(library_path, value)?;
    profiles
        .iter_mut()
        .filter_map(Value::as_object_mut)
        .find(|profile| profile.get("id").and_then(Value::as_str) == Some(profile_id))
        .ok_or_else(|| {
            NativeWhisperxError::InvalidConfig(format!(
                "Speaker Library profile `{profile_id}` was not found"
            ))
        })
}

fn write_validated_speaker_library_value(
    library_path: &Path,
    value: &Value,
) -> Result<SpeakerLibraryValidation, NativeWhisperxError> {
    let bytes = serde_json::to_vec_pretty(value)?;
    let (validation, _) = parse_canonical_speaker_library_json(library_path, &bytes)?;
    if let Some(parent) = library_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let temp_path = library_path.with_file_name(format!(
        ".{}.tmp-{}-{}",
        library_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(SPEAKER_LIBRARY_FILE),
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default()
    ));
    fs::write(&temp_path, &bytes)?;
    if let Err(error) = fs::rename(&temp_path, library_path) {
        let _ = fs::remove_file(&temp_path);
        return Err(error.into());
    }
    Ok(validation)
}

fn parse_canonical_speaker_library_json(
    library_path: &Path,
    bytes: &[u8],
) -> Result<(SpeakerLibraryValidation, SpeakerLibrary), NativeWhisperxError> {
    let value = serde_json::from_slice::<Value>(bytes)?;
    validate_canonical_snapshot_shape(library_path, &value)?;

    let json = std::str::from_utf8(bytes).map_err(|error| {
        NativeWhisperxError::InvalidConfig(format!(
            "Speaker Library `{}` is not valid UTF-8: {error}",
            library_path.display()
        ))
    })?;
    let library = SpeakerLibrary::from_json_str(json).map_err(|error| {
        NativeWhisperxError::InvalidConfig(format!(
            "Speaker Library `{}` is malformed or incompatible: {error}",
            library_path.display()
        ))
    })?;

    let validation = SpeakerLibraryValidation {
        path: library_path.to_path_buf(),
        profile_count: library.len(),
    };
    Ok((validation, library))
}

fn validate_canonical_snapshot_shape(
    library_path: &Path,
    value: &Value,
) -> Result<(), NativeWhisperxError> {
    let object = value.as_object().ok_or_else(|| {
        NativeWhisperxError::InvalidConfig(format!(
            "Speaker Library `{}` must be a JSON object",
            library_path.display()
        ))
    })?;
    for key in object.keys() {
        if !matches!(key.as_str(), "version" | "embedding_model" | "profiles") {
            return Err(NativeWhisperxError::InvalidConfig(format!(
                "Speaker Library `{}` is not canonical: unexpected top-level field `{key}`",
                library_path.display()
            )));
        }
    }
    Ok(())
}

pub fn read_speaker_directory_state(
    resolved: &ResolvedSpeakerDirectory,
) -> Result<SpeakerDirectoryState, NativeWhisperxError> {
    let library_path = speaker_library_path(&resolved.path);
    let (library_state, profiles) = match fs::read(&library_path) {
        Ok(bytes) => match parse_canonical_speaker_library_json(&library_path, &bytes) {
            Ok((validation, library)) => {
                let profiles = library
                    .profiles()
                    .map(|profile| SpeakerProfileState {
                        id: profile.id().as_str().to_string(),
                        label: profile.label().as_str().to_string(),
                        metadata: profile.metadata_map().clone(),
                    })
                    .collect();
                (
                    SpeakerLibraryState {
                        path: validation.path,
                        status: SpeakerLibraryValidationStatus::Valid,
                        profile_count: validation.profile_count,
                        message: None,
                    },
                    profiles,
                )
            }
            Err(error) => (
                SpeakerLibraryState {
                    path: library_path.clone(),
                    status: SpeakerLibraryValidationStatus::Invalid,
                    profile_count: 0,
                    message: Some(error.to_string()),
                },
                Vec::new(),
            ),
        },
        Err(error) if error.kind() == ErrorKind::NotFound => (
            SpeakerLibraryState {
                path: library_path,
                status: SpeakerLibraryValidationStatus::Missing,
                profile_count: 0,
                message: Some("Speaker Library file is missing".to_string()),
            },
            Vec::new(),
        ),
        Err(error) => return Err(error.into()),
    };

    let trace = read_speaker_trace_state(&speaker_trace_path(&resolved.path))?;

    Ok(SpeakerDirectoryState {
        scope: resolved.scope.into(),
        path: resolved.path.clone(),
        library: library_state,
        profiles,
        trace,
    })
}

fn read_speaker_trace_state(trace_path: &Path) -> Result<SpeakerTraceState, NativeWhisperxError> {
    match fs::read(trace_path) {
        Ok(bytes) => match serde_json::from_slice::<SpeakerTrace>(&bytes) {
            Ok(trace) => Ok(SpeakerTraceState {
                path: trace_path.to_path_buf(),
                status: SpeakerTraceStateStatus::Valid,
                scan_root: Some(trace.scan_root),
                speakers: trace.speakers,
                errors: trace.errors,
                message: None,
            }),
            Err(error) => Ok(SpeakerTraceState {
                path: trace_path.to_path_buf(),
                status: SpeakerTraceStateStatus::Invalid,
                scan_root: None,
                speakers: Vec::new(),
                errors: Vec::new(),
                message: Some(format!(
                    "Speaker Trace is malformed or incompatible: {error}"
                )),
            }),
        },
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(SpeakerTraceState {
            path: trace_path.to_path_buf(),
            status: SpeakerTraceStateStatus::Missing,
            scan_root: None,
            speakers: Vec::new(),
            errors: Vec::new(),
            message: Some("Speaker Trace file is missing".to_string()),
        }),
        Err(error) => Err(error.into()),
    }
}

impl From<ResolvedSpeakerDirectoryScope> for SpeakerDirectoryStateScope {
    fn from(value: ResolvedSpeakerDirectoryScope) -> Self {
        match value {
            ResolvedSpeakerDirectoryScope::Local => Self::Local,
            ResolvedSpeakerDirectoryScope::Global => Self::Global,
            ResolvedSpeakerDirectoryScope::Explicit => Self::Explicit,
        }
    }
}

pub fn rebuild_speaker_trace(
    speaker_directory: &Path,
    scan_root: &Path,
) -> Result<SpeakerTraceRebuildReport, NativeWhisperxError> {
    let library = load_speaker_library_for_trace(speaker_directory)?;
    let scan_root = absolute_from_base(&std::env::current_dir()?, scan_root);
    let scan = scan_transcript_files(&scan_root)?;
    let mut builder = SpeakerTraceBuilder::new(scan_root.clone(), &library);
    let mut accepted_entries = 0usize;
    for json_path in scan.json_candidates {
        match read_trace_transcript(&json_path) {
            Ok(Some(transcript)) => {
                accepted_entries += builder.add_transcript(json_path, transcript);
            }
            Ok(None) => {}
            Err(message) => builder.add_error(json_path, message),
        }
    }

    let trace = builder.finish();
    let stats = SpeakerTraceRebuildStats {
        scanned_files: scan.scanned_files,
        accepted_entries,
        ignored_non_json_files: scan.ignored_non_json_files,
        malformed_json_errors: trace.errors.len(),
    };
    let trace_path = speaker_trace_path(speaker_directory);
    if let Some(parent) = trace_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&trace_path, serde_json::to_string_pretty(&trace)?)?;

    Ok(SpeakerTraceRebuildReport {
        trace_path,
        trace,
        stats,
    })
}

fn absolute_from_base(base: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}

fn load_speaker_library_for_trace(
    speaker_directory: &Path,
) -> Result<SpeakerLibrary, NativeWhisperxError> {
    let path = speaker_library_path(speaker_directory);
    match fs::read_to_string(&path) {
        Ok(json) => SpeakerLibrary::from_json_str(&json).map_err(|error| {
            NativeWhisperxError::InvalidConfig(format!(
                "Speaker Library `{}` is malformed or incompatible: {error}",
                path.display()
            ))
        }),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(SpeakerLibrary::new()),
        Err(error) => Err(error.into()),
    }
}

#[derive(Debug, Default)]
struct SpeakerTraceScan {
    json_candidates: Vec<PathBuf>,
    scanned_files: usize,
    ignored_non_json_files: usize,
}

fn scan_transcript_files(scan_root: &Path) -> Result<SpeakerTraceScan, NativeWhisperxError> {
    let mut scan = SpeakerTraceScan::default();
    collect_transcript_files(scan_root, &mut scan)?;
    scan.json_candidates.sort();
    Ok(scan)
}

fn collect_transcript_files(
    path: &Path,
    scan: &mut SpeakerTraceScan,
) -> Result<(), NativeWhisperxError> {
    if path.is_file() {
        scan.scanned_files += 1;
        if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
        {
            scan.json_candidates.push(path.to_path_buf());
        } else {
            scan.ignored_non_json_files += 1;
        }
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_transcript_files(&path, scan)?;
        } else if path.is_file() {
            scan.scanned_files += 1;
            if path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
            {
                scan.json_candidates.push(path);
            } else {
                scan.ignored_non_json_files += 1;
            }
        }
    }
    Ok(())
}

fn read_trace_transcript(path: &Path) -> Result<Option<TranscriptionContract>, String> {
    let bytes =
        fs::read(path).map_err(|error| format!("failed to read transcript JSON: {error}"))?;
    let value = serde_json::from_slice::<Value>(&bytes)
        .map_err(|error| format!("malformed transcript JSON: {error}"))?;
    if !value
        .as_object()
        .and_then(|object| object.get("segments"))
        .is_some_and(Value::is_array)
    {
        return Ok(None);
    }

    if let Ok(transcript) = serde_json::from_value::<TranscriptionContract>(value.clone()) {
        if let Err(error) = transcript.validate_strict() {
            return Err(format!("malformed Native JSON transcript: {error}"));
        }
        return Ok(Some(transcript));
    }

    import_whisperx_json(&bytes)
        .map(Some)
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum SpeakerTraceKey {
    Enrolled { profile_id: String, label: String },
    Anonymous { label: String },
}

#[derive(Debug, Default)]
struct SpeakerTraceFileBuilder {
    segment_count: usize,
    total_duration: f64,
    spans: Vec<SpeakerTraceSpan>,
}

#[derive(Debug)]
struct SpeakerTraceBuilder {
    scan_root: PathBuf,
    profile_ids: BTreeMap<String, String>,
    profile_labels: BTreeMap<String, (String, String)>,
    speakers: BTreeMap<SpeakerTraceKey, BTreeMap<PathBuf, SpeakerTraceFileBuilder>>,
    errors: Vec<SpeakerTraceError>,
}

impl SpeakerTraceBuilder {
    fn new(scan_root: PathBuf, library: &SpeakerLibrary) -> Self {
        let mut profile_ids = BTreeMap::new();
        let mut profile_labels = BTreeMap::new();
        for profile in library.profiles() {
            let profile_id = profile.id().as_str().to_string();
            let label = profile.label().as_str().to_string();
            profile_ids.insert(profile_id.clone(), label.clone());
            profile_labels
                .entry(label.clone())
                .or_insert((profile_id, label));
        }

        Self {
            scan_root,
            profile_ids,
            profile_labels,
            speakers: BTreeMap::new(),
            errors: Vec::new(),
        }
    }

    fn add_transcript(&mut self, source_file: PathBuf, transcript: TranscriptionContract) -> usize {
        let mut accepted_entries = 0usize;
        for segment in transcript.segments {
            let Some(speaker) = segment
                .speaker
                .as_deref()
                .map(str::trim)
                .filter(|speaker| !speaker.is_empty())
            else {
                continue;
            };
            accepted_entries += 1;
            let key = self.trace_key(speaker);
            let file = self
                .speakers
                .entry(key)
                .or_default()
                .entry(source_file.clone())
                .or_default();
            file.segment_count += 1;
            if let Some(duration) = segment.duration_seconds() {
                file.total_duration += duration;
            }
            file.spans.push(SpeakerTraceSpan {
                start_seconds: segment.start_seconds,
                end_seconds: segment.end_seconds,
                snippet: segment.text.trim().to_string(),
            });
        }
        accepted_entries
    }

    fn add_error(&mut self, source_file: PathBuf, message: String) {
        self.errors.push(SpeakerTraceError {
            source_file,
            message,
        });
    }

    fn finish(self) -> SpeakerTrace {
        let speakers = self
            .speakers
            .into_iter()
            .map(|(key, files)| {
                let files = files
                    .into_iter()
                    .map(|(source_file, file)| SpeakerTraceFile {
                        source_file,
                        segment_count: file.segment_count,
                        total_duration: file.total_duration,
                        spans: file.spans,
                    })
                    .collect();
                match key {
                    SpeakerTraceKey::Enrolled { profile_id, label } => SpeakerTraceSpeaker {
                        kind: SpeakerTraceSpeakerKind::Enrolled,
                        profile_id: Some(profile_id),
                        label: Some(label),
                        anonymous_label: None,
                        files,
                    },
                    SpeakerTraceKey::Anonymous { label } => SpeakerTraceSpeaker {
                        kind: SpeakerTraceSpeakerKind::Anonymous,
                        profile_id: None,
                        label: None,
                        anonymous_label: Some(label),
                        files,
                    },
                }
            })
            .collect();
        SpeakerTrace {
            version: 1,
            scan_root: self.scan_root,
            speakers,
            errors: self.errors,
        }
    }

    fn trace_key(&self, speaker: &str) -> SpeakerTraceKey {
        if let Some(label) = self.profile_ids.get(speaker) {
            return SpeakerTraceKey::Enrolled {
                profile_id: speaker.to_string(),
                label: label.clone(),
            };
        }
        if let Some((profile_id, label)) = self.profile_labels.get(speaker) {
            return SpeakerTraceKey::Enrolled {
                profile_id: profile_id.clone(),
                label: label.clone(),
            };
        }
        SpeakerTraceKey::Anonymous {
            label: speaker.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_resolution_prefers_existing_local_speaker_directory() {
        let temp = tempfile::tempdir().expect("tempdir");
        let local = temp.path().join(LOCAL_SPEAKER_DIRECTORY);
        fs::create_dir_all(&local).expect("local speaker directory");

        let resolved =
            resolve_speaker_directory(&SpeakerDirectorySelection::default(), temp.path()).unwrap();

        assert_eq!(resolved.path, local);
        assert_eq!(resolved.scope, ResolvedSpeakerDirectoryScope::Local);
    }

    #[test]
    fn explicit_resolution_is_relative_to_current_directory() {
        let temp = tempfile::tempdir().expect("tempdir");

        let resolved = resolve_speaker_directory(
            &SpeakerDirectorySelection {
                scope: SpeakerDirectoryScope::Auto,
                explicit_path: Some(PathBuf::from("controlled-speakers")),
            },
            temp.path(),
        )
        .unwrap();

        assert_eq!(resolved.path, temp.path().join("controlled-speakers"));
        assert_eq!(resolved.scope, ResolvedSpeakerDirectoryScope::Explicit);
    }

    #[test]
    fn validates_upstream_speaker_library_snapshot() {
        let temp = tempfile::tempdir().expect("tempdir");
        let directory = temp.path().join("speakers");
        fs::create_dir_all(&directory).expect("speaker directory");
        fs::write(speaker_library_path(&directory), valid_library_json()).expect("library");

        let validation = validate_speaker_library(&directory).unwrap();

        assert_eq!(validation.profile_count, 1);
        assert_eq!(validation.path, speaker_library_path(&directory));
    }

    #[test]
    fn validation_rejects_incompatible_speaker_library_snapshot() {
        let temp = tempfile::tempdir().expect("tempdir");
        let directory = temp.path().join("speakers");
        fs::create_dir_all(&directory).expect("speaker directory");
        fs::write(
            speaker_library_path(&directory),
            valid_library_json().replace("\"values\": [1.0, 0.0]", "\"values\": [2.0, 0.0]"),
        )
        .expect("library");

        let error = validate_speaker_library(&directory)
            .expect_err("incompatible library should fail")
            .to_string();

        assert!(error.contains("malformed or incompatible"));
        assert!(error.contains("normalized"));
    }

    #[test]
    fn validation_rejects_trace_provenance_in_library_snapshot() {
        let temp = tempfile::tempdir().expect("tempdir");
        let directory = temp.path().join("speakers");
        fs::create_dir_all(&directory).expect("speaker directory");
        fs::write(
            speaker_library_path(&directory),
            valid_library_json().replace(
                "\"profiles\": [",
                "\"speaker_trace\": {\"files\": []}, \"profiles\": [",
            ),
        )
        .expect("library");

        let error = validate_speaker_library(&directory)
            .expect_err("trace provenance should fail")
            .to_string();

        assert!(error.contains("unexpected top-level field `speaker_trace`"));
    }

    #[test]
    fn updates_profile_label_and_metadata_without_changing_id() {
        let temp = tempfile::tempdir().expect("tempdir");
        let directory = temp.path().join("speakers");
        fs::create_dir_all(&directory).expect("speaker directory");
        fs::write(speaker_library_path(&directory), valid_library_json()).expect("library");

        let validation = update_speaker_profile(
            &directory,
            "speaker-a",
            SpeakerProfileEdit {
                id: Some("speaker-a".to_string()),
                label: Some("Updated Speaker".to_string()),
                metadata: Some(BTreeMap::from([
                    ("note".to_string(), "changed".to_string()),
                    ("external_id".to_string(), "crm-123".to_string()),
                ])),
            },
        )
        .expect("profile edit");

        assert_eq!(validation.profile_count, 1);
        let state = read_speaker_directory_state(&ResolvedSpeakerDirectory {
            path: directory.clone(),
            scope: ResolvedSpeakerDirectoryScope::Explicit,
        })
        .expect("state");
        assert_eq!(state.profiles[0].id, "speaker-a");
        assert_eq!(state.profiles[0].label, "Updated Speaker");
        assert_eq!(
            state.profiles[0]
                .metadata
                .get("external_id")
                .map(String::as_str),
            Some("crm-123")
        );
    }

    #[test]
    fn update_rejects_profile_id_mutation() {
        let temp = tempfile::tempdir().expect("tempdir");
        let directory = temp.path().join("speakers");
        fs::create_dir_all(&directory).expect("speaker directory");
        fs::write(speaker_library_path(&directory), valid_library_json()).expect("library");

        let error = update_speaker_profile(
            &directory,
            "speaker-a",
            SpeakerProfileEdit {
                id: Some("speaker-renamed".to_string()),
                label: Some("Updated Speaker".to_string()),
                metadata: None,
            },
        )
        .expect_err("id mutation should fail")
        .to_string();

        assert!(error.contains("profile ids are immutable"));
        let state = read_speaker_directory_state(&ResolvedSpeakerDirectory {
            path: directory.clone(),
            scope: ResolvedSpeakerDirectoryScope::Explicit,
        })
        .expect("state");
        assert_eq!(state.profiles[0].id, "speaker-a");
        assert_eq!(state.profiles[0].label, "Speaker A");
    }

    #[test]
    fn update_rejects_invalid_label_before_write() {
        let temp = tempfile::tempdir().expect("tempdir");
        let directory = temp.path().join("speakers");
        fs::create_dir_all(&directory).expect("speaker directory");
        fs::write(speaker_library_path(&directory), valid_library_json()).expect("library");

        let error = update_speaker_profile(
            &directory,
            "speaker-a",
            SpeakerProfileEdit {
                id: None,
                label: Some("   ".to_string()),
                metadata: None,
            },
        )
        .expect_err("empty label should fail")
        .to_string();

        assert!(error.contains("profile label must not be empty"));
        let saved = fs::read_to_string(speaker_library_path(&directory)).expect("library");
        assert!(saved.contains("\"label\": \"Speaker A\""));
    }

    #[test]
    fn deletes_profile_and_validates_saved_library() {
        let temp = tempfile::tempdir().expect("tempdir");
        let directory = temp.path().join("speakers");
        fs::create_dir_all(&directory).expect("speaker directory");
        fs::write(speaker_library_path(&directory), two_profile_library_json()).expect("library");

        let validation = delete_speaker_profile(&directory, "speaker-b").expect("delete profile");

        assert_eq!(validation.profile_count, 1);
        let state = read_speaker_directory_state(&ResolvedSpeakerDirectory {
            path: directory.clone(),
            scope: ResolvedSpeakerDirectoryScope::Explicit,
        })
        .expect("state");
        assert_eq!(state.profiles.len(), 1);
        assert_eq!(state.profiles[0].id, "speaker-a");
    }

    #[test]
    fn deleting_final_profile_removes_library_file() {
        let temp = tempfile::tempdir().expect("tempdir");
        let directory = temp.path().join("speakers");
        fs::create_dir_all(&directory).expect("speaker directory");
        let library_path = speaker_library_path(&directory);
        fs::write(&library_path, valid_library_json()).expect("library");

        let validation = delete_speaker_profile(&directory, "speaker-a").expect("delete profile");

        assert_eq!(validation.profile_count, 0);
        assert!(!library_path.exists());
        let state = read_speaker_directory_state(&ResolvedSpeakerDirectory {
            path: directory.clone(),
            scope: ResolvedSpeakerDirectoryScope::Explicit,
        })
        .expect("state");
        assert_eq!(
            state.library.status,
            SpeakerLibraryValidationStatus::Missing
        );
        assert!(state.profiles.is_empty());
    }

    #[test]
    fn rejects_draft_profile_creation_without_embeddings() {
        let error = reject_draft_speaker_profile_creation()
            .expect_err("draft profile creation should fail")
            .to_string();

        assert!(error.contains("without embeddings is not supported"));
    }

    #[test]
    fn rebuild_trace_indexes_whisperx_and_native_json_transcripts() {
        let temp = tempfile::tempdir().expect("tempdir");
        let directory = temp.path().join("speakers");
        let scan_root = temp.path().join("outputs");
        fs::create_dir_all(&directory).expect("speaker directory");
        fs::create_dir_all(&scan_root).expect("scan root");
        fs::write(speaker_library_path(&directory), two_profile_library_json()).expect("library");
        fs::write(
            scan_root.join("sample.json"),
            r#"{
              "language": "en",
              "segments": [
                {"id": 0, "start": 0.0, "end": 1.2, "text": "Known by id", "speaker": "speaker-a"},
                {"id": 1, "start": 1.2, "end": 2.0, "text": "Still known", "speaker": "speaker-a"},
                {"id": 2, "start": 2.5, "end": 3.0, "text": "Unknown turn", "speaker": "SPEAKER_99"}
              ]
            }"#,
        )
        .expect("whisperx json");
        fs::write(
            scan_root.join("sample.native.json"),
            r#"{
              "segments": [
                {
                  "index": 0,
                  "startSeconds": 3.0,
                  "endSeconds": 4.5,
                  "text": "Known by label",
                  "speaker": "Speaker B",
                  "isFinal": true
                }
              ]
            }"#,
        )
        .expect("native json");
        fs::write(scan_root.join("sample.srt"), "not parsed").expect("srt");
        fs::write(
            scan_root.join("report.json"),
            r#"{"kind": "not a transcript"}"#,
        )
        .expect("unrelated json");

        let report = rebuild_speaker_trace(&directory, &scan_root).expect("trace rebuild");

        assert_eq!(report.trace.errors, Vec::new());
        assert_eq!(report.stats.scanned_files, 4);
        assert_eq!(report.stats.accepted_entries, 4);
        assert_eq!(report.stats.ignored_non_json_files, 1);
        assert_eq!(report.stats.malformed_json_errors, 0);
        assert_eq!(report.trace.speakers.len(), 3);
        let speaker_a = report
            .trace
            .speakers
            .iter()
            .find(|speaker| speaker.profile_id.as_deref() == Some("speaker-a"))
            .expect("speaker-a trace");
        assert_eq!(speaker_a.kind, SpeakerTraceSpeakerKind::Enrolled);
        assert_eq!(speaker_a.label.as_deref(), Some("Speaker A"));
        assert_eq!(speaker_a.files.len(), 1);
        assert_eq!(speaker_a.files[0].segment_count, 2);
        assert_eq!(speaker_a.files[0].total_duration, 2.0);
        assert_eq!(speaker_a.files[0].spans[0].start_seconds, Some(0.0));
        assert_eq!(speaker_a.files[0].spans[0].end_seconds, Some(1.2));
        assert_eq!(speaker_a.files[0].spans[0].snippet, "Known by id");

        let speaker_b = report
            .trace
            .speakers
            .iter()
            .find(|speaker| speaker.profile_id.as_deref() == Some("speaker-b"))
            .expect("speaker-b trace");
        assert_eq!(speaker_b.kind, SpeakerTraceSpeakerKind::Enrolled);
        assert_eq!(speaker_b.label.as_deref(), Some("Speaker B"));
        assert_eq!(speaker_b.files[0].spans[0].snippet, "Known by label");

        let anonymous = report
            .trace
            .speakers
            .iter()
            .find(|speaker| speaker.anonymous_label.as_deref() == Some("SPEAKER_99"))
            .expect("anonymous trace");
        assert_eq!(anonymous.kind, SpeakerTraceSpeakerKind::Anonymous);
        assert_eq!(anonymous.files[0].spans[0].snippet, "Unknown turn");

        let saved = fs::read_to_string(speaker_trace_path(&directory)).expect("saved trace");
        assert!(saved.contains("\"sourceFile\""));
        assert!(saved.contains("\"segmentCount\""));
        assert!(saved.contains("\"totalDuration\""));
        assert!(saved.contains("\"spans\""));
    }

    #[test]
    fn rebuild_trace_reports_malformed_json_without_aborting_valid_files() {
        let temp = tempfile::tempdir().expect("tempdir");
        let directory = temp.path().join("speakers");
        let scan_root = temp.path().join("outputs");
        fs::create_dir_all(&directory).expect("speaker directory");
        fs::create_dir_all(&scan_root).expect("scan root");
        fs::write(speaker_library_path(&directory), valid_library_json()).expect("library");
        fs::write(
            scan_root.join("valid.json"),
            r#"{"segments": [{"id": 0, "start": 0.0, "end": 1.0, "text": "ok", "speaker": "speaker-a"}]}"#,
        )
        .expect("valid json");
        fs::write(scan_root.join("broken.json"), "{").expect("broken json");

        let report = rebuild_speaker_trace(&directory, &scan_root).expect("trace rebuild");

        assert_eq!(report.trace.speakers.len(), 1);
        assert_eq!(report.stats.scanned_files, 2);
        assert_eq!(report.stats.accepted_entries, 1);
        assert_eq!(report.stats.ignored_non_json_files, 0);
        assert_eq!(report.stats.malformed_json_errors, 1);
        assert_eq!(report.trace.errors.len(), 1);
        assert!(report.trace.errors[0].source_file.ends_with("broken.json"));
        assert!(report.trace.errors[0]
            .message
            .contains("malformed transcript JSON"));
        let saved = fs::read_to_string(speaker_trace_path(&directory)).expect("saved trace");
        assert!(saved.contains("broken.json"));
        assert!(saved.contains("speaker-a"));
    }

    fn valid_library_json() -> String {
        r#"{
          "version": 1,
          "embedding_model": {
            "family": "SpeechBrain",
            "name": "spkrec",
            "version": "1",
            "dimensions": 2
          },
          "profiles": [{
            "id": "speaker-a",
            "label": "Speaker A",
            "embeddings": [{
              "values": [1.0, 0.0],
              "model": {
                "family": "SpeechBrain",
                "name": "spkrec",
                "version": "1",
                "dimensions": 2
              },
              "sample_rate": 16000
            }],
            "metadata": {
              "note": "fixture"
            }
          }]
        }"#
        .to_string()
    }

    fn two_profile_library_json() -> String {
        valid_library_json().replace(
            r#"{
            "id": "speaker-a",
            "label": "Speaker A",
            "embeddings": [{
              "values": [1.0, 0.0],
              "model": {
                "family": "SpeechBrain",
                "name": "spkrec",
                "version": "1",
                "dimensions": 2
              },
              "sample_rate": 16000
            }],
            "metadata": {
              "note": "fixture"
            }
          }"#,
            r#"{
            "id": "speaker-a",
            "label": "Speaker A",
            "embeddings": [{
              "values": [1.0, 0.0],
              "model": {
                "family": "SpeechBrain",
                "name": "spkrec",
                "version": "1",
                "dimensions": 2
              },
              "sample_rate": 16000
            }],
            "metadata": {
              "note": "fixture"
            }
          },
          {
            "id": "speaker-b",
            "label": "Speaker B",
            "embeddings": [{
              "values": [0.0, 1.0],
              "model": {
                "family": "SpeechBrain",
                "name": "spkrec",
                "version": "1",
                "dimensions": 2
              },
              "sample_rate": 16000
            }],
            "metadata": {
              "note": "second fixture"
            }
          }"#,
        )
    }
}

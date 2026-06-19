use std::fs;
use std::path::{Path, PathBuf};

use audio_analysis_speakers::SpeakerLibrary;
use serde_json::Value;

use crate::NativeWhisperxError;

pub const LOCAL_SPEAKER_DIRECTORY: &str = ".native-whisperx/speakers";
pub const GLOBAL_SPEAKER_DIRECTORY_APP: &str = "native-whisperx";
pub const GLOBAL_SPEAKER_DIRECTORY_NAME: &str = "speakers";
pub const SPEAKER_LIBRARY_FILE: &str = "library.json";
pub const SPEAKER_TRACE_FILE: &str = "speaker-trace.json";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeakerDirectorySelection {
    pub scope: SpeakerDirectoryScope,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeakerDirectoryScope {
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
    validate_canonical_speaker_library_json(library_path, &bytes)
}

fn validate_canonical_speaker_library_json(
    library_path: &Path,
    bytes: &[u8],
) -> Result<SpeakerLibraryValidation, NativeWhisperxError> {
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

    Ok(SpeakerLibraryValidation {
        path: library_path.to_path_buf(),
        profile_count: library.len(),
    })
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

fn absolute_from_base(base: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
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
}

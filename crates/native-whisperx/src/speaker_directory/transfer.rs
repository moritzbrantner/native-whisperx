//! Speaker Directory transfer policy over the canonical Speaker Library owner.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use audio_analysis_speakers::{
    SpeakerLibrary, SpeakerLibraryImportMode as CanonicalImportMode, SpeakerLibraryImportPlan,
};
use serde::{Deserialize, Serialize};

use super::{
    load_speaker_library_if_present, speaker_library_error, speaker_library_path,
    speaker_profile_status, SpeakerLibraryValidation,
};
use crate::NativeWhisperxError;

/// The user's explicit treatment of identities already in the selected directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpeakerLibraryImportMode {
    Merge,
    Replace,
}

/// Product-safe export facts. Embedding vectors stay in the canonical owner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerLibraryExportSummary {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub profile_count: usize,
    pub confirmed_profiles: usize,
    pub draft_profiles: usize,
    pub unclassified_profiles: usize,
}

/// A specific reviewed local export. Applying it writes the reviewed snapshot,
/// even if a later edit changes the active library in the meantime.
pub struct SpeakerLibraryExportPreview {
    library: SpeakerLibrary,
    summary: SpeakerLibraryExportSummary,
}

impl std::fmt::Debug for SpeakerLibraryExportPreview {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SpeakerLibraryExportPreview")
            .field("summary", &self.summary)
            .finish_non_exhaustive()
    }
}

impl SpeakerLibraryExportPreview {
    pub fn summary(&self) -> &SpeakerLibraryExportSummary {
        &self.summary
    }

    /// Applies the caller's explicit local voice-data export confirmation.
    pub fn apply(self) -> Result<SpeakerLibraryTransferReceipt, NativeWhisperxError> {
        self.library
            .export_new(&self.summary.destination)
            .map_err(speaker_library_error)?;
        Ok(SpeakerLibraryTransferReceipt {
            path: self.summary.destination,
            profile_count: self.library.len(),
            changed: true,
            backup_path: None,
        })
    }
}

/// Product-safe import confirmation facts, including the selected paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerLibraryImportSummary {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub mode: SpeakerLibraryImportMode,
    pub existing_profiles: usize,
    pub incoming_profiles: usize,
    pub confirmed_profiles: usize,
    pub draft_profiles: usize,
    pub unclassified_profiles: usize,
    pub added_profiles: usize,
    pub unchanged_profiles: usize,
    pub replaced_profiles: usize,
    pub removed_profiles: usize,
    pub resulting_profiles: usize,
    pub changed: bool,
    pub backup_path: Option<PathBuf>,
}

/// Binds the validated canonical plan to the directory and backup shown to the user.
#[derive(Debug)]
pub struct SpeakerLibraryImportPreview {
    plan: SpeakerLibraryImportPlan,
    directory: PathBuf,
    backup: PathBuf,
    summary: SpeakerLibraryImportSummary,
}

impl SpeakerLibraryImportPreview {
    pub fn summary(&self) -> &SpeakerLibraryImportSummary {
        &self.summary
    }

    /// Applies the caller's explicit import decision. The canonical owner checks
    /// freshness, model compatibility, atomic promotion and no-change behavior.
    pub fn apply(self) -> Result<SpeakerLibraryTransferReceipt, NativeWhisperxError> {
        if !self.summary.changed && !self.directory.try_exists()? {
            return Ok(SpeakerLibraryTransferReceipt {
                path: self.summary.destination,
                profile_count: self.summary.resulting_profiles,
                changed: false,
                backup_path: None,
            });
        }
        std::fs::create_dir_all(&self.directory)?;
        let receipt = self
            .plan
            .commit(&self.summary.destination, &self.backup)
            .map_err(speaker_library_error)?;
        Ok(SpeakerLibraryTransferReceipt {
            path: self.summary.destination,
            profile_count: receipt.summary.resulting_profiles,
            changed: receipt.changed,
            backup_path: receipt.backup_path,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerLibraryTransferReceipt {
    pub path: PathBuf,
    pub profile_count: usize,
    pub changed: bool,
    pub backup_path: Option<PathBuf>,
}

/// Creates an explicitly selected empty library. Existing identity files are
/// never overwritten, and Speaker Trace is never an input or output of this operation.
pub fn create_speaker_library(
    directory: &Path,
) -> Result<SpeakerLibraryValidation, NativeWhisperxError> {
    std::fs::create_dir_all(directory)?;
    let path = speaker_library_path(directory);
    SpeakerLibrary::new()
        .export_new(&path)
        .map_err(speaker_library_error)?;
    Ok(SpeakerLibraryValidation {
        path,
        profile_count: 0,
    })
}

/// Reads one canonical snapshot for an explicit local export confirmation.
/// Path authorization and obtaining consent are the host application's boundary.
pub fn prepare_speaker_library_export(
    directory: &Path,
    destination: &Path,
) -> Result<SpeakerLibraryExportPreview, NativeWhisperxError> {
    let source = speaker_library_path(directory);
    let library = SpeakerLibrary::load(&source).map_err(speaker_library_error)?;
    let (confirmed_profiles, draft_profiles, unclassified_profiles) = profile_counts(&library);
    let summary = SpeakerLibraryExportSummary {
        source,
        destination: destination.to_path_buf(),
        profile_count: library.len(),
        confirmed_profiles,
        draft_profiles,
        unclassified_profiles,
    };
    Ok(SpeakerLibraryExportPreview { library, summary })
}

/// Plans a canonical import without changing files or creating a directory.
/// Malformed sources, incompatible models and unresolved collisions are rejected
/// by the canonical owner before the product can present a successful preview.
pub fn prepare_speaker_library_import(
    directory: &Path,
    source: &Path,
    mode: SpeakerLibraryImportMode,
) -> Result<SpeakerLibraryImportPreview, NativeWhisperxError> {
    let incoming = SpeakerLibrary::load(source).map_err(speaker_library_error)?;
    let existing = load_speaker_library_if_present(directory)?;
    let canonical_mode = match mode {
        SpeakerLibraryImportMode::Merge => CanonicalImportMode::Merge,
        SpeakerLibraryImportMode::Replace => CanonicalImportMode::Replace,
    };
    let plan = existing
        .as_ref()
        .unwrap_or(&SpeakerLibrary::new())
        .prepare_import(&incoming, canonical_mode)
        .map_err(speaker_library_error)?;
    let counts = plan.summary();
    let (confirmed_profiles, draft_profiles, unclassified_profiles) = profile_counts(&incoming);
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| {
            NativeWhisperxError::InvalidConfig(format!(
                "cannot name a Speaker Library backup: {error}"
            ))
        })?
        .as_nanos();
    let backup = directory.join(format!(".library.json.before-import-{nonce}.backup"));
    let summary = SpeakerLibraryImportSummary {
        source: source.to_path_buf(),
        destination: speaker_library_path(directory),
        mode,
        existing_profiles: counts.existing_profiles,
        incoming_profiles: counts.incoming_profiles,
        confirmed_profiles,
        draft_profiles,
        unclassified_profiles,
        added_profiles: counts.added_profiles,
        unchanged_profiles: counts.unchanged_profiles,
        replaced_profiles: counts.replaced_profiles,
        removed_profiles: counts.removed_profiles,
        resulting_profiles: counts.resulting_profiles,
        changed: counts.changed,
        backup_path: (existing.is_some() && counts.changed).then(|| backup.clone()),
    };
    Ok(SpeakerLibraryImportPreview {
        plan,
        directory: directory.to_path_buf(),
        backup,
        summary,
    })
}

fn profile_counts(library: &SpeakerLibrary) -> (usize, usize, usize) {
    let mut confirmed = 0;
    let mut draft = 0;
    let mut other = 0;
    for profile in library.profiles() {
        match speaker_profile_status(profile).as_str() {
            "confirmed" => confirmed += 1,
            "draft" => draft += 1,
            _ => other += 1,
        }
    }
    (confirmed, draft, other)
}

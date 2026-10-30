//! Session-scoped Speaker Library consent over narrowly typed local operations.

use std::path::PathBuf;

use anyhow::{bail, Context};
use native_whisperx::{
    create_speaker_library, prepare_speaker_library_export, prepare_speaker_library_import,
    read_speaker_directory_state, resolve_speaker_directory, speaker_library_path,
    ResolvedSpeakerDirectory, SpeakerDirectoryScope, SpeakerDirectorySelection,
    SpeakerLibraryExportPreview, SpeakerLibraryImportMode, SpeakerLibraryImportPreview,
    SpeakerLibraryValidationStatus, SPEAKER_LIBRARY_FILE,
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::workbench::WorkbenchSession;

#[derive(Default)]
pub(crate) struct LibrarySession {
    next_id: u64,
    pending: Option<PendingOperation>,
}

struct PendingOperation {
    id: String,
    action: PendingAction,
}

enum PendingAction {
    Export(SpeakerLibraryExportPreview),
    New(ResolvedSpeakerDirectory),
    Import(Box<SpeakerLibraryImportPreview>),
}

#[derive(Deserialize)]
#[serde(tag = "scope", rename_all = "camelCase", deny_unknown_fields)]
enum DirectoryChoice {
    Auto,
    Local,
    Global,
    Explicit { path: PathBuf },
}

#[derive(Clone, Copy)]
enum SelectionIntent {
    Load,
    Create,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LoadRequest {
    selection: DirectoryChoice,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum PreviewRequest {
    Export {
        destination: PathBuf,
    },
    New {
        selection: DirectoryChoice,
    },
    Import {
        source: PathBuf,
        mode: SpeakerLibraryImportMode,
    },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Confirmation {
    id: String,
    confirmed: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PreviewId {
    id: String,
}

impl LibrarySession {
    pub(crate) fn operation(
        &mut self,
        path: &str,
        body: &[u8],
        resolved: &mut ResolvedSpeakerDirectory,
        workbench: &mut WorkbenchSession,
    ) -> anyhow::Result<Value> {
        match path {
            "/api/library/preview" => {
                workbench.ensure_idle()?;
                self.pending = None;
                let request: PreviewRequest =
                    serde_json::from_slice(body).context("invalid library preview request")?;
                let (kind, summary, action) = match request {
                    PreviewRequest::Export { destination } => {
                        let destination = workbench.authorized_new_path(&destination)?;
                        let export = prepare_speaker_library_export(&resolved.path, &destination)?;
                        (
                            "export",
                            serde_json::to_value(export.summary())?,
                            PendingAction::Export(export),
                        )
                    }
                    PreviewRequest::Import { source, mode } => {
                        let source = workbench.authorized_path(&source)?;
                        if !source.is_file() {
                            bail!("choose a portable Speaker Library file");
                        }
                        let import = prepare_speaker_library_import(&resolved.path, &source, mode)?;
                        (
                            "import",
                            serde_json::to_value(import.summary())?,
                            PendingAction::Import(Box::new(import)),
                        )
                    }
                    PreviewRequest::New { selection } => {
                        let candidate =
                            resolve_choice(selection, SelectionIntent::Create, workbench)?;
                        let destination = speaker_library_path(&candidate.path);
                        if destination.try_exists()? {
                            bail!("a Speaker Library already exists there; choose a new directory");
                        }
                        (
                            "new",
                            json!({"directory": candidate.path, "destination": destination, "profileCount": 0}),
                            PendingAction::New(candidate),
                        )
                    }
                };
                self.next_id = self
                    .next_id
                    .checked_add(1)
                    .context("library preview identifier exhausted")?;
                let id = self.next_id.to_string();
                let response = json!({"id": id, "kind": kind, "summary": summary});
                self.pending = Some(PendingOperation { id, action });
                Ok(response)
            }
            "/api/library/confirm" => {
                workbench.ensure_idle()?;
                let request: Confirmation = serde_json::from_slice(body)?;
                if !request.confirmed {
                    bail!("explicit confirmation is required for this Speaker Library operation");
                }
                if self
                    .pending
                    .as_ref()
                    .is_none_or(|pending| pending.id != request.id)
                {
                    bail!("library preview expired; review a fresh summary");
                }
                let pending = self.pending.take().context("library preview unavailable")?;
                match pending.action {
                    PendingAction::Export(export) => {
                        Ok(json!({"kind": "export", "receipt": export.apply()?}))
                    }
                    PendingAction::Import(import) => {
                        let receipt = import.apply()?;
                        let state = read_speaker_directory_state(resolved)?;
                        Ok(json!({"kind": "import", "receipt": receipt, "state": state}))
                    }
                    PendingAction::New(candidate) => {
                        let receipt = create_speaker_library(&candidate.path)?;
                        let state = read_speaker_directory_state(&candidate)?;
                        workbench.select_speaker_directory(candidate.path.clone())?;
                        *resolved = candidate;
                        Ok(
                            json!({"kind": "new", "receipt": {"path": receipt.path, "profileCount": receipt.profile_count, "changed": true, "backupPath": null}, "state": state}),
                        )
                    }
                }
            }
            "/api/library/load" => {
                workbench.ensure_idle()?;
                let request: LoadRequest = serde_json::from_slice(body)?;
                let candidate =
                    resolve_choice(request.selection, SelectionIntent::Load, workbench)?;
                let state = read_speaker_directory_state(&candidate)?;
                if state.library.status != SpeakerLibraryValidationStatus::Valid {
                    bail!(
                        "{}",
                        state
                            .library
                            .message
                            .as_deref()
                            .unwrap_or("select a valid existing Speaker Library")
                    );
                }
                workbench.select_speaker_directory(candidate.path.clone())?;
                *resolved = candidate;
                self.pending = None;
                Ok(json!({"kind": "load", "state": state}))
            }
            "/api/library/cancel" => {
                let request: PreviewId = serde_json::from_slice(body)?;
                let cancelled = self
                    .pending
                    .as_ref()
                    .is_some_and(|pending| pending.id == request.id);
                if cancelled {
                    self.pending = None;
                }
                Ok(json!({"cancelled": cancelled}))
            }
            _ => bail!("unknown library operation"),
        }
    }
}

fn resolve_choice(
    choice: DirectoryChoice,
    intent: SelectionIntent,
    workbench: &WorkbenchSession,
) -> anyhow::Result<ResolvedSpeakerDirectory> {
    let selection = match choice {
        DirectoryChoice::Auto => SpeakerDirectorySelection::default(),
        DirectoryChoice::Local => SpeakerDirectorySelection {
            scope: SpeakerDirectoryScope::Local,
            explicit_path: None,
        },
        DirectoryChoice::Global => SpeakerDirectorySelection {
            scope: SpeakerDirectoryScope::Global,
            explicit_path: None,
        },
        DirectoryChoice::Explicit { path } => {
            let mut directory = match intent {
                SelectionIntent::Load => workbench.authorized_path(&path)?,
                SelectionIntent::Create => workbench.authorized_new_path(&path)?,
            };
            if directory.is_file() {
                if matches!(intent, SelectionIntent::Create) {
                    bail!("choose a directory for the new Speaker Library");
                }
                if directory
                    .file_name()
                    .is_none_or(|name| name != SPEAKER_LIBRARY_FILE)
                {
                    bail!("Load selects a Speaker Directory or library.json; use Import for a portable copy");
                }
                directory = directory
                    .parent()
                    .context("library requires a directory")?
                    .to_path_buf();
            }
            SpeakerDirectorySelection {
                explicit_path: Some(directory),
                ..SpeakerDirectorySelection::default()
            }
        }
    };
    Ok(resolve_speaker_directory(&selection, workbench.root())?)
}

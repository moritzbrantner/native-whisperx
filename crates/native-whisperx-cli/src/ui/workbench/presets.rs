//! Small product preset document, separate from Jobs and transcript data.
use super::*;
use native_whisperx::WorkbenchPreset;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const MAX_BYTES: u64 = 512 * 1024;
const MAX_PRESETS: usize = 50;
pub(super) const BUILTIN_IDS: [&str; 3] = [
    "builtin:transcription",
    "builtin:interview",
    "builtin:translation",
];

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct SavedPreset {
    pub id: String,
    pub preset: WorkbenchPreset,
}
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Document {
    schema_version: u8,
    presets: Vec<SavedPreset>,
    active_id: Option<String>,
}
impl Default for Document {
    fn default() -> Self {
        Self {
            schema_version: 1,
            presets: vec![],
            active_id: None,
        }
    }
}
pub(super) struct Presets {
    root: PathBuf,
    path: PathBuf,
    document: Document,
    problem: Option<String>,
    lock: Option<File>,
}
impl Presets {
    pub fn open(root: &Path) -> Self {
        let mut store = Self {
            root: root.to_owned(),
            path: root.join(".native-whisperx-workbench/presets.json"),
            document: Document::default(),
            problem: None,
            lock: None,
        };
        match store.read() {
            Ok(document) => store.document = document,
            Err(error) => store.problem = Some(error.to_string()),
        }
        store
    }
    pub fn value(&self) -> Value {
        json!({"presets":self.document.presets,"activeId":self.document.active_id,
            "problem":self.problem,"canReset":self.problem.is_some() && self.validate_paths().is_ok()})
    }
    pub fn save(&mut self, id: Option<String>, preset: WorkbenchPreset) -> anyhow::Result<bool> {
        preset.validate()?;
        self.begin()?;
        let mut document = self.document.clone();
        let id = match id {
            Some(id) => {
                let entry = document
                    .presets
                    .iter_mut()
                    .find(|entry| entry.id == id)
                    .context("saved preset does not exist")?;
                entry.preset = preset;
                id
            }
            None => {
                if document.presets.len() >= MAX_PRESETS {
                    bail!("Keep at most 50 saved presets; delete an unused preset first");
                }
                let id = super::super::server::generate_session_token()?;
                document.presets.push(SavedPreset {
                    id: id.clone(),
                    preset,
                });
                id
            }
        };
        document.active_id = Some(id);
        self.persist(document)
    }
    pub fn select(&mut self, id: Option<String>) -> anyhow::Result<bool> {
        // Selecting the unsaved default must remain a read-only no-op on first use.
        if id.is_none() && !self.path.exists() && self.problem.is_none() {
            return Ok(false);
        }
        self.begin()?;
        if let Some(id) = &id {
            if !BUILTIN_IDS.contains(&id.as_str())
                && !self.document.presets.iter().any(|entry| entry.id == *id)
            {
                bail!("preset does not exist");
            }
        }
        let mut document = self.document.clone();
        document.active_id = id;
        self.persist(document)
    }
    pub fn delete(&mut self, id: &str) -> anyhow::Result<bool> {
        self.begin()?;
        if !self.document.presets.iter().any(|entry| entry.id == id) {
            bail!("saved preset does not exist");
        }
        let mut document = self.document.clone();
        document.presets.retain(|entry| entry.id != id);
        if document.active_id.as_deref() == Some(id) {
            document.active_id = None;
        }
        self.persist(document)
    }
    pub fn reset(&mut self) -> anyhow::Result<PathBuf> {
        self.validate_paths()?;
        if self.problem.is_none() {
            bail!("saved presets do not require recovery");
        }
        self.acquire_lock()?;
        let backup = self.path.with_file_name(format!(
            "presets-backup-{}.json",
            super::super::server::generate_session_token()?
        ));
        let mut input = File::open(&self.path)?;
        let mut output = private_file(&backup, true)?;
        // Preserve even an oversized/older file without holding its bytes in memory.
        std::io::copy(&mut input, &mut output)?;
        output.sync_all()?;
        self.persist(Document::default())?;
        self.problem = None;
        Ok(backup)
    }
    fn read(&self) -> anyhow::Result<Document> {
        self.validate_paths()?;
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Document::default())
            }
            Err(error) => return Err(error.into()),
        };
        let mut bytes = Vec::new();
        file.take(MAX_BYTES + 1).read_to_end(&mut bytes)?;
        if bytes.len() as u64 > MAX_BYTES {
            bail!("Saved presets exceed the 512 KiB limit; back up and reset them");
        }
        let document: Document = serde_json::from_slice(&bytes)
            .context("Saved presets are malformed; back up and reset them")?;
        if document.schema_version != 1 {
            bail!("This saved preset version is unsupported; back up and reset it");
        }
        if document.presets.len() > MAX_PRESETS {
            bail!("Saved presets exceed the 50 preset limit");
        }
        let mut ids = std::collections::BTreeSet::new();
        for entry in &document.presets {
            if entry.id.len() != 64
                || !entry.id.bytes().all(|byte| byte.is_ascii_hexdigit())
                || !ids.insert(entry.id.as_str())
            {
                bail!("Saved preset identifiers are invalid; back up and reset them");
            }
            entry
                .preset
                .validate()
                .context("A saved preset is unsupported; back up and reset it")?;
        }
        if document
            .active_id
            .as_ref()
            .is_some_and(|id| !BUILTIN_IDS.contains(&id.as_str()) && !ids.contains(id.as_str()))
        {
            bail!("The active preset is missing; back up and reset saved presets");
        }
        Ok(document)
    }
    fn begin(&mut self) -> anyhow::Result<()> {
        if let Some(problem) = &self.problem {
            bail!("{problem}");
        }
        self.acquire_lock()?;
        // Another session may have saved since this session opened. Refresh only
        // after obtaining the writer lock, before deriving a mutation.
        self.document = self.read()?;
        Ok(())
    }
    fn acquire_lock(&mut self) -> anyhow::Result<()> {
        self.validate_paths()?;
        if self.lock.is_none() {
            let parent = self.path.parent().context("preset parent")?;
            if !parent.exists() {
                let mut builder = std::fs::DirBuilder::new();
                #[cfg(unix)]
                {
                    use std::os::unix::fs::DirBuilderExt;
                    builder.mode(0o700);
                }
                builder.create(parent)?;
            }
            let file = private_file(&self.path.with_extension("lock"), false)?;
            file.try_lock().context(
                "Another local session owns the saved presets; close it before changing presets",
            )?;
            self.lock = Some(file);
        }
        Ok(())
    }
    fn persist(&mut self, document: Document) -> anyhow::Result<bool> {
        self.validate_paths()?;
        if document == self.document && self.problem.is_none() {
            return Ok(false);
        }
        let bytes = serde_json::to_vec_pretty(&document)?;
        if bytes.len() as u64 > MAX_BYTES {
            bail!("Saved presets exceed the 512 KiB limit");
        }
        let temporary = self.path.with_file_name(format!(
            ".presets-{}.tmp",
            super::super::server::generate_session_token()?
        ));
        let result: anyhow::Result<()> = (|| {
            let mut output = private_file(&temporary, true)?;
            output.write_all(&bytes)?;
            output.sync_all()?;
            std::fs::rename(&temporary, &self.path)?;
            Ok(())
        })();
        if result.is_err() {
            let _ = std::fs::remove_file(&temporary);
        }
        result?;
        self.document = document;
        Ok(true)
    }
    fn validate_paths(&self) -> anyhow::Result<()> {
        let parent = self.path.parent().context("preset parent")?;
        if parent.exists() && (!parent.is_dir() || !parent.canonicalize()?.starts_with(&self.root))
        {
            bail!("Saved presets must remain inside the workbench directory");
        }
        for path in [&self.path, &self.path.with_extension("lock")] {
            match std::fs::symlink_metadata(path) {
                Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
                    bail!("Saved presets require regular local files")
                }
                Ok(_) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
        }
        Ok(())
    }
}
fn private_file(path: &Path, create_new: bool) -> anyhow::Result<File> {
    let mut options = OpenOptions::new();
    options.write(true);
    if create_new {
        options.create_new(true);
    } else {
        options.read(true).create(true).truncate(false);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    Ok(options.open(path)?)
}

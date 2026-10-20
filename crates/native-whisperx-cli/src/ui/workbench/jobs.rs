//! Product job history; inference stays in the existing Multi-Input run.
use super::*;
use native_whisperx::{MultiInputTranscriptionOutcome, OutputFile, OutputFormat};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::Write;

#[derive(Clone)]
pub(super) struct History {
    path: PathBuf,
    root: PathBuf,
    inner: Arc<Mutex<Stored>>,
}
struct Stored {
    document: Document,
    lock: Option<File>,
}
#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Document {
    schema_version: u8,
    jobs: Vec<Batch>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    reviews: Vec<SavedReview>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SavedReview {
    result_id: String,
    original_result_id: String,
    input: PathBuf,
    canonical: PathBuf,
    outputs: Vec<OutputFile>,
    translated: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Batch {
    id: String,
    status: Status,
    settings: WorkbenchRequest,
    speaker_directory: PathBuf,
    files: Vec<Row>,
    error: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Row {
    input: PathBuf,
    status: Status,
    tasks: BTreeMap<String, Value>,
    current_task: Option<String>,
    seconds: Option<f64>,
    error: Option<String>,
    outputs: Vec<OutputFile>,
}
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Status {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
    NotRun,
    Interrupted,
}
impl Status {
    fn unfinished(self) -> bool {
        matches!(self, Self::Queued | Self::Running)
    }
}

impl History {
    pub(super) fn open(root: &Path) -> anyhow::Result<Self> {
        let root = root.canonicalize()?;
        let path = root.join(".native-whisperx-workbench/jobs.json");
        validate_history_paths(&root, &path)?;
        let mut document = Document {
            schema_version: 1,
            jobs: vec![],
            reviews: vec![],
        };
        let mut lock = None;
        if path.exists() {
            if std::fs::metadata(&path)?.len() > 20 * 1024 * 1024 {
                bail!("Jobs history exceeds the local size limit");
            }
            lock = Some(lock_history(&path)?);
            document = serde_json::from_slice(&std::fs::read(&path)?)
                .context("invalid local Jobs history; preserve the file before repairing it")?;
            if document.schema_version != 1 {
                bail!("unsupported Jobs history version");
            }
        }
        let history = Self {
            path,
            root,
            inner: Arc::new(Mutex::new(Stored { document, lock })),
        };
        let mut stored = history
            .inner
            .lock()
            .map_err(|_| anyhow::anyhow!("Jobs unavailable"))?;
        let mut changed = false;
        for batch in &mut stored.document.jobs {
            if batch.status.unfinished() {
                changed = true;
                batch.status = Status::Interrupted;
                batch.error = Some(
                    "The previous local session stopped. Retry an unfinished file explicitly."
                        .into(),
                );
                for row in &mut batch.files {
                    if row.status.unfinished() {
                        row.status = Status::Interrupted;
                        row.current_task = None;
                        for task in row.tasks.values_mut() {
                            if task["status"] == "running" {
                                task["status"] = json!("interrupted");
                            }
                        }
                    }
                }
            }
        }
        if changed {
            history.save(&mut stored)?;
        }
        drop(stored);
        Ok(history)
    }
    pub(super) fn value(&self) -> anyhow::Result<Value> {
        let stored = self
            .inner
            .lock()
            .map_err(|_| anyhow::anyhow!("Jobs unavailable"))?;
        Ok(json!({"jobs": stored.document.jobs, "reviews": stored.document.reviews}))
    }
    fn change(&self, id: &str, update: impl FnOnce(&mut Batch)) -> anyhow::Result<()> {
        let mut stored = self
            .inner
            .lock()
            .map_err(|_| anyhow::anyhow!("Jobs unavailable"))?;
        let batch = stored
            .document
            .jobs
            .iter_mut()
            .find(|batch| batch.id == id)
            .context("job does not exist")?;
        update(batch);
        self.save(&mut stored)
    }
    fn save(&self, stored: &mut Stored) -> anyhow::Result<()> {
        if stored.lock.is_none() {
            std::fs::create_dir_all(self.path.parent().context("history parent")?)?;
            validate_history_paths(&self.root, &self.path)?;
            stored.lock = Some(lock_history(&self.path)?);
        }
        let bytes = serde_json::to_vec_pretty(&stored.document)?;
        if bytes.len() > 20 * 1024 * 1024 {
            bail!("Jobs history is full; archive its index before adding work");
        }
        let mut nonce = [0_u8; 16];
        getrandom::fill(&mut nonce).context("history staging identifier unavailable")?;
        let suffix = nonce
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let temporary = self.path.with_extension(format!("{suffix}.pending"));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options
            .open(&temporary)
            .context("could not stage Jobs history")?;
        let result = (|| -> anyhow::Result<()> {
            file.write_all(&bytes)?;
            file.sync_all()?;
            std::fs::rename(&temporary, &self.path)?;
            Ok(())
        })();
        if result.is_err() {
            let _ = std::fs::remove_file(temporary);
        }
        result
    }
}
fn validate_history_paths(root: &Path, path: &Path) -> anyhow::Result<()> {
    let parent = path.parent().context("history parent")?;
    if parent.exists() && !parent.canonicalize()?.starts_with(root) {
        bail!("Jobs history is outside the workbench directory");
    }
    for candidate in [path.to_path_buf(), path.with_extension("lock")] {
        if let Ok(metadata) = std::fs::symlink_metadata(&candidate) {
            if !metadata.is_file() || metadata.file_type().is_symlink() {
                bail!("Jobs history requires regular local files");
            }
        }
    }
    Ok(())
}
fn lock_history(path: &Path) -> anyhow::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(path.with_extension("lock"))?;
    file.try_lock()
        .context("another local session owns this directory's Jobs history")?;
    Ok(file)
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Queue {
    inputs: Vec<PathBuf>,
    settings: WorkbenchRequest,
}

impl WorkbenchSession {
    pub(crate) fn recorded_review(
        &self,
        result_id: &str,
    ) -> anyhow::Result<super::super::review::RecordedReview> {
        use super::super::review::RecordedReview;
        if result_id.starts_with("session:") {
            let state = self
                .state
                .lock()
                .map_err(|_| anyhow::anyhow!("result unavailable"))?;
            if state["status"] != "completed" || state["reviewId"] != result_id {
                bail!("this session result is unavailable; open Jobs or run Transcribe again");
            }
            let outputs: Vec<OutputFile> = serde_json::from_value(state["outputs"].clone())?;
            let canonical = outputs
                .iter()
                .find(|file| file.format == OutputFormat::NativeJson)
                .or_else(|| {
                    outputs
                        .iter()
                        .find(|file| file.format == OutputFormat::Json)
                })
                .context("select a JSON output before reviewing this result")?
                .path
                .clone();
            let input = PathBuf::from(
                state["input"]["path"]
                    .as_str()
                    .context("recording path unavailable")?,
            );
            return Ok(RecordedReview {
                input,
                canonical,
                translated: state["translated"].as_bool().unwrap_or(false),
            });
        }
        let stored = self
            .jobs
            .inner
            .lock()
            .map_err(|_| anyhow::anyhow!("Jobs unavailable"))?;
        if let Some(copy) = stored
            .document
            .reviews
            .iter()
            .find(|copy| copy.result_id == result_id)
        {
            return Ok(RecordedReview {
                input: copy.input.clone(),
                canonical: copy.canonical.clone(),
                translated: copy.translated,
            });
        }
        let (batch_id, index) = result_id
            .rsplit_once(':')
            .context("open a completed result from Jobs")?;
        let batch = stored
            .document
            .jobs
            .iter()
            .find(|batch| batch.id == batch_id)
            .context("recorded result is missing; open Jobs")?;
        let row = batch
            .files
            .get(index.parse::<usize>()?)
            .context("recorded file is missing; open Jobs")?;
        if row.status != Status::Completed {
            bail!("only completed files can be reviewed");
        }
        let canonical = row
            .outputs
            .iter()
            .find(|file| file.format == OutputFormat::NativeJson)
            .or_else(|| {
                row.outputs
                    .iter()
                    .find(|file| file.format == OutputFormat::Json)
            })
            .context("this older result has no JSON output; run it again with JSON selected")?
            .path
            .clone();
        Ok(RecordedReview {
            input: row.input.clone(),
            canonical,
            translated: batch.settings.options.translation.is_some(),
        })
    }

    pub(crate) fn record_corrected_review(
        &self,
        original_result_id: &str,
        input: PathBuf,
        outputs: Vec<OutputFile>,
        translated: bool,
    ) -> anyhow::Result<String> {
        let canonical = outputs
            .iter()
            .find(|output| output.format == OutputFormat::NativeJson)
            .context("corrected copy needs canonical Native JSON")?
            .path
            .clone();
        let result_id = format!("copy:{}", super::super::server::generate_session_token()?);
        let mut stored = self
            .jobs
            .inner
            .lock()
            .map_err(|_| anyhow::anyhow!("Jobs unavailable"))?;
        if stored.document.reviews.len() >= 500 {
            bail!("corrected-copy history is full; archive its index first");
        }
        stored.document.reviews.insert(
            0,
            SavedReview {
                result_id: result_id.clone(),
                original_result_id: original_result_id.into(),
                input,
                canonical,
                outputs,
                translated,
            },
        );
        if let Err(error) = self.jobs.save(&mut stored) {
            stored.document.reviews.remove(0);
            return Err(error);
        }
        Ok(result_id)
    }
    pub(super) fn job_result(&self, body: &[u8]) -> anyhow::Result<Value> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        struct OpenResult {
            batch_id: String,
            file_index: usize,
            output_index: usize,
        }
        let request: OpenResult = serde_json::from_slice(body)?;
        let output = {
            let stored = self
                .jobs
                .inner
                .lock()
                .map_err(|_| anyhow::anyhow!("Jobs unavailable"))?;
            let batch = stored
                .document
                .jobs
                .iter()
                .find(|batch| batch.id == request.batch_id)
                .context("job does not exist")?;
            let row = batch
                .files
                .get(request.file_index)
                .context("file does not exist")?;
            if row.status != Status::Completed {
                bail!("this file has no completed result");
            }
            row.outputs
                .get(request.output_index)
                .context("output does not exist")?
                .clone()
        };
        let path = self
            .authorized_path(&output.path)
            .context("completed output is unavailable; it may have moved")?;
        if !path.is_file() || std::fs::metadata(&path)?.len() > 32 * 1024 * 1024 {
            bail!("output is not a regular file within the viewer size limit");
        }
        let content = std::fs::read_to_string(&path)?;
        let text = if matches!(output.format, OutputFormat::Json | OutputFormat::NativeJson) {
            let document: Value =
                serde_json::from_str(&content).context("recorded JSON output is invalid")?;
            document["text"]
                .as_str()
                .or_else(|| document["transcript"]["text"].as_str())
                .map(str::to_owned)
                .unwrap_or_else(|| {
                    document["segments"]
                        .as_array()
                        .map(|segments| {
                            segments
                                .iter()
                                .filter_map(|segment| segment["text"].as_str())
                                .collect::<Vec<_>>()
                                .join(" ")
                        })
                        .unwrap_or_default()
                })
        } else if output.format == OutputFormat::Txt {
            content.clone()
        } else {
            String::new()
        };
        Ok(json!({"path":path,"content":content,"text":text}))
    }
    pub(super) fn retry(&mut self, body: &[u8]) -> anyhow::Result<Value> {
        self.ensure_idle()?;
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        struct Retry {
            batch_id: String,
            file_index: usize,
        }
        let request: Retry = serde_json::from_slice(body)?;
        let queue = {
            let stored = self
                .jobs
                .inner
                .lock()
                .map_err(|_| anyhow::anyhow!("Jobs unavailable"))?;
            let batch = stored
                .document
                .jobs
                .iter()
                .find(|batch| batch.id == request.batch_id)
                .context("job does not exist")?;
            let row = batch
                .files
                .get(request.file_index)
                .context("file does not exist")?;
            if row.status == Status::Completed || row.status.unfinished() {
                bail!("only failed, cancelled or unstarted files can be retried");
            }
            if batch.speaker_directory != self.speaker_directory {
                bail!("load the original speaker index before retrying this job");
            }
            Queue {
                inputs: vec![row.input.clone()],
                settings: batch.settings.clone(),
            }
        };
        self.queue(&serde_json::to_vec(&queue)?)
    }
    pub(super) fn queue(&mut self, body: &[u8]) -> anyhow::Result<Value> {
        self.ensure_idle()?;
        let mut request: Queue = serde_json::from_slice(body).context("invalid queue request")?;
        if request.inputs.is_empty() || request.inputs.len() > 100 {
            bail!("select between 1 and 100 files");
        }
        let mut inputs = Vec::new();
        for input in request.inputs {
            let path = self.authorized_path(&input)?;
            if path.is_dir() {
                let mut files = Vec::new();
                for entry in std::fs::read_dir(&path)? {
                    let entry = entry?;
                    let candidate = self.authorized_path(&entry.path())?;
                    if candidate.is_file() && supported_media(&candidate) {
                        files.push(candidate);
                    }
                    if files.len() > 100 {
                        bail!("folder contains more than 100 media files; select a smaller batch");
                    }
                }
                files.sort();
                inputs.extend(files);
            } else if path.is_file() {
                inputs.push(path);
            } else {
                bail!("select regular audio/video files or a folder");
            }
        }
        let mut seen = std::collections::HashSet::new();
        inputs.retain(|path| seen.insert(path.clone()));
        if inputs.is_empty() || inputs.len() > 100 {
            bail!("select between 1 and 100 media files");
        }
        self.authorize_options(&mut request.settings.options)?;
        // All files share model settings. Probe those resources once, using a
        // non-WAV input when present so the batch also checks media decoding.
        let representative = inputs
            .iter()
            .find(|path| {
                !path
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("wav"))
            })
            .or_else(|| inputs.first())
            .context("select media")?;
        let mut selected = request.settings.clone();
        selected.input = representative.clone();
        let (config, _) = self.prepare_authorized_request(selected)?;
        Self::require_resources(&config)?;
        let mut entropy = [0_u8; 16];
        getrandom::fill(&mut entropy).context("job identifier unavailable")?;
        let id = entropy
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let mut configs = Vec::new();
        let mut rows = Vec::new();
        let mut expected = Vec::new();
        for (index, input) in inputs.into_iter().enumerate() {
            let mut settings = request.settings.clone();
            settings.input = input.clone();
            let mut config = prepare_workbench(settings)?.into_config();
            if !config.output.formats.contains(&OutputFormat::NativeJson) {
                config.output.formats.push(OutputFormat::NativeJson);
            }
            config.diarization.speaker_directory = SpeakerDirectorySelection {
                explicit_path: Some(self.speaker_directory.clone()),
                ..SpeakerDirectorySelection::default()
            };
            let parent = config
                .output
                .output_dir
                .as_ref()
                .context("missing output directory")?;
            let output = parent
                .join(".native-whisperx-output")
                .join(&id)
                .join(index.to_string());
            let basename = input
                .file_stem()
                .and_then(|value| value.to_str())
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("transcript");
            config.output.basename = Some(basename.to_owned());
            config.output.output_dir = Some(output.clone());
            let formats = config
                .output
                .formats
                .iter()
                .flat_map(|format| match format {
                    OutputFormat::All => vec![
                        OutputFormat::Json,
                        OutputFormat::Srt,
                        OutputFormat::Vtt,
                        OutputFormat::Txt,
                        OutputFormat::Tsv,
                        OutputFormat::Audacity,
                    ],
                    other => vec![*other],
                })
                .map(|format| OutputFile {
                    format,
                    path: output.join(format!("{basename}.{}", format.extension())),
                })
                .collect::<Vec<_>>();
            expected.push(formats);
            rows.push(Row {
                input,
                status: Status::Queued,
                tasks: BTreeMap::new(),
                current_task: None,
                seconds: None,
                error: None,
                outputs: vec![],
            });
            configs.push(config);
        }
        // Reserve a fresh partition for this run. Existing partitions must be
        // authorized before creating any descendant (including through symlinks).
        let mut groups = std::collections::HashSet::new();
        for config in &configs {
            let output = config
                .output
                .output_dir
                .as_ref()
                .context("missing output folder")?;
            let group = output.parent().context("missing run folder")?;
            let partition = group.parent().context("missing output partition")?;
            if partition.exists() {
                self.authorized_path(partition)?;
            }
        }
        for config in &configs {
            let output = config
                .output
                .output_dir
                .as_ref()
                .context("missing output folder")?;
            let group = output.parent().context("missing run folder")?;
            let partition = group.parent().context("missing output partition")?;
            if !partition.exists() {
                std::fs::create_dir(partition)?;
            }
            self.authorized_path(partition)?;
            if groups.insert(group.to_path_buf()) {
                std::fs::create_dir(group)?;
            }
            self.authorized_path(group)?;
            std::fs::create_dir(output)?;
            self.authorized_path(output)?;
        }
        {
            let mut stored = self
                .jobs
                .inner
                .lock()
                .map_err(|_| anyhow::anyhow!("Jobs unavailable"))?;
            if stored.document.jobs.len() >= 500 {
                bail!("Jobs history is full; archive its index before adding work");
            }
            stored.document.jobs.insert(
                0,
                Batch {
                    id: id.clone(),
                    status: Status::Running,
                    settings: request.settings,
                    speaker_directory: self.speaker_directory.clone(),
                    files: rows,
                    error: None,
                },
            );
            if let Err(error) = self.jobs.save(&mut stored) {
                stored.document.jobs.remove(0);
                return Err(error);
            }
        }
        if let Some(worker) = self.worker.take() {
            worker
                .join()
                .map_err(|_| anyhow::anyhow!("previous worker failed"))?;
        }
        self.cancellation = CancellationHandle::new();
        let cancellation = self.cancellation.clone();
        let history = self.jobs.clone();
        let state = Arc::clone(&self.state);
        *state
            .lock()
            .map_err(|_| anyhow::anyhow!("job state unavailable"))? =
            json!({"status":"running","batchId":id,"tasks":{},"outputs":[]});
        let worker = BatchWorker {
            configs,
            expected,
            cancellation,
            history: history.clone(),
            id: id.clone(),
            state: Arc::clone(&state),
        };
        match std::thread::Builder::new()
            .name("native-workbench-batch".into())
            .spawn(move || worker.run())
        {
            Ok(worker) => self.worker = Some(worker),
            Err(error) => {
                let message = error.to_string();
                history.finish(&id, Status::Failed, Some(message.clone()))?;
                *state
                    .lock()
                    .map_err(|_| anyhow::anyhow!("job state unavailable"))? =
                    json!({"status":"failed","error":message});
                return Err(error.into());
            }
        }
        self.jobs.value()
    }
}
struct BatchWorker {
    configs: Vec<NativeWhisperxConfig>,
    expected: Vec<Vec<OutputFile>>,
    cancellation: CancellationHandle,
    history: History,
    id: String,
    state: Arc<Mutex<Value>>,
}
impl BatchWorker {
    fn run(self) {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut observer = JobProgress {
                history: self.history.clone(),
                id: self.id.clone(),
                expected: self.expected,
                cancellation: self.cancellation.clone(),
                error: None,
            };
            let result = native_whisperx::run_many_with_control(
                self.configs,
                &mut observer,
                &self.cancellation,
            );
            if let Some(error) = observer.error {
                return Err(native_whisperx::NativeWhisperxError::Transcription(error));
            }
            result
        }));
        let (status, error) = match result {
            Ok(Ok(MultiInputTranscriptionOutcome::Completed(_))) => (Status::Completed, None),
            Ok(Ok(MultiInputTranscriptionOutcome::Cancelled { .. })) => (Status::Cancelled, None),
            Ok(Err(error)) => (Status::Failed, Some(error.to_string())),
            Err(_) => (
                Status::Failed,
                Some("native transcription worker panicked".into()),
            ),
        };
        let saved = self.history.finish(&self.id, status, error.clone());
        if let Ok(mut current) = self.state.lock() {
            *current =
                json!({"status":status,"batchId":self.id,"tasks":{},"outputs":[],"error":error});
            if let Err(error) = saved {
                current["status"] = json!("failed");
                current["error"] = json!(format!("Jobs history could not be saved: {error}"));
            }
        }
    }
}
impl History {
    fn finish(&self, id: &str, status: Status, error: Option<String>) -> anyhow::Result<()> {
        self.change(id, |batch| {
            batch.status = status;
            batch.error = error.clone();
            let mut assigned_failure = batch.files.iter().any(|row| row.status == Status::Failed);
            for row in &mut batch.files {
                if row.status.unfinished() {
                    row.status = if status == Status::Failed && !assigned_failure {
                        assigned_failure = true;
                        row.error = error.clone();
                        Status::Failed
                    } else if status == Status::Cancelled {
                        Status::Cancelled
                    } else {
                        Status::NotRun
                    };
                }
                for task in row.tasks.values_mut() {
                    if task["status"] == "running" {
                        task["status"] =
                            serde_json::to_value(row.status).unwrap_or(json!("failed"));
                    }
                }
                row.current_task = None;
            }
        })
    }
}
pub(super) fn supported_media(path: &Path) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "wav"
                    | "mp3"
                    | "mp4"
                    | "m4a"
                    | "flac"
                    | "ogg"
                    | "opus"
                    | "webm"
                    | "mkv"
                    | "aac"
                    | "aiff"
                    | "mov"
            )
        })
}
struct JobProgress {
    history: History,
    id: String,
    expected: Vec<Vec<OutputFile>>,
    cancellation: CancellationHandle,
    error: Option<String>,
}
impl TranscriptionProgressObserver for JobProgress {
    fn observe(&mut self, event: TranscriptionProgressEvent) {
        let index = match &event {
            TranscriptionProgressEvent::FileStart { file_index, .. }
            | TranscriptionProgressEvent::FileEnd { file_index, .. }
            | TranscriptionProgressEvent::TaskStart { file_index, .. }
            | TranscriptionProgressEvent::TaskEnd { file_index, .. }
            | TranscriptionProgressEvent::Failure { file_index, .. }
            | TranscriptionProgressEvent::Cancelled { file_index, .. } => *file_index,
            _ => return,
        };
        let result = self.history.change(&self.id, |batch| {
            let Some(row) = batch.files.get_mut(index) else {
                return;
            };
            match event {
                TranscriptionProgressEvent::FileStart { .. } => row.status = Status::Running,
                TranscriptionProgressEvent::FileEnd {
                    duration_seconds, ..
                } => {
                    row.status = Status::Completed;
                    row.seconds = Some(duration_seconds);
                    row.current_task = None;
                    row.outputs = self.expected[index].clone();
                }
                TranscriptionProgressEvent::TaskStart { task, .. } => {
                    let name = task_name(task);
                    row.current_task = Some(name.into());
                    row.tasks.insert(name.into(), json!({"status":"running"}));
                }
                TranscriptionProgressEvent::TaskEnd {
                    task,
                    duration_seconds,
                    ..
                } => {
                    row.current_task = None;
                    row.tasks.insert(
                        task_name(task).into(),
                        json!({"status":"completed","seconds":duration_seconds}),
                    );
                }
                TranscriptionProgressEvent::Failure { message, task, .. } => {
                    row.status = Status::Failed;
                    row.error = Some(message.clone());
                    if let Some(task) = task {
                        row.tasks.insert(
                            task_name(task).into(),
                            json!({"status":"failed","error":message}),
                        );
                    }
                }
                TranscriptionProgressEvent::Cancelled { task, .. } => {
                    row.status = Status::Cancelled;
                    if let Some(task) = task {
                        row.tasks
                            .insert(task_name(task).into(), json!({"status":"cancelled"}));
                    }
                }
                _ => {}
            }
        });
        if let Err(error) = result {
            self.error = Some(format!("Jobs history could not be saved: {error}"));
            self.cancellation.cancel();
        }
    }
}

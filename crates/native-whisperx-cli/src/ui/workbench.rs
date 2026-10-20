//! One session-owned finite transcription job for the local workbench.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use anyhow::{bail, Context};
use native_whisperx::{
    prepare_workbench, CancellationHandle, FiniteTranscriptionOutcome, NativeWhisperxConfig,
    SpeakerDirectorySelection, TranscriptionProgressEvent, TranscriptionProgressObserver,
    TranscriptionProgressTask, WorkbenchRequest,
};
use serde_json::{json, Value};

mod jobs;

pub(crate) struct WorkbenchSession {
    jobs: jobs::History,
    root: PathBuf,
    speaker_directory: PathBuf,
    state: Arc<Mutex<Value>>,
    cancellation: CancellationHandle,
    worker: Option<JoinHandle<()>>,
}

impl WorkbenchSession {
    pub(crate) fn new(root: PathBuf, speaker_directory: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            jobs: jobs::History::open(&root)?,
            root: root.canonicalize()?,
            speaker_directory,
            state: Arc::new(Mutex::new(json!({"status": "idle"}))),
            cancellation: CancellationHandle::new(),
            worker: None,
        })
    }

    pub(crate) fn operation(&mut self, path: &str, body: &[u8]) -> anyhow::Result<Value> {
        match path {
            "/api/workbench/jobs" => self.jobs.value(),
            "/api/workbench/queue" => self.queue(body),
            "/api/workbench/retry" => self.retry(body),
            "/api/workbench/job-result" => self.job_result(body),
            "/api/workbench/state" => Ok(self
                .state
                .lock()
                .map_err(|_| anyhow::anyhow!("job state unavailable"))?
                .clone()),
            "/api/workbench/capabilities" => Ok(json!({
                "native": cfg!(feature = "native"), "cuda": cfg!(feature = "cuda"),
                "align": cfg!(feature = "native"), "identifySpeakers": cfg!(feature = "pyannote-diarization") && cfg!(feature = "pyannote-vad"),
                "translate": cfg!(feature = "translation"), "root": self.root,
                "speakerDirectory": self.speaker_directory,
            })),
            "/api/workbench/files" => {
                #[derive(serde::Deserialize)]
                #[serde(deny_unknown_fields)]
                struct Browse {
                    directory: PathBuf,
                }
                let request: Browse = serde_json::from_slice(body)?;
                let directory = self.authorized_path(&request.directory)?;
                if !directory.is_dir() {
                    bail!("select a directory");
                }
                let mut entries = Vec::new();
                for entry in std::fs::read_dir(&directory)?.take(1001) {
                    let entry = entry?;
                    let path = entry.path();
                    let Ok(path) = self.authorized_path(&path) else {
                        continue;
                    };
                    if path.is_file() || path.is_dir() {
                        entries.push(json!({"path": path.strip_prefix(&self.root)?, "name": entry.file_name().to_string_lossy(), "directory": path.is_dir()}));
                    }
                }
                entries.sort_by(|a, b| {
                    b["directory"]
                        .as_bool()
                        .cmp(&a["directory"].as_bool())
                        .then_with(|| a["name"].as_str().cmp(&b["name"].as_str()))
                });
                Ok(
                    json!({"root": self.root, "directory": directory.strip_prefix(&self.root)?, "entries": entries, "truncated": entries.len() > 1000}),
                )
            }
            "/api/workbench/result" => {
                let state = self
                    .state
                    .lock()
                    .map_err(|_| anyhow::anyhow!("job state unavailable"))?;
                if state["status"] != "completed" {
                    bail!("no completed result");
                }
                #[derive(serde::Deserialize)]
                #[serde(deny_unknown_fields)]
                struct OpenResult {
                    #[serde(default)]
                    index: usize,
                }
                let request: OpenResult = serde_json::from_slice(body)?;
                let path = state["outputs"]
                    .get(request.index)
                    .and_then(|output| output["path"].as_str())
                    .context("output does not exist")?;
                let path = self.authorized_path(Path::new(path))?;
                if std::fs::metadata(&path)?.len() > 32 * 1024 * 1024 {
                    bail!("result is too large to open in the workbench");
                }
                let content = std::fs::read_to_string(&path)?;
                Ok(json!({"text": state["text"], "path": path, "content": content}))
            }
            "/api/workbench/run" => {
                let mut request: WorkbenchRequest =
                    serde_json::from_slice(body).context("invalid workbench request")?;
                request.input = self.authorized_path(&request.input)?;
                if !request.input.is_file() {
                    bail!("select a regular audio or video file");
                }
                for path in [
                    &mut request.output_dir,
                    &mut request.model_dir,
                    &mut request.vad_bundle,
                    &mut request.diarization_bundle,
                ]
                .into_iter()
                .flatten()
                {
                    *path = self.authorized_path(path)?;
                    if !path.is_dir() {
                        bail!("selected folder must be an existing directory");
                    }
                }
                let prepared = prepare_workbench(request)?;
                let stages = serde_json::to_value(prepared.stages())?;
                let mut config = prepared.into_config();
                config.diarization.speaker_directory = SpeakerDirectorySelection {
                    explicit_path: Some(self.speaker_directory.clone()),
                    ..SpeakerDirectorySelection::default()
                };
                self.start(config, stages)?;
                self.operation("/api/workbench/state", &[])
            }
            "/api/workbench/cancel" => {
                self.cancellation.cancel();
                let mut state = self
                    .state
                    .lock()
                    .map_err(|_| anyhow::anyhow!("job state unavailable"))?;
                if state["status"] == "running" {
                    state["status"] = json!("cancelling");
                }
                Ok(state.clone())
            }
            _ => bail!("unknown workbench operation"),
        }
    }

    pub(crate) fn authorized_path(&self, path: &Path) -> anyhow::Result<PathBuf> {
        if path.as_os_str().is_empty() {
            bail!("select an audio or video file");
        }
        let candidate = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        };
        let canonical = candidate
            .canonicalize()
            .context("selected path does not exist")?;
        if !canonical.starts_with(&self.root) {
            bail!("selected path is outside the workbench directory");
        }
        Ok(canonical)
    }

    pub(crate) fn authorized_recording(&self, path: &Path) -> anyhow::Result<PathBuf> {
        let path = self.authorized_path(path)?;
        if !jobs::supported_media(&path) || !path.is_file() || std::fs::metadata(&path)?.len() == 0
        {
            bail!("choose a supported nonempty audio/video recording");
        }
        Ok(path)
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) fn select_speaker_directory(&mut self, path: PathBuf) -> anyhow::Result<()> {
        self.ensure_idle()?;
        self.speaker_directory = path;
        Ok(())
    }

    pub(crate) fn authorized_new_path(&self, path: &Path) -> anyhow::Result<PathBuf> {
        if path.as_os_str().is_empty() {
            bail!("choose a local destination");
        }
        let candidate = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        };
        match std::fs::symlink_metadata(&candidate) {
            Ok(_) => self.authorized_path(&candidate),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let parent = self.authorized_path(
                    candidate
                        .parent()
                        .context("destination requires a parent directory")?,
                )?;
                if !parent.is_dir() {
                    bail!("destination parent is not a directory");
                }
                Ok(parent.join(
                    candidate
                        .file_name()
                        .context("destination requires a filename")?,
                ))
            }
            Err(error) => Err(error.into()),
        }
    }

    pub(crate) fn ensure_idle(&self) -> anyhow::Result<()> {
        if self
            .worker
            .as_ref()
            .is_some_and(|worker| !worker.is_finished())
        {
            bail!("finish or cancel the current transcription before changing the Speaker Library");
        }
        Ok(())
    }

    fn start(&mut self, mut config: NativeWhisperxConfig, stages: Value) -> anyhow::Result<()> {
        if self
            .worker
            .as_ref()
            .is_some_and(|worker| !worker.is_finished())
        {
            bail!("a transcription is already running");
        }
        if let Some(worker) = self.worker.take() {
            worker
                .join()
                .map_err(|_| anyhow::anyhow!("previous transcription worker failed"))?;
        }
        if !config
            .output
            .formats
            .contains(&native_whisperx::OutputFormat::NativeJson)
        {
            config
                .output
                .formats
                .push(native_whisperx::OutputFormat::NativeJson);
        }
        let review_id = format!("session:{}", super::server::generate_session_token()?);
        let translated = config.translation.enabled;
        self.cancellation = CancellationHandle::new();
        let cancellation = self.cancellation.clone();
        let state = Arc::clone(&self.state);
        let selected_tasks = stages
            .as_array()
            .context("invalid selected tasks")?
            .iter()
            .filter_map(Value::as_str)
            .map(|name| (name.to_owned(), json!({"status": "pending"})))
            .collect::<serde_json::Map<_, _>>();
        *state
            .lock()
            .map_err(|_| anyhow::anyhow!("job state unavailable"))? = json!({"status": "running", "stages": stages, "tasks": selected_tasks, "currentTask": null, "outputs": [], "input": config.input, "reviewId":review_id,"translated":translated});
        let worker_state = Arc::clone(&state);
        let worker = std::thread::Builder::new()
            .name("native-workbench".into())
            .spawn(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut observer = WorkbenchProgress(Arc::clone(&worker_state));
                    native_whisperx::run_with_control(config, &mut observer, &cancellation)
                }));
                if let Ok(mut state) = worker_state.lock() {
                    match result {
                        Ok(Ok(FiniteTranscriptionOutcome::Completed(report))) => {
                            state["status"] = json!("completed");
                            state["outputs"] = json!(report.output_files);
                            state["text"] = json!(report.transcript.text);
                        }
                        Ok(Ok(FiniteTranscriptionOutcome::Cancelled(_))) => {
                            state["status"] = json!("cancelled")
                        }
                        Ok(Err(error)) => {
                            state["status"] = json!("failed");
                            state["error"] = json!(error.to_string());
                        }
                        Err(_) => {
                            state["status"] = json!("failed");
                            state["error"] = json!("native transcription worker panicked");
                        }
                    }
                    if state["status"] != "completed" {
                        if let Some(tasks) = state["tasks"].as_object_mut() {
                            for task in tasks.values_mut() {
                                if task["status"] == "pending" {
                                    task["status"] = json!("notRun");
                                }
                            }
                        }
                    }
                    state["currentTask"] = Value::Null;
                }
            });
        match worker {
            Ok(worker) => {
                self.worker = Some(worker);
                Ok(())
            }
            Err(error) => {
                *state
                    .lock()
                    .map_err(|_| anyhow::anyhow!("job state unavailable"))? =
                    json!({"status": "failed", "error": error.to_string()});
                Err(error.into())
            }
        }
    }
}

impl Drop for WorkbenchSession {
    fn drop(&mut self) {
        self.cancellation.cancel();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

struct WorkbenchProgress(Arc<Mutex<Value>>);
impl TranscriptionProgressObserver for WorkbenchProgress {
    fn observe(&mut self, event: TranscriptionProgressEvent) {
        let Ok(mut state) = self.0.lock() else {
            return;
        };
        match event {
            TranscriptionProgressEvent::TaskStart { task, .. } => {
                let name = task_name(task);
                state["currentTask"] = json!(name);
                state["activity"] = Value::Null;
                let starts = state["tasks"][name]["starts"].as_u64().unwrap_or_default() + 1;
                state["tasks"][name] = json!({"status": "running", "starts": starts});
            }
            TranscriptionProgressEvent::TaskEnd {
                task,
                duration_seconds,
                ..
            } => {
                let starts = state["tasks"][task_name(task)]["starts"]
                    .as_u64()
                    .unwrap_or_default();
                state["tasks"][task_name(task)] =
                    json!({"status": "completed", "seconds": duration_seconds, "starts": starts});
            }
            TranscriptionProgressEvent::ModelResolutionStart { model_id, .. } => {
                state["activity"] = json!(format!("Resolving {model_id}"))
            }
            TranscriptionProgressEvent::ModelLoadStart { model_id, .. } => {
                state["activity"] = json!(format!("Loading {model_id}"))
            }
            TranscriptionProgressEvent::ModelLoadEnd { .. }
            | TranscriptionProgressEvent::ModelResolutionEnd { .. } => {
                state["activity"] = Value::Null
            }
            TranscriptionProgressEvent::Failure {
                task: Some(task),
                message,
                ..
            } => {
                state["tasks"][task_name(task)] = json!({"status": "failed", "error": message});
            }
            TranscriptionProgressEvent::Cancelled {
                task: Some(task), ..
            } => state["tasks"][task_name(task)] = json!({"status": "cancelled"}),
            _ => {}
        }
    }
}
fn task_name(task: TranscriptionProgressTask) -> &'static str {
    match task {
        TranscriptionProgressTask::Decode => "decode",
        TranscriptionProgressTask::Vad => "vad",
        TranscriptionProgressTask::Asr => "transcribe",
        TranscriptionProgressTask::Alignment => "align",
        TranscriptionProgressTask::Diarization => "identifySpeakers",
        TranscriptionProgressTask::Translation => "translate",
        TranscriptionProgressTask::Output => "writeOutputs",
    }
}

//! Session-authorized review of recorded native results. No inference is run.
use super::workbench::WorkbenchSession;
use anyhow::{bail, Context};
use native_whisperx::{
    export_reviewed_transcription_layer, import_review_transcription, review_transcription,
    review_translation_source, translation_source_transcription, OutputConfig, OutputFormat,
    ReviewExportLayer, ReviewedTranscription, SegmentCorrection, SubtitleConfig,
    TranscriptionContract,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::VecDeque,
    io::{Read, Seek, SeekFrom, Write},
    net::TcpStream,
    path::{Path, PathBuf},
};

pub(crate) struct RecordedReview {
    pub input: PathBuf,
    pub canonical: PathBuf,
    pub translated: bool,
}
#[derive(Default)]
pub(crate) struct ReviewSession {
    entries: VecDeque<Entry>,
}
struct Entry {
    result_id: String,
    snapshot_id: String,
    media_id: String,
    recording: PathBuf,
    canonical: PathBuf,
    original: TranscriptionContract,
    translated: bool,
    preview: Option<Preview>,
}
struct Preview {
    id: String,
    reviewed: ReviewedTranscription,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Selection {
    result_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Correction {
    segment_id: String,
    text: String,
    start_seconds: f64,
    end_seconds: f64,
    speaker: Option<String>,
}
impl Correction {
    fn into_product(self) -> anyhow::Result<SegmentCorrection> {
        Ok(SegmentCorrection {
            segment_id: self.segment_id.parse().context("invalid segment ID")?,
            text: self.text,
            start_seconds: self.start_seconds,
            end_seconds: self.end_seconds,
            speaker: self.speaker,
        })
    }
}

impl ReviewSession {
    pub(crate) fn operation(
        &mut self,
        path: &str,
        body: &[u8],
        workbench: &mut WorkbenchSession,
    ) -> anyhow::Result<Value> {
        match path {
            "/api/review/open" => {
                let selection: Selection = serde_json::from_slice(body)?;
                if let Some(entry) = self
                    .entries
                    .iter()
                    .find(|entry| entry.result_id == selection.result_id)
                {
                    return entry.value(workbench);
                }
                let recorded = workbench.recorded_review(&selection.result_id)?;
                let canonical = workbench.authorized_path(&recorded.canonical).context("result file is missing or outside this launch directory; open Jobs or run Transcribe again")?;
                if !canonical.is_file() || std::fs::metadata(&canonical)?.len() > 32 * 1024 * 1024 {
                    bail!("choose a recorded JSON result within the 32 MiB review limit");
                }
                let original = import_review_transcription(&std::fs::read(&canonical)?)?;
                // Parse retained source now so malformed bilingual metadata never
                // masquerades as a valid editable source layer.
                let source = translation_source_transcription(&original)?;
                let entry = Entry {
                    result_id: selection.result_id,
                    snapshot_id: super::server::generate_session_token()?,
                    media_id: super::server::generate_session_token()?,
                    recording: recorded.input,
                    canonical,
                    original,
                    translated: recorded.translated || source.is_some(),
                    preview: None,
                };
                let value = entry.value(workbench)?;
                if self.entries.len() >= 4 {
                    self.entries.pop_front();
                }
                self.entries.push_back(entry);
                Ok(value)
            }
            "/api/review/preview" => {
                #[derive(Deserialize)]
                #[serde(rename_all = "camelCase", deny_unknown_fields)]
                struct Request {
                    result_id: String,
                    corrections: Vec<Correction>,
                    source_corrections: Vec<Correction>,
                }
                let request: Request = serde_json::from_slice(body)?;
                if request.corrections.len() + request.source_corrections.len() > 10_000 {
                    bail!("too many segment corrections in one copy");
                }
                let entry = self.entry_mut(&request.result_id)?;
                entry.preview = None;
                let edits = request
                    .corrections
                    .into_iter()
                    .map(Correction::into_product)
                    .collect::<anyhow::Result<Vec<_>>>()?;
                let source_edits = request
                    .source_corrections
                    .into_iter()
                    .map(Correction::into_product)
                    .collect::<anyhow::Result<Vec<_>>>()?;
                let mut reviewed = review_transcription(&entry.original, &edits)?;
                let cleared_target = reviewed.cleared_alignment().to_vec();
                let mut cleared_source = Vec::new();
                if !source_edits.is_empty() {
                    let source_review =
                        review_translation_source(reviewed.transcript(), &source_edits)?;
                    cleared_source = source_review.cleared_alignment().to_vec();
                    reviewed = source_review;
                }
                let preview_id = super::server::generate_session_token()?;
                let value = json!({"previewId":preview_id,"transcript":transcript_value(reviewed.transcript()),
                    "sourceTranscript":translation_source_transcription(reviewed.transcript())?.as_ref().map(transcript_value),
                    "clearedAlignment":cleared_target.iter().map(u64::to_string).collect::<Vec<_>>(),
                    "clearedSourceAlignment":cleared_source.iter().map(u64::to_string).collect::<Vec<_>>()});
                entry.preview = Some(Preview {
                    id: preview_id,
                    reviewed,
                });
                Ok(value)
            }
            "/api/review/reset" => {
                let selection: Selection = serde_json::from_slice(body)?;
                let entry = self.entry_mut(&selection.result_id)?;
                entry.preview = None;
                entry.value(workbench)
            }
            "/api/review/recording" => {
                #[derive(Deserialize)]
                #[serde(rename_all = "camelCase", deny_unknown_fields)]
                struct Request {
                    result_id: String,
                    path: PathBuf,
                }
                let request: Request = serde_json::from_slice(body)?;
                let recording = workbench.authorized_recording(&request.path)?;
                let entry = self.entry_mut(&request.result_id)?;
                entry.recording = recording;
                entry.media_id = super::server::generate_session_token()?;
                entry.value(workbench)
            }
            "/api/review/save" => {
                #[derive(Deserialize)]
                #[serde(rename_all = "camelCase", deny_unknown_fields)]
                struct Request {
                    result_id: String,
                    preview_id: String,
                    directory: PathBuf,
                    basename: String,
                    formats: Vec<OutputFormat>,
                    #[serde(default)]
                    subtitles: SubtitleConfig,
                    #[serde(default)]
                    export_layer: ReviewExportLayer,
                }
                let mut request: Request = serde_json::from_slice(body)?;
                let directory = workbench.authorized_path(&request.directory)?;
                if !directory.is_dir() {
                    bail!("choose an existing output folder");
                }
                let entry = self.entry_mut(&request.result_id)?;
                let preview = entry
                    .preview
                    .as_ref()
                    .filter(|preview| preview.id == request.preview_id)
                    .context("preview the current corrections before saving")?;
                if !request.formats.contains(&OutputFormat::NativeJson) {
                    request.formats.push(OutputFormat::NativeJson);
                }
                let outputs = export_reviewed_transcription_layer(
                    &preview.reviewed,
                    &OutputConfig {
                        output_dir: Some(directory),
                        basename: Some(request.basename),
                        formats: request.formats,
                        subtitles: request.subtitles,
                        ..OutputConfig::default()
                    },
                    request.export_layer,
                )?;
                let result_id = match workbench.record_corrected_review(
                    &entry.result_id,
                    entry.recording.clone(),
                    outputs.clone(),
                    entry.translated,
                ) {
                    Ok(id) => id,
                    Err(error) => {
                        for output in &outputs {
                            let _ = std::fs::remove_file(&output.path);
                        }
                        return Err(error.context(
                            "corrected copy could not be registered; new exports were removed",
                        ));
                    }
                };
                entry.preview = None;
                Ok(json!({"resultId":result_id,"outputs":outputs}))
            }
            _ => bail!("unknown review operation"),
        }
    }
    fn entry_mut(&mut self, result_id: &str) -> anyhow::Result<&mut Entry> {
        self.entries
            .iter_mut()
            .find(|entry| entry.result_id == result_id)
            .context("reopen this result before reviewing or saving it")
    }
    pub(crate) fn serve_media(
        &self,
        stream: &mut TcpStream,
        media_id: &str,
        range: Option<&str>,
        head: bool,
        workbench: &WorkbenchSession,
    ) -> anyhow::Result<()> {
        let Some(entry) = self.entries.iter().find(|entry| entry.media_id == media_id) else {
            return super::handlers::write_http_response(
                stream,
                404,
                "Not Found",
                "text/plain; charset=utf-8",
                "recording capability is unavailable; reopen the result",
            );
        };
        let path = match workbench.authorized_path(&entry.recording) {
            Ok(path) => path,
            Err(_) => {
                return super::handlers::write_http_response(
                    stream,
                    404,
                    "Not Found",
                    "text/plain; charset=utf-8",
                    "recording is missing; locate its local file in Review",
                )
            }
        };
        let mut file = std::fs::File::open(&path)?;
        let size = file.metadata()?.len();
        let parsed = byte_range(range, size);
        let Some((start, end, partial)) = parsed else {
            write!(stream,"HTTP/1.1 416 Range Not Satisfiable\r\nContent-Range: bytes */{size}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")?;
            return Ok(());
        };
        let length = end - start + 1;
        let (status, reason) = if partial {
            (206, "Partial Content")
        } else {
            (200, "OK")
        };
        write!(stream,"HTTP/1.1 {status} {reason}\r\nContent-Type: {}\r\nContent-Length: {length}\r\nAccept-Ranges: bytes\r\nCache-Control: no-store\r\nCross-Origin-Resource-Policy: same-origin\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n", media_type(&path))?;
        if partial {
            write!(stream, "Content-Range: bytes {start}-{end}/{size}\r\n")?;
        }
        write!(stream, "\r\n")?;
        if !head {
            file.seek(SeekFrom::Start(start))?;
            std::io::copy(&mut file.take(length), stream)?;
        }
        Ok(())
    }
}
impl Entry {
    fn value(&self, workbench: &WorkbenchSession) -> anyhow::Result<Value> {
        let media_error = match workbench.authorized_path(&self.recording) {
            Ok(path) if path.is_file() && std::fs::metadata(&path)?.len() > 0 => None,
            _ => Some("Source recording is missing, empty or outside this launch directory. Locate a compatible local recording below."),
        };
        let provenance = self
            .original
            .attributes
            .get("nativeWhisperx.translation.provenance")
            .map(|value| serde_json::from_str::<Value>(value))
            .transpose()?;
        Ok(
            json!({"resultId":self.result_id,"snapshotId":self.snapshot_id,
            "originalName":self.canonical.file_name().map(|name|name.to_string_lossy()),
            "sourceName":self.recording.file_name().map(|name|name.to_string_lossy()),
            "mediaUrl":media_error.is_none().then(||format!("/api/review/media/{}",self.media_id)),
            "mediaType":media_type(&self.recording),"mediaError":media_error,
            "transcript":transcript_value(&self.original),
            "sourceTranscript":translation_source_transcription(&self.original)?.as_ref().map(transcript_value),
            "isTranslation":self.translated,"translationProvenance":provenance,
            "root":workbench.root()}),
        )
    }
}
pub(super) fn transcript_value(transcript: &TranscriptionContract) -> Value {
    let mut previous_end = None;
    let segments = transcript.segments.iter().map(|segment| {
        let overlap = segment.start_seconds().zip(previous_end).is_some_and(|(start,end)|start < end);
        previous_end = segment.end_seconds();
        json!({"id":segment.index.to_string(),"text":segment.text,"startSeconds":segment.start_seconds(),
            "endSeconds":segment.end_seconds(),"speaker":segment.speaker,"wordCount":segment.words().len(),"overlapsPrevious":overlap})
    }).collect::<Vec<_>>();
    json!({"language":transcript.language,"segments":segments})
}
fn media_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("wav") => "audio/wav",
        Some("mp3") => "audio/mpeg",
        Some("flac") => "audio/flac",
        Some("m4a" | "aac") => "audio/mp4",
        Some("ogg" | "opus") => "audio/ogg",
        Some("mp4" | "mov" | "m4v") => "video/mp4",
        Some("webm") => "video/webm",
        _ => "application/octet-stream",
    }
}
fn byte_range(range: Option<&str>, size: u64) -> Option<(u64, u64, bool)> {
    if size == 0 {
        return None;
    }
    let Some(range) = range else {
        return Some((0, size - 1, false));
    };
    let (start, end) = range.strip_prefix("bytes=")?.split_once('-')?;
    if start.is_empty() {
        let suffix = end.parse::<u64>().ok()?.min(size);
        return (suffix > 0).then(|| (size - suffix, size - 1, true));
    }
    let start = start.parse::<u64>().ok()?;
    let end = if end.is_empty() {
        size - 1
    } else {
        end.parse::<u64>().ok()?.min(size - 1)
    };
    (start <= end && start < size).then_some((start, end, true))
}

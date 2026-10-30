//! Manual corrections to a Native WhisperX result and non-overwriting copies.
//! Canonical builders validate timing; existing product projections render files.

use crate::{NativeWhisperxError, OutputConfig, OutputFile, TranscriptionContract};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs::OpenOptions, io::Write, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SegmentCorrection {
    pub segment_id: u64,
    pub text: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub speaker: Option<String>,
}

/// A validated copy. The input transcript is never mutated or reordered.
#[derive(Debug, Clone)]
pub struct ReviewedTranscription {
    transcript: TranscriptionContract,
    cleared_alignment: Vec<u64>,
}
impl ReviewedTranscription {
    pub fn transcript(&self) -> &TranscriptionContract {
        &self.transcript
    }
    pub fn cleared_alignment(&self) -> &[u64] {
        &self.cleared_alignment
    }
}
fn invalid(message: impl Into<String>) -> NativeWhisperxError {
    NativeWhisperxError::InvalidConfig(message.into())
}
fn contract_error(error: media_core::DetectError) -> NativeWhisperxError {
    invalid(error.to_string())
}

/// Imports either canonical Native JSON or the existing WhisperX JSON contract.
pub fn import_review_transcription(
    bytes: &[u8],
) -> Result<TranscriptionContract, NativeWhisperxError> {
    let value: serde_json::Value = serde_json::from_slice(bytes)?;
    let native = value["segments"]
        .as_array()
        .and_then(|segments| segments.first())
        .map_or_else(
            || value.get("attributes").is_some(),
            |segment| segment.get("index").is_some(),
        );
    let transcript = if native {
        serde_json::from_value(value)?
    } else {
        crate::import_whisperx_json(bytes)?
    };
    transcript.validate_strict().map_err(contract_error)?;
    Ok(transcript)
}

/// Applies explicit edits by stable segment ID, retaining neighboring records.
/// Edited text/timing discards stale word/character alignment for that segment;
/// an annotation-only speaker change preserves alignment and updates its labels.
pub fn review_transcription(
    original: &TranscriptionContract,
    corrections: &[SegmentCorrection],
) -> Result<ReviewedTranscription, NativeWhisperxError> {
    original.validate_strict().map_err(contract_error)?;
    let mut original_ids = HashSet::new();
    for segment in &original.segments {
        if !original_ids.insert(segment.index) {
            return Err(invalid("result has duplicate segment IDs"));
        }
    }
    let mut corrected_ids = HashSet::new();
    let mut transcript = original.clone();
    let mut cleared_alignment = Vec::new();
    for correction in corrections {
        if !corrected_ids.insert(correction.segment_id) {
            return Err(invalid("a segment was corrected more than once"));
        }
        let segment = transcript
            .segments
            .iter_mut()
            .find(|segment| segment.index == correction.segment_id)
            .ok_or_else(|| invalid("the selected segment no longer exists"))?;
        if correction
            .speaker
            .as_ref()
            .is_some_and(|speaker| speaker.trim().is_empty())
        {
            return Err(invalid("choose a speaker label or Unassigned"));
        }
        let alignment_changed = segment.text != correction.text
            || segment.start_seconds() != Some(correction.start_seconds)
            || segment.end_seconds() != Some(correction.end_seconds);
        let mut replacement =
            media_core::TranscriptSegmentContract::new(segment.index, correction.text.clone())
                .with_time_range(Some(correction.start_seconds), Some(correction.end_seconds))
                .map_err(contract_error)?
                .with_confidence(segment.confidence())
                .map_err(contract_error)?;
        replacement.language = segment.language.clone();
        replacement.is_final = segment.is_final;
        replacement.attributes = segment.attributes.clone();
        replacement.speaker = correction.speaker.clone();
        if alignment_changed {
            if !segment.words().is_empty() || !segment.chars().is_empty() {
                cleared_alignment.push(segment.index);
                replacement.attributes.insert(
                    "nativeWhisperxReview.wordAlignment".into(),
                    "clearedForManualCorrection".into(),
                );
            }
        } else {
            for original_word in segment.words() {
                let mut word = original_word.clone();
                if segment.speaker != correction.speaker {
                    word.speaker = correction.speaker.clone();
                }
                replacement.push_word(word).map_err(contract_error)?;
            }
            for character in segment.chars() {
                replacement
                    .push_char(character.clone())
                    .map_err(contract_error)?;
            }
        }
        *segment = replacement;
    }
    transcript.text = Some(
        transcript
            .segments
            .iter()
            .map(|segment| segment.text.trim())
            .filter(|text| !text.is_empty())
            .collect::<Vec<_>>()
            .join(" "),
    );
    transcript.validate_strict().map_err(contract_error)?;
    Ok(ReviewedTranscription {
        transcript,
        cleared_alignment,
    })
}

pub(crate) const TRANSLATION_SOURCE_ATTRIBUTE: &str = "nativeWhisperx.translation.source";
#[cfg(any(feature = "translation", test))]
pub(crate) const TRANSLATION_PROVENANCE_ATTRIBUTE: &str = "nativeWhisperx.translation.provenance";

/// Returns the separately retained source of a translated product result.
pub fn translation_source_transcription(
    translated: &TranscriptionContract,
) -> Result<Option<TranscriptionContract>, NativeWhisperxError> {
    let Some(source) = translated.attributes.get(TRANSLATION_SOURCE_ATTRIBUTE) else {
        return Ok(None);
    };
    let source: TranscriptionContract = serde_json::from_str(source)?;
    source.validate_strict().map_err(contract_error)?;
    Ok(Some(source))
}

/// Corrects only the retained source layer. Target text/timing and model
/// provenance stay unchanged; no implicit retranslation is performed.
pub fn review_translation_source(
    translated: &TranscriptionContract,
    corrections: &[SegmentCorrection],
) -> Result<ReviewedTranscription, NativeWhisperxError> {
    translated.validate_strict().map_err(contract_error)?;
    let source = translation_source_transcription(translated)?
        .ok_or_else(|| invalid("this result has no separately retained source transcript"))?;
    let reviewed = review_transcription(&source, corrections)?;
    let mut transcript = translated.clone();
    transcript.attributes.insert(
        TRANSLATION_SOURCE_ATTRIBUTE.into(),
        serde_json::to_string(&reviewed.transcript)?,
    );
    Ok(ReviewedTranscription {
        transcript,
        cleared_alignment: reviewed.cleared_alignment,
    })
}

/// Exports through existing Native WhisperX output options and projections.
/// Every destination must be new; conflicts leave existing files untouched.
pub fn export_reviewed_transcription(
    reviewed: &ReviewedTranscription,
    output: &OutputConfig,
) -> Result<Vec<OutputFile>, NativeWhisperxError> {
    export_reviewed_transcription_layer(reviewed, output, ReviewExportLayer::Transcript)
}

/// Language projected into text/subtitle/WhisperX exports. Native JSON always
/// retains the complete corrected bilingual document and its provenance.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReviewExportLayer {
    #[default]
    Transcript,
    Source,
}

pub fn export_reviewed_transcription_layer(
    reviewed: &ReviewedTranscription,
    output: &OutputConfig,
    layer: ReviewExportLayer,
) -> Result<Vec<OutputFile>, NativeWhisperxError> {
    reviewed
        .transcript
        .validate_strict()
        .map_err(contract_error)?;
    let source = match layer {
        ReviewExportLayer::Transcript => None,
        ReviewExportLayer::Source => Some(
            translation_source_transcription(&reviewed.transcript)?.ok_or_else(|| {
                invalid("this result has no separately retained source transcript")
            })?,
        ),
    };
    let directory = output
        .output_dir
        .as_ref()
        .ok_or_else(|| invalid("choose an output folder"))?;
    if !directory.is_dir() {
        return Err(invalid("choose an existing output folder"));
    }
    let basename = output.basename.as_deref().unwrap_or("corrected");
    if basename.trim().is_empty()
        || basename == "."
        || basename == ".."
        || basename.contains(['/', '\\'])
    {
        return Err(invalid(
            "corrected-copy name must be a single filename without separators",
        ));
    }
    let mut formats = Vec::new();
    for format in output
        .formats
        .iter()
        .copied()
        .flat_map(crate::output::expand_output_format)
    {
        if !formats.contains(&format) {
            formats.push(format);
        }
    }
    if formats.is_empty() {
        return Err(invalid("select at least one output format"));
    }
    let rendered = formats
        .into_iter()
        .map(|format| {
            let file = OutputFile {
                format,
                path: directory.join(format!("{basename}.{}", format.extension())),
            };
            if std::fs::symlink_metadata(&file.path).is_ok() {
                return Err(invalid(format!(
                    "output already exists: {}",
                    file.path.display()
                )));
            }
            let projection = if format == crate::OutputFormat::NativeJson {
                &reviewed.transcript
            } else {
                source.as_ref().unwrap_or(&reviewed.transcript)
            };
            let contents =
                crate::output::render_transcript_output(projection, format, output, true)?;
            Ok((file, contents))
        })
        .collect::<Result<Vec<_>, NativeWhisperxError>>()?;
    let mut staging = CorrectedCopyStaging::default();
    for (file, contents) in &rendered {
        let sequence = EXPORT_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let temporary = directory.join(format!(
            ".native-whisperx-corrected-{}-{sequence}.pending",
            std::process::id()
        ));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut writer = options.open(&temporary)?;
        staging.temporary.push(temporary.clone());
        writer.write_all(contents.as_bytes())?;
        writer.sync_all()?;
        std::fs::hard_link(&temporary, &file.path)?;
        staging.published.push(file.path.clone());
    }
    staging.committed = true;
    Ok(rendered.into_iter().map(|(file, _)| file).collect())
}
static EXPORT_SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[derive(Default)]
struct CorrectedCopyStaging {
    temporary: Vec<PathBuf>,
    published: Vec<PathBuf>,
    committed: bool,
}
impl Drop for CorrectedCopyStaging {
    fn drop(&mut self) {
        for path in &self.temporary {
            let _ = std::fs::remove_file(path);
        }
        if !self.committed {
            for path in &self.published {
                let _ = std::fs::remove_file(path);
            }
        }
    }
}

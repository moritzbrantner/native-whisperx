//! Speaker correction and draft Speaker Library profile persistence.

use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};
#[cfg(feature = "diarization")]
use std::{collections::BTreeSet, path::Path};

use audio_analysis_speakers::{
    SpeakerAudio, SpeakerEmbedding, SpeakerEmbeddingExtractor, SpectralSpeakerEmbedder,
};
use audio_analysis_transcription::{LoadedAudio, TranscriptionPipelineResponse};
use text_transcripts::TranscriptionContract;

#[cfg(feature = "diarization")]
use crate::config::is_pyannote_diarization_model;
use crate::config::{
    NativeWhisperxConfig, NativeWhisperxError, OutputConfig, SpeakerCorrectionReport,
    SpeakerCorrectionRequest,
};
use crate::config_mapping::map_input_source;
use crate::output::write_outputs;

#[cfg(feature = "diarization")]
pub(crate) fn save_draft_speakers_from_response(
    response: &mut TranscriptionPipelineResponse,
    config: &NativeWhisperxConfig,
) -> Result<(), NativeWhisperxError> {
    if config.asr.provider != crate::config::AsrProvider::Native
        || !config.diarization.enabled
        || config.diarization.disable_speaker_library
        || is_pyannote_diarization_model(&config.diarization.model_id)
    {
        return Ok(());
    }

    if !config.diarization.save_draft_speakers {
        response
            .diagnostics
            .push("speakerLibraryDraftProfilesSaved=0".to_string());
        return Ok(());
    }

    let Some(diarization) = &response.diarization else {
        response
            .diagnostics
            .push("speakerLibraryDraftProfilesSaved=0".to_string());
        return Ok(());
    };
    let current_dir = std::env::current_dir()?;
    let resolved =
        crate::resolve_speaker_directory(&config.diarization.speaker_directory, &current_dir)?;
    let mut library = crate::speaker_directory::load_speaker_library_if_present(&resolved.path)?
        .unwrap_or_default();
    let existing_labels = library
        .profiles()
        .map(|profile| profile.id().as_str().to_string())
        .collect::<BTreeSet<_>>();
    let labels = diarization
        .segments
        .iter()
        .filter(|segment| is_transient_speaker_label(&segment.speaker))
        .map(|segment| segment.speaker.clone())
        .collect::<BTreeSet<_>>();
    if labels.is_empty() {
        response
            .diagnostics
            .push("speakerLibraryDraftProfilesSaved=0".to_string());
        return Ok(());
    }

    let audio = LoadedAudio::mono_16khz_from_source(&map_input_source(&config.input))
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))?;
    let mut saved = 0usize;
    for label in labels {
        if existing_labels.contains(&label) {
            continue;
        }
        let ranges = diarization
            .segments
            .iter()
            .filter(|segment| segment.speaker == label)
            .map(|segment| crate::SpeakerCorrectionRange {
                start_seconds: segment.start_seconds as f64,
                end_seconds: segment.end_seconds as f64,
            })
            .collect::<Vec<_>>();
        let embedding = speaker_embedding_for_ranges(&audio, &ranges)?;
        let draft_id = format!(
            "draft-{}-{}",
            slug_speaker_id(&label),
            current_unix_seconds()
        );
        let draft_label = format!("draft_{}", slug_speaker_id(&label));
        let mut metadata = BTreeMap::new();
        metadata.insert("status".to_string(), "draft".to_string());
        metadata.insert("detectedLabel".to_string(), label);
        let now = current_unix_seconds_string();
        metadata.insert("createdAt".to_string(), now.clone());
        metadata.insert("updatedAt".to_string(), now);
        let (updated, _) = crate::speaker_directory::upsert_speaker_profile_embedding(
            &library,
            &draft_id,
            &draft_label,
            embedding,
            metadata,
        )?;
        library = updated;
        saved += 1;
    }
    if saved > 0 {
        crate::speaker_directory::save_speaker_library(&resolved.path, &library)?;
    }
    response
        .diagnostics
        .push(format!("speakerLibraryDraftProfilesSaved={saved}"));
    Ok(())
}

#[cfg(not(feature = "diarization"))]
pub(crate) fn save_draft_speakers_from_response(
    _response: &mut TranscriptionPipelineResponse,
    _config: &NativeWhisperxConfig,
) -> Result<(), NativeWhisperxError> {
    Ok(())
}

#[cfg(feature = "diarization")]
fn is_transient_speaker_label(label: &str) -> bool {
    label
        .strip_prefix("speaker_")
        .is_some_and(|suffix| !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit()))
}

pub fn correct_speaker(
    request: SpeakerCorrectionRequest,
) -> Result<SpeakerCorrectionReport, NativeWhisperxError> {
    let current_dir = std::env::current_dir()?;
    let resolved = crate::resolve_speaker_directory(&request.speaker_directory, &current_dir)?;
    let ranges =
        speaker_correction_ranges(&request.transcript, &request.from_speaker, &request.ranges)?;
    let audio = LoadedAudio::mono_16khz_from_source(&map_input_source(&request.audio))
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))?;
    let embedding = speaker_embedding_for_ranges(&audio, &ranges)?;
    let library = crate::speaker_directory::load_speaker_library_if_present(&resolved.path)?
        .unwrap_or_default();
    let profile_id = request
        .speaker_id
        .clone()
        .unwrap_or_else(|| slug_speaker_id(&request.to_label));
    let mut metadata = BTreeMap::new();
    metadata.insert("status".to_string(), "confirmed".to_string());
    metadata.insert("correctedFrom".to_string(), request.from_speaker.clone());
    metadata.insert("updatedAt".to_string(), current_unix_seconds_string());
    let (library, updated_existing_profile) =
        crate::speaker_directory::upsert_speaker_profile_embedding(
            &library,
            &profile_id,
            &request.to_label,
            embedding,
            metadata,
        )?;
    crate::speaker_directory::save_speaker_library(&resolved.path, &library)?;

    let mut transcript = request.transcript;
    replace_speaker_labels(
        &mut transcript,
        &request.from_speaker,
        &request.to_label,
        &request.ranges,
    );
    let response = speaker_correction_response(transcript.clone());
    let output_files = write_outputs(&response, &request.output)?
        .into_iter()
        .map(|file| file.path)
        .collect();

    Ok(SpeakerCorrectionReport {
        transcript,
        speaker_directory_path: resolved.path,
        profile_id,
        label: request.to_label,
        corrected_from: request.from_speaker,
        enrolled_seconds: ranges
            .iter()
            .map(|range| range.end_seconds - range.start_seconds)
            .sum(),
        updated_existing_profile,
        output_files,
    })
}

fn speaker_correction_response(transcript: TranscriptionContract) -> TranscriptionPipelineResponse {
    TranscriptionPipelineResponse {
        accepted: true,
        operation: "audio.transcription.correctSpeakers".to_string(),
        provider: "native-whisperx".to_string(),
        model_id: "speaker-correction".to_string(),
        transcript,
        vad_segments: Vec::new(),
        alignment: None,
        diarization: None,
        artifacts: Vec::new(),
        diagnostics: Vec::new(),
    }
}

fn speaker_correction_ranges(
    transcript: &TranscriptionContract,
    from_speaker: &str,
    filters: &[crate::SpeakerCorrectionRange],
) -> Result<Vec<crate::SpeakerCorrectionRange>, NativeWhisperxError> {
    let mut ranges = Vec::new();
    for filter in filters {
        validate_speaker_correction_range(*filter)?;
    }
    for segment in &transcript.segments {
        if segment.speaker.as_deref() != Some(from_speaker) {
            continue;
        }
        let Some((start_seconds, end_seconds)) = segment.start_seconds.zip(segment.end_seconds)
        else {
            continue;
        };
        let segment_range = crate::SpeakerCorrectionRange {
            start_seconds,
            end_seconds,
        };
        validate_speaker_correction_range(segment_range)?;
        if filters.is_empty()
            || filters
                .iter()
                .any(|filter| filter.overlaps(start_seconds, end_seconds))
        {
            ranges.push(segment_range);
        }
    }
    if ranges.is_empty() {
        return Err(NativeWhisperxError::InvalidConfig(format!(
            "speaker correction found no timed transcript segments for speaker `{from_speaker}`"
        )));
    }
    Ok(ranges)
}

fn validate_speaker_correction_range(
    range: crate::SpeakerCorrectionRange,
) -> Result<(), NativeWhisperxError> {
    if !range.start_seconds.is_finite()
        || !range.end_seconds.is_finite()
        || range.start_seconds < 0.0
        || range.end_seconds <= range.start_seconds
    {
        return Err(NativeWhisperxError::InvalidConfig(
            "speaker correction ranges must be finite, non-negative, and have positive duration"
                .to_string(),
        ));
    }
    Ok(())
}

fn speaker_embedding_for_ranges(
    audio: &LoadedAudio,
    ranges: &[crate::SpeakerCorrectionRange],
) -> Result<SpeakerEmbedding, NativeWhisperxError> {
    let mut samples = Vec::new();
    for range in ranges {
        validate_speaker_correction_range(*range)?;
        let start = ((range.start_seconds * audio.sample_rate as f64).floor() as usize)
            .min(audio.samples.len());
        let end = ((range.end_seconds * audio.sample_rate as f64).ceil() as usize)
            .min(audio.samples.len());
        if end > start {
            samples.extend_from_slice(&audio.samples[start..end]);
        }
    }
    if samples.is_empty() {
        return Err(NativeWhisperxError::InvalidConfig(
            "speaker correction did not select any audio samples".to_string(),
        ));
    }
    let speaker_audio = SpeakerAudio::mono(&samples, audio.sample_rate).map_err(|error| {
        NativeWhisperxError::Transcription(format!("speaker correction audio invalid: {error}"))
    })?;
    let mut embedder = SpectralSpeakerEmbedder::default();
    embedder
        .embed_speaker(&speaker_audio)
        .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
}

fn replace_speaker_labels(
    transcript: &mut TranscriptionContract,
    from_speaker: &str,
    to_label: &str,
    filters: &[crate::SpeakerCorrectionRange],
) {
    for segment in &mut transcript.segments {
        if segment.speaker.as_deref() != Some(from_speaker) {
            continue;
        }
        let selected = if filters.is_empty() {
            true
        } else {
            segment
                .start_seconds
                .zip(segment.end_seconds)
                .is_some_and(|(start, end)| {
                    filters.iter().any(|filter| filter.overlaps(start, end))
                })
        };
        if !selected {
            continue;
        }
        segment.speaker = Some(to_label.to_string());
        for word in &mut segment.words {
            if word.speaker.as_deref() == Some(from_speaker) {
                word.speaker = Some(to_label.to_string());
            }
        }
    }
}

fn slug_speaker_id(value: &str) -> String {
    let slug = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if slug.is_empty() {
        "speaker".to_string()
    } else {
        slug
    }
}

fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

fn current_unix_seconds_string() -> String {
    current_unix_seconds().to_string()
}

#[cfg(feature = "diarization")]
#[allow(dead_code)]
pub(crate) fn read_whisperx_directory_state_for_ui(
    path: &Path,
) -> Result<crate::SpeakerDirectoryState, NativeWhisperxError> {
    let resolved = crate::speaker_directory::read_speaker_directory_state(
        &crate::speaker_directory::ResolvedSpeakerDirectory {
            path: path.to_path_buf(),
            scope: crate::speaker_directory::ResolvedSpeakerDirectoryScope::Local,
        },
    )?;
    Ok(resolved)
}

#[cfg(feature = "diarization")]
#[allow(dead_code)]
pub(crate) fn ensure_output_format_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

impl From<&NativeWhisperxConfig> for OutputConfig {
    fn from(_value: &NativeWhisperxConfig) -> Self {
        OutputConfig::default()
    }
}

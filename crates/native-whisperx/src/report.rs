use audio_analysis_transcription::TranscriptionPipelineResponse;
use text_transcripts::TranscriptionContract;

use crate::config::{is_pyannote_diarization_model, AsrProvider, NativeWhisperxConfig};

pub(crate) fn append_native_diarization_diagnostics(
    response: &mut TranscriptionPipelineResponse,
    config: &NativeWhisperxConfig,
) {
    if config.asr.provider != AsrProvider::Native
        || !config.diarization.enabled
        || !is_pyannote_diarization_model(&config.diarization.model_id)
    {
        return;
    }

    for diagnostic in [
        "diarizationPhase=segmentation",
        "diarizationPhase=embedding",
        "diarizationPhase=plda",
        "diarizationPhase=vbx",
        "diarizationPhase=clustering",
    ] {
        if !response
            .diagnostics
            .iter()
            .any(|existing| existing == diagnostic)
        {
            response.diagnostics.push(diagnostic.to_string());
        }
    }
}

pub(crate) fn append_native_alignment_diagnostics(
    response: &mut TranscriptionPipelineResponse,
    config: &NativeWhisperxConfig,
) {
    if config.asr.provider != AsrProvider::Native || !config.alignment.enabled {
        return;
    }
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentModelId",
        format!(
            "alignmentModelId={}",
            canonical_alignment_model_id(&config.alignment.model_id)
        ),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentFallbackCount",
        "alignmentFallbackCount=0".to_string(),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentRetryCount",
        "alignmentRetryCount=0".to_string(),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentWordTimingMissingCount",
        format!(
            "alignmentWordTimingMissingCount={}",
            alignment_word_timing_missing_count(&response.transcript)
        ),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentCharTimingMissingCount",
        format!(
            "alignmentCharTimingMissingCount={}",
            if config.alignment.return_char_alignments {
                alignment_char_timing_missing_count(&response.transcript)
            } else {
                0
            }
        ),
    );
}

fn canonical_alignment_model_id(model_id: &str) -> &str {
    if model_id.eq_ignore_ascii_case("WAV2VEC2_ASR_BASE_960H") {
        "facebook/wav2vec2-base-960h"
    } else {
        model_id
    }
}

fn push_diagnostic_if_missing(diagnostics: &mut Vec<String>, key: &str, diagnostic: String) {
    let prefix = format!("{key}=");
    if diagnostics
        .iter()
        .any(|existing| existing.starts_with(&prefix))
    {
        return;
    }
    diagnostics.push(diagnostic);
}

fn alignment_word_timing_missing_count(transcript: &TranscriptionContract) -> usize {
    transcript
        .segments
        .iter()
        .flat_map(|segment| segment.words.iter())
        .filter(|word| word.start_seconds.zip(word.end_seconds).is_none())
        .count()
}

fn alignment_char_timing_missing_count(transcript: &TranscriptionContract) -> usize {
    transcript
        .segments
        .iter()
        .flat_map(|segment| segment.chars.iter())
        .filter(|character| character.start_seconds.zip(character.end_seconds).is_none())
        .count()
}

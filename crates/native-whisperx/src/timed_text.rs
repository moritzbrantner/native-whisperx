//! Compatibility conversion from the current transcript contract to neutral timed text.

use media_core::TranscriptionContract;
use media_core::{
    TimedTextCharContract, TimedTextContract, TimedTextSegmentContract, TimedTextWordContract,
};

use crate::NativeWhisperxError;

/// Converts the current canonical transcript result into foundation-owned timed text.
///
/// The conversion preserves neutral media facts and rejects values that violate the
/// foundation timed-text invariants. WhisperX output policy is applied separately.
pub fn transcription_to_timed_text(
    transcript: &TranscriptionContract,
) -> Result<TimedTextContract, NativeWhisperxError> {
    let segments = transcript
        .segments
        .iter()
        .map(convert_segment)
        .collect::<Result<Vec<_>, _>>()?;
    let contract = TimedTextContract {
        text: transcript.text.clone(),
        language: transcript.language.clone(),
        segments,
        source: transcript.source.clone(),
        attributes: transcript.attributes.clone(),
    };
    contract.validate().map_err(timed_text_error)?;
    Ok(contract)
}

fn convert_segment(
    source: &media_core::TranscriptSegmentContract,
) -> Result<TimedTextSegmentContract, NativeWhisperxError> {
    let mut segment = TimedTextSegmentContract::new(source.index, source.text.clone())
        .with_time_range(source.start_seconds(), source.end_seconds())
        .map_err(timed_text_error)?
        .with_confidence(source.confidence())
        .map_err(timed_text_error)?;
    segment.language = source.language.clone();
    segment.speaker = source.speaker.clone();
    segment.is_final = source.is_final;
    segment.attributes = source.attributes.clone();
    for word in source.words() {
        segment
            .push_word(convert_word(word)?)
            .map_err(timed_text_error)?;
    }
    for character in source.chars() {
        segment
            .push_char(convert_char(character)?)
            .map_err(timed_text_error)?;
    }
    Ok(segment)
}

fn convert_word(
    source: &media_core::TranscriptWordContract,
) -> Result<TimedTextWordContract, NativeWhisperxError> {
    let mut word = TimedTextWordContract::new(source.text.clone())
        .with_time_range(source.start_seconds(), source.end_seconds())
        .map_err(timed_text_error)?
        .with_confidence(source.confidence())
        .map_err(timed_text_error)?;
    word.speaker = source.speaker.clone();
    word.attributes = source.attributes.clone();
    Ok(word)
}

fn convert_char(
    source: &media_core::TranscriptCharContract,
) -> Result<TimedTextCharContract, NativeWhisperxError> {
    let mut character = TimedTextCharContract::new(source.character.clone())
        .with_time_range(source.start_seconds(), source.end_seconds())
        .map_err(timed_text_error)?
        .with_confidence(source.confidence())
        .map_err(timed_text_error)?;
    character.attributes = source.attributes.clone();
    Ok(character)
}

pub(crate) fn timed_text_error(error: media_core::DetectError) -> NativeWhisperxError {
    NativeWhisperxError::Transcription(format!("timed-text conversion failed: {error}"))
}

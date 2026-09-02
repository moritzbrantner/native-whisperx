use media_core::{TranscriptSegmentContract, TranscriptionContract};

pub(crate) trait TranscriptionContractExt {
    fn joined_text(&self) -> String;
    fn text_or_joined(&self) -> String;
}

pub(crate) fn clear_segment_alignment(
    segment: &mut TranscriptSegmentContract,
) -> media_core::Result<()> {
    let confidence = segment.confidence();
    let mut replacement = TranscriptSegmentContract::new(segment.index, segment.text.clone())
        .with_time_range(segment.start_seconds(), segment.end_seconds())?
        .with_confidence(confidence)?;
    replacement.language = segment.language.clone();
    replacement.speaker = segment.speaker.clone();
    replacement.is_final = segment.is_final;
    replacement.attributes = segment.attributes.clone();
    *segment = replacement;
    Ok(())
}

impl TranscriptionContractExt for TranscriptionContract {
    fn joined_text(&self) -> String {
        self.segments
            .iter()
            .map(|segment| segment.text.trim())
            .filter(|text| !text.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn text_or_joined(&self) -> String {
        self.text
            .as_deref()
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| self.joined_text())
    }
}

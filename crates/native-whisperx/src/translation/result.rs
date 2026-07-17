//! Immutable translated-result contracts and execution.

use audio_analysis_transcription::TranscriptionPipelineResponse;
use serde::Serialize;

use super::{CuratedLanguage, TranslationLeg, TranslationPlan, TranslationPlanProvenance};

/// A boundary that executes one planned model leg for one transcript segment.
///
/// Implementations may resolve and reuse models however they choose. The
/// operation supplies legs in plan order and skips segments whose text is
/// empty after trimming.
pub trait SegmentTranslationProvider {
    /// Translates one non-empty segment using the supplied planned model leg.
    fn translate_segment(
        &mut self,
        leg: &TranslationLeg,
        text: &str,
    ) -> Result<String, TranslationModelError>;
}

/// A typed failure reported by a [`SegmentTranslationProvider`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub struct TranslationModelError {
    message: String,
}

impl TranslationModelError {
    /// Creates a provider failure with a user-facing explanation.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Returns the provider's failure explanation.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// A translated transcript kept separate from its source pipeline response.
///
/// Ordered legs are retained as both execution and model provenance: each leg
/// records its source, target, and canonical model ID.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslatedTranscriptionResult {
    transcript: text_transcripts::TranscriptionContract,
    source_language: CuratedLanguage,
    target_language: CuratedLanguage,
    provenance: TranslationPlanProvenance,
    legs: Vec<TranslationLeg>,
}

impl TranslatedTranscriptionResult {
    /// Returns the separately owned target-language transcript.
    pub fn transcript(&self) -> &text_transcripts::TranscriptionContract {
        &self.transcript
    }

    /// Consumes the result and returns its target-language transcript.
    pub fn into_transcript(self) -> text_transcripts::TranscriptionContract {
        self.transcript
    }

    /// Returns the original language consumed by the plan.
    pub const fn source_language(&self) -> CuratedLanguage {
        self.source_language
    }

    /// Returns the result language produced by the plan.
    pub const fn target_language(&self) -> CuratedLanguage {
        self.target_language
    }

    /// Returns whether the result came from a direct or Pivot Translation.
    pub const fn provenance(&self) -> TranslationPlanProvenance {
        self.provenance
    }

    /// Returns executed legs in order, including each model ID.
    pub fn legs(&self) -> &[TranslationLeg] {
        &self.legs
    }
}

/// A typed failure while executing an already validated translation plan.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TranslationError {
    /// A model leg failed while translating a source segment.
    #[error(
        "translation leg {leg_index} ({leg_source}->{leg_target}, model `{model_id}`) failed for segment {segment_index}: {source}"
    )]
    LegFailed {
        /// Zero-based position in the ordered plan.
        leg_index: usize,
        /// Stable source transcript segment index.
        segment_index: u64,
        /// Source language of the failed leg.
        leg_source: CuratedLanguage,
        /// Target language of the failed leg.
        leg_target: CuratedLanguage,
        /// Canonical model ID of the failed leg.
        model_id: String,
        /// Provider failure that stopped execution.
        #[source]
        source: TranslationModelError,
    },
}

/// Executes a plan without taking ownership of or mutating the source result.
///
/// Segment boundaries, timings, speaker facts, confidence, attributes, and
/// transcript metadata are copied to the target result. Source-language word
/// and character alignments are omitted because they do not describe the
/// translated text.
pub fn translate_transcription(
    source: &TranscriptionPipelineResponse,
    plan: &TranslationPlan,
    provider: &mut dyn SegmentTranslationProvider,
) -> Result<TranslatedTranscriptionResult, TranslationError> {
    let mut transcript = source.transcript.clone();

    for (leg_index, leg) in plan.legs().iter().enumerate() {
        for segment in &mut transcript.segments {
            let source_text = segment.text.trim();
            if source_text.is_empty() {
                continue;
            }
            segment.text = provider
                .translate_segment(leg, source_text)
                .map_err(|source| TranslationError::LegFailed {
                    leg_index,
                    segment_index: segment.index,
                    leg_source: leg.source(),
                    leg_target: leg.target(),
                    model_id: leg.model_id().to_string(),
                    source,
                })?;
        }
    }

    let target_language = plan.target().code().to_string();
    for segment in &mut transcript.segments {
        segment.language = Some(target_language.clone());
        segment.words.clear();
        segment.chars.clear();
    }
    transcript.language = Some(target_language);
    transcript.text = Some(transcript.joined_text());

    Ok(TranslatedTranscriptionResult {
        transcript,
        source_language: plan.source(),
        target_language: plan.target(),
        provenance: plan.provenance(),
        legs: plan.legs().to_vec(),
    })
}

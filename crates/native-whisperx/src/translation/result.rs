//! Immutable translated-result contracts and execution.

use audio_analysis_transcription::TranscriptionPipelineResponse;
use serde::Serialize;
use std::{path::PathBuf, time::Instant};

use super::{CuratedLanguage, TranslationLeg, TranslationPlan, TranslationPlanProvenance};
use crate::workflow::{
    CancellationHandle, FiniteCancellation, NoopTranscriptionProgressObserver,
    TranscriptionProgressEvent, TranscriptionProgressObserver, TranscriptionProgressTask,
};

/// A boundary that executes one planned model leg for one transcript segment.
///
/// Implementations may resolve and reuse models however they choose. The
/// operation supplies legs in plan order and skips segments whose text is
/// empty after trimming.
pub trait SegmentTranslationProvider {
    /// Stable provider identifier used by the Transcription Progress Stream.
    fn provider_id(&self) -> &str {
        "segment-translation-provider"
    }

    /// Resolves and prepares one planned model leg before its first segment.
    ///
    /// The default implementation preserves lightweight providers that need no
    /// preparation. Native model providers use this boundary to emit public
    /// model resolution, download, load, and reuse progress. The executor
    /// checks cooperative cancellation immediately before and after this call;
    /// blocking model resolution and loading are not interrupted midway.
    fn prepare_leg(
        &mut self,
        _leg: &TranslationLeg,
        _file_index: usize,
        _observer: &mut dyn TranscriptionProgressObserver,
    ) -> Result<(), TranslationModelError> {
        Ok(())
    }

    /// Translates one non-empty segment using the supplied planned model leg.
    fn translate_segment(
        &mut self,
        leg: &TranslationLeg,
        text: &str,
    ) -> Result<String, TranslationModelError>;
}

/// Typed result of translating an immutable transcription with cooperative control.
#[derive(Debug)]
pub enum TranslatedTranscriptionOutcome {
    /// Every direct or Pivot Translation leg completed in plan order.
    Completed(TranslatedTranscriptionResult),
    /// Translation stopped at a safe leg or segment boundary.
    Cancelled(FiniteCancellation),
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
    let mut observer = NoopTranscriptionProgressObserver;
    let cancellation = CancellationHandle::new();
    match translate_transcription_with_control(
        source,
        plan,
        provider,
        0,
        PathBuf::from("<translation>"),
        &mut observer,
        &cancellation,
    )? {
        TranslatedTranscriptionOutcome::Completed(result) => Ok(result),
        TranslatedTranscriptionOutcome::Cancelled(_) => {
            unreachable!("the compatibility translation entry point uses an uncancelled handle")
        }
    }
}

/// Executes direct or Pivot Translation with ordered leg progress and cancellation.
pub fn translate_transcription_with_control(
    source: &TranscriptionPipelineResponse,
    plan: &TranslationPlan,
    provider: &mut dyn SegmentTranslationProvider,
    file_index: usize,
    input: PathBuf,
    observer: &mut dyn TranscriptionProgressObserver,
    cancellation: &CancellationHandle,
) -> Result<TranslatedTranscriptionOutcome, TranslationError> {
    let started = Instant::now();
    if cancellation.is_cancelled() {
        return Ok(cancelled_translation(file_index, input, observer, started));
    }

    let mut transcript = source.transcript.clone();
    observer.observe(TranscriptionProgressEvent::TaskStart {
        file_index,
        task: TranscriptionProgressTask::Translation,
    });

    for (leg_index, leg) in plan.legs().iter().enumerate() {
        if cancellation.is_cancelled() {
            return Ok(cancelled_translation(file_index, input, observer, started));
        }
        let leg_started = Instant::now();
        let provider_id = provider.provider_id().to_string();
        observer.observe(TranscriptionProgressEvent::TranslationLegStart {
            file_index,
            leg_index,
            total_legs: plan.legs().len(),
            provenance: plan.provenance(),
            source: leg.source(),
            target: leg.target(),
            provider: provider_id.clone(),
            model_id: leg.model_id().to_string(),
        });
        if cancellation.is_cancelled() {
            return Ok(cancelled_translation(file_index, input, observer, started));
        }
        if let Some(segment_index) = transcript
            .segments
            .iter()
            .find(|segment| !segment.text.trim().is_empty())
            .map(|segment| segment.index)
        {
            if let Err(source) = provider.prepare_leg(leg, file_index, observer) {
                let error = TranslationError::LegFailed {
                    leg_index,
                    segment_index,
                    leg_source: leg.source(),
                    leg_target: leg.target(),
                    model_id: leg.model_id().to_string(),
                    source,
                };
                observer.observe(TranscriptionProgressEvent::Failure {
                    file_index,
                    input: input.clone(),
                    task: Some(TranscriptionProgressTask::Translation),
                    duration_seconds: started.elapsed().as_secs_f64(),
                    message: error.to_string(),
                });
                return Err(error);
            }
        }
        if cancellation.is_cancelled() {
            return Ok(cancelled_translation(file_index, input, observer, started));
        }
        for segment in &mut transcript.segments {
            if cancellation.is_cancelled() {
                return Ok(cancelled_translation(file_index, input, observer, started));
            }
            let source_text = segment.text.trim();
            if source_text.is_empty() {
                continue;
            }
            segment.text = match provider.translate_segment(leg, source_text) {
                Ok(translated) => translated,
                Err(source) => {
                    let error = TranslationError::LegFailed {
                        leg_index,
                        segment_index: segment.index,
                        leg_source: leg.source(),
                        leg_target: leg.target(),
                        model_id: leg.model_id().to_string(),
                        source,
                    };
                    observer.observe(TranscriptionProgressEvent::Failure {
                        file_index,
                        input: input.clone(),
                        task: Some(TranscriptionProgressTask::Translation),
                        duration_seconds: started.elapsed().as_secs_f64(),
                        message: error.to_string(),
                    });
                    return Err(error);
                }
            };
        }
        if cancellation.is_cancelled() {
            return Ok(cancelled_translation(file_index, input, observer, started));
        }
        observer.observe(TranscriptionProgressEvent::TranslationLegEnd {
            file_index,
            leg_index,
            total_legs: plan.legs().len(),
            provenance: plan.provenance(),
            source: leg.source(),
            target: leg.target(),
            provider: provider_id,
            model_id: leg.model_id().to_string(),
            duration_seconds: leg_started.elapsed().as_secs_f64(),
        });
        if cancellation.is_cancelled() {
            return Ok(cancelled_translation(file_index, input, observer, started));
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

    let result = TranslatedTranscriptionResult {
        transcript,
        source_language: plan.source(),
        target_language: plan.target(),
        provenance: plan.provenance(),
        legs: plan.legs().to_vec(),
    };
    observer.observe(TranscriptionProgressEvent::TaskEnd {
        file_index,
        task: TranscriptionProgressTask::Translation,
        duration_seconds: started.elapsed().as_secs_f64(),
    });
    Ok(TranslatedTranscriptionOutcome::Completed(result))
}

fn cancelled_translation(
    file_index: usize,
    input: PathBuf,
    observer: &mut dyn TranscriptionProgressObserver,
    started: Instant,
) -> TranslatedTranscriptionOutcome {
    let cancellation = FiniteCancellation::new(
        file_index,
        input.clone(),
        Some(TranscriptionProgressTask::Translation),
    );
    observer.observe(TranscriptionProgressEvent::Cancelled {
        file_index,
        input,
        task: Some(TranscriptionProgressTask::Translation),
        duration_seconds: started.elapsed().as_secs_f64(),
    });
    TranslatedTranscriptionOutcome::Cancelled(cancellation)
}

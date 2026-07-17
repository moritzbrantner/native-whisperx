use native_whisperx::{
    translate_transcription, translate_transcription_with_control, CancellationHandle,
    CuratedLanguage, SegmentTranslationProvider, TranscriptionPipelineResponse,
    TranscriptionProgressEvent, TranscriptionProgressObserver, TranslatedTranscriptionOutcome,
    TranslatedTranscriptionResult, TranslationError, TranslationLeg, TranslationModelError,
    TranslationPlan, TranslationPlanProvenance,
};
use serde_json::json;
use std::path::PathBuf;

#[derive(Default)]
struct RecordingTranslator {
    calls: Vec<(String, String)>,
}

struct FailingTranslator {
    fail_model_id: &'static str,
    calls: Vec<(String, String)>,
}

#[derive(Default)]
struct RecordingProgress {
    events: Vec<TranscriptionProgressEvent>,
    cancellation: Option<CancellationHandle>,
    cancel_on_leg: Option<usize>,
}

impl TranscriptionProgressObserver for RecordingProgress {
    fn observe(&mut self, event: TranscriptionProgressEvent) {
        if matches!(
            event,
            TranscriptionProgressEvent::TranslationLegStart { leg_index, .. }
                if self.cancel_on_leg == Some(leg_index)
        ) {
            self.cancellation
                .as_ref()
                .expect("cancelling observer should have a handle")
                .cancel();
        }
        self.events.push(event);
    }
}

#[test]
fn direct_and_pivot_translation_emit_ordered_leg_progress() {
    for (target, expected_legs) in [
        (CuratedLanguage::English, 1),
        (CuratedLanguage::Portuguese, 2),
    ] {
        let source = source_response();
        let plan = TranslationPlan::new(CuratedLanguage::German, target).expect("valid plan");
        let mut translator = RecordingTranslator::default();
        let cancellation = CancellationHandle::new();
        let mut progress = RecordingProgress::default();

        let outcome = translate_transcription_with_control(
            &source,
            &plan,
            &mut translator,
            3,
            PathBuf::from("subtitle.wav"),
            &mut progress,
            &cancellation,
        )
        .expect("offline translation should complete");

        assert!(matches!(
            outcome,
            TranslatedTranscriptionOutcome::Completed(_)
        ));
        let starts = progress
            .events
            .iter()
            .filter_map(|event| match event {
                TranscriptionProgressEvent::TranslationLegStart {
                    file_index,
                    leg_index,
                    total_legs,
                    provenance,
                    model_id,
                    ..
                } => Some((
                    *file_index,
                    *leg_index,
                    *total_legs,
                    *provenance,
                    model_id.as_str(),
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(starts.len(), expected_legs);
        assert!(starts
            .iter()
            .enumerate()
            .all(|(index, event)| event.0 == 3 && event.1 == index && event.2 == expected_legs));
        assert_eq!(starts[0].3, plan.provenance());
        assert_eq!(starts[0].4, plan.legs()[0].model_id());
    }
}

#[test]
fn cancellation_between_pivot_translation_legs_skips_the_later_model() {
    let source = source_response();
    let source_before = source.clone();
    let plan = TranslationPlan::new(CuratedLanguage::German, CuratedLanguage::Portuguese)
        .expect("German to Portuguese should pivot through English");
    let mut translator = RecordingTranslator::default();
    let cancellation = CancellationHandle::new();
    let mut progress = RecordingProgress {
        events: Vec::new(),
        cancellation: Some(cancellation.clone()),
        cancel_on_leg: Some(1),
    };

    let outcome = translate_transcription_with_control(
        &source,
        &plan,
        &mut translator,
        0,
        PathBuf::from("subtitle.wav"),
        &mut progress,
        &cancellation,
    )
    .expect("cooperative cancellation is not a translation failure");

    assert!(matches!(
        outcome,
        TranslatedTranscriptionOutcome::Cancelled(_)
    ));
    assert_eq!(source, source_before);
    assert_eq!(translator.calls.len(), 2);
    assert!(translator
        .calls
        .iter()
        .all(|(model_id, _)| model_id == plan.legs()[0].model_id()));
    assert!(progress
        .events
        .iter()
        .any(|event| matches!(event, TranscriptionProgressEvent::Cancelled { .. })));
}

impl SegmentTranslationProvider for FailingTranslator {
    fn translate_segment(
        &mut self,
        leg: &TranslationLeg,
        text: &str,
    ) -> Result<String, TranslationModelError> {
        self.calls
            .push((leg.model_id().to_string(), text.to_string()));
        if leg.model_id() == self.fail_model_id {
            return Err(TranslationModelError::new("offline provider failure"));
        }
        Ok(format!("{text} translated"))
    }
}

#[test]
fn first_leg_failure_is_typed_and_leaves_the_source_available() {
    let source = source_response();
    let source_before = source.clone();
    let plan = TranslationPlan::new(CuratedLanguage::German, CuratedLanguage::Portuguese)
        .expect("German to Portuguese should use an English pivot");
    let mut translator = FailingTranslator {
        fail_model_id: "Helsinki-NLP/opus-mt-de-en",
        calls: Vec::new(),
    };

    let error = translate_transcription(&source, &plan, &mut translator)
        .expect_err("first pivot leg should fail");

    assert_eq!(source, source_before);
    assert_eq!(
        error,
        TranslationError::LegFailed {
            leg_index: 0,
            segment_index: 0,
            leg_source: CuratedLanguage::German,
            leg_target: CuratedLanguage::English,
            model_id: "Helsinki-NLP/opus-mt-de-en".to_string(),
            source: TranslationModelError::new("offline provider failure"),
        }
    );
    assert_eq!(translator.calls.len(), 1);
}

#[test]
fn controlled_translation_failure_emits_failure_instead_of_cancellation() {
    let source = source_response();
    let plan = TranslationPlan::new(CuratedLanguage::German, CuratedLanguage::English)
        .expect("direct plan");
    let mut translator = FailingTranslator {
        fail_model_id: "Helsinki-NLP/opus-mt-de-en",
        calls: Vec::new(),
    };
    let cancellation = CancellationHandle::new();
    let mut progress = RecordingProgress::default();

    translate_transcription_with_control(
        &source,
        &plan,
        &mut translator,
        2,
        PathBuf::from("failed.wav"),
        &mut progress,
        &cancellation,
    )
    .expect_err("provider failure should remain typed");

    assert!(progress.events.iter().any(|event| matches!(
        event,
        TranscriptionProgressEvent::Failure {
            file_index: 2,
            input,
            task: Some(native_whisperx::TranscriptionProgressTask::Translation),
            ..
        } if input == &PathBuf::from("failed.wav")
    )));
    assert!(!progress
        .events
        .iter()
        .any(|event| matches!(event, TranscriptionProgressEvent::Cancelled { .. })));
}

#[test]
fn second_leg_failure_is_typed_and_leaves_the_source_available() {
    let source = source_response();
    let source_before = source.clone();
    let plan = TranslationPlan::new(CuratedLanguage::German, CuratedLanguage::Portuguese)
        .expect("German to Portuguese should use an English pivot");
    let mut translator = FailingTranslator {
        fail_model_id: "Helsinki-NLP/opus-mt-tc-big-en-pt",
        calls: Vec::new(),
    };

    let error = translate_transcription(&source, &plan, &mut translator)
        .expect_err("second pivot leg should fail");

    assert_eq!(source, source_before);
    assert_eq!(
        error,
        TranslationError::LegFailed {
            leg_index: 1,
            segment_index: 0,
            leg_source: CuratedLanguage::English,
            leg_target: CuratedLanguage::Portuguese,
            model_id: "Helsinki-NLP/opus-mt-tc-big-en-pt".to_string(),
            source: TranslationModelError::new("offline provider failure"),
        }
    );
    assert_eq!(translator.calls.len(), 3);
}

#[test]
fn empty_segments_keep_their_timing_without_invoking_models() {
    let mut source = source_response();
    source.transcript.text = Some(String::new());
    source.transcript.segments.truncate(1);
    source.transcript.segments[0].text.clear();
    let source_before = source.clone();
    let plan = TranslationPlan::new(CuratedLanguage::German, CuratedLanguage::English)
        .expect("German to English should have a direct plan");
    let mut translator = RecordingTranslator::default();

    let translated = translate_transcription(&source, &plan, &mut translator)
        .expect("an empty segment should not require model execution");

    assert_eq!(source, source_before);
    assert!(translator.calls.is_empty());
    assert_eq!(translated.transcript().text.as_deref(), Some(""));
    assert_eq!(translated.transcript().segments.len(), 1);
    assert_eq!(translated.transcript().segments[0].text, "");
    assert_eq!(
        translated.transcript().segments[0].start_seconds,
        Some(1.25)
    );
    assert_eq!(translated.transcript().segments[0].end_seconds, Some(2.5));
    assert_eq!(
        translated.transcript().segments[0].language.as_deref(),
        Some("en")
    );
    assert!(translated.transcript().segments[0].words.is_empty());
    assert!(translated.transcript().segments[0].chars.is_empty());
}

#[test]
fn pivot_translation_executes_ordered_legs_and_records_model_provenance() {
    let source = source_response();
    let source_before = source.clone();
    let plan = TranslationPlan::new(CuratedLanguage::German, CuratedLanguage::Portuguese)
        .expect("German to Portuguese should use an English pivot");
    let mut translator = RecordingTranslator::default();

    let translated = translate_transcription(&source, &plan, &mut translator)
        .expect("pivot translation should succeed");

    assert_eq!(source, source_before);
    assert_eq!(
        translated.provenance(),
        TranslationPlanProvenance::PivotTranslation {
            pivot: CuratedLanguage::English,
        }
    );
    assert_eq!(translated.legs(), plan.legs());
    assert_eq!(
        translated
            .legs()
            .iter()
            .map(TranslationLeg::model_id)
            .collect::<Vec<_>>(),
        vec![
            "Helsinki-NLP/opus-mt-de-en",
            "Helsinki-NLP/opus-mt-tc-big-en-pt",
        ]
    );
    assert_eq!(translated.target_language(), CuratedLanguage::Portuguese);
    assert_eq!(translated.transcript().language.as_deref(), Some("pt"));
    assert_eq!(
        translated.transcript().text.as_deref(),
        Some("Guten Tag translated translated Wie geht es dir? translated translated")
    );
    assert_eq!(
        translator.calls,
        vec![
            (
                "Helsinki-NLP/opus-mt-de-en".to_string(),
                "Guten Tag".to_string(),
            ),
            (
                "Helsinki-NLP/opus-mt-de-en".to_string(),
                "Wie geht es dir?".to_string(),
            ),
            (
                "Helsinki-NLP/opus-mt-tc-big-en-pt".to_string(),
                "Guten Tag translated".to_string(),
            ),
            (
                "Helsinki-NLP/opus-mt-tc-big-en-pt".to_string(),
                "Wie geht es dir? translated".to_string(),
            ),
        ]
    );
}

impl SegmentTranslationProvider for RecordingTranslator {
    fn translate_segment(
        &mut self,
        leg: &TranslationLeg,
        text: &str,
    ) -> Result<String, TranslationModelError> {
        self.calls
            .push((leg.model_id().to_string(), text.to_string()));
        Ok(format!("{text} translated"))
    }
}

#[test]
fn direct_translation_returns_a_separate_timed_result_without_changing_the_source() {
    let source = source_response();
    let source_before = source.clone();
    let plan = TranslationPlan::new(CuratedLanguage::German, CuratedLanguage::English)
        .expect("German to English should have a direct plan");
    let mut translator = RecordingTranslator::default();

    let translated = translate_transcription(&source, &plan, &mut translator)
        .expect("direct translation should succeed");

    assert_eq!(source, source_before);
    assert_direct_result(&translated);
    assert_eq!(
        translator.calls,
        vec![
            (
                "Helsinki-NLP/opus-mt-de-en".to_string(),
                "Guten Tag".to_string()
            ),
            (
                "Helsinki-NLP/opus-mt-de-en".to_string(),
                "Wie geht es dir?".to_string()
            ),
        ]
    );
}

fn assert_direct_result(translated: &TranslatedTranscriptionResult) {
    assert_eq!(translated.source_language(), CuratedLanguage::German);
    assert_eq!(translated.target_language(), CuratedLanguage::English);
    assert_eq!(translated.legs().len(), 1);
    assert_eq!(
        translated.legs()[0].model_id(),
        "Helsinki-NLP/opus-mt-de-en"
    );

    let transcript = translated.transcript();
    assert_eq!(transcript.language.as_deref(), Some("en"));
    assert_eq!(
        transcript.text.as_deref(),
        Some("Guten Tag translated Wie geht es dir? translated")
    );
    assert_eq!(transcript.source.as_deref(), Some("fixture.wav"));
    assert_eq!(
        transcript.attributes.get("workflow"),
        Some(&"offline-test".to_string())
    );
    assert_eq!(transcript.segments[0].start_seconds, Some(1.25));
    assert_eq!(transcript.segments[0].end_seconds, Some(2.5));
    assert_eq!(transcript.segments[1].start_seconds, Some(3.0));
    assert_eq!(transcript.segments[1].end_seconds, Some(4.75));
    assert_eq!(transcript.segments[0].language.as_deref(), Some("en"));
    assert_eq!(
        transcript.segments[0].speaker.as_deref(),
        Some("SPEAKER_00")
    );
    assert!(transcript.segments[0].words.is_empty());
    assert!(transcript.segments[0].chars.is_empty());
}

fn source_response() -> TranscriptionPipelineResponse {
    serde_json::from_value(json!({
        "accepted": true,
        "operation": "transcribe",
        "provider": "native",
        "modelId": "small",
        "transcript": {
            "text": "Guten Tag Wie geht es dir?",
            "language": "de",
            "source": "fixture.wav",
            "attributes": { "workflow": "offline-test" },
            "segments": [
                {
                    "index": 0,
                    "startSeconds": 1.25,
                    "endSeconds": 2.5,
                    "text": "Guten Tag",
                    "language": "de",
                    "speaker": "SPEAKER_00",
                    "confidence": 0.98,
                    "isFinal": true,
                    "words": [{
                        "text": "Guten",
                        "startSeconds": 1.25,
                        "endSeconds": 1.8,
                        "confidence": 0.97,
                        "speaker": "SPEAKER_00",
                        "attributes": { "aligned": "true" }
                    }],
                    "chars": [{
                        "char": "G",
                        "start": 1.25,
                        "end": 1.3,
                        "score": 0.96,
                        "attributes": { "aligned": "true" }
                    }],
                    "attributes": { "segment": "first" }
                },
                {
                    "index": 1,
                    "startSeconds": 3.0,
                    "endSeconds": 4.75,
                    "text": "Wie geht es dir?",
                    "language": "de",
                    "speaker": "SPEAKER_01",
                    "confidence": 0.95,
                    "isFinal": true,
                    "words": [],
                    "chars": [],
                    "attributes": { "segment": "second" }
                }
            ]
        },
        "vadSegments": [{ "startSeconds": 1.0, "endSeconds": 5.0, "score": 0.9 }],
        "alignment": { "provider": "native", "modelId": "wav2vec2", "wordCount": 5 },
        "diarization": null,
        "artifacts": [{ "kind": "json", "path": "fixture.json" }],
        "diagnostics": ["asrModel=small", "phaseAlignmentSeconds=0.25"]
    }))
    .expect("source response fixture should deserialize")
}

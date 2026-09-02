use native_whisperx::{transcription_to_timed_text, TranscriptionContract};
use serde_json::json;

#[test]
fn canonical_transcript_converts_to_neutral_timed_text_without_losing_media_facts() {
    let transcript: TranscriptionContract = serde_json::from_value(json!({
        "text": "hello",
        "language": "en",
        "source": "fixture.wav",
        "attributes": { "workflow": "test" },
        "segments": [{
            "index": 7,
            "startSeconds": 1.25,
            "endSeconds": 2.5,
            "text": "hello",
            "language": "en",
            "speaker": "SPEAKER_00",
            "confidence": 0.95,
            "isFinal": true,
            "attributes": { "segment": "kept" },
            "words": [{
                "text": "hello",
                "startSeconds": 1.25,
                "endSeconds": 2.5,
                "confidence": 0.9,
                "speaker": "SPEAKER_00",
                "attributes": { "word": "kept" }
            }],
            "chars": [{
                "char": "h",
                "start": 1.25,
                "end": 1.4,
                "score": 0.85,
                "attributes": { "char": "kept" }
            }]
        }]
    }))
    .expect("canonical transcript fixture");

    let timed_text = transcription_to_timed_text(&transcript).expect("neutral timed text");

    assert_eq!(timed_text.text.as_deref(), Some("hello"));
    assert_eq!(timed_text.language.as_deref(), Some("en"));
    assert_eq!(timed_text.source.as_deref(), Some("fixture.wav"));
    assert_eq!(timed_text.attributes["workflow"], "test");
    let segment = &timed_text.segments[0];
    assert_eq!(segment.index, 7);
    assert_eq!(
        segment
            .time_range()
            .expect("valid range")
            .unwrap()
            .start_seconds(),
        1.25
    );
    assert_eq!(segment.speaker.as_deref(), Some("SPEAKER_00"));
    assert_eq!(segment.confidence(), Some(0.95));
    assert_eq!(segment.attributes["segment"], "kept");
    assert_eq!(segment.words()[0].confidence(), Some(0.9));
    assert_eq!(segment.words()[0].attributes["word"], "kept");
    assert_eq!(segment.chars()[0].confidence(), Some(0.85));
    assert_eq!(segment.chars()[0].attributes["char"], "kept");
}

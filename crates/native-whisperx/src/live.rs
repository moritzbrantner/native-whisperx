use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub enum LiveTranscriptEvent {
    SessionStarted(LiveSessionStarted),
    Partial(LivePartialTranscript),
    Final(LiveFinalTranscriptSegment),
    Error(LiveTranscriptError),
    SessionEnded(LiveSessionEnded),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveSessionStarted {
    pub session_id: String,
    pub sequence: u64,
    pub source: String,
    pub ingest_started_at_utc: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub model_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LivePartialTranscript {
    pub session_id: String,
    pub sequence: u64,
    pub window_start_seconds: f64,
    pub window_end_seconds: f64,
    pub window_start_at_utc: String,
    pub window_end_at_utc: String,
    pub text: String,
    pub segments: Vec<LivePartialSegment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LivePartialSegment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveFinalTranscriptSegment {
    pub session_id: String,
    pub sequence: u64,
    pub segment_id: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub start_at_utc: String,
    pub end_at_utc: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTranscriptError {
    pub session_id: String,
    pub sequence: u64,
    pub message: String,
    pub recoverable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveSessionEnded {
    pub session_id: String,
    pub sequence: u64,
    pub reason: LiveSessionEndReason,
    pub processed_audio_seconds: f64,
    pub final_segment_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LiveSessionEndReason {
    Completed,
    Error,
    Cancelled,
}

pub fn live_transcript_events_to_jsonl(
    events: &[LiveTranscriptEvent],
) -> Result<String, serde_json::Error> {
    let mut jsonl = String::new();
    for event in events {
        jsonl.push_str(&serde_json::to_string(event)?);
        jsonl.push('\n');
    }
    Ok(jsonl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_started_serializes_camel_case_event_contract() {
        let event = LiveTranscriptEvent::SessionStarted(LiveSessionStarted {
            session_id: "session-1".to_string(),
            sequence: 1,
            source: "rtsp://camera/live".to_string(),
            ingest_started_at_utc: "2026-06-22T15:30:00Z".to_string(),
            sample_rate: 16_000,
            channels: 1,
            model_id: "tiny.en".to_string(),
            language: Some("en".to_string()),
        });

        let json = serde_json::to_value(&event).expect("event serializes");

        assert_eq!(
            json,
            serde_json::json!({
                "event": "sessionStarted",
                "sessionId": "session-1",
                "sequence": 1,
                "source": "rtsp://camera/live",
                "ingestStartedAtUtc": "2026-06-22T15:30:00Z",
                "sampleRate": 16000,
                "channels": 1,
                "modelId": "tiny.en",
                "language": "en"
            })
        );
    }

    #[test]
    fn live_transcript_events_serialize_all_jsonl_shapes() {
        let events = vec![
            LiveTranscriptEvent::Partial(LivePartialTranscript {
                session_id: "session-1".to_string(),
                sequence: 2,
                window_start_seconds: 0.0,
                window_end_seconds: 5.0,
                window_start_at_utc: "2026-06-22T15:30:00Z".to_string(),
                window_end_at_utc: "2026-06-22T15:30:05Z".to_string(),
                text: "hello wor".to_string(),
                segments: vec![LivePartialSegment {
                    start_seconds: 0.4,
                    end_seconds: 1.8,
                    text: "hello wor".to_string(),
                    language: Some("en".to_string()),
                }],
            }),
            LiveTranscriptEvent::Final(LiveFinalTranscriptSegment {
                session_id: "session-1".to_string(),
                sequence: 3,
                segment_id: "seg-000001".to_string(),
                start_seconds: 0.4,
                end_seconds: 1.9,
                start_at_utc: "2026-06-22T15:30:00.400Z".to_string(),
                end_at_utc: "2026-06-22T15:30:01.900Z".to_string(),
                text: "hello world".to_string(),
                language: Some("en".to_string()),
            }),
            LiveTranscriptEvent::Error(LiveTranscriptError {
                session_id: "session-1".to_string(),
                sequence: 4,
                message: "processing fell behind live input".to_string(),
                recoverable: false,
            }),
            LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                session_id: "session-1".to_string(),
                sequence: 5,
                reason: LiveSessionEndReason::Error,
                processed_audio_seconds: 12.5,
                final_segment_count: 1,
            }),
        ];

        let jsonl = live_transcript_events_to_jsonl(&events).expect("events serialize");
        let lines = jsonl.lines().collect::<Vec<_>>();

        assert_eq!(lines.len(), 4);
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(lines[0]).expect("partial json"),
            serde_json::json!({
                "event": "partial",
                "sessionId": "session-1",
                "sequence": 2,
                "windowStartSeconds": 0.0,
                "windowEndSeconds": 5.0,
                "windowStartAtUtc": "2026-06-22T15:30:00Z",
                "windowEndAtUtc": "2026-06-22T15:30:05Z",
                "text": "hello wor",
                "segments": [{
                    "startSeconds": 0.4,
                    "endSeconds": 1.8,
                    "text": "hello wor",
                    "language": "en"
                }]
            })
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(lines[1]).expect("final json"),
            serde_json::json!({
                "event": "final",
                "sessionId": "session-1",
                "sequence": 3,
                "segmentId": "seg-000001",
                "startSeconds": 0.4,
                "endSeconds": 1.9,
                "startAtUtc": "2026-06-22T15:30:00.400Z",
                "endAtUtc": "2026-06-22T15:30:01.900Z",
                "text": "hello world",
                "language": "en"
            })
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(lines[2]).expect("error json"),
            serde_json::json!({
                "event": "error",
                "sessionId": "session-1",
                "sequence": 4,
                "message": "processing fell behind live input",
                "recoverable": false
            })
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(lines[3]).expect("ended json"),
            serde_json::json!({
                "event": "sessionEnded",
                "sessionId": "session-1",
                "sequence": 5,
                "reason": "error",
                "processedAudioSeconds": 12.5,
                "finalSegmentCount": 1
            })
        );
        assert!(jsonl.ends_with('\n'));
    }
}

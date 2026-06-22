use serde::{Deserialize, Serialize};
use thiserror::Error;

const DEFAULT_STABILITY_TOLERANCE_SECONDS: f64 = 0.4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiveWindowingConfig {
    pub window_seconds: f64,
    pub hop_seconds: f64,
    pub finalize_lag_seconds: f64,
    pub max_buffer_lag_seconds: f64,
    pub stability_tolerance_seconds: f64,
}

impl Default for LiveWindowingConfig {
    fn default() -> Self {
        Self {
            window_seconds: 5.0,
            hop_seconds: 2.5,
            finalize_lag_seconds: 5.0,
            max_buffer_lag_seconds: 30.0,
            stability_tolerance_seconds: DEFAULT_STABILITY_TOLERANCE_SECONDS,
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum LiveWindowingError {
    #[error("{field} must be finite and greater than zero")]
    InvalidPositiveSeconds { field: &'static str },
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiveAsrSegmentCandidate {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub start_at_utc: String,
    pub end_at_utc: String,
    pub text: String,
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiveWindowTranscriptObservation {
    pub session_id: String,
    pub window_start_seconds: f64,
    pub window_end_seconds: f64,
    pub window_start_at_utc: String,
    pub window_end_at_utc: String,
    pub latest_ingested_audio_seconds: f64,
    pub segments: Vec<LiveAsrSegmentCandidate>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiveWindow {
    pub start_seconds: f64,
    pub end_seconds: f64,
}

#[derive(Debug, Clone)]
pub struct LiveWindowPlanner {
    config: LiveWindowingConfig,
}

impl LiveWindowPlanner {
    pub fn new(config: LiveWindowingConfig) -> Result<Self, LiveWindowingError> {
        validate_config(config)?;
        Ok(Self { config })
    }

    pub fn ready_windows(&self, processed_audio_seconds: f64) -> Vec<LiveWindow> {
        if !processed_audio_seconds.is_finite()
            || processed_audio_seconds < self.config.window_seconds
        {
            return Vec::new();
        }

        let mut windows = Vec::new();
        let mut start_seconds = 0.0;
        while start_seconds + self.config.window_seconds <= processed_audio_seconds {
            windows.push(LiveWindow {
                start_seconds,
                end_seconds: start_seconds + self.config.window_seconds,
            });
            start_seconds += self.config.hop_seconds;
        }
        windows
    }
}

#[derive(Debug, Clone)]
pub struct LiveWindowState {
    config: LiveWindowingConfig,
    next_sequence: u64,
    final_segment_count: u64,
    pending_segments: Vec<PendingLiveSegment>,
    finalized_segments: Vec<FinalizedLiveSegmentKey>,
    failed: bool,
}

impl LiveWindowState {
    pub fn new(config: LiveWindowingConfig) -> Result<Self, LiveWindowingError> {
        validate_config(config)?;

        Ok(Self {
            config,
            next_sequence: 1,
            final_segment_count: 0,
            pending_segments: Vec::new(),
            finalized_segments: Vec::new(),
            failed: false,
        })
    }

    pub fn observe_window(
        &mut self,
        observation: LiveWindowTranscriptObservation,
    ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowingError> {
        let buffer_lag_seconds =
            observation.latest_ingested_audio_seconds - observation.window_end_seconds;
        if buffer_lag_seconds > self.config.max_buffer_lag_seconds {
            self.failed = true;
            return Ok(vec![LiveTranscriptEvent::Error(LiveTranscriptError {
                session_id: observation.session_id,
                sequence: self.next_sequence(),
                message: format!(
                    "processing fell behind live input by {buffer_lag_seconds:.3} seconds"
                ),
                recoverable: false,
            })]);
        }

        let stable_segments = self.mark_stable_segments(&observation);
        let sequence = self.next_sequence();
        let partial_segments = observation
            .segments
            .iter()
            .map(|segment| LivePartialSegment {
                start_seconds: segment.start_seconds,
                end_seconds: segment.end_seconds,
                text: segment.text.clone(),
                language: segment.language.clone(),
            })
            .collect();
        let mut events = vec![LiveTranscriptEvent::Partial(LivePartialTranscript {
            session_id: observation.session_id.clone(),
            sequence,
            window_start_seconds: observation.window_start_seconds,
            window_end_seconds: observation.window_end_seconds,
            window_start_at_utc: observation.window_start_at_utc.clone(),
            window_end_at_utc: observation.window_end_at_utc.clone(),
            text: observation
                .segments
                .iter()
                .map(|segment| segment.text.trim())
                .filter(|text| !text.is_empty())
                .collect::<Vec<_>>()
                .join(" "),
            segments: partial_segments,
        })];

        for stable_segment in stable_segments {
            if stable_segment.end_seconds
                <= observation.window_end_seconds - self.config.finalize_lag_seconds
            {
                events.push(LiveTranscriptEvent::Final(
                    self.finalize_segment(observation.session_id.clone(), stable_segment),
                ));
            }
        }

        self.pending_segments
            .retain(|segment| !(segment.stable && segment.finalized));
        self.add_pending_segments(observation);

        Ok(events)
    }

    pub fn final_segment_count(&self) -> u64 {
        self.final_segment_count
    }

    pub fn has_failed(&self) -> bool {
        self.failed
    }

    fn next_sequence(&mut self) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        sequence
    }

    fn mark_stable_segments(
        &mut self,
        observation: &LiveWindowTranscriptObservation,
    ) -> Vec<PendingLiveSegment> {
        let mut stable_segments = Vec::new();
        let finalized_segments = &self.finalized_segments;
        let stability_tolerance_seconds = self.config.stability_tolerance_seconds;

        for pending in &mut self.pending_segments {
            if pending.finalized
                || segment_matches_finalized(
                    finalized_segments,
                    stability_tolerance_seconds,
                    pending.start_seconds,
                    pending.end_seconds,
                    &pending.normalized_text,
                )
                || !windows_overlap(
                    pending.window_start_seconds,
                    pending.window_end_seconds,
                    observation.window_start_seconds,
                    observation.window_end_seconds,
                )
            {
                continue;
            }

            if observation.segments.iter().any(|candidate| {
                normalized_text(&candidate.text) == pending.normalized_text
                    && seconds_within_tolerance(
                        candidate.start_seconds,
                        pending.start_seconds,
                        self.config.stability_tolerance_seconds,
                    )
                    && seconds_within_tolerance(
                        candidate.end_seconds,
                        pending.end_seconds,
                        self.config.stability_tolerance_seconds,
                    )
            }) {
                pending.stable = true;
                stable_segments.push(pending.clone());
            }
        }

        stable_segments
    }

    fn finalize_segment(
        &mut self,
        session_id: String,
        mut stable_segment: PendingLiveSegment,
    ) -> LiveFinalTranscriptSegment {
        self.final_segment_count += 1;
        stable_segment.finalized = true;
        self.finalized_segments.push(FinalizedLiveSegmentKey {
            start_seconds: stable_segment.start_seconds,
            end_seconds: stable_segment.end_seconds,
            normalized_text: stable_segment.normalized_text.clone(),
        });

        if let Some(pending) = self
            .pending_segments
            .iter_mut()
            .find(|pending| pending.id == stable_segment.id)
        {
            pending.finalized = true;
        }

        LiveFinalTranscriptSegment {
            session_id,
            sequence: self.next_sequence(),
            segment_id: format!("seg-{segment_id:06}", segment_id = self.final_segment_count),
            start_seconds: stable_segment.start_seconds,
            end_seconds: stable_segment.end_seconds,
            start_at_utc: stable_segment.start_at_utc,
            end_at_utc: stable_segment.end_at_utc,
            text: stable_segment.normalized_text,
            language: stable_segment.language,
        }
    }

    fn add_pending_segments(&mut self, observation: LiveWindowTranscriptObservation) {
        for segment in observation.segments {
            let normalized_text = normalized_text(&segment.text);
            if normalized_text.is_empty()
                || self.segment_matches_finalized(
                    segment.start_seconds,
                    segment.end_seconds,
                    &normalized_text,
                )
            {
                continue;
            }

            self.pending_segments.push(PendingLiveSegment {
                id: format!(
                    "{:.3}:{:.3}:{}",
                    segment.start_seconds, segment.end_seconds, normalized_text
                ),
                window_start_seconds: observation.window_start_seconds,
                window_end_seconds: observation.window_end_seconds,
                start_seconds: segment.start_seconds,
                end_seconds: segment.end_seconds,
                start_at_utc: segment.start_at_utc,
                end_at_utc: segment.end_at_utc,
                normalized_text,
                language: segment.language,
                stable: false,
                finalized: false,
            });
        }
    }

    fn segment_matches_finalized(
        &self,
        start_seconds: f64,
        end_seconds: f64,
        normalized_text: &str,
    ) -> bool {
        segment_matches_finalized(
            &self.finalized_segments,
            self.config.stability_tolerance_seconds,
            start_seconds,
            end_seconds,
            normalized_text,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PendingLiveSegment {
    id: String,
    window_start_seconds: f64,
    window_end_seconds: f64,
    start_seconds: f64,
    end_seconds: f64,
    start_at_utc: String,
    end_at_utc: String,
    normalized_text: String,
    language: Option<String>,
    stable: bool,
    finalized: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct FinalizedLiveSegmentKey {
    start_seconds: f64,
    end_seconds: f64,
    normalized_text: String,
}

fn segment_matches_finalized(
    finalized_segments: &[FinalizedLiveSegmentKey],
    tolerance_seconds: f64,
    start_seconds: f64,
    end_seconds: f64,
    normalized_text: &str,
) -> bool {
    finalized_segments.iter().any(|finalized| {
        finalized.normalized_text == normalized_text
            && seconds_within_tolerance(finalized.start_seconds, start_seconds, tolerance_seconds)
            && seconds_within_tolerance(finalized.end_seconds, end_seconds, tolerance_seconds)
    })
}

fn validate_config(config: LiveWindowingConfig) -> Result<(), LiveWindowingError> {
    validate_positive_seconds("window_seconds", config.window_seconds)?;
    validate_positive_seconds("hop_seconds", config.hop_seconds)?;
    validate_positive_seconds("finalize_lag_seconds", config.finalize_lag_seconds)?;
    validate_positive_seconds("max_buffer_lag_seconds", config.max_buffer_lag_seconds)?;
    validate_positive_seconds(
        "stability_tolerance_seconds",
        config.stability_tolerance_seconds,
    )
}

fn validate_positive_seconds(field: &'static str, value: f64) -> Result<(), LiveWindowingError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(LiveWindowingError::InvalidPositiveSeconds { field })
    }
}

fn normalized_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn seconds_within_tolerance(left: f64, right: f64, tolerance: f64) -> bool {
    (left - right).abs() <= tolerance
}

fn windows_overlap(first_start: f64, first_end: f64, second_start: f64, second_end: f64) -> bool {
    first_start < second_end && second_start < first_end
}

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
    fn near_live_window_state_emits_partial_before_any_final() {
        let mut state = LiveWindowState::new(LiveWindowingConfig {
            window_seconds: 5.0,
            hop_seconds: 2.5,
            finalize_lag_seconds: 5.0,
            max_buffer_lag_seconds: 30.0,
            stability_tolerance_seconds: 0.4,
        })
        .expect("valid live config");

        let events = state
            .observe_window(LiveWindowTranscriptObservation {
                session_id: "session-1".to_string(),
                window_start_seconds: 0.0,
                window_end_seconds: 5.0,
                window_start_at_utc: "2026-06-22T15:30:00Z".to_string(),
                window_end_at_utc: "2026-06-22T15:30:05Z".to_string(),
                latest_ingested_audio_seconds: 5.0,
                segments: vec![LiveAsrSegmentCandidate {
                    start_seconds: 0.4,
                    end_seconds: 1.8,
                    start_at_utc: "2026-06-22T15:30:00.400Z".to_string(),
                    end_at_utc: "2026-06-22T15:30:01.800Z".to_string(),
                    text: "hello wor".to_string(),
                    language: Some("en".to_string()),
                }],
            })
            .expect("window is accepted");

        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], LiveTranscriptEvent::Partial(_)));
        assert_eq!(state.final_segment_count(), 0);
    }

    #[test]
    fn overlapping_matching_window_promotes_stable_segment_after_finalize_lag() {
        let mut state = LiveWindowState::new(LiveWindowingConfig::default()).expect("valid config");

        state
            .observe_window(observation(
                "session-1",
                0.0,
                5.0,
                5.0,
                vec![candidate(0.4, 1.8, " hello   world ")],
            ))
            .expect("first window accepted");

        let events = state
            .observe_window(observation(
                "session-1",
                2.5,
                7.5,
                7.5,
                vec![candidate(0.45, 1.75, "hello world")],
            ))
            .expect("second window accepted");

        assert_eq!(events.len(), 2);
        assert!(matches!(events[0], LiveTranscriptEvent::Partial(_)));
        assert_eq!(
            events[1],
            LiveTranscriptEvent::Final(LiveFinalTranscriptSegment {
                session_id: "session-1".to_string(),
                sequence: 3,
                segment_id: "seg-000001".to_string(),
                start_seconds: 0.4,
                end_seconds: 1.8,
                start_at_utc: "2026-06-22T15:30:00.400Z".to_string(),
                end_at_utc: "2026-06-22T15:30:01.800Z".to_string(),
                text: "hello world".to_string(),
                language: Some("en".to_string()),
            })
        );
        assert_eq!(state.final_segment_count(), 1);
    }

    #[test]
    fn rolling_windows_use_configured_window_and_hop_seconds() {
        let planner = LiveWindowPlanner::new(LiveWindowingConfig {
            window_seconds: 4.0,
            hop_seconds: 1.5,
            finalize_lag_seconds: 5.0,
            max_buffer_lag_seconds: 30.0,
            stability_tolerance_seconds: 0.4,
        })
        .expect("valid planner");

        assert_eq!(
            planner.ready_windows(8.2),
            vec![
                LiveWindow {
                    start_seconds: 0.0,
                    end_seconds: 4.0,
                },
                LiveWindow {
                    start_seconds: 1.5,
                    end_seconds: 5.5,
                },
                LiveWindow {
                    start_seconds: 3.0,
                    end_seconds: 7.0,
                },
            ]
        );
    }

    #[test]
    fn finalized_segments_are_not_revised_by_later_windows() {
        let mut state = LiveWindowState::new(LiveWindowingConfig::default()).expect("valid config");

        state
            .observe_window(observation(
                "session-1",
                0.0,
                5.0,
                5.0,
                vec![candidate(0.4, 1.8, "hello world")],
            ))
            .expect("first window accepted");
        let finalizing_events = state
            .observe_window(observation(
                "session-1",
                2.5,
                7.5,
                7.5,
                vec![candidate(0.45, 1.75, "hello world")],
            ))
            .expect("second window accepted");
        let revised_events = state
            .observe_window(observation(
                "session-1",
                5.0,
                10.0,
                10.0,
                vec![candidate(0.42, 1.78, "changed text")],
            ))
            .expect("later window accepted");
        let matching_events_after_final = state
            .observe_window(observation(
                "session-1",
                7.4,
                12.4,
                12.4,
                vec![candidate(0.44, 1.76, "hello world")],
            ))
            .expect("matching later window accepted");

        let final_events = finalizing_events
            .iter()
            .chain(revised_events.iter())
            .chain(matching_events_after_final.iter())
            .filter_map(|event| match event {
                LiveTranscriptEvent::Final(final_segment) => Some(final_segment),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(final_events.len(), 1);
        assert_eq!(final_events[0].text, "hello world");
        assert_eq!(state.final_segment_count(), 1);
    }

    #[test]
    fn lag_failure_emits_error_instead_of_dropping_speech() {
        let mut state = LiveWindowState::new(LiveWindowingConfig {
            window_seconds: 5.0,
            hop_seconds: 2.5,
            finalize_lag_seconds: 5.0,
            max_buffer_lag_seconds: 3.0,
            stability_tolerance_seconds: 0.4,
        })
        .expect("valid config");

        let events = state
            .observe_window(observation(
                "session-1",
                0.0,
                5.0,
                9.1,
                vec![candidate(0.4, 1.8, "hello world")],
            ))
            .expect("lag is reported as an event");

        assert_eq!(
            events,
            vec![LiveTranscriptEvent::Error(LiveTranscriptError {
                session_id: "session-1".to_string(),
                sequence: 1,
                message: "processing fell behind live input by 4.100 seconds".to_string(),
                recoverable: false,
            })]
        );
        assert!(state.has_failed());
    }

    fn observation(
        session_id: &str,
        window_start_seconds: f64,
        window_end_seconds: f64,
        latest_ingested_audio_seconds: f64,
        segments: Vec<LiveAsrSegmentCandidate>,
    ) -> LiveWindowTranscriptObservation {
        LiveWindowTranscriptObservation {
            session_id: session_id.to_string(),
            window_start_seconds,
            window_end_seconds,
            window_start_at_utc: format!("2026-06-22T15:30:{window_start_seconds:02.0}Z"),
            window_end_at_utc: format!("2026-06-22T15:30:{window_end_seconds:02.0}Z"),
            latest_ingested_audio_seconds,
            segments,
        }
    }

    fn candidate(start_seconds: f64, end_seconds: f64, text: &str) -> LiveAsrSegmentCandidate {
        LiveAsrSegmentCandidate {
            start_seconds,
            end_seconds,
            start_at_utc: format!("2026-06-22T15:30:{start_seconds:06.3}Z"),
            end_at_utc: format!("2026-06-22T15:30:{end_seconds:06.3}Z"),
            text: text.to_string(),
            language: Some("en".to_string()),
        }
    }

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

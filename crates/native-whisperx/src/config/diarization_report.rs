//! Product observations from a completed diarization run, distinct from identity certainty.

use std::collections::BTreeSet;

use audio_analysis_transcription::SpeakerDiarizationResponse;
use serde::{Deserialize, Serialize};

/// Speaker-count evidence provided by the completed workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeSpeakerCountStatus {
    /// An unconstrained model estimate without reported count certainty.
    TentativeAutomatic,
    /// A model result produced with user-selected speaker bounds.
    Constrained,
    /// The provider did not report how its count was selected.
    Unspecified,
}

/// Count uncertainty actually exposed by the provider integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeSpeakerCountUncertainty {
    /// No calibrated speaker-count uncertainty was reported. Turn scores are
    /// not count confidence and must not be substituted for it.
    NotReported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeSpeakerCountConstraints {
    pub minimum: Option<usize>,
    pub maximum: Option<usize>,
}

/// A source-timed speaker activity interval. Labels are observations, not a
/// claim that a person's identity has been verified.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeSpeakerTurn {
    pub speaker: String,
    pub start_seconds: f32,
    pub end_seconds: f32,
}

/// Product-owned model/count/turn facts, without embedding vectors or provider DTOs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeDiarizationReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    pub model_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_revision: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_set_sha256: Option<String>,
    /// Distinct labels in the model's actual turn timeline, including zero.
    pub speaker_count: usize,
    pub count_status: NativeSpeakerCountStatus,
    pub count_uncertainty: NativeSpeakerCountUncertainty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count_constraints: Option<NativeSpeakerCountConstraints>,
    /// Raw activity intervals preserve overlap, short turns, and returning labels.
    pub turns: Vec<NativeSpeakerTurn>,
}

impl NativeDiarizationReport {
    pub(crate) fn from_response(response: SpeakerDiarizationResponse) -> Self {
        let facts = &response.diagnostics;
        let minimum = count_bound(facts, "diarizationMinSpeakers");
        let maximum = count_bound(facts, "diarizationMaxSpeakers");
        let (count_status, count_constraints) = match (minimum, maximum) {
            (Some(None), Some(None)) => (NativeSpeakerCountStatus::TentativeAutomatic, None),
            (Some(minimum), Some(maximum)) => (
                NativeSpeakerCountStatus::Constrained,
                Some(NativeSpeakerCountConstraints { minimum, maximum }),
            ),
            _ => (NativeSpeakerCountStatus::Unspecified, None),
        };
        let provider = fact(facts, "diarizationProvider").map(str::to_owned);
        let source_revision = fact(facts, "pyannoteSourceRevision").map(str::to_owned);
        let artifact_set_sha256 = fact(facts, "pyannoteArtifactSetSha256").map(str::to_owned);
        let speaker_count = response
            .segments
            .iter()
            .map(|turn| turn.speaker.as_str())
            .collect::<BTreeSet<_>>()
            .len();
        Self {
            provider,
            model_id: response.model_id,
            source_revision,
            artifact_set_sha256,
            speaker_count,
            count_status,
            count_uncertainty: NativeSpeakerCountUncertainty::NotReported,
            count_constraints,
            turns: response
                .segments
                .into_iter()
                .map(|turn| NativeSpeakerTurn {
                    speaker: turn.speaker,
                    start_seconds: turn.start_seconds,
                    end_seconds: turn.end_seconds,
                })
                .collect(),
        }
    }
}

fn fact<'a>(diagnostics: &'a [String], key: &str) -> Option<&'a str> {
    diagnostics.iter().rev().find_map(|entry| {
        let (name, value) = entry.split_once('=')?;
        (name == key).then_some(value)
    })
}

// Missing selection evidence is different from a reported unbounded count.
fn count_bound(diagnostics: &[String], key: &str) -> Option<Option<usize>> {
    let value = fact(diagnostics, key)?;
    if value == "none" {
        Some(None)
    } else {
        value
            .parse::<usize>()
            .ok()
            .filter(|bound| *bound > 0)
            .map(Some)
    }
}

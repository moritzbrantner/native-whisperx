//! User-facing workflow report types returned after output writing.

use std::{collections::BTreeSet, path::PathBuf};

use audio_analysis_transcription::{SpeakerDiarizationResponse, TranscriptionPipelineResponse};
use media_core::TranscriptionContract;
use serde::{Deserialize, Serialize};

use super::{
    AutomaticWorkflowSelection, AutomaticWorkflowSelectionResource, ConfigSelection,
    ModelResourceSource, OutputFormat,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWhisperxReport {
    /// Canonical transcript produced by the composed workflow.
    pub transcript: TranscriptionContract,
    /// Product-safe execution identity, without runtime option DTOs.
    pub provenance: NativeTranscriptionProvenance,
    /// Structured performance facts extracted from workflow diagnostics.
    #[serde(default, skip_serializing_if = "NativePerformanceReport::is_empty")]
    pub performance: NativePerformanceReport,
    /// Human-readable execution diagnostics emitted by the composed workflow.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<String>,
    /// Product-owned speech-activity intervals used for parity and inspection.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vad_segments: Vec<NativeVadSegment>,
    #[serde(default)]
    pub output_files: Vec<OutputFile>,
    #[serde(
        default,
        skip_serializing_if = "NativeWorkflowSelectionReport::is_empty"
    )]
    pub workflow_selection: NativeWorkflowSelectionReport,
}

impl NativeWhisperxReport {
    pub(crate) fn from_pipeline_response(
        response: TranscriptionPipelineResponse,
        output_files: Vec<OutputFile>,
        workflow_selection: NativeWorkflowSelectionReport,
    ) -> Self {
        let TranscriptionPipelineResponse {
            provider,
            model_id,
            transcript,
            vad_segments,
            diarization,
            mut diagnostics,
            ..
        } = response;
        append_speaker_embedding_validation_diagnostics(
            &mut diagnostics,
            diarization.as_ref(),
            &transcript,
        );
        let performance =
            NativePerformanceReport::from_diagnostics(&diagnostics, vad_segments.len());
        let vad_segments = vad_segments
            .into_iter()
            .map(|segment| NativeVadSegment {
                start_seconds: segment.start_seconds,
                end_seconds: segment.end_seconds,
                score: segment.score,
            })
            .collect();
        Self {
            transcript,
            provenance: NativeTranscriptionProvenance { provider, model_id },
            performance,
            diagnostics,
            output_files,
            workflow_selection,
            vad_segments,
        }
    }
}

fn append_speaker_embedding_validation_diagnostics(
    diagnostics: &mut Vec<String>,
    diarization: Option<&SpeakerDiarizationResponse>,
    transcript: &TranscriptionContract,
) {
    let Some(embeddings) =
        diarization.and_then(|diarization| diarization.speaker_embeddings.as_ref())
    else {
        return;
    };

    let dimensions = embeddings
        .values()
        .map(|embedding| embedding.dimensions())
        .collect::<BTreeSet<_>>();
    let dimension = if dimensions.len() == 1 {
        dimensions.first().copied().unwrap_or_default()
    } else {
        0
    };
    let finite = embeddings
        .values()
        .all(|embedding| embedding.values().iter().all(|value| value.is_finite()));
    let normalized = embeddings.values().all(|embedding| {
        let norm = embedding
            .values()
            .iter()
            .map(|value| value * value)
            .sum::<f32>()
            .sqrt();
        (norm - 1.0).abs() <= 0.001
    });
    let transcript_speakers = transcript
        .segments
        .iter()
        .filter_map(|segment| segment.speaker.clone())
        .collect::<BTreeSet<_>>();
    let embedding_speakers = embeddings.keys().cloned().collect::<BTreeSet<_>>();

    diagnostics.push(format!(
        "diarizationSpeakerEmbeddingCount={}",
        embeddings.len()
    ));
    diagnostics.push(format!("diarizationSpeakerEmbeddingDimension={dimension}"));
    diagnostics.push(format!("diarizationSpeakerEmbeddingsFinite={finite}"));
    diagnostics.push(format!(
        "diarizationSpeakerEmbeddingsNormalized={normalized}"
    ));
    diagnostics.push(format!(
        "diarizationSpeakerEmbeddingClusterAssociation={}",
        transcript_speakers == embedding_speakers
    ));
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeVadSegment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub score: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeTranscriptionProvenance {
    /// Stable execution-provider identifier reported by the workflow.
    pub provider: String,
    /// Requested or resolved transcription model identifier.
    pub model_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativePerformanceReport {
    #[serde(default)]
    pub vad_segment_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_seconds: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_seconds: Option<f64>,
}

impl NativePerformanceReport {
    fn from_diagnostics(diagnostics: &[String], vad_segment_count: usize) -> Self {
        Self {
            vad_segment_count,
            output_seconds: diagnostic_f64(diagnostics, "phaseOutputSeconds"),
            total_seconds: diagnostic_f64(diagnostics, "phaseNativeTotalSeconds"),
        }
    }

    fn is_empty(&self) -> bool {
        self.vad_segment_count == 0 && self.output_seconds.is_none() && self.total_seconds.is_none()
    }
}

fn diagnostic_f64(diagnostics: &[String], key: &str) -> Option<f64> {
    let prefix = format!("{key}=");
    diagnostics
        .iter()
        .find_map(|diagnostic| diagnostic.strip_prefix(&prefix)?.parse().ok())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputFile {
    pub format: OutputFormat,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWorkflowSelectionReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_vad: Option<SelectedVadReport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_diarization_model: Option<SelectedDiarizationModelReport>,
}

impl NativeWorkflowSelectionReport {
    pub fn from_selection(selection: &AutomaticWorkflowSelection) -> Self {
        let selected_vad = selection
            .decisions
            .iter()
            .find(|decision| decision.target == AutomaticWorkflowSelectionResource::Vad)
            .map(|decision| SelectedVadReport {
                method: selection.config.vad.method.as_whisperx_arg().to_string(),
                selection: decision.selection,
                resource_source: decision.source,
            });
        let selected_diarization_model = selection
            .config
            .diarization
            .enabled
            .then(|| {
                selection
                    .decisions
                    .iter()
                    .find(|decision| {
                        decision.target == AutomaticWorkflowSelectionResource::Diarization
                    })
                    .map(|decision| SelectedDiarizationModelReport {
                        model_id: selection.config.diarization.model_id.clone(),
                        selection: decision.selection,
                        resource_source: decision.source,
                    })
            })
            .flatten();

        Self {
            selected_vad,
            selected_diarization_model,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.selected_vad.is_none() && self.selected_diarization_model.is_none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedVadReport {
    pub method: String,
    pub selection: ConfigSelection,
    pub resource_source: ModelResourceSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedDiarizationModelReport {
    pub model_id: String,
    pub selection: ConfigSelection,
    pub resource_source: ModelResourceSource,
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, path::PathBuf};

    use audio_analysis_speakers::{
        AudioRuntime, SpeakerDiarizationResponse, SpeakerEmbedding, SpeakerEmbeddingModel,
        SpeakerEmbeddingModelFamily, SpeakerSegmentPrediction,
    };
    use serde_json::json;

    use super::*;
    use crate::config::{
        AlignmentConfig, AsrConfig, DiarizationConfig, InputSource, NativeWhisperxConfig,
        OutputConfig, TranslationConfig, VadConfig, VadMethod,
    };

    #[test]
    fn automatic_selection_report_summarizes_automatic_choices_without_paths_or_tokens() {
        let secret = "hf_secret_token";
        let selection = AutomaticWorkflowSelection {
            config: NativeWhisperxConfig {
                input: InputSource::Path {
                    path: PathBuf::from("sample.wav"),
                },
                asr: AsrConfig::default(),
                translation: TranslationConfig::default(),
                vad: VadConfig {
                    method: VadMethod::Pyannote,
                    ..VadConfig::default()
                },
                alignment: AlignmentConfig::default(),
                diarization: DiarizationConfig {
                    enabled: true,
                    model_id: "pyannote/speaker-diarization-community-1".to_string(),
                    hf_token: Some(secret.to_string()),
                    ..DiarizationConfig::default()
                },
                output: OutputConfig::default(),
            },
            decisions: vec![
                automatic_decision(
                    AutomaticWorkflowSelectionResource::Vad,
                    Some("pyannote/segmentation-3.0"),
                ),
                automatic_decision(
                    AutomaticWorkflowSelectionResource::Diarization,
                    Some("pyannote/speaker-diarization-community-1"),
                ),
            ],
        };

        let report = NativeWorkflowSelectionReport::from_selection(&selection);

        assert_eq!(
            report.selected_vad,
            Some(SelectedVadReport {
                method: "pyannote".to_string(),
                selection: ConfigSelection::Automatic,
                resource_source: ModelResourceSource::ModelDir,
            })
        );
        assert_eq!(
            report.selected_diarization_model,
            Some(SelectedDiarizationModelReport {
                model_id: "pyannote/speaker-diarization-community-1".to_string(),
                selection: ConfigSelection::Automatic,
                resource_source: ModelResourceSource::ModelDir,
            })
        );
        let json = serde_json::to_string(&report).expect("selection report json");
        assert!(!json.contains("/models"));
        assert!(!json.contains(secret));
    }

    #[test]
    fn automatic_selection_report_summarizes_explicit_choices() {
        let selection = AutomaticWorkflowSelection {
            config: NativeWhisperxConfig {
                input: InputSource::Path {
                    path: PathBuf::from("sample.wav"),
                },
                asr: AsrConfig::default(),
                translation: TranslationConfig::default(),
                vad: VadConfig {
                    selection: ConfigSelection::Explicit,
                    method: VadMethod::Energy,
                    ..VadConfig::default()
                },
                alignment: AlignmentConfig::default(),
                diarization: DiarizationConfig {
                    enabled: true,
                    model_selection: ConfigSelection::Explicit,
                    model_id: "native-spectral-speaker-baseline".to_string(),
                    ..DiarizationConfig::default()
                },
                output: OutputConfig::default(),
            },
            decisions: vec![
                explicit_decision(AutomaticWorkflowSelectionResource::Vad, Some("energy")),
                explicit_decision(
                    AutomaticWorkflowSelectionResource::Diarization,
                    Some("native-spectral-speaker-baseline"),
                ),
            ],
        };

        let report = NativeWorkflowSelectionReport::from_selection(&selection);

        assert_eq!(
            report.selected_vad,
            Some(SelectedVadReport {
                method: "energy".to_string(),
                selection: ConfigSelection::Explicit,
                resource_source: ModelResourceSource::ExplicitConfig,
            })
        );
        assert_eq!(
            report.selected_diarization_model,
            Some(SelectedDiarizationModelReport {
                model_id: "native-spectral-speaker-baseline".to_string(),
                selection: ConfigSelection::Explicit,
                resource_source: ModelResourceSource::ExplicitConfig,
            })
        );
    }

    #[test]
    fn report_serialization_uses_product_owned_provenance_and_performance() {
        let response: TranscriptionPipelineResponse = serde_json::from_value(json!({
            "accepted": true,
            "operation": "transcribe",
            "provider": "candle-whisper",
            "modelId": "openai/whisper-small",
            "transcript": {
                "text": "hello",
                "language": "en",
                "segments": [],
                "attributes": {}
            },
            "vadSegments": [{
                "startSeconds": 0.0,
                "endSeconds": 1.0,
                "score": 0.9
            }],
            "alignment": null,
            "diarization": null,
            "artifacts": [],
            "diagnostics": [
                "phaseOutputSeconds=0.125",
                "phaseNativeTotalSeconds=1.500"
            ]
        }))
        .expect("pipeline response fixture");
        let report = NativeWhisperxReport::from_pipeline_response(
            response,
            Vec::new(),
            NativeWorkflowSelectionReport::default(),
        );

        let json = serde_json::to_value(report).expect("product report json");

        assert!(json.get("response").is_none());
        assert!(json.get("accepted").is_none());
        assert!(json.get("operation").is_none());
        assert!(json.get("artifacts").is_none());
        assert_eq!(json["transcript"]["text"], "hello");
        assert_eq!(json["provenance"]["provider"], "candle-whisper");
        assert_eq!(json["provenance"]["modelId"], "openai/whisper-small");
        assert_eq!(json["performance"]["vadSegmentCount"], 1);
        assert_eq!(json["performance"]["outputSeconds"], 0.125);
        assert_eq!(json["performance"]["totalSeconds"], 1.5);
        assert_eq!(json["vadSegments"][0]["endSeconds"], 1.0);
    }

    #[test]
    fn report_validates_pyannote_embedding_shape_without_serializing_vectors() {
        let mut transcript = crate::import_whisperx_json(include_bytes!(
            "../../../../tests/fixtures/whisperx-parity-sample.json"
        ))
        .expect("fixture should import");
        transcript.segments[0].speaker = Some("SPEAKER_00".to_string());
        transcript.segments[1].speaker = Some("SPEAKER_01".to_string());
        let model = SpeakerEmbeddingModel::new(
            SpeakerEmbeddingModelFamily::Pyannote,
            "pyannote/speaker-diarization-community-1",
            "1",
            2,
        )
        .expect("model");
        let embedding =
            |values| SpeakerEmbedding::new(values, model.clone(), 16_000).expect("embedding");
        let response = TranscriptionPipelineResponse {
            accepted: true,
            operation: "transcribe".to_string(),
            provider: "native-speaker-diarization".to_string(),
            model_id: "tiny.en".to_string(),
            transcript,
            vad_segments: Vec::new(),
            alignment: None,
            diarization: Some(SpeakerDiarizationResponse {
                accepted: true,
                operation: "audio.speakers.diarize".to_string(),
                model_id: "pyannote/speaker-diarization-community-1".to_string(),
                runtime: AudioRuntime::Onnx,
                segments: vec![
                    SpeakerSegmentPrediction {
                        speaker: "SPEAKER_00".to_string(),
                        start_seconds: 0.0,
                        end_seconds: 1.0,
                        score: Some(1.0),
                    },
                    SpeakerSegmentPrediction {
                        speaker: "SPEAKER_01".to_string(),
                        start_seconds: 1.0,
                        end_seconds: 2.0,
                        score: Some(1.0),
                    },
                ],
                speaker_embeddings: Some(BTreeMap::from([
                    ("SPEAKER_00".to_string(), embedding(vec![1.0, 0.0])),
                    ("SPEAKER_01".to_string(), embedding(vec![0.0, 1.0])),
                ])),
                diagnostics: Vec::new(),
            }),
            artifacts: Vec::new(),
            diagnostics: Vec::new(),
        };

        let report = NativeWhisperxReport::from_pipeline_response(
            response,
            Vec::new(),
            NativeWorkflowSelectionReport::default(),
        );

        for expected in [
            "diarizationSpeakerEmbeddingCount=2",
            "diarizationSpeakerEmbeddingDimension=2",
            "diarizationSpeakerEmbeddingsFinite=true",
            "diarizationSpeakerEmbeddingsNormalized=true",
            "diarizationSpeakerEmbeddingClusterAssociation=true",
        ] {
            assert!(report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic == expected));
        }
        let json = serde_json::to_string(&report).expect("report json");
        assert!(!json.contains("values"));
    }

    fn automatic_decision(
        target: AutomaticWorkflowSelectionResource,
        model_id: Option<&str>,
    ) -> super::super::AutomaticWorkflowSelectionDecision {
        super::super::AutomaticWorkflowSelectionDecision {
            target,
            selection: ConfigSelection::Automatic,
            model_id: model_id.map(str::to_string),
            source: ModelResourceSource::ModelDir,
            path: Some(PathBuf::from("/models/secret")),
        }
    }

    fn explicit_decision(
        target: AutomaticWorkflowSelectionResource,
        model_id: Option<&str>,
    ) -> super::super::AutomaticWorkflowSelectionDecision {
        super::super::AutomaticWorkflowSelectionDecision {
            target,
            selection: ConfigSelection::Explicit,
            model_id: model_id.map(str::to_string),
            source: ModelResourceSource::ExplicitConfig,
            path: Some(PathBuf::from("/models/secret")),
        }
    }
}

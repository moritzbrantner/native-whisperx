//! User-facing workflow report types returned after output writing.

use std::{collections::BTreeSet, path::PathBuf};

use audio_analysis_transcription::{SpeakerDiarizationResponse, TranscriptionPipelineResponse};
use media_core::TranscriptionContract;
use serde::{Deserialize, Serialize};

use super::{
    AutomaticWorkflowSelection, AutomaticWorkflowSelectionResource, ConfigSelection,
    ModelResourceSource, NativeDiarizationReport, OutputFormat,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWhisperxReport {
    /// Canonical transcript produced by the composed workflow.
    pub transcript: TranscriptionContract,
    /// Product-safe execution identity, without runtime option DTOs.
    pub provenance: NativeTranscriptionProvenance,
    /// Observed count and raw speaker turns from a completed diarization model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diarization: Option<NativeDiarizationReport>,
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
        let diarization = diarization.map(NativeDiarizationReport::from_response);
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
            provenance: NativeTranscriptionProvenance::from_diagnostics(
                provider,
                model_id,
                &diagnostics,
            ),
            diarization,
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
    /// Device the ASR model ran on, such as `cuda:0` or `cpu`, when the
    /// provider reports it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Driver-reported name of the CUDA device the ASR model ran on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// Why `--device auto` fell back to the CPU for ASR (the CUDA probe error).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_fallback_reason: Option<String>,
    /// Resolved ASR compute type, such as `fp16`, `fp32` or `int8`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<String>,
    /// Device VAD ran on; absent when VAD did not run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vad_device: Option<String>,
    /// Device forced alignment ran on; absent when alignment did not run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment_device: Option<String>,
    /// Why `--device auto` fell back to the CPU for alignment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment_device_fallback_reason: Option<String>,
    /// Device diarization ran on; absent when diarization did not run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diarization_device: Option<String>,
}

impl NativeTranscriptionProvenance {
    fn from_diagnostics(provider: String, model_id: String, diagnostics: &[String]) -> Self {
        let fact = |key: &str| diagnostic_str(diagnostics, key).map(str::to_string);
        Self {
            provider,
            model_id,
            device: fact("device"),
            device_name: fact("deviceName"),
            device_fallback_reason: fact("deviceFallbackReason"),
            compute_type: fact("resolvedComputeType"),
            vad_device: fact("vadDevice"),
            alignment_device: fact("alignmentDevice"),
            alignment_device_fallback_reason: fact("alignmentDeviceFallbackReason"),
            diarization_device: fact("diarizationDevice"),
        }
    }
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
    diagnostic_str(diagnostics, key)?.parse().ok()
}

fn diagnostic_str<'a>(diagnostics: &'a [String], key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    diagnostics
        .iter()
        .find_map(|diagnostic| diagnostic.strip_prefix(&prefix))
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
    fn report_keeps_tentative_automatic_count_and_anonymous_overlapping_turns() {
        let response = speaker_observation_response();
        let report = NativeWhisperxReport::from_pipeline_response(
            response,
            Vec::new(),
            NativeWorkflowSelectionReport::default(),
        );
        let value = serde_json::to_value(report).unwrap();
        let observed = &value["diarization"];
        assert_eq!(observed["speakerCount"], 2);
        assert_eq!(observed["countStatus"], "tentativeAutomatic");
        assert_eq!(observed["countUncertainty"], "notReported");
        assert_eq!(observed["provider"], "pyannote");
        assert_eq!(
            observed["modelId"],
            "pyannote/speaker-diarization-community-1"
        );
        assert_eq!(observed["sourceRevision"], "fixture-revision");
        assert_eq!(observed["artifactSetSha256"], "fixture-artifacts");
        assert_eq!(observed["turns"].as_array().unwrap().len(), 3);
        assert_eq!(
            observed["turns"][1]["startSeconds"].as_f64(),
            Some(f64::from(0.8_f32))
        );
        assert_eq!(observed["turns"][2]["speaker"], "SPEAKER_00");
        assert!(observed["turns"][0].get("score").is_none());
        assert!(observed.get("confidence").is_none());
    }

    fn speaker_observation_response() -> TranscriptionPipelineResponse {
        serde_json::from_value(json!({
            "accepted": true, "operation": "transcribe", "provider": "candle-whisper", "modelId": "small",
            "transcript": { "language": "en", "segments": [] },
            "vadSegments": [], "alignment": null, "artifacts": [], "diagnostics": [],
            "diarization": {
                "accepted": true, "operation": "audio.speakers.diarize",
                "modelId": "pyannote/speaker-diarization-community-1", "runtime": "onnx",
                "segments": [
                    { "speaker": "SPEAKER_00", "startSeconds": 0.0, "endSeconds": 1.0, "score": 1.0 },
                    { "speaker": "SPEAKER_01", "startSeconds": 0.8, "endSeconds": 1.5, "score": 1.0 },
                    { "speaker": "SPEAKER_00", "startSeconds": 5.0, "endSeconds": 6.0, "score": 1.0 }
                ],
                "diagnostics": ["diarizationProvider=pyannote", "diarizationMinSpeakers=none",
                    "diarizationMaxSpeakers=none", "pyannoteSourceRevision=fixture-revision",
                    "pyannoteArtifactSetSha256=fixture-artifacts"]
            }
        })).expect("available model response with uncalibrated count")
    }

    #[test]
    fn constrained_count_is_not_presented_as_automatic_count_evidence() {
        let mut response = speaker_observation_response();
        let facts = &mut response.diarization.as_mut().unwrap().diagnostics;
        facts.push("diarizationMinSpeakers=2".into());
        facts.push("diarizationMaxSpeakers=3".into());
        let report = NativeWhisperxReport::from_pipeline_response(
            response,
            Vec::new(),
            NativeWorkflowSelectionReport::default(),
        );
        let value = serde_json::to_value(report).unwrap();
        assert_eq!(value["diarization"]["countStatus"], "constrained");
        assert_eq!(
            value["diarization"]["countConstraints"],
            json!({"minimum": 2, "maximum": 3})
        );
        assert_eq!(value["diarization"]["countUncertainty"], "notReported");
    }

    #[test]
    fn absent_count_selection_facts_remain_unspecified() {
        let mut response = speaker_observation_response();
        response.diarization.as_mut().unwrap().diagnostics.clear();
        let report = NativeWhisperxReport::from_pipeline_response(
            response,
            Vec::new(),
            NativeWorkflowSelectionReport::default(),
        );
        let value = serde_json::to_value(report).unwrap();
        assert_eq!(value["diarization"]["speakerCount"], 2);
        assert_eq!(value["diarization"]["countStatus"], "unspecified");
        assert!(value["diarization"].get("countConstraints").is_none());
    }

    #[test]
    fn completed_model_with_no_turns_retains_zero_instead_of_dropping_the_observation() {
        let mut response = speaker_observation_response();
        response.diarization.as_mut().unwrap().segments.clear();
        let report = NativeWhisperxReport::from_pipeline_response(
            response,
            Vec::new(),
            NativeWorkflowSelectionReport::default(),
        );
        let value = serde_json::to_value(report).unwrap();
        assert_eq!(value["diarization"]["speakerCount"], 0);
        assert_eq!(value["diarization"]["turns"], json!([]));
        assert_eq!(value["diarization"]["countStatus"], "tentativeAutomatic");
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

        assert!(json.get("diarization").is_none());
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
        assert!(json["provenance"].get("device").is_none());
        assert!(json["provenance"].get("vadDevice").is_none());
    }

    fn provenance_for(diagnostics: &[&str]) -> serde_json::Value {
        let response: TranscriptionPipelineResponse = serde_json::from_value(json!({
            "accepted": true,
            "operation": "transcribe",
            "provider": "candle-whisper",
            "modelId": "openai/whisper-small",
            "transcript": {"text": "hello", "language": "en", "segments": [], "attributes": {}},
            "vadSegments": [],
            "alignment": null,
            "diarization": null,
            "artifacts": [],
            "diagnostics": diagnostics,
        }))
        .expect("pipeline response fixture");
        let report = NativeWhisperxReport::from_pipeline_response(
            response,
            Vec::new(),
            NativeWorkflowSelectionReport::default(),
        );
        serde_json::to_value(report).expect("product report json")["provenance"].clone()
    }

    #[test]
    fn provenance_records_the_cuda_device_of_each_stage() {
        let provenance = provenance_for(&[
            "resolvedComputeType=fp16",
            "provider=candle-whisper",
            "device=cuda:0",
            "deviceRequested=auto",
            "deviceName=NVIDIA GeForce RTX 3060 Ti",
            "vadDevice=cpu",
            "alignmentDevice=cuda:0",
            "alignmentDeviceRequested=auto",
            "alignmentDeviceName=NVIDIA GeForce RTX 3060 Ti",
            "diarizationDevice=cpu",
        ]);

        assert_eq!(
            provenance,
            json!({
                "provider": "candle-whisper",
                "modelId": "openai/whisper-small",
                "device": "cuda:0",
                "deviceName": "NVIDIA GeForce RTX 3060 Ti",
                "computeType": "fp16",
                "vadDevice": "cpu",
                "alignmentDevice": "cuda:0",
                "diarizationDevice": "cpu",
            })
        );
    }

    #[test]
    fn provenance_records_why_auto_fell_back_to_the_cpu() {
        let reason =
            "CUDA device cuda:0 cannot be used because the binary lacks the `cuda` feature";
        let provenance = provenance_for(&[
            "resolvedComputeType=fp32",
            "device=cpu",
            "deviceRequested=auto",
            &format!("deviceFallbackReason={reason}"),
            "alignmentDevice=cpu",
            &format!("alignmentDeviceFallbackReason={reason}"),
        ]);

        assert_eq!(provenance["device"], "cpu");
        assert_eq!(provenance["deviceFallbackReason"], reason);
        assert_eq!(provenance["alignmentDevice"], "cpu");
        assert_eq!(provenance["alignmentDeviceFallbackReason"], reason);
        assert_eq!(provenance["computeType"], "fp32");
        assert!(provenance.get("deviceName").is_none());
        assert!(provenance.get("vadDevice").is_none());
        assert!(provenance.get("diarizationDevice").is_none());
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

//! Diagnostic enrichment for native alignment and diarization workflow reports.

use audio_analysis_transcription::TranscriptionPipelineResponse;
use text_transcripts::TranscriptionContract;

use crate::config::{
    is_pyannote_diarization_model, AsrProvider, AutomaticWorkflowSelection,
    AutomaticWorkflowSelectionResource, ConfigSelection, ModelResourceSource, NativeWhisperxConfig,
};

pub(crate) fn append_automatic_workflow_selection_diagnostics(
    response: &mut TranscriptionPipelineResponse,
    selection: &AutomaticWorkflowSelection,
) {
    for decision in &selection.decisions {
        let prefix = match decision.target {
            AutomaticWorkflowSelectionResource::Vad => "automaticWorkflowSelectionVad",
            AutomaticWorkflowSelectionResource::Diarization => {
                "automaticWorkflowSelectionDiarization"
            }
        };
        push_diagnostic_if_missing(
            &mut response.diagnostics,
            &format!("{prefix}Mode"),
            format!(
                "{prefix}Mode={}",
                match decision.selection {
                    ConfigSelection::Automatic => "automatic",
                    ConfigSelection::Explicit => "explicit",
                }
            ),
        );
        if decision.target == AutomaticWorkflowSelectionResource::Vad {
            push_diagnostic_if_missing(
                &mut response.diagnostics,
                &format!("{prefix}Method"),
                format!(
                    "{prefix}Method={}",
                    selection.config.vad.method.as_whisperx_arg()
                ),
            );
        }
        if let Some(model_id) = &decision.model_id {
            push_diagnostic_if_missing(
                &mut response.diagnostics,
                &format!("{prefix}ModelId"),
                format!("{prefix}ModelId={model_id}"),
            );
        }
        push_diagnostic_if_missing(
            &mut response.diagnostics,
            &format!("{prefix}ResourceSource"),
            format!(
                "{prefix}ResourceSource={}",
                match decision.source {
                    ModelResourceSource::ExistingEnergyVad => "existing-energy-vad",
                    ModelResourceSource::ExplicitConfig => "explicit-config",
                    ModelResourceSource::ModelDir => "model-dir",
                    ModelResourceSource::HuggingFaceCache => "hugging-face-cache",
                    ModelResourceSource::Unresolved => "unresolved",
                    ModelResourceSource::HuggingFaceDownload => "hugging-face-download",
                }
            ),
        );
    }
}

pub(crate) fn append_native_diarization_diagnostics(
    response: &mut TranscriptionPipelineResponse,
    config: &NativeWhisperxConfig,
) {
    if config.asr.provider != AsrProvider::Native
        || !config.diarization.enabled
        || !is_pyannote_diarization_model(&config.diarization.model_id)
    {
        return;
    }

    for diagnostic in [
        "diarizationPhase=segmentation",
        "diarizationPhase=embedding",
        "diarizationPhase=plda",
        "diarizationPhase=vbx",
        "diarizationPhase=clustering",
    ] {
        if !response
            .diagnostics
            .iter()
            .any(|existing| existing == diagnostic)
        {
            response.diagnostics.push(diagnostic.to_string());
        }
    }
}

pub(crate) fn append_native_alignment_diagnostics(
    response: &mut TranscriptionPipelineResponse,
    config: &NativeWhisperxConfig,
) {
    if config.asr.provider != AsrProvider::Native || !config.alignment.enabled {
        return;
    }
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentModelId",
        format!(
            "alignmentModelId={}",
            canonical_alignment_model_id(&config.alignment.model_id)
        ),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentFallbackCount",
        "alignmentFallbackCount=0".to_string(),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentRetryCount",
        "alignmentRetryCount=0".to_string(),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentWordTimingMissingCount",
        format!(
            "alignmentWordTimingMissingCount={}",
            alignment_word_timing_missing_count(&response.transcript)
        ),
    );
    push_diagnostic_if_missing(
        &mut response.diagnostics,
        "alignmentCharTimingMissingCount",
        format!(
            "alignmentCharTimingMissingCount={}",
            if config.alignment.return_char_alignments {
                alignment_char_timing_missing_count(&response.transcript)
            } else {
                0
            }
        ),
    );
}

fn canonical_alignment_model_id(model_id: &str) -> &str {
    if model_id.eq_ignore_ascii_case("WAV2VEC2_ASR_BASE_960H") {
        "facebook/wav2vec2-base-960h"
    } else {
        model_id
    }
}

fn push_diagnostic_if_missing(diagnostics: &mut Vec<String>, key: &str, diagnostic: String) {
    let prefix = format!("{key}=");
    if diagnostics
        .iter()
        .any(|existing| existing.starts_with(&prefix))
    {
        return;
    }
    diagnostics.push(diagnostic);
}

fn alignment_word_timing_missing_count(transcript: &TranscriptionContract) -> usize {
    transcript
        .segments
        .iter()
        .flat_map(|segment| segment.words.iter())
        .filter(|word| word.start_seconds.zip(word.end_seconds).is_none())
        .count()
}

fn alignment_char_timing_missing_count(transcript: &TranscriptionContract) -> usize {
    transcript
        .segments
        .iter()
        .flat_map(|segment| segment.chars.iter())
        .filter(|character| character.start_seconds.zip(character.end_seconds).is_none())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use audio_analysis_transcription::TranscriptionPipelineResponse;

    use crate::config::{
        AlignmentConfig, AsrConfig, AutomaticWorkflowSelection, AutomaticWorkflowSelectionDecision,
        AutomaticWorkflowSelectionResource, ConfigSelection, DiarizationConfig, InputSource,
        ModelResourceSource, NativeWhisperxConfig, OutputConfig, TranslationConfig, VadConfig,
        VadMethod,
    };
    use crate::import_whisperx_json;

    const WHISPERX_SAMPLE: &[u8] =
        include_bytes!("../../../tests/fixtures/whisperx-parity-sample.json");

    #[test]
    fn automatic_workflow_selection_diagnostics_report_mode_model_and_source() {
        let mut response = fixture_response_with_chars();
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
                diarization: DiarizationConfig::default(),
                output: OutputConfig::default(),
            },
            decisions: vec![
                AutomaticWorkflowSelectionDecision {
                    target: AutomaticWorkflowSelectionResource::Vad,
                    selection: ConfigSelection::Automatic,
                    model_id: Some("pyannote/segmentation-3.0".to_string()),
                    source: ModelResourceSource::ModelDir,
                    path: None,
                },
                AutomaticWorkflowSelectionDecision {
                    target: AutomaticWorkflowSelectionResource::Diarization,
                    selection: ConfigSelection::Automatic,
                    model_id: Some("pyannote/speaker-diarization-community-1".to_string()),
                    source: ModelResourceSource::ModelDir,
                    path: Some(PathBuf::from("/models/pyannote")),
                },
            ],
        };

        append_automatic_workflow_selection_diagnostics(&mut response, &selection);

        assert!(response
            .diagnostics
            .contains(&"automaticWorkflowSelectionDiarizationMode=automatic".to_string()));
        assert!(response
            .diagnostics
            .contains(&"automaticWorkflowSelectionVadMethod=pyannote".to_string()));
        assert!(response.diagnostics.contains(
            &"automaticWorkflowSelectionVadModelId=pyannote/segmentation-3.0".to_string()
        ));
        assert!(response
            .diagnostics
            .contains(&"automaticWorkflowSelectionVadResourceSource=model-dir".to_string()));
        assert!(response.diagnostics.contains(
            &"automaticWorkflowSelectionDiarizationModelId=pyannote/speaker-diarization-community-1"
                .to_string()
        ));
        assert!(response.diagnostics.contains(
            &"automaticWorkflowSelectionDiarizationResourceSource=model-dir".to_string()
        ));
        assert!(!response
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.contains("/models/pyannote")));
    }

    #[test]
    fn native_pyannote_diarization_diagnostics_identify_phases() {
        let mut response = fixture_response_with_chars();
        let config = NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig {
                enabled: true,
                model_id: "pyannote/speaker-diarization-community-1".to_string(),
                model_bundle: Some(PathBuf::from("/models/pyannote-diarization")),
                ..DiarizationConfig::default()
            },
            output: OutputConfig::default(),
        };

        append_native_diarization_diagnostics(&mut response, &config);

        for expected in [
            "diarizationPhase=segmentation",
            "diarizationPhase=embedding",
            "diarizationPhase=plda",
            "diarizationPhase=vbx",
            "diarizationPhase=clustering",
        ] {
            assert!(
                response
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic == expected),
                "missing {expected}: {:?}",
                response.diagnostics
            );
        }
    }

    #[test]
    fn native_alignment_diagnostics_include_fallback_and_retry_counts() {
        let mut response = fixture_response_with_chars();

        append_native_alignment_diagnostics(
            &mut response,
            &NativeWhisperxConfig {
                input: InputSource::Path {
                    path: PathBuf::from("sample.wav"),
                },
                asr: AsrConfig::default(),
                translation: TranslationConfig::default(),
                vad: VadConfig::default(),
                alignment: AlignmentConfig {
                    enabled: true,
                    return_char_alignments: true,
                    ..AlignmentConfig::default()
                },
                diarization: DiarizationConfig::default(),
                output: OutputConfig::default(),
            },
        );

        for expected in [
            "alignmentFallbackCount=0",
            "alignmentRetryCount=0",
            "alignmentWordTimingMissingCount=0",
            "alignmentCharTimingMissingCount=0",
        ] {
            assert!(
                response
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic == expected),
                "diagnostics should include `{expected}`: {:?}",
                response.diagnostics
            );
        }
    }

    fn fixture_response_with_chars() -> TranscriptionPipelineResponse {
        let mut transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        transcript.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });
        TranscriptionPipelineResponse {
            accepted: true,
            operation: "audio.transcription.transcribe".to_string(),
            provider: "fixture".to_string(),
            model_id: "fixture".to_string(),
            transcript,
            vad_segments: Vec::new(),
            alignment: None,
            diarization: None,
            artifacts: Vec::new(),
            diagnostics: Vec::new(),
        }
    }
}

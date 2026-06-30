//! Library-owned runtime surface for `audio-analysis-transcription`.

use runtime_core::{
    structured_surface_response, MobileCapability, OperationId, PackageSurface,
    RuntimeCapabilities, SurfaceOperation, SurfaceRequest, SurfaceResponse,
};
use serde::Deserialize;

use crate::{
    candle_whisper_provider_plan, import_whisperx_json, setup_error, transcribe,
    transcription_provider_plans, whisper_cpp_provider_plan, whisperx_provider_plan,
    AlignmentOptions, CandleWhisperDecodeRuntime, CandleWhisperOptions, DiarizationOptions,
    NativeDevicePreference, TranscriptionPipelineRequest, TranscriptionProviderSelection,
    TranscriptionSource, TranscriptionTask, VadOptions, WhisperXCommandOptions, WhisperXDevice,
};

/// Returns the package surface exposed by every transport wrapper.
pub fn package_surface() -> PackageSurface {
    PackageSurface {
        library: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        capabilities: RuntimeCapabilities {
            native: true,
            server: true,
            wasm: false,
            mobile: MobileCapability::Unsupported,
            requirements: vec![
                runtime_core::RuntimeRequirement {
                    name: "candle-whisper-model-bundle".to_string(),
                    description: Some(
                        "Required for native Candle Whisper ASR execution.".to_string(),
                    ),
                    required: false,
                },
                runtime_core::RuntimeRequirement {
                    name: "cuda".to_string(),
                    description: Some(
                        "Optional optimized device path when built with the cuda feature."
                            .to_string(),
                    ),
                    required: false,
                },
                runtime_core::RuntimeRequirement {
                    name: "whisperx".to_string(),
                    description: Some(
                        "Optional Python command for explicit external compatibility runs."
                            .to_string(),
                    ),
                    required: false,
                },
            ],
            max_recommended_input_bytes: None,
        },
        operations: vec![
            operation(
                "describe",
                "Describe package",
                "Rust-native audio and video transcription orchestration for video-analysis.",
                serde_json::json!({"includeOperations": true}),
                true,
            ),
            operation(
                "audio.transcription.transcribe",
                "Transcribe audio or video",
                "Runs transcription or Whisper translate-to-English through native providers when built with explicit features and local bundles; WhisperX remains an external compatibility path.",
                serde_json::json!({
                    "source": {"path": "speech-de.wav"},
                    "provider": {
                        "kind": "candleWhisper",
                        "modelId": "openai/whisper-large-v3-turbo",
                        "task": "translate",
                        "device": "cuda"
                    },
                    "vad": {"enabled": true},
                    "alignment": {"enabled": false},
                    "diarization": {"enabled": false},
                    "output": {"formats": ["json", "srt", "webvtt"]}
                }),
                false,
            ),
            operation(
                "audio.transcription.importWhisperX",
                "Import WhisperX JSON",
                "Parses existing WhisperX JSON output through text-transcripts without running models.",
                serde_json::json!({"content": "{\"segments\":[{\"start\":0.0,\"end\":1.0,\"text\":\"Hello.\"}]}"}),
                true,
            ),
            operation(
                "audio.transcription.providers",
                "Inspect transcription providers",
                "Lists native and compatibility transcription providers and runtime constraints.",
                serde_json::json!({"includeExternal": true}),
                true,
            ),
            operation(
                "audio.transcription.plan",
                "Plan transcription runtime",
                "Explains the native transcription runtime without running models.",
                serde_json::json!({"provider": {"kind": "candleWhisper", "modelId": "openai/whisper-large-v3-turbo"}}),
                true,
            ),
            operation(
                "audio.transcription.modelPlan",
                "Plan ASR model",
                "Explains Candle Whisper and compatibility model requirements.",
                serde_json::json!({"provider": "candle-whisper"}),
                true,
            ),
            operation(
                "audio.transcription.vadPlan",
                "Plan VAD",
                "Explains deterministic energy VAD chunking defaults.",
                serde_json::json!({"vad": {"enabled": true}}),
                true,
            ),
            operation(
                "audio.transcription.alignmentPlan",
                "Plan alignment",
                "Explains deterministic CTC alignment and opt-in wav2vec2 bundle requirements.",
                serde_json::json!({"alignment": {"enabled": true, "modelId": "facebook/wav2vec2-base-960h"}}),
                true,
            ),
            operation(
                "audio.transcription.alignmentBundlePlan",
                "Plan alignment bundle",
                "Inspects local wav2vec2 bundle compatibility without executing model inference.",
                serde_json::json!({}),
                true,
            ),
            operation(
                "audio.transcription.decodePlan",
                "Plan audio decode",
                "Explains whether a transcription source uses direct samples, native WAV loading, opt-in audio-io media decode, or external WhisperX compatibility.",
                serde_json::json!({"source": {"path": "clip.mp4"}, "provider": {"kind": "candleWhisper"}}),
                true,
            ),
            operation(
                "audio.transcription.diarizationPlan",
                "Plan diarization",
                "Explains current heuristic native diarization status and future model-backed provider options.",
                serde_json::json!({"diarization": {"enabled": true, "assignmentPolicy": "majority"}}),
                true,
            ),
        ],
    }
}

fn operation(
    id: &str,
    name: &str,
    description: &str,
    example_request: serde_json::Value,
    wasm_supported: bool,
) -> SurfaceOperation {
    let mut operation = SurfaceOperation {
        id: OperationId::new(id),
        name: name.to_string(),
        description: Some(description.to_string()),
        curation: runtime_core::SurfaceOperationCuration::from_operation_id(id),
        input_schema: serde_json::json!({"type": "object", "additionalProperties": true, "xOperationCategory": runtime_core::operation_category(id)}),
        output_schema: serde_json::json!({"type": "object", "xOperationCategory": runtime_core::operation_category(id)}),
        example_request,
        wasm_supported,
        server_supported: true,
    };
    if let Some(contract) = landscape_contract(id) {
        runtime_core::attach_landscape_contract(&mut operation, contract);
    }
    operation
}

fn landscape_contract(id: &str) -> Option<runtime_core::landscape::LandscapeOperationContract> {
    match id {
        "audio.transcription.transcribe" => {
            Some(runtime_core::landscape::LandscapeOperationContract::new(
                runtime_core::landscape::LandscapeFunction::new(
                    "audio.transcription.transcribe",
                    env!("CARGO_PKG_NAME"),
                )
                .input(runtime_core::landscape::LandscapePort::new(
                    "source",
                    runtime_core::landscape::well_known::audio_source(),
                ))
                .input(runtime_core::landscape::LandscapePort::new(
                    "config",
                    runtime_core::landscape::well_known::audio_transcription_config(),
                ))
                .output(
                    runtime_core::landscape::LandscapePort::new(
                        "segments",
                        runtime_core::landscape::well_known::text_transcript_segment(),
                    )
                    .many(),
                )
                .output(runtime_core::landscape::LandscapePort::new(
                    "document",
                    runtime_core::landscape::well_known::text_document(),
                ))
                .stability(runtime_core::landscape::LandscapeStability::Experimental),
            ))
        }
        "audio.transcription.importWhisperX" => {
            Some(runtime_core::landscape::LandscapeOperationContract::new(
                runtime_core::landscape::LandscapeFunction::new(
                    "audio.transcription.importWhisperX",
                    env!("CARGO_PKG_NAME"),
                )
                .input(runtime_core::landscape::LandscapePort::new(
                    "source",
                    runtime_core::landscape::well_known::audio_source(),
                ))
                .output(
                    runtime_core::landscape::LandscapePort::new(
                        "segments",
                        runtime_core::landscape::well_known::text_transcript_segment(),
                    )
                    .many(),
                )
                .output(runtime_core::landscape::LandscapePort::new(
                    "document",
                    runtime_core::landscape::well_known::text_document(),
                )),
            ))
        }
        _ => None,
    }
}

/// Runs one library-owned operation.
pub fn run_surface_operation(request: SurfaceRequest) -> Result<SurfaceResponse, String> {
    let operation = request.operation.clone();
    let value = match request.operation.as_str() {
        "describe" => describe_value(request.input),
        "audio.transcription.transcribe" => transcribe_value(request.input)?,
        "audio.transcription.importWhisperX" => import_whisperx_value(parse_input(request.input)?)?,
        "audio.transcription.providers" => providers_value(request.input),
        "audio.transcription.plan" => plan_value(request.input),
        "audio.transcription.modelPlan" => model_plan_value(request.input),
        "audio.transcription.vadPlan" => vad_plan_value(request.input),
        "audio.transcription.alignmentPlan" => alignment_plan_value(request.input),
        "audio.transcription.alignmentBundlePlan" => alignment_bundle_plan_value(request.input)?,
        "audio.transcription.decodePlan" => decode_plan_value(request.input),
        "audio.transcription.diarizationPlan" => diarization_plan_value(request.input),
        operation => {
            return Err(runtime_core::SurfaceError::unsupported_operation(
                operation,
                env!("CARGO_PKG_NAME"),
            )
            .to_error_string())
        }
    };
    Ok(response(operation, value))
}

fn response(operation: OperationId, value: serde_json::Value) -> SurfaceResponse {
    let (title, message, summary) = match operation.as_str() {
        "describe" => (
            "Transcription package metadata",
            "Inspected native transcription operations and runtime support.",
            serde_json::json!({
                "operationCount": value.get("operationCount").cloned().unwrap_or(serde_json::Value::Null)
            }),
        ),
        "audio.transcription.transcribe" => (
            "Audio transcription result",
            "Ran audio/video-to-text through the selected provider.",
            serde_json::json!({
                "provider": value.get("provider").cloned().unwrap_or(serde_json::Value::Null),
                "modelId": value.get("modelId").cloned().unwrap_or(serde_json::Value::Null),
                "segmentCount": value.pointer("/transcript/segments").and_then(serde_json::Value::as_array).map_or(0, Vec::len)
            }),
        ),
        "audio.transcription.importWhisperX" => (
            "WhisperX import result",
            "Parsed existing WhisperX JSON through text-transcripts.",
            serde_json::json!({
                "segmentCount": value.get("segments").and_then(serde_json::Value::as_array).map_or(0, Vec::len),
                "hasText": value.get("text").and_then(serde_json::Value::as_str).map(|text| !text.is_empty()).unwrap_or(false)
            }),
        ),
        "audio.transcription.providers" => (
            "Transcription providers",
            "Inspected native and compatibility transcription provider support.",
            serde_json::json!({
                "providerCount": value.get("providers").and_then(serde_json::Value::as_array).map_or(0, Vec::len)
            }),
        ),
        "audio.transcription.plan"
        | "audio.transcription.modelPlan"
        | "audio.transcription.vadPlan"
        | "audio.transcription.alignmentPlan"
        | "audio.transcription.alignmentBundlePlan"
        | "audio.transcription.decodePlan"
        | "audio.transcription.diarizationPlan" => (
            "Transcription runtime plan",
            "Planned transcription setup without execution.",
            serde_json::json!({
                "defaultProvider": value.get("defaultProvider").cloned().unwrap_or(serde_json::Value::Null),
                "normalizationOwner": value.get("normalizationOwner").cloned().unwrap_or(serde_json::Value::Null)
            }),
        ),
        _ => (
            "Transcription operation result",
            "Completed the transcription package operation.",
            serde_json::json!({}),
        ),
    };
    structured_surface_response(operation, title, message, summary, value)
}

fn describe_value(input: serde_json::Value) -> serde_json::Value {
    let surface = package_surface();
    serde_json::json!({
        "library": surface.library,
        "version": surface.version,
        "operationCount": surface.operations.len(),
        "operations": surface.operations.iter().map(|operation| operation.id.as_str()).collect::<Vec<_>>(),
        "input": input
    })
}

fn transcribe_value(input: serde_json::Value) -> Result<serde_json::Value, String> {
    let request: TranscriptionPipelineRequest =
        serde_json::from_value(input).map_err(|error| error.to_string())?;
    let response = transcribe(request).map_err(|error| error.to_string())?;
    Ok(serde_json::json!(response))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportContentRequest {
    content: String,
}

fn import_whisperx_value(request: ImportContentRequest) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!(import_whisperx_json(
        request.content.as_bytes()
    )
    .map_err(|error| error.to_string())?))
}

fn providers_value(input: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "input": input,
        "providers": transcription_provider_plans().into_iter().map(|plan| {
            serde_json::json!({
                "id": plan.provider_id,
                "externalRuntime": plan.external_runtime,
                "wasmSupported": plan.wasm_supported,
                "primary": plan.primary,
                "setup": plan.setup,
                "diagnostics": plan.diagnostics,
            })
        }).collect::<Vec<_>>()
    })
}

fn plan_value(input: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "operation": "audio.transcription.transcribe",
        "defaultProvider": "candle-whisper",
        "supportedTasks": ["transcribe", "translate"],
        "translation": {
            "runtime": "whisper-task",
            "targetLanguage": "en",
            "requiresProvider": "candle-whisper",
            "alignmentSupported": false,
            "alignmentRestriction": "native Whisper translation output cannot be wav2vec2/CTC-aligned against source-language audio"
        },
        "gpu": {
            "cuda": "opt-in through the cuda feature and provider.device=cuda",
            "cpuFallback": true
        },
        "normalizationOwner": "moritzbrantner-text-transcripts",
        "vadProvider": "energy-vad",
        "alignmentProvider": "ctc-forced-aligner",
        "diarizationProvider": "audio-analysis-speakers-native-baseline",
        "candleWhisperDecode": candle_whisper_decode_runtime_plan(),
        "providers": transcription_provider_plans(),
        "input": input
    })
}

fn model_plan_value(input: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "defaultProvider": "candle-whisper",
        "supportedTasks": ["transcribe", "translate"],
        "translation": {
            "runtime": "whisper-task",
            "targetLanguage": "en",
            "postAsrMt": "out-of-scope",
            "alignmentSupported": false
        },
        "gpu": {
            "cuda": "optional Candle CUDA execution when built with cuda",
            "requiredFeature": "cuda"
        },
        "normalizationOwner": "moritzbrantner-text-transcripts",
        "asr": candle_whisper_provider_plan(),
        "candleWhisperDecode": candle_whisper_decode_runtime_plan(),
        "compatibility": [whisper_cpp_provider_plan(), whisperx_provider_plan()],
        "models": [
            "openai/whisper-large-v3",
            "openai/whisper-large-v3-turbo",
            "facebook/wav2vec2-base-960h",
            "pyannote/speaker-diarization-3.1"
        ],
        "input": input
    })
}

fn candle_whisper_decode_runtime_plan() -> serde_json::Value {
    serde_json::json!({
        "default": "autoregressiveKvCache",
        "options": [
            {
                "id": "autoregressiveKvCache",
                "execution": CandleWhisperDecodeRuntime::AutoregressiveKvCache.execution_id(),
                "supported": true,
                "batchSemantics": "semantic chunk grouping with per-window autoregressive decode and KV-cache reuse"
            },
            {
                "id": "activeRowTensorBatch",
                "execution": CandleWhisperDecodeRuntime::ActiveRowTensorBatch.execution_id(),
                "supported": false,
                "batchSemantics": "future true tensor-batched active-row decode",
                "requires": ["batchChunks=true", "maxBatchSize greater than one or unbounded"]
            }
        ]
    })
}

fn vad_plan_value(input: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "defaultProvider": "energy-vad",
        "normalizationOwner": "moritzbrantner-text-transcripts",
        "options": VadOptions::default(),
        "input": input
    })
}

fn alignment_plan_value(input: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "defaultProvider": "candle-whisper",
        "provider": "ctc-forced-aligner",
        "modelId": "facebook/wav2vec2-base-960h",
        "normalizationOwner": "moritzbrantner-text-transcripts",
        "requiresFeature": "alignment",
        "input": input
    })
}

fn alignment_bundle_plan_value(input: serde_json::Value) -> Result<serde_json::Value, String> {
    let bundle_path = input
        .get("bundlePath")
        .or_else(|| input.get("modelBundle"))
        .or_else(|| input.pointer("/alignment/modelBundle"))
        .or_else(|| input.pointer("/alignment/modelBundlePath"))
        .and_then(serde_json::Value::as_str);

    let Some(bundle_path) = bundle_path else {
        return Ok(serde_json::json!({
            "defaultProvider": "ctc-forced-aligner",
            "modelFamily": "wav2vec2",
            "requiresFeature": "alignment",
            "requiresModelBundles": true,
            "bundleProvided": false,
            "layout": serde_json::Value::Null,
            "supported": false,
            "unsupportedReasons": [],
            "execution": "not-run",
            "input": input
        }));
    };

    let bundle = std::path::Path::new(bundle_path);
    if !bundle.exists() {
        return Err(setup_error(format!(
            "required wav2vec2 alignment bundle `{}` is missing",
            bundle.display()
        ))
        .to_string());
    }

    #[cfg(feature = "alignment")]
    {
        let report = crate::native_wav2vec2::inspect_wav2vec2_bundle_layout(bundle)
            .map_err(|error| error.to_string())?;
        let supported =
            report.missing_required_keys.is_empty() && report.unsupported_reasons.is_empty();
        Ok(serde_json::json!({
            "defaultProvider": "ctc-forced-aligner",
            "modelFamily": "wav2vec2",
            "requiresFeature": "alignment",
            "requiresModelBundles": true,
            "bundleProvided": true,
            "layout": {
                "architecture": report.architecture,
                "doStableLayerNorm": report.do_stable_layer_norm,
                "positionalConvLayout": report.positional_conv_layout,
                "featureExtractorNorm": report.feature_extractor_norm,
                "encoderLayerCount": report.encoder_layer_count,
                "missingRequiredKeys": report.missing_required_keys
            },
            "supported": supported,
            "unsupportedReasons": report.unsupported_reasons,
            "execution": "not-run",
            "input": input
        }))
    }

    #[cfg(not(feature = "alignment"))]
    {
        Ok(serde_json::json!({
            "defaultProvider": "ctc-forced-aligner",
            "modelFamily": "wav2vec2",
            "requiresFeature": "alignment",
            "requiresModelBundles": true,
            "bundleProvided": true,
            "layout": serde_json::Value::Null,
            "supported": false,
            "unsupportedReasons": ["current build does not include the alignment feature"],
            "execution": "not-run",
            "input": input
        }))
    }
}

fn decode_plan_value(input: serde_json::Value) -> serde_json::Value {
    let provider_kind = input
        .pointer("/provider/kind")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("candleWhisper");
    let source = input.get("source").unwrap_or(&input);
    let audio_io_enabled = cfg!(feature = "audio-io");
    let plan = if provider_kind == "externalWhisperX" || provider_kind == "whisperx" {
        serde_json::json!({
            "sourceKind": source_kind(source),
            "decodePath": "external-whisperx-compatibility",
            "opensFiles": false,
            "executesFfmpeg": false,
            "featureGated": false,
            "notes": "External WhisperX compatibility owns media/container decode for this provider."
        })
    } else if source.get("samples").is_some() {
        serde_json::json!({
            "sourceKind": "samples",
            "decodePath": "direct-samples",
            "opensFiles": false,
            "executesFfmpeg": false,
            "featureGated": false,
            "normalization": "normalize_samples_source"
        })
    } else if let Some(path) = source.get("path").and_then(serde_json::Value::as_str) {
        let extension = std::path::Path::new(path)
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if extension == "wav" {
            serde_json::json!({
                "sourceKind": "path",
                "pathExtension": extension,
                "decodePath": "native-wav-reader",
                "opensFiles": false,
                "executesFfmpeg": false,
                "featureGated": false,
                "normalization": "native mono mix and resample to 16 kHz"
            })
        } else {
            serde_json::json!({
                "sourceKind": "path",
                "pathExtension": extension,
                "decodePath": if audio_io_enabled { "audio-io-media-decode" } else { "unsupported-runtime-without-audio-io" },
                "opensFiles": false,
                "executesFfmpeg": false,
                "featureGated": true,
                "requiredFeature": "audio-io",
                "audioIoFeatureEnabled": audio_io_enabled,
                "normalization": if audio_io_enabled { "audio-io mono decode then normalize_samples_source to 16 kHz" } else { "not available" }
            })
        }
    } else {
        serde_json::json!({
            "sourceKind": source_kind(source),
            "decodePath": "unknown-source",
            "opensFiles": false,
            "executesFfmpeg": false,
            "featureGated": false,
            "notes": "Provide source.samples or source.path for a concrete decode plan."
        })
    };
    serde_json::json!({
        "defaultProvider": "candle-whisper",
        "normalizationOwner": "moritzbrantner-audio-analysis-transcription",
        "defaultNativeBoundary": "wav-or-direct-samples",
        "audioIoFeatureEnabled": audio_io_enabled,
        "plan": plan,
        "input": input
    })
}

fn diarization_plan_value(input: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "defaultProvider": "native-speaker-diarization",
        "currentRuntime": "heuristic-native",
        "productionParity": false,
        "assignmentPolicies": ["majority", "nearestStart", "strictContained"],
        "speakerBounds": {
            "minSpeakers": "validated and reported only",
            "maxSpeakers": "validated and reported only",
            "enforcedAsClusteringConstraints": false
        },
        "futureProviders": [
            "onnx-speaker-embedding",
            "pyannote-style-speaker-embedding",
            "external-pyannote-compatibility"
        ],
        "input": input
    })
}

fn source_kind(source: &serde_json::Value) -> &'static str {
    if source.get("samples").is_some() {
        "samples"
    } else if source.get("path").is_some() {
        "path"
    } else {
        "unknown"
    }
}

fn parse_input<T: for<'de> Deserialize<'de>>(input: serde_json::Value) -> Result<T, String> {
    runtime_core::parse_surface_input(None, input)
}

/// Builds a default native Candle Whisper request for local callers.
pub fn default_native_request(path: impl Into<std::path::PathBuf>) -> TranscriptionPipelineRequest {
    TranscriptionPipelineRequest {
        source: TranscriptionSource::Path { path: path.into() },
        provider: TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions::default()),
        vad: VadOptions::default(),
        alignment: AlignmentOptions::default(),
        diarization: DiarizationOptions::default(),
        output: Default::default(),
    }
}

/// Builds a default WhisperX compatibility request for local callers.
pub fn default_whisperx_request(
    path: impl Into<std::path::PathBuf>,
) -> TranscriptionPipelineRequest {
    TranscriptionPipelineRequest {
        source: TranscriptionSource::Path { path: path.into() },
        provider: TranscriptionProviderSelection::ExternalWhisperX(WhisperXCommandOptions {
            device: WhisperXDevice::Cpu,
            compute_type: Some("int8".to_string()),
            ..WhisperXCommandOptions::default()
        }),
        vad: VadOptions::default(),
        alignment: AlignmentOptions::default(),
        diarization: DiarizationOptions::default(),
        output: Default::default(),
    }
}

/// Builds a CUDA-preferring native request.
pub fn cuda_native_request(path: impl Into<std::path::PathBuf>) -> TranscriptionPipelineRequest {
    let mut request = default_native_request(path);
    request.provider = TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
        device: NativeDevicePreference::Cuda,
        ..CandleWhisperOptions::default()
    });
    request
}

/// Builds a CUDA-preferring native Whisper translate-to-English request.
pub fn cuda_translate_request(path: impl Into<std::path::PathBuf>) -> TranscriptionPipelineRequest {
    let mut request = cuda_native_request(path);
    request.provider = TranscriptionProviderSelection::CandleWhisper(CandleWhisperOptions {
        task: TranscriptionTask::Translate,
        device: NativeDevicePreference::Cuda,
        ..CandleWhisperOptions::default()
    });
    request.alignment.enabled = false;
    request
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_surface_lists_transcription_operations() {
        let surface = package_surface();
        let ids = surface
            .operations
            .iter()
            .map(|operation| operation.id.as_str())
            .collect::<Vec<_>>();
        assert!(ids.contains(&"audio.transcription.transcribe"));
        assert!(ids.contains(&"audio.transcription.importWhisperX"));
        assert!(ids.contains(&"audio.transcription.providers"));
        assert!(ids.contains(&"audio.transcription.plan"));
        assert!(ids.contains(&"audio.transcription.modelPlan"));
        assert!(ids.contains(&"audio.transcription.vadPlan"));
        assert!(ids.contains(&"audio.transcription.alignmentPlan"));
        assert!(ids.contains(&"audio.transcription.alignmentBundlePlan"));
        assert!(ids.contains(&"audio.transcription.decodePlan"));
        assert!(ids.contains(&"audio.transcription.diarizationPlan"));
    }

    #[test]
    fn import_whisperx_operation_returns_transcript() {
        let response = run_surface_operation(SurfaceRequest {
            operation: OperationId::new("audio.transcription.importWhisperX"),
            input: serde_json::json!({
                "content": "{\"segments\":[{\"start\":0.0,\"end\":1.0,\"text\":\"hello\"}]}"
            }),
        })
        .expect("import");
        assert_eq!(
            response.value["operation"],
            "audio.transcription.importWhisperX"
        );
        assert_eq!(
            response.value["result"]["segments"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn plan_reports_candle_as_primary_native_runtime() {
        let response = run_surface_operation(SurfaceRequest {
            operation: OperationId::new("audio.transcription.plan"),
            input: serde_json::json!({}),
        })
        .expect("plan");
        assert_eq!(
            response.value["result"]["defaultProvider"],
            "candle-whisper"
        );
        assert_eq!(
            response.value["result"]["normalizationOwner"],
            "moritzbrantner-text-transcripts"
        );
        assert_eq!(
            response.value["result"]["translation"]["runtime"],
            "whisper-task"
        );
        assert_eq!(
            response.value["result"]["translation"]["targetLanguage"],
            "en"
        );
        assert_eq!(
            response.value["result"]["translation"]["alignmentSupported"],
            false
        );
        assert_eq!(
            response.value["result"]["candleWhisperDecode"]["default"],
            "autoregressiveKvCache"
        );
        assert_eq!(
            response.value["result"]["candleWhisperDecode"]["options"][0]["execution"],
            "candle-whisper-autoregressive-kv-cache"
        );
    }

    #[test]
    fn model_plan_exposes_future_active_row_decode_runtime_as_unsupported() {
        let response = run_surface_operation(SurfaceRequest {
            operation: OperationId::new("audio.transcription.modelPlan"),
            input: serde_json::json!({}),
        })
        .expect("model plan");

        let options = response.value["result"]["candleWhisperDecode"]["options"]
            .as_array()
            .unwrap();
        let active_row = options
            .iter()
            .find(|option| option["id"] == "activeRowTensorBatch")
            .unwrap();
        assert_eq!(
            active_row["execution"],
            "candle-whisper-active-row-tensor-batch"
        );
        assert_eq!(active_row["supported"], false);
    }

    #[test]
    fn decode_plan_reports_non_wav_audio_io_boundary_without_opening_files() {
        let response = run_surface_operation(SurfaceRequest {
            operation: OperationId::new("audio.transcription.decodePlan"),
            input: serde_json::json!({"source": {"path": "clip.mp4"}}),
        })
        .expect("decode plan");
        assert_eq!(
            response.value["result"]["plan"]["decodePath"],
            if cfg!(feature = "audio-io") {
                "audio-io-media-decode"
            } else {
                "unsupported-runtime-without-audio-io"
            }
        );
        assert_eq!(response.value["result"]["plan"]["opensFiles"], false);
        assert_eq!(response.value["result"]["plan"]["executesFfmpeg"], false);
    }

    #[test]
    fn decode_plan_routes_wav_and_samples_without_execution() {
        let wav = run_surface_operation(SurfaceRequest {
            operation: OperationId::new("audio.transcription.decodePlan"),
            input: serde_json::json!({"source": {"path": "clip.wav"}}),
        })
        .expect("wav decode plan");
        assert_eq!(
            wav.value["result"]["plan"]["decodePath"],
            "native-wav-reader"
        );
        assert_eq!(wav.value["result"]["plan"]["opensFiles"], false);

        let samples = run_surface_operation(SurfaceRequest {
            operation: OperationId::new("audio.transcription.decodePlan"),
            input: serde_json::json!({"source": {"samples": [0.0], "sampleRate": 16000, "channels": 1}}),
        })
        .expect("samples decode plan");
        assert_eq!(
            samples.value["result"]["plan"]["decodePath"],
            "direct-samples"
        );
        assert_eq!(samples.value["result"]["plan"]["opensFiles"], false);
    }

    #[test]
    fn alignment_bundle_plan_runs_without_local_files() {
        let response = run_surface_operation(SurfaceRequest {
            operation: OperationId::new("audio.transcription.alignmentBundlePlan"),
            input: serde_json::json!({}),
        })
        .expect("alignment bundle plan");
        assert_eq!(
            response.value["result"]["defaultProvider"],
            "ctc-forced-aligner"
        );
        assert_eq!(response.value["result"]["bundleProvided"], false);
        assert_eq!(response.value["result"]["execution"], "not-run");
    }

    #[test]
    fn alignment_bundle_plan_missing_path_returns_setup_error() {
        let response = run_surface_operation(SurfaceRequest {
            operation: OperationId::new("audio.transcription.alignmentBundlePlan"),
            input: serde_json::json!({"bundlePath": "/definitely/missing/wav2vec2"}),
        });

        let error = response.unwrap_err();
        assert!(error.contains("setup_error"));
        assert!(error.contains("missing"));
    }

    #[test]
    fn diarization_plan_reports_heuristic_status_without_model_parity_claim() {
        let response = run_surface_operation(SurfaceRequest {
            operation: OperationId::new("audio.transcription.diarizationPlan"),
            input: serde_json::json!({}),
        })
        .expect("diarization plan");
        assert_eq!(
            response.value["result"]["currentRuntime"],
            "heuristic-native"
        );
        assert_eq!(response.value["result"]["productionParity"], false);
        assert_eq!(
            response.value["result"]["speakerBounds"]["enforcedAsClusteringConstraints"],
            false
        );
    }
}

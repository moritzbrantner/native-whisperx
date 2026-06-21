#![doc = include_str!("../README.md")]

#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::path::{Path, PathBuf};

mod silero_vad;
mod speaker_directory;

#[cfg(feature = "diarization")]
use audio_analysis_speakers::{
    AudioRuntime, DiarizationSegment, DiarizedSpeaker, EnergyVadConfig,
    EnergyVoiceActivityDetector, SpeakerDiarizer, SpeakerIdentificationOptions, SpeakerLibrary,
    SpeakerSegmentPrediction, SpeechSpan, WindowedSpeakerDiarizer,
};
#[cfg(all(test, feature = "diarization"))]
use audio_analysis_transcription::SpeakerDiarizationOptions;
pub use audio_analysis_transcription::{
    AlignmentInterpolationMethod, TranscriptionPipelineRequest, TranscriptionPipelineResponse,
};
#[cfg(test)]
use audio_analysis_transcription::{
    CandleWhisperDecodeRuntime, NativeDevicePreference, SpeakerAssignmentPolicy,
    SpeechActivitySegment, TranscriptionProviderSelection, TranscriptionSource,
    TranscriptionTask as UpstreamTranscriptionTask, WhisperXDevice,
};
#[cfg(feature = "diarization")]
use audio_analysis_transcription::{
    DiarizationOptions, NativeSpeakerDiarizationProvider, SpeakerDiarizationResponse,
    TranscriptDiarizationProvider,
};
pub use speaker_directory::{
    delete_speaker_profile, global_speaker_directory, list_speaker_profiles,
    local_speaker_directory, read_speaker_directory_state, rebuild_speaker_trace,
    reject_draft_speaker_profile_creation, resolve_speaker_directory, speaker_library_path,
    speaker_trace_path, update_speaker_profile, validate_speaker_library,
    validate_speaker_library_file, ResolvedSpeakerDirectory, ResolvedSpeakerDirectoryScope,
    SpeakerCorrectionRange, SpeakerDirectoryScope, SpeakerDirectorySelection,
    SpeakerDirectoryState, SpeakerDirectoryStateScope, SpeakerLibraryState,
    SpeakerLibraryValidation, SpeakerLibraryValidationStatus, SpeakerProfileEdit,
    SpeakerProfileState, SpeakerProfileSummary, SpeakerTrace, SpeakerTraceError, SpeakerTraceFile,
    SpeakerTraceRebuildReport, SpeakerTraceRebuildStats, SpeakerTraceSpan, SpeakerTraceSpeaker,
    SpeakerTraceSpeakerKind, SpeakerTraceState, SpeakerTraceStateStatus,
    GLOBAL_SPEAKER_DIRECTORY_APP, GLOBAL_SPEAKER_DIRECTORY_NAME, LOCAL_SPEAKER_DIRECTORY,
    SPEAKER_LIBRARY_FILE, SPEAKER_TRACE_FILE,
};
pub use text_transcripts::TranscriptionContract;

mod config;
mod config_mapping;
mod output;
mod parity;
mod report;
mod workflow;

pub use config::*;
pub use config_mapping::build_transcription_request;
pub use output::write_outputs;
pub use parity::{compare_with_whisperx, run_parity_fixture_suite, run_parity_preflight};
pub use workflow::{run, run_many};

#[cfg(all(test, feature = "silero-vad"))]
use config_mapping::resolve_silero_model_path;
#[cfg(all(test, feature = "silero-vad"))]
use config_mapping::validate_native_silero_config;
#[cfg(test)]
use config_mapping::{map_diarization, native_language_hint, validate_native_diarization_support};
#[cfg(test)]
use output::write_outputs_with_options;
#[cfg(test)]
use output::{compare_expected_outputs, compare_output_json, whisperx_json_value};
#[cfg(test)]
use parity::{
    compare_diagnostics, compare_transcripts, compare_vad_segments, expected_transcript_matches,
    missing_required_diagnostics, parity_fixture_case_passed, parity_fixture_failure_summary,
    resolve_fixture_case_paths, run_parity_fixture_suite_with_runner,
};
#[cfg(test)]
use report::{append_native_alignment_diagnostics, append_native_diarization_diagnostics};
mod diarization;
mod speaker;
mod translation;

#[cfg(feature = "diarization")]
pub(crate) use diarization::native_diarization_provider;
pub use speaker::correct_speaker;
pub(crate) use speaker::save_draft_speakers_from_response;
pub use translation::import_whisperx_json;
pub(crate) use translation::run_native_with_translation;
#[cfg(all(test, feature = "translation"))]
use translation::{
    resolve_translation_bundle, TranslationWeightFormat, REQUIRED_TRANSLATION_FILES,
};
#[cfg(test)]
use translation::{translate_response_segments, SegmentTranslator, TranslationRunOptions};
#[cfg(test)]
mod tests {
    use super::*;

    const WHISPERX_SAMPLE: &[u8] =
        include_bytes!("../../../tests/fixtures/whisperx-parity-sample.json");

    #[test]
    fn map_diarization_maps_all_assignment_policy_variants() {
        for (input, expected) in [
            (
                AssignmentPolicy::Majority,
                SpeakerAssignmentPolicy::Majority,
            ),
            (
                AssignmentPolicy::NearestStart,
                SpeakerAssignmentPolicy::NearestStart,
            ),
            (
                AssignmentPolicy::StrictContained,
                SpeakerAssignmentPolicy::StrictContained,
            ),
        ] {
            let mapped = map_diarization(&DiarizationConfig {
                enabled: true,
                assignment_policy: input,
                ..DiarizationConfig::default()
            });
            assert_eq!(mapped.assignment_policy, expected);
        }
    }

    #[test]
    fn map_diarization_maps_pyannote_bundle_and_phase_artifacts() {
        let mapped = map_diarization(&DiarizationConfig {
            enabled: true,
            model_id: "pyannote/speaker-diarization-community-1".to_string(),
            model_bundle: Some(PathBuf::from("/models/pyannote-diarization")),
            manifest_file: Some("manifest.json".to_string()),
            segmentation_model_file: Some("segmentation.onnx".to_string()),
            embedding_model_file: Some("embedding.onnx".to_string()),
            plda_transform_file: Some("plda_transform.json".to_string()),
            plda_model_file: Some("plda_model.json".to_string()),
            clustering_config_file: Some("clustering.json".to_string()),
            return_speaker_embeddings: true,
            min_speakers: Some(2),
            max_speakers: Some(2),
            ..DiarizationConfig::default()
        });

        assert_eq!(mapped.model_id, "pyannote/speaker-diarization-community-1");
        assert_eq!(
            mapped.pyannote_model_bundle.as_deref(),
            Some(Path::new("/models/pyannote-diarization"))
        );
        assert_eq!(
            mapped.pyannote_manifest_file.as_deref(),
            Some("manifest.json")
        );
        assert_eq!(
            mapped.pyannote_segmentation_model_file.as_deref(),
            Some("segmentation.onnx")
        );
        assert_eq!(
            mapped.pyannote_embedding_model_file.as_deref(),
            Some("embedding.onnx")
        );
        assert_eq!(
            mapped.pyannote_plda_transform_file.as_deref(),
            Some("plda_transform.json")
        );
        assert_eq!(
            mapped.pyannote_plda_model_file.as_deref(),
            Some("plda_model.json")
        );
        assert_eq!(
            mapped.pyannote_clustering_config_file.as_deref(),
            Some("clustering.json")
        );
        assert!(mapped.return_speaker_embeddings);
        assert_eq!(mapped.min_speakers, Some(2));
        assert_eq!(mapped.max_speakers, Some(2));
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn native_diarization_with_runtime_speaker_library_labels_known_speaker(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let audio = two_speaker_loaded_audio();
        let library = speaker_library_matching_first_span(&audio)?;
        let transcript = timed_transcript(vec![("hello", 0.20, 0.50), ("world", 1.00, 1.40)])?;
        let mut provider = ConfiguredNativeDiarizationProvider {
            speaker_library: RuntimeSpeakerLibraryStatus::Loaded(RuntimeSpeakerLibrary {
                path: PathBuf::from("/project/.native-whisperx/speakers/library.json"),
                profile_count: 1,
                filtered_draft_profiles: 0,
                use_draft_profiles: true,
                library,
            }),
        };

        let response = provider.diarize(
            audio,
            &transcript,
            &DiarizationOptions {
                enabled: true,
                speaker: SpeakerDiarizationOptions {
                    model_id: "native-spectral-speaker-baseline".to_string(),
                    ..SpeakerDiarizationOptions::default()
                },
            },
        )?;

        assert_eq!(response.segments[0].speaker, "known-speaker");
        assert!(response
            .diagnostics
            .contains(&"speakerLibraryStatus=loaded".to_string()));
        assert!(response
            .diagnostics
            .contains(&"speakerLibraryProfiles=1".to_string()));
        Ok(())
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn native_diarization_missing_runtime_speaker_library_keeps_anonymous_labels(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let audio = two_speaker_loaded_audio();
        let transcript = timed_transcript(vec![("hello", 0.20, 0.50), ("world", 1.00, 1.40)])?;
        let mut provider = ConfiguredNativeDiarizationProvider {
            speaker_library: RuntimeSpeakerLibraryStatus::Missing(PathBuf::from(
                "/missing/library.json",
            )),
        };

        let response = provider.diarize(
            audio,
            &transcript,
            &DiarizationOptions {
                enabled: true,
                speaker: SpeakerDiarizationOptions {
                    model_id: "native-spectral-speaker-baseline".to_string(),
                    ..SpeakerDiarizationOptions::default()
                },
            },
        )?;

        assert!(response
            .segments
            .iter()
            .all(|segment| segment.speaker.starts_with("speaker_")));
        assert!(response
            .diagnostics
            .contains(&"speakerLibraryStatus=missing".to_string()));
        Ok(())
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn runtime_speaker_library_can_be_disabled_explicitly() {
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
                disable_speaker_library: true,
                speaker_directory: SpeakerDirectorySelection {
                    scope: SpeakerDirectoryScope::Local,
                    explicit_path: Some(PathBuf::from("/ignored")),
                },
                ..DiarizationConfig::default()
            },
            output: OutputConfig::default(),
        };

        assert!(matches!(
            runtime_speaker_library_status(&config).expect("status"),
            RuntimeSpeakerLibraryStatus::Disabled
        ));
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn external_whisperx_ignores_runtime_speaker_library_selection() {
        let config = NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                provider: AsrProvider::ExternalWhisperX,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig {
                enabled: true,
                speaker_directory: SpeakerDirectorySelection {
                    scope: SpeakerDirectoryScope::Auto,
                    explicit_path: Some(PathBuf::from("/ignored")),
                },
                ..DiarizationConfig::default()
            },
            output: OutputConfig::default(),
        };

        assert!(matches!(
            runtime_speaker_library_status(&config).expect("status"),
            RuntimeSpeakerLibraryStatus::ExternalWhisperX
        ));
        let request = build_transcription_request(&config).expect("external request should build");
        match request.provider {
            TranscriptionProviderSelection::ExternalWhisperX(options) => {
                assert!(!options
                    .extra_args
                    .iter()
                    .any(|arg| arg.contains("speaker-library")
                        || arg.contains("speakerLibrary")
                        || arg.contains("speaker_directory")));
            }
            other => panic!("expected external provider, got {other:?}"),
        }
    }

    #[cfg(feature = "diarization")]
    #[test]
    fn transcription_request_json_does_not_serialize_runtime_speaker_library() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig {
                enabled: true,
                speaker_directory: SpeakerDirectorySelection {
                    scope: SpeakerDirectoryScope::Auto,
                    explicit_path: Some(PathBuf::from("/project/speakers")),
                },
                ..DiarizationConfig::default()
            },
            output: OutputConfig::default(),
        })
        .expect("request should build");

        let json = serde_json::to_string(&request).expect("request JSON");
        assert!(!json.contains("Speaker A"));
        assert!(!json.contains("profiles"));
        assert!(!json.contains("speakerDirectory"));
        assert!(!json.contains("speakerLibrary"));
    }

    #[test]
    fn native_speaker_embeddings_require_pyannote_bundle() {
        let error = validate_native_diarization_support(&DiarizationConfig {
            enabled: true,
            return_speaker_embeddings: true,
            ..DiarizationConfig::default()
        })
        .expect_err("non-pyannote embeddings should be rejected")
        .to_string();

        assert!(error.contains("native speaker embeddings require"));

        let error = validate_native_diarization_support(&DiarizationConfig {
            enabled: true,
            model_id: "pyannote/speaker-diarization-community-1".to_string(),
            return_speaker_embeddings: true,
            ..DiarizationConfig::default()
        })
        .expect_err("pyannote embeddings without a bundle should be rejected")
        .to_string();

        assert!(error.contains("native speaker embeddings require"));
    }

    #[test]
    fn native_diarization_bundle_requires_pyannote_model() {
        let error = validate_native_diarization_support(&DiarizationConfig {
            enabled: true,
            model_bundle: Some(PathBuf::from("/models/pyannote-diarization")),
            ..DiarizationConfig::default()
        })
        .expect_err("bundle without pyannote model should be rejected")
        .to_string();

        assert!(error.contains("modelBundle is only supported for pyannote"));
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
    fn maps_native_surface_defaults() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        assert!(matches!(request.source, TranscriptionSource::Path { .. }));
        assert!(request.vad.enabled);
        assert!(request.alignment.enabled);
        assert_eq!(request.output.formats, vec!["json"]);
        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.model_id, "small");
                assert_eq!(
                    options.decode_runtime,
                    CandleWhisperDecodeRuntime::ActiveRowTensorBatch
                );
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn maps_native_single_row_batch_to_kv_cache_runtime() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                max_batch_size: Some(1),
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(
                    options.decode_runtime,
                    CandleWhisperDecodeRuntime::AutoregressiveKvCache
                );
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn maps_native_disabled_batching_to_kv_cache_runtime() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                batch_chunks: false,
                max_batch_size: Some(4),
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(
                    options.decode_runtime,
                    CandleWhisperDecodeRuntime::AutoregressiveKvCache
                );
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn maps_native_english_only_whisper_alias_to_language_hint() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                model_id: "tiny.en".to_string(),
                language: None,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.language.as_deref(), Some("en"));
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn maps_native_multilingual_whisper_model_without_language_hint() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                model_id: "small".to_string(),
                language: None,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.language, None);
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn explicit_native_language_overrides_english_only_model_hint() {
        let asr = AsrConfig {
            model_id: "openai/whisper-tiny.en".to_string(),
            language: Some("de".to_string()),
            ..AsrConfig::default()
        };

        assert_eq!(native_language_hint(&asr).as_deref(), Some("de"));
    }

    #[test]
    fn maps_config_to_transcription_request() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                language: Some("en".to_string()),
                whisper_bundle: Some(PathBuf::from("models/whisper")),
                device: DevicePreference::Cpu,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig {
                enabled: true,
                model_id: "facebook/wav2vec2-base-960h".to_string(),
                model_bundle: Some(PathBuf::from("models/wav2vec2")),
                model_dir: Some(PathBuf::from("models/cache")),
                model_cache_only: true,
                interpolate_method: AlignmentInterpolationMethod::Linear,
                return_char_alignments: true,
            },
            diarization: DiarizationConfig::default(),
            output: OutputConfig {
                formats: vec![OutputFormat::Json, OutputFormat::Srt],
                ..OutputConfig::default()
            },
        })
        .expect("request should build");

        assert!(matches!(request.source, TranscriptionSource::Path { .. }));
        assert!(request.alignment.enabled);
        assert_eq!(
            request.alignment.model_dir,
            Some(PathBuf::from("models/cache"))
        );
        assert!(request.alignment.model_cache_only);
        assert_eq!(
            request.alignment.interpolate_method,
            AlignmentInterpolationMethod::Linear
        );
        assert_eq!(request.alignment.device, NativeDevicePreference::Cpu);
        assert!(request.alignment.return_char_alignments);
        assert_eq!(request.output.formats, vec!["json", "srt"]);
        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.language.as_deref(), Some("en"));
                assert_eq!(options.device, NativeDevicePreference::Cpu);
                assert_eq!(options.model_bundle, Some(PathBuf::from("models/whisper")));
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn maps_native_asr_cuda_device_to_alignment_options() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                device: DevicePreference::Cuda,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig {
                enabled: true,
                ..AlignmentConfig::default()
            },
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        assert_eq!(request.alignment.device, NativeDevicePreference::Cuda);
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

    #[test]
    fn maps_native_asr_model_cache_options() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                model_dir: Some(PathBuf::from("models")),
                model_cache_only: true,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("request should build");

        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.model_dir, Some(PathBuf::from("models")));
                assert!(options.model_cache_only);
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn accepts_native_decode_controls_that_match_greedy_defaults() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                decode: WhisperxDecodeConfig {
                    temperature: vec![0.0],
                    condition_on_previous_text: Some(false),
                    ..WhisperxDecodeConfig::default()
                },
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("greedy native decode defaults should build");

        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(_) => {}
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn rejects_native_decode_controls_with_specific_reasons() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                decode: WhisperxDecodeConfig {
                    beam_size: Some(5),
                    initial_prompt: Some("context".to_string()),
                    logprob_threshold: Some(-1.0),
                    ..WhisperxDecodeConfig::default()
                },
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native decode controls should be rejected");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        let message = error.to_string();
        assert!(message.contains("--beam_size (beam search is not exposed"));
        assert!(message.contains("--initial_prompt (prompt-prefilled decoder context"));
        assert!(message
            .contains("--logprob_threshold (fallback thresholds require token log probability"));
        assert!(message.contains("external-whisperx"));
    }

    #[test]
    fn reports_each_unsupported_native_decode_control() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                compute_type: Some("int8".to_string()),
                device_index: Some("0".to_string()),
                decode: WhisperxDecodeConfig {
                    temperature: vec![0.0, 0.2],
                    best_of: Some(3),
                    patience: Some(1.2),
                    length_penalty: Some(1.1),
                    suppress_tokens: Some("-1".to_string()),
                    suppress_numerals: true,
                    initial_prompt: Some("domain prompt".to_string()),
                    hotwords: Some("proper nouns".to_string()),
                    condition_on_previous_text: Some(true),
                    fp16: Some(false),
                    compression_ratio_threshold: Some(2.4),
                    logprob_threshold: Some(-1.0),
                    no_speech_threshold: Some(0.6),
                    threads: Some(4),
                    ..WhisperxDecodeConfig::default()
                },
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native unsupported controls should be rejected");

        let message = error.to_string();
        for expected in [
            "--compute_type",
            "--device_index",
            "--temperature",
            "--best_of",
            "--patience",
            "--length_penalty",
            "--suppress_tokens",
            "--suppress_numerals",
            "--initial_prompt",
            "--hotwords",
            "--condition_on_previous_text",
            "--fp16",
            "--compression_ratio_threshold",
            "--logprob_threshold",
            "--no_speech_threshold",
            "--threads",
        ] {
            assert!(
                message.contains(expected),
                "error should mention `{expected}`: {message}"
            );
        }
    }

    #[cfg(feature = "pyannote-vad")]
    #[test]
    fn rejects_native_pyannote_vad_without_model_bundle() {
        let error = build_transcription_request(&NativeWhisperxConfig {
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
        })
        .expect_err("native pyannote VAD should be rejected");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains("--vad-model-bundle"));
    }

    #[cfg(not(feature = "pyannote-vad"))]
    #[test]
    fn rejects_native_pyannote_vad_without_feature() {
        let error = build_transcription_request(&NativeWhisperxConfig {
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
        })
        .expect_err("native pyannote VAD should require a feature");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains("pyannote-vad feature"));
    }

    #[cfg(feature = "pyannote-vad")]
    #[test]
    fn accepts_native_pyannote_vad_with_local_onnx_bundle() {
        let temp = tempfile::tempdir().expect("tempdir");
        let model = temp.path().join("pyannote_vad.onnx");
        fs::write(&model, b"fixture").expect("model file");

        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig {
                method: VadMethod::Pyannote,
                model_bundle: Some(temp.path().to_path_buf()),
                model_file: Some("pyannote_vad.onnx".to_string()),
                ..VadConfig::default()
            },
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("native pyannote VAD should accept an explicit local ONNX bundle");

        assert!(request.vad.enabled);
    }

    #[cfg(not(feature = "silero-vad"))]
    #[test]
    fn rejects_native_silero_without_feature() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig {
                method: VadMethod::Silero,
                ..VadConfig::default()
            },
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native silero VAD should be rejected");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains("silero-vad feature"));
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn silero_requires_model_bundle_with_feature() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig {
                method: VadMethod::Silero,
                ..VadConfig::default()
            },
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native silero VAD should require a model bundle");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains("--vad-model-bundle"));
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn resolves_silero_direct_onnx_path() {
        let temp = tempfile::tempdir().expect("tempdir");
        let model = temp.path().join("silero.onnx");
        fs::write(&model, b"fixture").expect("model file");
        let vad = VadConfig {
            model_bundle: Some(model.clone()),
            ..VadConfig::default()
        };

        assert_eq!(resolve_silero_model_path(&vad).expect("path"), model);
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn resolves_silero_bundle_directory() {
        let temp = tempfile::tempdir().expect("tempdir");
        let model = temp.path().join("silero_vad.onnx");
        fs::write(&model, b"fixture").expect("model file");
        let vad = VadConfig {
            model_bundle: Some(temp.path().to_path_buf()),
            ..VadConfig::default()
        };

        assert_eq!(resolve_silero_model_path(&vad).expect("path"), model);
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn resolves_silero_custom_model_file() {
        let temp = tempfile::tempdir().expect("tempdir");
        let model = temp.path().join("model.onnx");
        fs::write(&model, b"fixture").expect("model file");
        let vad = VadConfig {
            model_bundle: Some(temp.path().to_path_buf()),
            model_file: Some("model.onnx".to_string()),
            ..VadConfig::default()
        };

        assert_eq!(resolve_silero_model_path(&vad).expect("path"), model);
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn rejects_invalid_silero_onset_before_model_resolution() {
        let error = validate_native_silero_config(&VadConfig {
            method: VadMethod::Silero,
            onset: Some(0.0),
            ..VadConfig::default()
        })
        .expect_err("invalid onset should fail");

        assert!(error.to_string().contains("vad_onset"));
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    fn rejects_invalid_silero_chunk_size_before_model_resolution() {
        let error = validate_native_silero_config(&VadConfig {
            method: VadMethod::Silero,
            chunk_size: Some(0.0),
            ..VadConfig::default()
        })
        .expect_err("invalid chunk size should fail");

        assert!(error.to_string().contains("chunk_size"));
    }

    #[test]
    fn rejects_native_translate_with_alignment() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                task: TranscriptionTask::Translate,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native translate should be rejected");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains(
            "native --task translate requires --translation-model or --translation-bundle"
        ));
    }

    #[test]
    fn rejects_native_translate_without_alignment() {
        let error = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                task: TranscriptionTask::Translate,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig {
                enabled: false,
                ..AlignmentConfig::default()
            },
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect_err("native translate should require a translation model");

        assert!(matches!(error, NativeWhisperxError::InvalidConfig(_)));
        assert!(error.to_string().contains(
            "native --task translate requires --translation-model or --translation-bundle"
        ));
    }

    #[test]
    fn maps_native_translate_with_translation_model_to_post_asr_transcribe_request() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                task: TranscriptionTask::Translate,
                language: Some("de".to_string()),
                ..AsrConfig::default()
            },
            translation: TranslationConfig {
                enabled: true,
                model_id: Some("Helsinki-NLP/opus-mt-de-en".to_string()),
                target_language: Some("en".to_string()),
                ..TranslationConfig::default()
            },
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("native post-ASR translation should build with alignment");

        assert!(request.alignment.enabled);
        match request.provider {
            TranscriptionProviderSelection::CandleWhisper(options) => {
                assert_eq!(options.language.as_deref(), Some("de"));
                assert_eq!(options.task, UpstreamTranscriptionTask::Transcribe);
            }
            other => panic!("expected native provider, got {other:?}"),
        }
    }

    #[test]
    fn translates_segments_with_configured_languages_and_max_tokens() {
        #[derive(Default)]
        struct FakeTranslator {
            seen: Vec<TranslationRunOptions>,
        }

        impl SegmentTranslator for FakeTranslator {
            fn model_id(&self) -> &str {
                "Helsinki-NLP/opus-mt-de-en"
            }

            fn model_source(&self) -> &'static str {
                "hugging-face-cache"
            }

            fn translate_segment(
                &mut self,
                text: &str,
                options: &TranslationRunOptions,
            ) -> Result<String, NativeWhisperxError> {
                self.seen.push(options.clone());
                Ok(format!("{text} translated"))
            }
        }

        let config = NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                task: TranscriptionTask::Translate,
                language: Some("de".to_string()),
                ..AsrConfig::default()
            },
            translation: TranslationConfig {
                enabled: true,
                model_id: Some("Helsinki-NLP/opus-mt-de-en".to_string()),
                source_language: Some("de".to_string()),
                target_language: Some("en".to_string()),
                max_new_tokens: 7,
                ..TranslationConfig::default()
            },
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        };
        let mut segment = text_transcripts::TranscriptSegmentContract::new(0, "Guten Tag");
        segment
            .words
            .push(text_transcripts::TranscriptWordContract {
                text: "Guten".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.2),
                confidence: None,
                speaker: None,
                attributes: Default::default(),
            });
        let mut response = TranscriptionPipelineResponse {
            accepted: true,
            operation: "transcribe".to_string(),
            provider: "native".to_string(),
            model_id: "small".to_string(),
            transcript: TranscriptionContract::new(vec![segment]),
            vad_segments: Vec::new(),
            alignment: None,
            diarization: None,
            artifacts: Vec::new(),
            diagnostics: Vec::new(),
        };
        let mut translator = FakeTranslator::default();

        translate_response_segments(&mut response, &config, &mut translator)
            .expect("translation should update transcript");

        assert_eq!(response.transcript.language.as_deref(), Some("en"));
        assert_eq!(
            response.transcript.text.as_deref(),
            Some("Guten Tag translated")
        );
        assert_eq!(response.transcript.segments[0].text, "Guten Tag translated");
        assert_eq!(
            response.transcript.segments[0].language.as_deref(),
            Some("en")
        );
        assert!(response.transcript.segments[0].words.is_empty());
        assert_eq!(
            translator.seen,
            vec![TranslationRunOptions {
                source_language: Some("de".to_string()),
                target_language: "en".to_string(),
                max_new_tokens: 7,
            }]
        );
        assert!(response
            .diagnostics
            .contains(&"translationModelSource=hugging-face-cache".to_string()));
        assert!(response
            .diagnostics
            .contains(&"translationMaxNewTokens=7".to_string()));
    }

    #[cfg(feature = "translation")]
    #[test]
    fn translation_cache_only_resolves_fake_hugging_face_snapshot() {
        let temp = tempfile::tempdir().unwrap();
        let snapshot = temp
            .path()
            .join("models--Helsinki-NLP--opus-mt-de-en/snapshots/abc123");
        fs::create_dir_all(&snapshot).unwrap();
        for file in REQUIRED_TRANSLATION_FILES {
            fs::write(snapshot.join(file), "{}").unwrap();
        }
        fs::write(snapshot.join("model.safetensors"), "").unwrap();

        let resolved = resolve_translation_bundle(&TranslationConfig {
            enabled: true,
            model_id: Some("Helsinki-NLP/opus-mt-de-en".to_string()),
            model_dir: Some(temp.path().to_path_buf()),
            model_cache_only: true,
            ..TranslationConfig::default()
        })
        .expect("cache snapshot should resolve");

        assert_eq!(resolved.root, snapshot);
        assert_eq!(resolved.source, "hugging-face-cache");
        assert_eq!(resolved.weight_format, TranslationWeightFormat::Safetensors);
    }

    #[test]
    fn maps_external_whisperx_all_surface_args() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                provider: AsrProvider::ExternalWhisperX,
                task: TranscriptionTask::Translate,
                model_id: "small".to_string(),
                language: Some("en".to_string()),
                device: DevicePreference::Cuda,
                device_index: Some("0".to_string()),
                compute_type: Some("int8".to_string()),
                max_batch_size: Some(8),
                decode: WhisperxDecodeConfig {
                    temperature: vec![0.0, 0.2],
                    best_of: Some(3),
                    beam_size: Some(5),
                    patience: Some(1.2),
                    length_penalty: Some(1.1),
                    suppress_tokens: Some("-1".to_string()),
                    suppress_numerals: true,
                    initial_prompt: Some("domain prompt".to_string()),
                    hotwords: Some("proper nouns".to_string()),
                    condition_on_previous_text: Some(false),
                    fp16: Some(false),
                    compression_ratio_threshold: Some(2.4),
                    logprob_threshold: Some(-1.0),
                    no_speech_threshold: Some(0.6),
                    threads: Some(4),
                },
                external_whisperx: ExternalWhisperxConfig {
                    model: "small".to_string(),
                    align_model: Some("external-align".to_string()),
                    ..ExternalWhisperxConfig::default()
                },
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig {
                method: VadMethod::Silero,
                onset: Some(0.5),
                offset: Some(0.363),
                chunk_size: Some(20.0),
                ..VadConfig::default()
            },
            alignment: AlignmentConfig {
                enabled: false,
                model_id: "fallback-align".to_string(),
                model_dir: Some(PathBuf::from("model-cache")),
                model_cache_only: true,
                return_char_alignments: true,
                ..AlignmentConfig::default()
            },
            diarization: DiarizationConfig {
                enabled: true,
                model_id: "pyannote/speaker-diarization-community-1".to_string(),
                hf_token: Some("token".to_string()),
                return_speaker_embeddings: true,
                min_speakers: Some(1),
                max_speakers: Some(2),
                ..DiarizationConfig::default()
            },
            output: OutputConfig {
                formats: vec![OutputFormat::All],
                subtitles: SubtitleConfig {
                    max_line_width: Some(42),
                    max_line_count: Some(2),
                    highlight_words: true,
                    segment_resolution: SegmentResolution::Chunk,
                },
                ..OutputConfig::default()
            },
        })
        .expect("request should build");

        assert_eq!(
            request.output.formats,
            vec!["txt", "vtt", "srt", "tsv", "aud", "json"]
        );
        match request.provider {
            TranscriptionProviderSelection::ExternalWhisperX(options) => {
                assert_eq!(options.model, "small");
                assert_eq!(options.task, UpstreamTranscriptionTask::Translate);
                assert_eq!(options.language.as_deref(), Some("en"));
                assert_eq!(options.device, WhisperXDevice::Cuda);
                assert_eq!(options.compute_type.as_deref(), Some("int8"));
                assert_eq!(options.batch_size, Some(8));
                assert!(options.no_align);
                assert_eq!(options.align_model.as_deref(), Some("external-align"));
                assert_eq!(options.model_dir, Some(PathBuf::from("model-cache")));
                assert!(!options.model_cache_only);
                assert!(options.return_char_alignments);
                assert!(!options.diarize);
                assert!(contains_pair(
                    &options.extra_args,
                    "--model_cache_only",
                    "True"
                ));
                assert!(contains_pair(&options.extra_args, "--device_index", "0"));
                assert!(contains_pair(&options.extra_args, "--vad_method", "silero"));
                assert!(contains_pair(&options.extra_args, "--vad_onset", "0.5"));
                assert!(contains_pair(&options.extra_args, "--vad_offset", "0.363"));
                assert!(contains_pair(&options.extra_args, "--chunk_size", "20"));
                assert!(contains_pair(&options.extra_args, "--temperature", "0,0.2"));
                assert!(contains_pair(&options.extra_args, "--best_of", "3"));
                assert!(contains_pair(&options.extra_args, "--beam_size", "5"));
                assert!(contains_pair(&options.extra_args, "--patience", "1.2"));
                assert!(contains_pair(
                    &options.extra_args,
                    "--length_penalty",
                    "1.1"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--suppress_tokens",
                    "-1"
                ));
                assert!(options
                    .extra_args
                    .contains(&"--suppress_numerals".to_string()));
                assert!(contains_pair(
                    &options.extra_args,
                    "--initial_prompt",
                    "domain prompt"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--hotwords",
                    "proper nouns"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--condition_on_previous_text",
                    "false"
                ));
                assert!(contains_pair(&options.extra_args, "--fp16", "false"));
                assert!(contains_pair(
                    &options.extra_args,
                    "--compression_ratio_threshold",
                    "2.4"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--logprob_threshold",
                    "-1"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--no_speech_threshold",
                    "0.6"
                ));
                assert!(contains_pair(&options.extra_args, "--threads", "4"));
                assert!(options.extra_args.contains(&"--diarize".to_string()));
                assert!(contains_pair(
                    &options.extra_args,
                    "--diarize_model",
                    "pyannote/speaker-diarization-community-1"
                ));
                assert!(contains_pair(&options.extra_args, "--hf_token", "token"));
                assert!(options
                    .extra_args
                    .contains(&"--speaker_embeddings".to_string()));
                assert!(contains_pair(&options.extra_args, "--max_line_width", "42"));
                assert!(contains_pair(&options.extra_args, "--max_line_count", "2"));
                assert!(contains_pair(
                    &options.extra_args,
                    "--highlight_words",
                    "True"
                ));
                assert!(contains_pair(
                    &options.extra_args,
                    "--segment_resolution",
                    "chunk"
                ));
            }
            other => panic!("expected external provider, got {other:?}"),
        }
    }

    #[test]
    fn maps_external_silero_still_delegated() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                provider: AsrProvider::ExternalWhisperX,
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: VadConfig {
                method: VadMethod::Silero,
                model_bundle: Some(PathBuf::from("native-only/silero_vad.onnx")),
                model_file: Some("ignored.onnx".to_string()),
                ..VadConfig::default()
            },
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        })
        .expect("external silero should build");

        match request.provider {
            TranscriptionProviderSelection::ExternalWhisperX(options) => {
                assert!(contains_pair(&options.extra_args, "--vad_method", "silero"));
                assert!(!options
                    .extra_args
                    .iter()
                    .any(|arg| arg.contains("vad_model")));
            }
            other => panic!("expected external provider, got {other:?}"),
        }
    }

    #[test]
    fn imports_whisperx_fixture() {
        let transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        assert_eq!(transcript.language.as_deref(), Some("en"));
        assert_eq!(transcript.segments.len(), 2);
        assert_eq!(transcript.text_or_joined(), "hello world second speaker");
    }

    #[test]
    fn correct_speaker_persists_confirmed_profile_and_writes_corrected_output() {
        let temp = tempfile::tempdir().expect("tempdir");
        let speaker_directory = temp.path().join("speakers");
        let output_dir = temp.path().join("out");

        let report = correct_speaker(SpeakerCorrectionRequest {
            transcript: correction_transcript(),
            audio: InputSource::Samples {
                samples: correction_samples(),
                sample_rate: 16_000,
                channels: 1,
                source: Some("sample.wav".to_string()),
            },
            from_speaker: "speaker_0".to_string(),
            to_label: "Alice".to_string(),
            speaker_id: Some("alice".to_string()),
            ranges: Vec::new(),
            speaker_directory: SpeakerDirectorySelection {
                scope: SpeakerDirectoryScope::Auto,
                explicit_path: Some(speaker_directory.clone()),
            },
            output: OutputConfig {
                output_dir: Some(output_dir.clone()),
                basename: Some("sample.corrected".to_string()),
                formats: vec![OutputFormat::Json],
                ..OutputConfig::default()
            },
        })
        .expect("correction should succeed");

        assert_eq!(report.profile_id, "alice");
        assert_eq!(report.label, "Alice");
        assert_eq!(report.corrected_from, "speaker_0");
        assert!(!report.updated_existing_profile);
        assert!(report.enrolled_seconds > 0.9);
        assert_eq!(
            report.transcript.segments[0].speaker.as_deref(),
            Some("Alice")
        );
        assert_eq!(
            report.transcript.segments[1].speaker.as_deref(),
            Some("speaker_1")
        );
        assert!(speaker_library_path(&speaker_directory).is_file());
        let library =
            fs::read_to_string(speaker_library_path(&speaker_directory)).expect("library");
        assert!(library.contains(r#""id": "alice""#));
        assert!(library.contains(r#""status": "confirmed""#));
        let corrected = output_dir.join("sample.corrected.json");
        assert!(corrected.is_file());
        assert!(fs::read_to_string(corrected)
            .expect("corrected")
            .contains("Alice"));
    }

    #[test]
    fn correct_speaker_range_limits_relabeling() {
        let temp = tempfile::tempdir().expect("tempdir");
        let speaker_directory = temp.path().join("speakers");
        let mut transcript = correction_transcript();
        transcript.segments[1].speaker = Some("speaker_0".to_string());

        let report = correct_speaker(SpeakerCorrectionRequest {
            transcript,
            audio: InputSource::Samples {
                samples: correction_samples(),
                sample_rate: 16_000,
                channels: 1,
                source: Some("sample.wav".to_string()),
            },
            from_speaker: "speaker_0".to_string(),
            to_label: "Alice".to_string(),
            speaker_id: Some("alice".to_string()),
            ranges: vec![SpeakerCorrectionRange {
                start_seconds: 0.0,
                end_seconds: 1.0,
            }],
            speaker_directory: SpeakerDirectorySelection {
                scope: SpeakerDirectoryScope::Auto,
                explicit_path: Some(speaker_directory),
            },
            output: OutputConfig::default(),
        })
        .expect("correction should succeed");

        assert_eq!(
            report.transcript.segments[0].speaker.as_deref(),
            Some("Alice")
        );
        assert_eq!(
            report.transcript.segments[1].speaker.as_deref(),
            Some("speaker_0")
        );
    }

    #[test]
    fn correct_speaker_rejects_empty_selected_audio() {
        let temp = tempfile::tempdir().expect("tempdir");
        let error = correct_speaker(SpeakerCorrectionRequest {
            transcript: correction_transcript(),
            audio: InputSource::Samples {
                samples: correction_samples(),
                sample_rate: 16_000,
                channels: 1,
                source: Some("sample.wav".to_string()),
            },
            from_speaker: "missing".to_string(),
            to_label: "Alice".to_string(),
            speaker_id: Some("alice".to_string()),
            ranges: Vec::new(),
            speaker_directory: SpeakerDirectorySelection {
                scope: SpeakerDirectoryScope::Auto,
                explicit_path: Some(temp.path().join("speakers")),
            },
            output: OutputConfig::default(),
        })
        .expect_err("missing source speaker should fail");

        assert!(error
            .to_string()
            .contains("found no timed transcript segments"));
    }

    #[test]
    fn writes_requested_outputs() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        let files = write_outputs_with_options(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![
                    OutputFormat::Json,
                    OutputFormat::NativeJson,
                    OutputFormat::Srt,
                    OutputFormat::Vtt,
                    OutputFormat::Txt,
                    OutputFormat::Tsv,
                    OutputFormat::Audacity,
                ],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig::default(),
            },
            true,
        )
        .expect("outputs should write");

        assert_eq!(files.len(), 7);
        let json_path = temp.path().join("sample.json");
        let native_json_path = temp.path().join("sample.native.json");
        let srt_path = temp.path().join("sample.srt");
        let vtt_path = temp.path().join("sample.vtt");
        let txt_path = temp.path().join("sample.txt");
        let tsv_path = temp.path().join("sample.tsv");
        let aud_path = temp.path().join("sample.aud");
        assert!(json_path.is_file());
        assert!(native_json_path.is_file());
        assert!(srt_path.is_file());
        assert!(vtt_path.is_file());
        assert!(txt_path.is_file());
        assert!(tsv_path.is_file());
        assert!(aud_path.is_file());

        let whisperx_json: serde_json::Value =
            serde_json::from_slice(&fs::read(json_path).expect("json"))
                .expect("valid whisperx json");
        assert!(whisperx_json.get("segments").is_some());
        assert!(whisperx_json.get("word_segments").is_some());
        assert!(whisperx_json["segments"][0].get("start").is_some());
        assert!(whisperx_json["segments"][0].get("end").is_some());
        assert!(whisperx_json["segments"][0].get("startSeconds").is_none());
        assert_eq!(whisperx_json["segments"][0]["chars"][0]["char"], "h");

        let native_json: serde_json::Value =
            serde_json::from_slice(&fs::read(native_json_path).expect("native json"))
                .expect("valid native json");
        assert!(native_json["segments"][0].get("startSeconds").is_some());
        assert!(native_json["segments"][0].get("chars").is_some());

        let txt = fs::read_to_string(txt_path).expect("txt");
        assert_eq!(
            txt,
            "[SPEAKER_00]: hello world\n[SPEAKER_01]: second speaker\n"
        );
        let srt = fs::read_to_string(srt_path).expect("srt");
        assert!(srt.contains("00:00:00,000 --> 00:00:01,100"));
        assert!(srt.contains("[SPEAKER_00]: hello world"));
        let vtt = fs::read_to_string(vtt_path).expect("vtt");
        assert!(vtt.starts_with("WEBVTT\n\n"));
        assert!(vtt.contains("00:01.350 --> 00:02.350"));
        assert!(vtt.contains("[SPEAKER_01]: second speaker"));
        let tsv = fs::read_to_string(tsv_path).expect("tsv");
        assert!(tsv.starts_with("start\tend\ttext\n"));
        assert!(tsv.contains("0\t1200\thello world"));
        assert!(tsv.contains("1350\t2400\tsecond speaker"));
        let aud = fs::read_to_string(aud_path).expect("aud");
        assert!(aud.contains("0\t1.2\t[[SPEAKER_00]]hello world"));
        assert!(aud.contains("1.35\t2.4\t[[SPEAKER_01]]second speaker"));
    }

    #[test]
    fn all_format_writes_whisperx_compatible_set_without_native_json() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs_with_options(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::All],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig::default(),
            },
            true,
        )
        .expect("outputs should write");

        let mut names = fs::read_dir(temp.path())
            .expect("read output dir")
            .map(|entry| {
                entry
                    .expect("dir entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<_>>();
        names.sort();
        assert_eq!(
            names,
            vec![
                "sample.aud",
                "sample.json",
                "sample.srt",
                "sample.tsv",
                "sample.txt",
                "sample.vtt",
            ]
        );
    }

    #[test]
    fn output_stems_keep_multi_input_writes_collision_safe() {
        let temp = tempfile::tempdir().expect("tempdir");
        let mut first = fixture_response_with_chars();
        first.transcript.source = Some("audio/first-input.wav".to_string());
        let mut second = fixture_response_with_chars();
        second.transcript.source = Some("audio/second-input.wav".to_string());
        let output = OutputConfig {
            output_dir: Some(temp.path().to_path_buf()),
            formats: vec![OutputFormat::All],
            basename: None,
            pretty_json: true,
            subtitles: SubtitleConfig::default(),
        };

        write_outputs_with_options(&first, &output, true).expect("first outputs should write");
        write_outputs_with_options(&second, &output, true).expect("second outputs should write");

        let mut names = fs::read_dir(temp.path())
            .expect("read output dir")
            .map(|entry| {
                entry
                    .expect("dir entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<_>>();
        names.sort();

        assert_eq!(
            names,
            vec![
                "first-input.aud",
                "first-input.json",
                "first-input.srt",
                "first-input.tsv",
                "first-input.txt",
                "first-input.vtt",
                "second-input.aud",
                "second-input.json",
                "second-input.srt",
                "second-input.tsv",
                "second-input.txt",
                "second-input.vtt",
            ]
        );
    }

    #[test]
    fn txt_writes_each_segment_without_speakers() {
        let mut response = fixture_response_with_chars();
        for segment in &mut response.transcript.segments {
            segment.speaker = None;
        }
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Txt],
                basename: Some("sample".to_string()),
                ..OutputConfig::default()
            },
        )
        .expect("txt should write");

        let txt = fs::read_to_string(temp.path().join("sample.txt")).expect("txt");
        assert_eq!(txt, "hello world\nsecond speaker\n");
    }

    #[test]
    fn tsv_includes_header_and_replaces_tabs() {
        let mut response = fixture_response_with_chars();
        response.transcript.segments[0].text = " hello\tworld ".to_string();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Tsv],
                basename: Some("sample".to_string()),
                ..OutputConfig::default()
            },
        )
        .expect("tsv should write");

        let tsv = fs::read_to_string(temp.path().join("sample.tsv")).expect("tsv");
        assert!(tsv.starts_with("start\tend\ttext\n"));
        assert!(tsv.contains("0\t1200\thello world\n"));
    }

    #[test]
    fn subtitle_options_highlight_and_wrap_text() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Srt, OutputFormat::Vtt],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig {
                    max_line_width: Some(8),
                    max_line_count: None,
                    highlight_words: true,
                    segment_resolution: SegmentResolution::Sentence,
                },
            },
        )
        .expect("subtitles should write");

        let srt = fs::read_to_string(temp.path().join("sample.srt")).expect("srt");
        assert!(srt.contains("<u>hello</u>"));
        assert!(srt.contains("[SPEAKER_00]: <u>hello</u> \nworld"));
        assert!(srt.contains("[SPEAKER_00]: hello \n<u>world</u>"));
        let vtt = fs::read_to_string(temp.path().join("sample.vtt")).expect("vtt");
        assert!(vtt.contains("<u>hello</u>"));
    }

    #[test]
    fn subtitle_max_line_count_merges_overflow() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Srt],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig {
                    max_line_width: Some(8),
                    max_line_count: Some(1),
                    highlight_words: false,
                    segment_resolution: SegmentResolution::Sentence,
                },
            },
        )
        .expect("subtitles should write");

        let srt = fs::read_to_string(temp.path().join("sample.srt")).expect("srt");
        assert!(srt.contains("[SPEAKER_00]: hello\n\n2"));
        assert!(srt.contains("[SPEAKER_00]: world\n\n3"));
        assert!(srt.contains("[SPEAKER_01]: second\n\n4"));
        assert!(srt.contains("[SPEAKER_01]: speaker\n\n"));
    }

    #[test]
    fn subtitle_word_cues_join_languages_without_spaces() {
        let mut response = fixture_response_with_chars();
        response.transcript.language = Some("ja".to_string());
        response.transcript.segments[0].speaker = None;
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Srt],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig::default(),
            },
        )
        .expect("subtitles should write");

        let srt = fs::read_to_string(temp.path().join("sample.srt")).expect("srt");
        assert!(srt.contains("helloworld"));
    }

    #[test]
    fn whisperx_json_omits_chars_when_not_requested() {
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

        let without_chars = whisperx_json_value(&transcript, false);
        let with_chars = whisperx_json_value(&transcript, true);

        assert!(without_chars["segments"][0].get("chars").is_none());
        assert!(with_chars["segments"][0].get("chars").is_some());
    }

    #[test]
    fn parity_comparison_accepts_permutation_equivalent_speaker_turns() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.segments[0].speaker = Some("reference-speaker-b".to_string());
        whisperx.segments[1].speaker = Some("reference-speaker-a".to_string());

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig {
                text: false,
                language: false,
                segment_text: false,
                word_text: false,
                char_count: false,
                char_content: false,
                segment_count: false,
                word_count: false,
                segment_timing: false,
                word_timing: false,
                speaker_turns: true,
                vad_segments: false,
                vad_segment_timing: false,
                vad_segment_count: false,
            },
        );

        assert!(comparison.speaker_turns_match);
        assert!(comparison.passed);
    }

    #[test]
    fn parity_comparison_reports_text_language_word_and_char_categories() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.language = Some("de".to_string());
        whisperx.segments[0].text = "hello changed".to_string();
        whisperx.segments[0].words[0].text = "changed".to_string();
        whisperx.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig::default(),
        );

        assert_eq!(comparison.language_matches, Some(false));
        assert_eq!(comparison.segment_text_matches, Some(false));
        assert_eq!(comparison.word_text_matches, Some(false));
        assert_eq!(comparison.char_count_matches, Some(false));
        assert_eq!(comparison.char_content_matches, Some(false));
        assert!(!comparison.passed);
        for expected in [
            "language differs: native=Some(\"en\") reference=Some(\"de\")",
            "segment text sequence differs: native=[\"hello world\", \"second speaker\"] reference=[\"hello changed\", \"second speaker\"]",
            "word text sequence differs: native=[\"hello\", \"world\", \"second\", \"speaker\"] reference=[\"changed\", \"world\", \"second\", \"speaker\"]",
            "char alignment count differs",
            "char alignment content differs",
        ] {
            assert!(
                comparison
                    .differences
                    .iter()
                    .any(|difference| difference.contains(expected)),
                "comparison should report `{expected}`: {:?}",
                comparison.differences
            );
        }
    }

    #[test]
    fn parity_comparison_fails_character_content_mismatches() {
        let mut native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        native.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });
        let mut whisperx = native.clone();
        whisperx.segments[0].chars[0].character = "x".to_string();

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig::default(),
        );

        assert_eq!(comparison.char_count_matches, Some(true));
        assert_eq!(comparison.char_content_matches, Some(false));
        assert!(!comparison.passed);
        assert!(comparison
            .differences
            .iter()
            .any(|difference| { difference.contains("char alignment content differs at char 0") }));
    }

    #[test]
    fn parity_comparison_config_defaults_to_strict() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav"
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");
        let parity_config: ParityConfig =
            serde_json::from_str(r#"{"input":"audio/input.wav"}"#).expect("config should parse");

        assert_eq!(
            fixture_suite.fixtures[0].comparison,
            ParityComparisonConfig::default()
        );
        assert_eq!(parity_config.comparison, ParityComparisonConfig::default());
    }

    #[test]
    fn parity_comparison_config_can_make_timing_report_only() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.segments[0].start_seconds = Some(4.0);
        whisperx.segments[0].words[0].start_seconds = Some(4.0);
        let config = ParityComparisonConfig {
            segment_timing: false,
            word_timing: false,
            ..ParityComparisonConfig::default()
        };

        let comparison =
            compare_transcripts(&native, &whisperx, ParityTolerance::default(), &config);

        assert!(!comparison.segment_timing_matches);
        assert!(!comparison.word_timing_matches);
        assert!(comparison.passed);
        let segment_difference = comparison
            .differences
            .iter()
            .find(|difference| {
                difference.starts_with("report-only: segment timing differs at segment 0")
            })
            .expect("segment timing difference should be reported");
        assert!(segment_difference.contains("native start="));
        assert!(segment_difference.contains("reference start=4.000s"));
        assert!(segment_difference.contains("start_delta="));
        assert!(segment_difference.contains("tolerance=0.100s"));

        let word_difference = comparison
            .differences
            .iter()
            .find(|difference| difference.starts_with("report-only: word timing differs at word 0"))
            .expect("word timing difference should be reported");
        assert!(word_difference.contains("native start="));
        assert!(word_difference.contains("reference start=4.000s"));
        assert!(word_difference.contains("start_delta="));
        assert!(word_difference.contains("tolerance=0.050s"));
    }

    #[test]
    fn parity_comparison_strict_timing_differences_fail_with_numeric_deltas() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.segments[0].start_seconds = Some(4.0);
        whisperx.segments[0].end_seconds = Some(5.0);
        whisperx.segments[0].words[0].start_seconds = Some(4.0);
        whisperx.segments[0].words[0].end_seconds = Some(4.5);

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig::default(),
        );

        assert!(!comparison.segment_timing_matches);
        assert!(!comparison.word_timing_matches);
        assert!(!comparison.passed);
        let segment_difference = comparison
            .differences
            .iter()
            .find(|difference| difference.starts_with("segment timing differs at segment 0"))
            .expect("segment timing difference should be reported");
        assert!(segment_difference.contains("native start="));
        assert!(segment_difference.contains("native end="));
        assert!(segment_difference.contains("reference start=4.000s"));
        assert!(segment_difference.contains("reference end=5.000s"));
        assert!(segment_difference.contains("start_delta="));
        assert!(segment_difference.contains("end_delta="));
        assert!(segment_difference.contains("tolerance=0.100s"));

        let word_difference = comparison
            .differences
            .iter()
            .find(|difference| difference.starts_with("word timing differs at word 0"))
            .expect("word timing difference should be reported");
        assert!(word_difference.contains("native start="));
        assert!(word_difference.contains("native end="));
        assert!(word_difference.contains("reference start=4.000s"));
        assert!(word_difference.contains("reference end=4.500s"));
        assert!(word_difference.contains("start_delta="));
        assert!(word_difference.contains("end_delta="));
        assert!(word_difference.contains("tolerance=0.050s"));
    }

    #[test]
    fn fixture_suite_keeps_report_only_differences_visible() {
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", true, "audio/input.wav")],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |_| {
            let mut report = fixture_parity_report();
            report.comparison.segment_timing_matches = false;
            report.comparison.differences =
                vec!["report-only: segment timing differs at segment 0".to_string()];
            Ok(report)
        })
        .expect("suite should run");

        assert!(report.passed);
        assert!(report.cases[0].passed);
        assert!(report.cases[0]
            .failure_summary
            .iter()
            .any(|difference| difference == "report-only: segment timing differs at segment 0"));
    }

    #[test]
    fn parity_fixture_manifest_accepts_comparison_config() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "comparison": {
                    "segmentTiming": false,
                    "charContent": false
                  }
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        assert!(!fixture_suite.fixtures[0].comparison.segment_timing);
        assert!(!fixture_suite.fixtures[0].comparison.char_content);
        assert!(fixture_suite.fixtures[0].comparison.word_timing);
        assert!(fixture_suite.fixtures[0].comparison.char_count);
    }

    #[test]
    fn parity_fixture_manifest_accepts_expected_target() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "expectedTarget": "whisperx"
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        assert_eq!(
            fixture_suite.fixtures[0].expected_target,
            ExpectedTranscriptTarget::Whisperx
        );
    }

    #[test]
    fn legacy_fixture_expected_target_defaults_to_native() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav"
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        assert_eq!(
            fixture_suite.fixtures[0].expected_target,
            ExpectedTranscriptTarget::Native
        );
    }

    #[test]
    fn compare_with_whisperx_expected_target_uses_whisperx_transcript() {
        let expected = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let whisperx = expected.clone();
        let mut native = expected.clone();
        native.segments[0].text = "native transcript mismatch".to_string();
        native.segments.pop();

        let (expected_segment_count_matches, expected_text_matches) = expected_transcript_matches(
            Some(&expected),
            ExpectedTranscriptTarget::Whisperx,
            &native,
            &whisperx,
        );

        assert_eq!(expected_text_matches, Some(true));
        assert_eq!(expected_segment_count_matches, Some(true));

        let mut report = fixture_parity_report();
        report.expected_target = ExpectedTranscriptTarget::Whisperx;
        report.expected_text_matches = expected_text_matches;
        report.expected_segment_count_matches = expected_segment_count_matches;
        report.comparison.differences =
            vec!["report-only: native transcript differs from WhisperX transcript".to_string()];

        assert!(parity_fixture_case_passed(&report, &[], &[]));
    }

    #[test]
    fn parity_fixture_manifest_accepts_whisperx_diarization_config() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "diarization": {
                    "enabled": true,
                    "modelId": "native-spectral-speaker-baseline"
                  },
                  "whisperxDiarization": {
                    "enabled": true,
                    "modelId": "pyannote/speaker-diarization-community-1",
                    "hfTokenEnv": "HF_TOKEN",
                    "returnSpeakerEmbeddings": true
                  }
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        let fixture = &fixture_suite.fixtures[0];
        assert_eq!(
            fixture.diarization.model_id,
            "native-spectral-speaker-baseline"
        );
        let whisperx_diarization = fixture
            .whisperx_diarization
            .as_ref()
            .expect("whisperx diarization config");
        assert_eq!(
            whisperx_diarization.model_id,
            "pyannote/speaker-diarization-community-1"
        );
        assert_eq!(
            whisperx_diarization.hf_token_env.as_deref(),
            Some("HF_TOKEN")
        );
        assert!(whisperx_diarization.return_speaker_embeddings);
    }

    #[test]
    fn parity_fixture_manifest_without_whisperx_diarization_keeps_shared_behavior() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "diarization": {
                    "enabled": true,
                    "modelId": "legacy-shared-model"
                  }
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        let fixture = &fixture_suite.fixtures[0];
        assert_eq!(fixture.diarization.model_id, "legacy-shared-model");
        assert!(fixture.whisperx_diarization.is_none());
    }

    #[test]
    fn diagnostic_comparison_reports_provider_specific_entries() {
        let differences = compare_diagnostics(
            &["shared".to_string(), "native-only".to_string()],
            &["shared".to_string(), "whisperx-only".to_string()],
        );

        assert_eq!(
            differences,
            vec![
                "native diagnostic only: native-only".to_string(),
                "whisperx diagnostic only: whisperx-only".to_string()
            ]
        );
    }

    #[test]
    fn output_comparison_reports_exact_json_and_missing_outputs() {
        let temp = tempfile::tempdir().expect("tempdir");
        let expected_txt = temp.path().join("expected.txt");
        let actual_txt = temp.path().join("actual.txt");
        let expected_json = temp.path().join("expected.json");
        let actual_json = temp.path().join("actual.json");
        let missing_expected = temp.path().join("missing.srt");
        let actual_srt = temp.path().join("actual.srt");
        fs::write(&expected_txt, "hello\n").expect("expected txt");
        fs::write(&actual_txt, "hello changed\n").expect("actual txt");
        fs::write(&expected_json, "{\n  \"a\": 1\n}\n").expect("expected json");
        fs::write(&actual_json, "{\"a\":1}").expect("actual json");
        fs::write(&actual_srt, "1\n").expect("actual srt");

        let actual_outputs = vec![
            OutputFile {
                format: OutputFormat::Txt,
                path: actual_txt,
            },
            OutputFile {
                format: OutputFormat::Json,
                path: actual_json,
            },
            OutputFile {
                format: OutputFormat::Srt,
                path: actual_srt,
            },
        ];
        let comparisons = compare_expected_outputs(
            &actual_outputs,
            &[
                ExpectedOutputFile {
                    format: OutputFormat::Txt,
                    path: expected_txt,
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                },
                ExpectedOutputFile {
                    format: OutputFormat::Json,
                    path: expected_json,
                    comparison: OutputComparisonMode::JsonSemantic,
                    gating: true,
                },
                ExpectedOutputFile {
                    format: OutputFormat::Vtt,
                    path: temp.path().join("expected.vtt"),
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                },
                ExpectedOutputFile {
                    format: OutputFormat::Srt,
                    path: missing_expected,
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                },
            ],
        )
        .expect("comparison should run");

        assert!(!comparisons[0].passed);
        assert!(comparisons[0]
            .difference
            .as_deref()
            .is_some_and(|difference| difference.contains("line 1 differs")));
        assert!(comparisons[1].passed);
        assert!(!comparisons[2].passed);
        assert!(comparisons[2]
            .difference
            .as_deref()
            .is_some_and(|difference| difference.contains("missing actual")));
        assert!(!comparisons[3].passed);
        assert!(comparisons[3]
            .difference
            .as_deref()
            .is_some_and(|difference| difference.contains("missing expected")));
    }

    #[test]
    fn output_json_semantic_compares_whisperx_transcript_contract() {
        let difference =
            compare_json_output_values(semantic_expected_whisperx_json(), semantic_actual_json());

        assert_eq!(difference, None);
    }

    #[test]
    fn output_json_semantic_fails_changed_word_text() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["word_segments"][1]["word"] = serde_json::json!("planet");

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript word 1 text differs"));
    }

    #[test]
    fn output_json_semantic_fails_word_timing_beyond_tolerance() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["word_segments"][0]["start"] = serde_json::json!(0.200);

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript word 0 start timing differs"));
        assert!(difference.contains("tolerance=0.050s"));
    }

    #[test]
    fn output_json_semantic_fails_segment_timing_beyond_tolerance() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["segments"][0]["end"] = serde_json::json!(1.500);

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript segment 0 end timing differs"));
        assert!(difference.contains("tolerance=0.100s"));
    }

    #[test]
    fn output_json_semantic_fails_char_count_mismatch_when_chars_present() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["segments"][0]["chars"] = serde_json::json!([
            {
                "char": "h",
                "start": 0.002,
                "end": 0.098
            }
        ]);

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript char count differs"));
    }

    #[test]
    fn parity_fixture_fails_failed_output_comparison() {
        let report = fixture_parity_report();
        let failed_outputs = vec![ExpectedOutputComparison {
            format: OutputFormat::Txt,
            comparison: OutputComparisonMode::Exact,
            gating: true,
            expected_path: PathBuf::from("expected.txt"),
            actual_path: Some(PathBuf::from("actual.txt")),
            passed: false,
            difference: Some("line 1 differs".to_string()),
        }];

        assert!(!parity_fixture_case_passed(&report, &[], &failed_outputs));
    }

    #[test]
    fn preflight_resolves_relative_manifest_paths_under_root() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path();
        fs::create_dir_all(root.join("audio")).expect("audio");
        fs::create_dir_all(root.join("models")).expect("models");
        fs::write(root.join("audio/input.wav"), b"audio").expect("input");

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![minimal_fixture("case", true, "audio/input.wav")],
            },
            root.join("fixtures.json"),
            root.to_path_buf(),
            PathBuf::from("/bin/true"),
            root.join("models"),
            false,
            false,
        );

        assert!(!report.cases[0]
            .missing
            .iter()
            .any(|missing| missing.contains("input")));
    }

    #[test]
    fn preflight_reports_missing_input() {
        let temp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(temp.path().join("models")).expect("models");

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![minimal_fixture("case", true, "audio/missing.wav")],
            },
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            false,
            false,
        );

        assert!(!report.cases[0].passed);
        assert!(report.cases[0]
            .missing
            .iter()
            .any(|missing| missing.contains("audio/missing.wav")));
    }

    #[test]
    fn preflight_reports_missing_expected_output_when_required() {
        let temp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(temp.path().join("audio")).expect("audio");
        fs::create_dir_all(temp.path().join("models")).expect("models");
        fs::write(temp.path().join("audio/input.wav"), b"audio").expect("input");
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.expected_outputs.push(ExpectedOutputFile {
            format: OutputFormat::Srt,
            path: PathBuf::from("expected/missing.srt"),
            comparison: OutputComparisonMode::Exact,
            gating: true,
        });

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![fixture],
            },
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            true,
            false,
        );

        assert!(report.cases[0]
            .missing
            .iter()
            .any(|missing| missing.contains("expected/missing.srt")));
    }

    #[test]
    fn preflight_ignores_missing_non_gating_resources_unless_included() {
        let temp = tempfile::tempdir().expect("tempdir");
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", false, "audio/missing.wav")],
        };

        let ignored = run_parity_preflight(
            suite.clone(),
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            false,
            false,
        );
        assert!(ignored.passed);
        assert!(ignored.cases[0].missing.is_empty());
        assert!(!ignored.cases[0].warnings.is_empty());

        let included = run_parity_preflight(
            suite,
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            false,
            true,
        );
        assert!(!included.passed);
        assert!(!included.cases[0].missing.is_empty());
    }

    #[test]
    fn preflight_reports_missing_onnx_runtime_for_onnx_diarization() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path();
        fs::create_dir_all(root.join("audio")).expect("audio");
        fs::create_dir_all(root.join("models/diarization")).expect("diarization model bundle");
        fs::create_dir_all(root.join("models")).expect("models");
        fs::write(root.join("audio/input.wav"), b"audio").expect("input");
        for file in [
            "pyannote_diarization_manifest.json",
            "segmentation.onnx",
            "embedding.onnx",
            "plda_transform.json",
            "plda_model.json",
            "clustering.json",
        ] {
            fs::write(root.join("models/diarization").join(file), b"model").expect(file);
        }
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.diarization = DiarizationConfig {
            enabled: true,
            model_bundle: Some(PathBuf::from("models/diarization")),
            ..DiarizationConfig::default()
        };
        let previous_ort = std::env::var_os("ORT_DYLIB_PATH");
        std::env::set_var("ORT_DYLIB_PATH", root.join("missing-onnxruntime.so"));

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![fixture],
            },
            root.join("fixtures.json"),
            root.to_path_buf(),
            PathBuf::from("/bin/true"),
            root.join("models"),
            false,
            false,
        );

        if let Some(previous_ort) = previous_ort {
            std::env::set_var("ORT_DYLIB_PATH", previous_ort);
        } else {
            std::env::remove_var("ORT_DYLIB_PATH");
        }
        assert!(!report.cases[0].passed);
        assert!(report.cases[0]
            .missing
            .iter()
            .any(|missing| missing == "ORT_DYLIB_PATH is not set to an existing file"));
    }

    #[test]
    fn preflight_skips_hf_token_env_when_diarization_is_disabled() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path();
        fs::create_dir_all(root.join("audio")).expect("audio");
        fs::create_dir_all(root.join("models")).expect("models");
        fs::write(root.join("audio/input.wav"), b"audio").expect("input");
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.diarization = DiarizationConfig {
            enabled: false,
            model_id: "pyannote/speaker-diarization-community-1".to_string(),
            hf_token_env: Some("NATIVE_WHISPERX_TEST_MISSING_HF_TOKEN".to_string()),
            ..DiarizationConfig::default()
        };
        std::env::remove_var("NATIVE_WHISPERX_TEST_MISSING_HF_TOKEN");

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![fixture],
            },
            root.join("fixtures.json"),
            root.to_path_buf(),
            PathBuf::from("/bin/true"),
            root.join("models"),
            false,
            false,
        );

        assert!(!report.cases[0]
            .missing
            .iter()
            .any(|missing| { missing.contains("NATIVE_WHISPERX_TEST_MISSING_HF_TOKEN") }));
    }

    #[test]
    fn fixture_suite_records_gating_case_error_and_fails_suite() {
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", true, "audio/input.wav")],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |_| {
            Err(NativeWhisperxError::InvalidConfig(
                "setup failed".to_string(),
            ))
        })
        .expect("suite should not abort");

        assert!(!report.passed);
        assert!(!report.cases[0].passed);
        assert_eq!(
            report.cases[0].error.as_deref(),
            Some("invalid configuration: setup failed")
        );
    }

    #[test]
    fn fixture_suite_passes_separate_whisperx_diarization_config() {
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.diarization = DiarizationConfig {
            enabled: true,
            model_id: "native-spectral-speaker-baseline".to_string(),
            min_speakers: Some(2),
            max_speakers: Some(2),
            ..DiarizationConfig::default()
        };
        fixture.whisperx_diarization = Some(DiarizationConfig {
            enabled: true,
            model_id: "pyannote/speaker-diarization-community-1".to_string(),
            hf_token_env: Some("HF_TOKEN".to_string()),
            return_speaker_embeddings: true,
            min_speakers: Some(2),
            max_speakers: Some(2),
            ..DiarizationConfig::default()
        });
        let suite = ParityFixtureSuite {
            fixtures: vec![fixture],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |config| {
            assert_eq!(
                config.diarization.model_id,
                "native-spectral-speaker-baseline"
            );
            let whisperx_diarization = config
                .whisperx_diarization
                .expect("whisperx diarization config");
            assert_eq!(
                whisperx_diarization.model_id,
                "pyannote/speaker-diarization-community-1"
            );
            assert!(whisperx_diarization.return_speaker_embeddings);
            Ok(fixture_parity_report())
        })
        .expect("suite should run");

        assert!(report.passed);
    }

    #[test]
    fn fixture_suite_records_non_gating_case_error_and_keeps_suite_passed() {
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", false, "audio/input.wav")],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |_| {
            Err(NativeWhisperxError::InvalidConfig(
                "setup failed".to_string(),
            ))
        })
        .expect("suite should not abort");

        assert!(report.passed);
        assert!(!report.cases[0].passed);
        assert!(report.cases[0].error.is_some());
    }

    #[test]
    fn failure_summary_includes_output_diff_and_missing_diagnostics() {
        let report = fixture_parity_report();
        let summary = parity_fixture_failure_summary(
            Some(&report),
            &["asrModelSource=hugging-face-cache".to_string()],
            &[ExpectedOutputComparison {
                format: OutputFormat::Txt,
                comparison: OutputComparisonMode::Exact,
                gating: true,
                expected_path: PathBuf::from("expected.txt"),
                actual_path: Some(PathBuf::from("actual.txt")),
                passed: false,
                difference: Some("line 1 differs: expected \"a\", actual \"b\"".to_string()),
            }],
            None,
        );

        assert!(summary
            .iter()
            .any(|line| line.contains("missing required diagnostic")));
        assert!(summary.iter().any(|line| line.contains("line 1 differs")));
    }

    #[test]
    fn parity_fixture_resolves_relative_paths_against_root() {
        let fixture = resolve_fixture_case_paths(
            ParityFixtureCase {
                name: "case".to_string(),
                gating: true,
                input: PathBuf::from("audio/input.wav"),
                clip_seconds: None,
                timeout_seconds: None,
                expected_json: Some(PathBuf::from("expected/input.json")),
                expected_target: ExpectedTranscriptTarget::Native,
                comparison: ParityComparisonConfig::default(),
                expected_outputs: vec![ExpectedOutputFile {
                    format: OutputFormat::Srt,
                    path: PathBuf::from("expected/input.srt"),
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                }],
                native_asr: AsrConfig {
                    whisper_bundle: Some(PathBuf::from("models/whisper")),
                    model_dir: Some(PathBuf::from("models")),
                    external_whisperx: ExternalWhisperxConfig {
                        command: PathBuf::from("bin/whisperx"),
                        output_dir: Some(PathBuf::from("external-out")),
                        ..ExternalWhisperxConfig::default()
                    },
                    ..AsrConfig::default()
                },
                translation: TranslationConfig {
                    model_bundle: Some(PathBuf::from("models/translation")),
                    model_dir: Some(PathBuf::from("models")),
                    ..TranslationConfig::default()
                },
                vad: VadConfig {
                    model_bundle: Some(PathBuf::from("models/silero")),
                    ..VadConfig::default()
                },
                alignment: AlignmentConfig {
                    model_bundle: Some(PathBuf::from("models/wav2vec2")),
                    model_dir: Some(PathBuf::from("models")),
                    ..AlignmentConfig::default()
                },
                diarization: DiarizationConfig {
                    speaker_embedding_model_bundle: Some(PathBuf::from("models/speakers")),
                    ..DiarizationConfig::default()
                },
                whisperx_diarization: None,
                whisperx: ExternalWhisperxConfig {
                    command: PathBuf::from("bin/whisperx"),
                    output_dir: Some(PathBuf::from("whisperx-out")),
                    ..ExternalWhisperxConfig::default()
                },
                language: Some("en".to_string()),
                output: OutputConfig {
                    output_dir: Some(PathBuf::from("out")),
                    ..OutputConfig::default()
                },
                required_diagnostics: Vec::new(),
            },
            Some(Path::new("/smoke")),
        );

        assert_eq!(fixture.input, PathBuf::from("/smoke/audio/input.wav"));
        assert_eq!(
            fixture.expected_json,
            Some(PathBuf::from("/smoke/expected/input.json"))
        );
        assert_eq!(
            fixture.expected_outputs[0].path,
            PathBuf::from("/smoke/expected/input.srt")
        );
        assert_eq!(
            fixture.native_asr.whisper_bundle,
            Some(PathBuf::from("/smoke/models/whisper"))
        );
        assert_eq!(
            fixture.native_asr.external_whisperx.command,
            PathBuf::from("/smoke/bin/whisperx")
        );
        assert_eq!(
            fixture.translation.model_bundle,
            Some(PathBuf::from("/smoke/models/translation"))
        );
        assert_eq!(
            fixture.translation.model_dir,
            Some(PathBuf::from("/smoke/models"))
        );
        assert_eq!(
            fixture.vad.model_bundle,
            Some(PathBuf::from("/smoke/models/silero"))
        );
        assert_eq!(
            fixture.alignment.model_bundle,
            Some(PathBuf::from("/smoke/models/wav2vec2"))
        );
        assert_eq!(
            fixture.diarization.speaker_embedding_model_bundle,
            Some(PathBuf::from("/smoke/models/speakers"))
        );
        assert_eq!(
            fixture.whisperx.command,
            PathBuf::from("/smoke/bin/whisperx")
        );
        assert_eq!(fixture.output.output_dir, Some(PathBuf::from("/smoke/out")));
    }

    #[test]
    fn parity_fixture_reports_required_diagnostics() {
        let mut report = fixture_parity_report();
        report
            .native_report
            .response
            .diagnostics
            .push("asrModelSource=hugging-face-cache".to_string());

        let missing = missing_required_diagnostics(
            &report,
            &[
                "asrModelSource=hugging-face-cache".to_string(),
                "asrModelId=openai/whisper-tiny.en".to_string(),
            ],
        );

        assert_eq!(
            missing,
            vec!["asrModelId=openai/whisper-tiny.en".to_string()]
        );
        assert!(!parity_fixture_case_passed(&report, &missing, &[]));
    }

    #[test]
    fn parity_fixture_passes_when_comparison_expected_and_diagnostics_pass() {
        let mut report = fixture_parity_report();
        report.expected_text_matches = Some(true);
        report.expected_segment_count_matches = Some(true);
        report
            .native_report
            .response
            .diagnostics
            .push("asrModelSource=hugging-face-cache".to_string());

        let missing = missing_required_diagnostics(
            &report,
            &["asrModelSource=hugging-face-cache".to_string()],
        );

        assert!(missing.is_empty());
        assert!(parity_fixture_case_passed(&report, &missing, &[]));
    }

    #[test]
    fn parity_fixture_fails_expected_json_mismatches() {
        let mut report = fixture_parity_report();
        report.expected_text_matches = Some(false);
        report.expected_segment_count_matches = Some(true);

        assert!(!parity_fixture_case_passed(&report, &[], &[]));

        report.expected_text_matches = Some(true);
        report.expected_segment_count_matches = Some(false);

        assert!(!parity_fixture_case_passed(&report, &[], &[]));
    }

    #[test]
    fn parity_fixture_fails_failed_comparison() {
        let mut report = fixture_parity_report();
        report.comparison.passed = false;

        assert!(!parity_fixture_case_passed(&report, &[], &[]));
    }

    #[test]
    fn vad_segment_comparison_fails_count_mismatch() {
        let transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let config = ParityComparisonConfig::default();
        let mut comparison = compare_transcripts(
            &transcript,
            &transcript,
            ParityTolerance::default(),
            &config,
        );
        let native = vec![
            SpeechActivitySegment::new(0.0, 1.0, 0.9).unwrap(),
            SpeechActivitySegment::new(2.0, 3.0, 0.8).unwrap(),
        ];
        let whisperx = vec![SpeechActivitySegment::new(0.0, 1.0, 0.7).unwrap()];

        compare_vad_segments(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &config,
            &mut comparison,
        );

        assert_eq!(comparison.vad_segment_count_matches, Some(false));
        assert_eq!(comparison.vad_segment_timing_matches, Some(false));
        assert!(!comparison.passed);
        assert!(comparison
            .differences
            .iter()
            .any(|difference| difference.contains("VAD segment count differs")));
    }

    #[test]
    fn vad_segment_timing_can_be_report_only() {
        let transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let config = ParityComparisonConfig {
            vad_segment_timing: false,
            ..ParityComparisonConfig::default()
        };
        let mut comparison = compare_transcripts(
            &transcript,
            &transcript,
            ParityTolerance::default(),
            &config,
        );
        let native = vec![SpeechActivitySegment::new(0.0, 1.0, 0.9).unwrap()];
        let whisperx = vec![SpeechActivitySegment::new(0.25, 1.0, 0.7).unwrap()];

        compare_vad_segments(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &config,
            &mut comparison,
        );

        assert_eq!(comparison.vad_segment_count_matches, Some(true));
        assert_eq!(comparison.vad_segment_timing_matches, Some(false));
        assert!(comparison.passed);
        assert!(comparison.differences.iter().any(|difference| {
            difference.starts_with("report-only: VAD segment timing differs")
        }));
    }

    fn minimal_fixture(name: &str, gating: bool, input: &str) -> ParityFixtureCase {
        ParityFixtureCase {
            name: name.to_string(),
            gating,
            input: PathBuf::from(input),
            clip_seconds: None,
            timeout_seconds: None,
            expected_json: None,
            expected_target: ExpectedTranscriptTarget::Native,
            comparison: ParityComparisonConfig::default(),
            expected_outputs: Vec::new(),
            native_asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            whisperx_diarization: None,
            whisperx: ExternalWhisperxConfig::default(),
            language: None,
            output: OutputConfig::default(),
            required_diagnostics: Vec::new(),
        }
    }

    fn fixture_parity_report() -> ParityReport {
        let native_report = NativeWhisperxReport {
            response: fixture_response_with_chars(),
            output_files: Vec::new(),
        };
        let whisperx_report = native_report.clone();
        ParityReport {
            native_report,
            whisperx_report,
            expected: None,
            expected_target: ExpectedTranscriptTarget::Native,
            comparison: ParityComparison {
                text_matches: true,
                language_matches: Some(true),
                segment_text_matches: Some(true),
                word_text_matches: Some(true),
                char_count_matches: Some(true),
                char_content_matches: Some(true),
                segment_count_matches: true,
                word_count_matches: true,
                segment_timing_matches: true,
                word_timing_matches: true,
                speaker_turns_match: true,
                vad_segment_count_matches: None,
                vad_segment_timing_matches: None,
                confidence_compared: true,
                passed: true,
                tolerance: ParityTolerance::default(),
                differences: Vec::new(),
                diagnostic_differences: Vec::new(),
            },
            expected_segment_count_matches: None,
            expected_text_matches: None,
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

    fn correction_transcript() -> TranscriptionContract {
        let mut first = text_transcripts::TranscriptSegmentContract::new(0, "hello");
        first.start_seconds = Some(0.0);
        first.end_seconds = Some(1.0);
        first.speaker = Some("speaker_0".to_string());
        first.words.push(text_transcripts::TranscriptWordContract {
            text: "hello".to_string(),
            start_seconds: Some(0.1),
            end_seconds: Some(0.9),
            confidence: Some(0.9),
            speaker: Some("speaker_0".to_string()),
            attributes: Default::default(),
        });
        let mut second = text_transcripts::TranscriptSegmentContract::new(1, "world");
        second.start_seconds = Some(1.0);
        second.end_seconds = Some(2.0);
        second.speaker = Some("speaker_1".to_string());
        TranscriptionContract::new(vec![first, second])
    }

    fn correction_samples() -> Vec<f32> {
        let sample_rate = 16_000;
        let mut samples = vec![0.0_f32; sample_rate as usize * 2];
        sine_into(
            &mut samples[0..sample_rate as usize],
            sample_rate,
            0.0,
            220.0,
        );
        sine_into(
            &mut samples[sample_rate as usize..sample_rate as usize * 2],
            sample_rate,
            1.0,
            440.0,
        );
        samples
    }

    #[cfg(feature = "diarization")]
    fn two_speaker_loaded_audio() -> LoadedAudio {
        let sample_rate = 16_000;
        let mut samples = vec![0.0_f32; sample_rate as usize * 2];
        let first_start = (0.20 * sample_rate as f32) as usize;
        let first_end = (0.50 * sample_rate as f32) as usize;
        let second_start = (1.00 * sample_rate as f32) as usize;
        let second_end = (1.40 * sample_rate as f32) as usize;
        sine_into(
            &mut samples[first_start..first_end],
            sample_rate,
            0.20,
            220.0,
        );
        sine_into(
            &mut samples[second_start..second_end],
            sample_rate,
            1.00,
            1_200.0,
        );
        LoadedAudio {
            samples,
            sample_rate,
            channels: 1,
            source: Some("synthetic-two-speaker".to_string()),
        }
    }

    fn sine_into(samples: &mut [f32], sample_rate: u32, start_seconds: f32, freq_hz: f32) {
        for (offset, sample) in samples.iter_mut().enumerate() {
            let t = start_seconds + offset as f32 / sample_rate as f32;
            *sample = (2.0 * std::f32::consts::PI * freq_hz * t).sin() * 0.5;
        }
    }

    #[cfg(feature = "diarization")]
    fn speaker_library_matching_first_span(
        audio: &LoadedAudio,
    ) -> std::result::Result<SpeakerLibrary, Box<dyn std::error::Error>> {
        use audio_analysis_speakers::{
            SpeakerEmbeddingExtractor, SpeakerId, SpeakerLabel, SpeakerProfile,
        };

        let start = (0.20 * audio.sample_rate as f32) as usize;
        let end = (0.50 * audio.sample_rate as f32) as usize;
        let speaker_audio = SpeakerAudio::mono(&audio.samples[start..end], audio.sample_rate)?;
        let mut embedder = SpectralSpeakerEmbedder::default();
        let embedding = embedder.embed_speaker(&speaker_audio)?;
        let mut library = SpeakerLibrary::new();
        library.add_profile(
            SpeakerProfile::new(
                SpeakerId::new("known-speaker")?,
                SpeakerLabel::new("Known Speaker")?,
            )
            .with_embedding(embedding)?,
        )?;
        Ok(library)
    }

    #[cfg(feature = "diarization")]
    fn timed_transcript(
        words: Vec<(&str, f64, f64)>,
    ) -> std::result::Result<TranscriptionContract, Box<dyn std::error::Error>> {
        let mut segment = text_transcripts::TranscriptSegmentContract::new(
            0,
            words
                .iter()
                .map(|(word, _, _)| *word)
                .collect::<Vec<_>>()
                .join(" "),
        );
        segment.start_seconds = Some(0.0);
        segment.end_seconds = Some(2.0);
        segment.words = words
            .into_iter()
            .map(
                |(text, start_seconds, end_seconds)| text_transcripts::TranscriptWordContract {
                    text: text.to_string(),
                    start_seconds: Some(start_seconds),
                    end_seconds: Some(end_seconds),
                    confidence: None,
                    speaker: None,
                    attributes: Default::default(),
                },
            )
            .collect();
        Ok(TranscriptionContract::from_segments(
            None,
            Some("en".to_string()),
            vec![segment],
        )?)
    }

    fn compare_json_output_values(
        expected: serde_json::Value,
        actual: serde_json::Value,
    ) -> Option<String> {
        let temp = tempfile::tempdir().expect("tempdir");
        let expected_path = temp.path().join("expected.json");
        let actual_path = temp.path().join("actual.json");
        fs::write(
            &expected_path,
            serde_json::to_string(&expected).expect("expected json"),
        )
        .expect("write expected json");
        fs::write(
            &actual_path,
            serde_json::to_string_pretty(&actual).expect("actual json"),
        )
        .expect("write actual json");
        compare_output_json(&expected_path, &actual_path).expect("json comparison")
    }

    fn semantic_expected_whisperx_json() -> serde_json::Value {
        serde_json::json!({
            "language": "en",
            "segments": [
                {
                    "start": 0.0,
                    "end": 1.2,
                    "text": " hello world",
                    "avg_logprob": -0.1,
                    "no_speech_prob": 0.01,
                    "words": [
                        {
                            "word": " hello",
                            "start": 0.0,
                            "end": 0.5,
                            "score": 0.9876
                        },
                        {
                            "word": "world",
                            "start": 0.55,
                            "end": 1.2,
                            "score": 0.902
                        }
                    ],
                    "chars": [
                        {
                            "char": "h",
                            "start": 0.0,
                            "end": 0.1
                        },
                        {
                            "char": "i",
                            "start": null,
                            "end": null
                        }
                    ]
                }
            ],
            "word_segments": [
                {
                    "word": " hello",
                    "start": 0.0,
                    "end": 0.5,
                    "score": 0.9876
                },
                {
                    "word": "world",
                    "start": 0.55,
                    "end": 1.2,
                    "score": 0.902
                }
            ]
        })
    }

    fn semantic_actual_json() -> serde_json::Value {
        serde_json::json!({
            "text": "hello world",
            "source": "sample.wav",
            "language": "en",
            "segments": [
                {
                    "id": 0,
                    "start": 0.004,
                    "end": 1.196,
                    "text": "hello world",
                    "score": 0.95,
                    "words": [
                        {
                            "word": "hello",
                            "start": 0.002,
                            "end": 0.501,
                            "score": 0.987
                        },
                        {
                            "word": " world",
                            "start": 0.552,
                            "end": 1.198,
                            "score": 0.9025
                        }
                    ],
                    "chars": [
                        {
                            "char": "h",
                            "start": 0.002,
                            "end": 0.098
                        },
                        {
                            "char": "i"
                        }
                    ]
                }
            ],
            "word_segments": [
                {
                    "word": "hello",
                    "start": 0.002,
                    "end": 0.501,
                    "score": 0.987
                },
                {
                    "word": " world",
                    "start": 0.552,
                    "end": 1.198,
                    "score": 0.9025
                }
            ]
        })
    }

    fn contains_pair(args: &[String], flag: &str, value: &str) -> bool {
        args.windows(2)
            .any(|pair| pair[0] == flag && pair[1] == value)
    }
}

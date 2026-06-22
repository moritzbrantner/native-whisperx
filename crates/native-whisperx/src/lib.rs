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
    TranscriptionProviderSelection, TranscriptionSource,
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
pub use workflow::{run, run_many, run_many_reusing_native_provider};

#[cfg(all(test, feature = "silero-vad"))]
use config_mapping::resolve_silero_model_path;
#[cfg(all(test, feature = "silero-vad"))]
use config_mapping::validate_native_silero_config;
#[cfg(test)]
use config_mapping::{map_diarization, native_language_hint, validate_native_diarization_support};
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
    fn crate_root_preserves_public_compatibility_exports() {
        fn assert_type<T>() {}

        assert_type::<crate::TranscriptionPipelineRequest>();
        assert_type::<crate::TranscriptionPipelineResponse>();
        assert_type::<crate::NativeWhisperxConfig>();
        assert_type::<crate::InputSource>();
        assert_type::<crate::AsrConfig>();
        assert_type::<crate::ExternalWhisperxConfig>();
        assert_type::<crate::WhisperxDecodeConfig>();
        assert_type::<crate::TranslationConfig>();
        assert_type::<crate::VadConfig>();
        assert_type::<crate::AlignmentConfig>();
        assert_type::<crate::DiarizationConfig>();
        assert_type::<crate::OutputConfig>();
        assert_type::<crate::SubtitleConfig>();
        assert_type::<crate::ParityConfig>();
        assert_type::<crate::NativeWhisperxReport>();
        assert_type::<crate::ParityReport>();
        assert_type::<crate::ParityFixtureSuiteReport>();
        assert_type::<crate::ParityPreflightReport>();
        assert_type::<crate::SpeakerCorrectionRequest>();
        assert_type::<crate::SpeakerCorrectionReport>();
        assert_type::<crate::SpeakerDirectoryState>();
        assert_type::<crate::SpeakerTraceState>();
        assert_type::<crate::NativeWhisperxError>();
    }

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
    fn maps_native_unbounded_batching_to_active_row_runtime() {
        let request = build_transcription_request(&NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from("sample.wav"),
            },
            asr: AsrConfig {
                max_batch_size: None,
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

    fn contains_pair(args: &[String], flag: &str, value: &str) -> bool {
        args.windows(2)
            .any(|pair| pair[0] == flag && pair[1] == value)
    }
}

use std::path::PathBuf;

use native_whisperx::{
    inspect_workflow_mapping, AlignmentConfig, AsrConfig, DevicePreference, DiarizationConfig,
    InputSource, NativeWhisperxConfig, OutputConfig, TranslationConfig, VadConfig,
    WhisperxDecodeConfig,
};

#[test]
fn native_runtime_controls_remain_public_config_and_mapping_inputs() {
    let asr = AsrConfig {
        device: DevicePreference::Cuda,
        device_index: Some("2".to_string()),
        decode: WhisperxDecodeConfig {
            threads: Some(3),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };
    let serialized = serde_json::to_value(&asr).expect("ASR config should serialize");

    inspect_workflow_mapping(&config_with_asr(asr))
        .expect("one device index and a positive thread count should be accepted");
    assert_eq!(serialized["deviceIndex"], "2");
    assert_eq!(serialized["decode"]["threads"], 3);
}

#[test]
fn native_workflow_accepts_an_initial_prompt_before_model_setup() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            initial_prompt: Some("Moenarch Whisper vocabulary".to_string()),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    inspect_workflow_mapping(&config_with_asr(asr))
        .expect("a native initial prompt should reach request-scoped decoding");
}

#[test]
fn native_workflow_accepts_explicit_suppressed_token_ids_before_model_setup() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            suppress_tokens: Some("7, 42".to_string()),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    inspect_workflow_mapping(&config_with_asr(asr))
        .expect("explicit native tokenizer IDs should reach request-scoped decoding");
}

#[test]
fn native_workflow_rejects_a_non_numeric_suppressed_token_id() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            suppress_tokens: Some("7, not-a-token-id".to_string()),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    let error = inspect_workflow_mapping(&config_with_asr(asr))
        .expect_err("suppressed token IDs must be numeric");

    assert!(error.to_string().contains(
        "native --suppress-tokens entry `not-a-token-id` must be a non-negative tokenizer ID"
    ));
}

#[test]
fn native_workflow_accepts_numeral_suppression_before_model_setup() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            suppress_numerals: true,
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    inspect_workflow_mapping(&config_with_asr(asr))
        .expect("native numeral suppression should reach request-scoped decoding");
}

#[test]
fn native_workflow_accepts_request_local_previous_text_conditioning() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            condition_on_previous_text: Some(true),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    inspect_workflow_mapping(&config_with_asr(asr))
        .expect("previous-text conditioning should be accepted for one native request");
}

#[test]
fn native_workflow_accepts_a_compression_ratio_fallback_threshold() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            compression_ratio_threshold: Some(2.4),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    inspect_workflow_mapping(&config_with_asr(asr))
        .expect("a native compression-ratio threshold should configure fallback decoding");
}

#[test]
fn native_workflow_accepts_a_log_probability_fallback_threshold() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            logprob_threshold: Some(-1.0),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    inspect_workflow_mapping(&config_with_asr(asr))
        .expect("a native log-probability threshold should configure fallback decoding");
}

#[test]
fn native_workflow_accepts_a_no_speech_rejection_threshold() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            no_speech_threshold: Some(0.6),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    inspect_workflow_mapping(&config_with_asr(asr))
        .expect("a native no-speech threshold should configure silence rejection");
}

#[test]
fn native_workflow_rejects_a_non_positive_compression_ratio_threshold() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            compression_ratio_threshold: Some(0.0),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    let error = inspect_workflow_mapping(&config_with_asr(asr))
        .expect_err("compression-ratio thresholds must be positive");

    assert!(error
        .to_string()
        .contains("native --compression-ratio-threshold must be finite and greater than zero"));
}

#[test]
fn native_workflow_rejects_an_out_of_range_no_speech_threshold() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            no_speech_threshold: Some(1.1),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    let error = inspect_workflow_mapping(&config_with_asr(asr))
        .expect_err("no-speech thresholds must be probabilities");

    assert!(error
        .to_string()
        .contains("native --no-speech-threshold must be between zero and one"));
}

#[test]
fn native_workflow_rejects_a_non_finite_log_probability_threshold() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            logprob_threshold: Some(f32::NAN),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    let error = inspect_workflow_mapping(&config_with_asr(asr))
        .expect_err("log-probability thresholds must be finite");

    assert!(error
        .to_string()
        .contains("native --logprob-threshold must be finite"));
}

#[test]
fn native_decode_rejects_sampling_and_beam_search_together() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            temperature: vec![0.2],
            best_of: Some(1),
            beam_size: Some(5),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    let error = inspect_workflow_mapping(&config_with_asr(asr))
        .expect_err("beam search and positive-temperature sampling are incompatible");

    assert!(error
        .to_string()
        .contains("native beam search requires an all-zero --temperature schedule"));
}

#[test]
fn native_decode_rejects_zero_best_of_before_model_setup() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            best_of: Some(0),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    let error = inspect_workflow_mapping(&config_with_asr(asr))
        .expect_err("best_of must select at least one candidate");

    assert!(error
        .to_string()
        .contains("native --best_of must be greater than zero"));
}

#[test]
fn native_decode_rejects_zero_beam_size_before_model_setup() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            beam_size: Some(0),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    let error = inspect_workflow_mapping(&config_with_asr(asr))
        .expect_err("beam_size must select at least one candidate");

    assert!(error
        .to_string()
        .contains("native --beam_size must be greater than zero"));
}

#[test]
fn native_decode_rejects_best_of_during_beam_search() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            temperature: vec![0.0],
            best_of: Some(2),
            beam_size: Some(5),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    let error = inspect_workflow_mapping(&config_with_asr(asr))
        .expect_err("best_of and beam search are mutually exclusive");

    assert!(error
        .to_string()
        .contains("native --best_of must be 1 when --beam_size is greater than 1"));
}

#[test]
fn native_decode_rejects_best_of_without_sampling() {
    let asr = AsrConfig {
        decode: WhisperxDecodeConfig {
            temperature: vec![0.0],
            best_of: Some(2),
            ..WhisperxDecodeConfig::default()
        },
        ..AsrConfig::default()
    };

    let error = inspect_workflow_mapping(&config_with_asr(asr))
        .expect_err("best_of requires positive-temperature sampling");

    assert!(error
        .to_string()
        .contains("native --best_of greater than 1 requires a positive --temperature"));
}

#[test]
fn native_decode_rejects_invalid_beam_score_factors() {
    let invalid = [
        (
            WhisperxDecodeConfig {
                temperature: vec![0.0],
                beam_size: Some(5),
                patience: Some(0.0),
                ..WhisperxDecodeConfig::default()
            },
            "native --patience must be finite and greater than zero",
        ),
        (
            WhisperxDecodeConfig {
                temperature: vec![0.0],
                beam_size: Some(5),
                length_penalty: Some(-0.1),
                ..WhisperxDecodeConfig::default()
            },
            "native --length_penalty must be finite and greater than or equal to zero",
        ),
    ];

    for (decode, expected) in invalid {
        let error = inspect_workflow_mapping(&config_with_asr(AsrConfig {
            decode,
            ..AsrConfig::default()
        }))
        .expect_err("invalid beam score factors must fail before model setup");
        assert!(error.to_string().contains(expected), "{error}");
    }
}

#[test]
fn native_decode_rejects_beam_score_factors_without_beam_search() {
    let invalid = [
        (
            WhisperxDecodeConfig {
                patience: Some(1.2),
                ..WhisperxDecodeConfig::default()
            },
            "native --patience only applies when --beam_size is greater than 1",
        ),
        (
            WhisperxDecodeConfig {
                length_penalty: Some(0.8),
                ..WhisperxDecodeConfig::default()
            },
            "native --length_penalty only applies when --beam_size is greater than 1",
        ),
    ];

    for (decode, expected) in invalid {
        let error = inspect_workflow_mapping(&config_with_asr(AsrConfig {
            decode,
            ..AsrConfig::default()
        }))
        .expect_err("beam score factors must not be silently ignored");
        assert!(error.to_string().contains(expected), "{error}");
    }
}

#[test]
fn native_workflow_accepts_a_valid_sampling_schedule_before_model_setup() {
    let config = NativeWhisperxConfig {
        input: InputSource::Path {
            path: PathBuf::from("sample.wav"),
        },
        asr: AsrConfig {
            decode: WhisperxDecodeConfig {
                temperature: vec![0.2, 0.4],
                best_of: Some(3),
                ..WhisperxDecodeConfig::default()
            },
            ..AsrConfig::default()
        },
        translation: TranslationConfig::default(),
        vad: VadConfig::default(),
        alignment: AlignmentConfig::default(),
        diarization: DiarizationConfig::default(),
        output: OutputConfig::default(),
    };

    inspect_workflow_mapping(&config)
        .expect("valid native sampling controls should pass workflow validation");
}

fn config_with_asr(asr: AsrConfig) -> NativeWhisperxConfig {
    NativeWhisperxConfig {
        input: InputSource::Path {
            path: PathBuf::from("sample.wav"),
        },
        asr,
        translation: TranslationConfig::default(),
        vad: VadConfig::default(),
        alignment: AlignmentConfig::default(),
        diarization: DiarizationConfig::default(),
        output: OutputConfig::default(),
    }
}

#[test]
#[ignore = "requires the pinned German cache probe and Whisper-small bundle"]
fn native_sampling_schedule_reaches_the_candle_decoder() {
    let (config, _output_dir) = real_decode_config(WhisperxDecodeConfig {
        temperature: vec![0.0, 0.2],
        logprob_threshold: Some(1.0),
        ..WhisperxDecodeConfig::default()
    });

    let report =
        native_whisperx::run(config).expect("the pinned native sampling request should run");

    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.starts_with("temperatureSchedule=0,0.2")));
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "decodeStrategy=temperatureSampling"));
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.starts_with("temperatureFallbackAttempts=0,0.2")));
}

#[test]
#[ignore = "requires the pinned German cache probe and Whisper-small bundle"]
fn native_prompt_and_suppression_controls_reach_the_candle_decoder() {
    let (config, _output_dir) = real_decode_config(WhisperxDecodeConfig {
        initial_prompt: Some("Moenarch Whisper vocabulary".to_string()),
        suppress_tokens: Some("0".to_string()),
        suppress_numerals: true,
        ..WhisperxDecodeConfig::default()
    });

    let report = native_whisperx::run(config)
        .expect("the pinned native prompt and suppression request should run");

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic
            .strip_prefix("initialPromptTokenCount=")
            .and_then(|value| value.parse::<usize>().ok())
            .is_some_and(|count| count > 0)
    }));
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "suppressedTokenCount=1"));
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "suppressNumerals=true"));
}

#[test]
#[ignore = "requires the pinned German cache probe and Whisper-small bundle"]
fn native_no_speech_threshold_rejects_a_low_confidence_window() {
    let (config, _output_dir) = real_decode_config(WhisperxDecodeConfig {
        logprob_threshold: Some(1.0),
        no_speech_threshold: Some(0.0),
        ..WhisperxDecodeConfig::default()
    });

    let report = native_whisperx::run(config)
        .expect("the pinned native no-speech rejection request should run");

    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "noSpeechRejected=true"));
}

#[test]
#[ignore = "requires the pinned German cache probe and Whisper-small bundle"]
fn native_previous_text_conditioning_is_request_local_across_reused_sessions() {
    let (mut first, _first_output_dir) = real_decode_config(WhisperxDecodeConfig {
        condition_on_previous_text: Some(true),
        ..WhisperxDecodeConfig::default()
    });
    let (mut second, _second_output_dir) = real_decode_config(WhisperxDecodeConfig {
        condition_on_previous_text: Some(true),
        ..WhisperxDecodeConfig::default()
    });
    first.vad.max_chunk_seconds = 1.0;
    second.vad.max_chunk_seconds = 1.0;

    let reports = native_whisperx::run_many_reusing_native_provider(vec![first, second])
        .expect("two conditioned requests should reuse only the loaded model session");

    assert_eq!(reports.len(), 2);
    assert!(reports.iter().all(|report| report.vad_segments.len() >= 2));
    assert_eq!(reports[0].transcript, reports[1].transcript);
    assert!(reports.iter().all(|report| report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "conditionOnPreviousText=true")));
}

fn real_decode_config(decode: WhisperxDecodeConfig) -> (NativeWhisperxConfig, tempfile::TempDir) {
    let input = std::env::var_os("CANDLE_WHISPER_GERMAN_WAV")
        .map(PathBuf::from)
        .expect("CANDLE_WHISPER_GERMAN_WAV");
    let bundle = std::env::var_os("CANDLE_WHISPER_SMALL_BUNDLE")
        .map(PathBuf::from)
        .expect("CANDLE_WHISPER_SMALL_BUNDLE");
    let output_dir = tempfile::tempdir().expect("temporary output directory");
    let config = NativeWhisperxConfig {
        input: InputSource::Path { path: input },
        asr: AsrConfig {
            model_id: "openai/whisper-small".to_string(),
            whisper_bundle: Some(bundle),
            model_cache_only: true,
            device: DevicePreference::Cpu,
            compute_type: Some("fp32".to_string()),
            decode,
            ..AsrConfig::default()
        },
        translation: TranslationConfig::default(),
        vad: VadConfig::default(),
        alignment: AlignmentConfig {
            enabled: false,
            ..AlignmentConfig::default()
        },
        diarization: DiarizationConfig::default(),
        output: OutputConfig {
            output_dir: Some(output_dir.path().to_path_buf()),
            ..OutputConfig::default()
        },
    };
    (config, output_dir)
}

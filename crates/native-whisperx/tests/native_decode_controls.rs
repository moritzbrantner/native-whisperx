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
fn native_decode_accepts_whisperx_beam_and_best_of_with_a_fallback_schedule() {
    // faster-whisper: --beam_size applies at temperature 0, --best_of at the
    // positive fallback temperatures.
    for decode in [
        WhisperxDecodeConfig {
            temperature: vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0],
            best_of: Some(5),
            beam_size: Some(5),
            ..WhisperxDecodeConfig::default()
        },
        WhisperxDecodeConfig {
            temperature: vec![0.0],
            best_of: Some(5),
            ..WhisperxDecodeConfig::default()
        },
    ] {
        let asr = AsrConfig {
            decode,
            ..AsrConfig::default()
        };
        inspect_workflow_mapping(&config_with_asr(asr))
            .expect("beam search and sampling share one fallback schedule");
    }
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
            decode: WhisperxDecodeConfig {
                temperature: vec![0.2],
                best_of: Some(2),
                ..WhisperxDecodeConfig::default()
            },
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

    let report =
        native_whisperx::run(config).expect("the pinned native sampling request should run");

    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.starts_with("temperatureSchedule=0.2")));
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "bestOf=2"));
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "decodeStrategy=temperatureSampling"));
}

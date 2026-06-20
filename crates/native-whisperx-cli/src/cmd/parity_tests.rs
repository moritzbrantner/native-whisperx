use super::*;

    #[test]
    fn speed_comparison_reports_native_faster_and_speedup_ratio() {
        let comparison = bench_speed_comparison(10.0, Some(25.0));

        assert_eq!(comparison.native_faster_than_whisperx, Some(true));
        assert_eq!(comparison.native_speedup_ratio, Some(2.5));
    }

    #[test]
    fn speed_comparison_reports_native_regression() {
        let comparison = bench_speed_comparison(25.0, Some(10.0));

        assert_eq!(comparison.native_faster_than_whisperx, Some(false));
        assert_eq!(comparison.native_speedup_ratio, Some(0.4));
    }

    #[test]
    fn speed_comparison_is_absent_without_reference_run() {
        let comparison = bench_speed_comparison(10.0, None);

        assert_eq!(comparison.native_faster_than_whisperx, None);
        assert_eq!(comparison.native_speedup_ratio, None);
    }

    #[test]
    fn speed_gate_fails_only_when_reference_proves_native_slower() {
        assert!(!bench_iteration_passes_speed_gate(&serde_json::json!({
            "nativeFasterThanWhisperx": false
        })));
        assert!(bench_iteration_passes_speed_gate(&serde_json::json!({
            "nativeFasterThanWhisperx": true
        })));
        assert!(bench_iteration_passes_speed_gate(&serde_json::json!({})));
    }

    #[test]
    fn native_bench_config_uses_whisperx_batch_size_when_native_is_unspecified() {
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            native_asr: AsrConfig {
                max_batch_size: None,
                ..AsrConfig::default()
            },
            whisperx: ExternalWhisperxConfig {
                batch_size: Some(8),
                ..ExternalWhisperxConfig::default()
            },
            ..bench_fixture_defaults()
        };

        let config = native_bench_config(&fixture);

        assert_eq!(config.asr.max_batch_size, Some(8));
    }

    #[test]
    fn native_bench_config_keeps_explicit_native_batch_size() {
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            native_asr: AsrConfig {
                max_batch_size: Some(6),
                ..AsrConfig::default()
            },
            whisperx: ExternalWhisperxConfig {
                batch_size: Some(8),
                ..ExternalWhisperxConfig::default()
            },
            ..bench_fixture_defaults()
        };

        let config = native_bench_config(&fixture);

        assert_eq!(config.asr.max_batch_size, Some(6));
    }

    #[test]
    fn whisperx_bench_config_uses_native_fixture_device_target() {
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            native_asr: AsrConfig {
                device: DevicePreference::Cuda,
                device_index: Some("0".to_string()),
                ..AsrConfig::default()
            },
            ..bench_fixture_defaults()
        };

        let config = whisperx_bench_config(&fixture);

        assert_eq!(config.asr.device, DevicePreference::Cuda);
        assert_eq!(config.asr.device_index.as_deref(), Some("0"));
    }

    #[test]
    fn whisperx_bench_config_uses_fixture_reference_batch_size() {
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            whisperx: ExternalWhisperxConfig {
                batch_size: Some(8),
                ..ExternalWhisperxConfig::default()
            },
            ..bench_fixture_defaults()
        };

        let config = whisperx_bench_config(&fixture);

        assert_eq!(config.asr.max_batch_size, Some(8));
    }

    #[test]
    fn infers_ort_dylib_path_from_whisperx_environment_for_native_onnx_vad() {
        let temp = tempfile::tempdir().expect("tempdir");
        let whisperx = temp.path().join("bin").join("whisperx");
        fs::create_dir_all(whisperx.parent().expect("bin")).expect("bin dir");
        fs::write(&whisperx, "").expect("whisperx");
        let capi = temp
            .path()
            .join("lib")
            .join("python3.11")
            .join("site-packages")
            .join("onnxruntime")
            .join("capi");
        fs::create_dir_all(&capi).expect("capi dir");
        let dylib = capi.join("libonnxruntime.so.1.27.0");
        fs::write(&dylib, "").expect("dylib");
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            vad: VadConfig {
                method: VadMethod::Silero,
                ..VadConfig::default()
            },
            whisperx: ExternalWhisperxConfig {
                command: whisperx,
                ..ExternalWhisperxConfig::default()
            },
            ..bench_fixture_defaults()
        };

        assert_eq!(
            inferred_ort_dylib_path_with_env(&fixture, None),
            Some(dylib)
        );
    }

    #[test]
    fn does_not_infer_ort_dylib_when_env_is_explicit() {
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            vad: VadConfig {
                method: VadMethod::Silero,
                ..VadConfig::default()
            },
            ..bench_fixture_defaults()
        };

        assert_eq!(
            inferred_ort_dylib_path_with_env(&fixture, Some(OsString::from("/explicit/lib.so"))),
            None
        );
    }

    #[test]
    fn does_not_infer_ort_dylib_for_energy_vad() {
        let temp = tempfile::tempdir().expect("tempdir");
        let whisperx = temp.path().join("bin").join("whisperx");
        fs::create_dir_all(whisperx.parent().expect("bin")).expect("bin dir");
        fs::write(&whisperx, "").expect("whisperx");
        let fixture = ParityFixtureCase {
            name: "bench".to_string(),
            input: PathBuf::from("audio.wav"),
            vad: VadConfig {
                method: VadMethod::Energy,
                ..VadConfig::default()
            },
            whisperx: ExternalWhisperxConfig {
                command: whisperx,
                ..ExternalWhisperxConfig::default()
            },
            ..bench_fixture_defaults()
        };

        assert_eq!(inferred_ort_dylib_path_with_env(&fixture, None), None);
    }

    #[test]
    fn bench_phase_json_exposes_native_total_seconds() {
        let phases = bench_phase_json(
            &[
                "phaseDecodeSeconds=0.100000".to_string(),
                "phaseVadSeconds=0.200000".to_string(),
                "phaseAsrSeconds=0.300000".to_string(),
                "phaseAlignmentSeconds=0.400000".to_string(),
                "phaseOutputSeconds=0.500000".to_string(),
                "phaseNativeTotalSeconds=1.500000".to_string(),
            ],
            1.6,
        );

        assert_eq!(phases["decodeSeconds"], serde_json::json!(0.1));
        assert_eq!(phases["vadSeconds"], serde_json::json!(0.2));
        assert_eq!(phases["asrSeconds"], serde_json::json!(0.3));
        assert_eq!(phases["alignmentSeconds"], serde_json::json!(0.4));
        assert_eq!(phases["outputSeconds"], serde_json::json!(0.5));
        assert_eq!(phases["nativeTotalSeconds"], serde_json::json!(1.5));
        assert_eq!(phases["totalElapsedSeconds"], serde_json::json!(1.6));
    }

    fn bench_fixture_defaults() -> ParityFixtureCase {
        ParityFixtureCase {
            name: String::new(),
            gating: false,
            input: PathBuf::new(),
            clip_seconds: None,
            timeout_seconds: None,
            expected_json: None,
            expected_target: native_whisperx::ExpectedTranscriptTarget::Native,
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

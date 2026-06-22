//! Parity commands for comparisons, fixture suites, goldens, and benchmarks.

use super::*;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct ParityArgs {
    pub(crate) input: PathBuf,
    #[arg(long, visible_alias = "whisperx_command")]
    pub(crate) whisperx_command: Option<PathBuf>,
    #[arg(long, visible_alias = "whisperx_model", default_value = "small")]
    pub(crate) whisperx_model: String,
    #[arg(long, visible_alias = "expected_json")]
    pub(crate) expected_json: Option<PathBuf>,
    #[arg(long, visible_alias = "whisper_bundle")]
    pub(crate) whisper_bundle: Option<PathBuf>,
    #[arg(long, default_value = "small")]
    pub(crate) model: String,
    #[arg(long, value_enum, default_value_t = CliDevicePreference::Auto)]
    pub(crate) device: CliDevicePreference,
    #[arg(long = "no-align", visible_alias = "no_align")]
    pub(crate) no_align: bool,
    #[arg(long, visible_alias = "alignment_bundle")]
    pub(crate) alignment_bundle: Option<PathBuf>,
    #[arg(
        long = "align-model",
        visible_alias = "align_model",
        default_value = "facebook/wav2vec2-base-960h"
    )]
    pub(crate) alignment_model: String,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    pub(crate) model_cache_only: bool,
    #[arg(long = "interpolate-method", visible_alias = "interpolate_method", value_enum, default_value_t = CliAlignmentInterpolationMethod::Nearest)]
    pub(crate) interpolate_method: CliAlignmentInterpolationMethod,
    #[arg(
        long = "return-char-alignments",
        visible_alias = "return_char_alignments"
    )]
    pub(crate) return_char_alignments: bool,
    #[arg(long, visible_alias = "speaker_embedding_bundle")]
    pub(crate) speaker_embedding_bundle: Option<PathBuf>,
    #[arg(long, visible_alias = "min_speakers")]
    pub(crate) min_speakers: Option<usize>,
    #[arg(long, visible_alias = "max_speakers")]
    pub(crate) max_speakers: Option<usize>,
    #[arg(long)]
    pub(crate) language: Option<String>,
    #[arg(long, visible_alias = "output_dir")]
    pub(crate) output_dir: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct ParityFixturesArgs {
    pub(crate) manifest: PathBuf,
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,
    #[arg(long, visible_alias = "whisperx_command")]
    pub(crate) whisperx_command: Option<PathBuf>,
    #[arg(long = "output-dir", visible_alias = "output_dir")]
    pub(crate) output_dir: Option<PathBuf>,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    pub(crate) model_cache_only: bool,
    #[arg(long = "case")]
    pub(crate) cases: Vec<String>,
    #[arg(long = "case-timeout-seconds", visible_alias = "case_timeout_seconds")]
    pub(crate) case_timeout_seconds: Option<u64>,
    #[arg(
        long = "require-non-gating-passed",
        visible_alias = "require_non_gating_passed"
    )]
    pub(crate) require_non_gating_passed: bool,
}

#[derive(Debug, Args)]
pub(crate) struct ParityBenchArgs {
    pub(crate) manifest: PathBuf,
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,
    #[arg(long, visible_alias = "whisperx_command")]
    pub(crate) whisperx_command: Option<PathBuf>,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    pub(crate) model_cache_only: bool,
    #[arg(long = "iterations", default_value_t = 3)]
    pub(crate) iterations: usize,
    #[arg(long = "warmups", default_value_t = 1)]
    pub(crate) warmups: usize,
    #[arg(long = "case")]
    pub(crate) cases: Vec<String>,
    #[arg(long = "case-timeout-seconds", visible_alias = "case_timeout_seconds")]
    pub(crate) case_timeout_seconds: Option<u64>,
    #[arg(long = "native-only", visible_alias = "native_only")]
    pub(crate) native_only: bool,
    #[arg(long = "report-only", visible_alias = "report_only")]
    pub(crate) report_only: bool,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Args)]
pub(crate) struct ParitySummaryArgs {
    pub(crate) report: PathBuf,
    #[arg(long = "preflight-report", visible_alias = "preflight_report")]
    pub(crate) preflight_report: Option<PathBuf>,
    #[arg(long = "allow-missing-report", visible_alias = "allow_missing_report")]
    pub(crate) allow_missing_report: bool,
    #[arg(long)]
    pub(crate) suite: Option<String>,
    #[arg(long)]
    pub(crate) features: Option<String>,
    #[arg(long)]
    pub(crate) runner: Option<String>,
    #[arg(long)]
    pub(crate) manifest: Option<PathBuf>,
    #[arg(long = "output-dir", visible_alias = "output_dir")]
    pub(crate) output_dir: Option<PathBuf>,
    #[arg(long = "smoke-root", visible_alias = "smoke_root")]
    pub(crate) smoke_root: Option<PathBuf>,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "whisperx-command", visible_alias = "whisperx_command")]
    pub(crate) whisperx_command: Option<PathBuf>,
    #[arg(long = "progress-log", visible_alias = "progress_log")]
    pub(crate) progress_log: Option<PathBuf>,
    #[arg(long = "ort-dylib-path", visible_alias = "ort_dylib_path")]
    pub(crate) ort_dylib_path: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct ParityFixtureCaseArgs {
    #[arg(long)]
    pub(crate) fixture: PathBuf,
    #[arg(long)]
    pub(crate) root: PathBuf,
    #[arg(long)]
    pub(crate) report: PathBuf,
}

#[derive(Debug, Args)]
pub(crate) struct ParityBenchCaseArgs {
    #[arg(long)]
    pub(crate) fixture: PathBuf,
    #[arg(long)]
    pub(crate) report: PathBuf,
    #[arg(long)]
    pub(crate) iterations: usize,
    #[arg(long)]
    pub(crate) warmups: usize,
    #[arg(long = "native-only", visible_alias = "native_only")]
    pub(crate) native_only: bool,
}

#[derive(Debug, Args)]
pub(crate) struct ParityBenchMultiInputCaseArgs {
    #[arg(long)]
    pub(crate) fixture: PathBuf,
    #[arg(long)]
    pub(crate) report: PathBuf,
    #[arg(long)]
    pub(crate) iterations: usize,
    #[arg(long)]
    pub(crate) warmups: usize,
    #[arg(long = "native-only", visible_alias = "native_only")]
    pub(crate) native_only: bool,
}

#[derive(Debug, Args)]
pub(crate) struct ParityGoldensArgs {
    pub(crate) manifest: PathBuf,
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,
    #[arg(
        long,
        visible_alias = "whisperx_command",
        default_value = ".audio-tools/whisperx-venv/bin/whisperx"
    )]
    pub(crate) whisperx_command: PathBuf,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    pub(crate) model_cache_only: bool,
    #[arg(long = "case")]
    pub(crate) cases: Vec<String>,
    #[arg(long = "include-non-gating", visible_alias = "include_non_gating")]
    pub(crate) include_non_gating: bool,
    #[arg(long)]
    pub(crate) overwrite: bool,
    #[arg(long = "dry-run", visible_alias = "dry_run")]
    pub(crate) dry_run: bool,
}

pub(crate) fn parity_command(args: ParityArgs) -> anyhow::Result<()> {
    let report = compare_with_whisperx(ParityConfig {
        input: args.input,
        expected_json: args.expected_json,
        expected_target: ExpectedTranscriptTarget::Native,
        comparison: ParityComparisonConfig::default(),
        native_asr: AsrConfig {
            provider: AsrProvider::Native,
            model_id: args.model,
            whisper_bundle: args.whisper_bundle,
            model_dir: args.model_dir.clone(),
            model_cache_only: args.model_cache_only,
            device: args.device.into(),
            ..AsrConfig::default()
        },
        translation: TranslationConfig::default(),
        vad: VadConfig::default(),
        alignment: alignment_config(
            args.no_align,
            args.alignment_model,
            args.alignment_bundle,
            args.model_dir,
            args.model_cache_only,
            args.interpolate_method,
            args.return_char_alignments,
        ),
        diarization: DiarizationConfig {
            enabled: args.speaker_embedding_bundle.is_some()
                || args.min_speakers.is_some()
                || args.max_speakers.is_some(),
            speaker_embedding_model_bundle: args.speaker_embedding_bundle,
            min_speakers: args.min_speakers,
            max_speakers: args.max_speakers,
            ..DiarizationConfig::default()
        },
        whisperx_diarization: None,
        whisperx: ExternalWhisperxConfig {
            command: args
                .whisperx_command
                .unwrap_or_else(|| PathBuf::from("whisperx")),
            model: args.whisperx_model,
            output_dir: args.output_dir.clone(),
            ..ExternalWhisperxConfig::default()
        },
        language: args.language,
        output: OutputConfig {
            output_dir: args.output_dir,
            formats: vec![OutputFormat::Json],
            basename: Some("whisperx-parity".to_string()),
            pretty_json: true,
            subtitles: SubtitleConfig::default(),
        },
    })?;

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(crate) fn parity_fixtures_command(args: ParityFixturesArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.manifest)
        .with_context(|| format!("failed to read {}", args.manifest.display()))?;
    let mut suite: ParityFixtureSuite = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.manifest.display()))?;
    let root = args
        .root
        .or_else(smoke_root_from_env_or_dotenv)
        .with_context(|| {
            "parity-fixtures requires --root, SMOKE_ROOT, or SMOKE_ROOT in .env for local audio, expected JSON, and model cache paths"
        })?;
    let root = absolute_from_cwd(root)?;
    let whisperx_command = args.whisperx_command.map(absolute_from_cwd).transpose()?;
    let output_dir = args.output_dir.map(absolute_from_cwd).transpose()?;
    let suite_report_path = output_dir
        .as_ref()
        .map(|output_dir| output_dir.join("report.json"));
    let model_dir = args.model_dir.map(absolute_from_cwd).transpose()?;
    let filters = args.cases.iter().cloned().collect::<HashSet<_>>();

    for case_name in &filters {
        if !suite_case_name_exists(&suite.fixtures, case_name) {
            anyhow::bail!("no fixture case named {case_name} matched the manifest");
        }
    }

    if !filters.is_empty() {
        suite
            .fixtures
            .retain(|fixture| filters.contains(&fixture.name));
    }

    for fixture in &mut suite.fixtures {
        if let Some(command) = &whisperx_command {
            fixture.whisperx.command = command.clone();
        }
        if let Some(output_dir) = &output_dir {
            fixture.output.output_dir = Some(output_dir.join(&fixture.name));
        }
        if let Some(model_dir) = &model_dir {
            fixture.native_asr.model_dir = Some(model_dir.clone());
            fixture.alignment.model_dir = Some(model_dir.clone());
        }
        if args.model_cache_only {
            fixture.native_asr.model_cache_only = true;
            fixture.alignment.model_cache_only = true;
        }
    }

    let report = run_parity_fixture_suite_with_progress(
        suite,
        root.clone(),
        args.case_timeout_seconds.map(Duration::from_secs),
        args.require_non_gating_passed,
    )?;
    if let Some(report_path) = &suite_report_path {
        if let Some(parent) = report_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create parity fixture report directory {}",
                    parent.display()
                )
            })?;
        }
        fs::write(report_path, serde_json::to_vec_pretty(&report)?).with_context(|| {
            format!(
                "failed to write parity fixture report {}",
                report_path.display()
            )
        })?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    if !report.passed {
        anyhow::bail!("one or more parity fixtures failed");
    }
    Ok(())
}

fn run_parity_fixture_suite_with_progress(
    suite: ParityFixtureSuite,
    root: PathBuf,
    case_timeout: Option<Duration>,
    require_non_gating_passed: bool,
) -> anyhow::Result<ParityFixtureSuiteReport> {
    let total = suite.fixtures.len();
    let mut cases = Vec::with_capacity(total);

    for (index, fixture) in suite.fixtures.into_iter().enumerate() {
        let case_number = index + 1;
        let case_name = fixture.name.clone();
        let gating = fixture.gating;
        let started_at = unix_timestamp_string(SystemTime::now());
        let start = Instant::now();
        eprintln!(
            "parity-fixtures: starting case {case_number}/{total}: {case_name}{}",
            if gating { " [gating]" } else { "" }
        );

        let fixture_timeout = fixture.timeout_seconds.map(Duration::from_secs);
        let timeout = case_timeout.or(fixture_timeout);
        let mut case = run_single_parity_fixture_case(fixture, root.clone(), timeout)?;
        let elapsed = start.elapsed();
        case.started_at = Some(started_at);
        case.elapsed_seconds = Some(duration_seconds(elapsed));
        case.timed_out = case.error.as_deref().is_some_and(is_timeout_error);
        if case.timed_out {
            eprintln!(
                "parity-fixtures: timed out case {case_number}/{total}: {case_name} after {}",
                format_duration(elapsed)
            );
        } else if case.passed {
            eprintln!(
                "parity-fixtures: completed case {case_number}/{total}: {case_name} passed in {}",
                format_duration(elapsed)
            );
        } else {
            eprintln!(
                "parity-fixtures: completed case {case_number}/{total}: {case_name} failed in {}",
                format_duration(elapsed)
            );
        }
        cases.push(case);
    }

    let passed = cases
        .iter()
        .filter(|case| require_non_gating_passed || case.gating)
        .all(|case| case.passed);
    Ok(ParityFixtureSuiteReport { passed, cases })
}

fn run_single_parity_fixture_case(
    fixture: ParityFixtureCase,
    root: PathBuf,
    case_timeout: Option<Duration>,
) -> anyhow::Result<ParityFixtureCaseReport> {
    let name = fixture.name.clone();
    let gating = fixture.gating;
    let Some(timeout) = case_timeout else {
        return run_single_parity_fixture_case_now(fixture, root);
    };
    if timeout.is_zero() {
        let error = format!(
            "parity fixture case `{name}` exceeded timeout of {}",
            format_duration(timeout)
        );
        return Ok(failed_parity_fixture_case(name, gating, error));
    }

    let temp_prefix = parity_case_temp_prefix(&name);
    let fixture_path = temp_prefix.with_extension("fixture.json");
    let report_path = temp_prefix.with_extension("report.json");
    fs::write(&fixture_path, serde_json::to_vec(&fixture)?)?;

    let result = run_single_parity_fixture_case_child(&fixture_path, &root, &report_path, timeout)
        .and_then(|status| {
            if !status.success() {
                let error =
                    format!("parity fixture case `{name}` worker exited with status {status}");
                return Ok(failed_parity_fixture_case(name.clone(), gating, error));
            }
            let bytes = fs::read(&report_path).with_context(|| {
                format!(
                    "parity fixture case `{name}` worker did not write {}",
                    report_path.display()
                )
            })?;
            serde_json::from_slice::<ParityFixtureCaseReport>(&bytes).map_err(anyhow::Error::from)
        });

    let _ = fs::remove_file(&fixture_path);
    let _ = fs::remove_file(&report_path);

    match result {
        Ok(case) => Ok(case),
        Err(error) if is_timeout_error(&error.to_string()) => {
            Ok(failed_parity_fixture_case(name, gating, error.to_string()))
        }
        Err(error) => Err(error),
    }
}

fn run_single_parity_fixture_case_child(
    fixture_path: &Path,
    root: &Path,
    report_path: &Path,
    timeout: Duration,
) -> anyhow::Result<ExitStatus> {
    let mut child = ProcessCommand::new(std::env::current_exe()?)
        .arg("__parity-fixture-case")
        .arg("--fixture")
        .arg(fixture_path)
        .arg("--root")
        .arg(root)
        .arg("--report")
        .arg(report_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| "failed to spawn parity fixture case worker")?;

    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!(
                "parity fixture case worker exceeded timeout of {}",
                format_duration(timeout)
            );
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn parity_case_temp_prefix(case_name: &str) -> PathBuf {
    let millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let safe_name = case_name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>();
    std::env::temp_dir().join(format!(
        "native-whisperx-parity-{safe_name}-{}-{millis}",
        std::process::id()
    ))
}

fn run_single_parity_fixture_case_now(
    fixture: ParityFixtureCase,
    root: PathBuf,
) -> anyhow::Result<ParityFixtureCaseReport> {
    let name = fixture.name.clone();
    let gating = fixture.gating;
    let report = run_parity_fixture_suite(
        ParityFixtureSuite {
            fixtures: vec![fixture],
            multi_input_fixtures: Vec::new(),
        },
        Some(&root),
    )?;
    Ok(report.cases.into_iter().next().unwrap_or_else(|| {
        failed_parity_fixture_case(
            name.clone(),
            gating,
            format!("parity fixture case `{name}` produced no report"),
        )
    }))
}

pub(crate) fn parity_fixture_case_command(args: ParityFixtureCaseArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.fixture)
        .with_context(|| format!("failed to read {}", args.fixture.display()))?;
    let fixture: ParityFixtureCase = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.fixture.display()))?;
    let report = run_single_parity_fixture_case_now(fixture, args.root)?;
    fs::write(&args.report, serde_json::to_vec(&report)?)
        .with_context(|| format!("failed to write {}", args.report.display()))?;
    Ok(())
}

pub(crate) fn parity_bench_command(args: ParityBenchArgs) -> anyhow::Result<()> {
    if args.iterations == 0 {
        anyhow::bail!("--iterations must be greater than zero");
    }
    let bytes = fs::read(&args.manifest)
        .with_context(|| format!("failed to read {}", args.manifest.display()))?;
    let mut suite: ParityFixtureSuite = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.manifest.display()))?;
    validate_parity_bench_suite(&suite)?;
    let root = smoke_root_or_arg(args.root, "parity-bench")?;
    let whisperx_command = args.whisperx_command.map(absolute_from_cwd).transpose()?;
    let model_dir = args
        .model_dir
        .map(absolute_from_cwd)
        .transpose()?
        .unwrap_or_else(|| root.join("models"));
    let filters = args.cases.iter().cloned().collect::<HashSet<_>>();

    for case_name in &filters {
        if !bench_suite_case_name_exists(&suite, case_name) {
            anyhow::bail!("no fixture case named {case_name} matched the manifest");
        }
    }
    if !filters.is_empty() {
        suite
            .fixtures
            .retain(|fixture| filters.contains(&fixture.name));
        suite
            .multi_input_fixtures
            .retain(|fixture| filters.contains(&fixture.name));
        validate_parity_bench_suite(&suite)?;
    }

    let mut case_results =
        Vec::with_capacity(suite.fixtures.len() + suite.multi_input_fixtures.len());
    for mut fixture in suite.fixtures {
        prepare_fixture_for_cli_run(
            &mut fixture,
            &root,
            whisperx_command.as_ref(),
            &model_dir,
            args.model_cache_only,
        );
        let timeout = args
            .case_timeout_seconds
            .or(fixture.timeout_seconds)
            .map(Duration::from_secs);
        let options = BenchRunOptions {
            iterations: args.iterations,
            warmups: args.warmups,
            native_only: args.native_only,
        };
        let case_result = run_parity_bench_case_with_timeout(&fixture, options, timeout)
            .unwrap_or_else(|error| {
                failed_parity_bench_case(&fixture, options, false, error.to_string())
            });
        case_results.push(case_result);
    }
    for mut fixture in suite.multi_input_fixtures {
        prepare_multi_input_fixture_for_cli_run(
            &mut fixture,
            &root,
            whisperx_command.as_ref(),
            &model_dir,
            args.model_cache_only,
        )?;
        let timeout = args
            .case_timeout_seconds
            .or(fixture.timeout_seconds)
            .map(Duration::from_secs);
        let options = BenchRunOptions {
            iterations: args.iterations,
            warmups: args.warmups,
            native_only: args.native_only,
        };
        let case_result =
            run_parity_bench_multi_input_case_with_timeout(&fixture, options, timeout)
                .unwrap_or_else(|error| {
                    failed_parity_bench_multi_input_case(
                        &fixture,
                        options,
                        false,
                        error.to_string(),
                    )
                });
        case_results.push(case_result);
    }

    let passed = case_results
        .iter()
        .all(|case| case["passed"].as_bool().unwrap_or(true));
    let report = serde_json::json!({
        "passed": passed,
        "iterations": args.iterations,
        "warmups": args.warmups,
        "nativeOnly": args.native_only,
        "caseTimeoutSeconds": args.case_timeout_seconds,
        "cases": case_results,
    });
    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_parity_bench_report(&report);
    }
    if !passed && !args.report_only {
        anyhow::bail!("parity benchmark gate failed");
    }
    Ok(())
}

pub(crate) fn parity_bench_case_command(args: ParityBenchCaseArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.fixture)
        .with_context(|| format!("failed to read {}", args.fixture.display()))?;
    let fixture: ParityFixtureCase = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.fixture.display()))?;
    set_ort_dylib_path_from_fixture_if_missing(&fixture);
    let options = BenchRunOptions {
        iterations: args.iterations,
        warmups: args.warmups,
        native_only: args.native_only,
    };
    let report = run_parity_bench_case(&fixture, options).unwrap_or_else(|error| {
        failed_parity_bench_case(&fixture, options, false, error.to_string())
    });
    fs::write(&args.report, serde_json::to_vec(&report)?)
        .with_context(|| format!("failed to write {}", args.report.display()))?;
    Ok(())
}

pub(crate) fn parity_bench_multi_input_case_command(
    args: ParityBenchMultiInputCaseArgs,
) -> anyhow::Result<()> {
    let bytes = fs::read(&args.fixture)
        .with_context(|| format!("failed to read {}", args.fixture.display()))?;
    let fixture: ParityMultiInputFixtureCase = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.fixture.display()))?;
    set_ort_dylib_path_from_multi_input_fixture_if_missing(&fixture);
    let options = BenchRunOptions {
        iterations: args.iterations,
        warmups: args.warmups,
        native_only: args.native_only,
    };
    let report = run_parity_bench_multi_input_case(&fixture, options).unwrap_or_else(|error| {
        failed_parity_bench_multi_input_case(&fixture, options, false, error.to_string())
    });
    fs::write(&args.report, serde_json::to_vec(&report)?)
        .with_context(|| format!("failed to write {}", args.report.display()))?;
    Ok(())
}

fn prepare_fixture_for_cli_run(
    fixture: &mut ParityFixtureCase,
    root: &Path,
    whisperx_command: Option<&PathBuf>,
    model_dir: &Path,
    model_cache_only: bool,
) {
    fixture.input = resolve_cli_path_with_root(fixture.input.clone(), root);
    if let Some(command) = whisperx_command {
        fixture.whisperx.command = command.clone();
    }
    fixture.native_asr.whisper_bundle = fixture
        .native_asr
        .whisper_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.native_asr.model_dir = Some(model_dir.to_path_buf());
    fixture.alignment.model_bundle = fixture
        .alignment
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.alignment.model_dir = Some(model_dir.to_path_buf());
    fixture.translation.model_bundle = fixture
        .translation
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.translation.model_dir = Some(model_dir.to_path_buf());
    fixture.vad.model_bundle = fixture
        .vad
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.diarization.model_bundle = fixture
        .diarization
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.diarization.speaker_embedding_model_bundle = fixture
        .diarization
        .speaker_embedding_model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    if model_cache_only {
        fixture.native_asr.model_cache_only = true;
        fixture.alignment.model_cache_only = true;
        fixture.translation.model_cache_only = true;
    }
    if fixture.output.output_dir.is_none() {
        fixture.output.output_dir = Some(root.join("out").join("parity-bench").join(&fixture.name));
    }
}

fn prepare_multi_input_fixture_for_cli_run(
    fixture: &mut ParityMultiInputFixtureCase,
    root: &Path,
    whisperx_command: Option<&PathBuf>,
    model_dir: &Path,
    model_cache_only: bool,
) -> anyhow::Result<()> {
    if fixture.inputs.is_empty() {
        anyhow::bail!(
            "parity benchmark multi-input case `{}` must contain at least one input",
            fixture.name
        );
    }
    if fixture.output.basename.is_some() {
        anyhow::bail!(
            "parity benchmark multi-input case `{}` cannot set output.basename",
            fixture.name
        );
    }
    fixture.inputs = fixture
        .inputs
        .iter()
        .cloned()
        .map(|input| resolve_cli_path_with_root(input, root))
        .collect();
    if let Some(command) = whisperx_command {
        fixture.whisperx.command = command.clone();
    }
    fixture.native_asr.whisper_bundle = fixture
        .native_asr
        .whisper_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.native_asr.model_dir = Some(model_dir.to_path_buf());
    fixture.alignment.model_bundle = fixture
        .alignment
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.alignment.model_dir = Some(model_dir.to_path_buf());
    fixture.translation.model_bundle = fixture
        .translation
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.translation.model_dir = Some(model_dir.to_path_buf());
    fixture.vad.model_bundle = fixture
        .vad
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.diarization.model_bundle = fixture
        .diarization
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.diarization.speaker_embedding_model_bundle = fixture
        .diarization
        .speaker_embedding_model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    if model_cache_only {
        fixture.native_asr.model_cache_only = true;
        fixture.alignment.model_cache_only = true;
        fixture.translation.model_cache_only = true;
    }
    if fixture.output.output_dir.is_none() {
        fixture.output.output_dir = Some(root.join("out").join("parity-bench").join(&fixture.name));
    }
    Ok(())
}

fn set_ort_dylib_path_from_fixture_if_missing(fixture: &ParityFixtureCase) {
    if std::env::var_os("ORT_DYLIB_PATH").is_some() {
        return;
    }
    let Some(path) = inferred_ort_dylib_path(fixture) else {
        return;
    };
    std::env::set_var("ORT_DYLIB_PATH", path);
}

fn set_ort_dylib_path_from_multi_input_fixture_if_missing(fixture: &ParityMultiInputFixtureCase) {
    if std::env::var_os("ORT_DYLIB_PATH").is_some() {
        return;
    }
    let Some(path) = inferred_ort_dylib_path_from_parts(&fixture.vad, &fixture.whisperx) else {
        return;
    };
    std::env::set_var("ORT_DYLIB_PATH", path);
}

fn inferred_ort_dylib_path(fixture: &ParityFixtureCase) -> Option<PathBuf> {
    inferred_ort_dylib_path_with_env(fixture, std::env::var_os("ORT_DYLIB_PATH"))
}

fn inferred_ort_dylib_path_with_env(
    fixture: &ParityFixtureCase,
    ort_dylib_path: Option<OsString>,
) -> Option<PathBuf> {
    if ort_dylib_path.is_some() {
        return None;
    }
    inferred_ort_dylib_path_from_parts(&fixture.vad, &fixture.whisperx)
}

fn inferred_ort_dylib_path_from_parts(
    vad: &VadConfig,
    whisperx: &ExternalWhisperxConfig,
) -> Option<PathBuf> {
    if !matches!(vad.method, VadMethod::Silero | VadMethod::Pyannote) {
        return None;
    }
    let env_root = whisperx.command.parent()?.parent()?;
    find_onnxruntime_dylib(env_root)
}

fn find_onnxruntime_dylib(env_root: &Path) -> Option<PathBuf> {
    let lib_dir = env_root.join("lib");
    let mut candidates = Vec::new();
    for python_dir in fs::read_dir(&lib_dir).ok()?.filter_map(Result::ok) {
        let file_name = python_dir.file_name();
        if !file_name.to_string_lossy().starts_with("python") {
            continue;
        }
        let capi_dir = python_dir
            .path()
            .join("site-packages")
            .join("onnxruntime")
            .join("capi");
        let Ok(entries) = fs::read_dir(capi_dir) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if name.starts_with("libonnxruntime.so") || name.starts_with("libonnxruntime.dylib") {
                candidates.push(path);
            }
        }
    }
    candidates.sort();
    candidates.into_iter().next()
}

#[derive(Debug, Clone, Copy)]
struct BenchRunOptions {
    iterations: usize,
    warmups: usize,
    native_only: bool,
}

fn run_parity_bench_case_with_timeout(
    fixture: &ParityFixtureCase,
    options: BenchRunOptions,
    timeout: Option<Duration>,
) -> anyhow::Result<serde_json::Value> {
    let Some(timeout) = timeout else {
        return run_parity_bench_case(fixture, options);
    };
    if timeout.is_zero() {
        return Ok(failed_parity_bench_case(
            fixture,
            options,
            true,
            format!(
                "parity benchmark case `{}` exceeded timeout of {}",
                fixture.name,
                format_duration(timeout)
            ),
        ));
    }

    let temp_prefix = parity_case_temp_prefix(&fixture.name);
    let fixture_path = temp_prefix.with_extension("bench-fixture.json");
    let report_path = temp_prefix.with_extension("bench-report.json");
    fs::write(&fixture_path, serde_json::to_vec(fixture)?)?;

    let result =
        run_parity_bench_case_child(&fixture_path, &report_path, fixture, options, timeout)
            .and_then(|status| {
                if !status.success() {
                    return Ok(failed_parity_bench_case(
                        fixture,
                        options,
                        false,
                        format!(
                            "parity benchmark case `{}` worker exited with status {status}",
                            fixture.name
                        ),
                    ));
                }
                let bytes = fs::read(&report_path).with_context(|| {
                    format!(
                        "parity benchmark case `{}` worker did not write {}",
                        fixture.name,
                        report_path.display()
                    )
                })?;
                serde_json::from_slice::<serde_json::Value>(&bytes).map_err(anyhow::Error::from)
            });

    let _ = fs::remove_file(&fixture_path);
    let _ = fs::remove_file(&report_path);

    match result {
        Ok(case) => Ok(case),
        Err(error) if is_timeout_error(&error.to_string()) => Ok(failed_parity_bench_case(
            fixture,
            options,
            true,
            error.to_string(),
        )),
        Err(error) => Err(error),
    }
}

fn run_parity_bench_case_child(
    fixture_path: &Path,
    report_path: &Path,
    fixture: &ParityFixtureCase,
    options: BenchRunOptions,
    timeout: Duration,
) -> anyhow::Result<ExitStatus> {
    let mut command = ProcessCommand::new(std::env::current_exe()?);
    command
        .arg("__parity-bench-case")
        .arg("--fixture")
        .arg(fixture_path)
        .arg("--report")
        .arg(report_path)
        .arg("--iterations")
        .arg(options.iterations.to_string())
        .arg("--warmups")
        .arg(options.warmups.to_string())
        .args(options.native_only.then_some("--native-only"))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if let Some(ort_dylib_path) = inferred_ort_dylib_path(fixture) {
        command.env("ORT_DYLIB_PATH", ort_dylib_path);
    }
    let mut child = command
        .spawn()
        .with_context(|| "failed to spawn parity benchmark case worker")?;

    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!(
                "parity benchmark case worker exceeded timeout of {}",
                format_duration(timeout)
            );
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn run_parity_bench_multi_input_case_with_timeout(
    fixture: &ParityMultiInputFixtureCase,
    options: BenchRunOptions,
    timeout: Option<Duration>,
) -> anyhow::Result<serde_json::Value> {
    let Some(timeout) = timeout else {
        return run_parity_bench_multi_input_case(fixture, options);
    };
    if timeout.is_zero() {
        return Ok(failed_parity_bench_multi_input_case(
            fixture,
            options,
            true,
            format!(
                "parity benchmark multi-input case `{}` exceeded timeout of {}",
                fixture.name,
                format_duration(timeout)
            ),
        ));
    }

    let temp_prefix = parity_case_temp_prefix(&fixture.name);
    let fixture_path = temp_prefix.with_extension("bench-multi-input-fixture.json");
    let report_path = temp_prefix.with_extension("bench-multi-input-report.json");
    fs::write(&fixture_path, serde_json::to_vec(fixture)?)?;

    let result = run_parity_bench_multi_input_case_child(
        &fixture_path,
        &report_path,
        fixture,
        options,
        timeout,
    )
    .and_then(|status| {
        if !status.success() {
            return Ok(failed_parity_bench_multi_input_case(
                fixture,
                options,
                false,
                format!(
                    "parity benchmark multi-input case `{}` worker exited with status {status}",
                    fixture.name
                ),
            ));
        }
        let bytes = fs::read(&report_path).with_context(|| {
            format!(
                "parity benchmark multi-input case `{}` worker did not write {}",
                fixture.name,
                report_path.display()
            )
        })?;
        serde_json::from_slice::<serde_json::Value>(&bytes).map_err(anyhow::Error::from)
    });

    let _ = fs::remove_file(&fixture_path);
    let _ = fs::remove_file(&report_path);

    match result {
        Ok(case) => Ok(case),
        Err(error) if is_timeout_error(&error.to_string()) => Ok(
            failed_parity_bench_multi_input_case(fixture, options, true, error.to_string()),
        ),
        Err(error) => Err(error),
    }
}

fn run_parity_bench_multi_input_case_child(
    fixture_path: &Path,
    report_path: &Path,
    fixture: &ParityMultiInputFixtureCase,
    options: BenchRunOptions,
    timeout: Duration,
) -> anyhow::Result<ExitStatus> {
    let mut command = ProcessCommand::new(std::env::current_exe()?);
    command
        .arg("__parity-bench-multi-input-case")
        .arg("--fixture")
        .arg(fixture_path)
        .arg("--report")
        .arg(report_path)
        .arg("--iterations")
        .arg(options.iterations.to_string())
        .arg("--warmups")
        .arg(options.warmups.to_string())
        .args(options.native_only.then_some("--native-only"))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if std::env::var_os("ORT_DYLIB_PATH").is_none() {
        if let Some(ort_dylib_path) =
            inferred_ort_dylib_path_from_parts(&fixture.vad, &fixture.whisperx)
        {
            command.env("ORT_DYLIB_PATH", ort_dylib_path);
        }
    }
    let mut child = command
        .spawn()
        .with_context(|| "failed to spawn parity benchmark multi-input case worker")?;

    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!(
                "parity benchmark multi-input case worker exceeded timeout of {}",
                format_duration(timeout)
            );
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn run_parity_bench_case(
    fixture: &ParityFixtureCase,
    options: BenchRunOptions,
) -> anyhow::Result<serde_json::Value> {
    for warmup in 0..options.warmups {
        eprintln!(
            "parity-bench: warming case {} iteration {}/{}",
            fixture.name,
            warmup + 1,
            options.warmups
        );
        run_single_bench_iteration(fixture, options.native_only, warmup + 1, true)?;
    }

    let mut iterations_json = Vec::with_capacity(options.iterations);
    for iteration in 0..options.iterations {
        eprintln!(
            "parity-bench: starting case {} iteration {}/{}",
            fixture.name,
            iteration + 1,
            options.iterations
        );
        iterations_json.push(run_single_bench_iteration(
            fixture,
            options.native_only,
            iteration + 1,
            false,
        )?);
    }
    let passed = iterations_json
        .iter()
        .all(bench_iteration_passes_speed_gate);
    Ok(serde_json::json!({
        "name": fixture.name,
        "gating": fixture.gating,
        "passed": passed,
        "timedOut": false,
        "nativeOnly": options.native_only,
        "warmups": options.warmups,
        "iterations": iterations_json,
    }))
}

fn run_parity_bench_multi_input_case(
    fixture: &ParityMultiInputFixtureCase,
    options: BenchRunOptions,
) -> anyhow::Result<serde_json::Value> {
    for warmup in 0..options.warmups {
        eprintln!(
            "parity-bench: warming multi-input case {} iteration {}/{}",
            fixture.name,
            warmup + 1,
            options.warmups
        );
        run_single_multi_input_bench_iteration(fixture, options.native_only, warmup + 1, true)?;
    }

    let mut iterations_json = Vec::with_capacity(options.iterations);
    for iteration in 0..options.iterations {
        eprintln!(
            "parity-bench: starting multi-input case {} iteration {}/{}",
            fixture.name,
            iteration + 1,
            options.iterations
        );
        iterations_json.push(run_single_multi_input_bench_iteration(
            fixture,
            options.native_only,
            iteration + 1,
            false,
        )?);
    }
    let passed = iterations_json
        .iter()
        .all(bench_iteration_passes_speed_gate);
    Ok(serde_json::json!({
        "name": fixture.name,
        "kind": "multiInput",
        "gating": fixture.gating,
        "passed": passed,
        "timedOut": false,
        "nativeOnly": options.native_only,
        "warmups": options.warmups,
        "inputCount": fixture.inputs.len(),
        "iterations": iterations_json,
    }))
}

fn run_single_bench_iteration(
    fixture: &ParityFixtureCase,
    native_only: bool,
    iteration: usize,
    warmup: bool,
) -> anyhow::Result<serde_json::Value> {
    let (native_report, native_elapsed) = timed_run(native_bench_config(fixture))?;
    let whisperx_run = if native_only {
        None
    } else {
        Some(timed_run(whisperx_bench_config(fixture))?)
    };
    let audio_duration = fixture
        .clip_seconds
        .or_else(|| inferred_audio_duration_seconds(&native_report))
        .or_else(|| {
            whisperx_run
                .as_ref()
                .and_then(|(report, _)| inferred_audio_duration_seconds(report))
        });
    let whisperx_json = whisperx_run
        .as_ref()
        .map(|(report, elapsed)| bench_run_json(report, *elapsed, audio_duration, false));
    let whisperx_elapsed = whisperx_run
        .as_ref()
        .map(|(_, elapsed)| duration_seconds(*elapsed));
    let whisperx_realtime = whisperx_run.as_ref().and_then(|(_, elapsed)| {
        audio_duration.map(|duration| duration_seconds(*elapsed) / duration)
    });
    let native_elapsed_seconds = duration_seconds(native_elapsed);
    let native_phases =
        bench_phase_json(&native_report.response.diagnostics, native_elapsed_seconds);
    let native_asr_batch_diagnostics =
        bench_asr_batch_diagnostics_json(&native_report.response.diagnostics);
    let speed = bench_speed_comparison(native_elapsed_seconds, whisperx_elapsed);
    let missing_required_diagnostics = missing_required_diagnostics(
        &fixture.required_diagnostics,
        &native_report.response.diagnostics,
    );
    Ok(serde_json::json!({
        "iteration": iteration,
        "warmup": warmup,
        "nativeElapsedSeconds": native_elapsed_seconds,
        "whisperxElapsedSeconds": whisperx_elapsed,
        "audioDurationSeconds": audio_duration,
        "nativeRealtimeFactor": audio_duration.map(|duration| native_elapsed_seconds / duration),
        "whisperxRealtimeFactor": whisperx_realtime,
        "nativeFasterThanWhisperx": speed.native_faster_than_whisperx,
        "nativeSpeedupRatio": speed.native_speedup_ratio,
        "nativeTotalSeconds": native_phases
            .get("nativeTotalSeconds")
            .and_then(serde_json::Value::as_f64),
        "decodeSeconds": native_phases
            .get("decodeSeconds")
            .and_then(serde_json::Value::as_f64),
        "vadSeconds": native_phases
            .get("vadSeconds")
            .and_then(serde_json::Value::as_f64),
        "asrSeconds": native_phases
            .get("asrSeconds")
            .and_then(serde_json::Value::as_f64),
        "alignmentSeconds": native_phases
            .get("alignmentSeconds")
            .and_then(serde_json::Value::as_f64),
        "diarizationSeconds": native_phases
            .get("diarizationSeconds")
            .and_then(serde_json::Value::as_f64),
        "outputSeconds": native_phases
            .get("outputSeconds")
            .and_then(serde_json::Value::as_f64),
        "peakRssBytes": serde_json::Value::Null,
        "cudaActive": diagnostic_bool(&native_report.response.diagnostics, "cuda"),
        "alignmentCudaActive": diagnostic_bool(&native_report.response.diagnostics, "alignmentCuda"),
        "alignmentDevice": diagnostic_value(&native_report.response.diagnostics, "alignmentDevice"),
        "modelId": fixture.native_asr.model_id,
        "chunkCount": diagnostic_value(&native_report.response.diagnostics, "chunkCount"),
        "batchCount": diagnostic_value(&native_report.response.diagnostics, "batchCount"),
        "batchExecution": diagnostic_value(&native_report.response.diagnostics, "batchExecution"),
        "asrBatchDiagnostics": native_asr_batch_diagnostics,
        "missingRequiredDiagnostics": missing_required_diagnostics,
        "alignmentBatchExecution": diagnostic_value(&native_report.response.diagnostics, "alignmentBatchExecution"),
        "diarizationWindowExecution": diagnostic_value(&native_report.response.diagnostics, "diarizationWindowExecution"),
        "nativeDiagnostics": native_report.response.diagnostics.clone(),
        "whisperxDiagnostics": whisperx_run
            .as_ref()
            .map(|(report, _)| report.response.diagnostics.clone())
            .unwrap_or_default(),
        "native": bench_run_json_from_phases(
            &native_report,
            native_elapsed_seconds,
            audio_duration,
            true,
            native_phases,
        ),
        "whisperx": whisperx_json,
    }))
}

fn run_single_multi_input_bench_iteration(
    fixture: &ParityMultiInputFixtureCase,
    native_only: bool,
    iteration: usize,
    warmup: bool,
) -> anyhow::Result<serde_json::Value> {
    let (native_reports, native_elapsed) =
        timed_run_many(native_multi_input_bench_configs(fixture))?;
    let whisperx_run = if native_only {
        None
    } else {
        Some(timed_run_many(whisperx_multi_input_bench_configs(fixture))?)
    };
    let audio_duration = fixture
        .clip_seconds_per_input
        .map(|duration| duration * fixture.inputs.len() as f64)
        .or_else(|| aggregate_audio_duration_seconds(&native_reports))
        .or_else(|| {
            whisperx_run
                .as_ref()
                .and_then(|(reports, _)| aggregate_audio_duration_seconds(reports))
        });
    let whisperx_elapsed = whisperx_run
        .as_ref()
        .map(|(_, elapsed)| duration_seconds(*elapsed));
    let whisperx_realtime = whisperx_run.as_ref().and_then(|(_, elapsed)| {
        audio_duration.map(|duration| duration_seconds(*elapsed) / duration)
    });
    let native_elapsed_seconds = duration_seconds(native_elapsed);
    let speed = bench_speed_comparison(native_elapsed_seconds, whisperx_elapsed);
    let missing_by_input =
        missing_required_diagnostics_by_input(&fixture.required_diagnostics, &native_reports);
    let missing_required_diagnostics = flatten_missing_required_diagnostics(&missing_by_input);
    let native_diagnostics = reports_diagnostics_json(&native_reports);
    let whisperx_diagnostics = whisperx_run
        .as_ref()
        .map(|(reports, _)| reports_diagnostics_json(reports))
        .unwrap_or_else(Vec::new);
    let native_inputs =
        multi_input_bench_runs_json(&native_reports, fixture.clip_seconds_per_input, true);
    let whisperx_inputs = whisperx_run.as_ref().map(|(reports, _)| {
        multi_input_bench_runs_json(reports, fixture.clip_seconds_per_input, false)
    });
    Ok(serde_json::json!({
        "iteration": iteration,
        "warmup": warmup,
        "inputCount": fixture.inputs.len(),
        "inputs": fixture
            .inputs
            .iter()
            .map(path_to_string)
            .collect::<Vec<_>>(),
        "nativeElapsedSeconds": native_elapsed_seconds,
        "whisperxElapsedSeconds": whisperx_elapsed,
        "audioDurationSeconds": audio_duration,
        "nativeRealtimeFactor": audio_duration.map(|duration| native_elapsed_seconds / duration),
        "whisperxRealtimeFactor": whisperx_realtime,
        "nativeFasterThanWhisperx": speed.native_faster_than_whisperx,
        "nativeSpeedupRatio": speed.native_speedup_ratio,
        "peakRssBytes": serde_json::Value::Null,
        "modelId": fixture.native_asr.model_id,
        "missingRequiredDiagnostics": missing_required_diagnostics,
        "missingRequiredDiagnosticsByInput": missing_by_input,
        "nativeDiagnostics": native_diagnostics,
        "whisperxDiagnostics": whisperx_diagnostics,
        "native": aggregate_multi_input_run_json(
            &native_reports,
            native_elapsed_seconds,
            audio_duration,
            true,
        ),
        "whisperx": whisperx_run
            .as_ref()
            .map(|(reports, elapsed)| {
                aggregate_multi_input_run_json(
                    reports,
                    duration_seconds(*elapsed),
                    audio_duration,
                    false,
                )
            }),
        "nativeInputs": native_inputs,
        "whisperxInputs": whisperx_inputs,
    }))
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BenchSpeedComparison {
    native_faster_than_whisperx: Option<bool>,
    native_speedup_ratio: Option<f64>,
}

fn bench_speed_comparison(
    native_elapsed_seconds: f64,
    whisperx_elapsed_seconds: Option<f64>,
) -> BenchSpeedComparison {
    let native_elapsed_seconds = finite_positive_seconds(native_elapsed_seconds);
    let whisperx_elapsed_seconds = whisperx_elapsed_seconds.and_then(finite_positive_seconds);
    BenchSpeedComparison {
        native_faster_than_whisperx: native_elapsed_seconds
            .zip(whisperx_elapsed_seconds)
            .map(|(native, whisperx)| native < whisperx),
        native_speedup_ratio: native_elapsed_seconds
            .zip(whisperx_elapsed_seconds)
            .map(|(native, whisperx)| whisperx / native),
    }
}

fn finite_positive_seconds(value: f64) -> Option<f64> {
    (value.is_finite() && value > 0.0).then_some(value)
}

fn bench_iteration_passes_speed_gate(iteration: &serde_json::Value) -> bool {
    let faster = iteration
        .get("nativeFasterThanWhisperx")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let speedup_passes = iteration
        .get("nativeSpeedupRatio")
        .and_then(serde_json::Value::as_f64)
        .filter(|speedup| speedup.is_finite() && *speedup >= 1.001)
        .is_some();
    let diagnostics_pass = iteration
        .get("missingRequiredDiagnostics")
        .and_then(serde_json::Value::as_array)
        .is_some_and(Vec::is_empty);
    faster && speedup_passes && diagnostics_pass
}

fn missing_required_diagnostics(required: &[String], diagnostics: &[String]) -> Vec<String> {
    required
        .iter()
        .filter(|required| !diagnostics.iter().any(|diagnostic| diagnostic == *required))
        .cloned()
        .collect()
}

fn failed_parity_bench_case(
    fixture: &ParityFixtureCase,
    options: BenchRunOptions,
    timed_out: bool,
    error: String,
) -> serde_json::Value {
    serde_json::json!({
        "name": fixture.name,
        "gating": fixture.gating,
        "passed": false,
        "timedOut": timed_out,
        "nativeOnly": options.native_only,
        "warmups": options.warmups,
        "iterations": [],
        "error": error,
    })
}

fn failed_parity_bench_multi_input_case(
    fixture: &ParityMultiInputFixtureCase,
    options: BenchRunOptions,
    timed_out: bool,
    error: String,
) -> serde_json::Value {
    serde_json::json!({
        "name": fixture.name,
        "kind": "multiInput",
        "gating": fixture.gating,
        "passed": false,
        "timedOut": timed_out,
        "nativeOnly": options.native_only,
        "warmups": options.warmups,
        "inputCount": fixture.inputs.len(),
        "iterations": [],
        "error": error,
    })
}

fn timed_run(
    config: NativeWhisperxConfig,
) -> anyhow::Result<(native_whisperx::NativeWhisperxReport, Duration)> {
    let start = Instant::now();
    let report = run(config).map_err(anyhow::Error::from)?;
    Ok((report, start.elapsed()))
}

fn timed_run_many(
    configs: Vec<NativeWhisperxConfig>,
) -> anyhow::Result<(Vec<native_whisperx::NativeWhisperxReport>, Duration)> {
    let start = Instant::now();
    let reports = run_many(configs).map_err(anyhow::Error::from)?;
    Ok((reports, start.elapsed()))
}

fn native_bench_config(fixture: &ParityFixtureCase) -> NativeWhisperxConfig {
    let mut asr = fixture.native_asr.clone();
    asr.provider = AsrProvider::Native;
    asr.language = fixture.language.clone();
    asr.max_batch_size = asr.max_batch_size.or(fixture.whisperx.batch_size);
    NativeWhisperxConfig {
        input: InputSource::Path {
            path: fixture.input.clone(),
        },
        asr,
        translation: fixture.translation.clone(),
        vad: fixture.vad.clone(),
        alignment: fixture.alignment.clone(),
        diarization: fixture.diarization.clone(),
        output: fixture.output.clone(),
    }
}

fn native_multi_input_bench_configs(
    fixture: &ParityMultiInputFixtureCase,
) -> Vec<NativeWhisperxConfig> {
    fixture
        .inputs
        .iter()
        .cloned()
        .map(|input| {
            let mut asr = fixture.native_asr.clone();
            asr.provider = AsrProvider::Native;
            asr.language = fixture.language.clone();
            asr.max_batch_size = asr.max_batch_size.or(fixture.whisperx.batch_size);
            NativeWhisperxConfig {
                input: InputSource::Path { path: input },
                asr,
                translation: fixture.translation.clone(),
                vad: fixture.vad.clone(),
                alignment: fixture.alignment.clone(),
                diarization: fixture.diarization.clone(),
                output: provider_output_config(&fixture.output, "native"),
            }
        })
        .collect()
}

fn whisperx_bench_config(fixture: &ParityFixtureCase) -> NativeWhisperxConfig {
    NativeWhisperxConfig {
        input: InputSource::Path {
            path: fixture.input.clone(),
        },
        asr: AsrConfig {
            provider: AsrProvider::ExternalWhisperX,
            task: fixture.native_asr.task,
            language: fixture.language.clone(),
            device: fixture.native_asr.device,
            device_index: fixture.native_asr.device_index.clone(),
            model_dir: fixture.native_asr.model_dir.clone(),
            model_cache_only: fixture.native_asr.model_cache_only
                || fixture.alignment.model_cache_only,
            max_batch_size: fixture.whisperx.batch_size,
            external_whisperx: fixture.whisperx.clone(),
            ..AsrConfig::default()
        },
        translation: TranslationConfig::default(),
        vad: fixture.vad.clone(),
        alignment: fixture.alignment.clone(),
        diarization: fixture
            .whisperx_diarization
            .clone()
            .unwrap_or_else(|| fixture.diarization.clone()),
        output: fixture.output.clone(),
    }
}

fn whisperx_multi_input_bench_configs(
    fixture: &ParityMultiInputFixtureCase,
) -> Vec<NativeWhisperxConfig> {
    fixture
        .inputs
        .iter()
        .cloned()
        .map(|input| NativeWhisperxConfig {
            input: InputSource::Path { path: input },
            asr: AsrConfig {
                provider: AsrProvider::ExternalWhisperX,
                task: fixture.native_asr.task,
                language: fixture.language.clone(),
                device: fixture.native_asr.device,
                device_index: fixture.native_asr.device_index.clone(),
                model_dir: fixture.native_asr.model_dir.clone(),
                model_cache_only: fixture.native_asr.model_cache_only
                    || fixture.alignment.model_cache_only,
                max_batch_size: fixture.whisperx.batch_size,
                external_whisperx: fixture.whisperx.clone(),
                ..AsrConfig::default()
            },
            translation: TranslationConfig::default(),
            vad: fixture.vad.clone(),
            alignment: fixture.alignment.clone(),
            diarization: fixture
                .whisperx_diarization
                .clone()
                .unwrap_or_else(|| fixture.diarization.clone()),
            output: provider_output_config(&fixture.output, "whisperx"),
        })
        .collect()
}

fn provider_output_config(output: &OutputConfig, provider: &str) -> OutputConfig {
    let mut output = output.clone();
    if let Some(output_dir) = &output.output_dir {
        output.output_dir = Some(output_dir.join(provider));
    }
    output
}

fn bench_run_json(
    report: &native_whisperx::NativeWhisperxReport,
    elapsed: Duration,
    audio_duration: Option<f64>,
    native: bool,
) -> serde_json::Value {
    let elapsed_seconds = duration_seconds(elapsed);
    let phases = bench_phase_json(&report.response.diagnostics, elapsed_seconds);
    bench_run_json_from_phases(report, elapsed_seconds, audio_duration, native, phases)
}

fn aggregate_multi_input_run_json(
    reports: &[native_whisperx::NativeWhisperxReport],
    elapsed_seconds: f64,
    audio_duration: Option<f64>,
    native: bool,
) -> serde_json::Value {
    serde_json::json!({
        "elapsedSeconds": elapsed_seconds,
        "realtimeFactor": audio_duration.map(|duration| elapsed_seconds / duration),
        "inputCount": reports.len(),
        "phases": aggregate_phase_json(reports, elapsed_seconds),
        "counters": aggregate_counter_json(reports),
        "runtime": aggregate_runtime_json(reports, native),
        "diagnostics": reports_diagnostics_json(reports),
    })
}

fn multi_input_bench_runs_json(
    reports: &[native_whisperx::NativeWhisperxReport],
    audio_duration_per_input: Option<f64>,
    native: bool,
) -> Vec<serde_json::Value> {
    reports
        .iter()
        .map(|report| {
            let elapsed_seconds =
                diagnostic_f64(&report.response.diagnostics, "phaseNativeTotalSeconds")
                    .unwrap_or(0.0);
            bench_run_json_from_phases(
                report,
                elapsed_seconds,
                audio_duration_per_input,
                native,
                bench_phase_json(&report.response.diagnostics, elapsed_seconds),
            )
        })
        .collect()
}

fn bench_run_json_from_phases(
    report: &native_whisperx::NativeWhisperxReport,
    elapsed_seconds: f64,
    audio_duration: Option<f64>,
    native: bool,
    phases: serde_json::Value,
) -> serde_json::Value {
    let diagnostics = &report.response.diagnostics;
    serde_json::json!({
        "elapsedSeconds": elapsed_seconds,
        "realtimeFactor": audio_duration.map(|duration| elapsed_seconds / duration),
        "phases": phases,
        "counters": bench_counter_json(diagnostics),
        "runtime": bench_runtime_json(diagnostics, native),
        "diagnostics": diagnostics,
    })
}

fn aggregate_phase_json(
    reports: &[native_whisperx::NativeWhisperxReport],
    total_elapsed_seconds: f64,
) -> serde_json::Value {
    serde_json::json!({
        "decodeSeconds": sum_diagnostic_f64(reports, "phaseDecodeSeconds"),
        "vadSeconds": sum_diagnostic_f64(reports, "phaseVadSeconds"),
        "asrSeconds": sum_diagnostic_f64(reports, "phaseAsrSeconds"),
        "alignmentSeconds": sum_diagnostic_f64(reports, "phaseAlignmentSeconds"),
        "diarizationSeconds": sum_diagnostic_f64(reports, "phaseDiarizationSeconds"),
        "outputSeconds": sum_diagnostic_f64(reports, "phaseOutputSeconds"),
        "nativeTotalSeconds": sum_diagnostic_f64(reports, "phaseNativeTotalSeconds"),
        "totalElapsedSeconds": total_elapsed_seconds,
    })
}

fn aggregate_counter_json(reports: &[native_whisperx::NativeWhisperxReport]) -> serde_json::Value {
    serde_json::json!({
        "decodeSamples": sum_diagnostic_usize(reports, "phaseDecodeSamples"),
        "vadSegments": sum_diagnostic_usize(reports, "phaseVadSegments"),
        "vadWindows": sum_diagnostic_usize(reports, "phaseVadWindows"),
        "asrSegments": sum_diagnostic_usize(reports, "phaseAsrSegments"),
        "alignmentWords": sum_diagnostic_usize(reports, "phaseAlignmentWords"),
        "diarizationSpeakers": sum_diagnostic_usize(reports, "phaseDiarizationSpeakers"),
        "diarizationSegments": sum_diagnostic_usize(reports, "phaseDiarizationSegments"),
        "chunkCount": sum_diagnostic_usize(reports, "chunkCount"),
        "batchCount": sum_diagnostic_usize(reports, "batchCount"),
        "modelLoadCount": reports
            .iter()
            .filter(|report| {
                report
                    .response
                    .diagnostics
                    .iter()
                    .any(|item| item.starts_with("asrModelId="))
            })
            .count(),
        "asrCacheHit": reports.iter().all(|report| {
            diagnostic_value(&report.response.diagnostics, "asrModelSource")
                .as_deref()
                == Some("hugging-face-cache")
        }),
    })
}

fn aggregate_runtime_json(
    reports: &[native_whisperx::NativeWhisperxReport],
    native: bool,
) -> serde_json::Value {
    if native {
        serde_json::json!({
            "provider": "native",
            "cudaActive": reports.iter().all(|report| {
                diagnostic_bool(&report.response.diagnostics, "cuda") == Some(true)
            }),
            "alignmentCudaActive": reports.iter().all(|report| {
                diagnostic_bool(&report.response.diagnostics, "alignmentCuda") == Some(true)
            }),
            "batchExecution": unique_diagnostic_values(reports, "batchExecution"),
            "alignmentBatchExecution": unique_diagnostic_values(reports, "alignmentBatchExecution"),
            "diarizationWindowExecution": unique_diagnostic_values(reports, "diarizationWindowExecution"),
        })
    } else {
        serde_json::json!({
            "provider": "whisperx",
        })
    }
}

fn bench_phase_json(diagnostics: &[String], total_elapsed_seconds: f64) -> serde_json::Value {
    serde_json::json!({
        "decodeSeconds": diagnostic_f64(diagnostics, "phaseDecodeSeconds"),
        "vadSeconds": diagnostic_f64(diagnostics, "phaseVadSeconds"),
        "asrSeconds": diagnostic_f64(diagnostics, "phaseAsrSeconds"),
        "alignmentSeconds": diagnostic_f64(diagnostics, "phaseAlignmentSeconds"),
        "diarizationSeconds": diagnostic_f64(diagnostics, "phaseDiarizationSeconds"),
        "outputSeconds": diagnostic_f64(diagnostics, "phaseOutputSeconds"),
        "nativeTotalSeconds": diagnostic_f64(diagnostics, "phaseNativeTotalSeconds"),
        "totalElapsedSeconds": total_elapsed_seconds,
    })
}

fn bench_counter_json(diagnostics: &[String]) -> serde_json::Value {
    let model_source = diagnostic_value(diagnostics, "asrModelSource");
    let asr_cache_hit = model_source.as_deref() == Some("hugging-face-cache");
    serde_json::json!({
        "decodeSamples": diagnostic_usize(diagnostics, "phaseDecodeSamples"),
        "vadSegments": diagnostic_usize(diagnostics, "phaseVadSegments"),
        "vadWindows": diagnostic_usize(diagnostics, "phaseVadWindows"),
        "asrSegments": diagnostic_usize(diagnostics, "phaseAsrSegments"),
        "alignmentWords": diagnostic_usize(diagnostics, "phaseAlignmentWords"),
        "diarizationSpeakers": diagnostic_usize(diagnostics, "phaseDiarizationSpeakers"),
        "diarizationSegments": diagnostic_usize(diagnostics, "phaseDiarizationSegments"),
        "chunkCount": diagnostic_usize(diagnostics, "chunkCount"),
        "batchCount": diagnostic_usize(diagnostics, "batchCount"),
        "modelLoadCount": if diagnostics.iter().any(|item| item.starts_with("asrModelId=")) { 1 } else { 0 },
        "asrCacheHitCount": if asr_cache_hit { 1 } else { 0 },
    })
}

fn bench_runtime_json(diagnostics: &[String], native: bool) -> serde_json::Value {
    serde_json::json!({
        "provider": if native { "native" } else { "whisperx" },
        "cuda": diagnostic_bool(diagnostics, "cuda"),
        "device": diagnostic_value(diagnostics, "device"),
        "alignmentCuda": diagnostic_bool(diagnostics, "alignmentCuda"),
        "alignmentDevice": diagnostic_value(diagnostics, "alignmentDevice"),
        "modelId": diagnostic_value(diagnostics, "asrModelId"),
        "modelSource": diagnostic_value(diagnostics, "asrModelSource"),
        "modelResolved": diagnostic_value(diagnostics, "asrModelResolved"),
        "modelRuntimeReused": false,
        "processReusedAcrossIterations": true,
        "asrBatchDiagnostics": if native {
            bench_asr_batch_diagnostics_json(diagnostics)
        } else {
            serde_json::Value::Null
        },
    })
}

fn bench_asr_batch_diagnostics_json(diagnostics: &[String]) -> serde_json::Value {
    serde_json::json!({
        "batchExecution": diagnostic_value(diagnostics, "batchExecution"),
        "activeRowCompaction": diagnostic_bool(diagnostics, "activeRowCompaction"),
        "activeRowCompactionCount": diagnostic_usize(diagnostics, "activeRowCompactionCount"),
        "completedRowCount": diagnostic_usize(diagnostics, "completedRowCount"),
        "effectiveActiveBatchSize": diagnostic_usize(diagnostics, "effectiveActiveBatchSize"),
        "effectiveActiveBatchSizes": diagnostic_usize_list(diagnostics, "effectiveActiveBatchSizes"),
        "effectiveMaxBatchSize": diagnostic_usize(diagnostics, "effectiveMaxBatchSize"),
        "cacheReuse": diagnostic_value(diagnostics, "cacheReuse"),
        "timestampTokensRequested": diagnostic_bool(diagnostics, "timestampTokensRequested"),
        "timestampTokensPresent": diagnostic_bool(diagnostics, "timestampTokensPresent"),
        "timestampSegmentsRejected": diagnostic_bool(diagnostics, "timestampSegmentsRejected"),
        "timingFallbacks": diagnostic_values(diagnostics, "timingFallback"),
    })
}

fn inferred_audio_duration_seconds(report: &native_whisperx::NativeWhisperxReport) -> Option<f64> {
    let transcript = serde_json::to_value(&report.response.transcript).ok()?;
    let segment_max = transcript
        .get("segments")
        .and_then(|segments| segments.as_array())
        .into_iter()
        .flatten()
        .filter_map(|segment| segment.get("end").and_then(|end| end.as_f64()))
        .fold(None, max_option_f64);
    let vad_max = report
        .response
        .vad_segments
        .iter()
        .map(|segment| segment.end_seconds)
        .fold(None, max_option_f64);
    match (segment_max, vad_max) {
        (Some(segment), Some(vad)) => Some(segment.max(vad)),
        (Some(segment), None) => Some(segment),
        (None, Some(vad)) => Some(vad),
        (None, None) => None,
    }
}

fn aggregate_audio_duration_seconds(
    reports: &[native_whisperx::NativeWhisperxReport],
) -> Option<f64> {
    let durations = reports
        .iter()
        .map(inferred_audio_duration_seconds)
        .collect::<Option<Vec<_>>>()?;
    Some(durations.into_iter().sum())
}

fn reports_diagnostics_json(reports: &[native_whisperx::NativeWhisperxReport]) -> Vec<Vec<String>> {
    reports
        .iter()
        .map(|report| report.response.diagnostics.clone())
        .collect()
}

fn missing_required_diagnostics_by_input(
    required: &[String],
    reports: &[native_whisperx::NativeWhisperxReport],
) -> Vec<serde_json::Value> {
    reports
        .iter()
        .enumerate()
        .filter_map(|(index, report)| {
            let missing = missing_required_diagnostics(required, &report.response.diagnostics);
            (!missing.is_empty()).then(|| {
                serde_json::json!({
                    "inputIndex": index,
                    "missing": missing,
                })
            })
        })
        .collect()
}

fn flatten_missing_required_diagnostics(missing_by_input: &[serde_json::Value]) -> Vec<String> {
    missing_by_input
        .iter()
        .filter_map(|entry| {
            let input_index = entry
                .get("inputIndex")
                .and_then(serde_json::Value::as_u64)?;
            let missing = entry.get("missing").and_then(serde_json::Value::as_array)?;
            Some(
                missing
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .map(|diagnostic| format!("input {input_index}: {diagnostic}"))
                    .collect::<Vec<_>>(),
            )
        })
        .flatten()
        .collect()
}

fn max_option_f64(max: Option<f64>, value: f64) -> Option<f64> {
    Some(max.map_or(value, |max| max.max(value)))
}

fn diagnostic_bool(diagnostics: &[String], key: &str) -> Option<bool> {
    diagnostic_value(diagnostics, key).and_then(|value| match value.as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    })
}

fn diagnostic_value(diagnostics: &[String], key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    diagnostics
        .iter()
        .find_map(|diagnostic| diagnostic.strip_prefix(&prefix).map(ToOwned::to_owned))
}

fn diagnostic_values(diagnostics: &[String], key: &str) -> Vec<String> {
    let prefix = format!("{key}=");
    diagnostics
        .iter()
        .filter_map(|diagnostic| diagnostic.strip_prefix(&prefix).map(ToOwned::to_owned))
        .collect()
}

fn diagnostic_f64(diagnostics: &[String], key: &str) -> Option<f64> {
    diagnostic_value(diagnostics, key).and_then(|value| value.parse::<f64>().ok())
}

fn diagnostic_usize(diagnostics: &[String], key: &str) -> Option<usize> {
    diagnostic_value(diagnostics, key).and_then(|value| value.parse::<usize>().ok())
}

fn diagnostic_usize_list(diagnostics: &[String], key: &str) -> Option<Vec<usize>> {
    let value = diagnostic_value(diagnostics, key)?;
    if let Ok(parsed) = serde_json::from_str::<Vec<usize>>(&value) {
        return Some(parsed);
    }
    let parsed = value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    (!parsed.is_empty()).then_some(parsed)
}

fn sum_diagnostic_f64(reports: &[native_whisperx::NativeWhisperxReport], key: &str) -> Option<f64> {
    let values = reports
        .iter()
        .map(|report| diagnostic_f64(&report.response.diagnostics, key))
        .collect::<Option<Vec<_>>>()?;
    Some(values.into_iter().sum())
}

fn sum_diagnostic_usize(
    reports: &[native_whisperx::NativeWhisperxReport],
    key: &str,
) -> Option<usize> {
    let values = reports
        .iter()
        .map(|report| diagnostic_usize(&report.response.diagnostics, key))
        .collect::<Option<Vec<_>>>()?;
    Some(values.into_iter().sum())
}

fn unique_diagnostic_values(
    reports: &[native_whisperx::NativeWhisperxReport],
    key: &str,
) -> Vec<String> {
    let mut values = reports
        .iter()
        .filter_map(|report| diagnostic_value(&report.response.diagnostics, key))
        .collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

fn print_parity_bench_report(report: &serde_json::Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".to_string())
    );
}

fn failed_parity_fixture_case(
    name: String,
    gating: bool,
    error: String,
) -> ParityFixtureCaseReport {
    ParityFixtureCaseReport {
        name,
        gating,
        passed: false,
        started_at: None,
        elapsed_seconds: None,
        timed_out: is_timeout_error(&error),
        report: None,
        missing_required_diagnostics: Vec::new(),
        expected_output_matches: Vec::new(),
        failure_summary: vec![error.clone()],
        error: Some(error),
    }
}

fn is_timeout_error(error: &str) -> bool {
    error.contains("exceeded timeout")
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let millis = duration.subsec_millis();
    if seconds == 0 {
        format!("{millis}ms")
    } else if millis == 0 {
        format!("{seconds}s")
    } else {
        format!("{seconds}.{millis:03}s")
    }
}

fn duration_seconds(duration: Duration) -> f64 {
    duration.as_secs_f64()
}

fn unix_timestamp_string(time: SystemTime) -> String {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("{}.{}", duration.as_secs(), duration.subsec_millis()),
        Err(_) => "0.000".to_string(),
    }
}

pub(crate) fn parity_summary_command(args: ParitySummaryArgs) -> anyhow::Result<()> {
    let report = match fs::read(&args.report) {
        Ok(bytes) => Some(
            serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", args.report.display()))?,
        ),
        Err(error) if args.allow_missing_report && error.kind() == std::io::ErrorKind::NotFound => {
            None
        }
        Err(error) => {
            return Err(error).with_context(|| format!("failed to read {}", args.report.display()));
        }
    };
    let preflight = match &args.preflight_report {
        Some(path) => {
            let bytes =
                fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
            Some(
                serde_json::from_slice(&bytes)
                    .with_context(|| format!("failed to parse {}", path.display()))?,
            )
        }
        None => None,
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&parity_summary_json(
            report.as_ref(),
            preflight.as_ref(),
            &args
        ))?
    );
    Ok(())
}

fn parity_summary_json(
    report: Option<&ParityFixtureSuiteReport>,
    preflight: Option<&native_whisperx::ParityPreflightReport>,
    args: &ParitySummaryArgs,
) -> serde_json::Value {
    let report_cases = report.map(|report| report.cases.as_slice()).unwrap_or(&[]);
    let preflight_passed = preflight.map(|report| report.passed).unwrap_or(true);
    let passed = report
        .map(|report| report.passed && preflight_passed)
        .unwrap_or(false);
    let mut gating_failures = report_cases
        .iter()
        .filter(|case| case.gating && !case.passed)
        .map(parity_case_gating_failure_json)
        .collect::<Vec<_>>();
    if let Some(preflight) = preflight {
        gating_failures.extend(
            preflight
                .cases
                .iter()
                .filter(|case| case.gating && !case.passed)
                .map(preflight_failure_json),
        );
    }
    let mut non_gating_failures = report_cases
        .iter()
        .filter(|case| !case.gating && !case.passed)
        .map(parity_case_non_gating_failure_json)
        .collect::<Vec<_>>();
    if let Some(preflight) = preflight {
        non_gating_failures.extend(
            preflight
                .cases
                .iter()
                .filter(|case| !case.gating && !case.passed)
                .map(preflight_failure_json),
        );
    }

    serde_json::json!({
        "passed": passed,
        "rawReportMissing": report.is_none(),
        "workflow": parity_workflow_metadata_json(args, preflight),
        "preflight": preflight.map(preflight_summary_json),
        "gatingFailures": gating_failures,
        "nonGatingFailures": non_gating_failures,
        "failedCases": report_cases
            .iter()
            .filter(|case| !case.passed)
            .map(parity_case_failure_json)
            .collect::<Vec<_>>(),
        "erroredCases": report_cases
            .iter()
            .filter(|case| case.error.is_some())
            .map(parity_case_failure_json)
            .collect::<Vec<_>>(),
        "skippedCases": skipped_cases_json(report, preflight),
        "cases": report_cases.iter().map(parity_case_summary_json).collect::<Vec<_>>(),
    })
}

fn parity_case_gating_failure_json(case: &ParityFixtureCaseReport) -> serde_json::Value {
    serde_json::json!({
        "kind": "fixture",
        "name": case.name,
        "strictComparisonFailures": strict_comparison_failures(case),
        "missingRequiredDiagnostics": case.missing_required_diagnostics,
        "elapsedSeconds": case.elapsed_seconds,
        "startedAt": case.started_at,
        "timedOut": case.timed_out,
    })
}

fn parity_case_non_gating_failure_json(case: &ParityFixtureCaseReport) -> serde_json::Value {
    serde_json::json!({
        "kind": "fixture",
        "name": case.name,
        "reportOnlyDifferences": report_only_differences(case),
        "strictComparisonFailures": strict_comparison_failures(case),
        "error": case.error,
        "elapsedSeconds": case.elapsed_seconds,
        "startedAt": case.started_at,
        "timedOut": case.timed_out,
    })
}

fn parity_case_failure_json(case: &ParityFixtureCaseReport) -> serde_json::Value {
    serde_json::json!({
        "kind": "fixture",
        "name": case.name,
        "gating": case.gating,
        "error": case.error,
        "elapsedSeconds": case.elapsed_seconds,
        "startedAt": case.started_at,
        "timedOut": case.timed_out,
    })
}

fn parity_case_summary_json(case: &ParityFixtureCaseReport) -> serde_json::Value {
    let expected_target = case
        .report
        .as_ref()
        .map(|report| serde_json::json!(report.expected_target));
    let strict_comparison_failures = strict_comparison_failures(case);
    let report_only_differences = report_only_differences(case);
    let expected_json_matches = case.report.as_ref().and_then(|report| {
        report.expected.as_ref().map(|_| {
            let text = report.expected_text_matches.unwrap_or(true);
            let segment_count = report.expected_segment_count_matches.unwrap_or(true);
            serde_json::json!({
                "passed": text && segment_count,
                "text": text,
                "segmentCount": segment_count,
            })
        })
    });

    serde_json::json!({
        "kind": "fixture",
        "name": case.name,
        "passed": case.passed,
        "status": parity_case_status(case),
        "gating": case.gating,
        "expectedTarget": expected_target,
        "strictComparisonFailures": strict_comparison_failures,
        "reportOnlyDifferences": report_only_differences,
        "expectedJsonMatches": expected_json_matches,
        "missingRequiredDiagnostics": case.missing_required_diagnostics,
        "elapsedSeconds": case.elapsed_seconds,
        "startedAt": case.started_at,
        "timedOut": case.timed_out,
    })
}

fn parity_case_status(case: &ParityFixtureCaseReport) -> &'static str {
    if case.passed {
        "passed"
    } else if case.timed_out {
        "timed-out"
    } else if case.error.is_some() {
        "errored"
    } else {
        "failed"
    }
}

fn parity_workflow_metadata_json(
    args: &ParitySummaryArgs,
    preflight: Option<&native_whisperx::ParityPreflightReport>,
) -> serde_json::Value {
    serde_json::json!({
        "suite": args.suite,
        "features": parse_feature_list(args.features.as_deref()),
        "runner": args.runner,
        "manifest": args
            .manifest
            .as_ref()
            .map(path_to_string)
            .or_else(|| preflight.map(|report| path_to_string(&report.manifest))),
        "outputDir": args.output_dir.as_ref().map(path_to_string),
        "rawReport": path_to_string(&args.report),
        "preflightReport": args.preflight_report.as_ref().map(path_to_string),
        "progressLog": args.progress_log.as_ref().map(path_to_string),
        "smokeRoot": args
            .smoke_root
            .as_ref()
            .map(path_to_string)
            .or_else(|| preflight.map(|report| path_to_string(&report.root))),
        "modelDir": args
            .model_dir
            .as_ref()
            .map(path_to_string)
            .or_else(|| preflight.map(|report| path_to_string(&report.model_dir))),
        "whisperxCommand": args
            .whisperx_command
            .as_ref()
            .map(path_to_string)
            .or_else(|| preflight.map(|report| path_to_string(&report.whisperx_command))),
        "ortDylibPath": args.ort_dylib_path.as_ref().map(path_to_string),
    })
}

fn parse_feature_list(features: Option<&str>) -> Vec<String> {
    features
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|feature| !feature.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn path_to_string(path: impl AsRef<Path>) -> String {
    path.as_ref().display().to_string()
}

fn preflight_summary_json(report: &native_whisperx::ParityPreflightReport) -> serde_json::Value {
    serde_json::json!({
        "passed": report.passed,
        "manifest": path_to_string(&report.manifest),
        "root": path_to_string(&report.root),
        "whisperxCommand": path_to_string(&report.whisperx_command),
        "modelDir": path_to_string(&report.model_dir),
        "sourceCheckoutTag": report.source_checkout_tag,
        "missingResources": report
            .cases
            .iter()
            .filter(|case| !case.passed)
            .map(preflight_failure_json)
            .collect::<Vec<_>>(),
        "cases": report.cases.iter().map(preflight_case_summary_json).collect::<Vec<_>>(),
    })
}

fn preflight_case_summary_json(
    case: &native_whisperx::ParityPreflightCaseReport,
) -> serde_json::Value {
    serde_json::json!({
        "kind": "preflight",
        "name": case.name,
        "passed": case.passed,
        "status": if case.passed { "passed" } else { "failed" },
        "gating": case.gating,
        "missing": case.missing,
        "warnings": case.warnings,
    })
}

fn preflight_failure_json(case: &native_whisperx::ParityPreflightCaseReport) -> serde_json::Value {
    serde_json::json!({
        "kind": "preflight",
        "name": case.name,
        "gating": case.gating,
        "missing": case.missing,
        "warnings": case.warnings,
    })
}

fn skipped_cases_json(
    report: Option<&ParityFixtureSuiteReport>,
    preflight: Option<&native_whisperx::ParityPreflightReport>,
) -> Vec<serde_json::Value> {
    if report.is_some() {
        return Vec::new();
    }
    let Some(preflight) = preflight else {
        return Vec::new();
    };
    let reason = if preflight.passed {
        "fixture report missing"
    } else {
        "preflight failed"
    };
    preflight
        .cases
        .iter()
        .map(|case| {
            serde_json::json!({
                "kind": "preflight",
                "name": case.name,
                "gating": case.gating,
                "reason": reason,
                "missing": case.missing,
                "warnings": case.warnings,
            })
        })
        .collect()
}

fn strict_comparison_failures(case: &ParityFixtureCaseReport) -> Vec<String> {
    let mut failures = Vec::new();
    if let Some(error) = &case.error {
        failures.push(error.clone());
    }
    if let Some(report) = &case.report {
        if !report.comparison.passed {
            failures.extend(
                report
                    .comparison
                    .differences
                    .iter()
                    .filter(|difference| !is_report_only_difference(difference))
                    .cloned(),
            );
        }
        if report.expected_text_matches == Some(false) {
            failures.push("expected transcript text differs".to_string());
        }
        if report.expected_segment_count_matches == Some(false) {
            failures.push("expected transcript segment count differs".to_string());
        }
    }
    failures.extend(
        case.expected_output_matches
            .iter()
            .filter(|output| output.gating && !output.passed)
            .filter_map(output_difference_summary),
    );
    failures.extend(
        case.missing_required_diagnostics
            .iter()
            .map(|diagnostic| format!("missing required diagnostic: {diagnostic}")),
    );
    failures
}

fn report_only_differences(case: &ParityFixtureCaseReport) -> Vec<String> {
    let mut differences = Vec::new();
    if let Some(report) = &case.report {
        differences.extend(report.comparison.diagnostic_differences.iter().cloned());
        differences.extend(
            report
                .comparison
                .differences
                .iter()
                .filter(|difference| is_report_only_difference(difference))
                .cloned(),
        );
    }
    differences.extend(
        case.expected_output_matches
            .iter()
            .filter(|output| !output.gating && !output.passed)
            .filter_map(output_difference_summary),
    );
    differences
}

fn is_report_only_difference(difference: &str) -> bool {
    difference.starts_with("report-only: ")
}

fn output_difference_summary(output: &native_whisperx::ExpectedOutputComparison) -> Option<String> {
    output.difference.as_ref().map(|difference| {
        format!(
            "{} {} output differs: {difference}",
            output.format.as_transcription_format(),
            output_comparison_name(output.comparison)
        )
    })
}

fn output_comparison_name(comparison: OutputComparisonMode) -> &'static str {
    match comparison {
        OutputComparisonMode::Exact => "exact",
        OutputComparisonMode::JsonSemantic => "jsonSemantic",
        OutputComparisonMode::SubtitleSemantic => "subtitleSemantic",
    }
}

pub(crate) fn parity_goldens_command(args: ParityGoldensArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.manifest)
        .with_context(|| format!("failed to read {}", args.manifest.display()))?;
    let mut suite: ParityFixtureSuite = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.manifest.display()))?;
    let root = smoke_root_or_arg(args.root, "parity-goldens")?;
    let whisperx_command = absolute_from_cwd(args.whisperx_command)?;
    let model_dir = args
        .model_dir
        .map(absolute_from_cwd)
        .transpose()?
        .unwrap_or_else(|| root.join("models"));
    let filters = args.cases.iter().cloned().collect::<HashSet<_>>();
    let mut selected = Vec::new();

    for mut fixture in suite.fixtures.drain(..) {
        if !args.include_non_gating && !fixture.gating {
            continue;
        }
        if !filters.is_empty() && !filters.contains(&fixture.name) {
            continue;
        }
        if fixture.expected_json.is_none() && fixture.expected_outputs.is_empty() {
            continue;
        }
        fixture.input = resolve_cli_path_with_root(fixture.input, &root);
        fixture.expected_json = fixture
            .expected_json
            .take()
            .map(|path| resolve_cli_path_with_root(path, &root));
        for output in &mut fixture.expected_outputs {
            output.path = resolve_cli_path_with_root(output.path.clone(), &root);
        }
        selected.push(fixture);
    }

    for case_name in &filters {
        if !suite_case_name_exists(&selected, case_name) {
            anyhow::bail!("no golden-generating case named `{case_name}` matched the manifest");
        }
    }

    if selected.is_empty() {
        println!("No golden-generating cases matched.");
        return Ok(());
    }

    for fixture in selected {
        let plan = build_golden_plan(
            &fixture,
            &root,
            &whisperx_command,
            &model_dir,
            args.model_cache_only,
        )?;
        ensure_golden_targets_can_write(&plan, args.overwrite, args.dry_run)?;
        if args.dry_run {
            print_golden_plan(&plan);
            continue;
        }
        fs::create_dir_all(&plan.generated_dir)
            .with_context(|| format!("failed to create {}", plan.generated_dir.display()))?;
        let status = ProcessCommand::new(&plan.command)
            .args(&plan.args)
            .status()
            .with_context(|| format!("failed to run {}", plan.command.display()))?;
        if !status.success() {
            anyhow::bail!(
                "WhisperX golden generation for `{}` failed with status {status}",
                fixture.name
            );
        }
        copy_golden_outputs(&plan, args.overwrite)?;
    }

    Ok(())
}

pub(crate) fn smoke_root_or_arg(root: Option<PathBuf>, command: &str) -> anyhow::Result<PathBuf> {
    let root = root
        .or_else(smoke_root_from_env_or_dotenv)
        .with_context(|| {
            format!("{command} requires --root, SMOKE_ROOT, or SMOKE_ROOT in .env for local audio, expected JSON, and model cache paths")
        })?;
    absolute_from_cwd(root)
}

fn smoke_root_from_env_or_dotenv() -> Option<PathBuf> {
    std::env::var_os("SMOKE_ROOT")
        .map(PathBuf::from)
        .or_else(|| dotenv_value("SMOKE_ROOT").map(PathBuf::from))
}

fn dotenv_value(key: &str) -> Option<String> {
    let contents = fs::read_to_string(".env").ok()?;
    for line in contents.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let trimmed = trimmed.strip_prefix("export ").unwrap_or(trimmed);
        let Some((candidate, value)) = trimmed.split_once('=') else {
            continue;
        };
        if candidate.trim() != key {
            continue;
        }
        let value = value.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .or_else(|| {
                value
                    .strip_prefix('\'')
                    .and_then(|value| value.strip_suffix('\''))
            })
            .unwrap_or(value);
        if value.is_empty() {
            return None;
        }
        return Some(value.to_string());
    }
    None
}

#[derive(Debug)]
struct GoldenPlan {
    case_name: String,
    command: PathBuf,
    args: Vec<String>,
    generated_dir: PathBuf,
    copies: Vec<GoldenCopy>,
}

#[derive(Debug)]
struct GoldenCopy {
    format: OutputFormat,
    source: PathBuf,
    target: PathBuf,
}

fn build_golden_plan(
    fixture: &ParityFixtureCase,
    root: &Path,
    whisperx_command: &Path,
    model_dir: &Path,
    model_cache_only: bool,
) -> anyhow::Result<GoldenPlan> {
    let generated_dir = root
        .join("expected")
        .join("whisperx-3.8.6")
        .join("generated")
        .join(&fixture.name);
    let requested_formats = golden_requested_formats(fixture)?;
    let output_format = golden_output_format(fixture, &requested_formats);
    let input_stem = fixture
        .input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .with_context(|| format!("input {} has no UTF-8 file stem", fixture.input.display()))?;

    let mut args = vec![
        fixture.input.display().to_string(),
        "--model".to_string(),
        fixture.whisperx.model.clone(),
        "--model_dir".to_string(),
        model_dir.display().to_string(),
    ];
    if model_cache_only
        || fixture.native_asr.model_cache_only
        || fixture.alignment.model_cache_only
        || fixture.translation.model_cache_only
    {
        args.extend(["--model_cache_only".to_string(), "True".to_string()]);
    }
    if let Some(language) = &fixture.language {
        args.extend(["--language".to_string(), language.clone()]);
    }
    match fixture.native_asr.device {
        DevicePreference::Auto => {}
        DevicePreference::Cpu => args.extend(["--device".to_string(), "cpu".to_string()]),
        DevicePreference::Cuda => args.extend(["--device".to_string(), "cuda".to_string()]),
    }
    if let Some(device_index) = &fixture.native_asr.device_index {
        args.extend(["--device_index".to_string(), device_index.clone()]);
    }
    if let Some(compute_type) = fixture
        .native_asr
        .compute_type
        .as_ref()
        .or(fixture.whisperx.compute_type.as_ref())
    {
        args.extend(["--compute_type".to_string(), compute_type.clone()]);
    }
    if let Some(batch_size) = fixture
        .native_asr
        .max_batch_size
        .or(fixture.whisperx.batch_size)
    {
        args.extend(["--batch_size".to_string(), batch_size.to_string()]);
    }
    args.extend(["--output_format".to_string(), output_format.to_string()]);
    args.extend([
        "--output_dir".to_string(),
        generated_dir.display().to_string(),
    ]);
    push_golden_args(fixture, &mut args)?;

    let mut copies = Vec::new();
    if let Some(expected_json) = &fixture.expected_json {
        copies.push(GoldenCopy {
            format: OutputFormat::Json,
            source: generated_dir.join(format!("{input_stem}.json")),
            target: expected_json.clone(),
        });
    }
    for expected_output in &fixture.expected_outputs {
        copies.push(GoldenCopy {
            format: expected_output.format,
            source: generated_dir.join(format!(
                "{input_stem}.{}",
                expected_output.format.extension()
            )),
            target: expected_output.path.clone(),
        });
    }
    copies = dedup_copies(copies);

    Ok(GoldenPlan {
        case_name: fixture.name.clone(),
        command: whisperx_command.to_path_buf(),
        args,
        generated_dir,
        copies,
    })
}

fn golden_requested_formats(fixture: &ParityFixtureCase) -> anyhow::Result<Vec<OutputFormat>> {
    let mut formats = Vec::new();
    if fixture.expected_json.is_some() {
        formats.push(OutputFormat::Json);
    }
    for ExpectedOutputFile { format, .. } in &fixture.expected_outputs {
        if *format == OutputFormat::NativeJson {
            anyhow::bail!(
                "case `{}` requests native-json, which Python WhisperX cannot generate",
                fixture.name
            );
        }
        formats.push(*format);
    }
    Ok(formats)
}

fn golden_output_format(fixture: &ParityFixtureCase, formats: &[OutputFormat]) -> &'static str {
    if fixture.output.formats.contains(&OutputFormat::All)
        || formats.contains(&OutputFormat::All)
        || formats.len() > 1
    {
        "all"
    } else {
        formats
            .first()
            .copied()
            .unwrap_or(OutputFormat::Json)
            .as_transcription_format()
    }
}

fn push_golden_args(fixture: &ParityFixtureCase, args: &mut Vec<String>) -> anyhow::Result<()> {
    args.extend([
        "--task".to_string(),
        fixture.native_asr.task.as_whisperx_arg().to_string(),
    ]);
    if !fixture.alignment.enabled {
        args.push("--no_align".to_string());
    } else {
        args.extend([
            "--align_model".to_string(),
            fixture
                .whisperx
                .align_model
                .clone()
                .unwrap_or_else(|| fixture.alignment.model_id.clone()),
        ]);
        if fixture.alignment.return_char_alignments {
            args.push("--return_char_alignments".to_string());
        }
    }
    if fixture.vad.method != VadMethod::Energy {
        args.extend([
            "--vad_method".to_string(),
            fixture.vad.method.as_whisperx_arg().to_string(),
        ]);
    }
    push_cli_arg_display(args, "--vad_onset", fixture.vad.onset);
    push_cli_arg_display(args, "--vad_offset", fixture.vad.offset);
    push_cli_arg_display(args, "--chunk_size", fixture.vad.chunk_size);

    let decode = &fixture.native_asr.decode;
    if !decode.temperature.is_empty() {
        args.extend([
            "--temperature".to_string(),
            decode
                .temperature
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ]);
    }
    push_cli_arg_display(args, "--best_of", decode.best_of);
    push_cli_arg_display(args, "--beam_size", decode.beam_size);
    push_cli_arg_display(args, "--patience", decode.patience);
    push_cli_arg_display(args, "--length_penalty", decode.length_penalty);
    push_cli_arg(args, "--suppress_tokens", decode.suppress_tokens.as_deref());
    if decode.suppress_numerals {
        args.push("--suppress_numerals".to_string());
    }
    push_cli_arg(args, "--initial_prompt", decode.initial_prompt.as_deref());
    push_cli_arg(args, "--hotwords", decode.hotwords.as_deref());
    push_cli_arg_bool(
        args,
        "--condition_on_previous_text",
        decode.condition_on_previous_text,
    );
    push_cli_arg_bool(args, "--fp16", decode.fp16);
    push_cli_arg_display(
        args,
        "--compression_ratio_threshold",
        decode.compression_ratio_threshold,
    );
    push_cli_arg_display(args, "--logprob_threshold", decode.logprob_threshold);
    push_cli_arg_display(args, "--no_speech_threshold", decode.no_speech_threshold);
    push_cli_arg_display(args, "--threads", decode.threads);

    let whisperx_diarization = fixture
        .whisperx_diarization
        .as_ref()
        .unwrap_or(&fixture.diarization);
    if whisperx_diarization.enabled {
        args.push("--diarize".to_string());
        args.extend([
            "--diarize_model".to_string(),
            whisperx_diarization.model_id.clone(),
        ]);
        push_cli_arg_display(args, "--min_speakers", whisperx_diarization.min_speakers);
        push_cli_arg_display(args, "--max_speakers", whisperx_diarization.max_speakers);
        if let Some(token) = fixture
            .whisperx_diarization
            .as_ref()
            .and_then(|diarization| diarization.hf_token.clone())
            .or_else(|| whisperx_diarization.hf_token.clone())
            .or_else(|| {
                whisperx_diarization
                    .hf_token_env
                    .as_ref()
                    .and_then(|name| std::env::var(name).ok())
            })
            .or_else(|| {
                fixture
                    .diarization
                    .hf_token_env
                    .as_ref()
                    .and_then(|name| std::env::var(name).ok())
            })
            .or_else(|| {
                fixture
                    .whisperx
                    .hf_token_env
                    .as_ref()
                    .and_then(|name| std::env::var(name).ok())
            })
        {
            args.extend(["--hf_token".to_string(), token]);
        }
    }
    if whisperx_diarization.return_speaker_embeddings {
        args.push("--speaker_embeddings".to_string());
    }
    push_cli_arg_display(
        args,
        "--max_line_width",
        fixture.output.subtitles.max_line_width,
    );
    push_cli_arg_display(
        args,
        "--max_line_count",
        fixture.output.subtitles.max_line_count,
    );
    if fixture.output.subtitles.highlight_words {
        args.extend(["--highlight_words".to_string(), "True".to_string()]);
    }
    args.extend([
        "--segment_resolution".to_string(),
        match fixture.output.subtitles.segment_resolution {
            SegmentResolution::Sentence => "sentence",
            SegmentResolution::Chunk => "chunk",
        }
        .to_string(),
    ]);
    args.extend(fixture.whisperx.extra_args.clone());
    Ok(())
}

fn push_cli_arg(args: &mut Vec<String>, flag: &str, value: Option<&str>) {
    if let Some(value) = value {
        args.extend([flag.to_string(), value.to_string()]);
    }
}

fn push_cli_arg_display<T: std::fmt::Display>(
    args: &mut Vec<String>,
    flag: &str,
    value: Option<T>,
) {
    if let Some(value) = value {
        args.extend([flag.to_string(), value.to_string()]);
    }
}

fn push_cli_arg_bool(args: &mut Vec<String>, flag: &str, value: Option<bool>) {
    if let Some(value) = value {
        args.extend([
            flag.to_string(),
            if value { "True" } else { "False" }.to_string(),
        ]);
    }
}

fn dedup_copies(copies: Vec<GoldenCopy>) -> Vec<GoldenCopy> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for copy in copies {
        if seen.insert(copy.target.clone()) {
            deduped.push(copy);
        }
    }
    deduped
}

fn ensure_golden_targets_can_write(
    plan: &GoldenPlan,
    overwrite: bool,
    dry_run: bool,
) -> anyhow::Result<()> {
    if overwrite || dry_run {
        return Ok(());
    }
    for copy in &plan.copies {
        if copy.target.exists() {
            anyhow::bail!(
                "refusing to overwrite existing golden {}; pass --overwrite",
                copy.target.display()
            );
        }
    }
    Ok(())
}

fn copy_golden_outputs(plan: &GoldenPlan, overwrite: bool) -> anyhow::Result<()> {
    for copy in &plan.copies {
        if copy.target.exists() && !overwrite {
            anyhow::bail!(
                "refusing to overwrite existing golden {}; pass --overwrite",
                copy.target.display()
            );
        }
        let parent = copy
            .target
            .parent()
            .with_context(|| format!("target {} has no parent", copy.target.display()))?;
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
        fs::copy(&copy.source, &copy.target).with_context(|| {
            format!(
                "failed to copy generated {} output from {} to {}",
                copy.format.as_transcription_format(),
                copy.source.display(),
                copy.target.display()
            )
        })?;
    }
    Ok(())
}

fn print_golden_plan(plan: &GoldenPlan) {
    println!("case: {}", plan.case_name);
    println!("command: {}", shell_command(&plan.command, &plan.args));
    for copy in &plan.copies {
        println!(
            "target: {} <= {}",
            copy.target.display(),
            copy.source.display()
        );
    }
}

fn shell_command(command: &Path, args: &[String]) -> String {
    std::iter::once(shell_quote(&command.display().to_string()))
        .chain(args.iter().map(|arg| shell_quote(arg)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn shell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || "-_./:=,".contains(character))
    {
        return value.to_string();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn validate_parity_bench_suite(suite: &ParityFixtureSuite) -> anyhow::Result<()> {
    if suite.fixtures.is_empty() && suite.multi_input_fixtures.is_empty() {
        anyhow::bail!("parity benchmark manifest must contain at least one fixture case");
    }

    let mut names = HashSet::new();
    for fixture in &suite.fixtures {
        if !names.insert(fixture.name.clone()) {
            anyhow::bail!("duplicate parity benchmark case name `{}`", fixture.name);
        }
    }
    for fixture in &suite.multi_input_fixtures {
        if !names.insert(fixture.name.clone()) {
            anyhow::bail!("duplicate parity benchmark case name `{}`", fixture.name);
        }
        if fixture.inputs.is_empty() {
            anyhow::bail!(
                "parity benchmark multi-input case `{}` must contain at least one input",
                fixture.name
            );
        }
        if fixture.output.basename.is_some() {
            anyhow::bail!(
                "parity benchmark multi-input case `{}` cannot set output.basename",
                fixture.name
            );
        }
    }
    Ok(())
}

fn bench_suite_case_name_exists(suite: &ParityFixtureSuite, name: &str) -> bool {
    suite_case_name_exists(&suite.fixtures, name)
        || suite
            .multi_input_fixtures
            .iter()
            .any(|case| case.name == name)
}

fn suite_case_name_exists(cases: &[ParityFixtureCase], name: &str) -> bool {
    cases.iter().any(|case| case.name == name)
}

fn resolve_cli_path_with_root(path: PathBuf, root: &Path) -> PathBuf {
    if path.is_relative() {
        root.join(path)
    } else {
        path
    }
}

#[cfg(test)]
#[cfg(test)]
#[path = "parity_tests.rs"]
mod tests;

//! `parity-fixtures` suite execution and per-case child processes.

use super::*;

pub(crate) fn parity_fixtures_command(args: ParityFixturesArgs) -> anyhow::Result<()> {
    ensure_whisperx_compat_enabled("Python WhisperX parity fixture suite")?;
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
        apply_fixture_model_options(fixture, model_dir.as_deref(), args.model_cache_only);
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

pub(super) fn run_parity_fixture_suite_with_progress(
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

pub(super) fn run_single_parity_fixture_case(
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

    let result =
        run_single_parity_fixture_case_child(&fixture_path, &root, &report_path, &fixture, timeout)
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
                serde_json::from_slice::<ParityFixtureCaseReport>(&bytes)
                    .map_err(anyhow::Error::from)
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

pub(super) fn run_single_parity_fixture_case_child(
    fixture_path: &Path,
    root: &Path,
    report_path: &Path,
    fixture: &ParityFixtureCase,
    timeout: Duration,
) -> anyhow::Result<ExitStatus> {
    let mut command = ProcessCommand::new(std::env::current_exe()?);
    command
        .arg("__parity-fixture-case")
        .arg("--fixture")
        .arg(fixture_path)
        .arg("--root")
        .arg(root)
        .arg("--report")
        .arg(report_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if let Some(ort_dylib_path) = inferred_ort_dylib_path(fixture) {
        command.env("ORT_DYLIB_PATH", ort_dylib_path);
    }
    let mut child = command
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

pub(super) fn parity_case_temp_prefix(case_name: &str) -> PathBuf {
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

pub(super) fn run_single_parity_fixture_case_now(
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
    ensure_whisperx_compat_enabled("Python WhisperX parity fixture suite")?;
    let bytes = fs::read(&args.fixture)
        .with_context(|| format!("failed to read {}", args.fixture.display()))?;
    let fixture: ParityFixtureCase = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.fixture.display()))?;
    let report = run_single_parity_fixture_case_now(fixture, args.root)?;
    fs::write(&args.report, serde_json::to_vec(&report)?)
        .with_context(|| format!("failed to write {}", args.report.display()))?;
    Ok(())
}

pub(super) fn failed_parity_fixture_case(
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

pub(super) fn suite_case_name_exists(cases: &[ParityFixtureCase], name: &str) -> bool {
    cases.iter().any(|case| case.name == name)
}

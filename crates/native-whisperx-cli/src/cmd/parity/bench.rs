//! `parity-bench` command, suite validation, and benchmark provenance.

use super::*;

pub(crate) fn parity_bench_command(args: ParityBenchArgs) -> anyhow::Result<()> {
    if !args.native_only {
        ensure_whisperx_compat_enabled("Python WhisperX parity benchmark oracle")?;
    }
    if args.iterations < 3 {
        anyhow::bail!("--iterations must be at least 3 for benchmark evidence");
    }
    if args.warmups < 1 {
        anyhow::bail!("--warmups must be at least 1 for benchmark evidence");
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
    let suite_model_ids = benchmark_suite_model_ids(&suite);

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
        "schemaVersion": 1,
        "passed": passed,
        "iterations": args.iterations,
        "warmups": args.warmups,
        "nativeOnly": args.native_only,
        "caseTimeoutSeconds": args.case_timeout_seconds,
        "provenance": benchmark_provenance_json(&suite_model_ids, &model_dir, args.model_revision.as_deref()),
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

pub(super) fn benchmark_suite_model_ids(suite: &ParityFixtureSuite) -> Vec<String> {
    let mut model_ids = suite
        .fixtures
        .iter()
        .map(|fixture| fixture.native_asr.model_id.clone())
        .chain(
            suite
                .multi_input_fixtures
                .iter()
                .map(|fixture| fixture.native_asr.model_id.clone()),
        )
        .collect::<Vec<_>>();
    model_ids.sort();
    model_ids.dedup();
    model_ids
}

pub(super) fn benchmark_provenance_json(
    model_ids: &[String],
    model_dir: &Path,
    model_revision_override: Option<&str>,
) -> serde_json::Value {
    let git_sha = std::env::var("GITHUB_SHA")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| process_stdout("git", &["rev-parse", "HEAD"]));
    let cpu = fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|contents| {
            contents.lines().find_map(|line| {
                let (key, value) = line.split_once(':')?;
                (key.trim() == "model name").then(|| value.trim().to_string())
            })
        });
    let cuda = process_stdout(
        "nvidia-smi",
        &["--query-gpu=name,uuid", "--format=csv,noheader"],
    )
    .map(|value| value.lines().map(ToOwned::to_owned).collect::<Vec<_>>())
    .unwrap_or_default();

    serde_json::json!({
        "gitSha": git_sha,
        "crateVersions": {
            "native-whisperx": env!("CARGO_PKG_VERSION"),
            "native-whisperx-cli": env!("CARGO_PKG_VERSION"),
        },
        "models": model_ids
            .iter()
            .map(|model_id| serde_json::json!({
                "id": model_id,
                "revision": model_revision_override
                    .map(ToOwned::to_owned)
                    .or_else(|| cached_model_revision(model_dir, model_id)),
            }))
            .collect::<Vec<_>>(),
        "deviceIdentity": {
            "cpu": cpu,
            "cuda": cuda,
        },
        "driverRuntimeVersions": {
            "nvidiaDriver": process_stdout(
                "nvidia-smi",
                &["--query-gpu=driver_version", "--format=csv,noheader,nounits"],
            ),
            "cudaRuntime": cuda_runtime_version(),
            "rustc": process_stdout("rustc", &["--version"]),
        },
    })
}

pub(super) fn cuda_runtime_version() -> Option<String> {
    std::env::var("CUDA_VERSION")
        .ok()
        .filter(|version| !version.trim().is_empty())
        .or_else(|| process_stdout("nvcc", &["--version"]))
        .or_else(|| {
            let output = process_stdout("nvidia-smi", &[])?;
            let version = output
                .split("CUDA Version:")
                .nth(1)?
                .split_whitespace()
                .next()?;
            Some(version.to_string())
        })
}

pub(super) fn cached_model_revision(model_dir: &Path, model_id: &str) -> Option<String> {
    let repository = if model_id.contains('/') {
        model_id.to_string()
    } else {
        format!("openai/whisper-{model_id}")
    };
    let cache_name = format!("models--{}", repository.replace('/', "--"));
    [
        model_dir.join(&cache_name),
        model_dir.join("hub").join(cache_name),
    ]
    .into_iter()
    .find_map(|repository_dir| fs::read_to_string(repository_dir.join("refs/main")).ok())
    .map(|revision| revision.trim().to_string())
    .filter(|revision| !revision.is_empty())
}

pub(super) fn process_stdout(program: &str, args: &[&str]) -> Option<String> {
    let output = ProcessCommand::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let output = String::from_utf8(output.stdout).ok()?;
    let output = output.trim();
    (!output.is_empty()).then(|| output.to_string())
}

pub(super) fn print_parity_bench_report(report: &serde_json::Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".to_string())
    );
}

pub(super) fn validate_parity_bench_suite(suite: &ParityFixtureSuite) -> anyhow::Result<()> {
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

pub(super) fn bench_suite_case_name_exists(suite: &ParityFixtureSuite, name: &str) -> bool {
    suite_case_name_exists(&suite.fixtures, name)
        || suite
            .multi_input_fixtures
            .iter()
            .any(|case| case.name == name)
}

//! Benchmark case execution: timeouts, child processes, iterations, and gates.

use super::*;

pub(crate) fn parity_bench_case_command(args: ParityBenchCaseArgs) -> anyhow::Result<()> {
    if !args.native_only {
        ensure_whisperx_compat_enabled("Python WhisperX parity benchmark oracle")?;
    }
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
    if !args.native_only {
        ensure_whisperx_compat_enabled("Python WhisperX parity benchmark oracle")?;
    }
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

#[derive(Debug, Clone, Copy)]
pub(super) struct BenchRunOptions {
    pub(super) iterations: usize,
    pub(super) warmups: usize,
    pub(super) native_only: bool,
}

pub(super) fn run_parity_bench_case_with_timeout(
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

pub(super) fn run_parity_bench_case_child(
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

pub(super) fn run_parity_bench_multi_input_case_with_timeout(
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

pub(super) fn run_parity_bench_multi_input_case_child(
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
        if let Some(ort_dylib_path) = inferred_ort_dylib_path_from_parts(
            &fixture.vad,
            &fixture.diarization,
            &fixture.whisperx,
        ) {
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

pub(super) fn run_parity_bench_case(
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
        .all(|iteration| bench_iteration_passes_gate(iteration, fixture.benchmark_gate));
    Ok(serde_json::json!({
        "name": fixture.name,
        "gating": fixture.gating,
        "benchmarkGate": fixture.benchmark_gate,
        "passed": passed,
        "timedOut": false,
        "nativeOnly": options.native_only,
        "warmups": options.warmups,
        "iterations": iterations_json,
    }))
}

pub(super) fn run_parity_bench_multi_input_case(
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
        .all(|iteration| bench_iteration_passes_gate(iteration, fixture.benchmark_gate));
    Ok(serde_json::json!({
        "name": fixture.name,
        "kind": "multiInput",
        "gating": fixture.gating,
        "benchmarkGate": fixture.benchmark_gate,
        "passed": passed,
        "timedOut": false,
        "nativeOnly": options.native_only,
        "warmups": options.warmups,
        "inputCount": fixture.inputs.len(),
        "iterations": iterations_json,
    }))
}

pub(super) fn run_single_bench_iteration(
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
    let native_phases = bench_phase_json(&native_report.diagnostics, native_elapsed_seconds);
    let native_asr_batch_diagnostics = bench_asr_batch_diagnostics_json(&native_report.diagnostics);
    let speed = bench_speed_comparison(native_elapsed_seconds, whisperx_elapsed);
    let missing_required_diagnostics =
        missing_required_diagnostics(&fixture.required_diagnostics, &native_report.diagnostics);
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
        "cudaActive": diagnostic_bool(&native_report.diagnostics, "cuda"),
        "alignmentCudaActive": diagnostic_bool(&native_report.diagnostics, "alignmentCuda"),
        "alignmentDevice": diagnostic_value(&native_report.diagnostics, "alignmentDevice"),
        "modelId": fixture.native_asr.model_id,
        "chunkCount": diagnostic_value(&native_report.diagnostics, "chunkCount"),
        "batchCount": diagnostic_value(&native_report.diagnostics, "batchCount"),
        "batchExecution": diagnostic_value(&native_report.diagnostics, "batchExecution"),
        "asrBatchDiagnostics": native_asr_batch_diagnostics,
        "missingRequiredDiagnostics": missing_required_diagnostics,
        "alignmentBatchExecution": diagnostic_value(&native_report.diagnostics, "alignmentBatchExecution"),
        "diarizationWindowExecution": diagnostic_value(&native_report.diagnostics, "diarizationWindowExecution"),
        "nativeDiagnostics": native_report.diagnostics.clone(),
        "whisperxDiagnostics": whisperx_run
            .as_ref()
            .map(|(report, _)| report.diagnostics.clone())
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

pub(super) fn run_single_multi_input_bench_iteration(
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
pub(super) struct BenchSpeedComparison {
    pub(super) native_faster_than_whisperx: Option<bool>,
    pub(super) native_speedup_ratio: Option<f64>,
}

pub(super) fn bench_speed_comparison(
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

pub(super) fn finite_positive_seconds(value: f64) -> Option<f64> {
    (value.is_finite() && value > 0.0).then_some(value)
}

pub(super) fn bench_iteration_passes_speed_gate(iteration: &serde_json::Value) -> bool {
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

pub(super) fn bench_iteration_passes_gate(
    iteration: &serde_json::Value,
    gate: ParityBenchmarkGate,
) -> bool {
    match gate {
        ParityBenchmarkGate::NativeBeatsWhisperxEveryIteration => {
            bench_iteration_passes_speed_gate(iteration)
        }
        ParityBenchmarkGate::Comparative => bench_iteration_is_comparable(iteration),
    }
}

pub(super) fn bench_iteration_is_comparable(iteration: &serde_json::Value) -> bool {
    let has_finite_positive_timing = |key| {
        iteration
            .get(key)
            .and_then(serde_json::Value::as_f64)
            .is_some_and(|seconds| seconds.is_finite() && seconds > 0.0)
    };
    let diagnostics_pass = iteration
        .get("missingRequiredDiagnostics")
        .and_then(serde_json::Value::as_array)
        .is_some_and(Vec::is_empty);
    has_finite_positive_timing("nativeElapsedSeconds")
        && has_finite_positive_timing("whisperxElapsedSeconds")
        && diagnostics_pass
}

pub(super) fn missing_required_diagnostics(
    required: &[String],
    diagnostics: &[String],
) -> Vec<String> {
    required
        .iter()
        .filter(|required| !diagnostics.iter().any(|diagnostic| diagnostic == *required))
        .cloned()
        .collect()
}

pub(super) fn failed_parity_bench_case(
    fixture: &ParityFixtureCase,
    options: BenchRunOptions,
    timed_out: bool,
    error: String,
) -> serde_json::Value {
    serde_json::json!({
        "name": fixture.name,
        "gating": fixture.gating,
        "benchmarkGate": fixture.benchmark_gate,
        "passed": false,
        "timedOut": timed_out,
        "nativeOnly": options.native_only,
        "warmups": options.warmups,
        "iterations": [],
        "error": error,
    })
}

pub(super) fn failed_parity_bench_multi_input_case(
    fixture: &ParityMultiInputFixtureCase,
    options: BenchRunOptions,
    timed_out: bool,
    error: String,
) -> serde_json::Value {
    serde_json::json!({
        "name": fixture.name,
        "kind": "multiInput",
        "gating": fixture.gating,
        "benchmarkGate": fixture.benchmark_gate,
        "passed": false,
        "timedOut": timed_out,
        "nativeOnly": options.native_only,
        "warmups": options.warmups,
        "inputCount": fixture.inputs.len(),
        "iterations": [],
        "error": error,
    })
}

pub(super) fn timed_run(
    config: NativeWhisperxConfig,
) -> anyhow::Result<(native_whisperx::NativeWhisperxReport, Duration)> {
    let start = Instant::now();
    let report = match config.asr.provider {
        AsrProvider::Native => run(config),
        AsrProvider::ExternalWhisperX => native_whisperx::run_whisperx_oracle(config),
    }
    .map_err(anyhow::Error::from)?;
    Ok((report, start.elapsed()))
}

pub(super) fn timed_run_many(
    configs: Vec<NativeWhisperxConfig>,
) -> anyhow::Result<(Vec<native_whisperx::NativeWhisperxReport>, Duration)> {
    let start = Instant::now();
    let reports = if configs
        .iter()
        .all(|config| config.asr.provider == AsrProvider::Native)
    {
        run_many(configs)?
    } else {
        configs
            .into_iter()
            .map(|config| match config.asr.provider {
                AsrProvider::Native => run(config),
                AsrProvider::ExternalWhisperX => native_whisperx::run_whisperx_oracle(config),
            })
            .collect::<Result<Vec<_>, _>>()?
    };
    Ok((reports, start.elapsed()))
}

pub(super) fn native_bench_config(fixture: &ParityFixtureCase) -> NativeWhisperxConfig {
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

pub(super) fn native_multi_input_bench_configs(
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

pub(super) fn whisperx_bench_config(fixture: &ParityFixtureCase) -> NativeWhisperxConfig {
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

pub(super) fn whisperx_multi_input_bench_configs(
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

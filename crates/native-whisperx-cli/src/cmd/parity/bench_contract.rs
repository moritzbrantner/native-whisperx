//! `parity-bench-summary` command and the benchmark report contract.

use super::*;

pub(crate) fn parity_bench_summary_command(args: ParityBenchSummaryArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.report)
        .with_context(|| format!("failed to read {}", args.report.display()))?;
    let report: serde_json::Value = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.report.display()))?;
    let summary = parity_bench_summary_json(&report)?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

pub(super) fn parity_bench_summary_json(
    report: &serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    validate_parity_bench_report_contract(report)?;
    let cases = report["cases"]
        .as_array()
        .expect("validated benchmark cases")
        .iter()
        .map(|case| {
            let iterations = case["iterations"]
                .as_array()
                .expect("validated benchmark iterations")
                .iter()
                .map(benchmark_iteration_summary_json)
                .collect::<Vec<_>>();
            serde_json::json!({
                "name": case.get("name"),
                "gating": case.get("gating"),
                "benchmarkGate": case.get("benchmarkGate"),
                "passed": case.get("passed"),
                "timedOut": case.get("timedOut"),
                "nativeOnly": case.get("nativeOnly"),
                "warmups": case.get("warmups"),
                "iterations": iterations,
            })
        })
        .collect::<Vec<_>>();
    Ok(serde_json::json!({
        "schemaVersion": report.get("schemaVersion"),
        "passed": report.get("passed"),
        "sampling": {
            "warmups": report.get("warmups"),
            "measuredIterations": report.get("iterations"),
        },
        "provenance": report.get("provenance"),
        "cases": cases,
    }))
}

pub(super) fn benchmark_iteration_summary_json(iteration: &serde_json::Value) -> serde_json::Value {
    const FIELDS: &[&str] = &[
        "iteration",
        "warmup",
        "nativeElapsedSeconds",
        "whisperxElapsedSeconds",
        "audioDurationSeconds",
        "nativeRealtimeFactor",
        "whisperxRealtimeFactor",
        "nativeFasterThanWhisperx",
        "nativeSpeedupRatio",
        "nativeTotalSeconds",
        "decodeSeconds",
        "vadSeconds",
        "asrSeconds",
        "alignmentSeconds",
        "diarizationSeconds",
        "outputSeconds",
        "peakRssBytes",
        "cudaActive",
        "alignmentCudaActive",
        "alignmentDevice",
        "modelId",
        "chunkCount",
        "batchCount",
        "batchExecution",
        "asrBatchDiagnostics",
        "missingRequiredDiagnostics",
        "alignmentBatchExecution",
        "diarizationWindowExecution",
    ];
    let mut summary = serde_json::Map::new();
    for field in FIELDS {
        if let Some(value) = iteration.get(*field) {
            summary.insert((*field).to_string(), value.clone());
        }
    }
    serde_json::Value::Object(summary)
}

pub(super) fn validate_parity_bench_report_contract(
    report: &serde_json::Value,
) -> anyhow::Result<()> {
    if report
        .get("schemaVersion")
        .and_then(serde_json::Value::as_u64)
        != Some(1)
    {
        anyhow::bail!("benchmark report schemaVersion must be 1");
    }
    if report
        .get("iterations")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0)
        < 3
    {
        anyhow::bail!("benchmark report must contain at least 3 measured iterations");
    }
    if report
        .get("warmups")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0)
        < 1
    {
        anyhow::bail!("benchmark report must record at least 1 warm-up");
    }
    let provenance = report
        .get("provenance")
        .and_then(serde_json::Value::as_object)
        .context("benchmark report must include provenance")?;
    for field in [
        "gitSha",
        "crateVersions",
        "models",
        "deviceIdentity",
        "driverRuntimeVersions",
    ] {
        if !provenance.contains_key(field) {
            anyhow::bail!("benchmark provenance must include {field}");
        }
    }
    if provenance
        .get("gitSha")
        .and_then(serde_json::Value::as_str)
        .is_none_or(str::is_empty)
    {
        anyhow::bail!("benchmark provenance gitSha must be non-empty");
    }
    if provenance
        .get("crateVersions")
        .and_then(serde_json::Value::as_object)
        .is_none_or(serde_json::Map::is_empty)
    {
        anyhow::bail!("benchmark provenance crateVersions must be non-empty");
    }
    let models = provenance
        .get("models")
        .and_then(serde_json::Value::as_array)
        .filter(|models| !models.is_empty())
        .context("benchmark provenance models must be non-empty")?;
    for model in models {
        for field in ["id", "revision"] {
            if model
                .get(field)
                .and_then(serde_json::Value::as_str)
                .is_none_or(str::is_empty)
            {
                anyhow::bail!("benchmark model {field} must be non-empty");
            }
        }
    }
    let cases = report
        .get("cases")
        .and_then(serde_json::Value::as_array)
        .filter(|cases| !cases.is_empty())
        .context("benchmark report must contain at least one case")?;
    for case in cases {
        let name = case
            .get("name")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("<unnamed>");
        if case
            .get("warmups")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
            < 1
        {
            anyhow::bail!("benchmark case `{name}` must record at least 1 warm-up");
        }
        let iterations = case
            .get("iterations")
            .and_then(serde_json::Value::as_array)
            .context("benchmark case iterations must be an array")?;
        if iterations.len() < 3 {
            anyhow::bail!("benchmark case `{name}` must contain at least 3 measured iterations");
        }
        for iteration in iterations {
            for field in [
                "nativeElapsedSeconds",
                "whisperxElapsedSeconds",
                "nativeTotalSeconds",
                "decodeSeconds",
                "vadSeconds",
                "asrSeconds",
                "alignmentSeconds",
                "diarizationSeconds",
                "outputSeconds",
                "batchCount",
                "batchExecution",
                "asrBatchDiagnostics",
                "missingRequiredDiagnostics",
            ] {
                if iteration.get(field).is_none() {
                    anyhow::bail!(
                        "benchmark case `{name}` iteration is missing evidence field {field}"
                    );
                }
            }
        }
    }
    Ok(())
}

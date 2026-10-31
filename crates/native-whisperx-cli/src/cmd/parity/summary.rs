//! `parity-summary` command and fixture report summaries.

use super::*;

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

pub(super) fn parity_summary_json(
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

pub(super) fn parity_case_gating_failure_json(case: &ParityFixtureCaseReport) -> serde_json::Value {
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

pub(super) fn parity_case_non_gating_failure_json(
    case: &ParityFixtureCaseReport,
) -> serde_json::Value {
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

pub(super) fn parity_case_failure_json(case: &ParityFixtureCaseReport) -> serde_json::Value {
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

pub(super) fn parity_case_summary_json(case: &ParityFixtureCaseReport) -> serde_json::Value {
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

pub(super) fn parity_case_status(case: &ParityFixtureCaseReport) -> &'static str {
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

pub(super) fn parity_workflow_metadata_json(
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

pub(super) fn parse_feature_list(features: Option<&str>) -> Vec<String> {
    features
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|feature| !feature.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

pub(super) fn path_to_string(path: impl AsRef<Path>) -> String {
    path.as_ref().display().to_string()
}

pub(super) fn preflight_summary_json(
    report: &native_whisperx::ParityPreflightReport,
) -> serde_json::Value {
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

pub(super) fn preflight_case_summary_json(
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

pub(super) fn preflight_failure_json(
    case: &native_whisperx::ParityPreflightCaseReport,
) -> serde_json::Value {
    serde_json::json!({
        "kind": "preflight",
        "name": case.name,
        "gating": case.gating,
        "missing": case.missing,
        "warnings": case.warnings,
    })
}

pub(super) fn skipped_cases_json(
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

pub(super) fn strict_comparison_failures(case: &ParityFixtureCaseReport) -> Vec<String> {
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

pub(super) fn report_only_differences(case: &ParityFixtureCaseReport) -> Vec<String> {
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

pub(super) fn is_report_only_difference(difference: &str) -> bool {
    difference.starts_with("report-only: ")
}

pub(super) fn output_difference_summary(
    output: &native_whisperx::ExpectedOutputComparison,
) -> Option<String> {
    output.difference.as_ref().map(|difference| {
        format!(
            "{} {} output differs: {difference}",
            output.format.as_transcription_format(),
            output_comparison_name(output.comparison)
        )
    })
}

pub(super) fn output_comparison_name(comparison: OutputComparisonMode) -> &'static str {
    match comparison {
        OutputComparisonMode::Exact => "exact",
        OutputComparisonMode::JsonSemantic => "jsonSemantic",
        OutputComparisonMode::SubtitleSemantic => "subtitleSemantic",
    }
}

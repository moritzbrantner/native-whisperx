//! Benchmark run JSON and diagnostic parsing helpers.

use super::*;

pub(super) fn provider_output_config(output: &OutputConfig, provider: &str) -> OutputConfig {
    let mut output = output.clone();
    if let Some(output_dir) = &output.output_dir {
        output.output_dir = Some(output_dir.join(provider));
    }
    output
}

pub(super) fn bench_run_json(
    report: &native_whisperx::NativeWhisperxReport,
    elapsed: Duration,
    audio_duration: Option<f64>,
    native: bool,
) -> serde_json::Value {
    let elapsed_seconds = duration_seconds(elapsed);
    let phases = bench_phase_json(&report.diagnostics, elapsed_seconds);
    bench_run_json_from_phases(report, elapsed_seconds, audio_duration, native, phases)
}

pub(super) fn aggregate_multi_input_run_json(
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

pub(super) fn multi_input_bench_runs_json(
    reports: &[native_whisperx::NativeWhisperxReport],
    audio_duration_per_input: Option<f64>,
    native: bool,
) -> Vec<serde_json::Value> {
    reports
        .iter()
        .map(|report| {
            let elapsed_seconds =
                diagnostic_f64(&report.diagnostics, "phaseNativeTotalSeconds").unwrap_or(0.0);
            bench_run_json_from_phases(
                report,
                elapsed_seconds,
                audio_duration_per_input,
                native,
                bench_phase_json(&report.diagnostics, elapsed_seconds),
            )
        })
        .collect()
}

pub(super) fn bench_run_json_from_phases(
    report: &native_whisperx::NativeWhisperxReport,
    elapsed_seconds: f64,
    audio_duration: Option<f64>,
    native: bool,
    phases: serde_json::Value,
) -> serde_json::Value {
    let diagnostics = &report.diagnostics;
    serde_json::json!({
        "elapsedSeconds": elapsed_seconds,
        "realtimeFactor": audio_duration.map(|duration| elapsed_seconds / duration),
        "phases": phases,
        "counters": bench_counter_json(diagnostics),
        "runtime": bench_runtime_json(diagnostics, native),
        "diagnostics": diagnostics,
    })
}

pub(super) fn aggregate_phase_json(
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

pub(super) fn aggregate_counter_json(
    reports: &[native_whisperx::NativeWhisperxReport],
) -> serde_json::Value {
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
                    .diagnostics
                    .iter()
                    .any(|item| item.starts_with("asrModelId="))
            })
            .count(),
        "asrCacheHit": reports.iter().all(|report| {
            diagnostic_value(&report.diagnostics, "asrModelSource")
                .as_deref()
                == Some("hugging-face-cache")
        }),
    })
}

pub(super) fn aggregate_runtime_json(
    reports: &[native_whisperx::NativeWhisperxReport],
    native: bool,
) -> serde_json::Value {
    if native {
        serde_json::json!({
            "provider": "native",
            "cudaActive": reports.iter().all(|report| {
                diagnostic_bool(&report.diagnostics, "cuda") == Some(true)
            }),
            "alignmentCudaActive": reports.iter().all(|report| {
                diagnostic_bool(&report.diagnostics, "alignmentCuda") == Some(true)
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

pub(super) fn bench_phase_json(
    diagnostics: &[String],
    total_elapsed_seconds: f64,
) -> serde_json::Value {
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

pub(super) fn bench_counter_json(diagnostics: &[String]) -> serde_json::Value {
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

pub(super) fn bench_runtime_json(diagnostics: &[String], native: bool) -> serde_json::Value {
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

pub(super) fn bench_asr_batch_diagnostics_json(diagnostics: &[String]) -> serde_json::Value {
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

pub(super) fn inferred_audio_duration_seconds(
    report: &native_whisperx::NativeWhisperxReport,
) -> Option<f64> {
    let transcript = serde_json::to_value(&report.transcript).ok()?;
    let segment_max = transcript
        .get("segments")
        .and_then(|segments| segments.as_array())
        .into_iter()
        .flatten()
        .filter_map(|segment| segment.get("end").and_then(|end| end.as_f64()))
        .fold(None, max_option_f64);
    let vad_max = report
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

pub(super) fn aggregate_audio_duration_seconds(
    reports: &[native_whisperx::NativeWhisperxReport],
) -> Option<f64> {
    let durations = reports
        .iter()
        .map(inferred_audio_duration_seconds)
        .collect::<Option<Vec<_>>>()?;
    Some(durations.into_iter().sum())
}

pub(super) fn reports_diagnostics_json(
    reports: &[native_whisperx::NativeWhisperxReport],
) -> Vec<Vec<String>> {
    reports
        .iter()
        .map(|report| report.diagnostics.clone())
        .collect()
}

pub(super) fn missing_required_diagnostics_by_input(
    required: &[String],
    reports: &[native_whisperx::NativeWhisperxReport],
) -> Vec<serde_json::Value> {
    reports
        .iter()
        .enumerate()
        .filter_map(|(index, report)| {
            let missing = missing_required_diagnostics(required, &report.diagnostics);
            (!missing.is_empty()).then(|| {
                serde_json::json!({
                    "inputIndex": index,
                    "missing": missing,
                })
            })
        })
        .collect()
}

pub(super) fn flatten_missing_required_diagnostics(
    missing_by_input: &[serde_json::Value],
) -> Vec<String> {
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

pub(super) fn max_option_f64(max: Option<f64>, value: f64) -> Option<f64> {
    Some(max.map_or(value, |max| max.max(value)))
}

pub(super) fn diagnostic_bool(diagnostics: &[String], key: &str) -> Option<bool> {
    diagnostic_value(diagnostics, key).and_then(|value| match value.as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    })
}

pub(super) fn diagnostic_value(diagnostics: &[String], key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    diagnostics
        .iter()
        .find_map(|diagnostic| diagnostic.strip_prefix(&prefix).map(ToOwned::to_owned))
}

pub(super) fn diagnostic_values(diagnostics: &[String], key: &str) -> Vec<String> {
    let prefix = format!("{key}=");
    diagnostics
        .iter()
        .filter_map(|diagnostic| diagnostic.strip_prefix(&prefix).map(ToOwned::to_owned))
        .collect()
}

pub(super) fn diagnostic_f64(diagnostics: &[String], key: &str) -> Option<f64> {
    diagnostic_value(diagnostics, key).and_then(|value| value.parse::<f64>().ok())
}

pub(super) fn diagnostic_usize(diagnostics: &[String], key: &str) -> Option<usize> {
    diagnostic_value(diagnostics, key).and_then(|value| value.parse::<usize>().ok())
}

pub(super) fn diagnostic_usize_list(diagnostics: &[String], key: &str) -> Option<Vec<usize>> {
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

pub(super) fn sum_diagnostic_f64(
    reports: &[native_whisperx::NativeWhisperxReport],
    key: &str,
) -> Option<f64> {
    let values = reports
        .iter()
        .map(|report| diagnostic_f64(&report.diagnostics, key))
        .collect::<Option<Vec<_>>>()?;
    Some(values.into_iter().sum())
}

pub(super) fn sum_diagnostic_usize(
    reports: &[native_whisperx::NativeWhisperxReport],
    key: &str,
) -> Option<usize> {
    let values = reports
        .iter()
        .map(|report| diagnostic_usize(&report.diagnostics, key))
        .collect::<Option<Vec<_>>>()?;
    Some(values.into_iter().sum())
}

pub(super) fn unique_diagnostic_values(
    reports: &[native_whisperx::NativeWhisperxReport],
    key: &str,
) -> Vec<String> {
    let mut values = reports
        .iter()
        .filter_map(|report| diagnostic_value(&report.diagnostics, key))
        .collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

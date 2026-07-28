#!/usr/bin/env python3
"""Run and sanitize opt-in native Q8 CPU ASR evidence."""

import argparse
import datetime
import json
import os
import subprocess
import tempfile
import time
import wave
from pathlib import Path


SUMMARY_MEASUREMENT_FIELDS = (
    "wallSeconds",
    "realtimeFactor",
    "modelLoadSeconds",
    "encoderSeconds",
    "decoderSeconds",
    "asrSeconds",
    "generatedTokenCount",
    "timestampFallback",
    "outputJsonValid",
)
KNOWN_TIMESTAMP_FALLBACKS = {
    "missingTimestampMetadata",
    "unstableTimestampSegments",
}


def sanitize_measurement(measurement):
    sanitized = {
        field: measurement[field]
        for field in SUMMARY_MEASUREMENT_FIELDS
        if field != "timestampFallback"
    }
    fallback = measurement["timestampFallback"]
    reasons = fallback["reasons"]
    unknown_reasons = set(reasons) - KNOWN_TIMESTAMP_FALLBACKS
    if unknown_reasons:
        raise RuntimeError("raw evidence contains an unknown timestamp fallback")
    sanitized["timestampFallback"] = {
        "used": fallback["used"],
        "reasons": reasons,
    }
    return sanitized


def sanitize_report(raw):
    """Return the commit-eligible whitelist-only view of a raw evidence report."""
    configuration = raw["configuration"]
    clips = [
        {
            "id": clip["id"],
            "audioDurationSeconds": clip["audioDurationSeconds"],
            "measured": [sanitize_measurement(run) for run in clip["measured"]],
        }
        for clip in raw["clips"]
    ]
    one_second_runs = next(
        clip["measured"] for clip in clips if clip["id"] == "shrek-retold-1s"
    )
    threshold_applicable = "i5-6300u" in raw["cpu"]["model"].lower()
    threshold_passed = (
        all(run["wallSeconds"] < 45.0 for run in one_second_runs)
        if threshold_applicable
        else None
    )
    return {
        "schemaVersion": raw["schemaVersion"],
        "evidenceClass": raw["evidenceClass"],
        "generatedAt": raw["generatedAt"],
        "cpu": {"model": raw["cpu"]["model"]},
        "configuration": {
            field: configuration[field]
            for field in (
                "provider",
                "device",
                "computeType",
                "alignment",
                "warmupRunsPerClip",
                "measuredRunsPerClip",
            )
        },
        "oneSecondThreshold": {
            "applicable": threshold_applicable,
            "limitSeconds": 45.0,
            "passed": threshold_passed,
        },
        "clips": clips,
    }


def validate_acceptance(summary):
    threshold = summary["oneSecondThreshold"]
    if threshold["applicable"] and threshold["passed"] is not True:
        raise RuntimeError(
            "warmed one-second Q8 runs exceeded the 45-second i5-6300U limit"
        )


def diagnostic_values(report, key):
    prefix = f"{key}="
    diagnostics = report.get("response", {}).get("diagnostics", [])
    return [
        item[len(prefix) :]
        for item in diagnostics
        if isinstance(item, str) and item.startswith(prefix)
    ]


def required_diagnostic(report, key, conversion):
    values = diagnostic_values(report, key)
    if not values:
        raise RuntimeError(f"native Q8 report is missing required diagnostic `{key}`")
    try:
        return conversion(values[-1])
    except (TypeError, ValueError) as error:
        raise RuntimeError(
            f"native Q8 report has an invalid `{key}` diagnostic"
        ) from error


def wav_duration_seconds(path):
    try:
        with wave.open(str(path), "rb") as wav:
            return wav.getnframes() / wav.getframerate()
    except (OSError, EOFError, wave.Error) as error:
        raise RuntimeError("evidence input must be a readable WAV file") from error


def cpu_model():
    try:
        for line in Path("/proc/cpuinfo").read_text().splitlines():
            if line.lower().startswith("model name"):
                return line.split(":", 1)[1].strip()
    except OSError:
        pass
    return os.uname().machine


def validate_output_json(report):
    response = report.get("response", {})
    if response.get("accepted") is not True:
        raise RuntimeError("native Q8 report did not accept the transcription")
    transcript = response.get("transcript")
    if not isinstance(transcript, dict) or not isinstance(
        transcript.get("segments"), list
    ):
        raise RuntimeError("native Q8 report has no valid transcript segments array")
    output_files = report.get("outputFiles")
    if not isinstance(output_files, list) or not output_files:
        raise RuntimeError("native Q8 report has no generated JSON output")
    json_outputs = [
        Path(output) for output in output_files if str(output).lower().endswith(".json")
    ]
    if not json_outputs:
        raise RuntimeError("native Q8 report has no generated JSON output")
    try:
        for output in json_outputs:
            json.loads(output.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError("native Q8 generated output is not valid JSON") from error


def run_measurement(binary, bundle, clip, audio_duration, label, iteration):
    with tempfile.TemporaryDirectory(prefix="native-whisperx-q8-evidence-") as temp:
        run_root = Path(temp)
        report_path = run_root / "report.json"
        output_dir = run_root / "output"
        command = [
            str(binary),
            "transcribe",
            str(clip),
            "--provider",
            "native",
            "--device",
            "cpu",
            "--compute-type",
            "int8",
            "--whisper-bundle",
            str(bundle),
            "--language",
            "en",
            "--no-align",
            "--format",
            "json",
            "--report",
            str(report_path),
            "--output-dir",
            str(output_dir),
        ]
        started = time.monotonic()
        process = subprocess.run(command, check=False, capture_output=True, text=True)
        elapsed = time.monotonic() - started
        if process.returncode != 0:
            raise RuntimeError(
                f"native Q8 transcription failed for {label} iteration {iteration}"
            )
        try:
            report = json.loads(report_path.read_text())
        except (OSError, json.JSONDecodeError) as error:
            raise RuntimeError(
                f"native Q8 report is invalid for {label} iteration {iteration}"
            ) from error
        validate_output_json(report)
        diagnostics = report.get("response", {}).get("diagnostics", [])
        for expected in (
            "provider=candle-whisper",
            "requestedComputeType=int8",
            "resolvedComputeType=int8",
            "modelFormat=gguf-q8_0",
        ):
            if expected not in diagnostics:
                raise RuntimeError(
                    f"native Q8 report is missing required diagnostic `{expected}`"
                )
        fallback_reasons = diagnostic_values(report, "timingFallback")
        return {
            "wallSeconds": elapsed,
            "realtimeFactor": elapsed / audio_duration,
            "modelLoadSeconds": required_diagnostic(
                report, "phaseAsrModelLoadSeconds", float
            ),
            "encoderSeconds": required_diagnostic(
                report, "phaseTiming.encoderSeconds", float
            ),
            "decoderSeconds": required_diagnostic(
                report, "phaseTiming.decoderSeconds", float
            ),
            "asrSeconds": required_diagnostic(
                report, "phaseTiming.asrSeconds", float
            ),
            "generatedTokenCount": required_diagnostic(
                report, "generatedTokenCount", int
            ),
            "timestampFallback": {
                "used": bool(fallback_reasons),
                "reasons": fallback_reasons,
            },
            "outputJsonValid": True,
            "rawReport": report,
        }


def validate_resource(path, kind):
    if kind == "directory" and not path.is_dir():
        raise RuntimeError("caller-owned Q8 bundle is unavailable")
    if kind == "file" and not path.is_file():
        raise RuntimeError("caller-owned Shrek-derived WAV is unavailable")


def run_evidence(args):
    binary = Path(args.binary)
    bundle = Path(args.bundle)
    one_second = Path(args.one_second_wav)
    fifteen_seconds = Path(args.fifteen_second_wav)
    validate_resource(binary, "file")
    validate_resource(bundle, "directory")
    validate_resource(one_second, "file")
    validate_resource(fifteen_seconds, "file")
    clip_specs = [
        ("shrek-retold-1s", one_second, 1.0, 0.25),
        ("shrek-retold-15s", fifteen_seconds, 15.0, 0.5),
    ]
    clips = []
    for label, clip, expected_duration, tolerance in clip_specs:
        duration = wav_duration_seconds(clip)
        if abs(duration - expected_duration) > tolerance:
            raise RuntimeError(f"{label} WAV duration is outside its required range")
        warmup = run_measurement(
            binary, bundle, clip, duration, label, "warmup"
        )
        measured = [
            run_measurement(binary, bundle, clip, duration, label, index)
            for index in range(1, 4)
        ]
        clips.append(
            {
                "id": label,
                "input": str(clip),
                "audioDurationSeconds": duration,
                "warmup": warmup,
                "measured": measured,
            }
        )
    raw = {
        "schemaVersion": 1,
        "evidenceClass": "q8-cpu-asr-only",
        "generatedAt": datetime.datetime.now(datetime.timezone.utc)
        .isoformat()
        .replace("+00:00", "Z"),
        "cpu": {"model": cpu_model()},
        "configuration": {
            "provider": "native",
            "device": "cpu",
            "computeType": "int8",
            "alignment": False,
            "warmupRunsPerClip": 1,
            "measuredRunsPerClip": 3,
            "bundle": str(bundle),
        },
        "clips": clips,
    }
    summary = sanitize_report(raw)
    write_json(Path(args.raw_report), raw)
    write_json(Path(args.summary), summary)
    validate_acceptance(summary)


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def parse_args():
    parser = argparse.ArgumentParser(
        description="Run or sanitize native Q8 CPU ASR evidence"
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    run = subparsers.add_parser("run")
    run.add_argument("--binary", required=True)
    run.add_argument("--bundle", required=True)
    run.add_argument("--one-second-wav", required=True)
    run.add_argument("--fifteen-second-wav", required=True)
    run.add_argument("--raw-report", required=True)
    run.add_argument("--summary", required=True)
    sanitize = subparsers.add_parser("sanitize")
    sanitize.add_argument("raw_report")
    sanitize.add_argument("summary")
    return parser.parse_args()


def main():
    args = parse_args()
    try:
        if args.command == "run":
            run_evidence(args)
        else:
            raw = json.loads(Path(args.raw_report).read_text())
            write_json(Path(args.summary), sanitize_report(raw))
    except (OSError, json.JSONDecodeError, RuntimeError) as error:
        raise SystemExit(f"q8 CPU evidence failed: {error}") from error


if __name__ == "__main__":
    main()

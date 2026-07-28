#!/usr/bin/env python3
"""Run and sanitize opt-in native Q8 CPU ASR evidence."""

import argparse
import datetime
import json
import math
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
TIMING_FIELDS = SUMMARY_MEASUREMENT_FIELDS[:6]
KNOWN_TIMESTAMP_FALLBACKS = {
    "missingTimestampMetadata",
    "unstableTimestampSegments",
}
EXPECTED_CONFIGURATION = {
    "provider": "native",
    "device": "cpu",
    "computeType": "int8",
    "alignment": False,
    "warmupRunsPerClip": 1,
    "measuredRunsPerClip": 3,
}
EXPECTED_CLIPS = {
    "shrek-retold-1s": (0.75, 1.25),
    "shrek-retold-15s": (14.5, 15.5),
}
SCHEMA_VERSION = 1
EVIDENCE_CLASS = "q8-cpu-asr-only"
ONE_SECOND_LIMIT = 45.0


def require_object(value, path):
    if not isinstance(value, dict):
        raise RuntimeError(f"raw evidence `{path}` must be an object")
    return value


def require_list(value, path):
    if not isinstance(value, list):
        raise RuntimeError(f"raw evidence `{path}` must be an array")
    return value


def required(value, key, path):
    if key not in value:
        raise RuntimeError(f"raw evidence is missing required `{path}.{key}`")
    return value[key]


def require_exact(value, expected, path):
    if type(value) is not type(expected) or value != expected:
        raise RuntimeError(f"raw evidence `{path}` must be {expected!r}")
    return value


def require_nonempty_string(value, path):
    if (
        not isinstance(value, str)
        or not value
        or value != value.strip()
        or len(value) > 512
        or not value.isprintable()
    ):
        raise RuntimeError(f"raw evidence `{path}` must be a non-empty string")
    return value


def require_generated_at(value):
    require_nonempty_string(value, "generatedAt")
    if not value.endswith("Z"):
        raise RuntimeError("raw evidence `generatedAt` must be a UTC timestamp")
    try:
        parsed = datetime.datetime.fromisoformat(value[:-1] + "+00:00")
    except ValueError as error:
        raise RuntimeError(
            "raw evidence `generatedAt` must be a UTC timestamp"
        ) from error
    if parsed.utcoffset() != datetime.timedelta(0):
        raise RuntimeError("raw evidence `generatedAt` must be a UTC timestamp")
    return value


def require_nonnegative_number(value, path):
    if (
        type(value) not in (int, float)
        or value < 0
        or (type(value) is float and not math.isfinite(value))
    ):
        raise RuntimeError(
            f"raw evidence `{path}` must be a finite non-negative number"
        )
    return value


def require_token_count(value, path):
    if type(value) is not int or value < 0:
        raise RuntimeError(
            f"raw evidence `{path}` must be a non-negative integer"
        )
    return value


def sanitize_fallback(value, path):
    fallback = require_object(value, path)
    used = required(fallback, "used", path)
    if type(used) is not bool:
        raise RuntimeError(f"raw evidence `{path}.used` must be a boolean")
    reasons = require_list(required(fallback, "reasons", path), f"{path}.reasons")
    if any(type(reason) is not str for reason in reasons):
        raise RuntimeError(f"raw evidence `{path}.reasons` must contain strings")
    if len(reasons) != len(set(reasons)):
        raise RuntimeError(f"raw evidence `{path}.reasons` contains duplicates")
    unknown_reasons = set(reasons) - KNOWN_TIMESTAMP_FALLBACKS
    if unknown_reasons:
        raise RuntimeError("raw evidence contains an unknown timestamp fallback")
    if used is not bool(reasons):
        raise RuntimeError(
            f"raw evidence `{path}.used` must match whether reasons are present"
        )
    return {"used": used, "reasons": list(reasons)}


def sanitize_measurement(measurement, path="measurement"):
    measurement = require_object(measurement, path)
    sanitized = {}
    for field in TIMING_FIELDS:
        sanitized[field] = require_nonnegative_number(
            required(measurement, field, path), f"{path}.{field}"
        )
    sanitized["generatedTokenCount"] = require_token_count(
        required(measurement, "generatedTokenCount", path),
        f"{path}.generatedTokenCount",
    )
    sanitized["timestampFallback"] = sanitize_fallback(
        required(measurement, "timestampFallback", path),
        f"{path}.timestampFallback",
    )
    output_json_valid = required(measurement, "outputJsonValid", path)
    if output_json_valid is not True:
        raise RuntimeError(f"raw evidence `{path}.outputJsonValid` must be true")
    sanitized["outputJsonValid"] = output_json_valid
    return sanitized


def validated_configuration(raw):
    configuration = require_object(
        required(raw, "configuration", "raw evidence"), "configuration"
    )
    for field, expected in EXPECTED_CONFIGURATION.items():
        require_exact(
            required(configuration, field, "configuration"),
            expected,
            f"configuration.{field}",
        )
    return configuration


def validated_cpu(raw):
    cpu = require_object(required(raw, "cpu", "raw evidence"), "cpu")
    return {
        "model": require_nonempty_string(
            required(cpu, "model", "cpu"), "cpu.model"
        )
    }


def validated_clips(raw, require_warmup):
    clips = require_list(required(raw, "clips", "raw evidence"), "clips")
    if len(clips) != len(EXPECTED_CLIPS):
        raise RuntimeError("raw evidence must contain exactly two required clips")
    by_id = {}
    for index, value in enumerate(clips):
        path = f"clips[{index}]"
        clip = require_object(value, path)
        clip_id = required(clip, "id", path)
        if type(clip_id) is not str or clip_id not in EXPECTED_CLIPS:
            raise RuntimeError(f"raw evidence `{path}.id` is not a required clip")
        if clip_id in by_id:
            raise RuntimeError("raw evidence contains duplicate clip ids")
        minimum, maximum = EXPECTED_CLIPS[clip_id]
        duration = require_nonnegative_number(
            required(clip, "audioDurationSeconds", path),
            f"{path}.audioDurationSeconds",
        )
        if not minimum <= duration <= maximum:
            raise RuntimeError(
                f"raw evidence `{path}.audioDurationSeconds` is outside its required range"
            )
        if require_warmup:
            sanitize_measurement(
                required(clip, "warmup", path), f"{path}.warmup"
            )
        measured = require_list(required(clip, "measured", path), f"{path}.measured")
        if len(measured) != EXPECTED_CONFIGURATION["measuredRunsPerClip"]:
            raise RuntimeError(
                f"raw evidence `{path}.measured` must contain exactly three runs"
            )
        sanitized_measured = [
            sanitize_measurement(run, f"{path}.measured[{run_index}]")
            for run_index, run in enumerate(measured)
        ]
        by_id[clip_id] = {
            "id": clip_id,
            "audioDurationSeconds": duration,
            "measured": sanitized_measured,
        }
    if set(by_id) != set(EXPECTED_CLIPS):
        raise RuntimeError("raw evidence must contain both required clip ids")
    return [by_id[clip_id] for clip_id in EXPECTED_CLIPS]


def validate_raw_report(raw):
    raw = require_object(raw, "raw evidence")
    require_exact(
        required(raw, "schemaVersion", "raw evidence"),
        SCHEMA_VERSION,
        "schemaVersion",
    )
    require_exact(
        required(raw, "evidenceClass", "raw evidence"),
        EVIDENCE_CLASS,
        "evidenceClass",
    )
    require_generated_at(required(raw, "generatedAt", "raw evidence"))
    validated_cpu(raw)
    validated_configuration(raw)
    validated_clips(raw, require_warmup=True)


def sanitize_report(raw):
    """Return the commit-eligible whitelist-only view of a raw evidence report."""
    validate_raw_report(raw)
    configuration = validated_configuration(raw)
    cpu = validated_cpu(raw)
    clips = validated_clips(raw, require_warmup=True)
    one_second_runs = clips[0]["measured"]
    threshold_applicable = "i5-6300u" in raw["cpu"]["model"].lower()
    threshold_passed = (
        all(run["wallSeconds"] < ONE_SECOND_LIMIT for run in one_second_runs)
        if threshold_applicable
        else None
    )
    summary = {
        "schemaVersion": SCHEMA_VERSION,
        "evidenceClass": EVIDENCE_CLASS,
        "generatedAt": require_generated_at(raw["generatedAt"]),
        "cpu": cpu,
        "configuration": {
            field: configuration[field] for field in EXPECTED_CONFIGURATION
        },
        "oneSecondThreshold": {
            "applicable": threshold_applicable,
            "limitSeconds": ONE_SECOND_LIMIT,
            "passed": threshold_passed,
        },
        "clips": clips,
    }
    validate_summary(summary)
    return summary


def validate_summary(summary):
    summary = require_object(summary, "summary")
    require_exact(
        required(summary, "schemaVersion", "summary"),
        SCHEMA_VERSION,
        "schemaVersion",
    )
    require_exact(
        required(summary, "evidenceClass", "summary"),
        EVIDENCE_CLASS,
        "evidenceClass",
    )
    require_generated_at(required(summary, "generatedAt", "summary"))
    cpu = validated_cpu(summary)
    validated_configuration(summary)
    clips = validated_clips(summary, require_warmup=False)
    threshold = require_object(
        required(summary, "oneSecondThreshold", "summary"),
        "oneSecondThreshold",
    )
    applicable = required(threshold, "applicable", "oneSecondThreshold")
    if type(applicable) is not bool:
        raise RuntimeError(
            "raw evidence `oneSecondThreshold.applicable` must be a boolean"
        )
    expected_applicable = "i5-6300u" in cpu["model"].lower()
    if applicable is not expected_applicable:
        raise RuntimeError(
            "raw evidence threshold applicability does not match CPU identity"
        )
    require_exact(
        required(threshold, "limitSeconds", "oneSecondThreshold"),
        ONE_SECOND_LIMIT,
        "oneSecondThreshold.limitSeconds",
    )
    passed = required(threshold, "passed", "oneSecondThreshold")
    expected_passed = (
        all(run["wallSeconds"] < ONE_SECOND_LIMIT for run in clips[0]["measured"])
        if applicable
        else None
    )
    if type(passed) is not type(expected_passed) or passed != expected_passed:
        raise RuntimeError(
            "raw evidence threshold result does not match three measured runs"
        )


def validate_acceptance(summary):
    validate_summary(summary)
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


def validate_output_json(report, output_dir):
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
    try:
        output_root = output_dir.resolve(strict=True)
    except OSError as error:
        raise RuntimeError(
            "native Q8 output directory is unavailable after transcription"
        ) from error
    if not output_root.is_dir():
        raise RuntimeError(
            "native Q8 output directory is unavailable after transcription"
        )
    json_outputs = []
    for output in output_files:
        if (
            not isinstance(output, dict)
            or not isinstance(output.get("format"), str)
            or not output["format"]
            or not isinstance(output.get("path"), str)
            or not output["path"]
        ):
            raise RuntimeError(
                "native Q8 report has an invalid outputFiles entry"
            )
        try:
            resolved_path = Path(output["path"]).resolve(strict=True)
        except OSError as error:
            raise RuntimeError(
                "native Q8 report references an unavailable output file"
            ) from error
        if (
            not resolved_path.is_relative_to(output_root)
            or not resolved_path.is_file()
        ):
            raise RuntimeError(
                "native Q8 report references output outside the fresh output directory"
            )
        if output["format"] == "json":
            json_outputs.append(resolved_path)
    if len(json_outputs) != 1:
        raise RuntimeError(
            "native Q8 report must contain exactly one generated JSON output"
        )
    try:
        for output in json_outputs:
            generated = json.loads(output.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError("native Q8 generated output is not valid JSON") from error
    if (
        not isinstance(generated, dict)
        or not isinstance(generated.get("text"), str)
        or not isinstance(generated.get("segments"), list)
        or not isinstance(generated.get("word_segments"), list)
    ):
        raise RuntimeError(
            "native Q8 generated output does not match the WhisperX JSON contract"
        )


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
        validate_output_json(report, output_dir)
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
            summary = sanitize_report(raw)
            validate_acceptance(summary)
            write_json(Path(args.summary), summary)
    except (OSError, json.JSONDecodeError, RuntimeError) as error:
        raise SystemExit(f"q8 CPU evidence failed: {error}") from error


if __name__ == "__main__":
    main()

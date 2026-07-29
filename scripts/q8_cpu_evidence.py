#!/usr/bin/env python3
"""Run and sanitize matched native Q8-versus-FP32 CPU ASR evidence."""

import argparse
import datetime
import hashlib
import json
import math
import os
import statistics
import subprocess
import tempfile
import time
import wave
from pathlib import Path


SCHEMA_VERSION = 2
EVIDENCE_CLASS = "q8-fp32-cpu-asr-comparison"
SIDECAR_FILES = (
    "config.json",
    "generation_config.json",
    "tokenizer.json",
    "preprocessor_config.json",
)
MODES = {
    "q8": {
        "computeType": "int8",
        "diagnosticComputeType": "int8",
        "modelFile": "model.q8_0.gguf",
        "modelFormat": "gguf-q8_0",
    },
    "fp32": {
        "computeType": "float32",
        "diagnosticComputeType": "fp32",
        "modelFile": "model.safetensors",
        "modelFormat": "safetensors",
    },
}
EXPECTED_CONFIGURATION = {
    "provider": "native",
    "device": "cpu",
    "alignment": False,
    "warmupRunsPerModePerClip": 1,
    "measuredRunsPerModePerClip": 3,
    "alternatingOrder": True,
}
EXPECTED_CLIPS = {
    "shrek-retold-1s": {"duration": (0.75, 1.25), "maximumRatio": 1.10},
    "shrek-retold-15s": {"duration": (14.5, 15.5), "maximumRatio": 0.90},
}
TIMING_FIELDS = (
    "wallSeconds",
    "realtimeFactor",
    "modelLoadSeconds",
    "encoderSeconds",
    "decoderSeconds",
    "asrSeconds",
)
KNOWN_TIMESTAMP_FALLBACKS = {
    "missingTimestampMetadata",
    "unstableTimestampSegments",
}


def require_object(value, path):
    if not isinstance(value, dict):
        raise RuntimeError(f"evidence `{path}` must be an object")
    return value


def require_list(value, path):
    if not isinstance(value, list):
        raise RuntimeError(f"evidence `{path}` must be an array")
    return value


def required(value, key, path):
    if key not in value:
        raise RuntimeError(f"evidence is missing required `{path}.{key}`")
    return value[key]


def require_exact(value, expected, path):
    if type(value) is not type(expected) or value != expected:
        raise RuntimeError(f"evidence `{path}` must be {expected!r}")
    return value


def require_nonempty_string(value, path):
    if (
        not isinstance(value, str)
        or not value
        or value != value.strip()
        or len(value) > 512
        or not value.isprintable()
    ):
        raise RuntimeError(f"evidence `{path}` must be a non-empty string")
    return value


def require_transcript_string(value, path):
    if not isinstance(value, str):
        raise RuntimeError(f"evidence `{path}` must be a string")
    return value


def require_generated_at(value):
    require_nonempty_string(value, "generatedAt")
    if not value.endswith("Z"):
        raise RuntimeError("evidence `generatedAt` must be a UTC timestamp")
    try:
        parsed = datetime.datetime.fromisoformat(value[:-1] + "+00:00")
    except ValueError as error:
        raise RuntimeError("evidence `generatedAt` must be a UTC timestamp") from error
    if parsed.utcoffset() != datetime.timedelta(0):
        raise RuntimeError("evidence `generatedAt` must be a UTC timestamp")
    return value


def require_nonnegative_number(value, path):
    if (
        type(value) not in (int, float)
        or value < 0
        or (type(value) is float and not math.isfinite(value))
    ):
        raise RuntimeError(
            f"evidence `{path}` must be a finite non-negative number"
        )
    return value


def require_positive_number(value, path):
    value = require_nonnegative_number(value, path)
    if value == 0:
        raise RuntimeError(f"evidence `{path}` must be greater than zero")
    return value


def require_sha256(value, path):
    if (
        not isinstance(value, str)
        or len(value) != 64
        or any(character not in "0123456789abcdef" for character in value)
    ):
        raise RuntimeError(f"evidence `{path}` must be a lowercase SHA-256 hash")
    return value


def sanitize_fallback(value, path):
    fallback = require_object(value, path)
    used = required(fallback, "used", path)
    if type(used) is not bool:
        raise RuntimeError(f"evidence `{path}.used` must be a boolean")
    reasons = require_list(required(fallback, "reasons", path), f"{path}.reasons")
    if any(type(reason) is not str for reason in reasons):
        raise RuntimeError(f"evidence `{path}.reasons` must contain strings")
    if len(reasons) != len(set(reasons)):
        raise RuntimeError(f"evidence `{path}.reasons` contains duplicates")
    if set(reasons) - KNOWN_TIMESTAMP_FALLBACKS:
        raise RuntimeError("evidence contains an unknown timestamp fallback")
    if used is not bool(reasons):
        raise RuntimeError(
            f"evidence `{path}.used` must match whether reasons are present"
        )
    return {"used": used, "reasons": list(reasons)}


def sanitize_measurement(measurement, path):
    measurement = require_object(measurement, path)
    sanitized = {
        field: require_nonnegative_number(
            required(measurement, field, path), f"{path}.{field}"
        )
        for field in TIMING_FIELDS
    }
    tokens = required(measurement, "generatedTokenCount", path)
    if type(tokens) is not int or tokens < 0:
        raise RuntimeError(
            f"evidence `{path}.generatedTokenCount` must be a non-negative integer"
        )
    sanitized["generatedTokenCount"] = tokens
    sanitized["timestampFallback"] = sanitize_fallback(
        required(measurement, "timestampFallback", path),
        f"{path}.timestampFallback",
    )
    require_exact(
        required(measurement, "outputJsonValid", path),
        True,
        f"{path}.outputJsonValid",
    )
    sanitized["outputJsonValid"] = True
    return sanitized


def validated_cpu(report):
    cpu = require_object(required(report, "cpu", "evidence"), "cpu")
    return {
        "model": require_nonempty_string(required(cpu, "model", "cpu"), "cpu.model")
    }


def validated_configuration(report):
    configuration = require_object(
        required(report, "configuration", "evidence"), "configuration"
    )
    for field, expected in EXPECTED_CONFIGURATION.items():
        require_exact(
            required(configuration, field, "configuration"),
            expected,
            f"configuration.{field}",
        )
    mode_configuration = require_object(
        required(configuration, "modes", "configuration"), "configuration.modes"
    )
    if set(mode_configuration) != set(MODES):
        raise RuntimeError("evidence `configuration.modes` must contain q8 and fp32")
    sanitized_modes = {}
    for mode, expected in MODES.items():
        value = require_object(mode_configuration[mode], f"configuration.modes.{mode}")
        sanitized_modes[mode] = {
            "computeType": require_exact(
                required(value, "computeType", f"configuration.modes.{mode}"),
                expected["computeType"],
                f"configuration.modes.{mode}.computeType",
            ),
            "modelFile": require_exact(
                required(value, "modelFile", f"configuration.modes.{mode}"),
                expected["modelFile"],
                f"configuration.modes.{mode}.modelFile",
            ),
        }
    return {
        **{field: configuration[field] for field in EXPECTED_CONFIGURATION},
        "modes": sanitized_modes,
    }


def validated_bundle_hashes(report):
    bundles = require_object(
        required(report, "bundleHashes", "evidence"), "bundleHashes"
    )
    if set(bundles) != set(MODES):
        raise RuntimeError("evidence `bundleHashes` must contain q8 and fp32")
    sanitized = {}
    for mode, expected in MODES.items():
        bundle = require_object(bundles[mode], f"bundleHashes.{mode}")
        model = require_object(
            required(bundle, "model", f"bundleHashes.{mode}"),
            f"bundleHashes.{mode}.model",
        )
        sidecars = require_object(
            required(bundle, "sidecars", f"bundleHashes.{mode}"),
            f"bundleHashes.{mode}.sidecars",
        )
        if set(sidecars) != set(SIDECAR_FILES):
            raise RuntimeError(
                f"evidence `bundleHashes.{mode}.sidecars` must contain the four required sidecars"
            )
        sanitized[mode] = {
            "model": {
                "file": require_exact(
                    required(model, "file", f"bundleHashes.{mode}.model"),
                    expected["modelFile"],
                    f"bundleHashes.{mode}.model.file",
                ),
                "sha256": require_sha256(
                    required(model, "sha256", f"bundleHashes.{mode}.model"),
                    f"bundleHashes.{mode}.model.sha256",
                ),
            },
            "sidecars": {
                filename: require_sha256(
                    sidecars[filename],
                    f"bundleHashes.{mode}.sidecars.{filename}",
                )
                for filename in SIDECAR_FILES
            },
        }
    if sanitized["q8"]["sidecars"] != sanitized["fp32"]["sidecars"]:
        raise RuntimeError("Q8 and FP32 bundles must have byte-identical sidecars")
    return sanitized


def expected_execution_order():
    return [
        {"phase": "warmup", "iteration": 0, "modes": ["q8", "fp32"]},
        {"phase": "measured", "iteration": 1, "modes": ["fp32", "q8"]},
        {"phase": "measured", "iteration": 2, "modes": ["q8", "fp32"]},
        {"phase": "measured", "iteration": 3, "modes": ["fp32", "q8"]},
    ]


def validate_execution_order(value, path):
    order = require_list(value, path)
    require_exact(order, expected_execution_order(), path)
    return [
        {
            "phase": entry["phase"],
            "iteration": entry["iteration"],
            "modes": list(entry["modes"]),
        }
        for entry in order
    ]


def sanitize_clip(value, index, require_transcripts):
    path = f"clips[{index}]"
    clip = require_object(value, path)
    clip_id = required(clip, "id", path)
    if type(clip_id) is not str or clip_id not in EXPECTED_CLIPS:
        raise RuntimeError(f"evidence `{path}.id` is not a required clip")
    duration = require_nonnegative_number(
        required(clip, "audioDurationSeconds", path),
        f"{path}.audioDurationSeconds",
    )
    minimum, maximum = EXPECTED_CLIPS[clip_id]["duration"]
    if not minimum <= duration <= maximum:
        raise RuntimeError(
            f"evidence `{path}.audioDurationSeconds` is outside its required range"
        )
    execution_order = validate_execution_order(
        required(clip, "executionOrder", path), f"{path}.executionOrder"
    )
    raw_modes = require_object(required(clip, "modes", path), f"{path}.modes")
    if set(raw_modes) != set(MODES):
        raise RuntimeError(f"evidence `{path}.modes` must contain q8 and fp32")
    modes = {}
    raw_transcripts = {}
    for mode in MODES:
        mode_path = f"{path}.modes.{mode}"
        raw_mode = require_object(raw_modes[mode], mode_path)
        warmup = required(raw_mode, "warmup", mode_path)
        measured = require_list(
            required(raw_mode, "measured", mode_path), f"{mode_path}.measured"
        )
        if len(measured) != 3:
            raise RuntimeError(
                f"evidence `{mode_path}.measured` must contain exactly three runs"
            )
        modes[mode] = {
            "warmup": sanitize_measurement(warmup, f"{mode_path}.warmup"),
            "measured": [
                sanitize_measurement(run, f"{mode_path}.measured[{run_index}]")
                for run_index, run in enumerate(measured)
            ],
        }
        for run_index, run in enumerate(
            [modes[mode]["warmup"], *modes[mode]["measured"]]
        ):
            expected_rtf = run["asrSeconds"] / duration
            if not math.isclose(
                run["realtimeFactor"],
                expected_rtf,
                rel_tol=1e-9,
                abs_tol=1e-12,
            ):
                raise RuntimeError(
                    f"evidence `{mode_path}` run {run_index} realtime factor "
                    "must use reported ASR seconds"
                )
        if require_transcripts:
            raw_transcripts[mode] = [
                require_transcript_string(
                    required(warmup, "transcriptText", f"{mode_path}.warmup"),
                    f"{mode_path}.warmup.transcriptText",
                ),
                *[
                    require_transcript_string(
                        required(run, "transcriptText", f"{mode_path}.measured[{i}]"),
                        f"{mode_path}.measured[{i}].transcriptText",
                    )
                    for i, run in enumerate(measured)
                ],
            ]
    sanitized = {
        "id": clip_id,
        "audioDurationSeconds": duration,
        "executionOrder": execution_order,
        "modes": modes,
    }
    if require_transcripts:
        equality = [
            raw_transcripts["q8"][index] == raw_transcripts["fp32"][index]
            for index in range(4)
        ]
        sanitized["transcriptEquality"] = {
            "warmup": equality[0],
            "measured": equality[1:],
            "all": all(equality),
        }
    else:
        equality = require_object(
            required(clip, "transcriptEquality", path),
            f"{path}.transcriptEquality",
        )
        warmup_equal = required(equality, "warmup", f"{path}.transcriptEquality")
        measured_equal = require_list(
            required(equality, "measured", f"{path}.transcriptEquality"),
            f"{path}.transcriptEquality.measured",
        )
        all_equal = required(equality, "all", f"{path}.transcriptEquality")
        if (
            type(warmup_equal) is not bool
            or len(measured_equal) != 3
            or any(type(item) is not bool for item in measured_equal)
            or type(all_equal) is not bool
            or all_equal is not all([warmup_equal, *measured_equal])
        ):
            raise RuntimeError(f"evidence `{path}.transcriptEquality` is invalid")
        sanitized["transcriptEquality"] = {
            "warmup": warmup_equal,
            "measured": list(measured_equal),
            "all": all_equal,
        }
    return sanitized


def validated_clips(report, require_transcripts):
    clips = require_list(required(report, "clips", "evidence"), "clips")
    if len(clips) != len(EXPECTED_CLIPS):
        raise RuntimeError("evidence must contain exactly two required clips")
    by_id = {}
    for index, value in enumerate(clips):
        clip = sanitize_clip(value, index, require_transcripts)
        if clip["id"] in by_id:
            raise RuntimeError("evidence contains duplicate clip ids")
        by_id[clip["id"]] = clip
    if set(by_id) != set(EXPECTED_CLIPS):
        raise RuntimeError("evidence must contain both required clip ids")
    return [by_id[clip_id] for clip_id in EXPECTED_CLIPS]


def comparative_gate(clips):
    cases = []
    for clip in clips:
        q8_median = statistics.median(
            run["asrSeconds"] for run in clip["modes"]["q8"]["measured"]
        )
        fp32_median = statistics.median(
            run["asrSeconds"] for run in clip["modes"]["fp32"]["measured"]
        )
        require_positive_number(fp32_median, f"{clip['id']} FP32 median")
        ratio = q8_median / fp32_median
        maximum = EXPECTED_CLIPS[clip["id"]]["maximumRatio"]
        cases.append(
            {
                "clipId": clip["id"],
                "q8MedianAsrSeconds": q8_median,
                "fp32MedianAsrSeconds": fp32_median,
                "q8ToFp32Ratio": ratio,
                "maximumRatio": maximum,
                "passed": ratio <= maximum,
            }
        )
    return {
        "metric": "medianReportedAsrSeconds",
        "cases": cases,
        "passed": all(case["passed"] for case in cases),
    }


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
    validated_bundle_hashes(raw)
    validated_clips(raw, require_transcripts=True)


def sanitize_report(raw):
    """Return the commit-eligible whitelist-only view of a raw evidence report."""
    validate_raw_report(raw)
    clips = validated_clips(raw, require_transcripts=True)
    summary = {
        "schemaVersion": SCHEMA_VERSION,
        "evidenceClass": EVIDENCE_CLASS,
        "generatedAt": require_generated_at(raw["generatedAt"]),
        "cpu": validated_cpu(raw),
        "bundleHashes": validated_bundle_hashes(raw),
        "configuration": validated_configuration(raw),
        "clips": clips,
        "comparativeGate": comparative_gate(clips),
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
    validated_cpu(summary)
    validated_configuration(summary)
    validated_bundle_hashes(summary)
    clips = validated_clips(summary, require_transcripts=False)
    require_exact(
        required(summary, "comparativeGate", "summary"),
        comparative_gate(clips),
        "comparativeGate",
    )


def validate_acceptance(summary):
    validate_summary(summary)
    if summary["comparativeGate"]["passed"] is not True:
        raise RuntimeError(
            "Q8 median reported ASR seconds failed the matched FP32 comparative gate"
        )


def diagnostic_values(report, key):
    prefix = f"{key}="
    diagnostics = report.get("response", {}).get("diagnostics", [])
    return [
        item[len(prefix) :]
        for item in diagnostics
        if isinstance(item, str) and item.startswith(prefix)
    ]


def required_diagnostic_value(report, key):
    values = diagnostic_values(report, key)
    if not values:
        raise RuntimeError(f"native report is missing required diagnostic `{key}`")
    if len(values) != 1:
        raise RuntimeError(
            f"native report has duplicate or conflicting `{key}` diagnostics"
        )
    return values[0]


def require_exact_diagnostic(report, key, expected):
    value = required_diagnostic_value(report, key)
    if value != expected:
        raise RuntimeError(
            f"native report has an unexpected `{key}` diagnostic value"
        )


def required_diagnostic(report, key, conversion):
    try:
        value = conversion(required_diagnostic_value(report, key))
    except (TypeError, ValueError) as error:
        raise RuntimeError(f"native report has an invalid `{key}` diagnostic") from error
    if conversion is float:
        require_nonnegative_number(value, key)
    return value


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


def require_generated_number(value, path, nonnegative=False):
    if (
        type(value) not in (int, float)
        or (type(value) is float and not math.isfinite(value))
        or (nonnegative and value < 0)
    ):
        raise RuntimeError(
            "native generated output does not match the WhisperX JSON contract: "
            f"`{path}` has an invalid number"
        )
    return value


def validate_generated_word(value, path):
    if not isinstance(value, dict):
        raise RuntimeError(
            "native generated output does not match the WhisperX JSON contract: "
            f"`{path}` must be an object"
        )
    if not isinstance(value.get("word"), str):
        raise RuntimeError(
            "native generated output does not match the WhisperX JSON contract: "
            f"`{path}.word` must be a string"
        )
    for key in ("start", "end"):
        if key in value:
            require_generated_number(value[key], f"{path}.{key}", nonnegative=True)
    if (
        "start" in value
        and "end" in value
        and value["end"] < value["start"]
    ):
        raise RuntimeError(
            "native generated output does not match the WhisperX JSON contract: "
            f"`{path}` ends before it starts"
        )
    if "score" in value:
        require_generated_number(value["score"], f"{path}.score")
    if "speaker" in value and not isinstance(value["speaker"], str):
        raise RuntimeError(
            "native generated output does not match the WhisperX JSON contract: "
            f"`{path}.speaker` must be a string"
        )


def validate_generated_segment(value, path):
    if not isinstance(value, dict):
        raise RuntimeError(
            "native generated output does not match the WhisperX JSON contract: "
            f"`{path}` must be an object"
        )
    segment_id = value.get("id")
    if type(segment_id) is not int or segment_id < 0:
        raise RuntimeError(
            "native generated output does not match the WhisperX JSON contract: "
            f"`{path}.id` must be a non-negative integer"
        )
    if not isinstance(value.get("text"), str):
        raise RuntimeError(
            "native generated output does not match the WhisperX JSON contract: "
            f"`{path}.text` must be a string"
        )
    for key in ("start", "end"):
        require_generated_number(
            value.get(key), f"{path}.{key}", nonnegative=True
        )
    if value["end"] < value["start"]:
        raise RuntimeError(
            "native generated output does not match the WhisperX JSON contract: "
            f"`{path}` ends before it starts"
        )
    if "score" in value:
        require_generated_number(value["score"], f"{path}.score")
    if "speaker" in value and not isinstance(value["speaker"], str):
        raise RuntimeError(
            "native generated output does not match the WhisperX JSON contract: "
            f"`{path}.speaker` must be a string"
        )
    if "words" in value:
        words = value["words"]
        if not isinstance(words, list):
            raise RuntimeError(
                "native generated output does not match the WhisperX JSON contract: "
                f"`{path}.words` must be an array"
            )
        for index, word in enumerate(words):
            validate_generated_word(word, f"{path}.words[{index}]")


def validate_output_json(report, output_dir):
    response = report.get("response", {})
    if response.get("accepted") is not True:
        raise RuntimeError("native report did not accept the transcription")
    transcript = response.get("transcript")
    if not isinstance(transcript, dict) or not isinstance(
        transcript.get("segments"), list
    ):
        raise RuntimeError("native report has no valid transcript segments array")
    output_files = report.get("outputFiles")
    if not isinstance(output_files, list) or not output_files:
        raise RuntimeError("native report has no generated JSON output")
    try:
        output_root = output_dir.resolve(strict=True)
    except OSError as error:
        raise RuntimeError("native output directory is unavailable") from error
    if not output_root.is_dir():
        raise RuntimeError("native output directory is unavailable")
    json_outputs = []
    for output in output_files:
        if (
            not isinstance(output, dict)
            or not isinstance(output.get("format"), str)
            or not isinstance(output.get("path"), str)
            or not output["format"]
            or not output["path"]
        ):
            raise RuntimeError("native report has an invalid outputFiles entry")
        try:
            resolved_path = Path(output["path"]).resolve(strict=True)
        except OSError as error:
            raise RuntimeError("native report references an unavailable output") from error
        if not resolved_path.is_relative_to(output_root) or not resolved_path.is_file():
            raise RuntimeError("native report references output outside the fresh directory")
        if output["format"] == "json":
            json_outputs.append(resolved_path)
    if len(json_outputs) != 1:
        raise RuntimeError("native report must contain exactly one generated JSON output")
    try:
        generated = json.loads(json_outputs[0].read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError("native generated output is not valid JSON") from error
    if (
        not isinstance(generated, dict)
        or not isinstance(generated.get("text"), str)
        or not isinstance(generated.get("segments"), list)
        or not isinstance(generated.get("word_segments"), list)
    ):
        raise RuntimeError(
            "native generated output does not match the WhisperX JSON contract"
        )
    for index, segment in enumerate(generated["segments"]):
        validate_generated_segment(segment, f"segments[{index}]")
    for index, word in enumerate(generated["word_segments"]):
        validate_generated_word(word, f"word_segments[{index}]")
    return generated["text"]


def run_measurement(
    binary, bundle, clip, audio_duration, clip_id, mode, phase, iteration
):
    with tempfile.TemporaryDirectory(prefix="native-whisperx-q8-evidence-") as temp:
        run_root = Path(temp)
        report_path = run_root / "report.json"
        output_dir = run_root / "output"
        mode_config = MODES[mode]
        command = [
            str(binary),
            "transcribe",
            str(clip),
            "--provider",
            "native",
            "--device",
            "cpu",
            "--compute-type",
            mode_config["computeType"],
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
                f"native {mode} transcription failed for {clip_id} {phase} {iteration}"
            )
        try:
            report = json.loads(report_path.read_text())
        except (OSError, json.JSONDecodeError) as error:
            raise RuntimeError(
                f"native {mode} report is invalid for {clip_id} {phase} {iteration}"
            ) from error
        transcript_text = validate_output_json(report, output_dir)
        require_exact_diagnostic(report, "provider", "candle-whisper")
        require_exact_diagnostic(
            report,
            "requestedComputeType",
            mode_config["diagnosticComputeType"],
        )
        require_exact_diagnostic(
            report,
            "resolvedComputeType",
            mode_config["diagnosticComputeType"],
        )
        require_exact_diagnostic(report, "modelFormat", mode_config["modelFormat"])
        asr_seconds = required_diagnostic(report, "phaseTiming.asrSeconds", float)
        fallback_reasons = diagnostic_values(report, "timingFallback")
        return {
            "wallSeconds": elapsed,
            "realtimeFactor": asr_seconds / audio_duration,
            "modelLoadSeconds": required_diagnostic(
                report, "phaseAsrModelLoadSeconds", float
            ),
            "encoderSeconds": required_diagnostic(
                report, "phaseTiming.encoderSeconds", float
            ),
            "decoderSeconds": required_diagnostic(
                report, "phaseTiming.decoderSeconds", float
            ),
            "asrSeconds": asr_seconds,
            "generatedTokenCount": required_diagnostic(
                report, "generatedTokenCount", int
            ),
            "timestampFallback": {
                "used": bool(fallback_reasons),
                "reasons": fallback_reasons,
            },
            "outputJsonValid": True,
            "transcriptText": transcript_text,
            "rawReport": report,
        }


def sha256_file(path):
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def bundle_hashes(bundle, mode):
    try:
        if not bundle.is_dir():
            raise RuntimeError(f"caller-owned {mode} bundle is unavailable")
        expected = MODES[mode]
        required_files = [*SIDECAR_FILES, expected["modelFile"]]
        paths = {name: bundle / name for name in required_files}
        if any(not path.is_file() for path in paths.values()):
            raise RuntimeError(
                f"caller-owned {mode} bundle must contain {', '.join(required_files)}"
            )
        return {
            "model": {
                "file": expected["modelFile"],
                "sha256": sha256_file(paths[expected["modelFile"]]),
            },
            "sidecars": {
                filename: sha256_file(paths[filename]) for filename in SIDECAR_FILES
            },
        }
    except OSError as error:
        raise RuntimeError(
            f"caller-owned {mode} bundle could not be read or hashed"
        ) from error


def validate_file(path, message):
    if not path.is_file():
        raise RuntimeError(message)


def run_evidence(args):
    binary = Path(args.binary)
    bundles = {"q8": Path(args.q8_bundle), "fp32": Path(args.fp32_bundle)}
    clips_by_id = {
        "shrek-retold-1s": Path(args.one_second_wav),
        "shrek-retold-15s": Path(args.fifteen_second_wav),
    }
    validate_file(binary, "native-whisperx binary is unavailable")
    hashes = {mode: bundle_hashes(bundle, mode) for mode, bundle in bundles.items()}
    if hashes["q8"]["sidecars"] != hashes["fp32"]["sidecars"]:
        raise RuntimeError("Q8 and FP32 bundles must have byte-identical sidecars")
    clips = []
    for clip_id, clip_path in clips_by_id.items():
        validate_file(clip_path, "caller-owned Shrek-derived WAV is unavailable")
        duration = wav_duration_seconds(clip_path)
        minimum, maximum = EXPECTED_CLIPS[clip_id]["duration"]
        if not minimum <= duration <= maximum:
            raise RuntimeError(f"{clip_id} WAV duration is outside its required range")
        results = {
            mode: {"warmup": None, "measured": []} for mode in MODES
        }
        order = expected_execution_order()
        for entry in order:
            for mode in entry["modes"]:
                result = run_measurement(
                    binary,
                    bundles[mode],
                    clip_path,
                    duration,
                    clip_id,
                    mode,
                    entry["phase"],
                    entry["iteration"],
                )
                if entry["phase"] == "warmup":
                    results[mode]["warmup"] = result
                else:
                    results[mode]["measured"].append(result)
        clips.append(
            {
                "id": clip_id,
                "input": str(clip_path),
                "audioDurationSeconds": duration,
                "executionOrder": order,
                "modes": results,
            }
        )
    raw = {
        "schemaVersion": SCHEMA_VERSION,
        "evidenceClass": EVIDENCE_CLASS,
        "generatedAt": datetime.datetime.now(datetime.timezone.utc)
        .isoformat()
        .replace("+00:00", "Z"),
        "cpu": {"model": cpu_model()},
        "bundleHashes": hashes,
        "configuration": {
            **EXPECTED_CONFIGURATION,
            "modes": {
                mode: {
                    "computeType": config["computeType"],
                    "modelFile": config["modelFile"],
                    "bundle": str(bundles[mode]),
                }
                for mode, config in MODES.items()
            },
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
        description="Run or sanitize matched native Q8-versus-FP32 CPU ASR evidence"
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    run = subparsers.add_parser("run")
    run.add_argument("--binary", required=True)
    run.add_argument("--q8-bundle", required=True)
    run.add_argument("--fp32-bundle", required=True)
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

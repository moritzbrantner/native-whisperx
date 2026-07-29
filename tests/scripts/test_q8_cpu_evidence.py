import copy
import importlib.util
import json
import math
import os
import stat
import subprocess
import sys
import tempfile
import unittest
import wave
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT = REPO_ROOT / "scripts" / "q8_cpu_evidence.py"
SIDECARS = (
    "config.json",
    "generation_config.json",
    "tokenizer.json",
    "preprocessor_config.json",
)


def load_evidence_module():
    spec = importlib.util.spec_from_file_location("q8_cpu_evidence", SCRIPT)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


class Q8CpuEvidenceTests(unittest.TestCase):
    def test_sanitized_summary_reports_matched_comparative_gate(self):
        evidence = load_evidence_module()

        summary = evidence.sanitize_report(valid_raw_report())

        self.assertEqual(summary["schemaVersion"], 2)
        self.assertEqual(
            summary["comparativeGate"]["metric"], "medianReportedAsrSeconds"
        )
        self.assertEqual(
            summary["comparativeGate"]["cases"],
            [
                {
                    "clipId": "shrek-retold-1s",
                    "q8MedianAsrSeconds": 10.0,
                    "fp32MedianAsrSeconds": 10.0,
                    "q8ToFp32Ratio": 1.0,
                    "maximumRatio": 1.1,
                    "passed": True,
                },
                {
                    "clipId": "shrek-retold-15s",
                    "q8MedianAsrSeconds": 8.0,
                    "fp32MedianAsrSeconds": 10.0,
                    "q8ToFp32Ratio": 0.8,
                    "maximumRatio": 0.9,
                    "passed": True,
                },
            ],
        )
        self.assertTrue(summary["comparativeGate"]["passed"])

    def test_sanitized_summary_is_whitelist_only(self):
        evidence = load_evidence_module()
        raw = valid_raw_report()

        summary = evidence.sanitize_report(raw)

        serialized = json.dumps(summary)
        self.assertNotIn("/secret", serialized)
        self.assertNotIn("/private", serialized)
        self.assertNotIn("rawReport", serialized)
        self.assertNotIn("transcriptText", serialized)
        self.assertEqual(summary["cpu"], raw["cpu"])
        self.assertEqual(
            summary["bundleHashes"]["q8"]["model"]["file"], "model.q8_0.gguf"
        )
        self.assertEqual(
            summary["bundleHashes"]["fp32"]["model"]["file"], "model.safetensors"
        )

    def test_sanitizer_rejects_wrong_schema_and_configuration_constants(self):
        evidence = load_evidence_module()
        faults = {
            "schema": ("schemaVersion", 1),
            "class": ("evidenceClass", "q8-cpu-asr-only"),
            "provider": ("configuration.provider", "external-whisperx"),
            "device": ("configuration.device", "cuda"),
            "alignment": ("configuration.alignment", True),
            "warmups": ("configuration.warmupRunsPerModePerClip", 0),
            "measurements": ("configuration.measuredRunsPerModePerClip", 2),
            "alternation": ("configuration.alternatingOrder", False),
            "q8 compute": ("configuration.modes.q8.computeType", "float32"),
            "fp32 model": (
                "configuration.modes.fp32.modelFile",
                "model.q8_0.gguf",
            ),
        }
        for label, (path, invalid) in faults.items():
            with self.subTest(label=label):
                raw = valid_raw_report()
                set_path(raw, path, invalid)
                with self.assertRaises(RuntimeError):
                    evidence.sanitize_report(raw)

    def test_sanitizer_requires_safe_hashes_and_identical_sidecars(self):
        evidence = load_evidence_module()
        faults = {}
        malformed = valid_raw_report()
        malformed["bundleHashes"]["q8"]["model"]["sha256"] = "/secret/model"
        faults["malformed model hash"] = malformed
        different = valid_raw_report()
        different["bundleHashes"]["fp32"]["sidecars"]["config.json"] = "f" * 64
        faults["different sidecar"] = different
        missing = valid_raw_report()
        del missing["bundleHashes"]["q8"]["sidecars"]["tokenizer.json"]
        faults["missing sidecar"] = missing
        wrong_model = valid_raw_report()
        wrong_model["bundleHashes"]["fp32"]["model"]["file"] = "weights.bin"
        faults["wrong model filename"] = wrong_model
        for label, raw in faults.items():
            with self.subTest(label=label):
                with self.assertRaises(RuntimeError):
                    evidence.sanitize_report(raw)

    def test_sanitizer_requires_recorded_alternating_execution_order(self):
        evidence = load_evidence_module()
        raw = valid_raw_report()
        raw["clips"][0]["executionOrder"][1]["modes"] = ["q8", "fp32"]

        with self.assertRaisesRegex(RuntimeError, "executionOrder"):
            evidence.sanitize_report(raw)

    def test_sanitizer_requires_each_mode_warmup_and_three_measurements(self):
        evidence = load_evidence_module()
        faults = {}
        no_warmup = valid_raw_report()
        del no_warmup["clips"][0]["modes"]["q8"]["warmup"]
        faults["missing warmup"] = no_warmup
        two_runs = valid_raw_report()
        two_runs["clips"][1]["modes"]["fp32"]["measured"].pop()
        faults["two measurements"] = two_runs
        no_clip = valid_raw_report()
        no_clip["clips"].pop()
        faults["missing clip"] = no_clip
        duplicate = valid_raw_report()
        duplicate["clips"][1]["id"] = "shrek-retold-1s"
        faults["duplicate clip"] = duplicate
        for label, raw in faults.items():
            with self.subTest(label=label):
                with self.assertRaises(RuntimeError):
                    evidence.sanitize_report(raw)

    def test_sanitizer_rejects_invalid_phase_tokens_fallback_and_output(self):
        evidence = load_evidence_module()
        faults = {
            "non-finite phase": ("encoderSeconds", math.inf),
            "negative ASR": ("asrSeconds", -0.1),
            "fractional tokens": ("generatedTokenCount", 1.5),
            "bad output": ("outputJsonValid", False),
            "path timing": ("wallSeconds", "/private/timing"),
        }
        for label, (field, invalid) in faults.items():
            with self.subTest(label=label):
                raw = valid_raw_report()
                raw["clips"][0]["modes"]["q8"]["measured"][0][field] = invalid
                with self.assertRaises(RuntimeError):
                    evidence.sanitize_report(raw)
        for fallback in (
            {"used": True, "reasons": []},
            {"used": False, "reasons": ["missingTimestampMetadata"]},
            {"used": True, "reasons": ["unknown"]},
        ):
            with self.subTest(fallback=fallback):
                raw = valid_raw_report()
                raw["clips"][0]["modes"]["q8"]["measured"][0][
                    "timestampFallback"
                ] = fallback
                with self.assertRaises(RuntimeError):
                    evidence.sanitize_report(raw)

    def test_summary_records_transcript_equality_without_transcript_content(self):
        evidence = load_evidence_module()
        raw = valid_raw_report()
        raw["clips"][0]["modes"]["q8"]["measured"][1][
            "transcriptText"
        ] = "different words"

        summary = evidence.sanitize_report(raw)

        equality = summary["clips"][0]["transcriptEquality"]
        self.assertEqual(equality["measured"], [True, False, True])
        self.assertFalse(equality["all"])
        self.assertNotIn("different words", json.dumps(summary))

    def test_gate_uses_median_reported_asr_seconds_not_wall_clock(self):
        evidence = load_evidence_module()
        raw = valid_raw_report()
        q8_runs = raw["clips"][0]["modes"]["q8"]["measured"]
        fp32_runs = raw["clips"][0]["modes"]["fp32"]["measured"]
        for run, asr, wall in zip(q8_runs, [100.0, 1.0, 1.0], [0.01, 0.01, 0.01]):
            run["asrSeconds"] = asr
            run["realtimeFactor"] = asr
            run["wallSeconds"] = wall
        for run, asr, wall in zip(fp32_runs, [1.0, 1.0, 100.0], [999.0, 999.0, 999.0]):
            run["asrSeconds"] = asr
            run["realtimeFactor"] = asr
            run["wallSeconds"] = wall

        summary = evidence.sanitize_report(raw)

        case = summary["comparativeGate"]["cases"][0]
        self.assertEqual(case["q8MedianAsrSeconds"], 1.0)
        self.assertEqual(case["fp32MedianAsrSeconds"], 1.0)
        self.assertEqual(case["q8ToFp32Ratio"], 1.0)
        self.assertTrue(case["passed"])

    def test_acceptance_enforces_both_comparative_thresholds(self):
        evidence = load_evidence_module()
        for clip_index, q8_asr, message in (
            (0, 11.1, "matched FP32 comparative gate"),
            (1, 9.1, "matched FP32 comparative gate"),
        ):
            with self.subTest(clip_index=clip_index):
                raw = valid_raw_report()
                for run in raw["clips"][clip_index]["modes"]["q8"]["measured"]:
                    run["asrSeconds"] = q8_asr
                    run["realtimeFactor"] = q8_asr / (
                        1.0 if clip_index == 0 else 15.0
                    )
                summary = evidence.sanitize_report(raw)
                with self.assertRaisesRegex(RuntimeError, message):
                    evidence.validate_acceptance(summary)

    def test_runner_uses_matched_bundles_and_alternates_mode_order(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            result, raw_path, summary_path, log_path = run_evidence_with_fake(root)

            self.assertEqual(result.returncode, 0, result.stderr)
            raw = json.loads(raw_path.read_text())
            summary = json.loads(summary_path.read_text())
            expected_compute_order = [
                "int8",
                "float32",
                "float32",
                "int8",
                "int8",
                "float32",
                "float32",
                "int8",
            ]
            logged = [line.split(":") for line in log_path.read_text().splitlines()]
            self.assertEqual(
                [compute for clip, compute in logged[:8]], expected_compute_order
            )
            self.assertEqual(
                [compute for clip, compute in logged[8:]], expected_compute_order
            )
            self.assertEqual(
                raw["configuration"]["modes"]["q8"]["modelFile"],
                "model.q8_0.gguf",
            )
            self.assertEqual(
                raw["configuration"]["modes"]["fp32"]["modelFile"],
                "model.safetensors",
            )
            self.assertTrue(summary["comparativeGate"]["passed"])
            for clip in raw["clips"]:
                for mode in ("q8", "fp32"):
                    self.assertEqual(len(clip["modes"][mode]["measured"]), 3)
                    for run in [
                        clip["modes"][mode]["warmup"],
                        *clip["modes"][mode]["measured"],
                    ]:
                        self.assertEqual(
                            run["realtimeFactor"],
                            run["asrSeconds"] / clip["audioDurationSeconds"],
                        )

    def test_runner_hashes_exact_bundle_models_and_matching_sidecars(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            result, raw_path, _, _ = run_evidence_with_fake(root)

            self.assertEqual(result.returncode, 0, result.stderr)
            raw = json.loads(raw_path.read_text())
            self.assertEqual(
                raw["bundleHashes"]["q8"]["sidecars"],
                raw["bundleHashes"]["fp32"]["sidecars"],
            )
            self.assertNotEqual(
                raw["bundleHashes"]["q8"]["model"]["sha256"],
                raw["bundleHashes"]["fp32"]["model"]["sha256"],
            )

    def test_runner_rejects_missing_model_or_mismatched_sidecars_before_running(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            resources = make_resources(root)
            resources["fp32_bundle"].joinpath("model.safetensors").unlink()
            result = invoke_runner(root, resources)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("model.safetensors", result.stderr)
            self.assertFalse(resources["log"].exists())
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            resources = make_resources(root)
            resources["fp32_bundle"].joinpath("config.json").write_text("different")
            result = invoke_runner(root, resources)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("byte-identical sidecars", result.stderr)
            self.assertFalse(resources["log"].exists())

    def test_runner_requires_fresh_valid_output_and_mode_diagnostics(self):
        faults = {
            "invalid JSON contract": FAKE_NATIVE_WHISPERX.replace(
                '"segments": [],', '"segments": {},'
            ),
            "wrong model diagnostic": FAKE_NATIVE_WHISPERX.replace(
                'model_format = "gguf-q8_0" if compute == "int8" else "safetensors"',
                'model_format = "safetensors"',
            ),
            "non-finite phase": FAKE_NATIVE_WHISPERX.replace(
                '"phaseTiming.encoderSeconds=0.25"', '"phaseTiming.encoderSeconds=nan"'
            ),
        }
        for label, source in faults.items():
            with self.subTest(label=label), tempfile.TemporaryDirectory() as temp:
                root = Path(temp)
                result, _, _, _ = run_evidence_with_fake(root, source)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("q8 CPU evidence failed", result.stderr)

    def test_sanitize_command_rejects_failed_gate_without_emitting_summary(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            raw = valid_raw_report()
            for run in raw["clips"][1]["modes"]["q8"]["measured"]:
                run["asrSeconds"] = 9.1
                run["realtimeFactor"] = 9.1 / 15.0
            raw_path = root / "raw.json"
            summary_path = root / "summary.json"
            raw_path.write_text(json.dumps(raw))

            result = subprocess.run(
                [
                    sys.executable,
                    str(SCRIPT),
                    "sanitize",
                    str(raw_path),
                    str(summary_path),
                ],
                check=False,
                capture_output=True,
                text=True,
            )

            self.assertNotEqual(result.returncode, 0)
            self.assertIn("comparative gate", result.stderr)
            self.assertFalse(summary_path.exists())

    def test_summary_validation_rejects_tampered_medians_ratios_or_gate_result(self):
        evidence = load_evidence_module()
        for field, value in (
            ("q8MedianAsrSeconds", 999.0),
            ("q8ToFp32Ratio", 0.01),
            ("passed", False),
        ):
            with self.subTest(field=field):
                summary = evidence.sanitize_report(valid_raw_report())
                summary["comparativeGate"]["cases"][0][field] = value
                with self.assertRaises(RuntimeError):
                    evidence.validate_summary(summary)

    def test_sanitizer_requires_realtime_factor_from_reported_asr_seconds(self):
        evidence = load_evidence_module()
        raw = valid_raw_report()
        raw["clips"][1]["modes"]["q8"]["measured"][0]["realtimeFactor"] = 99.0

        with self.assertRaisesRegex(RuntimeError, "reported ASR seconds"):
            evidence.sanitize_report(raw)


def valid_measurement(
    asr_seconds=10.0, audio_duration=1.0, transcript="same transcript"
):
    return {
        "wallSeconds": 12.0,
        "realtimeFactor": asr_seconds / audio_duration,
        "modelLoadSeconds": 1.0,
        "encoderSeconds": 2.0,
        "decoderSeconds": 7.0,
        "asrSeconds": asr_seconds,
        "generatedTokenCount": 4,
        "timestampFallback": {
            "used": False,
            "reasons": [],
            "private": "/private/fallback",
        },
        "outputJsonValid": True,
        "transcriptText": transcript,
        "rawReport": {"response": {"diagnostics": ["localPath=/private/path"]}},
    }


def mode_runs(asr_seconds, audio_duration=1.0):
    measurement = valid_measurement(asr_seconds, audio_duration)
    return {
        "warmup": copy.deepcopy(measurement),
        "measured": [copy.deepcopy(measurement) for _ in range(3)],
    }


def valid_raw_report():
    sidecars = {
        "config.json": "1" * 64,
        "generation_config.json": "2" * 64,
        "tokenizer.json": "3" * 64,
        "preprocessor_config.json": "4" * 64,
    }
    order = [
        {"phase": "warmup", "iteration": 0, "modes": ["q8", "fp32"]},
        {"phase": "measured", "iteration": 1, "modes": ["fp32", "q8"]},
        {"phase": "measured", "iteration": 2, "modes": ["q8", "fp32"]},
        {"phase": "measured", "iteration": 3, "modes": ["fp32", "q8"]},
    ]
    return {
        "schemaVersion": 2,
        "evidenceClass": "q8-fp32-cpu-asr-comparison",
        "generatedAt": "2026-07-28T12:00:00Z",
        "cpu": {"model": "Generic CPU"},
        "bundleHashes": {
            "q8": {
                "model": {"file": "model.q8_0.gguf", "sha256": "a" * 64},
                "sidecars": copy.deepcopy(sidecars),
            },
            "fp32": {
                "model": {"file": "model.safetensors", "sha256": "b" * 64},
                "sidecars": copy.deepcopy(sidecars),
            },
        },
        "configuration": {
            "provider": "native",
            "device": "cpu",
            "alignment": False,
            "warmupRunsPerModePerClip": 1,
            "measuredRunsPerModePerClip": 3,
            "alternatingOrder": True,
            "modes": {
                "q8": {
                    "computeType": "int8",
                    "modelFile": "model.q8_0.gguf",
                    "bundle": "/secret/q8",
                },
                "fp32": {
                    "computeType": "float32",
                    "modelFile": "model.safetensors",
                    "bundle": "/secret/fp32",
                },
            },
        },
        "clips": [
            {
                "id": "shrek-retold-1s",
                "input": "/private/shrek-retold-1s.wav",
                "audioDurationSeconds": 1.0,
                "executionOrder": copy.deepcopy(order),
                "modes": {"q8": mode_runs(10.0), "fp32": mode_runs(10.0)},
            },
            {
                "id": "shrek-retold-15s",
                "input": "/private/shrek-retold-15s.wav",
                "audioDurationSeconds": 15.0,
                "executionOrder": copy.deepcopy(order),
                "modes": {
                    "q8": mode_runs(8.0, 15.0),
                    "fp32": mode_runs(10.0, 15.0),
                },
            },
        ],
    }


def set_path(value, path, replacement):
    target = value
    parts = path.split(".")
    for part in parts[:-1]:
        target = target[part]
    target[parts[-1]] = replacement


def write_silent_wav(path, seconds):
    with wave.open(str(path), "wb") as wav:
        wav.setnchannels(1)
        wav.setsampwidth(2)
        wav.setframerate(16_000)
        wav.writeframes(b"\0\0" * 16_000 * seconds)


def write_bundle(path, model_file, model_content):
    path.mkdir()
    for sidecar in SIDECARS:
        path.joinpath(sidecar).write_text(f"shared {sidecar}")
    path.joinpath(model_file).write_text(model_content)


def make_resources(root, fake_source=None):
    q8_bundle = root / "q8"
    fp32_bundle = root / "fp32"
    write_bundle(q8_bundle, "model.q8_0.gguf", "quantized model")
    write_bundle(fp32_bundle, "model.safetensors", "full precision model")
    one_second = root / "shrek-retold-1s.wav"
    fifteen_seconds = root / "shrek-retold-15s.wav"
    write_silent_wav(one_second, 1)
    write_silent_wav(fifteen_seconds, 15)
    binary = root / "fake-native-whisperx"
    binary.write_text(fake_source or FAKE_NATIVE_WHISPERX)
    binary.chmod(binary.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP)
    return {
        "q8_bundle": q8_bundle,
        "fp32_bundle": fp32_bundle,
        "one_second": one_second,
        "fifteen_seconds": fifteen_seconds,
        "binary": binary,
        "raw": root / "raw" / "evidence.json",
        "summary": root / "summary.json",
        "log": root / "invocations.log",
    }


def invoke_runner(root, resources):
    environment = os.environ.copy()
    environment["FAKE_INVOCATION_LOG"] = str(resources["log"])
    return subprocess.run(
        [
            sys.executable,
            str(SCRIPT),
            "run",
            "--binary",
            str(resources["binary"]),
            "--q8-bundle",
            str(resources["q8_bundle"]),
            "--fp32-bundle",
            str(resources["fp32_bundle"]),
            "--one-second-wav",
            str(resources["one_second"]),
            "--fifteen-second-wav",
            str(resources["fifteen_seconds"]),
            "--raw-report",
            str(resources["raw"]),
            "--summary",
            str(resources["summary"]),
        ],
        check=False,
        capture_output=True,
        text=True,
        env=environment,
    )


def run_evidence_with_fake(root, fake_source=None):
    resources = make_resources(root, fake_source)
    result = invoke_runner(root, resources)
    return result, resources["raw"], resources["summary"], resources["log"]


FAKE_NATIVE_WHISPERX = """#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

args = sys.argv[1:]
assert args[0] == "transcribe"
def value(flag):
    return args[args.index(flag) + 1]
assert value("--provider") == "native"
assert value("--device") == "cpu"
assert "--no-align" in args
assert value("--format") == "json"
compute = value("--compute-type")
bundle = Path(value("--whisper-bundle"))
if compute == "int8":
    assert bundle.joinpath("model.q8_0.gguf").is_file()
else:
    assert compute == "float32"
    assert bundle.joinpath("model.safetensors").is_file()
clip = Path(args[1])
with Path(os.environ["FAKE_INVOCATION_LOG"]).open("a") as log:
    log.write(f"{clip.stem}:{compute}\\n")
report_path = Path(value("--report"))
output_dir = Path(value("--output-dir"))
output_dir.mkdir(parents=True, exist_ok=True)
output_json = output_dir / (clip.stem + ".json")
output_json.write_text(json.dumps({
    "text": "same transcript",
    "segments": [],
    "word_segments": []
}))
diagnostic_compute = "int8" if compute == "int8" else "fp32"
model_format = "gguf-q8_0" if compute == "int8" else "safetensors"
asr_seconds = 0.8 if compute == "int8" and clip.stem.endswith("15s") else 1.0
report_path.write_text(json.dumps({
    "response": {
        "accepted": True,
        "transcript": {"segments": []},
        "diagnostics": [
            "provider=candle-whisper",
            f"requestedComputeType={diagnostic_compute}",
            f"resolvedComputeType={diagnostic_compute}",
            f"modelFormat={model_format}",
            "phaseAsrModelLoadSeconds=0.5",
            "phaseTiming.encoderSeconds=0.25",
            "phaseTiming.decoderSeconds=0.75",
            f"phaseTiming.asrSeconds={asr_seconds}",
            "generatedTokenCount=5",
            "timingFallback=missingTimestampMetadata"
        ]
    },
    "outputFiles": [{"format": "json", "path": str(output_json)}]
}))
"""


if __name__ == "__main__":
    unittest.main()

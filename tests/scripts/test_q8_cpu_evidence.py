import copy
import importlib.util
import json
import math
import stat
import subprocess
import sys
import tempfile
import unittest
import wave
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT = REPO_ROOT / "scripts" / "q8_cpu_evidence.py"


def load_evidence_module():
    spec = importlib.util.spec_from_file_location("q8_cpu_evidence", SCRIPT)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


class Q8CpuEvidenceTests(unittest.TestCase):
    def test_sanitized_summary_is_whitelist_only(self):
        evidence = load_evidence_module()
        raw = valid_raw_report()

        summary = evidence.sanitize_report(raw)

        serialized = evidence.json.dumps(summary)
        self.assertNotIn("/secret", serialized)
        self.assertNotIn("/private", serialized)
        self.assertNotIn("rawReport", serialized)
        self.assertNotIn("private", serialized)
        self.assertEqual(summary["cpu"]["model"], raw["cpu"]["model"])
        self.assertEqual(
            summary["clips"][0]["measured"][0]["generatedTokenCount"], 4
        )
        self.assertEqual(
            summary["oneSecondThreshold"],
            {
                "applicable": True,
                "limitSeconds": 45.0,
                "passed": True,
            },
        )

    def test_sanitizer_rejects_wrong_schema_and_configuration_constants(self):
        evidence = load_evidence_module()
        faults = {
            "schemaVersion": ("schemaVersion", 2),
            "evidenceClass": ("evidenceClass", "private:/tmp/evidence"),
            "provider": ("configuration.provider", "external-whisperx"),
            "device": ("configuration.device", "cuda"),
            "computeType": ("configuration.computeType", "float32"),
            "alignment": ("configuration.alignment", True),
            "warmupRunsPerClip": ("configuration.warmupRunsPerClip", 0),
            "measuredRunsPerClip": ("configuration.measuredRunsPerClip", 2),
        }

        for label, (path, invalid_value) in faults.items():
            with self.subTest(label=label):
                raw = valid_raw_report()
                set_path(raw, path, invalid_value)
                with self.assertRaises(RuntimeError):
                    evidence.sanitize_report(raw)

    def test_sanitizer_requires_cpu_timestamp_warmup_and_exact_clip_runs(self):
        evidence = load_evidence_module()
        faults = {}

        missing_clip = valid_raw_report()
        missing_clip["clips"].pop()
        faults["missing clip"] = missing_clip

        duplicate_clip = valid_raw_report()
        duplicate_clip["clips"][1]["id"] = "shrek-retold-1s"
        faults["duplicate clip"] = duplicate_clip

        missing_warmup = valid_raw_report()
        del missing_warmup["clips"][0]["warmup"]
        faults["missing warmup"] = missing_warmup

        insufficient_runs = valid_raw_report()
        insufficient_runs["clips"][0]["measured"].clear()
        faults["empty measured runs"] = insufficient_runs

        wrong_duration = valid_raw_report()
        wrong_duration["clips"][1]["audioDurationSeconds"] = 1.0
        faults["wrong duration"] = wrong_duration

        missing_cpu = valid_raw_report()
        missing_cpu["cpu"]["model"] = ""
        faults["missing CPU identity"] = missing_cpu

        invalid_timestamp = valid_raw_report()
        invalid_timestamp["generatedAt"] = "/private/generated-at"
        faults["invalid generated timestamp"] = invalid_timestamp

        for label, raw in faults.items():
            with self.subTest(label=label):
                with self.assertRaises(RuntimeError):
                    evidence.sanitize_report(raw)

    def test_sanitizer_rejects_invalid_measurement_scalars_and_fallbacks(self):
        evidence = load_evidence_module()
        faults = {
            "private path": ("wallSeconds", "/private/timing"),
            "NaN": ("encoderSeconds", math.nan),
            "infinity": ("decoderSeconds", math.inf),
            "negative timing": ("asrSeconds", -0.01),
            "fractional token count": ("generatedTokenCount", 1.5),
            "negative token count": ("generatedTokenCount", -1),
            "invalid output flag": ("outputJsonValid", False),
        }

        for label, (field, invalid_value) in faults.items():
            with self.subTest(label=label):
                raw = valid_raw_report()
                raw["clips"][0]["measured"][0][field] = invalid_value
                with self.assertRaises(RuntimeError):
                    evidence.sanitize_report(raw)

        fallback_faults = [
            {"used": "yes", "reasons": []},
            {"used": False, "reasons": ["missingTimestampMetadata"]},
            {"used": True, "reasons": []},
            {"used": True, "reasons": ["private:/tmp/fallback"]},
            {
                "used": True,
                "reasons": [
                    "missingTimestampMetadata",
                    "missingTimestampMetadata",
                ],
            },
        ]
        for invalid_fallback in fallback_faults:
            with self.subTest(fallback=invalid_fallback):
                raw = valid_raw_report()
                raw["clips"][0]["measured"][0][
                    "timestampFallback"
                ] = invalid_fallback
                with self.assertRaises(RuntimeError):
                    evidence.sanitize_report(raw)

    def test_runner_uses_one_warmup_and_three_cpu_no_align_measurements_per_clip(
        self,
    ):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            bundle = root / "bundle"
            bundle.mkdir()
            one_second = root / "shrek-retold-1s.wav"
            fifteen_seconds = root / "shrek-retold-15s.wav"
            write_silent_wav(one_second, 1)
            write_silent_wav(fifteen_seconds, 15)
            fake_binary = root / "fake-native-whisperx"
            fake_binary.write_text(FAKE_NATIVE_WHISPERX)
            fake_binary.chmod(
                fake_binary.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP
            )
            raw_report = root / "raw" / "q8-cpu.json"
            summary = root / "summary.json"

            result = subprocess.run(
                [
                    sys.executable,
                    str(SCRIPT),
                    "run",
                    "--binary",
                    str(fake_binary),
                    "--bundle",
                    str(bundle),
                    "--one-second-wav",
                    str(one_second),
                    "--fifteen-second-wav",
                    str(fifteen_seconds),
                    "--raw-report",
                    str(raw_report),
                    "--summary",
                    str(summary),
                ],
                check=False,
                capture_output=True,
                text=True,
            )

            self.assertEqual(result.returncode, 0, result.stderr)
            raw = json.loads(raw_report.read_text())
            self.assertEqual(raw["configuration"]["warmupRunsPerClip"], 1)
            self.assertEqual(raw["configuration"]["measuredRunsPerClip"], 3)
            self.assertEqual([clip["id"] for clip in raw["clips"]], [
                "shrek-retold-1s",
                "shrek-retold-15s",
            ])
            for clip in raw["clips"]:
                self.assertEqual(len(clip["measured"]), 3)
                for run in [clip["warmup"], *clip["measured"]]:
                    diagnostics = run["rawReport"]["response"]["diagnostics"]
                    self.assertIn("testArgs=provider:native", diagnostics)
                    self.assertIn("testArgs=device:cpu", diagnostics)
                    self.assertIn("testArgs=computeType:int8", diagnostics)
                    self.assertIn("testArgs=noAlign:true", diagnostics)
                    self.assertTrue(run["outputJsonValid"])
                    self.assertEqual(run["generatedTokenCount"], 5)
                    self.assertEqual(
                        run["timestampFallback"],
                        {"used": True, "reasons": ["missingTimestampMetadata"]},
                    )
            sanitized = json.loads(summary.read_text())
            self.assertNotIn(str(bundle), json.dumps(sanitized))
            self.assertNotIn(str(one_second), json.dumps(sanitized))

    def test_run_command_rejects_wrong_output_file_shape_and_invalid_diagnostics(self):
        faults = {
            "mixed string and object outputFiles entries": FAKE_NATIVE_WHISPERX.replace(
                '{"format": "json", "path": str(output_json)}',
                'str(output_json), {"format": "json", "path": str(output_json)}',
            ),
            "non-finite timing": FAKE_NATIVE_WHISPERX.replace(
                "phaseTiming.encoderSeconds=0.25",
                "phaseTiming.encoderSeconds=nan",
            ),
            "negative timing": FAKE_NATIVE_WHISPERX.replace(
                "phaseTiming.decoderSeconds=0.75",
                "phaseTiming.decoderSeconds=-0.75",
            ),
        }

        for label, fake_source in faults.items():
            with self.subTest(label=label):
                result = run_evidence_with_fake(fake_source)
                self.assertNotEqual(result.returncode, 0, result.stdout)
                self.assertIn("q8 CPU evidence failed", result.stderr)

    def test_run_command_requires_fresh_stable_whisperx_json_output(self):
        faults = {
            "empty JSON object": FAKE_NATIVE_WHISPERX.replace(
                '{\n    "text": "",\n    "segments": [],\n    "word_segments": []\n}',
                "{}",
            ),
            "wrong WhisperX field types": (
                FAKE_NATIVE_WHISPERX.replace('"text": ""', '"text": []')
                .replace('"segments": []', '"segments": {}')
                .replace('"word_segments": []', '"word_segments": ""')
            ),
            "reported path outside fresh output directory": (
                FAKE_NATIVE_WHISPERX.replace(
                    'output_json = output_dir / (Path(args[1]).stem + ".json")',
                    'output_json = output_dir.parent / "stale.json"',
                )
            ),
        }

        for label, fake_source in faults.items():
            with self.subTest(label=label):
                result = run_evidence_with_fake(fake_source)
                self.assertNotEqual(result.returncode, 0, result.stdout)
                self.assertIn("q8 CPU evidence failed", result.stderr)

    def test_i5_one_second_threshold_failure_blocks_evidence(self):
        evidence = load_evidence_module()
        raw = valid_raw_report()
        raw["clips"][0]["measured"][0]["wallSeconds"] = 45.0
        summary = evidence.sanitize_report(raw)

        with self.assertRaisesRegex(RuntimeError, "45-second"):
            evidence.validate_acceptance(summary)

    def test_sanitize_command_rejects_failed_i5_acceptance(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            raw = valid_raw_report()
            raw["clips"][0]["measured"][0]["wallSeconds"] = 45.0
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
            self.assertIn("45-second", result.stderr)
            self.assertFalse(summary_path.exists())

    def test_sanitize_command_rejects_invalid_evidence_without_emitting_summary(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            raw = valid_raw_report()
            raw["clips"][0]["warmup"]["wallSeconds"] = "/private/timing"
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
            self.assertIn("finite non-negative number", result.stderr)
            self.assertFalse(summary_path.exists())

    def test_acceptance_never_treats_empty_measurements_as_passing(self):
        evidence = load_evidence_module()
        summary = evidence.sanitize_report(valid_raw_report())
        summary["clips"][0]["measured"].clear()

        with self.assertRaisesRegex(RuntimeError, "exactly three"):
            evidence.validate_acceptance(summary)


def valid_measurement():
    return {
        "wallSeconds": 10.0,
        "realtimeFactor": 10.0,
        "modelLoadSeconds": 1.0,
        "encoderSeconds": 2.0,
        "decoderSeconds": 7.0,
        "asrSeconds": 9.0,
        "generatedTokenCount": 4,
        "timestampFallback": {
            "used": False,
            "reasons": [],
            "private": "/private/fallback",
        },
        "outputJsonValid": True,
        "rawReport": {
            "response": {"diagnostics": ["localPath=/private/path"]}
        },
    }


def valid_raw_report():
    one_second = valid_measurement()
    fifteen_seconds = valid_measurement()
    fifteen_seconds["realtimeFactor"] = 2.0 / 3.0
    return {
        "schemaVersion": 1,
        "evidenceClass": "q8-cpu-asr-only",
        "generatedAt": "2026-07-28T12:00:00Z",
        "cpu": {"model": "Intel(R) Core(TM) i5-6300U CPU @ 2.40GHz"},
        "configuration": {
            "provider": "native",
            "device": "cpu",
            "computeType": "int8",
            "alignment": False,
            "warmupRunsPerClip": 1,
            "measuredRunsPerClip": 3,
            "bundle": "/secret/models/q8",
        },
        "clips": [
            {
                "id": "shrek-retold-1s",
                "input": "/private/audio/shrek-1s.wav",
                "audioDurationSeconds": 1.0,
                "warmup": copy.deepcopy(one_second),
                "measured": [copy.deepcopy(one_second) for _ in range(3)],
            },
            {
                "id": "shrek-retold-15s",
                "input": "/private/audio/shrek-15s.wav",
                "audioDurationSeconds": 15.0,
                "warmup": copy.deepcopy(fifteen_seconds),
                "measured": [copy.deepcopy(fifteen_seconds) for _ in range(3)],
            },
        ],
    }


def set_path(value, path, replacement):
    target = value
    parts = path.split(".")
    for part in parts[:-1]:
        target = target[part]
    target[parts[-1]] = replacement


def run_evidence_with_fake(fake_source):
    with tempfile.TemporaryDirectory() as temp:
        root = Path(temp)
        bundle = root / "bundle"
        bundle.mkdir()
        one_second = root / "shrek-retold-1s.wav"
        fifteen_seconds = root / "shrek-retold-15s.wav"
        write_silent_wav(one_second, 1)
        write_silent_wav(fifteen_seconds, 15)
        fake_binary = root / "fake-native-whisperx"
        fake_binary.write_text(fake_source)
        fake_binary.chmod(fake_binary.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP)
        return subprocess.run(
            [
                sys.executable,
                str(SCRIPT),
                "run",
                "--binary",
                str(fake_binary),
                "--bundle",
                str(bundle),
                "--one-second-wav",
                str(one_second),
                "--fifteen-second-wav",
                str(fifteen_seconds),
                "--raw-report",
                str(root / "raw.json"),
                "--summary",
                str(root / "summary.json"),
            ],
            check=False,
            capture_output=True,
            text=True,
        )


def write_silent_wav(path, seconds):
    with wave.open(str(path), "wb") as wav:
        wav.setnchannels(1)
        wav.setsampwidth(2)
        wav.setframerate(16_000)
        wav.writeframes(b"\0\0" * 16_000 * seconds)


FAKE_NATIVE_WHISPERX = """#!/usr/bin/env python3
import json
import sys
from pathlib import Path

args = sys.argv[1:]
assert args[0] == "transcribe"
def value(flag):
    return args[args.index(flag) + 1]
assert value("--provider") == "native"
assert value("--device") == "cpu"
assert value("--compute-type") == "int8"
assert "--whisper-bundle" in args
assert "--no-align" in args
assert value("--format") == "json"
report_path = Path(value("--report"))
output_dir = Path(value("--output-dir"))
output_dir.mkdir(parents=True, exist_ok=True)
output_json = output_dir / (Path(args[1]).stem + ".json")
output_json.write_text(json.dumps({
    "text": "",
    "segments": [],
    "word_segments": []
}))
report_path.write_text(json.dumps({
    "response": {
        "accepted": True,
        "transcript": {"segments": []},
        "diagnostics": [
            "provider=candle-whisper",
            "requestedComputeType=int8",
            "resolvedComputeType=int8",
            "modelFormat=gguf-q8_0",
            "phaseAsrModelLoadSeconds=0.5",
            "phaseTiming.encoderSeconds=0.25",
            "phaseTiming.decoderSeconds=0.75",
            "phaseTiming.asrSeconds=1.0",
            "phaseAsrSeconds=1.1",
            "generatedTokenCount=5",
            "timingFallback=missingTimestampMetadata",
            "testArgs=provider:native",
            "testArgs=device:cpu",
            "testArgs=computeType:int8",
            "testArgs=noAlign:true"
        ]
    },
    "outputFiles": [{"format": "json", "path": str(output_json)}]
}))
"""


if __name__ == "__main__":
    unittest.main()

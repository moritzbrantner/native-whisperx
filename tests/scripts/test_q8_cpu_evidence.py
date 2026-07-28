import importlib.util
import json
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
        raw = {
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
                    "input": "/private/audio/shrek.wav",
                    "audioDurationSeconds": 1.0,
                    "warmup": {"wallSeconds": 40.0, "rawReport": {"secret": True}},
                    "measured": [
                        {
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
                                "response": {
                                    "diagnostics": ["localPath=/private/path"]
                                }
                            },
                        }
                    ]
                    * 3,
                }
            ],
        }

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

    def test_i5_one_second_threshold_failure_blocks_evidence(self):
        evidence = load_evidence_module()
        summary = {
            "oneSecondThreshold": {
                "applicable": True,
                "limitSeconds": 45.0,
                "passed": False,
            }
        }

        with self.assertRaisesRegex(RuntimeError, "45-second"):
            evidence.validate_acceptance(summary)

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
output_json.write_text(json.dumps({"segments": []}))
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
    "outputFiles": [str(output_json)]
}))
"""


if __name__ == "__main__":
    unittest.main()

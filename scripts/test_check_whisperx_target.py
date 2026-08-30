import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("check_whisperx_target.py")
SPEC = importlib.util.spec_from_file_location("check_whisperx_target", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(MODULE)


class WhisperxTargetTests(unittest.TestCase):
    def policy(self, baseline: str = "3.8.6") -> dict[str, object]:
        return {
            "schemaVersion": 1,
            "verifiedCompatibilityBaseline": baseline,
            "upstreamPackage": "whisperx",
        }

    def test_reports_current_when_versions_match(self) -> None:
        report = MODULE.build_report(self.policy(), "3.8.6")
        self.assertEqual(report["status"], "current")
        self.assertEqual(report["upstreamTarget"], "3.8.6")

    def test_reports_behind_upstream_for_newer_release(self) -> None:
        report = MODULE.build_report(self.policy(), "3.9.0")
        self.assertEqual(report["status"], "behind upstream")
        self.assertEqual(report["verifiedCompatibilityBaseline"], "3.8.6")

    def test_version_comparison_normalizes_component_width(self) -> None:
        self.assertEqual(MODULE.compare_versions("3.8", "3.8.0"), 0)
        self.assertLess(MODULE.compare_versions("3.8.6", "3.10.0"), 0)

    def test_offline_cli_override_does_not_require_network(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "policy.json"
            path.write_text(json.dumps(self.policy()), encoding="utf-8")
            self.assertEqual(
                MODULE.main(["--policy", str(path), "--latest-version", "3.9.0"]),
                0,
            )

    def test_invalid_policy_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "policy.json"
            path.write_text("{}", encoding="utf-8")
            with self.assertRaises(MODULE.ReconnaissanceError):
                MODULE.load_policy(path)


if __name__ == "__main__":
    unittest.main()

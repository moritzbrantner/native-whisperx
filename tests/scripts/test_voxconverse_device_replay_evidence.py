"""Trust boundaries for device-replay diarization evidence."""
import importlib.util
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

SCRIPTS = Path(__file__).resolve().parents[2] / "scripts"
sys.path.insert(0, str(SCRIPTS))
SPEC = importlib.util.spec_from_file_location("replay_evidence", SCRIPTS / "voxconverse_device_replay_evidence.py")
replay = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(replay)


class ReplayTests(unittest.TestCase):
    def test_parses_run_records(self):
        self.assertEqual(replay.RUN_LINE.match("iabca exit=0 wall=118.6").groups(), ("iabca", "0", "118.6"))
        self.assertIsNone(replay.RUN_LINE.match("iabca exit=0"))

    def test_failed_replay_validation_precedes_any_reference_access(self):
        baseline = Path(__file__).resolve().parents[1] / "parity/evidence/voxconverse-natural-conversations.json"
        with patch.object(replay, "load_replay", side_effect=ValueError("failed inference")), \
                patch.object(replay, "reference_turns") as reference, \
                patch.object(replay.subprocess, "check_output") as revision:
            with self.assertRaisesRegex(ValueError, "failed inference"):
                replay.evaluate(Path("unused"), Path("unused"), Path("unopened"), baseline, "cuda")
            reference.assert_not_called()
            revision.assert_not_called()


if __name__ == "__main__":
    unittest.main()

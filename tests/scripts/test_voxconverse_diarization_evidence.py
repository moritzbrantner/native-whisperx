"""Trust boundaries for resource-tier diarization acceptance evidence."""
import importlib.util
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

SCRIPTS = Path(__file__).resolve().parents[2] / "scripts"
sys.path.insert(0, str(SCRIPTS))
SPEC = importlib.util.spec_from_file_location("vox_evidence", SCRIPTS / "voxconverse_diarization_evidence.py")
evidence = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(evidence)


class ReferenceTests(unittest.TestCase):
    def test_preserves_overlap_and_anonymous_participant_labels(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "abcde.rttm"
            path.write_text("SPEAKER abcde 1 1.0 2.0 <NA> <NA> speaker00 <NA> <NA>\n"
                            "SPEAKER abcde 1 2.0 3.0 <NA> <NA> speaker01 <NA> <NA>\n")
            self.assertEqual(evidence.reference_turns(path, "abcde"), [
                {"speaker": "speaker00", "startSeconds": 1.0, "endSeconds": 3.0},
                {"speaker": "speaker01", "startSeconds": 2.0, "endSeconds": 5.0},
            ])

    def test_rejects_invalid_timing_and_wrong_recording(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "abcde.rttm"
            for timing, ident in [("nan 1", "abcde"), ("0 -1", "abcde"), ("0 1", "other")]:
                path.write_text(f"SPEAKER {ident} 1 {timing} <NA> <NA> speaker00 <NA> <NA>\n")
                with self.assertRaises(ValueError):
                    evidence.reference_turns(path, "abcde")

    def test_failed_freeze_validation_precedes_any_reference_access(self):
        with patch.object(evidence, "load_frozen", side_effect=ValueError("changed predictions")), \
                patch.object(evidence, "reference_turns") as reference, \
                patch.object(evidence.subprocess, "check_output") as revision:
            with self.assertRaisesRegex(ValueError, "changed predictions"):
                evidence.evaluate(Path("unused"), Path("unopened-annotations"))
            reference.assert_not_called()
            revision.assert_not_called()

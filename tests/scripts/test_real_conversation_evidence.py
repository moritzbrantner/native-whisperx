"""Offline trust-boundary checks for the resource-only AMI evidence evaluator."""

import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
import zipfile

MODULE = Path(__file__).resolve().parents[2] / "scripts" / "real_conversation_evidence.py"
SPEC = importlib.util.spec_from_file_location("real_conversation_evidence", MODULE)
evidence = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(evidence)


class HumanReferenceTests(unittest.TestCase):
    def archive(self):
        output = io.BytesIO()
        with zipfile.ZipFile(output, "w") as archive:
            archive.writestr("words/TEST.A.words.xml", '<nite:root xmlns:nite="http://nite.sourceforge.net/"><w starttime="9" endtime="11">a</w><w starttime="11" endtime="11" punc="true">.</w><w starttime="12" endtime="13">b</w></nite:root>')
            archive.writestr("words/TEST.B.words.xml", '<root><w starttime="10.5" endtime="12">c</w><w>untimed</w></root>')
            archive.writestr("words/TEST.C.words.xml", '<root><w starttime="20" endtime="21">outside</w></root>')
            archive.writestr("segments/TEST.A.segments.xml", '<root><segment transcriber_start="9" transcriber_end="13"/></root>')
        output.seek(0)
        return zipfile.ZipFile(output)

    def test_reference_clipping_preserves_overlap_and_excludes_punctuation_and_outside_speakers(self):
        with self.archive() as archive:
            turns, unknown = evidence.reference_turns(archive, "TEST", 10, 12, "words")
        self.assertEqual(turns, [
            {"speaker": "A", "startSeconds": 0.0, "endSeconds": 1.0},
            {"speaker": "B", "startSeconds": 0.5, "endSeconds": 2.0},
        ])
        self.assertEqual(unknown, 1)
        self.assertEqual(len({turn["speaker"] for turn in turns}), 2)

    def test_utterance_times_use_human_transcriber_boundaries(self):
        with self.archive() as archive:
            turns, unknown = evidence.reference_turns(archive, "TEST", 10, 12, "utterances")
        self.assertEqual(turns, [{"speaker": "A", "startSeconds": 0.0, "endSeconds": 2.0}])
        self.assertEqual(unknown, 0)

    def test_missing_human_annotations_fail_instead_of_becoming_a_zero_speaker_reference(self):
        with self.archive() as archive:
            with self.assertRaisesRegex(ValueError, "no words annotations"):
                evidence.reference_turns(archive, "MISSING", 0, 2, "words")

    def test_scoring_requires_both_prediction_freezes_before_opening_annotations(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "cohort-before-prediction.json").write_text(json.dumps({"items": []}))
            (root / "observed-predictions-frozen.json").write_text("{}")
            with self.assertRaises(FileNotFoundError) as context:
                evidence.load_frozen(root)
            self.assertIn("oracle-predictions-frozen.json", str(context.exception))


if __name__ == "__main__":
    unittest.main()

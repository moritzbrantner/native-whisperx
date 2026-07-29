#!/usr/bin/env python3
"""Resource-free contract tests for the community bundle converter."""

from __future__ import annotations

import importlib.util
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

CONVERTER_PATH = Path(__file__).with_name("convert_pyannote_community.py")
SPEC = importlib.util.spec_from_file_location("convert_pyannote_community", CONVERTER_PATH)
assert SPEC is not None and SPEC.loader is not None
converter = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(converter)


class CommunityBundleConverterTests(unittest.TestCase):
    def test_cli_requires_local_two_speaker_evidence_and_has_no_token_argument(self):
        result = subprocess.run(
            [
                sys.executable,
                str(Path(__file__).with_name("convert_pyannote_community.py")),
                "--help",
            ],
            check=True,
            capture_output=True,
            text=True,
        )

        self.assertIn("--two-speaker-fixture", result.stdout)
        self.assertNotIn("--token", result.stdout)

    def test_source_validation_rejects_partial_and_corrupt_snapshots(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory)
            with self.assertRaisesRegex(RuntimeError, "canonical source file is missing"):
                converter.validate_source(source)

            for name in converter.SOURCE_HASHES:
                path = source / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(b"not the pinned source")
            with self.assertRaisesRegex(RuntimeError, "source checksum mismatch"):
                converter.validate_source(source)

    def test_artifact_address_is_canonical_and_order_independent(self):
        left = converter.artifact_set_digest(
            {"embedding.onnx": "b", "segmentation.onnx": "a"}
        )
        right = converter.artifact_set_digest(
            {"segmentation.onnx": "a", "embedding.onnx": "b"}
        )

        self.assertEqual(left, right)
        self.assertEqual(len(left), 64)

    def test_vbx_configuration_matches_the_pinned_pipeline(self):
        self.assertEqual(
            converter.CLUSTERING,
            {
                "kind": "vbx",
                "threshold": 0.6,
                "fa": 0.07,
                "fb": 0.8,
                "maxIters": 20,
                "minActiveRatio": 0.2,
                "constrainedAssignment": True,
            },
        )


if __name__ == "__main__":
    unittest.main()

import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]
CHECKER_PATH = ROOT / "scripts/check-product-boundary.py"

spec = importlib.util.spec_from_file_location("check_product_boundary", CHECKER_PATH)
checker = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(checker)


class ProductBoundaryTests(unittest.TestCase):
    def setUp(self) -> None:
        self.boundary = checker.load_boundary()
        self.manifest = checker.load_manifest()

    def test_repository_boundary_matches_current_manifest(self) -> None:
        self.assertEqual(checker.validate(self.boundary, self.manifest), [])

    def test_rejects_moving_audio_decode_back_into_product(self) -> None:
        manifest = json.loads(json.dumps(self.manifest))
        manifest["dependencies"]["symphonia"] = "0.5"
        errors = checker.validate(self.boundary, manifest)
        self.assertTrue(
            any("reusable audio implementation dependencies" in error for error in errors),
            errors,
        )

    def test_rejects_reassigning_canonical_owner(self) -> None:
        boundary = json.loads(json.dumps(self.boundary))
        boundary["excludedAuthorities"][0]["ownerRepository"] = "moritzbrantner/native-whisperx"
        errors = checker.validate(boundary, self.manifest)
        self.assertTrue(any("must remain owned by" in error for error in errors), errors)

    def test_rejects_untracked_model_implementation_dependency(self) -> None:
        manifest = json.loads(json.dumps(self.manifest))
        manifest["dependencies"]["candle-example-provider"] = "0.1"
        errors = checker.validate(self.boundary, manifest)
        self.assertTrue(
            any("explicit ownership exception" in error for error in errors),
            errors,
        )


if __name__ == "__main__":
    unittest.main()

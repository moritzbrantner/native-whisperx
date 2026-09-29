import copy
import importlib.util
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
        self.workspace_manifest = checker.load_workspace_manifest()
        self.product_manifests = checker.load_product_manifests()

    def validate(self, *, boundary=None, manifests=None):
        return checker.validate(
            boundary if boundary is not None else self.boundary,
            self.workspace_manifest,
            manifests if manifests is not None else self.product_manifests,
        )

    def test_repository_boundary_matches_current_manifests(self) -> None:
        self.assertEqual(self.validate(), [])

    def test_rejects_moving_audio_decode_back_into_library(self) -> None:
        manifests = copy.deepcopy(self.product_manifests)
        manifests["native-whisperx"]["dependencies"]["symphonia"] = "0.5"
        errors = self.validate(manifests=manifests)
        self.assertTrue(
            any("unclassified direct dependencies" in error and "symphonia" in error for error in errors),
            errors,
        )

    def test_rejects_moving_audio_decode_into_cli(self) -> None:
        manifests = copy.deepcopy(self.product_manifests)
        manifests["native-whisperx-cli"]["dependencies"]["decoder"] = {
            "package": "symphonia",
            "version": "0.5",
        }
        errors = self.validate(manifests=manifests)
        self.assertTrue(
            any("native-whisperx-cli" in error and "symphonia" in error for error in errors),
            errors,
        )

    def test_rejects_target_specific_renamed_implementation_dependency(self) -> None:
        manifests = copy.deepcopy(self.product_manifests)
        manifests["native-whisperx"]["target"] = {
            "cfg(unix)": {
                "dependencies": {
                    "decoder": {
                        "package": "symphonia",
                        "version": "0.5",
                    }
                }
            }
        }
        errors = self.validate(manifests=manifests)
        self.assertTrue(
            any("symphonia" in error for error in errors),
            errors,
        )

    def test_rejects_repointing_required_upstream_alias(self) -> None:
        workspace = copy.deepcopy(self.workspace_manifest)
        workspace["workspace"]["dependencies"]["audio-analysis-io"] = {
            "package": "not-the-audio-io-owner",
            "version": "1",
        }
        errors = checker.validate(self.boundary, workspace, self.product_manifests)
        self.assertTrue(
            any("moenarch-audio-analysis-io" in error for error in errors),
            errors,
        )

    def test_rejects_reassigning_canonical_owner(self) -> None:
        boundary = copy.deepcopy(self.boundary)
        boundary["excludedAuthorities"][0]["ownerRepository"] = "moritzbrantner/native-whisperx"
        errors = self.validate(boundary=boundary)
        self.assertTrue(any("must remain owned by" in error for error in errors), errors)

    def test_rejects_capability_claimed_as_both_owned_and_excluded(self) -> None:
        boundary = copy.deepcopy(self.boundary)
        boundary["ownedCapabilities"].append(
            "generic-media-probe-track-selection-and-decode"
        )
        boundary["ownedCapabilities"].sort()
        errors = self.validate(boundary=boundary)
        self.assertTrue(
            any(
                "unexpected owned capabilities" in error
                or "both owned and excluded" in error
                for error in errors
            ),
            errors,
        )

    def test_rejects_untracked_model_implementation_dependency(self) -> None:
        manifests = copy.deepcopy(self.product_manifests)
        manifests["native-whisperx"]["dependencies"]["candle-example-provider"] = "0.1"
        errors = self.validate(manifests=manifests)
        self.assertTrue(
            any("unclassified direct dependencies" in error for error in errors),
            errors,
        )


if __name__ == "__main__":
    unittest.main()

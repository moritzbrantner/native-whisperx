#!/usr/bin/env python3
"""Validate Native WhisperX's composition-only repository boundary."""

from __future__ import annotations

import json
from pathlib import Path
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[1]
BOUNDARY_PATH = ROOT / "docs/ownership/native-whisperx-boundary.json"
MANIFEST_PATH = ROOT / "crates/native-whisperx/Cargo.toml"

EXPECTED_OWNED_CAPABILITIES = {
    "automatic-workflow-selection",
    "output-placement-and-product-projections",
    "parity-and-compatibility-evidence",
    "product-configuration-and-policy",
    "product-progress-reports-errors",
    "speaker-directory-and-trace-workflows",
    "workflow-composition",
}
EXPECTED_EXCLUDED_AUTHORITIES = {
    "generic-media-probe-track-selection-and-decode": (
        "moritzbrantner/audio-analysis",
        "moenarch-audio-analysis-io",
    ),
    "generic-asr-vad-alignment-diarization-and-transcription-sessions": (
        "moritzbrantner/audio-analysis",
        "moenarch-audio-analysis-transcription",
    ),
    "generic-speaker-identity-embeddings-and-library-lifecycle": (
        "moritzbrantner/audio-analysis",
        "moenarch-audio-analysis-speakers",
    ),
    "neutral-timed-text-contracts-and-format-only-rendering": (
        "moritzbrantner/moenarch-foundation",
        "moenarch-media-core",
    ),
    "generic-cancellation-and-model-resource-mechanics": (
        "moritzbrantner/moenarch-foundation",
        "moenarch-runtime-core",
    ),
}
REQUIRED_UPSTREAM_DEPENDENCIES = {
    "audio-analysis-io",
    "audio-analysis-speakers",
    "audio-analysis-transcription",
    "media-core",
    "runtime-core",
}
TRANSLATION_EXCEPTION_DEPENDENCIES = {
    "candle-core",
    "candle-nn",
    "candle-transformers",
    "model-runtime",
    "sentencepiece-rs",
    "text-model-runtime",
}
FORBIDDEN_REUSABLE_AUDIO_IMPLEMENTATION_DEPENDENCIES = {
    "ffmpeg-next",
    "hound",
    "ort",
    "rubato",
    "rustfft",
    "symphonia",
}


def load_boundary(path: Path = BOUNDARY_PATH) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def load_manifest(path: Path = MANIFEST_PATH) -> dict:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def validate(boundary: dict, manifest: dict) -> list[str]:
    errors: list[str] = []

    if boundary.get("schemaVersion") != 1:
        errors.append("ownership boundary schemaVersion must be 1")
    if boundary.get("repository") != "moritzbrantner/native-whisperx":
        errors.append("ownership boundary must identify moritzbrantner/native-whisperx")
    if boundary.get("layer") != "application":
        errors.append("native-whisperx must remain an application/composition layer")
    if not isinstance(boundary.get("purpose"), str) or not boundary["purpose"].strip():
        errors.append("ownership boundary must declare a non-empty purpose")

    owned = boundary.get("ownedCapabilities")
    if (
        not isinstance(owned, list)
        or any(not isinstance(item, str) or not item for item in owned)
        or owned != sorted(set(owned))
    ):
        errors.append("ownedCapabilities must be a sorted list of unique non-empty strings")
        owned_set: set[str] = set()
    else:
        owned_set = set(owned)
    missing_owned = EXPECTED_OWNED_CAPABILITIES - owned_set
    if missing_owned:
        errors.append("missing owned capabilities: " + ", ".join(sorted(missing_owned)))

    excluded = boundary.get("excludedAuthorities")
    excluded_by_authority: dict[str, dict] = {}
    if not isinstance(excluded, list):
        errors.append("excludedAuthorities must be a list")
        excluded = []
    for record in excluded:
        if not isinstance(record, dict) or not isinstance(record.get("authority"), str):
            errors.append("excludedAuthorities contains an invalid record")
            continue
        authority = record["authority"]
        if authority in excluded_by_authority:
            errors.append(f"excluded authority {authority} must be declared exactly once")
            continue
        excluded_by_authority[authority] = record

    for authority, (owner_repository, owner_package) in EXPECTED_EXCLUDED_AUTHORITIES.items():
        record = excluded_by_authority.get(authority)
        if record is None:
            errors.append(f"missing excluded authority: {authority}")
            continue
        if record.get("ownerRepository") != owner_repository:
            errors.append(f"{authority} must remain owned by {owner_repository}")
        if record.get("ownerPackage") != owner_package:
            errors.append(f"{authority} must remain implemented by {owner_package}")

    exceptions = boundary.get("transitionalExceptions")
    if not isinstance(exceptions, list) or len(exceptions) != 1:
        errors.append("translation execution must have exactly one explicit transitional exception")
        allowed_translation_dependencies: set[str] = set()
    else:
        exception = exceptions[0]
        if (
            not isinstance(exception, dict)
            or exception.get("authority") != "marian-opus-mt-translation-execution"
            or exception.get("trackingIssue") != 254
            or exception.get("status") != "owner-unresolved"
        ):
            errors.append("translation execution exception must remain tied to issue #254")
            allowed_translation_dependencies = set()
        else:
            allowed = exception.get("allowedDirectDependencies")
            if (
                not isinstance(allowed, list)
                or any(not isinstance(item, str) or not item for item in allowed)
                or allowed != sorted(set(allowed))
            ):
                errors.append(
                    "translation exception dependencies must be a sorted list of unique strings"
                )
                allowed_translation_dependencies = set()
            else:
                allowed_translation_dependencies = set(allowed)
                if allowed_translation_dependencies != TRANSLATION_EXCEPTION_DEPENDENCIES:
                    errors.append(
                        "translation exception dependency set drifted from the accepted #254 boundary"
                    )

    dependencies = manifest.get("dependencies", {})
    dependency_names = set(dependencies)
    missing_upstream = REQUIRED_UPSTREAM_DEPENDENCIES - dependency_names
    if missing_upstream:
        errors.append(
            "composition boundary is missing canonical upstream dependencies: "
            + ", ".join(sorted(missing_upstream))
        )

    forbidden = dependency_names & FORBIDDEN_REUSABLE_AUDIO_IMPLEMENTATION_DEPENDENCIES
    if forbidden:
        errors.append(
            "reusable audio implementation dependencies must stay upstream: "
            + ", ".join(sorted(forbidden))
        )

    direct_model_implementation_dependencies = {
        name
        for name in dependency_names
        if name.startswith("candle-")
        or name in {"model-runtime", "sentencepiece-rs", "text-model-runtime"}
    }
    unexpected_model_dependencies = (
        direct_model_implementation_dependencies - allowed_translation_dependencies
    )
    if unexpected_model_dependencies:
        errors.append(
            "direct model implementation dependencies need an explicit ownership exception: "
            + ", ".join(sorted(unexpected_model_dependencies))
        )

    return errors


def main() -> int:
    errors = validate(load_boundary(), load_manifest())
    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1
    print("native-whisperx composition-only ownership boundary valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Validate Native WhisperX's composition-only repository boundary."""

from __future__ import annotations

import json
from pathlib import Path
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[1]
BOUNDARY_PATH = ROOT / "docs/ownership/native-whisperx-boundary.json"
WORKSPACE_MANIFEST_PATH = ROOT / "Cargo.toml"
PRODUCT_MANIFEST_PATHS = {
    "native-whisperx": ROOT / "crates/native-whisperx/Cargo.toml",
    "native-whisperx-cli": ROOT / "crates/native-whisperx-cli/Cargo.toml",
}

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
REQUIRED_UPSTREAM_PACKAGES = {
    "moenarch-audio-analysis-io",
    "moenarch-audio-analysis-speakers",
    "moenarch-audio-analysis-transcription",
    "moenarch-media-core",
    "moenarch-runtime-core",
}
TRANSLATION_EXCEPTION_DEPENDENCIES = {
    "candle-core",
    "candle-nn",
    "candle-transformers",
    "model-runtime",
    "sentencepiece-rs",
    "text-model-runtime",
}
EXPECTED_DIRECT_PACKAGES = {
    "native-whisperx": {
        "candle-core",
        "candle-nn",
        "candle-transformers",
        "dirs",
        "moenarch-audio-analysis-io",
        "moenarch-audio-analysis-speakers",
        "moenarch-audio-analysis-transcription",
        "moenarch-media-core",
        "moenarch-model-runtime",
        "moenarch-runtime-core",
        "moenarch-text-model-runtime",
        "sentencepiece-rs",
        "serde",
        "serde_json",
        "sha2",
        "tempfile",
        "thiserror",
    },
    "native-whisperx-cli": {
        "anyhow",
        "assert_cmd",
        "clap",
        "glob",
        "indicatif",
        "native-whisperx",
        "predicates",
        "serde_json",
        "tempfile",
    },
}


def load_boundary(path: Path = BOUNDARY_PATH) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def load_manifest(path: Path) -> dict:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def load_workspace_manifest() -> dict:
    return load_manifest(WORKSPACE_MANIFEST_PATH)


def load_product_manifests() -> dict[str, dict]:
    return {name: load_manifest(path) for name, path in PRODUCT_MANIFEST_PATHS.items()}


def dependency_tables(manifest: dict):
    for name in ("dependencies", "dev-dependencies", "build-dependencies"):
        table = manifest.get(name, {})
        if isinstance(table, dict):
            yield table
    targets = manifest.get("target", {})
    if isinstance(targets, dict):
        for target in targets.values():
            if not isinstance(target, dict):
                continue
            for name in ("dependencies", "dev-dependencies", "build-dependencies"):
                table = target.get(name, {})
                if isinstance(table, dict):
                    yield table


def resolve_package_name(alias: str, spec: object, workspace_dependencies: dict) -> str:
    resolved = spec
    if isinstance(spec, dict) and spec.get("workspace") is True:
        resolved = workspace_dependencies.get(alias)
        if resolved is None:
            raise ValueError(f"workspace dependency {alias} is not declared at the workspace root")
    if isinstance(resolved, dict):
        package = resolved.get("package", alias)
        if not isinstance(package, str) or not package:
            raise ValueError(f"dependency {alias} has an invalid package identity")
        return package
    return alias


def direct_packages(manifest: dict, workspace_dependencies: dict) -> tuple[set[str], list[str]]:
    packages: set[str] = set()
    errors: list[str] = []
    for table in dependency_tables(manifest):
        for alias, spec in table.items():
            try:
                packages.add(resolve_package_name(alias, spec, workspace_dependencies))
            except ValueError as error:
                errors.append(str(error))
    return packages, errors


def validate(boundary: dict, workspace_manifest: dict, product_manifests: dict[str, dict]) -> list[str]:
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
    if owned_set != EXPECTED_OWNED_CAPABILITIES:
        missing = EXPECTED_OWNED_CAPABILITIES - owned_set
        unexpected = owned_set - EXPECTED_OWNED_CAPABILITIES
        if missing:
            errors.append("missing owned capabilities: " + ", ".join(sorted(missing)))
        if unexpected:
            errors.append("unexpected owned capabilities: " + ", ".join(sorted(unexpected)))

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

    excluded_set = set(excluded_by_authority)
    expected_excluded_set = set(EXPECTED_EXCLUDED_AUTHORITIES)
    unexpected_excluded = excluded_set - expected_excluded_set
    if unexpected_excluded:
        errors.append(
            "unexpected excluded authorities: " + ", ".join(sorted(unexpected_excluded))
        )
    contradictory = owned_set & excluded_set
    if contradictory:
        errors.append(
            "capabilities cannot be both owned and excluded: "
            + ", ".join(sorted(contradictory))
        )

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
    else:
        exception = exceptions[0]
        if (
            not isinstance(exception, dict)
            or exception.get("authority") != "marian-opus-mt-translation-execution"
            or exception.get("trackingIssue") != 254
            or exception.get("status") != "owner-unresolved"
        ):
            errors.append("translation execution exception must remain tied to issue #254")
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
            elif set(allowed) != TRANSLATION_EXCEPTION_DEPENDENCIES:
                errors.append(
                    "translation exception dependency set drifted from the accepted #254 boundary"
                )

    workspace = workspace_manifest.get("workspace", {})
    workspace_dependencies = workspace.get("dependencies", {})
    if not isinstance(workspace_dependencies, dict):
        errors.append("workspace dependencies must be a Cargo dependency table")
        workspace_dependencies = {}

    all_product_packages: set[str] = set()
    if set(product_manifests) != set(EXPECTED_DIRECT_PACKAGES):
        errors.append("product manifest set must contain exactly the library and CLI crates")
    for product_name, expected_packages in EXPECTED_DIRECT_PACKAGES.items():
        manifest = product_manifests.get(product_name)
        if not isinstance(manifest, dict):
            errors.append(f"missing product manifest: {product_name}")
            continue
        packages, resolution_errors = direct_packages(manifest, workspace_dependencies)
        errors.extend(f"{product_name}: {error}" for error in resolution_errors)
        all_product_packages |= packages

        missing = expected_packages - packages
        unexpected = packages - expected_packages
        if missing:
            errors.append(
                f"{product_name} is missing classified direct dependencies: "
                + ", ".join(sorted(missing))
            )
        if unexpected:
            errors.append(
                f"{product_name} has unclassified direct dependencies: "
                + ", ".join(sorted(unexpected))
            )

    missing_upstream = REQUIRED_UPSTREAM_PACKAGES - all_product_packages
    if missing_upstream:
        errors.append(
            "composition boundary is missing canonical upstream packages: "
            + ", ".join(sorted(missing_upstream))
        )

    return errors


def main() -> int:
    errors = validate(load_boundary(), load_workspace_manifest(), load_product_manifests())
    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1
    print("native-whisperx composition-only ownership boundary valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Validate the release-facing Cargo metadata for native-whisperx 0.1.14."""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
RELEASE_VERSION = "0.1.14"
TRANSCRIPTION_VERSION = "0.1.12"


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def uses_workspace_version(manifest: dict[str, Any]) -> bool:
    version = manifest["package"].get("version")
    return isinstance(version, dict) and version.get("workspace") is True


def validate() -> None:
    workspace = load_toml(ROOT / "Cargo.toml")
    library = load_toml(ROOT / "crates/native-whisperx/Cargo.toml")
    cli = load_toml(ROOT / "crates/native-whisperx-cli/Cargo.toml")

    workspace_version = workspace["workspace"]["package"]["version"]
    require(
        workspace_version == RELEASE_VERSION,
        f"workspace version must be {RELEASE_VERSION}, found {workspace_version!r}",
    )
    require(uses_workspace_version(library), "native-whisperx must use workspace version")
    require(uses_workspace_version(cli), "native-whisperx-cli must use workspace version")

    cli_library = cli["dependencies"]["native-whisperx"]
    require(
        cli_library.get("version") == f"={RELEASE_VERSION}",
        f"CLI native-whisperx dependency must be exactly ={RELEASE_VERSION}",
    )
    require(
        cli_library.get("default-features") is False,
        "CLI native-whisperx dependency must disable default features",
    )

    for package_name, manifest in (("native-whisperx", library), ("native-whisperx-cli", cli)):
        require(
            "translation" in manifest["features"]["default"],
            f"{package_name} default features must contain translation",
        )
        require(
            "translation" in manifest["package"]["metadata"]["docs"]["rs"]["features"],
            f"{package_name} docs.rs features must contain translation",
        )

    transcription = workspace["workspace"]["dependencies"][
        "audio-analysis-transcription"
    ]
    require(
        transcription.get("package") == "moenarch-audio-analysis-transcription",
        "transcription dependency must use the canonical moenarch package",
    )
    require(
        transcription.get("version") == TRANSCRIPTION_VERSION,
        f"transcription dependency must require registry version {TRANSCRIPTION_VERSION}",
    )
    require("path" not in transcription, "transcription dependency must not contain a path")

    members = workspace["workspace"]["members"]
    require(
        "vendor/moenarch-audio-analysis-transcription" not in members,
        "vendored transcription crate must not be a workspace member",
    )


def main() -> int:
    try:
        validate()
    except (KeyError, TypeError, ValueError) as error:
        print(f"release metadata check failed: {error}", file=sys.stderr)
        return 1

    print(
        "release metadata check passed: native-whisperx 0.1.14 uses "
        "moenarch-audio-analysis-transcription 0.1.12 from crates.io"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

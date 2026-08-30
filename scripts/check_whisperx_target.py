#!/usr/bin/env python3
"""Report the latest WhisperX target against the verified compatibility baseline.

Normal parity gates stay pinned to the verified baseline. This script is deliberately
non-gating reconnaissance: a newer upstream release reports `behind upstream`
without mutating fixtures, goldens, or dependency versions.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_POLICY = ROOT / "tests" / "parity" / "whisperx-version.json"
PYPI_URL = "https://pypi.org/pypi/{package}/json"


class ReconnaissanceError(RuntimeError):
    pass


def parse_release_version(value: str) -> tuple[int, ...]:
    """Parse the numeric release portion used for stable WhisperX releases."""
    match = re.fullmatch(r"\s*(\d+(?:\.\d+)*)\s*", value)
    if not match:
        raise ValueError(f"unsupported release version: {value!r}")
    return tuple(int(part) for part in match.group(1).split("."))


def compare_versions(left: str, right: str) -> int:
    left_parts = list(parse_release_version(left))
    right_parts = list(parse_release_version(right))
    width = max(len(left_parts), len(right_parts))
    left_parts.extend([0] * (width - len(left_parts)))
    right_parts.extend([0] * (width - len(right_parts)))
    return (left_parts > right_parts) - (left_parts < right_parts)


def load_policy(path: Path) -> dict[str, Any]:
    try:
        policy = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ReconnaissanceError(f"could not read compatibility policy {path}: {error}") from error

    if policy.get("schemaVersion") != 1:
        raise ReconnaissanceError("compatibility policy must declare schemaVersion 1")
    baseline = policy.get("verifiedCompatibilityBaseline")
    package = policy.get("upstreamPackage")
    if not isinstance(baseline, str) or not isinstance(package, str):
        raise ReconnaissanceError(
            "compatibility policy must contain verifiedCompatibilityBaseline and upstreamPackage strings"
        )
    parse_release_version(baseline)
    return policy


def discover_latest_version(package: str) -> str:
    request = urllib.request.Request(
        PYPI_URL.format(package=package),
        headers={"User-Agent": "native-whisperx-upstream-reconnaissance/1"},
    )
    try:
        with urllib.request.urlopen(request, timeout=15) as response:
            payload = json.load(response)
    except (OSError, urllib.error.URLError, json.JSONDecodeError) as error:
        raise ReconnaissanceError(f"could not discover latest {package} release: {error}") from error

    latest = payload.get("info", {}).get("version")
    if not isinstance(latest, str):
        raise ReconnaissanceError(f"PyPI response for {package} did not contain info.version")
    parse_release_version(latest)
    return latest


def build_report(policy: dict[str, Any], latest_version: str) -> dict[str, Any]:
    baseline = policy["verifiedCompatibilityBaseline"]
    comparison = compare_versions(baseline, latest_version)
    if comparison < 0:
        status = "behind upstream"
    elif comparison == 0:
        status = "current"
    else:
        status = "verified baseline ahead of upstream target"

    return {
        "schemaVersion": 1,
        "upstreamPackage": policy["upstreamPackage"],
        "upstreamTarget": latest_version,
        "verifiedCompatibilityBaseline": baseline,
        "status": status,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--policy", type=Path, default=DEFAULT_POLICY)
    parser.add_argument(
        "--latest-version",
        help="Use an explicit version instead of network discovery (for tests/manual inspection).",
    )
    args = parser.parse_args(argv)

    try:
        policy = load_policy(args.policy)
        latest = args.latest_version or discover_latest_version(policy["upstreamPackage"])
        report = build_report(policy, latest)
    except (ReconnaissanceError, ValueError) as error:
        print(json.dumps({"status": "reconnaissance failure", "error": str(error)}))
        return 2

    print(json.dumps(report, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())

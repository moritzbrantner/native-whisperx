#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SITE = ROOT / "site"
INDEX = SITE / "index.html"
WORKBENCH = SITE / "workbench.html"
WORKBENCH_JS = SITE / "workbench.js"
WORKBENCH_CSS = SITE / "workbench.css"
SITE_CSS = SITE / "assets" / "site.css"
TRANSCRIBE = SITE / "transcribe" / "index.html"
PAGES_WORKFLOW = ROOT / ".github" / "workflows" / "pages.yml"


class SiteCheckError(Exception):
    pass


def read(path: Path) -> str:
    if not path.is_file():
        raise SiteCheckError(f"missing required file: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def require(text: str, values: tuple[str, ...], owner: str) -> None:
    for value in values:
        if value not in text:
            raise SiteCheckError(f"{owner} missing expected contract marker: {value}")


def main() -> int:
    try:
        index = read(INDEX)
        workbench = read(WORKBENCH)
        workbench_js = read(WORKBENCH_JS)
        read(WORKBENCH_CSS)
        read(SITE_CSS)
        transcribe = read(TRANSCRIBE)
        pages = read(PAGES_WORKFLOW)

        require(
            index,
            (
                "Transcribe. Align. Diarize. Translate.",
                "workbench.html",
                "Browser WebGPU preview",
                "Installed native-whisperx",
                "Alignment",
                "Diarization",
                "Translation",
                "No silent approximation",
            ),
            "site/index.html",
        )
        require(
            workbench,
            (
                'id="browser-preview"',
                'id="native-workflow"',
                "Transcription",
                "Alignment",
                "Diarization",
                "Translation",
                "Not executed in browser preview",
                "Generated native command",
                'src="workbench.js"',
                'href="workbench.css"',
            ),
            "site/workbench.html",
        )
        require(
            workbench_js,
            (
                'const MODEL_ID = "onnx-community/whisper-tiny"',
                'device: "webgpu"',
                'task,',
                '"--no-align"',
                '"--return-char-alignments"',
                '"--diarize"',
                '"--min-speakers"',
                '"--max-speakers"',
                '"--translation-model"',
                '"--translation-source-language"',
                '"--translation-target-language"',
                '"--format"',
                'alignment: "not-run-in-browser-preview"',
                'diarization: "not-run-in-browser-preview"',
            ),
            "site/workbench.js",
        )
        require(transcribe, ("../workbench.html#browser-preview",), "site/transcribe/index.html")
        require(
            pages,
            (
                "python3 scripts/check-site.py",
                "node --check site/workbench.js",
                "actions/upload-pages-artifact@v3",
                "path: site",
            ),
            ".github/workflows/pages.yml",
        )

        if not re.search(r"<main\b", index) or not re.search(r"<main\b", workbench):
            raise SiteCheckError("site pages must contain a main landmark")
        if "alignment runs in browser" in workbench.lower() or "diarization runs in browser" in workbench.lower():
            raise SiteCheckError("workbench must not claim browser-native alignment or diarization")
    except SiteCheckError as error:
        print(f"site check failed: {error}", file=sys.stderr)
        return 1

    print("site check ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

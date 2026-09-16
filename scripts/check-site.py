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
ACCEPTANCE = SITE / "acceptance" / "index.html"
ACCEPTANCE_JS = SITE / "acceptance" / "acceptance.js"
VENDORED_TRANSCRIPTION = SITE / "vendor" / "audio-analysis-transcription.js"
PREPARE_SITE = ROOT / "scripts" / "prepare-site.sh"
PAGES_WORKFLOW = ROOT / ".github" / "workflows" / "pages.yml"
SITE_WORKFLOW = ROOT / ".github" / "workflows" / "site.yml"


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


def reject(text: str, values: tuple[str, ...], owner: str) -> None:
    for value in values:
        if value in text:
            raise SiteCheckError(f"{owner} contains forbidden ownership marker: {value}")


def main() -> int:
    try:
        index = read(INDEX)
        workbench = read(WORKBENCH)
        workbench_js = read(WORKBENCH_JS)
        read(WORKBENCH_CSS)
        read(SITE_CSS)
        transcribe = read(TRANSCRIBE)
        acceptance = read(ACCEPTANCE)
        acceptance_js = read(ACCEPTANCE_JS)
        vendored_transcription = read(VENDORED_TRANSCRIPTION)
        prepare_site = read(PREPARE_SITE)
        pages = read(PAGES_WORKFLOW)
        site_workflow = read(SITE_WORKFLOW)

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
                "audio-analysis",
                'src="workbench.js"',
                'href="workbench.css"',
            ),
            "site/workbench.html",
        )
        reject(
            workbench,
            (
                'name="browser-task"',
                "Translate speech to English",
                "Transformers.js on WebGPU",
            ),
            "site/workbench.html",
        )
        require(
            workbench_js,
            (
                'from "./vendor/audio-analysis-transcription.js"',
                "browserTranscriptionCapabilities",
                "supportsBrowserTranscription",
                "transcribeAudioBlob",
                "function handleBrowserProgress(update) {\n  throwIfCancelled();",
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
                'translation: "not-run-in-browser-preview"',
            ),
            "site/workbench.js",
        )
        reject(
            workbench_js,
            (
                "@huggingface/transformers",
                "onnx-community/whisper-tiny",
                "pipeline(\"automatic-speech-recognition\"",
                "new OfflineAudioContext",
            ),
            "site/workbench.js",
        )
        require(
            acceptance,
            (
                "Browser runtime acceptance",
                'id="capture"',
                'id="download"',
                'id="workbench"',
                'src="../transcribe/"',
                'src="./acceptance.js"',
                "real local audio file",
                "WebGPU ready",
                "Finished locally",
            ),
            "site/acceptance/index.html",
        )
        require(
            acceptance_js,
            (
                'webGpuCapability === "WebGPU ready"',
                'browserStatus.startsWith("Finished locally")',
                "transcriptLength: transcript.length",
                "timedSegmentsProduced: segmentCount > 0",
                'availableFormats.includes("native-json")',
                'availableFormats.includes("srt")',
                'availableFormats.includes("vtt")',
                "Object.values(checks).every(Boolean)",
                "native-whisperx-browser-acceptance-",
            ),
            "site/acceptance/acceptance.js",
        )
        reject(
            acceptance_js,
            (
                "transcribeAudioBlob(",
                "@huggingface/transformers",
                "pipeline(\"automatic-speech-recognition\"",
            ),
            "site/acceptance/acceptance.js",
        )
        require(
            vendored_transcription,
            (
                "export function browserTranscriptionCapabilities()",
                "export async function transcribeAudioBlob",
                'requiredAcceleration: "webgpu"',
                "translation: false",
                "server: false",
                "cpu: false",
            ),
            "site/vendor/audio-analysis-transcription.js",
        )
        require(
            prepare_site,
            (
                'AUDIO_ANALYSIS_REV="bf2cb13d155a874b166305da2f8dc669a05a2a58"',
                'SOURCE_PATH="packages/audio-analysis-transcription-wasm/index.js"',
            ),
            "scripts/prepare-site.sh",
        )
        require(transcribe, ("../workbench.html#browser-preview",), "site/transcribe/index.html")
        require(
            pages,
            (
                "bash scripts/prepare-site.sh",
                "python3 scripts/check-site.py",
                "node --check site/vendor/audio-analysis-transcription.js",
                "node --check site/workbench.js",
                "node --check site/acceptance/acceptance.js",
                "actions/upload-pages-artifact@",
                "path: site",
            ),
            ".github/workflows/pages.yml",
        )
        require(
            site_workflow,
            (
                "bash scripts/prepare-site.sh",
                "python3 scripts/check-site.py",
                "node --check site/vendor/audio-analysis-transcription.js",
                "node --check site/workbench.js",
                "node --check site/acceptance/acceptance.js",
            ),
            ".github/workflows/site.yml",
        )

        if not all(re.search(r"<main\b", page) for page in (index, workbench, acceptance)):
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

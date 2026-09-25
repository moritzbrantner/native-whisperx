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
VENDORED_TRANSLATION = SITE / "vendor" / "browser-translation.js"
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
        vendored_translation = read(VENDORED_TRANSLATION)
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
                'id="browser-model"',
                'id="browser-model-description"',
                'id="browser-transcription-stage"',
                'id="browser-translate"',
                'id="browser-translation-pair"',
                'id="translation-capability"',
                'id="source-transcript"',
                'aria-label="Download transcript"',
                "Download Native JSON",
                "Download TXT",
                "Download SRT",
                "Download WebVTT",
                'id="hf-token"',
                'type="password"',
                'id="clear-hf-token"',
                "Saved only in this browser on this device using local storage.",
                "is never inserted into the generated command",
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
                'from "./vendor/browser-translation.js"',
                "browserTranscriptionCapabilities",
                "browserTranscriptionModels",
                "supportsBrowserTranscription",
                "transcribeAudioBlob",
                "function selectedBrowserModel() {",
                "modelId: run.model.id,",
                "browserTranslationCapabilities",
                "supportsBrowserTranslation",
                "translateBrowserSegments",
                "let activeBrowserRun = null;",
                "activeBrowserRun.cancelRequested = true;",
                "activeBrowserRun !== null",
                "onProgress: (update) => handleBrowserProgress(run, update)",
                "function handleBrowserProgress(run, update) {",
                "run !== activeBrowserRun || run.cancelRequested",
                "function handleBrowserTranslationProgress(run, update) {",
                "throwIfCancelled(run);",
                'const HF_TOKEN_STORAGE_KEY = "native-whisperx:hf-token";',
                "window.localStorage.getItem(HF_TOKEN_STORAGE_KEY)",
                "window.localStorage.setItem(HF_TOKEN_STORAGE_KEY, token)",
                "window.localStorage.removeItem(HF_TOKEN_STORAGE_KEY)",
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
                'translation: "not-requested"',
                'translation: "completed"',
                "hasMatchingSegmentIdentityAndTiming",
                "sourceTranscriptRetainedInSession",
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
                "function handleBrowserProgress(update) {\n  throwIfCancelled();",
                "function handleBrowserTranslationProgress(update) {\n  throwIfCancelled();",
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
                "timedSegmentCount = segmentRows.filter(hasValidRenderedTiming).length",
                "timedSegmentsProduced: timedSegmentCount > 0",
                "endSeconds >= startSeconds",
                'availableFormats.includes("native-json")',
                'availableFormats.includes("srt")',
                'availableFormats.includes("vtt")',
                "Object.values(checks).every(Boolean)",
                "translationRequested",
                "translationCompleted",
                "translationTimingPreserved",
                "sourceTranscriptRetainedInSession",
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
                "export function browserTranscriptionModels()",
                "export async function transcribeAudioBlob",
                "function normalizeBrowserTranscriptText",
                'requiredAcceleration: "webgpu"',
                "translation: false",
                "server: false",
                "cpu: false",
            ),
            "site/vendor/audio-analysis-transcription.js",
        )
        require(
            vendored_translation,
            (
                "export {",
                "browserTranslationCapabilities",
                "supportsBrowserTranslation",
                "translateBrowserSegments",
                'requiredAcceleration: "webgpu"',
                'modelProvisioning: "browser-cache"',
                'server: false',
                'python: false',
                'cpu: false',
            ),
            "site/vendor/browser-translation.js",
        )
        require(
            prepare_site,
            (
                'AUDIO_ANALYSIS_REV="21cd5c84e5862be6dcf28bc18b8e97c21df865f2"',
                'SOURCE_PATH="packages/audio-analysis-transcription-wasm/index.js"',
                'PLATFORM_PACKAGES_REV="9eb1a19ba4b5bed3f02161682aa1a38abfb1f128"',
                'TRANSLATION_SOURCE_PATH="packages/browser-translation"',
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
                "node --check site/vendor/browser-translation.js",
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
                "node --check site/vendor/browser-translation.js",
                "node --check site/workbench.js",
                "node --check site/acceptance/acceptance.js",
            ),
            ".github/workflows/site.yml",
        )

        if '"--hf-token"' in workbench_js or '"--hf_token"' in workbench_js:
            raise SiteCheckError("stored HF token must not be serialized into the generated native command")
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

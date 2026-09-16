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
VENDORED_TRANSLATION = SITE / "vendor" / "platform-browser-translation.js"
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
                'id="browser-translate"',
                'id="browser-translation-model"',
                'id="browser-translation-source"',
                'id="browser-translation-target"',
                'id="browser-translation-status"',
                'id="source-transcript-wrap"',
                'id="browser-translation-model" type="text" value="onnx-community/opus-mt-de-en" readonly',
                "Optional post-ASR · platform-packages · WebGPU",
                "Not executed in browser preview",
                "Generated native command",
                "audio-analysis",
                "platform-packages",
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
                "reusable nlp-stack browser translation",
            ),
            "site/workbench.html",
        )
        require(
            workbench_js,
            (
                'from "./vendor/audio-analysis-transcription.js"',
                'from "./vendor/platform-browser-translation.js"',
                'const BROWSER_RUN_EVIDENCE_KEY = "__nativeWhisperxBrowserRunEvidence"',
                "browserTranscriptionCapabilities",
                "supportsBrowserTranscription",
                "transcribeAudioBlob",
                "browserTranslationCapabilities",
                "resolveBrowserTranslationPair",
                "supportsBrowserTranslation",
                "translateBrowserSegments",
                "function updateBrowserTranslationPair()",
                "function currentBrowserTranslationPair()",
                "function browserRunConfiguration()",
                "function setBrowserRunControlsDisabled(disabled)",
                "function publishCompletedBrowserRunEvidence(runConfig, file, sourceContract, contract, browserStatus)",
                "clearCompletedBrowserRunEvidence();",
                "runConfig.translationRequested",
                "translationPair: translationPair ? Object.freeze({ ...translationPair }) : null",
                "Browser ASR reported ${latestSourceContract.language}, but translation is configured for",
                "function handleBrowserProgress(update) {\n  throwIfCancelled();",
                "function handleBrowserTranslationProgress(update) {\n  throwIfCancelled();",
                "function applyBrowserTranslation(sourceContract, translated)",
                'words: [],',
                'chars: [],',
                'sourceWordAlignment: "not-projected-onto-translated-text"',
                'translation: "browser-post-asr"',
                'translation: "not-requested-in-browser-preview"',
                'alignment: "not-run-in-browser-preview"',
                'diarization: "not-run-in-browser-preview"',
                '"--no-align"',
                '"--return-char-alignments"',
                '"--diarize"',
                '"--translation-model"',
                '"--translation-source-language"',
                '"--translation-target-language"',
                '"--format"',
            ),
            "site/workbench.js",
        )
        reject(
            workbench_js,
            (
                "@huggingface/transformers",
                "pipeline(\"translation\"",
                "pipeline(\"automatic-speech-recognition\"",
                "new OfflineAudioContext",
            ),
            "site/workbench.js",
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
            vendored_translation,
            (
                "browserTranslationCapabilities",
                "createBrowserTranslationAdapter",
                "resolveBrowserTranslationPair",
                "supportsBrowserTranslation",
                "translateBrowserSegments",
                "platform-packages-transformers-js-webgpu-translation",
                "onnx-community/opus-mt-de-en",
                "onnx-community/opus-mt-en-de",
                "browser-cache",
                "webgpu",
                "No CPU, server, or Python fallback is used",
                "does not match",
            ),
            "site/vendor/platform-browser-translation.js",
        )
        reject(
            vendored_translation,
            (
                "NativeWhisperx",
                "startSeconds",
                "endSeconds",
                "renderSrt",
                "renderVtt",
                "SpeakerDirectory",
            ),
            "site/vendor/platform-browser-translation.js",
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
                'const BROWSER_RUN_EVIDENCE_KEY = "__nativeWhisperxBrowserRunEvidence"',
                "const completedRun = windowRef[BROWSER_RUN_EVIDENCE_KEY]",
                "No completed browser workflow evidence is available",
                "completedRun.translationRequested === true",
                "completedRun.translationCompleted === true",
                "completedRun.sourceTranscriptPreserved === true",
                "translationProvenanceComplete",
                "completedRun.translationRuntime",
                "completedRun.timedSegmentCount",
                "completedRun.projectionsAvailable === true",
                'availableFormats.includes("native-json")',
                'availableFormats.includes("srt")',
                'availableFormats.includes("vtt")',
                "Object.values(checks).every(Boolean)",
                "schemaVersion: 3",
                "native-whisperx-browser-acceptance-",
            ),
            "site/acceptance/acceptance.js",
        )
        reject(
            acceptance_js,
            (
                'documentRef.querySelector("#browser-translate")',
                'value(documentRef, "#browser-translation-model")',
                "transcribeAudioBlob(",
                "translateBrowserSegments(",
                "@huggingface/transformers",
                "pipeline(\"automatic-speech-recognition\"",
            ),
            "site/acceptance/acceptance.js",
        )
        require(
            prepare_site,
            (
                'AUDIO_ANALYSIS_REV="bf2cb13d155a874b166305da2f8dc669a05a2a58"',
                'AUDIO_SOURCE_PATH="packages/audio-analysis-transcription-wasm/index.js"',
                'PLATFORM_PACKAGES_REV="7241402712feef010bb9b5733560043ea845eb3d"',
                'PLATFORM_SOURCE_PATH="packages/browser-translation/src/browser.ts"',
                'PLATFORM_TARGET="$ROOT/site/vendor/platform-browser-translation.js"',
                "bun build",
                "--target=browser",
                "--format=esm",
            ),
            "scripts/prepare-site.sh",
        )
        reject(
            prepare_site,
            (
                "nlp-stack.git",
                "nlp-browser-translation.js",
            ),
            "scripts/prepare-site.sh",
        )
        require(transcribe, ("../workbench.html#browser-preview",), "site/transcribe/index.html")
        for workflow, owner in ((pages, ".github/workflows/pages.yml"), (site_workflow, ".github/workflows/site.yml")):
            require(
                workflow,
                (
                    "oven-sh/setup-bun@0c5077e51419868618aeaa5fe8019c62421857d6",
                    'bun-version: "1.4.0"',
                    "bash scripts/prepare-site.sh",
                    "python3 scripts/check-site.py",
                    "node --check site/vendor/audio-analysis-transcription.js",
                    "node --check site/vendor/platform-browser-translation.js",
                    "node --check site/workbench.js",
                    "node --check site/acceptance/acceptance.js",
                ),
                owner,
            )
        require(pages, ("actions/upload-pages-artifact@", "path: site"), ".github/workflows/pages.yml")

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

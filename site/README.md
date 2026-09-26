# native-whisperx GitHub Pages site

This directory contains the static site published by the GitHub Pages workflow.
It intentionally has no JavaScript build step or package-manager dependency.

The root page is a user-facing project surface. `workbench.html` is the speech
workflow workbench. The old `/transcribe/` entry point redirects to the browser
preview inside that workbench.

## Workbench model

The workbench deliberately exposes two runtime surfaces instead of pretending
that every native capability already executes inside WebAssembly:

1. **Browser-local speech workflow**
   - one local decode to 16 kHz mono through the pinned `audio-analysis` transcription adapter
   - multilingual Whisper ASR through the reusable WebGPU provider
   - optional speaker diarization through the pinned `audio-analysis` speaker adapter: 10-second pyannote segmentation windows, WavLM speaker embeddings, and cross-window cosine clustering in local WASM
   - optional curated German ↔ English post-ASR translation through the pinned `platform-packages` WebGPU adapter
   - speaker-aware Native JSON, TXT, SRT, and WebVTT projection; translation preserves source timing and speaker labels while dropping stale word/character alignments
   - alignment reported explicitly as unavailable in the browser slice
   - no silent server or Python inference fallback; ASR/translation are WebGPU, diarization is local WASM
2. **Full native workflow composer**
   - native Whisper transcription
   - default wav2vec2 alignment and optional character alignment
   - optional native diarization with speaker-count constraints
   - optional model-backed post-ASR translation
   - native output format selection
   - an exact `native-whisperx transcribe` command generated from the selected options

`native-whisperx` remains composition-only. Reusable browser/native ASR,
alignment, diarization, audio preparation, model caching, and model-runtime
mechanics belong to their canonical lower-level `audio-analysis` packages.
Browser translation execution belongs to the focused `platform-packages`
adapter. Pages owns browser interaction, pair selection, capability
presentation, source-transcript retention, projection into the Native
transcript shape, and native workflow composition.

The Pages workflow pins one exact `audio-analysis` commit and copies the
browser transcription and speaker adapters from
`packages/audio-analysis-transcription-wasm/index.js` and
`packages/audio-analysis-speakers-wasm/index.js` into `site/vendor/` before
validation and deployment. It also pins one exact reviewed `platform-packages`
commit and builds only `packages/browser-translation/src/browser.ts` with Bun
into the same generated vendor directory. The generated vendor directory is
not committed. This keeps the deployed site static while preserving upstream
implementation authority and deterministic provenance.

## Local preview

Prepare the pinned upstream adapter, then serve the directory rather than
opening the HTML through `file://`:

```bash
bash scripts/prepare-site.sh
python3 -m http.server 8000 -d site
```

Then open `http://127.0.0.1:8000/` or
`http://127.0.0.1:8000/workbench.html`.

## Validation

The static site contract is checked without downloading model weights:

```bash
bash scripts/prepare-site.sh
python3 scripts/check-site.py
node --check site/vendor/audio-analysis-transcription.js
node --check site/vendor/audio-analysis-speakers.js
node --check site/vendor/browser-translation.js
node --check site/workbench.js
node --check site/acceptance/acceptance.js
```

The checks verify required site files, the explicit browser/native runtime
boundary, both pinned upstream adapters, their no-fallback WebGPU contracts,
native command flags, the `/transcribe/` compatibility route, and the Pages
workflow validation step.

## Deployment

The Pages workflow prepares the pinned upstream adapter, validates the static
site, uploads `site/`, and deploys it with GitHub Pages. Repository Pages must
use GitHub Actions as the publishing source.

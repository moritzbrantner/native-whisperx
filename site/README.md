# native-whisperx GitHub Pages site

This directory contains the static site published by the GitHub Pages workflow.
It intentionally has no JavaScript build step or package-manager dependency.

The root page is a user-facing project surface. `workbench.html` is the speech
workflow workbench. The old `/transcribe/` entry point redirects to the browser
preview inside that workbench.

## Workbench model

The workbench deliberately exposes two runtime surfaces instead of pretending
that every native capability already executes inside WebAssembly:

1. **Browser WebGPU preview**
   - local browser audio decode and 16 kHz mono resampling owned by the pinned `audio-analysis` browser transcription adapter
   - multilingual Whisper transcription through that reusable WebGPU provider
   - timed transcript rendering and Native JSON, TXT, SRT, and WebVTT projection
   - alignment, diarization, and translation reported explicitly as unavailable in the browser slice
   - no silent server, Python, or CPU inference fallback
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
Pages owns browser interaction, capability presentation, projection into the
Native transcript shape, and native workflow composition.

The Pages workflow pins one exact `audio-analysis` commit and copies only
`packages/audio-analysis-transcription-wasm/index.js` into `site/vendor/` before
validation and deployment. The generated vendor directory is not committed.
This keeps the deployed site static while preserving upstream implementation
authority and deterministic provenance.

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
node --check site/workbench.js
```

The checks verify required site files, the explicit browser/native runtime
boundary, the pinned upstream ASR adapter, no-fallback WebGPU contract, native
command flags, the `/transcribe/` compatibility route, and the Pages workflow
validation step.

## Deployment

The Pages workflow prepares the pinned upstream adapter, validates the static
site, uploads `site/`, and deploys it with GitHub Pages. Repository Pages must
use GitHub Actions as the publishing source.

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
   - local browser audio decode and 16 kHz mono resampling
   - multilingual Whisper transcription through a pinned Transformers.js WebGPU reference runtime
   - Whisper speech translation to English as an explicitly labeled preview
   - timed transcript rendering and Native JSON, TXT, SRT, and WebVTT projection
   - no silent server, Python, or CPU inference fallback
2. **Full native workflow composer**
   - native Whisper transcription
   - default wav2vec2 alignment and optional character alignment
   - optional native diarization with speaker-count constraints
   - optional model-backed post-ASR translation
   - native output format selection
   - an exact `native-whisperx transcribe` command generated from the selected options

Alignment and diarization are intentionally reported as native-only in the
browser workbench. The current browser reference runtime does not approximate
them or claim Rust-Native Parity.

`native-whisperx` remains composition-only. Reusable browser/native ASR,
alignment, diarization, and model-runtime mechanics belong to their canonical
lower-level `audio-analysis` packages; Pages owns browser interaction,
capability presentation, model-cache policy for the preview, and native workflow
composition.

## Local preview

Serve the directory rather than opening the HTML through `file://`, because the
browser workbench uses an ES-module import and Web APIs:

```bash
python3 -m http.server 8000 -d site
```

Then open `http://127.0.0.1:8000/` or
`http://127.0.0.1:8000/workbench.html`.

## Validation

The static site contract is checked without downloading model weights:

```bash
python3 scripts/check-site.py
node --check site/workbench.js
```

The checks verify required site files, the four native-whisperx capability
labels, the explicit browser/native runtime boundary, the browser WebGPU model
contract, native command flags, the `/transcribe/` compatibility route, and the
Pages workflow validation step.

## Deployment

The Pages workflow validates the static site, uploads `site/`, and deploys it
with GitHub Pages. Repository Pages must use GitHub Actions as the publishing
source.

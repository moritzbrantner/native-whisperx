# native-whisperx GitHub Pages site

This directory contains the static site published by the GitHub Pages workflow.
It intentionally has no JavaScript build step or package manager dependency.

The contributor overview remains at `/`. The experimental browser transcription
surface is at `/transcribe/`.

## Browser transcription

`transcribe/` is the executable acceptance harness for
[`native-whisperx#272`](https://github.com/moritzbrantner/native-whisperx/issues/272).
It proves the browser/product slice independently from the reusable inference
provider:

- WebGPU capability detection with no silent server or CPU fallback
- local audio decode and resampling to 16 kHz mono
- lazy `whisper-tiny.en` model acquisition and browser-cache reuse
- timed transcript rendering
- Native JSON, TXT, SRT, and WebVTT projection

The current inference implementation is deliberately labeled a reference
runtime and uses Transformers.js WebGPU. It must not be described as
Rust-Native Parity. The reusable Rust/Burn WebGPU provider belongs to
`audio-analysis-transcription` and is tracked by
[`audio-analysis#51`](https://github.com/moritzbrantner/audio-analysis/issues/51).
Once that provider passes its `wasm32-unknown-unknown` gate, the Pages surface
should consume the Rust/WASM adapter without changing the product contract.

Alignment, diarization, translation, Python WhisperX, server inference, and
broad media decoding are outside the browser MVP.

## Local preview

Serve the directory rather than opening the HTML through `file://`, because the
browser transcription page uses an ES-module import and Web APIs:

```bash
python3 -m http.server 8000 -d site
```

Then open `http://127.0.0.1:8000/` for the contributor overview or
`http://127.0.0.1:8000/transcribe/` for browser transcription.

## Updating benchmark content

Benchmark copy on the site is curated from checked-in repository notes. The
benchmark section currently reports the hard 30s, 3m, and 10m local CUDA
throughput ladder plus a report-only multi-input baseline. When updating
numbers:

1. Update the benchmark source note first.
2. Copy only contributor-safe values into `index.html`.
3. Keep the benchmark context beside the numbers: input, model, device, and
   provider path.
4. Preserve the local-CUDA-gate caveat for the hard ladder, and keep multi-input
   benchmark values labeled as report-only baseline evidence.
5. Source multi-input benchmark values from `docs/native-performance-findings.md`
   before publishing them on the site.
6. Avoid local absolute paths, smoke-root paths, private cache paths, tokens, or
   machine-specific command output.

The current source is `docs/native-performance-findings.md`.

## Deployment

The Pages workflow uploads this directory as a static artifact and deploys it
with GitHub Pages. Repository admins still need GitHub Pages enabled for the
repository and configured to use GitHub Actions as the publishing source.

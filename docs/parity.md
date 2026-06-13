# Parity

Python WhisperX CLI parity is the product contract for native-whisperx, starting
with the versioned baseline in [`parity-matrix.md`](./parity-matrix.md). The
Rust implementation is the direction of travel, but Python delegation is
allowed while equivalent Rust features mature.

The native replacement work is tracked row-by-row in
[`parity-worklist.md`](./parity-worklist.md). That worklist is the operational
source of truth for whether a CLI surface is native complete, native partial,
delegated only, blocked by an upstream crate, or waiting on fixtures.

The Rust workflow composes these pieces:

- Candle Whisper ASR through `moritzbrantner-audio-analysis-transcription`,
  with explicit bundles or supported Hugging Face cache/download resolution
- wav2vec2 CTC alignment from a supported local bundle or Hugging Face cache
- heuristic or ONNX-backed speaker diarization when explicitly enabled
- transcript normalization and WhisperX JSON import through
  `moritzbrantner-text-transcripts`

## Current milestone

The current milestone is native ASR Hugging Face cache parity plus minimal
native translation task selection. Native ASR no longer requires
`--whisper-bundle` when a supported Whisper model is already in the Hugging Face
cache or downloads are allowed. `--whisper-bundle` remains the recommended
deterministic offline path. Native `--task translate --no-align` now selects
Whisper's translation task, while aligned translation remains delegated/planned.
Native pyannote VAD remains deferred and delegated to Python WhisperX.

The repository has an ignored/manual wrapper smoke for cache-only native ASR
resolution:

```bash
cargo test -p native-whisperx-cli \
  --test native_asr_cache_smoke \
  -- --ignored --nocapture
```

Set `SMOKE_ROOT` to a local smoke root before running it. See
[`model-bundles.md`](./model-bundles.md#manual-native-asr-cache-smoke) for the
required audio and Hugging Face cache layout. Default CI does not run this
smoke.

External Python WhisperX remains the compatibility bridge for behavior that is
not native yet. Unsupported native controls fail with explicit configuration
errors instead of being ignored, while delegated controls are forwarded through
the current external command argument bridge.

Default CI remains offline. It uses checked-in fixtures, fake command tests,
and mocked Silero probability tests; real Python WhisperX, real Silero ONNX
models, native ASR cache/download parity, model downloads, and HF-token-gated
diarization remain manual or opt-in checks. The real ASR cache smoke test is
ignored/manual.

Current parity failures or planned work versus Python WhisperX:

- faster-whisper throughput and batching parity
- pyannote VAD semantics
- pyannote-compatible diarization
- full native decode controls
- exact WhisperX sentence segmentation and subtitle layout parity
- production diarization must become pyannote-compatible
- ASR execution needs correctness plus runtime/resource benchmarks before Rust
  paths replace delegated parity paths
- ONNX Runtime dynamic-library discovery is host-sensitive

Parity reports now include additional comparison categories for language,
segment text sequence, word text sequence, character alignment count, and
diagnostic differences. Existing top-level text, timing, word count, segment
count, and speaker-turn fields remain part of the report.

## Surface changes

This milestone extends `--model-dir` and `--model-cache-only` to native ASR in
addition to native alignment and delegated Python WhisperX forwarding. It also
allows native translation only when alignment is disabled. Native behavioral
parity is still intentionally limited to the implemented Rust paths described
in the parity matrix.

Run native-vs-Python comparison only when local Python tooling is installed:

```bash
cargo run -p native-whisperx-cli --features whisperx-compat -- parity input.wav \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --whisperx-model tiny.en \
  --align-model facebook/wav2vec2-base-960h \
  --interpolate-method nearest \
  --expected-json expected.json \
  --language en
```

Set `HF_TOKEN` before parity runs that ask Python WhisperX to diarize.

Additional manual smoke commands for ASR cache resolution, Silero VAD, and
ONNX diarization are collected in
[`parity-worklist.md`](./parity-worklist.md#manual-parity-commands).

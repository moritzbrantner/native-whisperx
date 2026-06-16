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
- planned Helsinki-NLP OPUS-MT/Marian post-ASR segment translation when
  `--translation-model` is supplied, currently blocked by the unpublished
  upstream Marian runtime feature
- transcript normalization and WhisperX JSON import through
  `moritzbrantner-text-transcripts`

## Current milestone

The current milestone is native ASR timing parity after the Hugging Face cache
path. Native ASR no longer requires `--whisper-bundle` when a supported Whisper
model is already in the Hugging Face cache or downloads are allowed.
`--whisper-bundle` remains the recommended deterministic offline path. Native
English-only Whisper aliases such as `tiny.en` provide an `en` language hint
when no explicit language is supplied, which keeps the local language-detection
fixture aligned with WhisperX for English-only models. Native `--task translate
--translation-model ...` is kept in the planned contract but currently reports a
configuration error until the upstream Marian runtime is available from
crates.io. Native `--task translate` without a translation model is delegated to
Python WhisperX for parity today. Native pyannote VAD remains deferred and
delegated to Python WhisperX.

The repository has an ignored/manual wrapper smoke for cache-only native ASR
resolution and a local-only ASR parity fixture suite. The fixture suite is the
next native parity checkpoint: it compares native ASR against Python WhisperX
for real local audio, cache-only model resolution, explicit language, alignment,
optional character alignments, and output writer goldens. TXT/TSV/SRT/VTT/AUD
goldens are compared byte-for-byte; JSON goldens are compared semantically.
The core English ASR fixtures now gate native timing parity: no-alignment
`tiny.en`, no-alignment `small`, and English-only language detection gate
segment timing; the aligned `tiny.en` fixture gates segment and word timing;
and the char-alignment fixture gates segment timing, word timing, and character
count. Timing reports include native and WhisperX start/end values, absolute
deltas, and the configured tolerance for each mismatch. German ASR expansion,
alignment alias/cache behavior, translation, and
`tiny-output-subtitles-highlight` remain non-gating until independently
promoted. Output writer fixtures `tiny-output-subtitles-wrap` and
`tiny-output-segment-resolution-chunk` gate byte-for-byte SRT/VTT goldens, and
`tiny-output-all-defaults` gates TXT/VTT/SRT/TSV byte-for-byte goldens plus
contract-aware semantic JSON comparison. `tiny-output-subtitles-highlight`
stays report-only because highlighted word cue milliseconds are derived from
exact word timing and can still differ at the byte level while word timing is
within the strict 0.050s tolerance.

```bash
cargo test -p native-whisperx-cli \
  --test native_asr_cache_smoke \
  -- --ignored --nocapture
```

```bash
cargo run -p native-whisperx-cli -- parity-preflight tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --require-expected
```

```bash
cargo run -p native-whisperx-cli -- parity-goldens tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --overwrite
```

```bash
cargo run -p native-whisperx-cli -- parity-fixtures tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --output-dir "$SMOKE_ROOT/out/parity-fixtures"
```

Set `SMOKE_ROOT` to a local smoke root before running it. See
[`model-bundles.md`](./model-bundles.md#local-asr-parity-fixtures) for the
required audio, expected WhisperX JSON, and Hugging Face cache layout. Default
CI does not run these local real-model checks.

Full-resource parity measurements live in a separate non-gating manifest while
native Silero and diarization behavior is still converging:

```bash
HF_TOKEN=... \
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features whisperx-compat,silero-vad,onnx-diarization \
  -- parity-fixtures tests/parity/full-resource-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --output-dir "$SMOKE_ROOT/out/full-resource-parity"
```

External Python WhisperX remains the compatibility bridge for behavior that is
not native yet. Unsupported native controls fail with explicit configuration
errors instead of being ignored, while delegated controls are forwarded through
the current external command argument bridge.

Default CI remains offline. It uses checked-in fixtures, fake command tests,
and mocked Silero probability tests; real Python WhisperX, real Silero ONNX
models, native ASR cache/download parity, model downloads, and HF-token-gated
diarization remain local/manual or future opt-in checks. The real ASR cache
smoke test is ignored/manual, and the ASR parity fixture suite is local-only for
now. A later CI step can move the suite into a scheduled or labeled workflow
with prewarmed caches/secrets.

Current parity failures or planned work versus Python WhisperX:

- faster-whisper throughput and batching parity
- pyannote VAD semantics
- pyannote-compatible diarization
- full native decode controls
- broader WhisperX sentence segmentation coverage beyond the current writer
  goldens
- production diarization must become pyannote-compatible
- ASR execution needs correctness plus runtime/resource benchmarks before Rust
  paths replace delegated parity paths
- ONNX Runtime dynamic-library discovery is host-sensitive

Parity reports now include additional comparison categories for language,
segment text sequence, word text sequence, character alignment count, and
diagnostic differences. Fixture reports also include expected output-file
comparisons and distinguish gating from non-gating cases. Existing top-level
text, timing, word count, segment count, and speaker-turn fields remain part of
the report.

## Surface changes

This milestone extends `--model-dir` and `--model-cache-only` to native ASR,
native alignment, native Helsinki translation, and delegated Python WhisperX
forwarding. It also aligns `--segment_resolution` with WhisperX
`sentence|chunk`, with `sentence` as the default and `segment` retained only as
a legacy alias. Native behavioral parity is still intentionally limited to the
implemented Rust paths described in the parity matrix.

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

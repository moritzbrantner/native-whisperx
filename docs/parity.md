# Parity

Python WhisperX CLI parity is the product contract for native-whisperx, starting
with the versioned baseline in [`parity-matrix.md`](./parity-matrix.md). The
Rust implementation is the direction of travel, but Python delegation is
allowed while equivalent Rust features mature.

The Rust workflow composes these pieces:

- Candle Whisper ASR through `moritzbrantner-audio-analysis-transcription`
- wav2vec2 CTC alignment from a supported local bundle or Hugging Face cache
- heuristic or ONNX-backed speaker diarization when explicitly enabled
- transcript normalization and WhisperX JSON import through
  `moritzbrantner-text-transcripts`

## Current milestone

The current milestone is the native Silero VAD bridge. `energy` remains the
default native VAD, while `silero` can run natively only when the `silero-vad`
Cargo feature is enabled and the caller supplies a local ONNX model bundle.
Native pyannote VAD remains deferred and delegated to Python WhisperX.

External Python WhisperX remains the compatibility bridge for behavior that is
not native yet. Unsupported native controls fail with explicit configuration
errors instead of being ignored, while delegated controls are forwarded through
the current external command argument bridge.

Default CI remains offline. It uses checked-in fixtures, fake command tests,
and mocked Silero probability tests; real Python WhisperX, real Silero ONNX
models, model downloads, and HF-token-gated diarization remain manual or
opt-in checks. The real Silero model smoke test is ignored and gated by
`NATIVE_WHISPERX_SILERO_ONNX`.

Current parity failures or planned work versus Python WhisperX:

- faster-whisper throughput and batching parity
- pyannote VAD semantics
- pyannote-compatible diarization
- full native decode controls
- exact WhisperX sentence segmentation and subtitle layout parity
- production diarization must become pyannote-compatible
- ASR execution needs correctness plus runtime/resource benchmarks before Rust
  paths replace delegated parity paths
- ASR model-ID resolution still needs Hugging Face cache parity beyond local
  Whisper bundles
- ONNX Runtime dynamic-library discovery is host-sensitive

## Surface changes

This milestone adds native-only Silero model wiring flags,
`--vad-model-bundle`, `--vad-model-file`, `--vad-input-name`, and
`--vad-output-name`, plus native strict errors for missing feature/model
configuration. Native behavioral parity is still intentionally limited to the
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

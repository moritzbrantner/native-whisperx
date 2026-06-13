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

The CLI contract has been broadened to the WhisperX 3.8.6 surface. The Rust
parser now accepts the upstream-style task, device index, batch size, compute
type, VAD, diarization, decode, subtitle, output, and short alias flags that
belong to this milestone.

External Python WhisperX remains the compatibility bridge for behavior that is
not native yet. Unsupported native controls fail with explicit configuration
errors instead of being ignored, while delegated controls are forwarded through
the current external command argument bridge.

Default CI remains offline. It uses checked-in fixtures and fake command tests;
real Python WhisperX, model downloads, and HF-token-gated diarization remain
manual or opt-in checks.

Current parity failures or planned work versus Python WhisperX:

- faster-whisper throughput and batching parity
- silero and pyannote VAD semantics
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

This milestone adds CLI aliases and flags for the broader WhisperX contract,
output formats `all`, `tsv`, and `aud`, native strict errors for unsupported
runtime behavior, and an external delegation route for Python WhisperX-only
features. Native behavioral parity is still intentionally limited to the
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

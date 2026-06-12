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

Current parity failures or planned work versus Python WhisperX:

- production diarization must become pyannote-compatible
- ASR execution needs correctness plus runtime/resource benchmarks before Rust
  paths replace delegated parity paths
- ASR model-ID resolution still needs Hugging Face cache parity beyond local
  Whisper bundles
- ONNX Runtime dynamic-library discovery is host-sensitive

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

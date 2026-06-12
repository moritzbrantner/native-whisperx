# Parity

Python WhisperX remains compatibility and parity tooling, not the default Rust
path.

The Rust workflow composes these pieces:

- Candle Whisper ASR through `moritzbrantner-audio-analysis-transcription`
- wav2vec2 CTC alignment when a supported local bundle is supplied
- heuristic or ONNX-backed speaker diarization when explicitly enabled
- transcript normalization and WhisperX JSON import through
  `moritzbrantner-text-transcripts`

Known gaps versus Python WhisperX:

- production diarization quality is not pyannote parity
- ASR execution is deterministic sequential Candle execution, not full
  WhisperX throughput parity
- wav2vec2 support is limited to supported local bundle layouts
- ONNX Runtime dynamic-library discovery is host-sensitive

Run an external WhisperX comparison only when local Python tooling is installed:

```bash
cargo run -p native-whisperx-cli --features whisperx-compat -- parity input.wav \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --whisperx-model tiny.en \
  --expected-json expected.json \
  --language en
```

Set `HF_TOKEN` before parity runs that ask Python WhisperX to diarize.


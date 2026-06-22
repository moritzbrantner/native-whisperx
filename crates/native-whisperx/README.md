# native-whisperx

Reusable workflow library for composing `moritzbrantner-*` transcription,
alignment, diarization, and transcript crates into a WhisperX-style pipeline.

This crate owns workflow configuration and output writing. The reusable audio,
text, model-runtime, and speaker primitives remain in `rust-packages`.

## Release Role

`native-whisperx` is the library crate for Workflow Composition APIs. It is
published before `native-whisperx-cli`, which is the Cargo install package for
the `native-whisperx` terminal command.

## Workflow Surface

The crate composes native transcription, default wav2vec2 alignment, optional
speaker diarization, optional segment-level post-ASR translation, and output
writing. The user-facing parity target is the Python WhisperX CLI. Delegated
features remain delegated where the repository has not yet replaced them with a
Rust-Native Parity path.

Default `json` output is WhisperX JSON. Use `native-json` through the CLI when
you need the Rust transcript contract shape.

## Feature Flags

| Feature | Purpose |
| --- | --- |
| `native` | Native Candle Whisper and wav2vec2 alignment composition. Enabled by default. |
| `translation` | Helsinki-NLP OPUS-MT/Marian post-ASR segment translation. Enabled by `native`. |
| `cuda` | CUDA-backed Candle execution for hosts with a local CUDA toolchain. |
| `media-decode` | Non-WAV media/container decode through the audio I/O crate. |
| `diarization` | Heuristic speaker diarization composition. |
| `onnx-diarization` | Explicit ONNX speaker embedding diarization path. |
| `pyannote-diarization` | Explicit native pyannote community diarization bundle path. |
| `silero-vad` | Explicit Silero ONNX VAD path. |
| `pyannote-vad` | Explicit local pyannote ONNX VAD path. |
| `whisperx-compat` | External Python WhisperX command compatibility and parity checks. |

## Documentation

The canonical public API documentation target is
<https://docs.rs/native-whisperx>. Repository release and model-resource setup
details are in <https://github.com/moritzbrantner/native-whisperx>.

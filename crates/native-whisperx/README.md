# native-whisperx

Reusable workflow library for composing `moritzbrantner-*` transcription,
alignment, diarization, and transcript crates into a WhisperX-style pipeline.

This crate owns workflow configuration and output writing. The reusable audio,
text, model-runtime, and speaker primitives remain in `rust-packages`.

## Release Role

`native-whisperx` is the library crate for Workflow Composition APIs. It is
published before `native-whisperx-cli`. After that manual release order, Cargo
users install the CLI package with:

```bash
cargo install native-whisperx-cli
```

The installed terminal command is `native-whisperx`.

## Workflow Surface

The crate composes native transcription, default wav2vec2 alignment, optional
speaker diarization, optional segment-level post-ASR translation, and output
writing. The user-facing parity target is the Python WhisperX CLI. Delegated
features remain delegated where the repository has not yet replaced them with a
Rust-Native Parity path.

Default `json` output is WhisperX JSON. Use `native-json` through the CLI when
you need the Rust transcript contract shape.

Installing the CLI package does not make transcription resources available.
Model bundles, cache entries, CUDA, Python WhisperX compatibility resources,
and gated Hugging Face assets are resolved by the invoked workflow. Delegated
Feature paths remain delegated until separate Rust-Native Parity work replaces
them.

## Feature Flags

| Feature | Purpose |
| --- | --- |
| `native` | Native Candle Whisper and wav2vec2 alignment composition. Enabled by default. |
| `translation` | Helsinki-NLP OPUS-MT/Marian post-ASR segment translation. Opt in for translate workflows; it is not enabled by default. |
| `cuda` | CUDA-backed Candle execution for hosts with a local CUDA toolchain. |
| `media-decode` | FFmpeg-backed finite non-WAV media/container decode through the audio I/O crate. Enabled by default. |
| `diarization` | Heuristic speaker diarization composition. |
| `onnx-diarization` | Explicit ONNX speaker embedding diarization path. |
| `pyannote-diarization` | Native pyannote community diarization bundle path. Enabled by default for Automatic Workflow Selection, with runtime resources resolved lazily. |
| `silero-vad` | Explicit Silero ONNX VAD path. |
| `pyannote-vad` | Native pyannote ONNX VAD path. Enabled by default for Automatic Workflow Selection, with runtime resources resolved lazily. |
| `whisperx-compat` | External Python WhisperX command compatibility and parity checks. |

Default library and CLI packaging includes the pyannote VAD and pyannote
diarization code paths required by automatic native `--diarize`, but it does
not bundle models or eagerly access Hugging Face credentials, ONNX Runtime
dynamic-library configuration, CUDA, Python WhisperX, or parity resources.
Builds using `--no-default-features` remain a minimal compile path; enable only
the feature rows needed by that check.

## Finite Media Inputs

Default builds include finite media decode support. WAV files continue to use
the native WAV reader path. Non-WAV finite media files route through the
FFmpeg-backed media decode path when the required runtime tools are installed.

The guaranteed finite input set is `wav`, `mp3`, `m4a`, `aac`, `flac`, `ogg`,
`opus`, `mp4`, `mov`, `mkv`, and `webm`. Other FFmpeg-decodable files may work
on a best-effort basis, but they are not part of the guaranteed support set.
Video files are transcribed from the selected/default audio track only; video
frames are not analyzed.

Builds using `--no-default-features` do not implicitly include finite non-WAV
media decode. Enable `media-decode` explicitly for minimal builds that still
need FFmpeg-backed media/container input support.

## Finite Progress and Cancellation

`run_with_observer` and `run_many_with_observer` emit the ordered
Transcription Progress Stream while preserving their existing non-cancellable
return types. Embedding applications that need cooperative cancellation use
`run_with_control` or `run_many_with_control` with a cloneable
`CancellationHandle`; another thread may call `cancel()`, and Workflow
Composition stops at the next safe phase boundary. Cancellation is a typed
outcome and does not emit a generic failure. A cancelled Multi-Input
Transcription Run retains completed reports and identifies inputs that were not
finished.

`TranscriptionProgressEvent` is now `#[non_exhaustive]` because model
resolution/download and direct or Pivot Translation leg facts extend the
stream. Existing consumers that exhaustively matched the enum must add a
wildcard arm. Existing `run`, `run_with_observer`, `run_many`, and
`run_many_with_observer` calls remain available and use an uncancelled control
internally.

## Documentation

The canonical public API documentation target is
<https://docs.rs/native-whisperx>. Repository release and model-resource setup
details are in <https://github.com/moritzbrantner/native-whisperx>.

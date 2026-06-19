# Keep native pyannote VAD explicit until a compatible implementation exists

Native `--vad-method pyannote` must continue to fail explicitly instead of
falling back to energy or Silero VAD. Python WhisperX delegation remains the
compatibility path unless a local pyannote ONNX bundle is supplied.

## Context

The parity contract targets Python WhisperX 3.8.6. That surface exposes
pyannote VAD behavior through `--vad_method pyannote`, including threshold and
chunking semantics that are not equivalent to the current native energy VAD or
feature-gated Silero ONNX path.

## Decision

Do not implement native pyannote VAD as an alias for another native VAD method.
Native pyannote VAD is allowed only through the `pyannote-vad` feature with an
explicit local ONNX bundle and fixture-tested merged speech chunks. Without the
feature or bundle, native pyannote remains a direct configuration error.

Acceptable routes are:

- a pyannote-compatible Rust/ONNX model path,
- an explicit Python dependency bridge separate from full WhisperX delegation,
- or an upstream Rust provider that exposes pyannote-compatible segmentation.

## Consequences

The native provider now has a local-ONNX pyannote VAD path. The external
WhisperX provider continues to forward `--vad_method pyannote` for
compatibility. Native parity can only be claimed when pyannote segment goldens
pass against Python WhisperX fixtures.

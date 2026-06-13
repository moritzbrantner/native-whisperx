# Keep native pyannote VAD delegated until a compatible implementation exists

Native `--vad-method pyannote` must continue to fail explicitly instead of
falling back to energy or Silero VAD. Python WhisperX delegation remains the
compatibility path for pyannote VAD semantics.

## Context

The parity contract targets Python WhisperX 3.8.6. That surface exposes
pyannote VAD behavior through `--vad_method pyannote`, including threshold and
chunking semantics that are not equivalent to the current native energy VAD or
feature-gated Silero ONNX path.

## Decision

Do not implement native pyannote VAD as an alias for another native VAD method.
Keep native pyannote rejected with a direct configuration error until one of
these routes is implemented and fixture-tested:

- a pyannote-compatible Rust/ONNX model path,
- an explicit Python dependency bridge separate from full WhisperX delegation,
- or an upstream Rust provider that exposes pyannote-compatible segmentation.

## Consequences

The native provider may remain incomplete for pyannote VAD while still offering
honest behavior. The external WhisperX provider continues to forward
`--vad_method pyannote` for compatibility. Native parity can only be claimed
after pyannote segment goldens pass against Python WhisperX fixtures.

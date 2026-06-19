# Use this repository as the Rust-native parity proving ground

New Rust-Native Parity work implements user-visible WhisperX behavior in this
repository and its checked-in vendor code without adding new Python WhisperX or
faster-whisper runtime bridges. ADR 0003 remains valid for the existing
compatibility bridge, but this program is stricter: Python WhisperX is only the
reference oracle, and extraction to upstream crates waits until parity and
large-v3-turbo CUDA speed are proven here.

## Consequences

Some implementations may be less reusable initially because they stay in this
repository while the full ASR, alignment, VAD, diarization, translation, output
writer, decode-control, parity-harness, and benchmark surfaces converge. The
upstream extraction boundary should be revisited only after the benchmark ladder
and full-resource parity evidence show that the Rust-native path is both
correct and fast enough.

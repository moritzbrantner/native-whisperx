# Use this repository as the Rust-native parity proving ground

New Rust-Native Parity work implements user-visible WhisperX behavior in this
repository and its checked-in vendor code without adding new Python WhisperX or
faster-whisper runtime bridges. ADR 0003 remains valid for the existing
compatibility bridge, but this program is stricter: Python WhisperX is only the
reference oracle. Correctness parity can merge before large-v3-turbo CUDA speed
is proven when the speed gap is documented and split into explicit runtime
follow-up work.

## Consequences

Some implementations may be less reusable initially because they stay in this
repository while the full ASR, alignment, VAD, diarization, translation, output
writer, decode-control, parity-harness, and benchmark surfaces converge. The
upstream extraction boundary should be revisited after full-resource parity
evidence is correct and any remaining benchmark gap is assigned to concrete
runtime work. The 2026-06-20 long-form benchmark findings are recorded in
[`../native-performance-findings.md`](../native-performance-findings.md).

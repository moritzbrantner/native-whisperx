# ADR 0006: Use this repository as the Rust-native parity proving ground

## Status

Superseded by [ADR 0015](0015-composition-only-product-boundary.md).

## Historical decision

New Rust-Native Parity work implemented user-visible WhisperX behavior in this
repository and its checked-in vendor code without adding new Python WhisperX or
faster-whisper runtime bridges. ADR 0003 remained valid for the existing
compatibility bridge, but this program was stricter: Python WhisperX was only the
reference oracle. Correctness parity could merge before large-v3-turbo CUDA
speed was proven when the speed gap was documented and split into explicit
runtime follow-up work.

The proving-ground phase also allowed implementations to remain in this
repository temporarily while ASR, alignment, VAD, diarization, translation,
output-writer, decode-control, parity-harness, and benchmark surfaces converged.

## Superseding direction

Canonical upstream owners and exact source-development workflows now exist, so
the temporary implementation-locality exception is no longer appropriate.
ADR 0015 makes Native WhisperX permanently composition-only: reusable
capabilities are implemented in their canonical lower-level owners first, while
this repository owns Workflow Composition, compatibility policy, product APIs,
and the Parity Harness.

The historical 2026-06-20 long-form benchmark findings remain recorded in
[`../native-performance-findings.md`](../native-performance-findings.md).

# ADR 0007: Full Workflow Throughput Gate

## Status

Accepted.

## Context

Rust-native ASR, VAD, and output writing are already faster than the WhisperX
reference when alignment is disabled. The throughput regression appears in the
complete user-visible workflow because native wav2vec2 CTC alignment was
CPU-bound while the native Whisper ASR path could run on CUDA.

Weakening the benchmark to ASR-only would hide the user-visible bottleneck.

## Decision

Keep the throughput gate as a full-workflow benchmark that includes VAD, ASR,
alignment, and output.

`nativeAsr.device` is the source of the native workflow device. When the native
ASR device is `cuda`, lower-level alignment options receive `Cuda`; when it is
`cpu`, they receive `Cpu`; when it is `auto`, they receive `Auto`. The CLI does
not expose a separate `--alignment-device` flag in this slice.

The benchmark fixture must not contain `alignment.device`. Benchmark JSON
reports include `alignmentCudaActive` and `alignmentDevice`, sourced from native
alignment diagnostics, so the gate can prove that alignment used the inherited
runtime.

## Consequences

The native benchmark remains comparable to default WhisperX behavior because
alignment stays enabled.

If CUDA alignment still does not beat WhisperX, the gate should remain
alignment-inclusive and the next optimization should target wav2vec2 alignment
emission throughput, such as batching segment emission.

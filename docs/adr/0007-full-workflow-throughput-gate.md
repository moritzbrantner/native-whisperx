# ADR 0007: Full Workflow Throughput Gate

## Status

Accepted, amended 2026-06-21.

## Context

Rust-native ASR, VAD, and output writing were initially faster than the WhisperX
reference when alignment was disabled. The first throughput regression appeared
in the complete user-visible workflow because native wav2vec2 CTC alignment was
CPU-bound while the native Whisper ASR path could run on CUDA.

After CUDA alignment landed, the long-form regression moved to native Whisper
ASR. The 2026-06-20 benchmark ladder showed the 30 second and 3 minute rungs
beating WhisperX, but the 10 minute rung stayed slower because native ASR still
reported `batchExecution=candle-whisper-sequential`.

After active-row Candle Whisper decode and CUDA encoder microbatching landed,
the 2026-06-21 benchmark ladder passed the 30 second, 3 minute, and 10 minute
rungs with warmup enabled and three measured iterations per case.

Weakening the benchmark to ASR-only would hide the user-visible bottleneck.

## Decision

Keep the throughput measurement as a full-workflow benchmark that includes VAD,
ASR, alignment, and output. Restore the Rust-Native large-v3-turbo CUDA ladder
as a hard gate in the `final-full-surface` parity workflow suite. The gate
continues to depend on local smoke media, cached models, Python WhisperX, and
CUDA hardware, so it remains outside default offline CI.

`nativeAsr.device` is the source of the native workflow device. When the native
ASR device is `cuda`, lower-level alignment options receive `Cuda`; when it is
`cpu`, they receive `Cpu`; when it is `auto`, they receive `Auto`. The CLI does
not expose a separate `--alignment-device` flag in this slice.

The benchmark fixture must not contain `alignment.device`. Benchmark JSON
reports include `alignmentCudaActive` and `alignmentDevice`, sourced from native
alignment diagnostics, so the gate can prove that alignment used the inherited
runtime.

The WhisperX reference uses the CLI default compute type rather than a pinned
`float16` override. The gate compares default WhisperX behavior and should not
fail before comparison because a backend rejects an explicit compute override.

Each measured benchmark iteration must report finite native and WhisperX
elapsed seconds, `nativeFasterThanWhisperx=true`, and
`nativeSpeedupRatio >= 1.001`. The workflow must explicitly select all three
canonical ladder cases so missing cases fail the gate.

## Consequences

The native benchmark remains comparable to default WhisperX behavior because
alignment stays enabled and WhisperX remains the reference oracle.

The merge-blocking speed gate is restored for the final local parity suite.
Findings are recorded in
[`../native-performance-findings.md`](../native-performance-findings.md).

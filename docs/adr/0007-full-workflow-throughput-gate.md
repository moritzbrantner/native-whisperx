# ADR 0007: Full Workflow Throughput Gate

## Status

Accepted, amended 2026-06-20.

## Context

Rust-native ASR, VAD, and output writing were initially faster than the WhisperX
reference when alignment was disabled. The first throughput regression appeared
in the complete user-visible workflow because native wav2vec2 CTC alignment was
CPU-bound while the native Whisper ASR path could run on CUDA.

After CUDA alignment landed, the long-form regression moved to native Whisper
ASR. The 2026-06-20 benchmark ladder showed the 30 second and 3 minute rungs
beating WhisperX, but the 10 minute rung stayed slower because native ASR still
reported `batchExecution=candle-whisper-sequential`.

Weakening the benchmark to ASR-only would hide the user-visible bottleneck.

## Decision

Keep the throughput measurement as a full-workflow benchmark that includes VAD,
ASR, alignment, and output. It remains manual/report-only until native
long-form ASR has an efficient batched autoregressive generation path.

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

## Consequences

The native benchmark remains comparable to default WhisperX behavior because
alignment stays enabled.

The merge-blocking speed gate is deferred. The next optimization should target
native Candle Whisper long-form ASR generation, specifically decoder KV-cache
reuse and active-row compaction. Findings are recorded in
[`../native-performance-findings.md`](../native-performance-findings.md).

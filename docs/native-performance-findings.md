# Native Performance Findings

This note records the Rust-Native benchmark ladder evidence for the current
parity milestone and the runtime work that restored full-workflow throughput.

## Status

Correctness parity continues to use Python WhisperX as a reference oracle. The
30 second, 3 minute, and 10 minute large-v3-turbo CUDA ladder remains a manual
throughput report because it depends on local media, cached models, Python
WhisperX, and CUDA hardware. The 2026-06-21 `moenarch-*` active-row run passed
the full ladder with warmup enabled and three measured iterations per case.

The runtime work that restored the ladder is tracked separately:

- #34: umbrella for real native Candle Whisper long-form batching.
- #35: efficient autoregressive generation with proper decoder KV-cache
  behavior.
- #36: active-row compaction for batched generation.
- #37: restoring the native-vs-WhisperX performance gate after the runtime work
  lands.
- #65: benchmark evidence for true batched decode.
- #74/#75: publish and consume the `moenarch-audio-analysis-transcription`
  CUDA encoder microbatch fix.

## Measurements

All measurements below used the same local benchmark shape unless noted:

```bash
set -a
. ./.env
set +a
WHISPERX_COMMAND="$(conda run -n whisperx which whisperx)"
ORT_DYLIB_PATH="$PWD/.audio-tools/onnxruntime/lib/libonnxruntime.so.1.26.0"
LD_LIBRARY_PATH="$PWD/.audio-tools/onnxruntime/lib:$LD_LIBRARY_PATH"
cargo run -p native-whisperx-cli \
  --features whisperx-compat,media-decode,silero-vad,pyannote-vad,pyannote-diarization,cuda \
  -- parity-bench tests/parity/rust-native-bench-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command "$WHISPERX_COMMAND" \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --case-timeout-seconds 900 \
  --json
```

| Variant | Native elapsed | WhisperX elapsed | Speedup | Notes |
| --- | ---: | ---: | ---: | --- |
| Baseline 10m dev | 50.96-51.52s | 21.04-22.41s | 0.413-0.436x | ASR took 43.23-43.57s; diagnostics reported `batchExecution=candle-whisper-sequential`. |
| Cross-attention cache reuse, max batch 1, dev | 46.584s | 23.115s | 0.496x | Helped, but still far slower than WhisperX. |
| Tensor batch cap 4, dev | 79.26-84.58s | 22-23s | 0.273-0.290x | Rectangular token batching through the current decoder path regressed badly. |
| Tensor batch cap 2, dev | 68.810s | 58.708s | 0.853x | Still slower and noisy against the reference run. |
| Cross-attention cache reuse, max batch 1, release | 30.665s | 24.468s | 0.798x | Release mode helps, but does not close the gap. |
| Tensor batch cap 2, release | 37.976s | 23.896s | 0.629x | Tensor batching remains slower than sequential. |
| Forced no-timestamp sequential, release | 39.663s | 23.795s | 0.600x | The no-timestamp workaround does not solve long-form throughput. |

2026-06-21 active-row registry repair run:

- Report: `/home/moenarch/moritzbrantner/native-whisperx/.smoke/out/benchmarks/issue-65-moenarch-full-ladder-20260621T200940Z.json`
- Command shape: checked-in `tests/parity/rust-native-bench-fixtures.json`,
  `SMOKE_ROOT` loaded from `.env`, WhisperX resolved with
  `conda run -n whisperx which whisperx`, `--model-cache-only`,
  `--case-timeout-seconds 900`, `--json`.
- Native dependency repair: `moenarch-audio-analysis-transcription 0.1.8`
  from crates.io, including CUDA encoder microbatching and active-row decoder
  batching.

| Case | Native elapsed | WhisperX elapsed | Speedup | Diagnostics |
| --- | ---: | ---: | ---: | --- |
| 30s large-v3-turbo CUDA | 1.786-1.873s | 8.823-50.388s | 4.709-28.216x | Single chunk uses safe `candle-whisper-autoregressive-kv-cache`; `chunkCount=1`, `batchCount=1`, `completedRowCount=1`. |
| 3m large-v3-turbo CUDA | 5.024-5.327s | 10.795-11.894s | 2.083-2.367x | True `candle-whisper-active-row-tensor-batch`; `chunkCount=5`, `batchCount=1`, `effectiveActiveBatchSizes=1,3,4,5`, `activeRowCompactionCount=3`, `completedRowCount=5`. |
| 10m large-v3-turbo CUDA | 19.408-20.360s | 21.286-21.974s | 1.046-1.132x | True `candle-whisper-active-row-tensor-batch`; `chunkCount=20`, `batchCount=3`, `effectiveActiveBatchSizes=1,2,3,4,5,6,7,8,9,10`, `activeRowCompactionCount=19`, `completedRowCount=24`. |

The useful historical conclusion was narrow: wrapper-level batching was
insufficient. The repaired path now keeps the safe KV-cache fallback for
single-row work and uses active-row tensor batching for eligible multi-window
CUDA cases. The manual full-workflow throughput gate can be restored around the
same command shape once #75 lands, with the existing caveat that this remains a
local/report-only CUDA benchmark rather than a normal GitHub Actions check.

## Runtime Work

The durable fix belongs in the Candle Whisper runtime boundary, not in the
parity harness. The restored path now:

- Provides an autoregressive generation path that reuses self-attention KV cache
  correctly across token steps for each active row.
- Preserves cross-attention state for the current audio feature batch without
  rebuilding it for every token.
- Tracks per-row positions, generated token buffers, timestamp-token state, and
  fallback state explicitly.
- Compacts finished rows out of the active batch so completed windows do not
  continue to pay decode cost.
- Emits `batchExecution=candle-whisper-active-row-tensor-batch` only when real
  active-row batching is active; keeps
  `batchExecution=candle-whisper-autoregressive-kv-cache` for safe fallback
  cases.
- Microbatches CUDA encoder windows before stacking encoder features for the
  active-row decoder, which avoids the 8 GiB CUDA OOM observed in the 10 minute
  rung.

The benchmark gate can be restored with a minimal faster-than-WhisperX
threshold such as `nativeSpeedupRatio >= 1.001`.

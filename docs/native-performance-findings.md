# Native Performance Findings

This note records why the Rust-Native benchmark ladder is no longer a merge
gate for the current parity milestone, and what has to change before the gate
can come back.

## Status

Correctness parity continues to use Python WhisperX as a reference oracle. The
30 second, 3 minute, and 10 minute large-v3-turbo CUDA ladder remains useful as
a manual throughput report, but it is not a blocking GitHub Actions gate for
the foreseeable future.

The blocked runtime work is tracked separately:

- #34: umbrella for real native Candle Whisper long-form batching.
- #35: efficient autoregressive generation with proper decoder KV-cache
  behavior.
- #36: active-row compaction for batched generation.
- #37: restoring the native-vs-WhisperX performance gate after the runtime work
  lands.

## Measurements

All measurements below used the same local benchmark shape unless noted:

```bash
SMOKE_ROOT="$PWD/.smoke"
ORT_DYLIB_PATH="$PWD/.audio-tools/onnxruntime/lib/libonnxruntime.so.1.26.0"
LD_LIBRARY_PATH="$PWD/.audio-tools/onnxruntime/lib:$LD_LIBRARY_PATH"
cargo run -p native-whisperx-cli \
  --features whisperx-compat,media-decode,silero-vad,pyannote-vad,pyannote-diarization,cuda \
  -- parity-bench tests/parity/rust-native-bench-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command /home/moenarch/miniconda3/envs/whisperx/bin/whisperx \
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

The useful conclusion is narrow: wrapper-level batching is insufficient. The
current path groups chunks into nominal batches but still does autoregressive
decode in a shape that recomputes too much work and keeps finished rows in the
rectangular token tensor.

## Required Runtime Work

The durable fix belongs in the Candle Whisper decoder/runtime boundary, not in
the parity harness:

- Provide an autoregressive generation path that reuses self-attention KV cache
  correctly across token steps for each active row.
- Preserve cross-attention state for the current audio feature batch without
  rebuilding it for every token.
- Track per-row positions, generated token buffers, timestamp-token state, and
  fallback state explicitly.
- Compact finished rows out of the active batch so completed windows do not
  continue to pay decode cost.
- Emit `batchExecution=candle-whisper-tensor-batched` only when real tensor
  batching is active; keep `batchExecution=candle-whisper-sequential` for the
  existing fallback path.

After that lands, the benchmark gate can be restored with a minimal
faster-than-WhisperX threshold such as `nativeSpeedupRatio >= 1.001`.

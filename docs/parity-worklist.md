# Native Parity Worklist

This worklist tracks the native Rust replacement path for the Python WhisperX
3.8.6 CLI parity contract. `delegated` behavior remains valid compatibility,
but rows marked `delegated only`, `native partial`, or `blocked by upstream
crate` are not yet complete native parity.

## Status Vocabulary

| Status | Meaning |
| --- | --- |
| `native complete` | The Rust path owns the user-visible behavior. |
| `native partial` | A Rust path exists, but exact WhisperX behavior or fixture coverage is incomplete. |
| `delegated only` | The CLI can reach parity through Python WhisperX, not native Rust. |
| `blocked by upstream crate` | This app crate needs a published dependency API before implementation can be correct. |
| `needs fixture` | Behavior exists or is planned, but needs Python WhisperX golden data or model-backed smoke coverage. |

## CLI Surface

| Area | Native status | Fixture status | Next action |
| --- | --- | --- | --- |
| Multiple input files | native complete | covered by CLI smoke | Keep rejecting `--basename` with multiple inputs. |
| Transcription task | native partial | needs fixture | Expand real ASR parity fixtures before claiming replacement parity. |
| Translation task | native partial | needs fixture | Add real translation smoke/golden fixtures for `--task translate --no-align`; aligned translation remains planned/delegated. |
| Model selection | native partial | needs fixture | Add cache/download and alias fixture coverage for each supported Whisper alias. |
| Model cache | native partial | manual smoke only | Keep ignored `SMOKE_ROOT` smoke; add scheduled/manual run notes per release. |
| Language | native partial | needs fixture | Compare language field in parity reports and add non-English fixture coverage. |
| Device | native partial | manual smoke only | Keep CPU in CI; add CUDA smoke where available. |
| Device index | blocked by upstream crate | none | Add native device-index API upstream before accepting in native mode. |
| Compute type | blocked by upstream crate | none | Add native compute-type or quantization API upstream before accepting in native mode. |
| Batch size | native partial | needs benchmark | Native request maps `--batch_size` to `max_batch_size`; add runtime/resource benchmark before parity claim. |
| Logging/progress | delegated only | fake command covered | Add native progress/logging contract before accepting these as native controls. |
| VAD method | native partial | needs model smoke | Energy is native; Silero is feature-gated; pyannote remains rejected natively. |
| VAD thresholds/chunking | native partial | needs fixture | Add Silero and Python WhisperX VAD segment goldens. |
| Native VAD model wiring | native partial | mocked/compile only | Add real Silero ONNX smoke and setup diagnostics. |
| Alignment enablement | native complete | fixture/import coverage | Keep default alignment plus `--no-align` behavior covered. |
| Alignment model | native partial | needs fixture | Add Hugging Face cache and alias parity fixture coverage. |
| Interpolation | native complete | unit coverage | Add real alignment timing fixture before release parity claim. |
| Character alignments | native partial | fixture/import coverage | Parity reports now compare char counts; add timing/content goldens next. |
| Diarization | native partial | needs fixture | Heuristic/ONNX native paths need pyannote-compatible contract and fixtures. |
| Diarization model | delegated only | fake command covered | Keep native semantics blocked until a pyannote-compatible route exists. |
| Hugging Face token | delegated only | manual only | Define native model access semantics before accepting for native diarization. |
| Speaker bounds | native partial | needs fixture | Add two-speaker and bounds fixtures for native diarization. |
| Speaker embeddings | delegated only | fake command covered | Keep native output blocked until artifact shape is defined. |
| Decode controls | blocked by upstream crate | unit rejection coverage | Native errors now list each unsupported flag; add upstream APIs before accepting. |
| Subtitle controls | native partial | unit coverage | Generate Python WhisperX SRT/VTT goldens before changing layout behavior. |
| Output formats | native partial | unit coverage | Add Python WhisperX golden outputs for JSON/TXT/SRT/VTT/TSV. |
| Output directory | native complete | unit coverage | Keep output file list stable. |
| Short aliases | native complete | CLI smoke | Keep `-o`, `-f`, and `-P` covered by help/runtime tests. |
| Python-compatible top-level invocation | native complete | CLI smoke | Keep top-level input normalization covered. |

## Manual Parity Commands

Native ASR cache-only:

```bash
cargo test -p native-whisperx-cli \
  --test native_asr_cache_smoke \
  -- --ignored --nocapture
```

Python WhisperX comparison:

```bash
cargo run -p native-whisperx-cli --features whisperx-compat -- parity input.wav \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --whisperx-model tiny.en \
  --align-model facebook/wav2vec2-base-960h \
  --interpolate-method nearest \
  --expected-json expected.json \
  --language en
```

Silero VAD smoke:

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features silero-vad -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --vad-method silero \
  --vad-model-bundle "$SMOKE_ROOT/models/silero-vad" \
  --output-dir out
```

ONNX diarization smoke:

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features onnx-diarization -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --speaker-embedding-bundle "$SMOKE_ROOT/models/wespeaker-voxceleb-resnet34-LM/main" \
  --speaker-embedding-model-file speaker-embedding.onnx \
  --speaker-embedding-dim 256 \
  --output-dir out
```

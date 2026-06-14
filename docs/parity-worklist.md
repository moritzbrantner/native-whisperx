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
| Transcription task | native partial | local fixture harness | Run `tests/parity/asr-fixtures.json` locally before claiming replacement parity. |
| Translation task | native partial | ignored model smoke | Post-ASR Helsinki translation is native; add golden fixtures and broaden beyond German-to-English before claiming full parity. |
| Translation model | native partial | non-gating local fixture probe plus ignored model smoke | Keep `marian_translation_external` manual smoke; use `small-de-translate-cache` as cache-only wrapper coverage for `--translation-model` once expanded local resources are present. |
| Model selection | native partial | local fixture harness | Starter suite covers `tiny.en` and `small`; add more aliases as local fixtures mature. |
| Model cache | native partial | manual smoke plus local suite | Keep ignored `SMOKE_ROOT` smoke and run the local fixture suite per release. |
| Language | native partial | local fixture harness plus non-gating expansion probe | Explicit English and language-detection cases are gating; `small-de-no-align-cache` tracks non-English ASR as a non-gating local-resource probe. |
| Device | native partial | manual smoke only | Keep CPU in CI; add CUDA smoke where available. |
| Device index | blocked by upstream crate | none | Add native device-index API upstream before accepting in native mode. |
| Compute type | blocked by upstream crate | none | Add native compute-type or quantization API upstream before accepting in native mode. |
| Batch size | native partial | needs benchmark | Native request maps `--batch_size` to `max_batch_size`; add runtime/resource benchmark before parity claim. |
| Logging/progress | delegated only | fake command covered | Add native progress/logging contract before accepting these as native controls. |
| VAD method | native partial | full-resource non-gating manifest | Energy is native; Silero is feature-gated and measured in `tests/parity/full-resource-fixtures.json`; pyannote remains rejected natively. |
| VAD thresholds/chunking | native partial | full-resource non-gating manifest | Promote Silero timing/text checks to gating only after local WhisperX goldens pass consistently. |
| Native VAD model wiring | native partial | mocked/compile plus full-resource manifest | Keep real Silero ONNX setup diagnostics host-local until CI has ONNX Runtime provisioning. |
| Alignment enablement | native complete | fixture/import coverage | Keep default alignment plus `--no-align` behavior covered. |
| Alignment model | native partial | local fixture harness plus non-gating expansion probe | Starter suite covers default wav2vec2 alignment; `tiny-en-alignment-alias-cache` tracks `WAV2VEC2_ASR_BASE_960H` alias/cache behavior. |
| Interpolation | native complete | unit coverage | Add real alignment timing fixture before release parity claim. |
| Character alignments | native partial | local fixture harness | Starter suite includes a char-alignment expected JSON path; add timing/content goldens locally. |
| Diarization | native partial | full-resource non-gating manifest | Heuristic/ONNX native paths are measured against pyannote goldens; production parity still needs a pyannote-compatible contract. |
| Diarization model | delegated only | fake command covered | Keep native semantics blocked until a pyannote-compatible route exists. |
| Hugging Face token | delegated only | manual only | Define native model access semantics before accepting for native diarization. |
| Speaker bounds | native partial | full-resource non-gating manifest | Two-speaker bounds are represented in `tests/parity/full-resource-fixtures.json`; keep non-gating until assignment parity stabilizes. |
| Speaker embeddings | delegated only | full-resource non-gating manifest | Python WhisperX embedding output is represented in the full-resource suite; native output remains blocked until artifact shape is defined. |
| Decode controls | blocked by upstream crate | unit rejection coverage | Native errors now list each unsupported flag; add upstream APIs before accepting. |
| Subtitle controls | native partial | unit plus local golden output checks | SRT/VTT writer behavior follows WhisperX 3.8.6 word-cue splitting; local fixtures compare expected subtitle files byte-for-byte. |
| Output formats | native partial | unit plus local golden output checks | TXT/TSV/SRT/VTT/AUD target byte exactness; JSON parity is semantic. Keep adding Python WhisperX goldens as ASR fixtures mature. |
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

Local ASR parity fixture suite:

```bash
cargo run -p native-whisperx-cli -- parity-preflight tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --require-expected
```

```bash
cargo run -p native-whisperx-cli -- parity-goldens tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --overwrite
```

```bash
cargo run -p native-whisperx-cli -- parity-fixtures tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --output-dir "$SMOKE_ROOT/out/parity-fixtures"
```

Full-resource parity fixture suite:

```bash
HF_TOKEN=... \
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features whisperx-compat,silero-vad,onnx-diarization \
  -- parity-fixtures tests/parity/full-resource-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --output-dir "$SMOKE_ROOT/out/full-resource-parity"
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

Helsinki OPUS-MT translation smoke:

```bash
cargo test -p moritzbrantner-text-model-runtime \
  --features marian-translation,external-tests \
  --test marian_translation_external -- --ignored
```

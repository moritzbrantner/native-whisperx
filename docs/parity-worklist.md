# Native Parity Worklist

This worklist tracks the native Rust replacement path for the Python WhisperX
3.8.6 CLI parity contract. `delegated` behavior remains valid compatibility,
but rows marked `delegated only`, `native partial`, or `blocked by upstream
crate` are not yet complete native parity.

Rust-Native Parity is the stricter program lane: new parity work must use this
repository and its vendor code only, with Python WhisperX kept as a reference
oracle rather than an implementation bridge. The existing WhisperX Parity
contract can still document delegated compatibility, but delegated rows are
`reference-only` for the Rust-Native Parity track until a Rust/native path
replaces them.

## Status Vocabulary

| Status | Meaning |
| --- | --- |
| `native complete` | The Rust path owns the user-visible behavior. |
| `native partial` | A Rust path exists, but exact WhisperX behavior or fixture coverage is incomplete. |
| `delegated only` | The CLI can reach parity through Python WhisperX, not native Rust. |
| `blocked by upstream crate` | This app crate needs a published dependency API before implementation can be correct. |
| `needs fixture` | Behavior exists or is planned, but needs Python WhisperX golden data or model-backed smoke coverage. |

Rust-Native Parity completion reports should collapse these rows into
`rust-native complete`, `rust-native partial`, `blocked`, or `reference-only`.

## CLI Surface

| Area | Native status | Fixture status | Next action |
| --- | --- | --- | --- |
| Multiple input files | native complete | covered by CLI smoke | Keep rejecting `--basename` with multiple inputs. |
| Transcription task | native partial | local fixture harness | Core English ASR cache fixtures now gate segment timing, aligned word timing, and char count; keep expansion/output fixtures non-gating until promoted. |
| Translation task | native partial | gating local fixture probe | Post-ASR Helsinki translation runs through the native Marian path for `Helsinki-NLP/opus-mt-de-en`. |
| Translation model | native partial | gating local fixture probe | `small-de-translate-cache` gates `--translation-model`, cache-only model resolution, source/target language, and max-token plumbing. |
| Model selection | native partial | local fixture harness | Starter suite covers `tiny.en` and `small`; add more aliases as local fixtures mature. |
| Model cache | native partial | manual smoke plus local suite | Keep ignored `SMOKE_ROOT` smoke and run the local fixture suite per release. |
| Language | native partial | local fixture harness | Explicit English and English-only model alias inference are gating; `small-de-no-align-cache` gates German language/model-cache coverage but keeps transcript text, segment structure, and VAD structure report-only until non-English decode drift is resolved. |
| Device | native partial | full-resource fixture plus manual smoke | CUDA is the default native build path and full-resource parity requests `--device cuda`; CPU remains available as an explicit fallback. |
| Device index | blocked by upstream crate | none | Add native device-index API upstream before accepting in native mode. |
| Compute type | blocked by upstream crate | none | Add native compute-type or quantization API upstream before accepting in native mode. |
| Batch size | native partial | benchmark report | Native request maps `--batch_size` to `max_batch_size`; collect repeated `parity-bench` baselines before setting any parity gate. |
| Logging/progress | delegated only | fake command covered | Add native progress/logging contract before accepting these as native controls. |
| VAD method | native partial | full-resource gating manifest | Energy is native; Silero and local-ONNX pyannote are feature-gated and measured in `tests/parity/full-resource-fixtures.json` with direct VAD segment comparison. |
| VAD thresholds/chunking | native partial | full-resource gating manifest | Keep deterministic energy VAD timing report-only in ASR fixtures; Silero and pyannote full-resource fixtures gate merged VAD segment timing/count against WhisperX goldens. |
| Native VAD model wiring | native partial | mocked/compile plus full-resource manifest | Keep real Silero/pyannote ONNX setup diagnostics host-local until CI has ONNX Runtime provisioning. |
| Alignment enablement | native complete | fixture/import coverage | Keep default alignment plus `--no-align` behavior covered. |
| Alignment model | native partial | local fixture harness plus non-gating expansion probe | Starter suite covers default wav2vec2 alignment; `tiny-en-alignment-alias-cache` tracks `WAV2VEC2_ASR_BASE_960H` alias/cache behavior. |
| Interpolation | native complete | unit coverage | Add real alignment timing fixture before release parity claim. |
| Character alignments | native partial | local fixture harness | `tiny-en-char-alignments` now gates char count with WhisperX-compatible leading-space projection; keep broader char timing/content coverage local until promoted. |
| Diarization | native partial | full-resource gating manifest | Native pyannote community diarization uses an explicit local bundle and gates two-speaker turn parity against WhisperX goldens. |
| Diarization model | native/delegated | full-resource gating manifest | Native accepts pyannote diarization only with an explicit local bundle; external WhisperX still receives delegated pyannote model IDs. |
| Hugging Face token | delegated only | manual only | Define native model access semantics before accepting for native diarization. |
| Speaker bounds | native partial | full-resource non-gating manifest | Two-speaker bounds are represented in `tests/parity/full-resource-fixtures.json`; keep non-gating until assignment parity stabilizes. |
| Speaker embeddings | native/delegated | full-resource gating manifest | Native pyannote diarization can request speaker embeddings from the explicit pyannote bundle; other native embedding requests remain rejected. |
| Performance benchmark | native partial | `parity-bench` JSON report | Use `native-whisperx parity-bench` for native-vs-WhisperX elapsed time, realtime factor, diagnostics, and batch-path reporting. Do not gate speed until repeated baselines exist. |
| Rust-Native benchmark ladder | needs fixture | `tests/parity/rust-native-bench-fixtures.json` | Prove large-v3-turbo CUDA on 30s, 3m, and 10m Shrek-derived clips with native-only JSON reports, warmups, timeouts, phase diagnostics, and model/runtime reuse counters. |
| Decode controls | blocked by upstream crate | unit rejection coverage | Native errors now list each unsupported flag; add upstream Candle Whisper decode APIs before accepting beam size, temperature, best-of, previous-text conditioning, suppress tokens, or initial prompts. |
| Subtitle controls | native partial | unit plus local golden output checks | SRT/VTT writer behavior follows WhisperX 3.8.6 word-cue splitting; local fixtures compare expected subtitle files byte-for-byte. |
| Output formats | native partial | unit plus local golden output checks | TXT/TSV/SRT/VTT/AUD target byte exactness; JSON parity is semantic. Keep adding Python WhisperX goldens as ASR fixtures mature. |
| Output directory | native complete | unit coverage | Keep output file list stable. |
| Short aliases | native complete | CLI smoke | Keep `-o`, `-f`, and `-P` covered by help/runtime tests. |
| Python-compatible top-level invocation | native complete | CLI smoke | Keep top-level input normalization covered. |

## Manual Parity Commands

`tests/parity/asr-fixtures.json` now gates the proven core ASR timing checks.
`tiny-en-no-align-cache`, `small-en-no-align-cache`, and
`tiny-language-detection` gate segment timing. `small-de-no-align-cache` gates
German language and cache diagnostics only; current native decoding still emits
`Nativa Whisper X` plus an extra short `X` segment against the WhisperX 3.8.6
reference, so German transcript text, segment structure, and VAD structure stay
report-only rather than weakening the broader English ASR gates.
`tiny-en-aligned-cache` and
`tiny-en-alignment-alias-cache` gate segment and word timing, with the alias
case also requiring cache-source diagnostics. `tiny-en-char-alignments` gates
segment timing, word timing, and char count. ASR fixtures keep deterministic
energy VAD timing report-only because those checks belong to the dedicated
full-resource VAD probes. The native path uses an expanded deterministic ASR
window when Whisper timestamp-token segments are unstable, and wav2vec2 CTC
word projection now skips delimiter tokens, includes punctuation spans, and
sets aligned segment bounds from the first and last aligned words.

Output writer fixtures `tiny-output-subtitles-wrap` and
`tiny-output-segment-resolution-chunk` also gate byte-for-byte SRT/VTT goldens.
`tiny-output-all-defaults` requests `all` and gates TXT/VTT/SRT/TSV/AUD
byte-for-byte goldens plus semantic WhisperX transcript JSON output; it does
not request `native-json`, which remains an explicit Rust contract. The
`tiny-output-subtitles-highlight` exact SRT/VTT byte checks stay report-only
because WhisperX highlighted subtitles split cue boundaries at word-level
millisecond timestamps whose exact byte layout still drifts, while the gating
semantic SRT/VTT comparison checks cue text sequence with 0.050s timing
tolerance. The remaining local-resource expansion case is the blocked
translation fixture, `small-de-translate-cache`.

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

Compact fixture summary:

```bash
cargo run -p native-whisperx-cli -- parity-summary "$SMOKE_ROOT/out/parity-fixtures/report.json"
```

Performance benchmark track:

```bash
cargo run -p native-whisperx-cli -- parity-bench tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --iterations 3 \
  --json
```

Rust-Native Parity large-v3-turbo CUDA ladder:

```bash
cargo run -p native-whisperx-cli --features media-decode,silero-vad,pyannote-vad,pyannote-diarization,cuda -- \
  parity-bench tests/parity/rust-native-bench-fixtures.json \
  --root "$SMOKE_ROOT" \
  --native-only \
  --model-cache-only \
  --case-timeout-seconds 900 \
  --json
```

Use `--case shrek-retold-30s-large-v3-turbo-cuda`,
`--case shrek-retold-3m-large-v3-turbo-cuda`, or
`--case shrek-retold-10m-large-v3-turbo-cuda` to select a single rung. The
referenced clips are generated from the local Shrek reference media under
`$SMOKE_ROOT/audio`; generated clips and reports are local artifacts, not
checked-in fixtures. Use `SMOKE_ROOT="$PWD/.smoke"` when keeping them inside the
checkout.

Full-resource parity fixture suite:

```bash
HF_TOKEN=... \
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features whisperx-compat,silero-vad,pyannote-vad,pyannote-diarization,cuda \
  -- parity-fixtures tests/parity/full-resource-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --output-dir "$SMOKE_ROOT/out/full-resource-parity"
```

Add `--require-non-gating-passed` to make non-gating full-resource probes fail
an opt-in run while keeping default offline CI unchanged.

Silero VAD smoke:

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features silero-vad -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --vad-method silero \
  --vad-model-bundle "$SMOKE_ROOT/models/silero-vad" \
  --output-dir out
```

pyannote VAD smoke:

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features pyannote-vad -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --vad-method pyannote \
  --vad-model-bundle "$SMOKE_ROOT/models/pyannote-vad" \
  --vad-model-file segmentation.onnx \
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

Helsinki OPUS-MT translation smoke, in the upstream `rust-packages` workspace
after Marian translation support is published:

```bash
cargo test -p moritzbrantner-text-model-runtime \
  --features marian-translation,external-tests \
  --test marian_translation_external -- --ignored
```

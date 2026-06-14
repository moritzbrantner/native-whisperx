# Model Bundles And Resolution

The parity contract expects model IDs to resolve through Hugging Face model and
cache conventions. Local bundles remain supported for offline and controlled
environments. Native ASR and native alignment resolve supported model IDs
through Hugging Face cache/downloader conventions when explicit bundles are not
supplied. Default CI stays offline and does not download models; real
cache/download parity checks are ignored/manual.

## Whisper

Native ASR can use a local Candle-compatible Whisper bundle:

```text
config.json
generation_config.json
tokenizer.json
preprocessor_config.json
model.safetensors
```

`--whisper-bundle` is the recommended fully offline deterministic path. It has
priority over `--model-dir` for ASR.

Example:

```bash
cargo run -p native-whisperx-cli -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --language en \
  --output-dir out
```

Without `--whisper-bundle`, native ASR resolves `--model` through Hugging Face
cache conventions or downloads when cache-only is not requested. The first
supported target is Candle-compatible OpenAI Whisper safetensors repositories
with these required files:

```text
config.json
generation_config.json
tokenizer.json
preprocessor_config.json
model.safetensors
```

Cache-only example:

```bash
cargo run -p native-whisperx-cli -- transcribe input.wav \
  --model tiny.en \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --language en \
  --output-dir out
```

When `--model-cache-only` is set, native ASR never downloads and reports a setup
error listing the required files if the cache is incomplete. Without
`--model-cache-only`, native ASR may download the required files through the
shared Hugging Face cache.

## Helsinki-NLP OPUS-MT Translation

Native post-ASR translation currently supports Marian/OPUS-MT segment
translation, starting with `Helsinki-NLP/opus-mt-de-en` for German to English.
Accepted aliases are:

```text
Helsinki-NLP/opus-mt-de-en
Helsinki/opus-mt-de-en
opus-mt-de-en
helsinki:de-en
```

Required bundle files:

```text
config.json
generation_config.json
source.spm
target.spm
vocab.json
rust_model.ot or pytorch_model.bin or model.safetensors
```

Example:

```bash
cargo run -p native-whisperx-cli -- input.wav \
  --language de \
  --task translate \
  --translation-model Helsinki-NLP/opus-mt-de-en \
  --model small \
  --model-dir "$SMOKE_ROOT/models" \
  --format srt
```

Use `--translation-bundle` for a fully explicit local bundle. Without it,
translation uses the same `--model-dir` root as native ASR/alignment. With
`--model-cache-only`, the translator never downloads and reports missing bundle
files instead. Without cache-only, the requested Helsinki model may be resolved
through the Hugging Face downloader.

## Manual Native ASR Cache Smoke

This repository includes an ignored wrapper smoke for native ASR Hugging Face
cache resolution. It requires a real audio fixture and a real local
Hugging Face-style cache. `--model-cache-only` is used, so no download should
occur.

```bash
export SMOKE_ROOT=/path/to/smoke-root

cargo test -p native-whisperx-cli \
  --test native_asr_cache_smoke \
  -- --ignored --nocapture
```

Required layout:

```text
$SMOKE_ROOT/
  audio/native-transcription-smoke.wav
  models/models--openai--whisper-tiny.en/snapshots/<snapshot>/
    config.json
    generation_config.json
    tokenizer.json
    preprocessor_config.json
    model.safetensors
```

The smoke runs `--model tiny.en`, `--model-dir "$SMOKE_ROOT/models"`,
`--model-cache-only`, `--language en`, `--no-align`, and `--format json`. The
positive case asserts `asrModelSource=hugging-face-cache`; the negative case
uses an empty model directory and checks that the missing required files are
reported instead of silently downloading or falling back.

## Local ASR Parity Fixtures

The local ASR parity fixture harness compares native ASR against Python
WhisperX with real audio and locally cached models. It is intentionally not run
by default CI. A later workflow can move it into scheduled or labeled CI after a
runner has prewarmed model caches and any required secrets.

## Local WhisperX Source Reference

Python WhisperX source can be kept as optional local tooling for parity
inspection. Clone it under the ignored `.audio-tools/` directory and pin it to
the current parity baseline:

```bash
mkdir -p .audio-tools
git clone --branch v3.8.6 --depth 1 \
  https://github.com/m-bain/whisperX.git \
  .audio-tools/whisperx-src
```

If the checkout already exists, refresh and detach it at the pinned tag:

```bash
git -C .audio-tools/whisperx-src fetch --tags origin v3.8.6
git -C .audio-tools/whisperx-src checkout --detach v3.8.6
```

Use `.audio-tools/whisperx-src` only as a read-only reference for CLI defaults,
transcription flow, ASR batching, alignment, diarization, and output writer
parity. Do not commit the clone, vendor it, or use it as a runtime dependency.
Update this tag only when `docs/parity-matrix.md` intentionally moves the
WhisperX parity baseline.

Preflight local resources before running model-heavy parity work:

```bash
export SMOKE_ROOT=/path/to/smoke-root

cargo run -p native-whisperx-cli -- parity-preflight tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --require-expected
```

Generate or refresh ignored Python WhisperX 3.8.6 goldens from the manifest:

```bash
cargo run -p native-whisperx-cli -- parity-goldens tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --overwrite
```

Run the starter suite:

```bash
cargo run -p native-whisperx-cli -- parity-fixtures tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --output-dir "$SMOKE_ROOT/out/parity-fixtures"
```

Required layout:

```text
$SMOKE_ROOT/
  audio/native-transcription-smoke.wav
  expected/
    tiny-en-aligned-cache.whisperx.json
    tiny-en-char-alignments.whisperx.json
    whisperx-3.8.6/
      tiny-output-all-defaults.json
      tiny-output-all-defaults.txt
      tiny-output-all-defaults.vtt
      tiny-output-all-defaults.srt
      tiny-output-all-defaults.tsv
      tiny-output-subtitles-highlight.vtt
      tiny-output-subtitles-highlight.srt
      tiny-output-subtitles-wrap.vtt
      tiny-output-subtitles-wrap.srt
      tiny-output-segment-resolution-chunk.vtt
      tiny-output-segment-resolution-chunk.srt
  models/
    models--openai--whisper-tiny.en/snapshots/<snapshot>/
      config.json
      generation_config.json
      tokenizer.json
      preprocessor_config.json
      model.safetensors
    models--openai--whisper-small/snapshots/<snapshot>/
      config.json
      generation_config.json
      tokenizer.json
      preprocessor_config.json
      model.safetensors
    models--facebook--wav2vec2-base-960h/snapshots/<snapshot>/
      config.json
      tokenizer.json or vocab.json
      preprocessor_config.json
      model.safetensors
```

The ASR manifest also contains non-gating expansion probes for the next parity
wave. They are reported by default without failing the suite, and become
preflight-enforced when `--include-non-gating` is passed. To run those probes,
add:

```text
$SMOKE_ROOT/
  audio/native-transcription-smoke-de.wav
  audio/native-translation-de.wav
  expected/whisperx-3.8.6/
    tiny-en-alignment-alias-cache.json
    small-de-translate-cache.json
  models/
    models--Helsinki-NLP--opus-mt-de-en/snapshots/<snapshot>/
      config.json
      generation_config.json
      source.spm
      target.spm
      vocab.json
      rust_model.ot or pytorch_model.bin or model.safetensors
```

Those probes cover non-English ASR, the WhisperX
`WAV2VEC2_ASR_BASE_960H` alignment alias, and native Helsinki-NLP post-ASR
translation compared against Python WhisperX `--task translate`.

The parity harness compares TXT/TSV/SRT/VTT/AUD files exactly and compares JSON
semantically, so JSON whitespace does not matter. Keep these generated goldens
inside `SMOKE_ROOT`; do not commit them unless a future tiny checked-in fixture
is intentionally added.

## Opt-In Parity Workflow

`.github/workflows/parity-fixtures.yml` provides an opt-in real-resource
workflow for self-hosted or otherwise prewarmed parity runners. It does not run
on ordinary pushes. It can run by manual dispatch, by the weekly schedule when
the repository variable `PARITY_SMOKE_ROOT` is set, or on same-repository pull
requests labeled `run-parity-fixtures`.

Configure these repository variables for scheduled or labeled runs:

```text
PARITY_SMOKE_ROOT=/path/to/smoke-root
PARITY_WHISPERX_COMMAND=.audio-tools/whisperx-venv/bin/whisperx
PARITY_RUNNER=self-hosted
RUST_PACKAGES_REF=main
```

The workflow checks out this repository and `moritzbrantner/rust-packages` as a
sibling directory named `rust-packages-native-whisper-task`, matching the
temporary Cargo patch path used by this workspace. Manual dispatch can choose
the ASR or full-resource suite, opt into non-gating probes, and optionally
refresh ignored goldens under `SMOKE_ROOT`.

Run the full-resource parity suite when gated Hugging Face and ONNX Runtime
resources are available:

```bash
export SMOKE_ROOT=/path/to/smoke-root
export HF_TOKEN=...
export ORT_DYLIB_PATH=/path/to/libonnxruntime.so

cargo run -p native-whisperx-cli --features whisperx-compat,silero-vad,onnx-diarization \
  -- parity-fixtures tests/parity/full-resource-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --output-dir "$SMOKE_ROOT/out/full-resource-parity"
```

The full-resource suite is non-gating while native Silero and diarization
contracts are still being measured against Python WhisperX.

## wav2vec2 Alignment

Native alignment can use a supported `Wav2Vec2ForCTC` bundle:

```text
config.json
tokenizer.json or vocab.json
preprocessor_config.json
model.safetensors
```

Example:

```bash
cargo run -p native-whisperx-cli -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --alignment-bundle "$SMOKE_ROOT/models/wav2vec2-base-960h/main" \
  --output-dir out
```

Without `--alignment-bundle`, native alignment resolves `--align-model` through
Hugging Face cache conventions. The default is
`facebook/wav2vec2-base-960h`; the WhisperX alias
`WAV2VEC2_ASR_BASE_960H` maps to the same model.

```bash
cargo run -p native-whisperx-cli -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --align-model facebook/wav2vec2-base-960h \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --return-char-alignments \
  --output-dir out
```

Alignment writes `segments[].words` and top-level `word_segments` in WhisperX
JSON. Character timings are opt-in with `--return-char-alignments` and are
written as `segments[].chars`. Missing timestamps use `--interpolate-method`
with `nearest`, `linear`, or `ignore`.

## ONNX Speaker Embeddings

ONNX diarization is explicit and requires a local ONNX Runtime setup.

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features onnx-diarization -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --speaker-embedding-bundle "$SMOKE_ROOT/models/wespeaker-voxceleb-resnet34-LM/main" \
  --speaker-embedding-model-file speaker-embedding.onnx \
  --speaker-embedding-dim 256 \
  --output-dir out
```

Callers that pass explicit bundle paths own those files and their runtime setup.
When callers pass native ASR or alignment model IDs, native-whisperx resolves
them through the standard Hugging Face cache rather than an app-private bundle
format. The external Python WhisperX provider remains delegated and receives
the same `--model_dir` and `--model_cache_only` flags.

## Silero VAD ONNX

Native Silero VAD is opt-in with the `silero-vad` Cargo feature and requires a
local ONNX model supplied by the caller. A directory bundle should contain:

```text
silero_vad.onnx
```

The bundle can also point directly at an `.onnx` file. Use
`--vad-model-file` when the file inside a directory has a non-default name, and
`--vad-input-name` / `--vad-output-name` only for models whose tensor names do
not match the standard Silero ONNX layout.

Example:

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features silero-vad -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --vad-method silero \
  --vad-model-bundle "$SMOKE_ROOT/models/silero-vad" \
  --output-dir out
```

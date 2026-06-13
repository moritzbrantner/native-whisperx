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

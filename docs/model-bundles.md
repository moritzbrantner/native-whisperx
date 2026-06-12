# Model Bundles And Resolution

The parity contract expects model IDs to resolve through Hugging Face model and
cache conventions. Local bundles remain supported for offline and controlled
environments. Native alignment now resolves wav2vec2 model IDs through the
Hugging Face cache/downloader path; broader ASR model resolution still depends
on explicit Whisper bundles.

## Whisper

Native ASR can use a local Candle-compatible Whisper bundle:

```text
config.json
generation_config.json
tokenizer.json
preprocessor_config.json
model.safetensors
```

Example:

```bash
cargo run -p native-whisperx-cli -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --language en \
  --output-dir out
```

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
When callers pass alignment model IDs, native-whisperx resolves them through the
standard Hugging Face cache rather than an app-private bundle format.

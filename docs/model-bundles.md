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

### Q8 CPU bundle

Native Q8 ASR is an explicit local-bundle route. The bundle must contain these
regular files:

```text
config.json
generation_config.json
tokenizer.json
preprocessor_config.json
model.q8_0.gguf
```

Inspect a prepared bundle without loading model weights:

```bash
cargo run -p native-whisperx-cli -- inspect-models \
  --device cpu \
  --compute-type int8 \
  --whisper-bundle /path/to/q8-bundle \
  --no-align
```

Run the supported workflow with:

```bash
cargo run -p native-whisperx-cli -- transcribe input.wav \
  --provider native \
  --device cpu \
  --compute-type int8 \
  --whisper-bundle /path/to/q8-bundle \
  --language en \
  --no-align \
  --format json \
  --report /path/to/raw-report.json \
  --output-dir /path/to/output
```

Q8 does not use remote model resolution or automatic download. The local Q8
route is CPU-only; alignment, diarization, translation, and CUDA are rejected
before transcription. The command still uses native-whisperx's
enabled-by-default energy VAD to segment the input before ASR.

### Manual Q8 CPU evidence

The opt-in Q8 evidence runner is a same-host comparison against FP32, not a
host-specific absolute-time gate. It requires caller-owned Shrek
Retold-derived one-second and 15-second WAV clips plus two matched bundles:

- the Q8 bundle contains `model.q8_0.gguf`;
- the FP32 bundle contains `model.safetensors`;
- both bundles contain byte-identical copies of `config.json`,
  `generation_config.json`, `tokenizer.json`, and
  `preprocessor_config.json`.

Build the CLI, then place all generated evidence under the ignored
`.q8-cpu-evidence/` directory:

```bash
cargo build --release -p native-whisperx-cli

python3 scripts/q8_cpu_evidence.py run \
  --binary target/release/native-whisperx \
  --q8-bundle /path/to/q8-bundle \
  --fp32-bundle /path/to/matched-fp32-bundle \
  --one-second-wav /path/to/shrek-retold-1s.wav \
  --fifteen-second-wav /path/to/shrek-retold-15s.wav \
  --raw-report .q8-cpu-evidence/raw.json \
  --summary .q8-cpu-evidence/summary.json
```

For each clip and mode, the runner performs one warm-up and three measured
process runs through `transcribe --provider native --device cpu --no-align`.
Q8 uses `--compute-type int8`; FP32 uses `--compute-type float32`. The leading
mode alternates for each paired warm-up or measurement, and the report records
the order.

Every measurement must produce valid WhisperX JSON and records wall-clock,
realtime factor based on reported ASR time, model-load, encoder, decoder, and
ASR durations, generated-token count, and timestamp-fallback status. The
sanitized summary also records exact transcript-text equality for each Q8/FP32
pair, safe SHA-256 bundle hashes, and the three-run median reported
`asrSeconds` for each mode. The comparative gate requires:

- the 15-second Q8 median to be at most `0.90 * FP32`;
- the one-second Q8 median to be at most `1.10 * FP32`.

The CPU model is retained as evidence provenance, but the gate applies on every
selected self-hosted CPU runner and does not require an i5-6300U.

The raw report contains full native reports and machine-local paths. It must
remain uncommitted; the directory is ignored and the manual workflow retains
the raw file only as a workflow artifact. `summary.json` is produced by an
explicit whitelist. To sanitize an existing raw artifact separately:

```bash
python3 scripts/q8_cpu_evidence.py sanitize \
  .q8-cpu-evidence/raw.json \
  /path/to/q8-cpu-summary.json
```

Only the sanitized summary is commit-eligible. Do not check in synthetic or
fixture-generated timing as performance evidence.

Maintainers can dispatch `.github/workflows/q8-cpu-evidence.yml`, select the
self-hosted runner label with the `runner` workflow input, and configure
`NATIVE_WHISPERX_Q8_BUNDLE`, `NATIVE_WHISPERX_FP32_BUNDLE`,
`NATIVE_WHISPERX_Q8_WAV_1S`, and `NATIVE_WHISPERX_Q8_WAV_15S` as repository
secrets containing runner-local paths. The workflow verifies resource types
before building, while the runner verifies exact model filenames and identical
sidecars. It retains raw and sanitized reports as separate 90-day artifacts and
never prints the configured resource paths.

This Q8 evidence is a **CPU ASR diagnostic**: the command retains the default
energy VAD/segmentation step, but the recorded model-load, encoder, decoder,
token, timestamp-fallback, and ASR measurements characterize the explicit Q8
CPU route with alignment disabled. It is not the **Full Workflow Throughput
Gate**, which measures the complete VAD, ASR, alignment, and output workflow
against the WhisperX reference on CUDA. Q8 evidence does not replace, relax, or
redefine that gate.

## Helsinki-NLP OPUS-MT Translation

Native post-ASR translation uses Marian/OPUS-MT segment translation, starting
with `Helsinki-NLP/opus-mt-de-en` for German to English. Accepted aliases are:

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
model.safetensors or pytorch_model.bin
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

The `--translation-bundle` path uses a fully explicit local bundle. Without it,
translation uses the same `--model-dir` root as native ASR/alignment. The
`small-de-translate-cache` parity fixture is gating.

The public-provider direct and English-pivot evidence test is opt-in and
cache-only. It verifies the pinned model revisions and weight hashes from
`tests/fixtures/real-opus-mt-translation.json`, source immutability, ordered
progress, and per-leg plus total translation timing without downloading:

```bash
RUN_NATIVE_TRANSLATION_TESTS=1 \
NATIVE_WHISPERX_OPUS_MT_CACHE=/path/to/pinned/hugging-face-cache \
NATIVE_WHISPERX_TRANSLATION_REPORT=/path/to/translation-report.json \
cargo test -p native-whisperx --test native_opus_mt_provider \
  public_native_provider_executes_pinned_legacy_pickle_direct_and_pivot_models \
  -- --ignored --exact --nocapture
```

Ordinary CI parses the fixture and exercises the same public provider with an
empty cache and cooperative cancellation, but never downloads or loads a real
model. `small-de-translate-cache` gates cache/config diagnostics and keeps
native-versus-WhisperX transcript differences report-only because the two
paths use different translation runtimes.

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

## Automatic Native Diarization Resources

Automatic Workflow Selection is a Workflow Composition behavior. It is not a
new WhisperX JSON field, not a WhisperX Parity claim by itself, and not the
same thing as Rust-Native Parity. For native finite transcription, a plain
`--diarize` request with no explicit lower-level VAD or diarization model
settings selects the quality-preserving pyannote pair:

```text
pyannote VAD: pyannote/segmentation-3.0
diarization: pyannote/speaker-diarization-community-1
```

Automatic selection must resolve both resources before transcription starts.
If either the VAD or diarization resource is missing, native-whisperx fails
before transcription with setup guidance. It does not fall back to energy VAD,
Silero VAD, heuristic diarization, or external Python WhisperX delegation.

Lookup order for automatic native `--diarize` is:

1. The configured model directory from `--model-dir`.
2. Standard Hugging Face cache roots, including `HF_HOME/hub` when `HF_HOME`
   is set, otherwise `$HOME/.cache/huggingface/hub`.
3. The future download path when cache-only mode is false.

The current native automatic pyannote download boundary is intentionally
stricter than that final lookup order: download lookup is not yet wired to a
concrete pyannote bundle hydrator. When `--model-cache-only` is not set and the
resources are still missing from `--model-dir` and the standard Hugging Face
cache, the run still fails before transcription and says that automatic
pyannote download is not currently wired. Prepare local resources or pre-cache
compatible resources until that hydrator exists.

`--model-cache-only` is a hard no-download guarantee. In cache-only mode,
missing automatic pyannote VAD or pyannote community diarization resources fail
before transcription and no network request is attempted. This guarantee
applies to ordinary CLI runs, parity fixture runs, and maintainer smoke checks.

Native automatic selection uses environment or standard Hugging Face auth state
for future/prepared cache workflows. Do not pass Hugging Face token strings as
the native automatic-download interface, and do not expose token values in
commands, reports, diagnostics, or committed docs. Python WhisperX reference
runs may still require `HF_TOKEN` in the environment for gated pyannote
resources.

Prepare local automatic-selection resources under `--model-dir` using either
the direct model-id directory form or the standard Hugging Face cache form. The
resolver accepts a model-dir-local directory such as:

```text
$SMOKE_ROOT/models/
  pyannote--segmentation-3.0/
    segmentation.onnx
    pyannote_vad_manifest.json
    MODEL_PROVENANCE.md
  pyannote--speaker-diarization-community-1/
    pyannote_diarization_manifest.json
    segmentation.onnx
    embedding.onnx
    plda_transform.json
    plda_model.json
    clustering.json
    MODEL_PROVENANCE.md
```

It also accepts Hugging Face cache snapshots such as:

```text
$SMOKE_ROOT/models/
  models--pyannote--segmentation-3.0/
    refs/main
    snapshots/<snapshot>/
      segmentation.onnx
      pyannote_vad_manifest.json
  models--pyannote--speaker-diarization-community-1/
    refs/main
    snapshots/<snapshot>/
      pyannote_diarization_manifest.json
      segmentation.onnx
      embedding.onnx
      plda_transform.json
      plda_model.json
      clustering.json
```

Keep provenance beside local ONNX exports. The files above are runtime
resources, not Cargo package contents and not default CI requirements.

### Verified native pyannote VAD snapshots

`pyannote/segmentation-3.0` is access-gated even though its source license is
MIT. The repository-owned converter downloads only the pinned revision through
the operator's standard Hugging Face login or cache; it never accepts a token
argument and never writes weights into this repository.

On an authenticated conversion runner, create the immutable local bundle:

```bash
python scripts/convert_pyannote_segmentation.py \
  --output-root "${XDG_CACHE_HOME:-$HOME/.cache}/native-whisperx/model-bundles"
```

The converter pins `pyannote/segmentation-3.0` revision
`e66f3d3b9eb0873085418a7b813d3b369bf160bb`, records the source and generated
SHA-256 hashes, performs a deterministic PyTorch-versus-ONNX comparison, and
writes `segmentation.onnx`, `pyannote_vad_manifest.json`, and
`MODEL_PROVENANCE.md` beneath a Hugging Face-style `snapshots/<revision>`
directory. It requires `torch`, `pyannote.audio`, `onnx`, and `onnxruntime` in
the conversion environment. The exact resolved package versions are written
into each generated manifest.

Verify a derived snapshot without network access before installing it beneath
`--model-dir`:

```bash
native-whisperx bundle-verify --kind pyannote-vad --bundle \
  "${XDG_CACHE_HOME:-$HOME/.cache}/native-whisperx/model-bundles/models--pyannote--segmentation-3.0/snapshots/e66f3d3b9eb0873085418a7b813d3b369bf160bb"
```

Automatic native diarization treats a VAD snapshot as ready only after this
offline verification passes. A missing manifest, wrong pinned revision,
incomplete provenance, or checksum mismatch remains an actionable resource
error; native Workflow Composition does not fall back to another VAD.

Automatic cache-only smoke command for a prepared machine:

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli -- transcribe "$SMOKE_ROOT/audio/two-speaker.wav" \
  --model tiny.en \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --language en \
  --diarize \
  --min-speakers 2 \
  --max-speakers 2 \
  --output-dir "$SMOKE_ROOT/out/automatic-diarize-cache"
```

Boundary check for the current download-not-wired behavior:

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli -- transcribe "$SMOKE_ROOT/audio/two-speaker.wav" \
  --model tiny.en \
  --model-dir "$SMOKE_ROOT/empty-models" \
  --language en \
  --diarize \
  --min-speakers 2 \
  --max-speakers 2 \
  --output-dir "$SMOKE_ROOT/out/automatic-diarize-download-boundary"
```

Until the hydrator exists, that second command should fail before transcription
with a missing automatic pyannote VAD and diarization message, `cache-only=false`,
and the note that native automatic pyannote download is not currently wired.

## Manual Real FFmpeg Finite Media Evidence

The real FFmpeg finite media evidence smoke is an ignored maintainer check for
the guaranteed common non-WAV media set. It generates MP3, M4A, AAC, FLAC, OGG,
OPUS, MP4, MOV, MKV, and WebM fixtures from the same spoken WAV in a temporary
directory. The native finite Workflow Composition runs to completion for the
WAV baseline and every generated file; no binary media fixtures or fabricated
model results are committed.

Run it after changing finite media decode wiring, updating FFmpeg/audio I/O
dependencies, or validating a release environment's runtime media support.
`SMOKE_ROOT` must contain:

- `audio/native-transcription-smoke.wav`, a real spoken English WAV;
- `models/`, a Hugging Face cache containing the `tiny.en` files described by
  the [manual native ASR cache smoke](#manual-native-asr-cache-smoke).

Then run:

```bash
SMOKE_ROOT=/path/to/smoke-root \
RUN_NATIVE_FFMPEG_MEDIA_DECODE_SMOKE=1 \
NATIVE_FFMPEG_MEDIA_EVIDENCE_REPORT=/path/to/finite-media-evidence.json \
cargo test -p native-whisperx-cli \
  --test real_ffmpeg_media_decode_smoke \
  -- --ignored --nocapture
```

The preflight checks `ffmpeg` and `ffprobe` on `PATH` plus every required encoder
and muxer before generating fixtures. A failure lists all missing tools, codecs,
and containers and points to the FFmpeg inventory commands needed to inspect
the installation.

Every input uses the normal finite native `transcribe` workflow with
`--model-cache-only`, `--language en`, `--no-align`, and `--format json`.
Normalized transcript text must equal the WAV baseline, segment counts must
match, and each segment start/end must remain within 0.25 seconds of the
baseline. The JSON evidence records native predecode time, pipeline decode time,
their decode-only total, end-to-end workflow time, decoded sample count, sample
rate, channel count, decode route, and comparison results for every format. It
is always printed with `--nocapture`; set
`NATIVE_FFMPEG_MEDIA_EVIDENCE_REPORT` to persist it to a chosen path.

This check remains opt-in/self-hosted because the spoken WAV and real cached
model are external resources. It does not require CUDA, Python WhisperX,
network access, or Hugging Face credentials during the run.

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
      model.safetensors or pytorch_model.bin
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
on ordinary pushes. It can run by manual dispatch, on the nightly schedule when
`PARITY_SMOKE_ROOT` is configured, or on same-repository pull requests labeled
`run-parity-fixtures`.

Configure these repository variables for labeled runs:

```text
PARITY_SMOKE_ROOT=/path/to/smoke-root
PARITY_WHISPERX_COMMAND=.audio-tools/whisperx-venv/bin/whisperx
PARITY_RUNNER=self-hosted
```

The workflow uses the published crates.io dependency graph from this
repository. Manual dispatch can choose the ASR or full-resource suite, opt into
non-gating probes, and optionally refresh ignored goldens under `SMOKE_ROOT`.
Each run uploads `summary.json`, `preflight.json`, `report.json` when fixture
execution starts, and `progress.log`. Start with `summary.json` to separate
gating failures, non-gating/report-only failures, skipped preflight cases, and
execution errors before opening the raw report or progress log.

Run the full-resource parity suite when gated Hugging Face, prepared automatic
pyannote cache resources, and ONNX Runtime resources are available:

```bash
export SMOKE_ROOT=/path/to/smoke-root
export HF_TOKEN=...
export ORT_DYLIB_PATH=/path/to/libonnxruntime.so

cargo run -p native-whisperx-cli --features whisperx-compat,silero-vad,pyannote-vad,pyannote-diarization,cuda \
  -- parity-fixtures tests/parity/full-resource-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --output-dir "$SMOKE_ROOT/out/full-resource-parity"
```

The full-resource suite gates native Silero, pyannote VAD, and pyannote
diarization contracts against Python WhisperX where the fixture marks a case as
gating. The `diarization-two-speaker-pyannote-reference` case exercises
automatic native `--diarize` resource lookup with cache-only enabled when run
with the command above. Omit `--model-cache-only` only for the manual
download-boundary check; today that path should still fail before transcription
if the automatic pyannote resources are absent because the pyannote download
hydrator is not wired yet.

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
MODEL_PROVENANCE.md
```

The bundle can also point directly at an `.onnx` file. Use
`--vad-model-file` when the file inside a directory has a non-default name, and
`--vad-input-name` / `--vad-output-name` only for models whose tensor names do
not match the standard Silero ONNX layout. Local full-resource parity expects
the default smoke-root path:

```text
$SMOKE_ROOT/models/silero-vad/silero_vad.onnx
```

Record the source repository, revision, SHA256, and export/download command in
`MODEL_PROVENANCE.md`. Native parity compares WhisperX-compatible merged VAD
chunks; it does not require raw probability equality with Python WhisperX,
which loads the Torch Hub Silero model in 3.8.6.

Example:

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features silero-vad -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --vad-method silero \
  --vad-model-bundle "$SMOKE_ROOT/models/silero-vad" \
  --output-dir out
```

## pyannote VAD ONNX

Native pyannote VAD can be selected explicitly with `--vad-method pyannote` or
selected automatically by native finite `--diarize` when lower-level choices
are unspecified. It requires the `pyannote-vad` Cargo feature and a local ONNX
segmentation model supplied by the caller or found through automatic resource
lookup. A directory bundle should contain:

```text
segmentation.onnx
pyannote_vad_manifest.json
MODEL_PROVENANCE.md
```

The manifest is required for automatic cache lookup and records the pinned
source revision, conversion environment, checksums, tensor contract, and
deterministic PyTorch-versus-ONNX comparison. Local full-resource parity
expects:

```text
$SMOKE_ROOT/models/pyannote-vad/segmentation.onnx
```

Example:

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features pyannote-vad -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --vad-method pyannote \
  --vad-model-bundle "$SMOKE_ROOT/models/pyannote-vad" \
  --vad-model-file segmentation.onnx \
  --output-dir out
```

## pyannote Diarization ONNX

Native pyannote diarization can be selected explicitly with
`--diarize-model pyannote/speaker-diarization-community-1` plus a bundle, or
selected automatically by native finite `--diarize` when lower-level choices
are unspecified. It requires the `pyannote-diarization` Cargo feature and a
local pyannote community bundle supplied by the caller or found through
automatic resource lookup. The full-resource fixture expects:

```text
$SMOKE_ROOT/models/pyannote-diarization/pyannote_diarization_manifest.json
$SMOKE_ROOT/models/pyannote-diarization/segmentation.onnx
$SMOKE_ROOT/models/pyannote-diarization/embedding.onnx
$SMOKE_ROOT/models/pyannote-diarization/plda_transform.json
$SMOKE_ROOT/models/pyannote-diarization/plda_model.json
$SMOKE_ROOT/models/pyannote-diarization/clustering.json
```

Example:

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
cargo run -p native-whisperx-cli --features pyannote-diarization -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --diarize \
  --diarize-model pyannote/speaker-diarization-community-1 \
  --diarization-model-bundle "$SMOKE_ROOT/models/pyannote-diarization" \
  --min-speakers 2 \
  --max-speakers 2 \
  --output-dir out
```

# native-whisperx

`native-whisperx` is a Rust-first WhisperX parity workflow repository built
from the published `moritzbrantner-*` Rust crates.

This repository owns application composition:

- CLI commands
- workflow configuration
- output file writing
- parity checks against Python WhisperX
- setup documentation for local model bundles and Hugging Face cache resolution

It does not own the reusable ASR, transcript, model-runtime, alignment, or
speaker primitives. Those remain in
[`rust-packages`](https://github.com/moritzbrantner/rust-packages) and are
consumed here as published crates.

## Crate Layout

```text
crates/native-whisperx      # reusable workflow library
crates/native-whisperx-cli  # native-whisperx CLI binary
```

## Cargo Install

The first Cargo release keeps the two-crate shape. Publish
`native-whisperx` first, then publish `native-whisperx-cli`. The CLI crate is
the Cargo install package, and it installs the `native-whisperx` terminal
command:

```bash
cargo install native-whisperx-cli
native-whisperx --version
native-whisperx --help
native-whisperx speakers path --scope local
```

These installed-binary smoke commands are no-resource offline checks. They do
not transcribe media, download models, use CUDA, call Python WhisperX, read
Hugging Face credentials, or require a local smoke media root.

Installed transcription commands use the same local model bundle, cache, and
resource requirements as repository examples. Install success only proves that
the CLI package and `native-whisperx` command are available; transcription
readiness still depends on the requested Whisper, alignment, diarization, VAD,
translation, CUDA, Python WhisperX compatibility, and gated Hugging Face
resources. Delegated Feature paths remain delegated until replaced by explicit
Rust-Native Parity work.

## Workflow

```text
media or samples
  -> moritzbrantner-audio-analysis-transcription
  -> moritzbrantner-text-transcripts::TranscriptionContract
  -> default wav2vec2 alignment unless disabled
  -> optional speaker diarization
  -> optional segment-level post-ASR translation
  -> WhisperX JSON, Native JSON, SRT, WebVTT, or plain text outputs
```

## Feature Flags

| Feature | Purpose |
| --- | --- |
| `native` | Native Candle Whisper and wav2vec2 alignment composition. Enabled by default. |
| `translation` | Helsinki-NLP OPUS-MT/Marian post-ASR segment translation. Enabled by `native`. |
| `cuda` | CUDA-backed Candle execution. Opt in when a local CUDA toolchain is available. |
| `media-decode` | Opt-in non-WAV media/container decode through the audio I/O crate. |
| `diarization` | Heuristic speaker diarization composition. |
| `onnx-diarization` | Explicit ONNX speaker embedding diarization path. |
| `pyannote-diarization` | Explicit native pyannote community diarization bundle path. |
| `whisperx-compat` | External Python WhisperX command compatibility and parity checks. |

## Commands

Import existing WhisperX JSON:

```bash
cargo run -p native-whisperx-cli -- import-whisperx \
  tests/fixtures/whisperx-parity-sample.json
```

Inspect the request shape for local model bundles:

```bash
cargo run -p native-whisperx-cli -- inspect-models \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --alignment-bundle "$SMOKE_ROOT/models/wav2vec2-base-960h/main"
```

Run native transcription with an explicit Whisper bundle:

```bash
cargo run -p native-whisperx-cli -- transcribe input.wav \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --language en \
  --align-model facebook/wav2vec2-base-960h \
  --model-dir "$SMOKE_ROOT/models" \
  --interpolate-method nearest \
  --return-char-alignments \
  --output-dir out \
  --format json --format native-json --format srt --format vtt --format txt
```

Run native transcription from a locally cached Hugging Face Whisper model:

```bash
cargo run -p native-whisperx-cli -- transcribe input.wav \
  --model tiny.en \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --language en \
  --output-dir out
```

Transcribe multiple files by passing concrete paths or app-expanded wildcard
patterns. Relative and absolute paths are accepted. Quoted patterns such as
`'audio/*.wav'` are expanded by native-whisperx before transcription, so they do
not depend on shell glob behavior:

```bash
cargo run -p native-whisperx-cli -- transcribe 'audio/**/*.wav' \
  --model tiny.en \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --language en
```

When `--output-dir` is omitted, the default `json` transcript is written beside
each input file. When `--output-dir` is supplied, all outputs use that shared
directory and native-whisperx fails before transcription if two inputs would
write the same output basename. `--basename` is rejected with multiple expanded
inputs.

Run native post-ASR translation:

```bash
cargo run -p native-whisperx-cli -- input.wav \
  --language de \
  --task translate \
  --translation-model Helsinki-NLP/opus-mt-de-en \
  --model small \
  --model-dir "$SMOKE_ROOT/models" \
  --format srt
```

This path transcribes source-language segments with native Whisper, translates
segment text with the configured OPUS-MT Marian model, and preserves segment
timings for downstream writers.

Run the ignored manual cache-only native ASR smoke when `SMOKE_ROOT` contains
the required audio and Hugging Face cache layout:

```bash
cargo test -p native-whisperx-cli \
  --test native_asr_cache_smoke \
  -- --ignored --nocapture
```

Detailed setup is in [`docs/model-bundles.md`](docs/model-bundles.md#manual-native-asr-cache-smoke).

Run native-vs-Python WhisperX parity:

```bash
cargo run -p native-whisperx-cli --features whisperx-compat -- parity input.wav \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --whisper-bundle "$SMOKE_ROOT/whisper-tiny" \
  --whisperx-model tiny.en \
  --align-model facebook/wav2vec2-base-960h \
  --expected-json expected.json \
  --language en \
  --output-dir out
```

Preflight local parity resources, generate ignored Python WhisperX 3.8.6
goldens, then run the local ASR parity fixture suite:

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

`--format json` writes WhisperX-compatible JSON. Use `--format native-json`
when you need the Rust transcript contract shape.

Alignment is enabled by default. Use `--no-align` / `--no_align` to skip it.
Use `--whisper-bundle` and `--alignment-bundle` for explicit local bundles.
Without `--whisper-bundle`, native ASR can resolve supported Whisper models
through the Hugging Face cache or download path; `--model-cache-only` requires
the files to already exist locally and never downloads.
`--translation-model` reuses `--model-dir` and `--model-cache-only` for
translation model resolution unless `--translation-bundle` is supplied.

## Published-Crate Requirement

This workspace intentionally uses crates.io dependencies so a clean checkout can
resolve and test without a sibling `../rust-packages` repository. The published
dependency closure is tracked in `docs/publish-plan.md`.

During local co-development, keep local Cargo patches outside commits. One
option is to put overrides in a local Cargo config and keep that config
untracked, for example by adding `.cargo/` to `.git/info/exclude` before
creating `.cargo/config.toml`:

```toml
[patch.crates-io]
moritzbrantner-runtime-core = { path = "../rust-packages/crates/runtime/runtime-core" }
moritzbrantner-audio-analysis-speakers = { path = "../rust-packages/crates/audio/audio-analysis-speakers" }
moritzbrantner-audio-analysis-transcription = { path = "../rust-packages/crates/audio/audio-analysis-transcription" }
moritzbrantner-model-runtime = { path = "../rust-packages/crates/runtime/model-runtime" }
moritzbrantner-text-model-runtime = { path = "../rust-packages/crates/text/text-model-runtime" }
moritzbrantner-text-transcripts = { path = "../rust-packages/crates/text/text-transcripts" }
moritzbrantner-video-analysis-core = { path = "../rust-packages/crates/video/video-analysis-core" }
```

Add any transitive unpublished crates to the same local override only for local
validation. Do not commit patch entries to this repository.

## Pull Request CI

The default pull request workflow runs offline Rust gates on GitHub-hosted
Ubuntu runners:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test --workspace --no-default-features
cargo check --workspace --no-default-features --features whisperx-compat,media-decode,diarization
cargo check --workspace --no-default-features --features silero-vad
cargo check --workspace --no-default-features --features onnx-diarization
cargo check --workspace --no-default-features --features whisperx-compat,media-decode,silero-vad,diarization,onnx-diarization
```

The feature-matrix rows are compile-only gates. They cover the external
WhisperX compatibility bridge, media decode, heuristic diarization, Silero VAD,
ONNX diarization, and the combined offline optional feature set without running
model inference.

These checks do not require local model bundles, Python WhisperX, CUDA devices,
Hugging Face tokens, ONNX Runtime dynamic-library configuration, or self-hosted
parity resources. Real-resource parity checks remain in the opt-in
`parity-fixtures` workflow.

Runtime-only or intentionally constrained combinations stay outside pull
request CI:

| Feature or path | Gate | Reason and expected failure mode |
| --- | --- | --- |
| `silero-vad` runtime transcription | `parity-preflight` / `parity-fixtures` | Compile is covered in PR CI, but execution requires `ORT_DYLIB_PATH` and a local Silero ONNX bundle. Missing resources fail preflight before model execution. |
| `onnx-diarization` runtime transcription | `parity-preflight` / `parity-fixtures` | Compile is covered in PR CI, but execution requires ONNX Runtime plus local diarization model artifacts. Missing resources fail preflight before model execution. |
| `pyannote-vad` and `pyannote-diarization` full-resource parity | `parity-fixtures` `final-full-surface` | Meaningful validation requires local pyannote bundles, expected WhisperX goldens, Python WhisperX resources, and gated Hugging Face access for the delegated reference path. Missing resources are reported by preflight. |
| `cuda` | manual/report-only throughput ladder | The CUDA feature requires a compatible local CUDA toolchain and device, so it is not a GitHub-hosted pull request gate. |

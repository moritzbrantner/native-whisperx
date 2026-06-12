# native-whisperx

`native-whisperx` is a Rust-first WhisperX parity workflow repository built
from the published `moritzbrantner-*` Rust crates.

This repository owns application composition:

- CLI commands
- workflow configuration
- output file writing
- parity checks against Python WhisperX
- setup documentation for local model bundles

It does not own the reusable ASR, transcript, model-runtime, alignment, or
speaker primitives. Those remain in
[`rust-packages`](https://github.com/moritzbrantner/rust-packages) and are
consumed here as published crates.

## Crate Layout

```text
crates/native-whisperx      # reusable workflow library
crates/native-whisperx-cli  # native-whisperx CLI binary
```

## Workflow

```text
media or samples
  -> moritzbrantner-audio-analysis-transcription
  -> moritzbrantner-text-transcripts::TranscriptionContract
  -> default wav2vec2 alignment unless disabled
  -> optional speaker diarization
  -> WhisperX JSON, Native JSON, SRT, WebVTT, or plain text outputs
```

## Feature Flags

| Feature | Purpose |
| --- | --- |
| `native` | Native Candle Whisper and wav2vec2 alignment composition. Enabled by default. |
| `cuda` | CUDA-backed Candle execution. |
| `media-decode` | Opt-in non-WAV media/container decode through the audio I/O crate. |
| `diarization` | Heuristic speaker diarization composition. |
| `onnx-diarization` | Explicit ONNX speaker embedding diarization path. |
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

Run native transcription:

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

`--format json` writes WhisperX-compatible JSON. Use `--format native-json`
when you need the Rust transcript contract shape.

Alignment is enabled by default. Use `--no-align` / `--no_align` to skip it,
`--alignment-bundle` for an explicit local wav2vec2 bundle, or
`--model-cache-only` to require Hugging Face files to already exist locally.

## Published-Crate Requirement

This workspace intentionally uses crates.io dependencies. Before this repo can
be checked in a clean environment, publish the dependency closure documented in
`docs/publish-plan.md`.

During local co-development, use a temporary local Cargo patch outside commits:

```toml
[patch.crates-io]
moritzbrantner-audio-analysis-transcription = { path = "../rust-packages/crates/audio/audio-analysis-transcription" }
moritzbrantner-text-transcripts = { path = "../rust-packages/crates/text/text-transcripts" }
```

Add any transitive unpublished crates to the same local patch only for local
validation. Do not commit those patches to this repository.

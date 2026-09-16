# native-whisperx

`native-whisperx` is a Rust-first WhisperX-compatible workflow product. It owns
Workflow Composition, product configuration and policy, output placement,
product reports/progress/errors, Speaker Directory/Trace behavior, and parity
against Python WhisperX.

Reusable ASR, alignment, VAD, diarization, media, speaker, transcript, runtime,
and cancellation mechanics belong to their canonical lower-level repositories.
Native WhisperX consumes those capabilities rather than reimplementing them.
See [`docs/architecture.md`](docs/architecture.md) and ADR 0015 for the durable
ownership boundary.

## Current status

The Verified Compatibility Baseline is read from
[`tests/parity/whisperx-version.json`](tests/parity/whisperx-version.json). The
latest released WhisperX is the Upstream Target. As of 2026-09-16 both are
3.8.6; the JSON file remains the only normative baseline source.

The normal product path is native Rust. Python WhisperX is never a silent
fallback. The optional `whisperx-compat` feature still permits an explicitly
selected external provider for compatibility and supplies the parity oracle,
golden generation, and preflight tooling. Issue #252 removes that remaining
normal-product provider after the native acceptance program closes; Python then
remains parity/reference tooling only.

Three evidence gates remain intentionally open:

- #207: hosted exact-source CUDA evidence for automatic pyannote diarization,
  exact/ranged speaker bounds, and speaker embeddings. The structural gates and
  dedicated `.github/workflows/pyannote-parity-gate.yml` workflow are merged;
  missing licensed resources or skipped evidence are not green.
- #272: deployed browser WebGPU transcription acceptance. The `/acceptance/`
  surface captures fail-closed runtime evidence from the real `/transcribe/`
  workbench, but a WebGPU-capable browser still has to complete a real local
  audio run; static deployment alone is not proof of GPU inference.
- #286: optional browser post-ASR translation is structurally composed from the
  pinned nlp-stack browser translation provider, but a real WebGPU/model-backed
  translated run still provides the runtime acceptance evidence.

See [`docs/parity-matrix.md`](docs/parity-matrix.md) for the capability/evidence
matrix and [`docs/parity-worklist.md`](docs/parity-worklist.md) for the remaining
closure sequence.

## Crates

```text
crates/native-whisperx      # headless product/workflow library
crates/native-whisperx-cli  # thin CLI
```

## Feature flags

| Feature | Default | Product capability |
| --- | --- | --- |
| `native` | yes | Candle Whisper ASR plus alignment composition |
| `translation` | yes | configured OPUS-MT/Marian post-ASR translation |
| `media-decode` | yes | finite FFmpeg-backed media/container input |
| `pyannote-vad` | yes | native pyannote ONNX VAD used by automatic diarization |
| `pyannote-diarization` | yes | native pyannote community diarization bundle path |
| `cuda` | no | CUDA-backed native execution |
| `diarization` | no | base diarization capability |
| `onnx-diarization` | no | explicit ONNX speaker-embedding diarization |
| `silero-vad` | no | explicit Silero VAD |
| `whisperx-compat` | no | explicit Python WhisperX compatibility/parity tooling |

Default features are lazy: help/version/Speaker Directory commands do not load
models, download resources, use CUDA, or spawn Python. `--model-cache-only` is a
hard no-download guarantee.

## Native transcription

```bash
cargo run -p native-whisperx-cli -- transcribe input.wav \
  --model tiny.en \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --language en \
  --output-dir out
```

Alignment is enabled by default. `--no-align` disables it. Native JSON, WhisperX
JSON, TXT, SRT, WebVTT, TSV, and AUD projections are product-owned output
behavior.

Finite media inputs support the documented common audio/video extensions when
`media-decode` and the FFmpeg runtime are available. Input patterns may expand
to multiple finite files; output collision checks happen before transcription.

## Automatic diarization

```bash
cargo run -p native-whisperx-cli -- transcribe input.wav \
  --model tiny.en \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --language en \
  --diarize \
  --output-dir out
```

For native finite `--diarize`, Automatic Workflow Selection chooses pyannote
VAD plus `pyannote/speaker-diarization-community-1` when lower-level choices are
unspecified. Resources are resolved from the caller-owned model directory and
supported cache layouts. Provider-owned bundle validation remains authoritative.
See [`docs/model-bundles.md`](docs/model-bundles.md).

## Translation

Built-in Whisper translation is not emulated. Native translation is explicit
post-ASR translation with a configured model, for example:

```bash
cargo run -p native-whisperx-cli -- input.wav \
  --language de \
  --task translate \
  --translation-model Helsinki-NLP/opus-mt-de-en \
  --model small \
  --model-dir "$SMOKE_ROOT/models" \
  --format srt
```

Product translation planning/policy belongs here. The new browser consumer has
now satisfied #254's evidence-driven extraction trigger for reusable browser
translation, so that WebGPU execution lives in `nlp-stack` rather than being
duplicated in this repository. The existing Rust Marian/OPUS-MT execution,
SentencePiece/vocabulary glue, weight loading, and provider caching remain
explicitly transitional Native implementation debt until a separate source
migration is justified.

## Parity and evidence

Python WhisperX is the reference oracle for compatibility evidence:

```bash
cargo run -p native-whisperx-cli --features whisperx-compat -- \
  parity-fixtures tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --output-dir "$SMOKE_ROOT/out/parity-fixtures"
```

The dedicated pyannote acceptance workflow runs the exact-source dependency
graph, preflights all resources, and gates exact bounds, ranged bounds, and
speaker embeddings separately. The CUDA benchmark ladder and Q8 CPU evidence
remain separate provenance-bearing evidence tracks.

See:

- [`docs/parity.md`](docs/parity.md) — parity execution model and commands
- [`docs/parity-matrix.md`](docs/parity-matrix.md) — current capability status
- [`docs/parity-worklist.md`](docs/parity-worklist.md) — remaining acceptance work
- [`docs/benchmark-evidence.md`](docs/benchmark-evidence.md) — benchmark evidence policy
- [`docs/source-development.md`](docs/source-development.md) — exact source-development graph

## Browser workbench

GitHub Pages exposes the local-first transcription workbench. Browser ASR model
decode/cache/WebGPU execution is consumed from the pinned `audio-analysis`
browser adapter. Optional browser post-ASR translation is consumed from a pinned
`nlp-stack` browser translation adapter with its own lazy browser model cache.
This repository owns interaction, workflow composition, timing preservation,
capability presentation, and Native WhisperX output projection. Alignment and
diarization remain explicitly unavailable in the browser rather than being
approximated.

When browser translation is enabled, source segment timing remains authoritative,
source word/character alignments are not relabeled as translated alignment, and
the source transcript remains separately visible in-session. Native JSON, TXT,
SRT, and WebVTT download actions project the translated text after translation
completes. No browser translation path silently falls back to CPU, a server, or
Python.

The deployed `/acceptance/` page embeds the actual workbench and can produce a
small JSON evidence record only when WebGPU is ready, a real local file has
finished locally, valid timed segments exist, and Native JSON/TXT/SRT/WebVTT
projections are available. If translation is requested, the same record also
requires local translation completion and source-transcript preservation. It
records check/runtime metadata and text lengths, not transcript contents or
audio bytes.

## Development

Hosted Rust CI activates the exact revisions declared in
`.coding-tooling.source-deps.json` before authoritative source-development
validation. Registry-only publication remains a separate release concern.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --no-default-features --locked
```

Do not treat missing, skipped, cancelled, or resource-incomplete evidence as a
pass.

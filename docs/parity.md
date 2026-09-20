# Parity

Native WhisperX treats Python WhisperX as a compatibility oracle, not as the
architectural owner of reusable speech capabilities.

The normative Verified Compatibility Baseline is
`tests/parity/whisperx-version.json`. The latest released WhisperX is the
Upstream Target. As of 2026-09-16 both are 3.8.6.

## Product/runtime boundary

The normal product path is native Rust. There is no silent Python fallback.
`whisperx-compat` is non-default and currently has two explicit roles:

1. parity/preflight/golden tooling;
2. an explicitly selected external WhisperX compatibility provider.

Issue #252 removes role 2 after the native acceptance program closes. Role 1
remains because Python WhisperX is the reference implementation used to produce
and compare compatibility evidence.

## Evidence classes

Use the smallest evidence class that can prove the behavior:

- **offline deterministic CI** — formatting, Clippy, workspace tests, no-default
  tests, feature matrix, CLI/report contracts, static Pages checks;
- **local-resource parity** — model-backed ASR/alignment/output fixtures against
  the verified WhisperX baseline;
- **full-resource parity** — licensed/large resources such as pyannote and CUDA;
- **benchmark evidence** — provenance-bearing performance reports, separate from
  semantic parity;
- **browser runtime acceptance** — deployed browser/WebGPU execution, separate
  from static-site validation.

Missing, skipped, cancelled, or resource-incomplete evidence is not green.

## Core ASR parity

Preflight the configured resources:

```bash
cargo run -p native-whisperx-cli --features whisperx-compat -- \
  parity-preflight tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --require-expected
```

Generate/update reference goldens only when intentionally reconciling the
Verified Compatibility Baseline:

```bash
cargo run -p native-whisperx-cli --features whisperx-compat -- \
  parity-goldens tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --overwrite
```

Run the gating ASR suite:

```bash
cargo run -p native-whisperx-cli --features whisperx-compat -- \
  parity-fixtures tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --output-dir "$SMOKE_ROOT/out/parity-fixtures"
```

The current native surface includes multilingual model aliases, prompt seeding,
explicit token/numeral suppression, previous-text conditioning, temperature and
fallback thresholds, beam/best-of controls, alignment, timed output writers,
finite media input, and configured post-ASR translation. Faster-whisper-only
hotwords, Python logging/progress flags, and separate WhisperX `--fp16`
emulation remain intentionally unsupported rather than delegated silently.

## Pyannote acceptance

`tests/parity/full-resource-fixtures.json` contains three gating pyannote cases:

- `diarization-shrek-retold-3m-pyannote-exact-reference`;
- `diarization-shrek-retold-3m-pyannote-range-reference`;
- `diarization-shrek-retold-3m-speaker-embeddings-pyannote-reference`.

They exercise permutation-aware speaker assignment, segment timing, exact and
ranged speaker bounds, and structural speaker-embedding validation. Embedding
vectors themselves are not exposed in public reports or compared numerically
across runtimes.

The exact and ranged cases run the reference through
`tests/parity/whisperx_same_pyannote_vad.py`. The wrapper reuses the configured,
version-checked WhisperX Python environment while replacing only its bundled
VAD with the pinned `pyannote/segmentation-3.0` revision used by native
automatic composition. The configured command remains the source of the Python
interpreter, and credentials remain child-environment-only.

`.github/workflows/pyannote-parity-gate.yml` is the dedicated #207 acceptance
workflow. It activates the exact source graph, runs full preflight, then runs the
three cases independently with cache-only native model resolution. The workflow
requires the configured self-hosted CUDA/parity runner, licensed pyannote
resources/Hugging Face access, and a Python WhisperX reference environment.

#207 remains open until the exact and ranged speaker-turn gates pass; local CUDA
evidence now verifies the same-model VAD count and timing gates independently.

## Benchmark evidence

Semantic parity and performance are separate gates. The CUDA benchmark ladder
uses `tests/parity/rust-native-bench-fixtures.json`; the multi-input benchmark
uses `tests/parity/rust-native-multi-input-bench-fixtures.json`. Raw reports
retain environment/model/runtime provenance; only sanitized summaries are
appropriate for committed documentation.

Q8/Int8 is an explicit non-default Native WhisperX extension. It is not a
claim that all WhisperX quantized compute aliases are supported. The native Q8
route remains constrained by its documented CPU/ASR-only requirements and has
its own evidence workflow.

## Browser evidence

Static Pages validation proves deployment shape and JavaScript contracts, not
actual WebGPU inference. The deployed `/acceptance/` page embeds the real
`/transcribe/` workbench and fails closed unless WebGPU is ready, a selected
local file finishes locally, valid timed segments are rendered, and the existing
Native JSON/TXT/SRT/WebVTT projection controls are available. Its downloadable
JSON records check/runtime metadata but not transcript contents or audio bytes.

#272 remains open until a WebGPU-capable browser produces a passing acceptance
record from a real local spoken-audio run.

## Status sources

- [`parity-matrix.md`](./parity-matrix.md) is the current capability/evidence
  matrix.
- [`parity-worklist.md`](./parity-worklist.md) contains only unfinished closure
  work; completed historical rows do not stay in the worklist.
- [`benchmark-evidence.md`](./benchmark-evidence.md) defines benchmark handling.
- [`architecture.md`](./architecture.md) and ADR 0015 define ownership.

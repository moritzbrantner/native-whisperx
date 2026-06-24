# Parity

Python WhisperX CLI parity is the product contract for native-whisperx, starting
with the versioned baseline in [`parity-matrix.md`](./parity-matrix.md). The
Rust implementation is the direction of travel, but Python delegation is
allowed while equivalent Rust features mature.

Rust-Native Parity is the stricter track for proving the same user-visible
WhisperX surface without adding new Python WhisperX or faster-whisper runtime
bridges. Python WhisperX 3.8.6 remains the reference oracle for goldens and
reports, but the implementation under test must run through this repository and
its vendor code. See
[`0006-rust-native-parity-proving-ground.md`](./adr/0006-rust-native-parity-proving-ground.md).

The native replacement work is tracked row-by-row in
[`parity-worklist.md`](./parity-worklist.md). That worklist is the operational
source of truth for whether a CLI surface is native complete, native partial,
delegated only, blocked by an upstream crate, or waiting on fixtures.

The Rust workflow composes these pieces:

- Candle Whisper ASR through `moritzbrantner-audio-analysis-transcription`,
  with explicit bundles or supported Hugging Face cache/download resolution
- wav2vec2 CTC alignment from a supported local bundle or Hugging Face cache
- Automatic Workflow Selection for native finite `--diarize`, which composes
  pyannote VAD plus `pyannote/speaker-diarization-community-1` when lower-level
  VAD and diarization model choices are unspecified
- heuristic or ONNX-backed speaker diarization when explicitly enabled
- Helsinki-NLP OPUS-MT/Marian post-ASR segment translation when
  `--translation-model` is supplied
- transcript normalization and WhisperX JSON import through
  `moritzbrantner-text-transcripts`

## Current milestone

The remediation track is native-first but still compatibility-bridged by Python
WhisperX 3.8.6. Current baseline:

- ASR, alignment, and output writing are partially native and covered by
  machine-readable fixture reports.
- Core ASR fixtures now include promoted German no-align, alignment alias/cache,
  and semantic highlighted-subtitle gates; exact highlighted SRT/VTT bytes stay
  report-only.
- Silero VAD, pyannote VAD, pyannote diarization, speaker embeddings, and
  speaker bounds are measured in the full-resource suite. Default runs keep
  non-gating probes report-only; the `final-full-surface` workflow suite
  enables `--require-non-gating-passed`. Full-resource preflight is currently
  blocked by missing local goldens/media/model resources listed below.
- Performance is benchmarked in normal reports and through the large-v3-turbo
  CUDA ladder. The `final-full-surface` workflow suite now runs that ladder as
  a hard gate after the full-resource parity suite. Findings are recorded in
  [`native-performance-findings.md`](./native-performance-findings.md).

The Rust-Native Parity program keeps that baseline but raises the bar for the
new parity track: ASR, alignment, VAD, diarization, translation, output writers,
decode controls, CLI compatibility, parity reports, and benchmarks must be
implemented or explicitly blocked in Rust/native code. Final correctness
evidence is the full-resource parity suite plus the 30 second, 3 minute, and 10
minute large-v3-turbo CUDA ladder derived from the local Shrek reference media.

Automatic Workflow Selection is a Workflow Composition concept, distinct from
WhisperX Parity and Rust-Native Parity. It decides how the native workflow is
composed from a higher-level request such as `--diarize`; it does not change
WhisperX JSON, Native JSON, subtitle, or text output contracts. The parity
tracks still decide whether the composed behavior matches Python WhisperX and
whether it is implemented through Rust/native code.

The current milestone is native ASR timing parity after the Hugging Face cache
path. Native ASR no longer requires `--whisper-bundle` when a supported Whisper
model is already in the Hugging Face cache or downloads are allowed.
`--whisper-bundle` remains the recommended deterministic offline path. Native
English-only Whisper aliases such as `tiny.en` provide an `en` language hint
when no explicit language is supplied, which keeps the local language-detection
fixture aligned with WhisperX for English-only models. Native `--task translate
--translation-model ...` transcribes source-language segments and translates
them through the native Helsinki-NLP OPUS-MT/Marian path. Native `--task
translate` without a translation model is delegated to Python WhisperX for
parity today. Native pyannote VAD is available through the feature-gated local
ONNX path and otherwise fails explicitly instead of falling back to another VAD.
For automatic native finite `--diarize`, native-whisperx selects pyannote VAD
plus `pyannote/speaker-diarization-community-1`, checks `--model-dir` first,
then standard Hugging Face cache roots, and fails before transcription if the
automatic VAD or diarization resources are missing. `--model-cache-only` is a
hard no-download guarantee. Without cache-only, the documented future lookup
order includes downloads, but the current native pyannote download path is not
wired to a bundle hydrator yet, so missing automatic pyannote resources still
produce setup guidance before transcription.

The repository has an ignored/manual wrapper smoke for cache-only native ASR
resolution and a local-only ASR parity fixture suite. The fixture suite is the
next native parity checkpoint: it compares native ASR against Python WhisperX
for real local audio, cache-only model resolution, explicit language, alignment,
optional character alignments, and output writer goldens. TXT/TSV/SRT/VTT/AUD
goldens are compared byte-for-byte; JSON goldens are compared semantically.
The core English ASR fixtures now gate native timing parity: no-alignment
`tiny.en`, no-alignment `small`, and English-only language detection gate
segment timing; the aligned `tiny.en` fixture gates segment and word timing;
and the char-alignment fixture gates segment timing, word timing, and character
count. Timing reports include native and WhisperX start/end values, absolute
deltas, and the configured tolerance for each mismatch. German ASR expansion,
alignment alias/cache behavior, translation, and
`tiny-output-subtitles-highlight` remain non-gating until independently
promoted. Output writer fixtures `tiny-output-subtitles-wrap` and
`tiny-output-segment-resolution-chunk` gate byte-for-byte SRT/VTT goldens, and
`tiny-output-all-defaults` gates TXT/VTT/SRT/TSV byte-for-byte goldens plus
contract-aware semantic JSON comparison. `tiny-output-subtitles-highlight`
stays report-only because highlighted word cue milliseconds are derived from
exact word timing and can still differ at the byte level while word timing is
within the strict 0.050s tolerance.

```bash
cargo test -p native-whisperx-cli \
  --test native_asr_cache_smoke \
  -- --ignored --nocapture
```

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

Fixture reports can be compacted for artifacts or dashboards:

```bash
cargo run -p native-whisperx-cli -- parity-summary "$SMOKE_ROOT/out/parity-fixtures/report.json"
```

The compact summary includes suite pass status, workflow metadata, per-case
`passed`, `gating`, `status`, `expectedTarget`, strict failures, report-only
differences, expected JSON match status, missing required diagnostics,
`startedAt`, `elapsedSeconds`, and `timedOut`. Workflow runs also pass
`--preflight-report` and `--allow-missing-report` so `summary.json` is still
uploaded when a resource preflight failure prevents `report.json` from being
created. In that case `skippedCases` records the selected cases that did not run
and `preflight.missingResources` records the missing local resources.

Benchmark runs compare native and delegated WhisperX execution without mutating
checked-in files or enforcing speed thresholds:

```bash
cargo run -p native-whisperx-cli -- parity-bench tests/parity/asr-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command .audio-tools/whisperx-venv/bin/whisperx \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --iterations 3 \
  --json
```

The Rust-Native Parity benchmark ladder uses generated local clips and keeps
both clips and reports out of git:

```bash
set -a
. ./.env
set +a
WHISPERX_COMMAND="$(conda run -n whisperx which whisperx)"
cargo run -p native-whisperx-cli --features whisperx-compat,media-decode,silero-vad,pyannote-vad,pyannote-diarization,cuda -- \
  parity-bench tests/parity/rust-native-bench-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command "$WHISPERX_COMMAND" \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --case-timeout-seconds 900 \
  --json
```

The selectable cases are:

- `shrek-retold-30s-large-v3-turbo-cuda`
- `shrek-retold-3m-large-v3-turbo-cuda`
- `shrek-retold-10m-large-v3-turbo-cuda`

The report-only multi-input benchmark uses
`tests/parity/rust-native-multi-input-bench-fixtures.json` and measures one
`Multi-Input Transcription Run` over five three-minute Shrek-derived slices.
Generate the local clips under the smoke root before running it:

```bash
ffmpeg -ss 00:00:00 -i "$SMOKE_ROOT/reference/Shrek Retold - Full Movie [pM70TROZQsI].webm" -t 180 -ac 1 -ar 16000 "$SMOKE_ROOT/audio/shrek-retold-5x3m-slice-01.wav"
ffmpeg -ss 00:18:00 -i "$SMOKE_ROOT/reference/Shrek Retold - Full Movie [pM70TROZQsI].webm" -t 180 -ac 1 -ar 16000 "$SMOKE_ROOT/audio/shrek-retold-5x3m-slice-02.wav"
ffmpeg -ss 00:36:00 -i "$SMOKE_ROOT/reference/Shrek Retold - Full Movie [pM70TROZQsI].webm" -t 180 -ac 1 -ar 16000 "$SMOKE_ROOT/audio/shrek-retold-5x3m-slice-03.wav"
ffmpeg -ss 00:54:00 -i "$SMOKE_ROOT/reference/Shrek Retold - Full Movie [pM70TROZQsI].webm" -t 180 -ac 1 -ar 16000 "$SMOKE_ROOT/audio/shrek-retold-5x3m-slice-04.wav"
ffmpeg -ss 01:12:00 -i "$SMOKE_ROOT/reference/Shrek Retold - Full Movie [pM70TROZQsI].webm" -t 180 -ac 1 -ar 16000 "$SMOKE_ROOT/audio/shrek-retold-5x3m-slice-05.wav"
```

Run the report-only benchmark with:

```bash
cargo run -p native-whisperx-cli --features whisperx-compat,media-decode,silero-vad,pyannote-vad,pyannote-diarization,cuda -- \
  parity-bench tests/parity/rust-native-multi-input-bench-fixtures.json \
  --root "$SMOKE_ROOT" \
  --whisperx-command "$WHISPERX_COMMAND" \
  --model-dir "$SMOKE_ROOT/models" \
  --model-cache-only \
  --case shrek-retold-5x3m-large-v3-turbo-cuda \
  --case-timeout-seconds 1800 \
  --report-only \
  --json
```

Set `SMOKE_ROOT` in the checkout `.env` before running it. The local WhisperX
reference command should come from the conda environment named `whisperx`.
See
[`model-bundles.md`](./model-bundles.md#local-asr-parity-fixtures) for the
required audio, expected WhisperX JSON, and Hugging Face cache layout. Default
CI does not run these local real-model checks.

Full-resource parity measurements live in a separate manifest for ONNX-backed
VAD and diarization behavior. The automatic native `--diarize` case exercises
prepared-cache pyannote lookup when run with `--model-cache-only`; maintainers
can rerun the same prepared machine without cache-only to validate the current
download-boundary failure path until a pyannote hydrator is implemented:

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

To make non-gating full-resource probes fail an opt-in run, pass
`--require-non-gating-passed` to `parity-fixtures`. The checked-in manifest
keeps those cases non-gating so default offline CI remains hermetic.

Use `parity-preflight` against the same manifest before an expensive parity
run. It resolves `SMOKE_ROOT`, checks selected audio/model/golden resources,
verifies the Python WhisperX command and source checkout for reference runs,
requires Hugging Face tokens only for enabled gated diarization paths, and
checks `ORT_DYLIB_PATH` for ONNX-backed VAD or diarization cases.

The final full-surface gate is exposed by the `parity-fixtures` workflow
`final-full-surface` suite. It runs full-resource parity with
`--require-non-gating-passed`, then runs the large-v3-turbo CUDA benchmark
ladder as a hard gate. The benchmark gate selects the 30 second, 3 minute, and
10 minute cases explicitly, requires finite native and WhisperX elapsed
seconds, and requires `nativeFasterThanWhisperx=true` plus
`nativeSpeedupRatio >= 1.001` for every measured iteration. The benchmark
remains a local final-suite gate because it requires local Shrek-derived media,
cached models, Python WhisperX, and CUDA hardware.

The final suite currently requires these missing local resources before it can
run end to end: expected WhisperX goldens, `two-speaker.wav`, `ORT_DYLIB_PATH`,
explicit pyannote VAD `models/pyannote-vad/segmentation.onnx` for the explicit
VAD fixture, automatic pyannote cache resources for
`pyannote/segmentation-3.0` and
`pyannote/speaker-diarization-community-1`, explicit pyannote diarization ONNX
artifacts under `models/pyannote-diarization` for the speaker-embeddings
fixture, `HF_TOKEN` for WhisperX pyannote diarization, and a checkout-local
`.audio-tools/whisperx-src` at the exact parity tag. Current preflight reports
explicit bundle, media, golden, token-presence, source-checkout, and ONNX
Runtime misses; automatic pyannote cache misses may be reported by the fixture
run itself before transcription.

## Parity Workflow Artifacts

`.github/workflows/parity-fixtures.yml` runs on manual dispatch, the nightly
schedule when `PARITY_SMOKE_ROOT` is configured, and same-repository pull
requests labeled `run-parity-fixtures`. Each run uploads one artifact named for
the selected suite. The artifact contains:

- `summary.json`: compact maintainer summary with selected suite, features,
  runner, manifest, output directory, `SMOKE_ROOT`, model directory, WhisperX
  command path, optional `ORT_DYLIB_PATH`, pass status, gating failures,
  non-gating failures, skipped cases, errored cases, and preflight missing
  resources. It records path-like configuration only; it never records
  `HF_TOKEN` or other secret values.
- `preflight.json`: full preflight report for missing local audio, model,
  golden, source-checkout, token-presence, and ONNX Runtime resources.
- `report.json`: raw fixture report when fixture execution starts.
- `progress.log`: stderr progress and diagnostics from preflight and fixture
  execution.

Use `summary.json` first to decide whether a failure is merge-gating,
report-only, skipped by preflight, or an execution error. Open `report.json` and
`progress.log` only when the compact summary does not contain enough detail.

External Python WhisperX remains the compatibility bridge for behavior that is
not native yet. Native decode accepts default-equivalent greedy controls
(`--temperature 0` and `--condition_on_previous_text false`) and fails every
behavior-changing unsupported control with a per-flag reason instead of
silently ignoring it. Delegated controls are forwarded through the current
external command argument bridge when `--provider external-whisperx` is
selected.

Default CI remains offline. It uses checked-in fixtures, fake command tests,
and mocked Silero probability tests; real Python WhisperX, real Silero ONNX
models, native ASR cache/download parity, model downloads, and HF-token-gated
diarization remain local/manual or future opt-in checks. The real ASR cache
smoke test is ignored/manual, and the ASR parity fixture suite is local-only for
now. A later CI step can move the suite into a scheduled or labeled workflow
with prewarmed caches/secrets.

Silero VAD parity is measured at the merged speech-chunk boundary surface:
native `response.vadSegments` is compared with delegated WhisperX
`response.vadSegments` using the segment timing tolerance. Raw Silero
probabilities are not a parity surface because Python WhisperX 3.8.6 loads the
Torch Hub model while native execution uses caller-provided ONNX. Native Silero
uses `vad_onset` and `chunk_size`; `vad_offset` is accepted for CLI/config
compatibility but WhisperX Silero merge behavior does not use it. Native
pyannote uses `vad_onset`, `vad_offset`, and `chunk_size` for hysteresis and
merged speech chunks.

Current parity failures or planned work versus Python WhisperX:

- behavior-changing native decode controls remain blocked until upstream Candle
  Whisper APIs expose sampling, beam search, prompt seeding, logit filtering,
  threshold metrics, precision, and thread-count controls
- full-resource parity preflight needs the missing local goldens/media/model
  resources listed above
- broader WhisperX sentence segmentation coverage beyond the current writer
  goldens
- ONNX Runtime dynamic-library discovery is host-sensitive

Parity reports now include additional comparison categories for language,
segment text sequence, word text sequence, character alignment count, direct
VAD segment count/timing, and diagnostic differences. Fixture reports also
include expected output-file comparisons, expected-output gating/report-only
classification, and optional case timing. Existing top-level text, timing, word
count, segment count, and speaker-turn fields remain part of the report.

## Surface changes

This milestone extends `--model-dir` and `--model-cache-only` to native ASR,
native alignment, native Helsinki translation, and delegated Python WhisperX
forwarding. It also aligns `--segment_resolution` with WhisperX
`sentence|chunk`, with `sentence` as the default and `segment` retained only as
a legacy alias. Native behavioral parity is still intentionally limited to the
implemented Rust paths described in the parity matrix.

Run native-vs-Python comparison only when local Python tooling is installed:

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

Set `HF_TOKEN` before parity runs that ask Python WhisperX to diarize. Native
automatic selection does not consume CLI token strings; future/prepared native
cache workflows use environment or standard Hugging Face auth state and must
not print token values.

Additional manual smoke commands for ASR cache resolution, Automatic Workflow
Selection pyannote resource checks, Silero VAD, and ONNX diarization are
collected in
[`parity-worklist.md`](./parity-worklist.md#manual-parity-commands).

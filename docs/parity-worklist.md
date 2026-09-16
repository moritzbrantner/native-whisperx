# Native WhisperX remaining work

This file contains only unfinished acceptance/migration work. Completed feature
rows belong in `parity-matrix.md`; they do not remain here as historical
`partial` or `delegated` states.

## 1. Finish pyannote acceptance — #207

Implementation is present on `main`:

- automatic pyannote VAD + community diarization resource selection;
- permutation-aware speaker comparison;
- exact `min_speakers == max_speakers` gating;
- ranged `min_speakers` / `max_speakers` gating;
- structural speaker-embedding validation without exposing/comparing raw vectors;
- preflight that reports missing automatic pyannote resources;
- provider-owned bundle validation;
- exact-source acceptance workflow in
  `.github/workflows/pyannote-parity-gate.yml`.

Remaining acceptance is evidence, not another implementation rewrite. Run the
dedicated workflow on the configured parity/CUDA runner with:

- the caller-owned smoke/model root;
- licensed pyannote resources/Hugging Face access;
- the Python WhisperX reference environment;
- CUDA available to the native path.

The run must complete the preflight plus all three cases:

1. `diarization-shrek-retold-3m-pyannote-exact-reference`;
2. `diarization-shrek-retold-3m-pyannote-range-reference`;
3. `diarization-shrek-retold-3m-speaker-embeddings-pyannote-reference`.

Attach/retain the machine-readable workflow artifacts. Missing, skipped,
cancelled, or resource-incomplete evidence does not close #207.

## 2. Reconcile documentation — #210

The final documentation set must agree on:

- composition-only ownership;
- Upstream Target vs Verified Compatibility Baseline terminology;
- the baseline JSON file as the only normative version source;
- implemented native decode controls, including prompt/suppression/history;
- pyannote implementation vs still-pending real-resource acceptance;
- Q8 as an explicit non-default Native WhisperX extension;
- Python's temporary explicit product-provider role and long-term oracle-only role;
- translation planning/policy as permanent product ownership while the current
  Marian/OPUS-MT execution remains explicitly transitional implementation debt
  per #254 rather than falsely claimed as already extracted;
- intentionally unsupported hotwords, Python logging flags, separate `--fp16`
  emulation, and live-input parity;
- browser WebGPU acceptance as #272 rather than proof inferred from static Pages
  deployment.

Once the documentation PR is integrated and its deterministic checks are green,
#210 can close. It does not need to pretend #207 or #272 already passed; it must
state those remaining evidence gaps accurately.

## 3. Close the native feature PRD — #195

After #207 and #210 are complete, re-read #195 against the exact merged default
branch. Close it only when the in-scope advertised native feature surface has no
undocumented partial row and the required evidence is present.

Do not make #195 depend on #272: browser transcription is a separate product
surface created after the original native parity PRD.

## 4. Retire Python as a normal product runtime — #252

This activates only after #195 is resolved.

Required migration:

- remove/deprecate the normal transcription provider path that executes Python
  WhisperX;
- remove product errors/help text that recommend switching to
  `--provider external-whisperx` for unsupported native combinations;
- keep parity, preflight, golden generation, and comparison commands able to
  execute the Python WhisperX oracle under a non-default compatibility/parity
  feature;
- keep default library/CLI builds Python-free;
- provide pre-1.0 Rust API/CLI migration guidance for any removed provider/config
  surface;
- preserve fail-closed behavior: unsupported native combinations name the actual
  limitation rather than silently delegating.

This is an ownership/API cleanup, not a second parity implementation.

## 5. Close the composition-only migration PRD — #246

After #252 lands, reassess #246 on merged `main`. The final check is that the
public product facade owns only product composition/contracts while reusable
execution mechanics remain in canonical lower-level owners. #254's explicit
translation exception remains valid: Marian extraction is deferred until a real
reuse/architecture trigger exists and is not itself a blocker for #246.

## 6. Browser runtime acceptance — #272

This is independent of the native 1.0 closure sequence above.

Already implemented:

- Pages `/transcribe/` surface;
- pinned `audio-analysis` browser transcription adapter;
- local browser decode/resample/model cache/WebGPU ASR ownership upstream;
- Native JSON/TXT/SRT/WebVTT projection in this product;
- explicit browser capability boundaries for alignment/diarization/translation;
- static-site and adapter-contract deployment validation;
- fail-closed `/acceptance/` harness that embeds the real workbench and records
  check/runtime metadata without transcript contents or audio bytes.

Remaining proof:

- open deployed `/acceptance/` in a WebGPU-capable browser;
- select a real local spoken-audio file in the embedded workbench;
- complete local transcription without a server/CPU fallback;
- capture a passing acceptance JSON proving WebGPU readiness, local completion,
  valid timed segments, and Native JSON/TXT/SRT/WebVTT projection availability;
- attach or record that evidence on #272.

Static deployment success alone is not sufficient.

## Termination rule

Do not invent additional parity features while these acceptance/migration items
are unresolved. Once #207, #210, #195, #252, and #246 are closed, the native
parity/composition program has a termination proof. #272 may continue as its own
browser product acceptance track.

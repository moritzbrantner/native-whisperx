Current closure status: #207, #210, #195, and #272 are closed. The sections
below retain their evidence requirements; #252 is the final product API migration
and #246 is reconciled on the merged result.

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
- Python's final oracle/reference/golden-only role and the product migration boundary;
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

The #195 activation gate is met. Product transcription now rejects Python
provider selection in all feature sets; unsupported native combinations name
the limitation. Explicit oracle execution, preflight, golden generation,
comparison, and parity benchmarks retain non-default `whisperx-compat` access.
CLI/Rust API migration is documented in `docs/python-oracle-migration.md` and
ADR 0016.

This completes the final Python-role documentation follow-up from #210.
Legacy configuration round trips are retained for migration, without restoring
a product runtime branch.

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

Acceptance recorded on #272 (2026-09-20):

- deployed `/acceptance/` ran in Chrome 153 on the physical NVIDIA adapter;
- a real local spoken-audio file completed in the embedded workbench;
- local transcription completed without a server/CPU fallback;
- the passing acceptance JSON records WebGPU readiness, local completion,
  valid timed segments, and Native JSON/TXT/SRT/WebVTT projection availability;
- #272 retains that evidence and is closed.

Static deployment success alone is not sufficient.

## Termination rule

Do not invent additional parity features while these acceptance/migration items
are unresolved. Once #207, #210, #195, #252, and #246 are closed, the native
parity/composition program has a termination proof. #272 may continue as its own
browser product acceptance track.

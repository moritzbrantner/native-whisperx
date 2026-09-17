# Native WhisperX remaining work

This file contains only unfinished acceptance/migration work. Completed feature
rows belong in `parity-matrix.md`; they do not remain here as historical
`partial` or `delegated` states. Documentation reconciliation #210 is complete.

## 1. Finish pyannote acceptance — #207

Implementation is present on `main`:

- automatic pyannote VAD + community diarization resource selection;
- permutation-aware speaker comparison;
- exact and ranged speaker-count gating;
- structural speaker-embedding validation without exposing/comparing raw vectors;
- preflight that reports missing automatic pyannote resources;
- provider-owned bundle validation;
- exact-source acceptance workflow in `.github/workflows/pyannote-parity-gate.yml`.

Remaining acceptance is external evidence. Run the dedicated workflow on the
configured parity/CUDA runner with the caller-owned smoke/model root, licensed
pyannote resources/Hugging Face access, the Python WhisperX reference
environment, and CUDA. Preflight plus the exact-bound, ranged-bound, and
speaker-embedding cases must all pass. Missing/skipped evidence is not green.

## 2. Close the native feature PRD — #195

#207 is the remaining native feature blocker. Once its required evidence exists,
re-read #195 against exact merged `main` and close it only if every in-scope
advertised native feature still matches the capability/evidence matrix.

Browser issues #272 and #286 are independent product surfaces and do not block
this original native parity PRD.

## 3. Retire Python as a normal product runtime — #252

This activates only after #195 is resolved. Remove/deprecate normal transcription
through Python WhisperX while retaining Python behind explicit non-default
parity/preflight/golden/comparison tooling. Default library/CLI builds stay
Python-free and unsupported native combinations remain fail-closed instead of
silently delegating.

## 4. Close the composition-only migration PRD — #246

After #252 lands, reassess #246 on merged `main`. The product facade should own
composition/contracts while reusable execution lives in canonical lower-level
owners. Browser translation supplied the reuse trigger anticipated by #254, but
repository ownership review placed browser execution in `platform-packages`, the
recorded browser implementation owner, rather than `nlp-stack`. The existing
Rust Marian implementation remains transitional until a separate source
migration is justified.

## 5. Browser transcription runtime acceptance — #272

Structural implementation is complete:

- deployed `/transcribe/` surface;
- pinned `audio-analysis` browser ASR adapter;
- local decode/resample/model-cache/WebGPU ASR;
- Native JSON/TXT/SRT/WebVTT projection;
- explicit browser alignment/diarization boundaries;
- static-site validation;
- fail-closed `/acceptance/` evidence harness.

Remaining proof is a real WebGPU browser run with local spoken audio and a
passing acceptance JSON. Static deployment success is not runtime proof.

## 6. Browser post-ASR translation acceptance — #286

Structural implementation source-builds a pinned reusable `platform-packages`
browser translation adapter after browser ASR. Translation is opt-in,
WebGPU-only, lazy and browser-cached, with no server/CPU/Python fallback. The
initial curated pairs are German→English and English→German; the adapter derives
the Marian model from the pair and rejects mismatches. Native WhisperX also
fails if browser ASR reports a language that conflicts with the selected source
pair.

Native WhisperX preserves source segment timing, does not relabel source
word/character alignment as translated alignment, keeps the source transcript
separately visible in-session, and projects translated Native JSON/TXT/SRT/WebVTT
outputs only after translation completes.

Remaining proof is a deployed model-backed browser run with translation enabled
and a passing acceptance JSON showing `translationRequested`, local translation
completion, source-transcript preservation, valid timed segments, and projection
availability. The evidence records metadata and text lengths, not transcript
contents or audio bytes.

## Termination rule

Do not invent additional parity features while these acceptance/migration items
are unresolved. Once #207, #195, #252, and #246 are closed, the native
parity/composition program has a termination proof. #272 and #286 may continue
as independent browser runtime acceptance tracks.

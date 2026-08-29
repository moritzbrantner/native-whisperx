# ADR 0015: Make Native WhisperX a composition-only product boundary

## Status

Accepted.

## Context

Earlier Rust-Native Parity work intentionally allowed reusable implementations to live in this repository while parity converged. That was useful while canonical lower-level ownership and source-development workflows were immature, but it now creates the wrong long-term incentive: parity work can accumulate reusable ASR, VAD, alignment, translation, media, speaker, resource, and runtime mechanics inside the application repository.

The ecosystem now has canonical owners such as `audio-analysis`, `nlp-stack`, and shared runtime/foundation crates, and source mode allows Native WhisperX to consume unreleased reviewed changes without publishing a crate first.

Native WhisperX also has a meaningful product surface beyond a CLI wrapper: Workflow Composition, compatibility configuration, Automatic Workflow Selection, parity evidence, product reports/progress/errors, Speaker Directory workflows, near-live semantics, and output policy.

## Decision

`native-whisperx` is permanently composition-only.

The ownership test is:

> If a capability would still make sense in a Rust transcription application that does not use Native WhisperX semantics, it belongs in its reusable upstream owner rather than in `native-whisperx`.

Native WhisperX owns user-visible workflow intent and compatibility behavior. Reusable execution mechanics move to their canonical crates.

The detailed ownership matrix is maintained in [`../architecture.md`](../architecture.md).

### Product API

`native-whisperx` remains a first-class headless product engine and `native-whisperx-cli` remains a thin terminal adapter.

The Rust API is a curated facade rather than a broad re-export hub. Canonical cross-project domain contracts such as `TranscriptionContract` may cross the boundary, but low-level provider/runtime/media DTOs should be imported from their owning crates directly.

`NativeWhisperxConfig`, product reports, progress events, outcomes, and programmatically meaningful errors are Native WhisperX-owned contracts. Upstream execution DTOs and implementation-specific error enums must not accidentally become part of the Native WhisperX stability surface.

The pre-1.0 period is the deliberate cleanup window for this boundary; 1.0 should represent an intentional semver-stable headless product API.

### Reusable capability ownership

- Generic media probing/track selection/decode belongs to `audio-analysis-io`.
- Reusable ASR/VAD/alignment/diarization execution, provider/session reuse, bounded-window transcription mechanics, and transcription lifecycle facts belong to `audio-analysis-transcription`.
- Reusable Speaker Library persistence and profile lifecycle belong to `audio-analysis-speakers`; Speaker Directory selection and Speaker Trace remain product concepts here.
- Marian/OPUS-MT execution belongs to `text-model-runtime`; translation planning/pivot policy remains here until a genuinely independent translation API warrants another crate.
- Generic SRT/VTT/TXT/TSV/Audacity rendering belongs to `text-transcripts`; WhisperX-specific output mapping and parity remain here.
- Generic cancellation and model resource mechanics belong to shared runtime/foundation infrastructure.
- Model-specific bundle validity belongs with the provider that consumes the bundle.

Native WhisperX retains resource policy, product defaults, compatibility validation, workflow ordering, output placement, parity harness behavior, and product narratives around those lower-level facts.

### Compatibility and extensions

WhisperX compatibility is a baseline, not a ceiling. On overlapping surfaces, compatible observable semantics win by default; native-only capabilities may extend the product additively.

Python WhisperX runtime delegation is transitional. The long-term role of Python WhisperX is oracle/reference/golden generation, not a permanent second execution architecture. No new silent Python fallback should be introduced.

The latest released WhisperX is the **Upstream Target**. The newest version with passing gating evidence is the **Verified Compatibility Baseline**. A new upstream release can place the project in a documented behind-upstream state without making deterministic `main` CI immediately red; the verified baseline advances once compatibility evidence passes.

Q8/Int8 is a supported first-class native extension but remains explicit and non-default.

### Product policy, features, and progress

Automatic Workflow Selection grows through narrow capability-specific policy rather than a generic planner/capability graph.

Feature flags describe user-visible Native WhisperX capabilities, not incidental backend technology. Library and CLI use aligned batteries-included defaults; specialized hardware, compatibility bridges, and implementation-specific alternatives remain opt-in. Enabled capabilities must still load/resolve resources lazily.

Upstream crates emit authoritative low-level lifecycle facts; Native WhisperX composes those facts into one stable product progress stream.

Provider choices are exposed at the product layer only when they represent a meaningful user tradeoff, not simply because multiple internal backends exist.

## Consequences

- Parity work that discovers a missing reusable capability must implement that capability in the canonical owner first, using source mode when needed.
- The proving-ground exception from ADR 0006 is removed.
- Some current code and public APIs must migrate before 1.0, including reusable translation execution, generic transcript writers, media predecode/track selection, Speaker Library persistence, live window/session mechanics, cancellation tokens, provider reuse, bundle validation, broad re-exports, upstream response embedding, and upstream error leakage.
- The Parity Harness stays in this repository unless a second independent consumer proves a common lower-level comparison abstraction is warranted.
- New crates are not created merely to satisfy composition-only ownership; independent versioning/API needs remain required.
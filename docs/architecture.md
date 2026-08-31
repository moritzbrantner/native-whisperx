# Native WhisperX Architecture

## Purpose

`native-whisperx` is the headless product engine and CLI composition layer for a Rust-first, WhisperX-compatible transcription product. It owns user-visible workflow semantics. Reusable execution capabilities live in their canonical lower-level owners.

The permanent ownership test is:

> If a capability would still make sense in a Rust transcription application that does not use Native WhisperX semantics, it does not belong in `native-whisperx`.

This is a strict composition-only rule. Native WhisperX may coordinate, validate, translate product configuration, report progress, place outputs, and prove compatibility. It must not become the permanent implementation home for reusable ASR, VAD, alignment, diarization, media decoding, timed-text formatting, cancellation, speaker storage, model-resource, or other generic runtime machinery.

## Product layers

```text
native-whisperx-cli
  terminal parsing, stdout/stderr, TTY rendering, shell-facing adapters
        |
        v
native-whisperx
  WhisperX-compatible product configuration
  workflow composition and automatic-selection policy
  multi-input/output/speaker/live product semantics
  unified progress, reports, errors, parity, compatibility
        |
        v
reusable capabilities
  audio-analysis-io
  audio-analysis-transcription
  audio-analysis-speakers
  media-core / shared foundation runtime
  semantic NLP capabilities where independently justified
```

The library is a first-class headless product API. The CLI should remain thin. Applications that want Native WhisperX semantics depend on `native-whisperx`; applications building their own transcription product should use the lower reusable capabilities directly.

## Native WhisperX ownership

Native WhisperX owns:

- Workflow Composition: which phases run, in what order, under which product semantics.
- `NativeWhisperxConfig` as the anti-corruption layer around the WhisperX-compatible surface.
- WhisperX aliases, defaults, compatibility validation, and mapping into typed upstream requests.
- Automatic Workflow Selection policy. Selection may expand capability by capability, but must not become a generic planner, scoring engine, or capability graph without demonstrated need.
- Resource policy: which model/resource a workflow requires, accepted product revisions, cache-only versus download-allowed behavior, and user-facing setup/reporting semantics.
- Translation planning: curated language policy, direct versus English-pivot plans, and translation provenance in the product workflow.
- Multi-Input Transcription Run semantics: file ordering, completed/unfinished outcomes, cancellation policy, output collision policy, and aggregate progress/reporting.
- Speaker Directory selection, local/global/explicit conventions, draft/confirmed workflow semantics, Speaker Trace, CLI, and Speaker Directory UI.
- Near-Live Window product policy, Local Ingest Clock semantics, partial/final stabilization, `LiveTranscriptEvent`, and CLI/stdin behavior.
- Output placement, basenames, collisions, Input-Local Output, format selection, and WhisperX-specific serialization.
- One unified Native WhisperX progress stream and product-level outcome contracts.
- Product-owned structured reports and errors.
- The Parity Harness: Python oracle execution, fixtures, tolerances, expected outputs, goldens, parity reports, and performance gates.
- Provider choices only when they represent meaningful user-facing tradeoffs, such as CPU versus CUDA, Q8 versus full precision, or materially different VAD strategies.

It does not own reusable provider implementations merely because parity work needs them.

## Canonical lower-level owners

### `audio-analysis-io`

Owns generic finite-media mechanics: container probing and stream inventory, audio-track selection, FFmpeg invocation, decode, resample/downmix/normalization, and media/decode error contracts.

Native WhisperX owns `--audio-track` semantics and maps them into the upstream source request. It should not maintain a special selected-track predecode path.

### `audio-analysis-transcription`

Owns reusable transcription execution: ASR provider execution and decode controls, VAD execution, alignment implementation, generic pipeline requests/responses, typed provider/runtime options, reusable provider/session state, bounded-window transcription mechanics, lifecycle facts, safe transcription cancellation boundaries, and provider-specific bundle validity for providers implemented there.

Native WhisperX may request reusable or bounded-window sessions, but should not know how Candle, ONNX, or another provider retains model state.

### `audio-analysis-speakers`

Owns reusable speaker identity mechanics: `SpeakerLibrary` / `SpeakerProfile` contracts, embeddings and matching, reusable profile load/save/validate/edit/delete lifecycle, and speaker-provider bundle validity where applicable.

Native WhisperX keeps the Speaker Directory product concept and Speaker Trace provenance.

### shared foundation / `media-core`

Owns neutral cross-domain mechanics, including generic cooperative cancellation, generic model/resource infrastructure, and neutral timed-text interchange/formatting where applicable.

The NLP architecture establishes neutral transcript/timed-text contracts and generic SRT/WebVTT/plain-text/other format-only rendering below NLP rather than in `text-transcripts`. The current migration owner is `moritzbrantner/moenarch-foundation#35`. Native WhisperX keeps WhisperX-specific subtitle-option mapping, output placement, WhisperX JSON, and parity goldens.

Provider-specific bundle schemas remain with the provider that consumes the bundle.

### reusable translation execution

Translation execution is reusable and therefore should not remain a permanent Native WhisperX implementation detail, but its final semantic owner is currently unresolved.

The current NLP architecture explicitly removes `text-model-runtime` as a durable layer and warns against moving Marian/OPUS-MT into another generic runtime abstraction. Therefore Native WhisperX must **not** implement the earlier plan to move Marian execution into `text-model-runtime`.

Until a focused translation ownership decision is made:

- Native WhisperX keeps its existing translation execution as transitional code rather than moving it to the wrong owner.
- No new generic translation/runtime crate is created automatically.
- A future extraction should place Marian/OPUS-MT model/tokenizer glue with a semantically justified translation capability or another explicitly approved owner.
- Native WhisperX continues to own `TranslationPlan`, curated language policy, pivot planning, and product workflow integration regardless of the eventual execution owner.

## Resource ownership

Native WhisperX owns resource **policy**, not generic resource **mechanics**.

For example, it may decide that automatic diarization requires a particular pyannote model and that cache-only is a hard no-download guarantee. The underlying runtime owns generic lookup/download/cache/auth/atomic-promotion mechanics, while the pyannote provider owns the exact files, tensor contract, and revisions that constitute a valid bundle.

Invariant:

> The component that consumes a model bundle defines whether that bundle is valid.

Product-facing commands such as `bundle-verify` may remain in the CLI, but call the authoritative provider verifier.

## Public API boundary

The Rust API is a curated product facade, not a convenience re-export hub.

Canonical cross-project domain contracts may cross the boundary when genuinely part of the product API. During the timed-text migration, existing `TranscriptionContract` compatibility may remain, but new product APIs should not deepen dependence on an NLP-owned neutral transcript contract that is itself scheduled to move below NLP.

Do not newly re-export or embed provider pipeline DTOs, Candle/ONNX-specific option types, media probing/inventory types, or low-level VAD/ASR/alignment provider types.

`NativeWhisperxReport` is product-owned. It may contain canonical transcript/timed-text data plus product outputs, workflow-selection facts, and relevant diagnostics/provenance/performance facts, but should not embed `TranscriptionPipelineResponse` as its public shape.

Native WhisperX owns its programmatically meaningful error taxonomy. Upstream errors are translated into stable product categories and may remain only as diagnostic/source detail below that boundary.

Use the pre-1.0 period for one deliberate cleanup of accidental public leakage. Version 1.0 is the point where the curated headless product API is intended to be semver-stable.

## Product configuration

`NativeWhisperxConfig` intentionally duplicates some concepts from typed upstream execution options. That duplication is an anti-corruption boundary, not a defect.

The product layer may accept WhisperX-compatible strings, historical spellings, aliases, and compatibility defaults, then validate and map them into typed upstream options. Upstream libraries should not need to understand every WhisperX CLI spelling.

## Progress and cancellation

Upstream crates own authoritative low-level facts. Native WhisperX owns the unified product narrative.

Examples of upstream facts include model resolution/download/load/reuse, transcription phase starts/ends, provider/session reuse, and safe cancellation boundaries.

Native WhisperX maps those facts into one `TranscriptionProgressEvent` stream alongside product events such as run/file lifecycle, translation legs, output writing, failure, and cancellation.

Generic cancellation belongs in shared runtime/foundation infrastructure. Native WhisperX retains workflow-level cancellation semantics: do not start later phases/files, do not write later outputs, preserve completed files, and return product-specific cancellation outcomes.

## Feature flags and defaults

Feature flags describe Native WhisperX capabilities, not incidental implementation technology.

Library and CLI defaults are aligned. Normal installation should be batteries-included without feature archaeology, while expensive/specialized modes remain opt-in. Compiled-in capabilities must still resolve and load model resources lazily.

Default-off examples include CUDA, Python runtime compatibility/oracle tooling, experimental alternatives, and implementation-specific backend variants. `default-features = false` remains the supported path for deliberately lean embedding builds.

Q8/Int8 is a first-class Native WhisperX extension but remains explicit and non-default. Its execution/bundle implementation is upstream; Native WhisperX owns user-facing constraints and early validation.

## Compatibility policy

Native WhisperX is a WhisperX-compatible product that may extend beyond WhisperX.

On overlapping surfaces, WhisperX-compatible observable semantics win by default. Native improvements should be additive and explicit rather than silently redefining existing WhisperX options.

Python WhisperX runtime delegation is transitional. The destination is native product execution plus Python WhisperX only as parity oracle/reference/golden source. Do not add new automatic Python fallback behavior.

The Parity Harness remains in `native-whisperx` even when individual comparison helpers look reusable. Extract common comparison primitives only after a second real consumer demonstrates the same requirements.

## Moving WhisperX target

Two versions are tracked:

- **Upstream Target**: the latest released WhisperX version the project is expected to catch up to.
- **Verified Compatibility Baseline**: the newest WhisperX version for which gating parity evidence currently passes.

A single machine-readable policy at
[`tests/parity/whisperx-version.json`](../tests/parity/whisperx-version.json)
records the Verified Compatibility Baseline. Parity preflight gates and
non-gating upstream reconnaissance both consume that policy; latest-version
discovery stays outside the gating path.

A new release may put the project in a documented `behind upstream` state without making deterministic `main` CI red immediately. Compatibility work then advances the verified baseline once evidence passes.

## Extraction rule

Composition-only does not mean extracting every helper. Move code when it represents a reusable capability with a clear lower-level owner. Keep code here when it exists specifically to define or prove Native WhisperX product behavior.

If a reusable seam is real but its owner is unresolved, do not invent a generic layer merely to satisfy the composition-only rule. Record the transitional debt and return the ownership decision to architecture shaping first.

Examples that stay here include parity fixture orchestration/tolerances, WhisperX compatibility mapping, automatic workflow policy, product reports/errors/progress composition, Speaker Trace, and Speaker Directory semantics.

Examples that move down include reusable inference mechanics, neutral timed-text formatting, media decode/probe mechanics, reusable speaker-profile lifecycle, bounded-window transcription mechanics, generic provider session reuse, generic cancellation, and model resource infrastructure.

Do not create a new crate solely to move code out of this repository. New crates require an independent API/versioning reason.

# Native WhisperX Architecture

## Purpose

`native-whisperx` is the headless product engine and CLI composition layer for a Rust-first, WhisperX-compatible transcription product. It owns user-visible workflow semantics. Reusable execution capabilities live in their canonical lower-level crates.

The permanent ownership test is:

> If a capability would still make sense in a Rust transcription application that does not use Native WhisperX semantics, it does not belong in `native-whisperx`.

This is a strict composition-only rule. Native WhisperX may coordinate, validate, translate product configuration, report progress, write product outputs, and prove compatibility. It must not become the implementation home for reusable ASR, VAD, alignment, diarization, model-runtime, translation-runtime, media-decoding, transcript-rendering, cancellation, or speaker-storage machinery.

## Product Layers

```text
native-whisperx-cli
  terminal parsing, stdout/stderr, TTY rendering, shell-facing adapters
        |
        v
native-whisperx
  WhisperX-compatible product configuration
  workflow composition and automatic selection policy
  multi-input/output/speaker/live product semantics
  unified progress, reports, errors, parity, compatibility
        |
        v
reusable Rust capabilities
  audio-analysis-io
  audio-analysis-transcription
  audio-analysis-speakers
  text-model-runtime
  text-transcripts
  model/runtime/foundation crates
```

The library is a first-class headless product API. The CLI should remain thin. Applications that want Native WhisperX semantics depend on `native-whisperx`; applications building their own transcription product should use the lower reusable crates directly.

## Ownership Matrix

### `native-whisperx`

Owns:

- Workflow Composition: which phases run, in what order, under which product semantics.
- `NativeWhisperxConfig` as the anti-corruption layer around the WhisperX-compatible product surface.
- WhisperX aliases, defaults, compatibility validation, and mapping into typed upstream requests.
- Automatic Workflow Selection policy. Selection may expand capability by capability, but must not become a generic planner, scoring engine, or capability graph without demonstrated need.
- Resource policy: which model/resource a workflow requires, accepted product revisions, cache-only versus download-allowed policy, and user-facing setup/reporting semantics.
- Translation planning: curated language policy, direct versus English-pivot plans, and translation provenance in the product workflow.
- Multi-Input Transcription Run semantics: file ordering, completed/unfinished outcomes, cancellation policy, output collision policy, and aggregate progress/reporting.
- Speaker Directory selection, local/global/explicit directory conventions, draft/confirmed workflow semantics, Speaker Trace, CLI, and Speaker Directory UI.
- Near-Live Window product policy, Local Ingest Clock semantics, partial/final stabilization, `LiveTranscriptEvent`, and CLI/stdin behavior.
- Output placement, basenames, collisions, Input-Local Output, format selection, and WhisperX-specific serialization.
- One unified Native WhisperX progress stream and product-level outcome contracts.
- Product-owned structured reports and errors.
- The Parity Harness: Python oracle execution, fixtures, tolerances, expected outputs, goldens, parity reports, and performance gates.
- Provider choices only when they represent meaningful user-facing tradeoffs, such as CPU versus CUDA, Q8 versus full precision, or materially different VAD strategies.

Does not own reusable provider implementations merely because parity work needs them.

### `audio-analysis-io`

Owns generic finite-media mechanics:

- container probing and stream inventory;
- audio-track selection;
- FFmpeg invocation;
- decode, resample, downmix, and normalization;
- media/decode error contracts.

Native WhisperX owns the `--audio-track` product semantics and maps them into the upstream source request. Native WhisperX should not call `ffprobe`/`ffmpeg` directly or maintain a special predecode path.

### `audio-analysis-transcription`

Owns reusable transcription execution:

- ASR provider execution and decode controls;
- VAD execution;
- alignment implementation;
- generic transcription pipeline requests/responses;
- typed provider/runtime options;
- reusable provider/session state and compatibility checks for model reuse;
- generic bounded-window transcription sessions for live/near-live input;
- upstream lifecycle/progress facts for transcription and model use;
- safe cancellation boundaries for transcription phases;
- provider-specific model bundle validity where the provider lives in this crate.

Native WhisperX may request a reusable session or bounded-window session, but should not know how Candle/ONNX/other provider state is retained.

### `audio-analysis-speakers`

Owns reusable speaker identity mechanics:

- `SpeakerLibrary` and `SpeakerProfile` contracts;
- embeddings, identification, and matching;
- load/save/validate/edit/delete lifecycle for reusable speaker profiles;
- speaker-provider-specific bundle validation where applicable.

Native WhisperX keeps the Speaker Directory product concept and Speaker Trace provenance. Stable identity and derived transcript provenance remain separate.

### `text-model-runtime`

Owns reusable Marian/OPUS-MT execution now:

- model loading;
- SentencePiece/tokenization runtime;
- weights and inference;
- runtime/device/cache integration;
- generic model lifecycle events.

Native WhisperX keeps `TranslationPlan`, curated language policy, pivot planning, and workflow integration. A dedicated translation crate should exist only if translation develops an independently meaningful API/versioning surface with multiple consumers or substantial translation-specific routing/quality/batching policy.

### `text-transcripts`

Owns canonical transcript data plus generic renderers:

- `TranscriptionContract` and related canonical transcript domain contracts;
- generic SRT, WebVTT, TXT, TSV, and Audacity renderers;
- generic subtitle cue/wrapping primitives.

It must not become WhisperX-aware. Native WhisperX maps WhisperX subtitle flags into generic renderer options and retains WhisperX JSON compatibility and parity goldens.

### shared runtime/foundation

Owns generic cloneable sticky cancellation and generic model/resource infrastructure such as cache/download/checksum/atomic-promotion mechanics. Provider-specific bundle schemas remain with the provider that consumes the bundle.

## Resource Ownership

Native WhisperX owns resource **policy**, not generic resource **mechanics**.

For example, Native WhisperX may decide that automatic diarization requires a particular pyannote model and that cache-only is a hard no-download guarantee. The underlying runtime owns Hugging Face lookup/download/cache/auth/atomic-promotion mechanics, while the pyannote provider owns the exact files, tensor contract, and revisions that constitute a valid bundle.

Invariant:

> The component that consumes a model bundle defines whether that bundle is valid.

Product-facing commands such as `bundle-verify` may remain in the CLI, but they call the authoritative provider verifier.

## Public API Boundary

The Rust API is a curated product facade, not a convenience re-export hub.

Canonical cross-project domain contracts may cross the boundary when they are genuinely part of the product API, especially `TranscriptionContract`. Low-level implementation types must not become part of Native WhisperX's compatibility surface merely for convenience.

Do not newly re-export or embed types such as:

- provider pipeline request/response DTOs;
- Candle/ONNX-specific compute/runtime option types;
- media probing/inventory types;
- low-level VAD/ASR/alignment provider types.

`NativeWhisperxReport` is product-owned. It may contain canonical transcript contracts plus product outputs, workflow-selection facts, relevant diagnostics/provenance/performance facts, but it should not embed `TranscriptionPipelineResponse` as its public shape.

Native WhisperX also owns its programmatically meaningful error taxonomy. Upstream errors are translated into stable product categories and may remain only as diagnostic/source detail below that boundary.

The pre-1.0 period should be used for one deliberate cleanup of accidental public leakage. Preserve compatibility shims or deprecations when cheap and useful, especially for the CLI, but do not let hypothetical compatibility prevent the intended 1.0 architecture. Version 1.0 is the point where the curated headless product API is intended to be semver-stable.

## Product Configuration

`NativeWhisperxConfig` intentionally duplicates some concepts from typed upstream execution options. That duplication is an anti-corruption boundary, not a defect.

The product layer may accept WhisperX-compatible strings, historical spellings, aliases, and compatibility defaults, then validate and map them into typed upstream enums/options. Upstream libraries should not need to understand every WhisperX CLI spelling.

The same rule applies to decode controls: `WhisperxDecodeConfig` belongs to the product compatibility contract even though actual decoding belongs upstream.

## Progress and Cancellation

Upstream crates own authoritative low-level facts. Native WhisperX owns the unified product narrative.

Examples of upstream facts:

- model resolution/download/load/reuse;
- transcription phase starts/ends;
- reusable-provider/session reuse;
- safe cancellation boundaries.

Native WhisperX maps those facts into one `TranscriptionProgressEvent` stream alongside product events such as run/file lifecycle, translation legs, output writing, failure, and cancellation. Embedding applications should not have to subscribe to several internal observer systems to render Native WhisperX progress.

Generic cancellation tokens belong in shared runtime/foundation infrastructure. Native WhisperX retains workflow-level cancellation semantics: do not start later phases/files, do not write later outputs, preserve already completed files, and return product-specific cancellation outcomes.

## Feature Flags and Defaults

Feature flags describe Native WhisperX capabilities, not incidental implementation technology.

A public feature may name a user-visible capability or meaningful deployment choice. Internal backend technology should not leak upward just because an upstream implementation currently uses it.

Library and CLI defaults are aligned. The normal installation should be batteries-included without feature archaeology, while expensive/specialized modes remain opt-in.

Default-on capabilities should include normal CPU/native transcription, finite media input, generic outputs, translation capability, and the normal automatic high-quality diarization path, while still resolving/loading models lazily. Default builds must not download or load unrelated models during help/startup.

Default-off examples include CUDA, Python runtime compatibility/oracle tooling, experimental alternatives, and implementation-specific backend variants. `default-features = false` remains the supported path for deliberately lean embedding builds.

Q8/Int8 is a first-class Native WhisperX extension but remains explicit and non-default. Its execution/bundle implementation is upstream; Native WhisperX owns user-facing constraints and early validation. Restrictions may be relaxed as upstream capabilities improve.

## Compatibility Policy

Native WhisperX is a WhisperX-compatible product that may extend beyond WhisperX.

On overlapping surfaces, WhisperX-compatible observable semantics win by default. Native improvements should be additive and explicit rather than silently redefining existing WhisperX options. Native-only capabilities such as Automatic Workflow Selection, Speaker Directory workflows, near-live transcription, or Q8 do not require a Python equivalent.

Python WhisperX runtime delegation is transitional. It remains useful while native coverage converges, but the destination is:

- native execution for the product;
- Python WhisperX only as parity oracle/reference/golden source.

Do not add new automatic Python fallback behavior. Unsupported native combinations should fail explicitly rather than silently delegating.

The Parity Harness remains in `native-whisperx` even when individual comparison helpers look reusable. Extract generic transcript-comparison primitives only after a second real consumer demonstrates the same requirements.

## Moving WhisperX Target

Two versions are tracked conceptually:

- **Upstream Target**: the latest released WhisperX version. This is where Native WhisperX is expected to catch up.
- **Verified Compatibility Baseline**: the newest WhisperX version for which the gating parity evidence currently passes.

A new WhisperX release may put the project in a documented `behind upstream` state without making otherwise deterministic `main` CI red immediately. Non-gating reconnaissance should identify the delta; compatibility work then advances the verified baseline once evidence passes.

The project should not silently remain on an old compatibility target indefinitely merely because the old baseline still passes.

## Automatic Selection

Automatic Workflow Selection remains explicit product policy, implemented capability by capability. Shared decision-reporting structures are acceptable, but do not introduce a generic workflow planner before concrete repeated needs exist.

Prefer narrow functions such as VAD, diarization, alignment, or translation policy resolvers over a general provider-negotiation framework.

## Extraction Rule

Composition-only does not mean extracting every helper.

Move code upstream when it represents a reusable capability whose natural owner is lower-level. Keep code here when it exists specifically to define or prove Native WhisperX product behavior.

Examples that stay here:

- parity fixture orchestration and tolerances;
- WhisperX output compatibility mapping;
- automatic workflow policy;
- product reports/errors/progress composition;
- Speaker Trace and Speaker Directory product semantics.

Examples that move upstream:

- reusable inference/execution mechanics;
- generic transcript writers;
- media decode/probe mechanics;
- reusable speaker-profile lifecycle;
- generic bounded-window transcription mechanics;
- generic provider session reuse;
- generic cancellation and model resource infrastructure.

Do not create a new crate solely to move code out of this repository. New crates require an independent API/versioning reason.
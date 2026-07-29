# Feature Contract: Selected-Media Input Compatibility

**Status:** Refactor-ready

## Capability

**Actor:** A Rust embedding application that already constructs or matches
native-whisperx finite-input configuration.

**Outcome:** The application can opt into an explicit container audio-stream
ordinal without changing the legacy input enum or its serialized config shape.

## Public Interface

- Stable entrypoint: `InputSource`, `NativeWhisperxConfig`,
  `NativeWhisperxError`, `SelectedMediaInput`, `SelectedMediaError`, and the
  `run_selected_media*` and `run_many_selected_media*` workflow functions.
- Inputs and preconditions: selected-media workflows receive a config whose
  input is `InputSource::Path` plus a separate zero-based audio-stream ordinal.
- Outputs and externally visible side effects: legacy configs serialize their
  input exactly as before; selected workflows return the same report or
  cooperative-cancellation outcome families as their legacy counterparts.
- Errors and invariants: `InputSource` remains exactly `Path | Samples`, and
  `NativeWhisperxError` retains its legacy exhaustive variants.
  `SelectedMediaInput::audio_track` is an audio ordinal, not a global container
  stream index. Typed selection reason and inventory are exposed only through
  the additive `SelectedMediaError`.

## Protected Behaviors

| ID | Scenario and observable outcome | Verification | Sensitivity |
| --- | --- | --- | --- |
| FC-001 | A downstream-style exhaustive match over `InputSource::Path` and `InputSource::Samples` compiles without a wildcard arm. | `crates/native-whisperx/src/lib.rs`: `legacy_input_source_remains_exhaustively_matchable` | Proven: a temporary third variant produced the intended non-exhaustive-match compiler error; removing it restored green. |
| FC-002 | A path-based `NativeWhisperxConfig` keeps the legacy `{"kind":"path","path":...}` input JSON, while `SelectedMediaInput` serializes separately. | `crates/native-whisperx/src/lib.rs`: `selected_media_api_keeps_legacy_config_input_serialization_unchanged` | Proven: a temporary serde rename of the path variant changed `kind` and produced the intended assertion failure; restoring the original representation returned green. |
| FC-003 | A downstream exhaustive match over every legacy `NativeWhisperxError` variant compiles without a wildcard arm. | `crates/native-whisperx/tests/finite_progress.rs`: `legacy_native_whisperx_error_remains_exhaustively_matchable_downstream` | Proven by TDD: the selected-media variant caused the intended non-exhaustive-match compiler error before the typed payload moved to `SelectedMediaError`; the unchanged legacy enum then returned green. |
| FC-004 | Invalid selected audio ordinals return `SelectedMediaError::StreamSelection` with typed `OutOfRange` or `NoAudioStreams` reason and the full probed stream inventory. | `crates/native-whisperx/src/lib.rs`: `invalid_audio_track_reports_available_streams_before_provider_work`, `selected_audio_ordinal_rejects_video_only_inventory_as_no_audio_streams` | Proven by TDD: removing the legacy error variant initially made the typed-pattern tests fail to compile until the selected entrypoint result type exposed the additive wrapper. |
| FC-005 | Canonical legacy entrypoints return `NativeWhisperxError`, while canonical selected-media entrypoints return `SelectedMediaError`. | `crates/native-whisperx/tests/finite_progress.rs`: `legacy_and_selected_entrypoints_keep_separate_error_types_downstream` | Proven by TDD: assigning the selected entrypoints while they still returned `NativeWhisperxError` produced a function-pointer type mismatch; the additive result types restored green. |

## Equivalent Scenarios

| ID | Scenario | Covered by | Reason |
| --- | --- | --- | --- |
| FC-006 | Existing `Samples` callers remain source compatible. | FC-001 | The exhaustive two-arm match protects the complete enum variant set. |
| FC-007 | Selected-media entrypoints do not embed the selector into legacy config JSON. | FC-002 | The test verifies the legacy config and additive selector as separate serialized values. |

## Intent Decisions

- Separate `SelectedMediaInput` plus canonical selected-media workflow
  entrypoints is the non-breaking representation.
- Adding a selected-media variant to `InputSource` is rejected because public
  Rust enums permit downstream exhaustive matches.
- Adding selected-media variants to `NativeWhisperxError` is rejected for the
  same reason. New selected entrypoints return `SelectedMediaError`; legacy
  entrypoints retain `Result<_, NativeWhisperxError>`.
- The selected ordinal is layered onto an existing `InputSource::Path`; legacy
  workflow functions retain FFmpeg default-stream behavior.

## Explicit Exclusions

| ID | Scenario or promise outside this contract | Reason |
| --- | --- | --- |
| FC-008 | Selecting by global container stream index. | The public selector intentionally follows FFmpeg `0:a:N` audio-ordinal semantics. |
| FC-009 | Serializing selected-media state inside `NativeWhisperxConfig::input`. | That would change the protected legacy config schema. |
| FC-010 | Returning typed selected-media state through `NativeWhisperxError`. | The additive `SelectedMediaError` protects exhaustive legacy error matches. |

## Approved Sensitivity Exceptions

| Clause | Missing proof | Compensating validation | Confidence gap |
| --- | --- | --- | --- |
| None | | | |

## Verification Commands

```text
cargo test -p native-whisperx legacy_input_source_remains_exhaustively_matchable
cargo test -p native-whisperx --test finite_progress legacy_native_whisperx_error_remains_exhaustively_matchable_downstream
cargo test -p native-whisperx selected_media_api_keeps_legacy_config_input_serialization_unchanged
cargo test -p native-whisperx --features media-decode audio_track
cargo test -p native-whisperx --test finite_progress
cargo test --workspace
```

## Change Protocol

Any intentional change to this capability must update this contract and its
mapped tests in the same change. Behavior-preserving refactors must leave the
contract unchanged and keep every mapped verification green.

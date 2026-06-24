# TDD Workflow

Use this guide for substantial Workflow Composition changes and any slice that
calls for TDD. GitHub Issues remain the durable source of truth for PRDs,
implementation slices, acceptance criteria, and out-of-scope boundaries.

## Loop

TDD in native-whisperx means one failing observable behavior test, one minimal
implementation, one green check, then the next behavior.

Do not use horizontal slicing for TDD work. Do not write a broad suite of
imagined tests before implementation, then fill in all of the code later. Each
red-green-refactor cycle should prove one behavior through the highest useful
public interface, then stop and choose the next behavior from what was learned.

Refactor only while green. A refactor should preserve the public behavior that
the current tests already prove.

## Test Seams

Prefer existing public seams over new abstractions:

1. CLI command behavior for user-visible native-whisperx behavior.
2. Exported library workflow interfaces for reusable Workflow Composition
   logic.
3. Checked-in fixtures for deterministic transcript, WhisperX JSON, Native
   JSON, output writing, Speaker Directory, Speaker Library, or Speaker Trace
   behavior that does not require model execution.
4. Parity Harness commands when the behavior is a WhisperX Parity or
   Rust-Native Parity surface.
5. Ignored or manual real-resource checks when validation requires local audio,
   model bundles, Python WhisperX, ONNX Runtime, CUDA, Hugging Face access, or
   self-hosted parity resources.

Tests should verify public behavior through CLI commands, exported library
workflow interfaces, checked-in fixtures, or parity commands. They should not
verify private helper names, internal call order, private implementation
details, or mocked internal Rust modules.

Mocks, fakes, or adapters belong at system boundaries: Python WhisperX, model
caches, ONNX Runtime, CUDA, Hugging Face access, filesystem resources, and
time or randomness where relevant. Do not introduce a seam just to mock Rust
code that is already inside the module under test.

## Domain Vocabulary

Use the repository language from `CONTEXT.md` in issue briefs, test names, and
review notes. Preserve these terms when they apply:

- Workflow Composition
- WhisperX Parity
- Rust-Native Parity
- Parity Harness
- Speaker Directory
- Speaker Library
- Speaker Trace
- identity-versus-trace separation

For speaker workflows, stable identity lives in the Speaker Library and derived
file, span, snippet, and run provenance lives in the Speaker Trace. Tests should
prove that behavior through public command or library behavior instead of
reaching into private storage helpers.

## Command Ladder

Start each cycle with the narrowest meaningful command that can prove the
behavior is missing or fixed.

- Red check: a single integration or library test, such as
  `cargo test -p native-whisperx-cli <test-name>` or
  `cargo test -p native-whisperx <test-name>`.
- Green check: rerun the same single test after the minimal implementation.
- Workspace checks before handoff: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`, and
  `cargo test --workspace`.
- No-default-feature check when the slice can affect feature boundaries:
  `cargo test --workspace --no-default-features`.
- Optional feature compile or test checks when the issue names a feature flag.
- Parity checks when the behavior is part of WhisperX Parity or Rust-Native
  Parity and the required resources are available.
- Ignored or manual smoke checks only when the issue explicitly calls for
  model-backed, Python WhisperX, ONNX Runtime, CUDA, Hugging Face, local audio,
  or self-hosted parity validation.

Default PR validation must remain offline. It must not require model bundles,
Python WhisperX, CUDA devices, Hugging Face tokens, ONNX Runtime configuration,
or self-hosted parity resources.

## Test Names

Behavior-focused names:

- `cli_expands_quoted_input_patterns_before_transcription`
- `multi_input_run_writes_default_outputs_beside_each_input`
- `whisperx_json_remains_the_default_json_output_contract`
- `speaker_trace_rebuild_preserves_speaker_library_identity`
- `parity_harness_reports_whisperx_json_differences`

Implementation-detail names to avoid:

- `parse_args_calls_expand_glob_helper`
- `writer_uses_internal_segment_vec_layout`
- `speaker_store_private_map_has_expected_keys`
- `mock_diarizer_receives_exact_method_order`
- `alignment_module_sets_private_flag`

## Future Issue Template

Use this shape for future TDD-ready implementation slices:

```markdown
## Parent

#<parent-prd-issue-number>

## Behavior

Describe the observable behavior in native-whisperx domain language.

## Public Interface

Name the CLI command, exported library workflow interface, checked-in fixture,
or Parity Harness command that proves the behavior.

## First Red Test

Name the first failing behavior test and the narrow command that should fail
before implementation.

## Green Implementation

Describe the minimal implementation expected to make that behavior pass.

## Follow-up Behaviors

List later behaviors separately. Do not write them as a horizontal suite before
the first behavior is green.

## Checks

List the focused red/green command and broader handoff checks.

## Out Of Scope

List behavior, tooling, CI, framework, resource, and parity changes that the
slice must not make.
```

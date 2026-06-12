# Workflow Composition

This context owns native-whisperx workflow language: how transcription, alignment,
diarization, output writing, and parity are composed into user-facing workflows.

## Language

**Workflow Composition**:
The orchestration of ASR, VAD, alignment, diarization, output writing, parity,
and CLI behavior into one user-facing transcription workflow.
_Avoid_: primitive ownership, model implementation

**WhisperX Parity**:
Feature compatibility with the Python WhisperX user-facing surface. The first
normative surface is the Python WhisperX CLI.
_Avoid_: loose similarity, best-effort compatibility

**Native**:
The Rust-first direction of the project. Native does not mean every current
feature is Rust-only.
_Avoid_: Rust-only

**Delegated Feature**:
A feature satisfied by calling Python WhisperX or a Python dependency while a
Rust implementation is planned.
_Avoid_: unsupported feature, external-only feature

**Parity Harness**:
The fixtures, runner, structured diff, and reports used to compare
native-whisperx behavior against Python WhisperX.
_Avoid_: smoke test, fixture check

**WhisperX JSON**:
The WhisperX-compatible JSON transcript contract exposed to users by default.
_Avoid_: native JSON

**Native JSON**:
The explicit JSON representation of the Rust transcript contract.
_Avoid_: default JSON, WhisperX JSON

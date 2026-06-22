# ADR 0011: Near-live windows before true streaming

## Status

Accepted.

## Context

native-whisperx already owns finite-file Workflow Composition: bounded inputs
flow through ASR, optional alignment and diarization, and output writers under
the WhisperX Parity and Rust-Native Parity contracts. Users also need Live Feed
Transcription for sources that are still arriving, but the current native ASR
pipeline accepts finite audio and finite speech chunks.

True provider-level streaming would require a reusable streaming provider
contract below the CLI. Shipping that design inside the first live feed slice
would expand the scope beyond the immediate user need and risk implying timing
accuracy that the first workflow does not own.

## Decision

Start Live Feed Transcription with Near-Live Windows before true provider-level
streaming. The first workflow decodes an audio or video feed into rolling finite
windows, runs those bounded windows through native ASR, and emits Live
Transcript Events as newline-delimited JSON on stdout.

The first workflow is native ASR-only and JSONL stdout only. It does not write
transcript files and does not emit WhisperX JSON files or Native JSON aggregate
files.

Live event timing is based on the Local Ingest Clock: local UTC timestamps and
feed-relative seconds derived from when decoded samples enter the live session.
The first workflow does not preserve or claim source PTS or broadcast timecode
accuracy.

## Deferred Scope

The first live feed workflow explicitly defers:

- True provider-level streaming session APIs.
- Alignment.
- Diarization.
- Translation.
- Subtitle outputs.
- File outputs.
- WhisperX JSON files.
- Native JSON aggregate files.
- External WhisperX provider support.
- Source PTS.
- Broadcast timecode.

## Consequences

The first live feed surface can reuse the existing native ASR pipeline without
changing finite-file `transcribe` behavior, Input Pattern Expansion,
Multi-Input Transcription Run output placement, WhisperX JSON, or Native JSON
contracts.

The live output contract is intentionally separate from WhisperX-compatible
file outputs. Downstream consumers should treat Live Transcript Events as a
JSONL event stream, not as incrementally appended WhisperX JSON.

Future work may add true provider-level streaming or source-synchronized timing
through a separate PRD and ADR. That work should decide how provider streaming,
source PTS, broadcast timecode, alignment, diarization, translation, and output
writers compose with the existing parity contracts.

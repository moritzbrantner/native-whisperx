# Native transcribe progress uses stdout and reports use --report

## Status

Accepted

## Context

Native finite transcription used to print the final `NativeWhisperxReport` JSON
to stdout after the workflow completed. That made automation simple, but normal
native runs had no clear live feedback while decode, VAD, ASR, alignment,
diarization, translation, model loading, output writing, or failure handling was
happening.

The repository now names this live user-facing surface the `Transcription
Progress Stream`. It is separate from transcript output files, final report
JSON, parity reports, and diagnostics.

## Decision

Native `transcribe` emits the `Transcription Progress Stream` on stdout by
default. Interactive terminals render progress with `indicatif`; redirected and
CI stdout use stable plain progress lines with no terminal rewrite behavior.

The former machine-readable final report JSON moves behind an explicit
`--report <PATH>` option. A single-input run writes the existing single
`NativeWhisperxReport` object shape. A `Multi-Input Transcription Run` writes
the existing report array shape.

External WhisperX delegation remains outside the normalized native progress
event model. Existing delegated logging/progress argument forwarding remains
compatible.

## Consequences

Users get immediate native workflow status on stdout for normal terminal and CI
runs. Automation that needs the final report must opt in with `--report <PATH>`.
Transcript output files and `Input-Local Output` placement rules remain
unchanged.

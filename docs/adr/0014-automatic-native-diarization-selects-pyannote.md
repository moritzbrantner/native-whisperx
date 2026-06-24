# Automatic native diarization selects the pyannote pair

Native finite `--diarize` may use Automatic Workflow Selection to choose
pyannote VAD plus `pyannote/speaker-diarization-community-1` when the user has
not explicitly selected lower-level VAD or diarization model settings.

## Context

Users should be able to request diarization without memorizing the native
pyannote VAD, pyannote diarization, bundle, cache, and authentication matrix.
The quality-preserving automatic path is narrower than general model guessing:
it applies to native finite Workflow Composition for diarization, and only when
VAD and diarization model choices are otherwise unspecified.

ADR 0005 kept native pyannote VAD explicit unless a compatible local ONNX path
was supplied. That decision protected Rust-Native Parity from silently treating
pyannote as energy or Silero VAD.

## Decision

For automatic native finite `--diarize`, select the pyannote VAD plus
`pyannote/speaker-diarization-community-1` diarization pair. Automatic
selection must either resolve the resources required by that pair or fail with
actionable setup guidance. It must not silently fall back to energy VAD, Silero
VAD, heuristic diarization, or external WhisperX delegation.

Native automatic downloads use environment or standard Hugging Face
authentication state. They do not require passing Hugging Face token strings
through CLI arguments, and diagnostics or reports must not expose secret token
values.

Explicit user-selected settings win over automatic selection. A user can still
choose lower-quality or resource-constrained native VAD and diarization
settings, including explicit energy or Silero VAD and explicit native
diarization model or bundle configuration. Those choices remain reproducible
and should be reported as explicit rather than automatic.

This ADR narrows and supersedes ADR 0005 only for the automatic native finite
`--diarize` case. ADR 0005 remains in force for explicit
`--vad-method pyannote`: explicit pyannote VAD is still not an alias for energy
or Silero and still requires the compatible feature/resource path.

## Consequences

The public configuration model needs to preserve whether VAD and diarization
model settings were automatic or explicit. Later resolver work can use that
metadata to resolve resources, respect cache-only mode, and surface structured
selection diagnostics without changing WhisperX JSON, Native JSON, subtitle, or
text transcript contracts.

Speaker Directory, Speaker Library, Speaker Trace, and identity-versus-trace
separation are unchanged by this decision.

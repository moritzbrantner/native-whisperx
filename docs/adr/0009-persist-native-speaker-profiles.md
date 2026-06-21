# ADR 0009: Persist Native Speaker Profiles In The Speaker Directory

## Status

Accepted.

## Context

Native diarization can emit transient speaker labels such as `speaker_0`, but
repeat transcription workflows need a stable place to store corrected speaker
identities and reusable embeddings. Earlier planning used the term "speaker
store"; later architecture established the Speaker Directory as the user-facing
storage root with separate identity and trace files.

## Decision

Persist reusable native speaker identity in `library.json` inside the Speaker
Directory. The file remains the canonical Speaker Library and stores profile
ids, labels, metadata, and embeddings. `speaker-trace.json` remains derived
provenance and must not become an identity source of truth.

The CLI accepts `--speaker-store` as a compatibility alias for
`--speaker-directory`, but the canonical model is still Speaker Directory.

Native diarization may load confirmed and draft profiles from the Speaker
Library. Newly detected native-compatible transient speakers may be saved as
draft profiles, and users can correct a transcript speaker label into a
confirmed profile. Corrected transcript output is written as a copy through the
normal output writer; raw original output files are not edited in place by
default.

External WhisperX remains parity-focused and does not use native-whisperx
speaker profiles for recognition.

## Consequences

- Stable speaker identity and derived trace data stay separated.
- Draft profiles can improve repeat recognition without pretending they are
  confirmed identities.
- Existing Speaker Directory commands and web UI remain aligned with the same
  `library.json` file.
- Compatibility aliases keep earlier speaker-store command examples usable
  without introducing a second storage system.

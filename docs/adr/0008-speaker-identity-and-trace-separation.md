# ADR 0008: Keep Speaker Library identity separate from Speaker Trace provenance

## Status

Accepted.

## Context

The Speaker Directory workflow needs a reusable source of enrolled speaker
identities and a way to inspect where those speakers appear across transcript
outputs. These are different kinds of data: identity is stable and used by
native diarization, while trace provenance is derived from transcript files and
can be rebuilt.

The upstream speaker crate already defines the Speaker Library snapshot format
for enrolled profiles. That format stores profile ids, labels, metadata, and
embeddings. It is not a transcript index.

## Decision

A Speaker Directory stores canonical speaker identity in `library.json` using
the upstream Speaker Library snapshot format.

Speaker appearance provenance belongs in a separate native-whisperx-owned
`speaker-trace.json` file. The trace is derived data rebuilt from structured
transcript JSON outputs and must not be embedded in `library.json`.

Default Speaker Directory resolution is local-first: an existing project-local
`.native-whisperx/speakers` directory wins. Users may force the global Speaker
Directory under the platform data-directory convention at
`<data-dir>/native-whisperx/speakers`, or pass an explicit Speaker Directory
path.

## Consequences

Speaker profile ids remain stable while labels and metadata can evolve without
mixing in file history. Deleting or rebuilding trace data cannot corrupt enrolled
speaker identities.

Anonymous Speaker Labels from transcript JSON stay visible as trace data until
they exactly match an enrolled profile id or label. They do not automatically
become enrolled identities.

Validation for `library.json` can rely on the upstream Speaker Library snapshot
parser and reject non-canonical top-level trace/provenance fields before the
file is used by future native diarization work.

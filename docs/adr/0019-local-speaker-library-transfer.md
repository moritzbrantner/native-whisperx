# ADR 0019: Local Speaker Library transfer and selection

Status: Accepted

## Context

The local workbench needs a reusable identity index without treating derived
Speaker Trace data as identity storage. Embeddings require deliberate export and
import consent. The selected index must also be the index used by transcription.

## Decision

Audio Analysis owns canonical Speaker Library serialization, model compatibility,
merge conflicts, guarded commit and preservation of profile identity/status.
Native WhisperX owns directory selection, consent, backup paths and presentation.
Both local views use the same selection and existing profile editor.

A session holds one opaque pending preview. Confirmation consumes that preview
once, binds the reviewed snapshot and requires explicit consent. Cancellation,
new previews and selection changes invalidate it. Preview and repeated identical
imports do not write files, including when the destination directory is absent.
Exports never overwrite existing files. Changed imports preserve the previous
canonical file in a uniquely named `.backup` file before replacing it.

HTTP summaries contain paths, counts and statuses, never embedding vectors.
Explicit paths stay within the launch directory after canonical resolution.
Existing Auto/Local/Global selection and the initial CLI directory are purpose-bound
grants using the existing resolver. A running native worker blocks selection and
transfer changes. Later runs use the actual selected directory.

Speaker Trace files are neither transferred nor rewritten. An unreadable derived
trace is reported as invalid independently of the usable Speaker Library.

Draft auto-update is an explicit per-run checkbox, off by default in the local
workbench. It requires speaker identification. Compatible unmatched identities
remain drafts; nothing automatically confirms them or converts model families.
Embeddings remain internal to native matching/persistence. CLI defaults remain
unchanged.

## Consequences

Portable files retain canonical model metadata, stable IDs, labels, embeddings
and confirmed/draft status. Model mismatch and merge identity/label conflicts
require a corrected source or explicit whole-library replacement. The original
file remains recoverable after replacement. There is no remote upload service.

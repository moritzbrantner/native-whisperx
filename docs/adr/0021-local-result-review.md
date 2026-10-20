# ADR 0021: Review native results as separate corrected copies

Status: Accepted

## Context

The local app needs synchronized source playback and manual transcript corrections
without replacing original results or introducing a second inference executor.
Neutral timing validation and format rendering already live in media-core;
Native WhisperX owns correction workflow, WhisperX projection and copy policy.

## Decision

Review opens a recorded completed result by an opaque result ID. Jobs IDs bind a
batch and file ordinal; single-file session results have session-scoped IDs.
Saved corrected copies have persistent opaque IDs in the existing small local
history index. Paths and transcript contents never appear in navigation URLs.
Stable segment IDs and a source/translation layer can be deep-linked.

A session holds at most four bounded result snapshots. Typed previews bind edits
to the reviewed snapshot; save requires the current opaque preview ID and consumes
it after successful registration. Client draft fingerprints prevent a late
preview response from becoming saveable after further edits. Reset discards
local changes and the pending preview without writing source files.

Canonical media-core builders validate each range and strict segment chronology.
Review never sorts segments or clamps measured timestamps. Supported overlapping
speech remains in source order and is marked in the UI. A text/timing edit clears
stale word/character alignment only for that segment, with an explicit warning.
Speaker-only annotation preserves word timing and updates that segment's labels.
The existing WhisperX subtitle projection handles contiguous aligned groups and
unaligned corrected segments together, without dropping either.

New local runs retain a canonical Native JSON sibling alongside chosen outputs.
Translation retains the source transcript separately in product attributes, plus
actual model/language provenance. Source and translated corrections remain
separate; editing either never retranslates or silently changes the other.
Canonical corrected Native JSON always preserves both. The user explicitly chooses
which layer is projected into WhisperX JSON, text and subtitle exports.
Older translated outputs without retained source data say so and offer a new run.

Save corrected copy renders through existing product output options and canonical
formatters. Every destination must be new. Private staged files are synchronized
and published without replacing existing paths; failures remove only newly
created exports. Registration failure also rolls back the new exports. The history
stores copy descriptors and output paths, not another transcript database.
Source recordings, original results and Speaker Libraries remain unchanged by
annotation or preview. Deliberate identity confirmation/correction opens the
existing Speakers workflow in another tab.

The native audio/video element is the playback authority. Source bytes are served
with byte-range support through a session-scoped opaque media capability. Host,
Origin and Fetch Metadata checks still apply. The URL carries neither a filesystem
path nor the session authorization token. Explicit relocation authorizes a local
matching recording within the launch directory; it changes the playback association
without rewriting original provenance or media. Missing/empty results and missing
or browser-incompatible recordings have direct recovery actions.

## Consequences

Review performs no inference. Corrected copies can reopen after server restart;
unchanged history reconciliation does not rewrite the index. Single-file session
IDs expire with their server, while saved copies and Jobs references persist.
The browser's codec support determines playback availability, independently of
native decode capability. No waveform, generic subtitle editor, re-encoding
pipeline or new persistence service is introduced.

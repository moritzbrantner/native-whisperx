# Workflow Composition

This context owns native-whisperx workflow language: how transcription, alignment,
diarization, output writing, and parity are composed into user-facing workflows.

## Language

**Product Boundary**:
The permanent rule that native-whisperx owns user-visible Native WhisperX
workflow semantics while reusable execution capabilities live in their canonical
lower-level crates. If a capability would still make sense in a Rust
transcription application that does not use Native WhisperX semantics, it is not
owned here.
_Avoid_: proving-ground implementation, implement-here-first, convenience ownership

**Workflow Composition**:
The orchestration of ASR, VAD, alignment, diarization, translation, output
writing, parity, and product behavior into one user-facing transcription
workflow.
_Avoid_: primitive ownership, model implementation

**Automatic Workflow Selection**:
A Workflow Composition mode where native-whisperx resolves unspecified
lower-level workflow choices from the user's higher-level request, such as
choosing a quality-preserving VAD and diarization pair for native finite
diarization. Automatic choices are distinct from explicit user-selected
configuration and must not change transcript output contracts by themselves.
Automatic selection grows through narrow capability-specific policies rather
than a generic planner or capability graph.
_Avoid_: silent fallback, transcript metadata, generic model guessing, generic planner

**WhisperX Parity**:
Feature compatibility with the Python WhisperX user-facing surface. The Python
WhisperX CLI is the first normative compatibility surface. Parity work tracks
both the latest Upstream Target and the newest Verified Compatibility Baseline.
_Avoid_: loose similarity, best-effort compatibility, permanently pinned target

**Upstream Target**:
The latest released Python WhisperX version that native-whisperx is expected to
catch up to. A new release may place the project in a documented behind-upstream
state without immediately changing deterministic gating evidence.
_Avoid_: verified baseline, arbitrary latest dependency in gating CI

**Verified Compatibility Baseline**:
The newest Python WhisperX version for which the required native-whisperx gating
parity evidence currently passes. This remains the deterministic gate until the
Upstream Target is reconciled and promoted.
_Avoid_: latest upstream release, permanent compatibility target

**Rust-Native Parity**:
A stricter WhisperX parity track where user-visible WhisperX behavior is
implemented in Rust/native canonical owners without adding new Python WhisperX
or faster-whisper runtime bridges. Python WhisperX may still be used as a
reference oracle.
_Avoid_: delegated parity, compatibility bridge, native-whisperx-local primitive implementation

**Native**:
The Rust-first direction of the project. Native does not mean every current
feature is Rust-only.
_Avoid_: Rust-only

**Native Extension**:
A user-visible Native WhisperX capability that intentionally goes beyond the
Python WhisperX surface, such as Automatic Workflow Selection, Speaker Directory
workflows, Near-Live transcription, or explicit Q8 execution. Native extensions
must be additive and must not silently redefine overlapping WhisperX semantics.
_Avoid_: compatibility regression, implementation backend

**Delegated Feature**:
A transitional compatibility feature satisfied by calling Python WhisperX or a
Python dependency while a Rust-native implementation matures. Runtime
delegation is not a permanent second execution architecture and must not become
an automatic fallback.
_Avoid_: unsupported feature, external-only feature, permanent backend

**Parity Harness**:
The native-whisperx-owned fixtures, runner, structured diff, goldens, tolerances,
reports, and performance gates used to compare native-whisperx behavior against
Python WhisperX. Reusable-looking comparison helpers remain here until a second
independent consumer proves a lower-level abstraction is warranted.
_Avoid_: smoke test, fixture check, generic transcript library

**Full Workflow Throughput Gate**:
A benchmark gate that compares the complete user-visible workflow, including
VAD, ASR, alignment, and output, against WhisperX.
_Avoid_: ASR-only benchmark, partial speed check

**WhisperX JSON**:
The WhisperX-compatible JSON transcript contract exposed to users by default.
_Avoid_: native JSON

**Input Pattern Expansion**:
The CLI behavior that turns user-provided wildcard input arguments into concrete
media file paths before a transcription workflow starts.
_Avoid_: shell globbing, path guessing, batch discovery

**Multi-Input Transcription Run**:
One user command that processes more than one concrete media file under a shared
transcription configuration. Native WhisperX owns the run/file/output/cancellation
semantics; reusable provider/session reuse is an upstream transcription
capability.
_Avoid_: ASR batch, model batch, fixture suite

**Transcription Progress Stream**:
The user-visible progress narrative emitted by native `transcribe` while a
finite transcription workflow runs. It combines Native WhisperX run, file,
task, translation, output, completion, failure, and cancellation semantics with
authoritative low-level lifecycle facts emitted by upstream runtimes/providers.
It is distinct from transcript output files, final report JSON, parity reports,
and diagnostics.
_Avoid_: final report, transcript JSON, parity diagnostics, independent upstream observer API

**Input-Local Output**:
The default output placement rule where transcript files are written beside
their source input when no explicit output directory is selected.
_Avoid_: stdout-only output, implicit output directory

**Live Feed Transcription**:
A workflow that transcribes an audio or video source while input is still
arriving and emits incremental transcript observations instead of bounded
transcript files. The first live feed workflow is near-live and ASR-only rather
than true provider-level streaming.
_Avoid_: finite file transcription, provider streaming

**Near-Live Window**:
A finite rolling audio window cut from a live feed. Generic PCM buffering,
window/hop scheduling, bounded transcription sessions, and cancellation
boundaries belong to reusable transcription infrastructure; Native WhisperX
owns the window policy and stabilization semantics.
_Avoid_: streaming chunk, source packet, product-independent buffer implementation

**Live Transcript Event**:
A newline-delimited JSON event emitted by Live Feed Transcription to stdout,
such as session lifecycle, partial transcript, final transcript, or error
events. Partial events may be revised by later windows; final events are stable
once emitted.
_Avoid_: WhisperX JSON file, subtitle cue, native JSON aggregate

**Local Ingest Clock**:
The local UTC wall-clock timeline assigned as samples enter a live transcription
session. It is the first live workflow's timing basis and is not source PTS,
broadcast timecode, or a claim of source-synchronized media timing.
_Avoid_: source PTS, broadcast timecode

**Native JSON**:
The explicit JSON representation of the Rust transcript contract.
_Avoid_: default JSON, WhisperX JSON

**Speaker Directory**:
The user-selected directory that stores reusable speaker identity data and
derived speaker trace data for native-whisperx speaker workflows. Resolution is
local-first by default, may be forced to the platform global data directory, or
may be set to an explicit path. Directory selection and trace semantics are
Native WhisperX product policy.
_Avoid_: model cache, transcript output directory, speaker database

**Speaker Directory UI**:
The browser interface served by `speakers open` for inspecting and managing a
Speaker Directory.
_Avoid_: repository website, frontend build pipeline, separate speaker model

**Speaker Library**:
The canonical enrolled-speaker identity file at `library.json` inside a Speaker
Directory. The reusable profile format and load/save/validate/edit/delete
lifecycle are owned by `audio-analysis-speakers`; Native WhisperX chooses where
the library lives and how it participates in Speaker Directory workflows.
_Avoid_: trace index, transcript provenance, anonymous diarization output

**Speaker Store**:
Compatibility wording for the Speaker Directory used by earlier CLI proposals
and aliases such as `--speaker-store`. New architecture and docs should use
Speaker Directory unless describing backward-compatible aliases.
_Avoid_: separate storage root, second identity system

**Confirmed Speaker Profile**:
A Speaker Library profile whose identity has been explicitly accepted or
corrected by the user. Missing `metadata.status` is treated as confirmed for
older libraries.
_Avoid_: anonymous speaker label, trace-only speaker

**Draft Speaker Profile**:
A Speaker Library profile saved automatically from native diarization for a
transient unknown speaker label. Drafts may be used for recognition by default
but can be excluded with `--no-use-draft-speakers`.
_Avoid_: user-confirmed identity, final speaker name

**Speaker Trace**:
Derived native-whisperx provenance at `speaker-trace.json` inside a Speaker
Directory, rebuilt from transcript JSON outputs. It records where speakers
appear in files and is not stored in the Speaker Library.
_Avoid_: identity source of truth, speaker enrollment profile

**Anonymous Speaker Label**:
A speaker string emitted by diarization or found in transcript JSON that does
not exactly match an enrolled Speaker Library profile id or current label. It
preserves anonymous diarization output without creating a speaker identity.
_Avoid_: stable speaker id, enrolled speaker profile

**Identity-versus-trace separation**:
The rule that stable speaker identity lives in the Speaker Library while file,
span, snippet, and run provenance lives only in the Speaker Trace.
_Avoid_: embedding transcript history in profiles, mutating identity from trace

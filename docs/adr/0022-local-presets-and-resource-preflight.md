# ADR 0022: Keep local presets separate from execution and resource ownership

Status: Accepted

## Context

The local task workbench needs reusable settings and actionable resource checks.
Provider cache/bundle rules belong upstream; product selections and local saved
configuration belong to Native WhisperX. A stored preset must remain readable
when this machine cannot execute it.

## Decision

`WorkbenchSettings` owns the existing finite task choices, model/languages,
device, local model/output paths, formats and optional Speaker Directory
reference. `WorkbenchRequest` adds input using serde flattening, preserving its
existing flat JSON wire shape and Jobs history compatibility. Rust callers now
use `request.options` or `settings.for_input(path)`. Named `WorkbenchPreset`
validation checks configuration independently of compiled features/resources.
Translation presets use supported direct pairs from the existing TranslationPlan;
this workbench does not silently add pivot models.

The frontend has one settings value for controls, preset application, preflight
and execution. File selections, run state and editor metadata remain separate.
An explicit selected preset ID survives navigation/relaunch. Selection is saved
before resolving an optional referenced index, so an unavailable index yields an
actionable failure without losing the selected configuration. Actual index loads
use the existing Library workflow. Execution checks that a required reference
matches the active index; it cannot silently substitute another library.

The local `.native-whisperx-workbench/presets.json` document stores schema version
1, at most 50 named presets, and one selected ID, bounded to 512 KiB. It contains
no input media, transcript, progress, runtime objects, or authorization tokens.
Settings never enter URLs. Private staged files, atomic replacement and a held
writer lock protect updates. Selecting/updating an unchanged preset reports a
no-op without rewriting the document. Malformed/older documents keep the app
usable and require an explicit backup-and-reset action; unsafe symlinks cannot
be read or reset. Recovery preserves original bytes before replacing the store.

Preflight composes canonical cache-only Whisper/alignment inspection from Audio
Analysis, existing pyannote integrity/static metadata verification, canonical
embedding descriptors for library compatibility, existing translation planning
and cache resolution, and audio I/O's FFmpeg/FFprobe availability. Inspections do
not decode input, download models or construct inference sessions. Static ONNX
inspection may read graph files and checksums. Device probing follows the actual
provider policy; explicit unavailable CUDA cannot become CPU silently.
Available files are not proof that weights/configuration can execute. Normal
workflow validation remains authoritative during the run.

Both single-file Start and batch queue enforce the same checks before processing.
A batch checks common resources once, including media tools when any input is
non-WAV. Active index changes invalidate visible preflight; failed checks offer a
manual recheck after local resources are prepared. Built-ins expose compiled
support separately from local availability. The curated English-to-German preset
explicitly selects CPU. Model/provider fields remain under Advanced settings.

## Consequences

No model manager, background download, generic preset schema, database, runtime
fallback or new package is introduced. Provider inspection remains upstream;
Native WhisperX exposes only its own resource messages and saved configuration.
Presets can be valid while unavailable on the current machine, and preflight can
pass while later inference detects corrupt/incompatible files. Jobs, Review and
Speaker Library/Trace ownership stay unchanged. PRD #376 / issue #381 owns this
product behavior; Audio Analysis #164 / PR #165 supplies its reusable prerequisite.

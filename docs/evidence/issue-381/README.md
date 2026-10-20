# Issue 381: presets and resource preflight

Native baseline: `27c0c1ae5d7630583d394e941e9d667d20d4b69e` (main after PR #390).
Audio Analysis source: `c8d89467f86119a6064ba363738eba5dd9862cd7` (merged PR #165).
Foundation source: `9a162dfd44d3cae7f88ad993de79defc0ac94cd6`.
Live convention source revision: `6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.

## Deterministic checks

Public headless and CLI/HTTP tests cover settings round-trip, one ordered task
configuration, forbidden session/transcript/input fields, feature-independent
saved settings, missing model/alignment caches, explicit unsupported CUDA,
invalid translation pairs, missing FFmpeg, and no processing/download on failure.

CLI tests save/update/reopen/delete presets, verify no-op selection/update retain
bytes and modification time, recover malformed and older documents through a
byte-preserving backup, and reject symlink storage without reading/disclosing or
modifying its target. Existing Jobs failure/retry and output confinement checks
still pass; their local sentinel model-file fixture reaches the intended decode
failure after resource preflight without loading weights.

React tests select/edit/save/reopen/run one settings value, recover invalid saved
state, prevent Start on resource failure, refresh preflight after active index
changes, and retain an explicit selection when a referenced index cannot load.
Required checks: `cargo fmt --all -- --check`, composition ownership check,
workspace Clippy with warnings denied, default/no-default workspace tests and the
frontend's `bun run check`. Generated embedded asset names match the built index.

## Separate resource-backed checks

The ignored `workbench_preflight_external` test used the approved local pyannote
community bundle and the checked-in incompatible-library fixture. It rejected
model incompatibility while invalid sentinel Whisper files were accepted only as
available files. No model/session was constructed; library bytes/mtime stayed
unchanged. It ran without requiring ONNX Runtime initialization.

A real native run used a local 9.99-second conversation, cached tiny.en, cached
alignment and English-to-German Marian models, verified local pyannote VAD and
community diarization, explicit CPU and ONNX Runtime 1.30.0. A saved preset was
reopened, updated on touch, restored with its speaker-index reference and started
by keyboard. Exactly one run request produced Decode, VAD, Transcribe, Align,
Identify speakers, Translate and Write outputs, each with `starts = 1`.
JSON/SRT/Native JSON were emitted, the canonical result retained source and target,
and the active library remained unchanged with auto-update off. This is execution
and configuration evidence, not a new speaker-quality acceptance claim.

The first browser layout placed Start at y=1395. The final 1280×960 view places it
at y=845 and task selection at y=471; task selection remains visible at touch
390×844 (Translate y=728), with no horizontal overflow. Advanced settings starts
collapsed. Reload and server restart restore the preset; unchanged restoration
preserves store bytes/mtime. Touch update retains the preset ID.

A browser regression first reproduced stale Ready after changing the active index
(one automatic probe while the server already reported unavailable). The final
browser check verifies immediate refresh, actionable referenced-index mismatch and
incompatible-model errors, with no additional inference. Screenshots capture the
configuration after restart, excluding transcript content. `observations.json`
records stage counts and final geometry without tokens or transcript text.

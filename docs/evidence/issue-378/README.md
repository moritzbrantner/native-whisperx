# Issue 378: local speaker index controls

The browser exercised the served native release binary on loopback, desktop and
390px touch viewports. `observations.json` records the measured outcomes;
`desktop.png` shows a reviewed replacement and `mobile.png` the applied result.
No horizontal overflow or browser errors were observed.

The synthetic confirmed/draft fixture round-tripped canonical identities and
metadata. New/cancel caused no writes, repeated import preserved modification
time and entries, conflicting merge failed, cancelled replacement preserved the
active file, and explicit replacement created a recoverable backup. The profile
editor refreshed corrected labels. Index actions triggered no transcription.
These synthetic vectors establish transfer behavior, not recognition quality.

Two actual native CUDA runs used a ten-second recording and cached Whisper small
and Pyannote community-1 resources. Loading a genuine compatible index matched a
stored identity without modifying the index. Creating a new index and explicitly
enabling auto-update produced one compatible draft and refreshed the visible
count. Each selected native stage started once. Voice vectors, audio and session
tokens remain outside the repository. This does not resolve the separate speaker
count/quality acceptance work in #362.

Source dependencies: Audio Analysis
`e35f82ba87facbedc92a97352ab2dce00ccfec66`, foundation
`9a162dfd44d3cae7f88ad993de79defc0ac94cd6`.
Resolved convention sourceRevision:
`6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.

Regression coverage includes headless canonical transfer/trace isolation, strict
session preview/confirmation/cancellation, selection, conflicts/backups, native
per-run flags, React consent/error/no-op paths, and prevention of nested-form
transcription submission. Frontend check includes the production asset build.

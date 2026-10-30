# Named language choices

Both interfaces expose named source-language choices. The browser uses the 99
codes advertised by its actual multilingual Whisper adapter and forwards an
explicit language per run; an empty choice keeps automatic detection. The local
app offers the same source suggestions while retaining manual codes and presets.
Source language is beside Transcribe. Local translation offers the eight curated
planner languages by name. Provider/model validation remains authoritative.

The browser's pair menu comes from actual translation capabilities. An incompatible
source/pair combination blocks Run with an explanation. Choosing a pair explicitly
selects its source language. Controls are locked during execution and the run
retains its own language snapshot. Alignment's control is now positioned between
Transcribe and Diarize, but remains disabled until actual browser execution exists.

Checks:

- React test first failed because named translation options were absent, then
  passed while preserving the existing single-run settings assertion.
- Browser smoke first failed because no source-language control existed, then
  passed explicit Japanese → automatic switching, translation compatibility,
  source preservation, cancellation/retry, desktop/touch and download checks.
  Model calls are protocol-faithful fixtures; this is not physical WebGPU evidence.
- All 29 frontend tests, TypeScript checking, formatting and production build pass.
- Full Rust workspace tests, all-target Clippy with warnings denied and formatting
  pass. Product boundary and static-site checks pass.
- Real loopback UI at 1280×960 and 390×844: named choices, actual cache-only preflight,
  desktop Start above the fold, no horizontal overflow. No model run was started.

The shared product-label module is TypeScript; site preparation bundles it into
an ignored browser artifact and the local UI compiles the same source. CI/Pages
path scopes include this shared source so language-only changes are verified and
deployed. Static Audio Analysis browser source is pinned to
`db255c6409581703609051f02f39b993ca5e3310` (audio-analysis#169). The Rust source pins
remain unchanged because this adoption uses only the browser adapter.
Shared convention source: `6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.

Parent #392 remains open for additional browser translation pairs and actual
alignment / HF-authorized pyannote workflows. No unsupported stage is simulated.

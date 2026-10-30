# Visible transcript timestamps

Both Result views in the local app now display segment start/end timestamps,
anonymous speaker labels when present and the associated text. The same canonical
transcript importer and UI summary used by Review & Edit provide the data; raw
output bytes remain available under file details. Missing segment timings display
`Untimed`; no word/segment timing or inference is invented.

The published website shows timed rows as its primary transcript, using
HH:MM:SS.mmm. It no longer places a duplicate untimed paragraph before them.
Retained source text for translation is available under a collapsed details control.

## Reproduction and checks

The existing HTTP completed-output test first failed because `transcript.segments`
was absent (`null` versus expected start `0.125`), then passed after canonical
import/summary was used for both current and Jobs results. The React interaction
likewise first failed to find `00:00:00.125 → 00:00:02.750`, then passed alongside
its existing single-run assertion. All 11 workbench HTTP tests pass.

Full Rust workspace tests, all-target Clippy with warnings denied and formatting
passed with the declared exact source graph. Frontend formatting/typechecking,
all 29 behavior tests and production asset generation passed.

`scripts/test-site-workbench.mjs` first failed the primary-transcript assertion,
then passed: real browser audio decoding and speaker preview, model-boundary ASR
and translation fixtures, timestamp rendering, four preserved downloads,
cancellation/retry, desktop and 390px touch layouts. These fixtures establish UI
orchestration, not physical WebGPU/model quality. Website structural check passed.

Local screenshots use the real loopback HTTP service and a recorded, authored
three-segment JSON fixture: two timed anonymous turns and an untimed turn. Both
1280px desktop and 390px mobile show times/labels, explicit missing timing and no
page overflow. No model calls or tokens appear in this evidence.

Audio Analysis source: `ec42fcfc1fa3a0d0b9dccdfcf5506ababe287fc1`.
Foundation source: `9a162dfd44d3cae7f88ad993de79defc0ac94cd6`.
Shared convention source: `6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.

Parent #392 remains open for additional languages and actual browser alignment /
HF-authorized pyannote execution. This slice changes presentation only.

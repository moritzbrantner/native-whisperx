# Issue 382: browser task workbench

Baseline: `452107db6598ce4da914de09145a661a55ae1d82`.
Live convention source revision: `6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.

The initial browser check failed because Run appeared at y=2283.84 on a
1280×900 desktop viewport. The revised surface puts input, compatible task
checkboxes and execution beside the transcript. Model settings, support detail
and the native command composer start collapsed. Unavailable translation is
disabled independently from ASR; alignment is explicitly local-only. Running
locks the selected input and tasks until cancellation or completion.

## Verification

`bash scripts/prepare-site.sh`, `python3 scripts/check-site.py`, Node syntax
checks for the workbench, unchanged acceptance harness, all four vendor adapters
and `scripts/test-site-workbench.mjs`, and `git diff --check` passed. Browser
adapter source pins and the deployed acceptance implementation are unchanged.
No native execution or React application code changes in this issue.

The optional browser smoke runs against the locally served prepared site, with
Playwright and Chromium supplied through `PLAYWRIGHT_MODULE` and `CHROMIUM_PATH`
(see `site/README.md`). It uses generated two-second WAV samples, the real
browser decoder and deterministic spectral speaker adapter. Protocol fixtures
replace ASR and translation model calls. The attached screenshots and
`observations.json` are from that deterministic tier, not inference evidence.

Verified in Chromium at desktop 1280×900 and touch 390×844:

- Compose ASR, anonymous speaker labels and English-to-German translation in one
  run, decode exactly once, retain the source transcript, and preserve segment
  IDs/times in translated outputs.
- Download Native JSON, TXT, SRT and VTT with speaker labels and translated text.
- Cancel during ASR before speaker/translation processing, with no stale
  download; retry succeeds. Input/task controls remain locked during the run.
- Keep Run above the desktop fold with input and all compatible tasks selected;
  touch controls work with no horizontal page overflow.
- Disable execution without WebGPU, disable unsupported translation separately,
  show local-only alignment, and keep the secondary command composer usable.
- Capture results through the unchanged acceptance iframe and preserve the
  `/transcribe/` compatibility redirect. No page errors, audio uploads,
  cross-origin requests or native-server calls occurred.

Physical WebGPU and model-backed translation were not rerun in this tier. The
existing separately classified deployed acceptance harness remains the authority
for real-provider browser evidence; this redesign does not change its checks or
promote deterministic model fixtures to physical execution evidence.

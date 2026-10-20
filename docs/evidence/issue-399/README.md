# Browser alignment integration

Pages now offers the real Audio Analysis CTC alignment step between Transcribe
and anonymous speaker preview. It reuses decoded 16 kHz PCM and the existing
transcript, without rerunning ASR. English/German are explicitly supported;
selecting alignment with automatic or another language disables Run before model
acquisition and explains the required choice. ASR still supports all 99 languages
when alignment is off.

The local CLI workbench already exposes its real native alignment before speaker
identification; its finite workflow remains the existing native model path. This
change enables the previously disabled Pages control using the reusable upstream
implementation, rather than adding another alignment algorithm to Native WhisperX.

`prepare-site.sh` builds the actual Rust WASM module from pinned Audio Analysis,
foundation and coding-tooling sources in an isolated temporary graph. The Pages
and site workflows install pinned Rust/WASM tooling and check the generated
browser entry. No package publication or model weights are included.

Validation:

- Static contract and JavaScript syntax checks pass.
- Playwright workflow checks cover explicit language guards, Transcribe → Align →
  Diarize → Translate ordering, one audio decode, preserved segment identity/times,
  aligned words surviving speaker assignment, translated text clearing source-word
  timings, four downloads, cancellation during ASR/alignment, alignment failure,
  retry, desktop Run above the fold and a 390px mobile layout without overflow.
- A separate website resource probe used real English wav2vec2 ONNX weights,
  ONNX Runtime Web 1.30.0 and the generated Rust WASM bridge. Only the ASR boundary
  was replaced by a transcript fixture. The real browser audio decoder ran once;
  all eight words received timings, source segment ID 7 remained stable and the
  Native JSON identified the actual model. Models/runtime were served through a
  local HTTP cache using identical bytes. This is CPU WASM execution, not physical
  WebGPU ASR evidence or a conversational-accuracy benchmark.
- Upstream real-model English and German adapter evidence is recorded in Audio
  Analysis `docs/evidence/browser-forced-alignment.md`.

Screenshots: [desktop](desktop.png), [mobile](mobile.png),
[real browser alignment](real-alignment.png). Synthetic speech was generated from
owned text; no user audio, credentials or trained weights are committed.

Model limits remain explicit: segments at most 30 seconds, English/German,
first-download sizes about 361/117 MB. Cancellation checks stage boundaries; an
already executing ONNX operation completes before the cancellation is observed.
Shared convention sourceRevision: `6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.

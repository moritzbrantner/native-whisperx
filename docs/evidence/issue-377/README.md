# Local workbench resource and browser evidence

The real CLI-served UI completed one 10-second WAV through Transcribe → Align →
Identify speakers → Translate (English to German) → Write outputs on CPU. Each
native phase started once. The emitted WhisperX JSON was opened through the
session API and displayed in the browser. This proves execution and interaction;
it does not establish model accuracy or resolve real-conversation acceptance #362.

[Structured observations](observations.json) retain input/binary hashes, exact
model/source identities, phase counts/times, status latency, cancellation and the
separate observed CUDA translation failure. The model bundles and source pins
are the same approved resources used by the existing native workflows. Python
was used only to obtain model cache files, never as a product execution fallback.

The input was the first 10 seconds of #372's short-video control, converted with
`ffmpeg -i short-video.mp4 -t 10 -ac 1 -ar 16000 -c:a pcm_s16le conversation.wav`.
See [the source-media evidence](../issue-372-alignment.md). No audio or embedding
vectors are committed here. Runtime observations were collected on a shared host
with four Rayon threads and ONNX Runtime 1.27.0; they are not throughput gates.

The server was launched from the common parent of the input and explicit model
bundles using `workbench --no-browser --print-url --port 39925 --speaker-directory
<WORKBENCH>/speakers`, with the documented `ORT_DYLIB_PATH` environment prerequisite.
The build used `cargo build --release -p native-whisperx-cli --locked --features
cuda` in source mode. CPU was selected in the UI for the completed run.

Browser checks used installed Chromium through Playwright: 1280×900 desktop,
390×844 mobile with touch enabled, native Start, progress, Open result, mobile tap
on Open result, and keyboard Enter on Start. Mobile scroll width equaled 390 px.
Recorded desktop page errors were empty. The largest sampled status-request
latency during inference was under 14 ms on this run; inference ran outside the
HTTP request loop.

A second native run was cancelled during ASR. The UI reached `cancelled`; later
alignment/diarization/translation/output tasks did not run, and its separate output
folder remained empty. Missing tokens, wrong Host/Origin/Fetch Metadata, paths
outside the launched directory, empty selection and untyped executable fields
are covered by offline CLI/API tests. Unavailable stages and backend failure
presentation are covered by React behavior tests.

The CUDA comparison failed in Marian with `no cuda implementation for
softmax-last-dim`. It produced no output and remained a visible failure. This
change does not claim CUDA support for that translation operation. CPU/Automatic
translation is the verified path. Provider limitations remain native errors.

Local checks passed: Rust formatting; workspace Clippy with warnings denied;
workspace tests; no-default-feature workspace tests; focused workflow and
loopback/API tests; React formatting/types/tests/build; and the composition-only
boundary check and its tests. The registry lockfile contains only the two newly
explicit utility dependency edges; generated source-mode lock changes are absent.

- [Desktop screenshot](desktop.png)
- [Mobile touch screenshot](mobile.png)

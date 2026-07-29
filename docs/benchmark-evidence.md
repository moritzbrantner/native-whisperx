# Benchmark Evidence Contract

Rust-Native performance evidence is produced only by the opt-in
`final-full-surface` workflow on a prepared hardware runner. Offline tests
validate the report contract; they do not publish synthetic timings as
performance results.

Every evidence run uses at least one warm-up followed by at least three
measured iterations. `parity-bench` rejects smaller samples before loading the
manifest or media. The canonical CUDA gate remains the 30 second, 3 minute,
and 10 minute large-v3-turbo ladder. Native must beat WhisperX with a finite
speedup of at least `1.001` in every measured CUDA iteration.

The 30 second large-v3-turbo CPU case is comparative evidence. It records the
same native and WhisperX timings, phase timings, and batch diagnostics, but has
no stable pass/fail performance threshold yet. Missing timings or required
diagnostics still make the CPU evidence invalid.

## Raw and compact reports

Raw benchmark JSON includes:

- the git SHA and workspace crate versions;
- the exact ASR model revision, resolved from the Hugging Face cache or passed
  with `--model-revision`;
- CPU and CUDA device identity plus Rust, NVIDIA driver, and CUDA runtime
  versions;
- per-iteration elapsed time, realtime factor, phase timings, batch
  diagnostics, required-diagnostic failures, and full native/reference
  diagnostics.

Raw reports can contain machine-local diagnostics and therefore must not be
committed. The workflow uploads CUDA and CPU raw reports as retained artifacts
for 90 days.

`parity-bench-summary <raw-report.json>` validates the sampling and provenance
shape, then emits a whitelist-only compact summary. It keeps comparable
timings, phase timings, gate results, and batch diagnostics while excluding
full diagnostic arrays and paths. Only this compact form is suitable for a
future checked-in benchmark evidence update.

No new timing claim should be added to the repository until it comes from a
retained hardware-run artifact with complete provenance.

## Q8 CPU ASR evidence is separate

The manual Q8 CPU workflow described in
[`model-bundles.md`](./model-bundles.md#manual-q8-cpu-evidence) compares an
explicit local Q8 bundle with a matched FP32 bundle on the same CPU, with
alignment disabled. The bundles must use their exact Q8 and safetensors model
filenames and byte-identical sidecars. It alternates Q8/FP32 run order and
compares the median reported ASR time from three measurements per mode for
one-second and 15-second Shrek Retold-derived clips. The transcription command
retains enabled-by-default energy VAD/segmentation, while the measured
diagnostics characterize the no-alignment native CPU ASR routes. It does not
run alignment, the WhisperX reference, or the CUDA ladder.

Consequently, the Q8-versus-FP32 ratios are diagnostic evidence and never
satisfy or weaken the Full Workflow Throughput Gate. Raw reports contain
machine-local paths, full native reports, and transcripts and must remain
uncommitted. Only the runner's whitelist-only summary—with CPU identity, safe
bundle hashes, execution order, phase timings, tokens, fallback status,
realtime factors, transcript-equality booleans, medians, ratios, and the
comparative gate—is commit-eligible.

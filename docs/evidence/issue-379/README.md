# Issue 379: native multi-input Jobs

The served local release app was exercised with cached native Whisper small,
CUDA float32/batch size 1, a ten-second WAV and the full 639.341-second MP4 from
#372. `observations.json` records measured per-file task states/times and hashes.
This is a finite local resource check, separate from offline default CI.
No audio, voice embeddings, session tokens or raw transcript files are committed.

## Outcomes

- A three-file batch completed the first recording, failed on the deliberately
  invalid middle WAV, and never started the third. The completed output reopened.
- Explicit Run on the third row executed only that file in a fresh run. The
  original batch was unchanged and its completed output hash stayed identical.
- A second three-file batch completed the first recording and entered ASR for
  the long MP4. Cancel waited for the native safe boundary. The active file and
  third file became cancelled; the third had no task starts or outputs. The
  first completed output hash remained unchanged.
- Killing the server during a further long-file ASR run and reopening the same
  directory preserved the first completed row/output and marked only unfinished
  rows Interrupted. No inference restarted. A later restart preserved the history
  bytes and nanosecond modification time exactly.
- The final desktop screenshot shows the failed batch with readable actions and
  native error details behind a disclosure. The final 390px touch check opened a
  completed result from recovered history. Both viewport and document stayed
  390px wide, with no browser errors. Tables scroll within their container.

The browser initially tried to fill a collapsed settings field; opening that
section corrected the script before inference began. Visual checks then found
squeezed table columns and a long result path widening the mobile viewport;
fixed column sizing and path wrapping repaired both. The final assertions check
an actual 390px viewport as well as matching document width.

## Reproduction and checks

Prepare the exact source siblings and activate source mode, then build:

```bash
bash scripts/source-deps activate
cargo build -p native-whisperx-cli --release --features cuda --locked
```

Launch `workbench --no-browser --print-url` from an isolated recording folder.
In batch mode select a short valid file, an invalid WAV and another valid file;
then explicitly retry the unstarted row. For cancellation/recovery use the short
file, a sufficiently long recording and the third file. Cancel after the first
completes, or terminate the local server during the second file and reopen the
same launch directory. Preserve the completed output hash and history index
before checking unchanged reconciliation.

Validation passed: workspace/default and no-default tests, Clippy all targets
with warnings denied, formatting, composition ownership boundary, and frontend
format/typecheck/22 React tests/production build. Public CLI/API tests cover
session-scoped no-write empty reads, failure ordering, explicit targeted retry,
stable history reopening, completed result reopening, completed retry rejection,
and output/history symlink confinement. Existing finite cancellation integration
checks remain in the workspace ladder. The checked-in assets match their build.

Source graph: Audio Analysis `e35f82ba87facbedc92a97352ab2dce00ccfec66`,
foundation `9a162dfd44d3cae7f88ad993de79defc0ac94cd6`.
Convention sourceRevision: `6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.
The final local binary hash is recorded in the observations. These runs establish
queue/control/result behavior, not diarization quality acceptance in #362.

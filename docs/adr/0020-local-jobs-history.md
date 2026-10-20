# ADR 0020: Local multi-input Jobs and durable results

Status: Accepted

## Context

The local workbench needs a bounded multi-file queue, per-file native progress,
cancellation, explicit retry and reopening completed outputs across UI restarts.
The existing Multi-Input Transcription Run already owns sequential execution,
provider/session reuse, failure ordering and cooperative cancellation.

## Decision

Batch mode authorizes paths within the launch directory and expands supported
media in a selected folder in filename order, without recursion. Explicit input
order is retained, canonical duplicates are removed and the batch is limited to
100 files. One settings selection becomes existing product configurations and
one call to `run_many_with_control`. The local session owns one worker and one
cancellation handle across single-file and batch modes. There is no new executor.

Jobs projects actual `TranscriptionProgressEvent` file/task boundaries into rows.
A failed file stops the existing run; remaining rows are Not run. Cancellation
preserves completed rows/outputs and marks unfinished rows cancelled. Stages show
busy indicators, not invented percentages. Run time is execution time; it does
not claim to be probed media duration.

Each batch uses a fresh random output partition
`.native-whisperx-output/<run-id>/<file-index>` under the selected output folder,
or beside each input when none was selected. Directories are authorized before
creating descendants, and run partitions are reserved without overwriting. The
existing output writer owns contents and formats. A retry creates a fresh
single-file run with recorded settings; it cannot mutate the original batch or
retry a completed file. The originally selected Speaker Directory must be loaded
before retry, and its current canonical contents remain authoritative.

A small product index at `.native-whisperx-workbench/jobs.json` stores bounded
history, settings, actual task states and output associations. It contains no
voice embeddings or session tokens. Writes use private staging files, sync and
atomic rename under an exclusive writer lock. A second process cannot write the
same history concurrently. The index is capped at 500 batches and 20 MiB; archive it explicitly when full. It is not a job database or scheduler.

Restart reconciliation marks unfinished batches Interrupted once while preserving
completed rows. It never restarts inference or associates old outputs with a new
run. Unchanged terminal history is read without rewriting it. Reopening requires
a recorded completed row, an authorized existing output and the viewer size limit.

## Consequences

Outputs and the small history index remain local after the browser/server closes.
Raw logs are not the primary Jobs surface. Missing/moved outputs and incompatible
or corrupt indexes report actionable failures instead of implying success.
Interrupted or failed files require explicit retry. Completed work stays intact.

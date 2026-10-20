# Speaker Directory UI

The Speaker Directory UI is the browser interface served by `speakers open` for
inspecting and managing a Speaker Directory. It is part of the CLI experience,
not the repository website.

Domain language stays in `CONTEXT.md`: Speaker Directory, Speaker Library,
Speaker Trace, Confirmed Speaker Profile, Draft Speaker Profile, Anonymous
Speaker Label, and identity-versus-trace separation. Implementation details
such as Bun, Vite, React, and checked-in production assets belong in this
document and ADR 0012, not in the glossary.

## Frontend Verification

Run from `crates/native-whisperx-cli/speaker-directory-ui`:

```bash
bun run check
git diff --exit-code -- dist
```

`bun run check` requires Bun and the frontend dependency set. It checks source
formatting, TypeScript, React tests, production build output, and normalizes the
checked-in `dist/` asset formatting. The `git diff` command catches stale
checked-in production assets after the build.

## Rust-Only Verification

These commands intentionally require only the Rust toolchain and the checked-in
repository contents:

```bash
cargo test -p native-whisperx-cli speakers_open_no_browser -- --nocapture
cargo package -p native-whisperx-cli --allow-dirty --no-verify
```

Cargo builds and package dry-runs must not run Bun, Node, npm, or Vite. The CLI
crate serves the already checked-in production assets from
`crates/native-whisperx-cli/speaker-directory-ui/dist/`.

## PRD Trace

Parent PRD #132 is traced through these child slices:

| PRD area | Child slice |
| --- | --- |
| React app scaffold, mock development mode, React Query, and frontend checks | #134 |
| CLI serving of checked-in React assets, stable API routes, content types, session token injection, and Rust-only Cargo packaging | #135 |
| Speaker Trace rendering, rebuild flows, Anonymous Speaker Label separation, and trace tests | #136 |
| Speaker Library summary, profile editing/deletion, metadata validation, session-token mutations, and profile tests | #137 |
| Documentation, ADR, CI frontend check, stale asset verification, and release/package verification notes | #138 |

## Local transcription workbench

Run `native-whisperx workbench` from the directory containing your recordings to
open Transcribe. `--no-browser --print-url` prints a loopback URL for manual
opening. The existing `--scope`, `--speaker-directory` and `--speaker-store`
options select the active Speaker Directory; `speakers open` still opens Speakers.

Enter an audio/video path or use Browse files. Choose optional alignment, speaker
identification and translation, output formats and an existing output folder.
Without an output folder, files are written beside the input. Tasks execute once
in the fixed native order. Start uses cached models; expand settings to select
model/language/device or verified bundles. All explicit paths must be inside the
launched directory, including after symlink resolution. Launch from a common
parent directory when recordings and explicit model bundles are in different
folders. Standard model cache resolution remains available without typing a
cache path.

The UI displays actual task states and failures. Cancel requests a stop at the
next native safe boundary. Open result reads an emitted file from the completed
job and displays its transcript and exact file contents. The first slice runs
one file at a time; queue/history, corrected-copy editing and saved presets are
separate issues.

Native transcription, alignment, diarization and translation depend on the build
and local resources. A compiled-in capability is not a model-readiness guarantee.
For example, the resource check for #377 observed Marian CUDA softmax failure;
CPU/Automatic translation remains a separate supported choice. Failed stages
are never presented as successful output. Speaker labels in results retain the
model's anonymous attribution and do not establish verified real-world identity.

See [ADR 0018](adr/0018-local-workbench-session.md) for the local session and path
boundary. Frontend changes require `bun run check` and refreshed checked-in
production assets as described above.

## Speaker index controls

Both views show the selected index scope/path, canonical file, profile count and
validity. Load selects an existing Speaker Directory (or its `library.json`). New
previews an empty index and creates/selects it only after confirmation. Auto,
Local, Global and explicit locations use the existing directory resolver.
Explicit paths and portable file destinations must be inside the launch folder;
the path picker can browse that folder.

Save exports canonical identity data to a new file after embedding consent.
Import previews Merge or Replace, then requires consent before applying. Merge
rejects conflicting IDs/labels and incompatible embedding models. Replace is a
whole-library choice, also checked for model compatibility. Changed imports
preserve the previous file beside `library.json` with a unique `.backup` suffix;
the picker can select those backups. Identical imports do not rewrite files.
Escape/Cancel dismiss a preview without applying it. Files contain sensitive
voice embeddings; store and share them deliberately. Speaker Trace is excluded.

Manage speakers opens the existing label/metadata/status editor. Loading or
importing an index refreshes that editor. Identify speakers uses the selected
index; the optional per-run draft auto-update checkbox saves compatible unknown
speakers as drafts. It starts unchecked and never confirms an identity for you.
See [ADR 0019](adr/0019-local-speaker-library-transfer.md) and
[the verification evidence](evidence/issue-378/README.md).

## Multiple inputs and Jobs

Enable **Process several files or a folder** in Transcribe. Enter the first path
and optional additional paths, one per line, or use Browse files → Use this
folder. A folder adds supported media in filename order without recursing into
subfolders. Explicit input order is preserved and duplicate canonical paths are
removed. Up to 100 files share one task selection and run sequentially through
the existing native multi-input API.

**View jobs** opens the Jobs destination, which shows each file's actual stage,
status, run time, task outcomes, error and completed outputs. Cancel waits for a
native safe boundary. Completed outputs survive cancellation; a failed file
stops the batch and leaves later files Not run. Retry/Run operates on only the
selected unfinished file with recorded task settings and a fresh output folder.
Load its recorded Speaker Directory first if you changed the active index.

Batch outputs live in fresh `.native-whisperx-output/<run-id>/<file-index>`
folders inside the chosen output folder, or beside each input by default. This
prevents basename collisions and protects previous outputs and source files.
Single-file mode retains its existing output behavior.

Local history is `.native-whisperx-workbench/jobs.json` in the launch directory.
Reopen the same launch directory to see results after restarting the UI/server.
An interrupted session is shown as Interrupted and never resumes automatically.
History retains the model/task settings and output paths of each original run;
changing current settings does not turn an old output into a new result. Missing
outputs report an error. Archive the index explicitly when its 500-batch limit
is reached. Keep its outputs when archiving if you still need them.

See [ADR 0020](adr/0020-local-jobs-history.md) for persistence and path ownership.

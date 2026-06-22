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

# ADR 0012: Check in Speaker Directory React production assets

## Status

Accepted.

## Context

The Speaker Directory UI is now a Bun/Vite/React application served by the
`speakers open` CLI command. The CLI crate is also published through Cargo, and
Rust contributors must be able to build and package it with only the Rust
toolchain.

The frontend workspace still needs normal frontend verification so UI source,
tests, and production assets do not drift apart.

## Decision

Check the Speaker Directory UI production build output into
`crates/native-whisperx-cli/speaker-directory-ui/dist/` and include that
directory in the `native-whisperx-cli` crate package metadata.

The CLI server serves those checked-in assets. Cargo builds and Cargo package
dry-runs do not run Bun, Node, npm, or Vite.

Frontend verification remains explicit: run `bun run check` from the Speaker
Directory UI workspace, which checks source formatting, types, tests, production
build output, and normalizes the checked-in `dist/` asset formatting. Then
verify the checked-in `dist/` assets are unchanged.

## Rejected Alternative

Do not run the frontend production build from Cargo build scripts, Cargo tests,
or Cargo package preparation. That would make ordinary Rust builds depend on a
frontend toolchain, network-installed packages, and Vite output stability, which
would weaken the CLI crate's package boundary.

## Consequences

Rust-only contributors and release dry-runs can build and package
`native-whisperx-cli` without installing Bun, Node, npm, or Vite.

Frontend contributors must refresh and commit `dist/` whenever React source
changes affect the production build. CI catches stale assets by running the
frontend check and failing if `dist/` changes.

The checked-in assets are release artifacts for the CLI crate, not a second
source of truth for Speaker Directory behavior.

# Publish Plan

This repository consumes published crates from `rust-packages`. Clean checkouts
resolve dependencies from crates.io; local co-development overrides must stay
outside committed manifests.

## Current Published Requirements

- `moritzbrantner-runtime-core` `0.2.0`
- `moritzbrantner-audio-analysis-speakers` `0.1.3`
- `moritzbrantner-audio-analysis-transcription` `0.1.6`

`moritzbrantner-runtime-core` `0.1.3` was yanked after verification because it
was semver-incompatible with older `0.1.x` dependents. The clean checkout
lockfile may still include `moritzbrantner-runtime-core` `0.1.2` for older
published crates that have not moved to the `0.2.x` API.

## Required Closure

Publish in dependency order:

1. `moritzbrantner-runtime-core`
2. `moritzbrantner-jobs-core`
3. `moritzbrantner-math-geometry-2d`
4. `moritzbrantner-math-signal-core`
5. `moritzbrantner-tensor-data`
6. `moritzbrantner-numbers-core`
7. `moritzbrantner-video-analysis-core`
8. `moritzbrantner-video-analysis-ingest`
9. `moritzbrantner-video-analysis-ffmpeg`
10. `moritzbrantner-audio-analysis-core`
11. `moritzbrantner-audio-analysis-fourier`
12. `moritzbrantner-audio-analysis-recognition`
13. `moritzbrantner-model-runtime`
14. `moritzbrantner-runtime-onnx`
15. `moritzbrantner-text-core`
16. `moritzbrantner-text-model-runtime`
17. `moritzbrantner-text-transcripts`
18. `moritzbrantner-audio-analysis-io`
19. `moritzbrantner-audio-analysis-speakers`
20. `moritzbrantner-audio-analysis-transcription`

## Gates

Run in `rust-packages` before publishing:

```bash
cargo test --test contract_ownership --test dependency_layers --test foundation_surface_audit --test package_structure --test package_interop_pipeline
bun run snapshot:check
bun run hygiene:generated
cargo fmt --check
git diff --check
scripts/check-preflight.sh
cargo doc --workspace --no-deps
```

For each crate:

```bash
cargo package --allow-dirty -p <crate>
cargo publish -p <crate>
```

Publishing remains manual.

## Native Supply-Chain Gate

Before publishing either crate from this repository, run the native dependency
policy gate:

```bash
cargo deny --workspace --all-features --locked check
```

The same command runs in the GitHub Actions workflow `supply chain`. It checks
RustSec advisories, yanked crates, license policy, duplicate dependency
versions, and unexpected crate sources from Cargo metadata only. It does not
build native-whisperx, download model bundles, require CUDA, run Python
WhisperX, or read parity fixture resources.

The workspace license policy is `MIT OR Apache-2.0`. Compatible permissive
licenses are listed in `deny.toml`; narrower exceptions must include a reason
beside the affected crate. Maintainers should update `deny.toml` in the same PR
as any dependency change that introduces a new accepted license, advisory
ignore, duplicate-version skip, registry, or git source. Advisory ignores must
reference the advisory ID and explain why the repository accepts the risk.
Duplicate-version skips must name the pinned crate version and the upstream path
that prevents deduplication.

## Native Package Dry-Run Gate

Before publishing either crate from this repository, run the release-facing
package dry-run for that crate:

```bash
cargo package -p native-whisperx --allow-dirty
cargo package -p native-whisperx-cli --allow-dirty
```

The same commands are available as the manual GitHub Actions workflow
`package dry-run`, with separate jobs for the library and CLI crates.

Release order matters. Run and fix the `native-whisperx` dry-run first, publish
`native-whisperx`, then run the `native-whisperx-cli` dry-run. If
`native-whisperx-cli` fails with:

```text
no matching package named `native-whisperx` found
location searched: crates.io index
required by package `native-whisperx-cli ...`
```

the CLI crate is correctly waiting on the library crate to exist on crates.io.
Publish `native-whisperx` first, then rerun the CLI package dry-run. If
`native-whisperx` has already been published under a different version, update
the CLI dependency version to match the published library version before
rerunning the gate. Do not remove the `version` field from the CLI path
dependency; Cargo requires that package metadata when preparing the CLI crate
for publishing.

Before publishing `native-whisperx-cli`, run the install smoke from this
repository:

```bash
cargo test -p native-whisperx-cli --test release_install_smoke -- --ignored --exact cargo_install_package_exposes_native_whisperx_command
```

The smoke installs the `native-whisperx-cli` package into an isolated Cargo
install root and executes the installed `native-whisperx` command. It verifies
`native-whisperx --version`, `native-whisperx --help`, and
`native-whisperx speakers path --scope local`. This is a no-resource offline
check: it does not transcribe media, download models, use CUDA, call Python
WhisperX, read Hugging Face credentials, or require a local smoke media root.

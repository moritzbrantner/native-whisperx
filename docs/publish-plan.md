# Publish Plan

This repository consumes published crates from `rust-packages`. Clean checkouts
resolve dependencies from crates.io; local co-development overrides must stay
outside committed manifests.

## Current Registry Requirements

- `moenarch-runtime-core` `0.2.0`
- `moenarch-audio-analysis-speakers` `0.1.3`
- `moenarch-audio-analysis-transcription` `0.1.12`

The transcription requirement is registry-owned and has no committed path
override. Version 0.1.12 contains the reusable Silero and pyannote VAD
providers plus the observer, model-resolution/download, and cooperative
cancellation APIs required by native-whisperx 0.1.14.

The prerequisite `moenarch-*` crates are published from the `rust-packages`
repository in dependency order. Their package verification and publication are
owned by that repository. Publishing `native-whisperx` and
`native-whisperx-cli` remains a manual maintainer operation.

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

## Native Package Stages

The release-facing gates are split around manual library publication. Before
publishing the library, run the metadata gate, perform full library package
verification, and inspect the CLI package contents without asking Cargo to
resolve its unpublished exact library dependency:

```bash
python3 scripts/check-release-metadata.py
cargo package -p native-whisperx --allow-dirty --locked
cargo package -p native-whisperx-cli --locked --list
```

For 0.1.14, both package versions and the CLI's exact `native-whisperx`
dependency requirement must be `0.1.14`. The default feature sets of both
packages include `translation`; explicit no-default-feature builds must not.
The library docs.rs feature list also includes `translation`, so the curated
planning, immutable translated-result, finite progress/cancellation, and live
progress/cancellation APIs are rendered for the release.

The manual `package dry-run` workflow exposes this as the
`library-prepublish` stage. It never publishes a crate.

After the maintainer publishes `native-whisperx` 0.1.14 and it is visible on
crates.io, run the `cli-post-library-publish` stage or the equivalent commands:

```bash
cargo info native-whisperx@0.1.14 --registry crates-io
cargo package -p native-whisperx-cli --allow-dirty --locked
cargo test -p native-whisperx-cli --test release_install_smoke -- --ignored --exact cargo_install_package_exposes_native_whisperx_command
```

Full CLI packaging and the install smoke are release postconditions because
Cargo cannot verify the exact `native-whisperx = "=0.1.14"` dependency until
the library is published. Do not remove the exact version requirement or use
`--no-verify` to bypass this ordering constraint.

The smoke installs the `native-whisperx-cli` package into an isolated Cargo
install root and executes the installed `native-whisperx` command. It verifies
`native-whisperx --version`, `native-whisperx --help`, and
`native-whisperx speakers path --scope local`. This is a no-resource offline
check: it does not transcribe media, download models, use CUDA, call Python
WhisperX, read Hugging Face credentials, or require a local smoke media root.

After the manual CLI publish, users install the package with:

```bash
cargo install native-whisperx-cli
```

That package installs the `native-whisperx` terminal command. A successful
install or release smoke does not prove transcription readiness: Workflow
Composition still resolves the requested Whisper, alignment, diarization, VAD,
translation, CUDA, Python WhisperX compatibility, and gated Hugging Face
resources at runtime. Default `json` output remains WhisperX JSON; explicit
`native-json` output remains Native JSON. Do not describe Delegated Feature
paths as complete Rust-Native Parity until the delegated runtime has been
replaced by native repository code and the relevant parity gates pass.

## First Native Release Checklist

Use this checklist after the `rust-packages` dependency closure above has been
published and committed manifests resolve only crates.io dependencies. Keep
publication manual; no GitHub Actions workflow in this repository publishes to
crates.io.

### Automated and Dry-Run Verification

Run or verify the normal pull request gates first:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test --workspace --no-default-features
cargo check -p native-whisperx --no-default-features --features translation
cargo check -p native-whisperx-cli --no-default-features --features translation
cargo check --workspace --no-default-features --features whisperx-compat,media-decode,diarization
cargo check --workspace --no-default-features --features silero-vad
cargo check --workspace --no-default-features --features onnx-diarization
cargo check --workspace --no-default-features --features whisperx-compat,translation,media-decode,silero-vad,diarization,onnx-diarization
```

Run the supply-chain policy gate:

```bash
cargo deny --workspace --all-features --locked check
```

The same dependency policy is checked by the `supply chain` workflow. If this
gate fails because of a new accepted advisory, license, duplicate dependency
version, registry, or git source, document the narrow exception and reason in
`deny.toml` before publishing.

Run the library-prepublication package gates:

```bash
python3 scripts/check-release-metadata.py
cargo package -p native-whisperx --allow-dirty --locked
cargo package -p native-whisperx-cli --locked --list
```

These package dry-runs intentionally use only the Rust toolchain. They must not
install or invoke Bun, Node, npm, or Vite. The `native-whisperx-cli` crate
packages the checked-in Speaker Directory UI production assets from
`crates/native-whisperx-cli/speaker-directory-ui/dist/`.

After manual library publication, run the CLI postconditions locally or select
`cli-post-library-publish` in the manual `package dry-run` workflow:

```bash
cargo info native-whisperx@0.1.14 --registry crates-io
cargo package -p native-whisperx-cli --allow-dirty --locked
cargo test -p native-whisperx-cli --test release_install_smoke -- --ignored --exact cargo_install_package_exposes_native_whisperx_command
```

If the CLI package dry-run or install smoke cannot resolve `native-whisperx`
from crates.io, that is the expected dependency-order failure before the library
is published. Do not publish `native-whisperx-cli` until the matching
`native-whisperx` version is visible on crates.io and the CLI dependency version
matches it.

### Manual Publish Commands

Only after the automated and dry-run verification above passes, publish the
library crate manually:

```bash
cargo publish -p native-whisperx
```

Wait for the matching `native-whisperx` version to be available from crates.io,
then rerun the CLI package dry-run and install smoke. Publish the CLI crate
manually only after those CLI gates pass:

```bash
cargo publish -p native-whisperx-cli
```

After the CLI crate is published, verify the user-facing install path:

```bash
cargo install native-whisperx-cli
native-whisperx --version
native-whisperx --help
native-whisperx speakers path --scope local
```

The published package name is `native-whisperx-cli`; the installed terminal
command is `native-whisperx`.

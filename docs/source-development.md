# Source development mode

Native WhisperX keeps registry-only dependencies as the release contract, but ordinary cross-repository development does not require publishing capability or foundation crates first.

The committed `.coding-tooling.source-deps.json` declares the reviewed direct dependency boundary and pins exact `audio-analysis` and `moenarch-foundation` revisions. Native WhisperX uses local-only source resolution: `bash scripts/source-deps activate` requires sibling checkouts whose Git `HEAD` exactly matches each declared revision. Missing local source is an error; source mode does not fall back to cloning private repositories or authenticated Git fetches.

The outer coding loop or agent workspace owns those sibling repositories/worktrees. It should prepare `../audio-analysis` and `../moenarch-foundation` at their pinned revisions before activation and may advance a pin when a task deliberately validates a newer source head.

Pull-request Rust CI prepares the same sibling layout from the two public repositories and their declared exact revisions before calling the normal activation script. This hosted preparation is explicit CI workspace setup, not a fallback in `scripts/source-deps`; local source mode remains offline and fails when its caller has not prepared the required checkouts.

Activation also saves the current registry-mode `Cargo.lock` and reconciles every declared direct package into a source-mode lock graph. Both generated files live under `.cargo/`, are ignored, and must never be committed. Repeated activation keeps the original registry lock backup. If activation fails, the wrapper removes the generated patch configuration and restores the registry lock.

Use `bash scripts/source-deps status` to inspect the mode. Run `bash scripts/source-deps deactivate` when source work is complete; it removes the generated patch configuration and restores the original `Cargo.lock` byte-for-byte. Deactivate before registry-only release verification.

## Development contract

- Feature work may change Native WhisperX and its immediate capability or foundation source without starting a crates.io release.
- Keep audio package versions compatible during source work. Version bumps belong to a later release task.
- Update the exact revision in `.coding-tooling.source-deps.json` whenever a validated source head changes.
- Patch only the packages that directly cross the repository boundary: audio I/O, speakers, and transcription. Their same-repository path dependencies carry core, Fourier, recognition, and other internal audio implementation crates from the same exact `audio-analysis` checkout; those transitive crates do not need duplicate consumer patch declarations.
- Patch `moenarch-media-core` directly from the exact foundation revision when consuming unreleased neutral timed-text or shared-runtime capabilities.
- Do not expand an application task into unrelated package maintenance. If more than two upstream repositories become necessary, treat that as an architecture boundary problem unless broader migration scope was explicitly assigned.
- Do not create new crates merely to avoid modifying an existing audio package boundary.

## Verification boundary

Source-mode verification proves that Native WhisperX works against the exact local source graph under development. It is valid implementation evidence even when those crate versions are not yet present on crates.io.

After activation, run Cargo with `--locked` so verification proves the reconciled source graph rather than silently changing it again. The wrapper's lifecycle can be checked without private source access by running `python3 scripts/test_source_deps.py`.

Cross-repository source verification belongs to the local coding loop because the upstream source repository is private while Native WhisperX is public. GitHub-hosted CI must not require a PAT or repository secret merely to reproduce ordinary implementation work. The local loop records the exact source revisions and verification commands used for the candidate.

Ordinary GitHub CI remains repository-local and may continue to exercise the last published dependency graph. A red registry-only dependency check caused solely by an intentionally unreleased upstream source change is release/distribution evidence, not a reason to publish during feature work.

Registry-only resolution remains mandatory before a Native WhisperX release. That later release task deactivates source mode, publishes or selects the required package versions through the release system, and verifies a clean checkout without patches.

## Registry mode status

Registry mode (the committed `Cargo.lock`, no patches) builds from crates.io.
The coordinated release for [#438](https://github.com/moritzbrantner/native-whisperx/issues/438)
published the audio-analysis 0.2.0 crates (core, io, recognition, speakers,
transcription) from audio-analysis `e1c27a6` and the foundation crates they pin
(runtime-core 0.2.2, runtime-onnx 0.1.2 on `ort` 2.0.0-rc.15, model-runtime
0.2.0, ...) from foundation `60b4392`. Those are also the revisions in
`.coding-tooling.source-deps.json`.

When Native WhisperX starts using unreleased upstream APIs again, registry mode
breaks until the next coordinated release. Source mode keeps working.

## Temporary Candle ZIP 9 backports

The application root pins a minimal public Candle 0.10.2 ZIP 9 backport in
`Cargo.toml`. The Candle 0.11 family uses the published 0.11.1 release, which
ships the same ZIP 9 filename handling upstream. The patch applies in both ordinary and Audio Analysis source
mode because Cargo ignores dependency workspaces' root patches. They preserve
existing tensor/model APIs and remove the ZIP advisory exceptions without a
registry release. Provenance, verification and removal conditions are recorded
in [issue 366 evidence](evidence/issue-366/README.md).

The `asr-candle-*` aliases in `crates/native-whisperx/Cargo.toml` deliberately
stay on Candle 0.10 to match Audio Analysis; bumping them alone resolves
`candle-core` 0.11 twice under different names and breaks `cargo metadata`.
Renovate ignores them (`renovate.json`); move them only together with Audio
Analysis's Candle version.

## Metal feature wiring

The `metal` feature enables Candle 0.10's `metal` features directly
(`asr-candle-*` in `crates/native-whisperx/Cargo.toml`) instead of naming
`audio-analysis-transcription/metal`, because no published transcription
release has that feature yet and the registry-mode supply-chain check resolves
the published graph. Audio Analysis detects Metal through Candle's own feature,
so the result is the same. Switch to `audio-analysis-transcription/metal` once a
release carries it. The macOS CI job (`metal-validation`) activates source mode
the same way the Linux job does and compile-checks `--features metal`.

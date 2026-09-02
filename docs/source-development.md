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

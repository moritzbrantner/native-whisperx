# Source development mode

Native WhisperX keeps registry-only dependencies as the release contract, but ordinary cross-repository development does not require publishing the audio capability crates first.

The committed `.coding-tooling.source-deps.json` declares the reviewed direct audio dependency boundary and pins it to an exact `audio-analysis` revision. Native WhisperX uses local-only source resolution: `bash scripts/source-deps activate` requires a sibling `audio-analysis` checkout whose Git `HEAD` exactly matches that declared revision. Missing local source is an error; source mode does not fall back to cloning private repositories or authenticated Git fetches.

The outer coding loop or agent workspace owns those sibling repositories/worktrees. It should prepare `../audio-analysis` at the pinned revision before activation and may advance the pin when a task deliberately validates a newer audio source head.

The generated `.cargo/config.toml` is ignored and must never be committed. Use `bash scripts/source-deps status` to inspect the mode and `bash scripts/source-deps deactivate` before registry-only release verification.

## Development contract

- Feature work may change Native WhisperX and the immediate audio capability source without starting a crates.io release.
- Keep audio package versions compatible during source work. Version bumps belong to a later release task.
- Update the exact revision in `.coding-tooling.source-deps.json` whenever the validated audio source head changes.
- Patch only the packages that directly cross the repository boundary: audio I/O, speakers, and transcription. Their same-repository path dependencies carry core, Fourier, recognition, and other internal audio implementation crates from the same exact `audio-analysis` checkout; those transitive crates do not need duplicate consumer patch declarations.
- Do not expand an application task into unrelated package maintenance. If more than two upstream repositories become necessary, treat that as an architecture boundary problem unless broader migration scope was explicitly assigned.
- Do not create new crates merely to avoid modifying an existing audio package boundary.

## Verification boundary

Source-mode verification proves that Native WhisperX works against the exact local source graph under development. It is valid implementation evidence even when those crate versions are not yet present on crates.io.

Cross-repository source verification belongs to the local coding loop because the upstream source repository is private while Native WhisperX is public. GitHub-hosted CI must not require a PAT or repository secret merely to reproduce ordinary implementation work. The local loop records the exact source revisions and verification commands used for the candidate.

Ordinary GitHub CI remains repository-local and may continue to exercise the last published dependency graph. A red registry-only dependency check caused solely by an intentionally unreleased upstream source change is release/distribution evidence, not a reason to publish during feature work.

Registry-only resolution remains mandatory before a Native WhisperX release. That later release task deactivates source mode, publishes or selects the required package versions through the release system, and verifies a clean checkout without patches.

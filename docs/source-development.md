# Source development mode

Native WhisperX keeps registry-only dependencies as the release contract, but ordinary cross-repository development does not require publishing the audio capability crates first.

The committed `.coding-tooling.source-deps.json` declares the reviewed direct audio dependency boundary and pins it to an exact `audio-analysis` revision. `bash scripts/source-deps activate` asks `coding-tooling` to materialize the local Cargo patch configuration. If a sibling `audio-analysis` checkout exists, its Git `HEAD` must exactly match the declared revision; otherwise coding-tooling can use the exact Git revision when the repository is accessible.

The generated `.cargo/config.toml` is ignored and must never be committed. Use `bash scripts/source-deps status` to inspect the mode and `bash scripts/source-deps deactivate` before registry-only release verification.

## Development contract

- Feature work may change Native WhisperX and the immediate audio capability source without starting a crates.io release.
- Keep audio package versions compatible during source work. Version bumps belong to a later release task.
- Update the exact revision in `.coding-tooling.source-deps.json` whenever the validated audio source head changes.
- Patch only the packages that directly cross the repository boundary: audio I/O, speakers, and transcription. Their same-repository path dependencies carry core, Fourier, recognition, and other internal audio implementation crates from the same exact `audio-analysis` checkout; those transitive crates do not need duplicate consumer patch declarations.
- Do not expand an application task into unrelated package maintenance. If more than two upstream repositories become necessary, treat that as an architecture boundary problem unless broader migration scope was explicitly assigned.
- Do not create new crates merely to avoid modifying an existing audio package boundary.

## Verification boundary

Source-mode verification proves that Native WhisperX works against the exact source graph under development. It is valid implementation evidence even when those crate versions are not yet present on crates.io.

Registry-only resolution remains mandatory before a Native WhisperX release. That later release task deactivates source mode, publishes or selects the required package versions through the release system, and verifies a clean checkout without patches.

Because `audio-analysis` and `coding-tooling` are private while Native WhisperX is public, untrusted fork pull requests cannot receive private-source credentials. Ordinary public CI therefore remains registry-only and continues to exercise the last published dependency graph.

Trusted same-repository pull requests and manual runs additionally use `.github/workflows/source-mode-ci.yml`. That workflow checks out the candidate commit, reads the exact `audio-analysis` revision from `.coding-tooling.source-deps.json`, checks out that source plus a pinned `coding-tooling` revision, activates source mode, and runs lint and Rust tests against the exact source graph. It requires the repository Actions secret `GH_PACKAGES_TOKEN` with `contents:read` access to both private repositories. A missing or under-scoped token is an infrastructure failure, not a reason to publish crates.

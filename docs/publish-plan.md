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

# Publish Plan

This repository consumes published crates from `rust-packages`. Publish the
crate dependency closure before expecting clean CI to resolve dependencies from
crates.io.

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


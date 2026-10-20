#!/usr/bin/env bash
# Build the actual upstream WASM bridge in an isolated exact source graph.
set -euo pipefail
AUDIO_WORKTREE="$1"
BUILD_ROOT="$2"
FOUNDATION_REV="9a162dfd44d3cae7f88ad993de79defc0ac94cd6"
TOOLING_REV="c9e071995abf65d2326930efb05e47e585af0efd"
command -v wasm-pack >/dev/null || { echo 'wasm-pack 0.14.0 is required to prepare browser alignment.' >&2; exit 1; }
command -v bun >/dev/null || { echo 'Bun is required to prepare browser alignment.' >&2; exit 1; }
for repository in moenarch-foundation coding-tooling; do
  if [[ "$repository" == moenarch-foundation ]]; then revision="$FOUNDATION_REV"; else revision="$TOOLING_REV"; fi
  git init -q "$BUILD_ROOT/$repository"
  git -C "$BUILD_ROOT/$repository" fetch --quiet --depth=1 "https://github.com/moritzbrantner/$repository.git" "$revision"
  git -C "$BUILD_ROOT/$repository" checkout --quiet --detach FETCH_HEAD
done
export CODING_TOOLING_DIR="$BUILD_ROOT/coding-tooling"
cd "$AUDIO_WORKTREE"
bash scripts/source-deps activate
bash packages/audio-analysis-transcription-wasm/scripts/build-wasm.sh
bash scripts/source-deps deactivate

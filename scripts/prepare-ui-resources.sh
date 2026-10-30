#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/browser-source-revisions.sh"
BUILD_ROOT="$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/native-whisperx-resources.XXXXXX")"
trap 'python3 -c "import shutil,sys; shutil.rmtree(sys.argv[1])" "$BUILD_ROOT"' EXIT
SOURCE="$BUILD_ROOT/audio-analysis"
TARGET="$ROOT/crates/native-whisperx-cli/speaker-directory-ui/vendor"
git init -q "$SOURCE"
git -C "$SOURCE" fetch --quiet --depth=1 https://github.com/moritzbrantner/audio-analysis.git "$AUDIO_ANALYSIS_REV"
git -C "$SOURCE" checkout --quiet --detach FETCH_HEAD
(cd "$SOURCE" && bun install --frozen-lockfile --ignore-scripts)
mkdir -p "$TARGET"
bun build "$SOURCE/packages/audio-analysis-speakers-wasm/resources.ts" --target=browser --format=esm --outfile "$TARGET/pyannote-resources.js"
(cd "$SOURCE/packages/audio-analysis-speakers-wasm" && bun run tsc -p tsconfig.community.json --noEmit false --declaration --emitDeclarationOnly --outDir "$BUILD_ROOT/types")
cp "$BUILD_ROOT/types/resources.d.ts" "$TARGET/pyannote-resources.d.ts"

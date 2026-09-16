#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AUDIO_ANALYSIS_REV="bf2cb13d155a874b166305da2f8dc669a05a2a58"
AUDIO_SOURCE_PATH="packages/audio-analysis-transcription-wasm/index.js"
AUDIO_TARGET="$ROOT/site/vendor/audio-analysis-transcription.js"
PLATFORM_PACKAGES_REV="7241402712feef010bb9b5733560043ea845eb3d"
PLATFORM_SOURCE_PATH="packages/browser-translation/src/browser.ts"
PLATFORM_TARGET="$ROOT/site/vendor/platform-browser-translation.js"
TEMP_ROOT="${RUNNER_TEMP:-${TMPDIR:-/tmp}}"
AUDIO_WORKTREE="$TEMP_ROOT/native-whisperx-audio-analysis-${AUDIO_ANALYSIS_REV}"
PLATFORM_WORKTREE="$TEMP_ROOT/native-whisperx-platform-packages-${PLATFORM_PACKAGES_REV}"

rm -rf "$AUDIO_WORKTREE" "$PLATFORM_WORKTREE"
trap 'rm -rf "$AUDIO_WORKTREE" "$PLATFORM_WORKTREE"' EXIT

checkout_exact() {
  local repository="$1"
  local revision="$2"
  local worktree="$3"

  git init -q "$worktree"
  git -C "$worktree" fetch --quiet --depth=1 "$repository" "$revision"
  git -C "$worktree" checkout --quiet --detach FETCH_HEAD
}

checkout_exact \
  https://github.com/moritzbrantner/audio-analysis.git \
  "$AUDIO_ANALYSIS_REV" \
  "$AUDIO_WORKTREE"
mkdir -p "$(dirname "$AUDIO_TARGET")"
cp "$AUDIO_WORKTREE/$AUDIO_SOURCE_PATH" "$AUDIO_TARGET"

checkout_exact \
  https://github.com/moritzbrantner/platform-packages.git \
  "$PLATFORM_PACKAGES_REV" \
  "$PLATFORM_WORKTREE"
command -v bun >/dev/null 2>&1 || {
  echo "bun is required to compile the pinned browser translation source" >&2
  exit 1
}
bun build "$PLATFORM_WORKTREE/$PLATFORM_SOURCE_PATH" \
  --target=browser \
  --format=esm \
  --outfile="$PLATFORM_TARGET"

grep -Fq 'export function browserTranscriptionCapabilities()' "$AUDIO_TARGET"
grep -Fq 'export async function transcribeAudioBlob' "$AUDIO_TARGET"
grep -Fq 'translation: false' "$AUDIO_TARGET"
grep -Fq 'server: false' "$AUDIO_TARGET"
grep -Fq 'cpu: false' "$AUDIO_TARGET"

grep -Fq 'browserTranslationCapabilities' "$PLATFORM_TARGET"
grep -Fq 'resolveBrowserTranslationPair' "$PLATFORM_TARGET"
grep -Fq 'translateBrowserSegments' "$PLATFORM_TARGET"
grep -Fq 'platform-packages-transformers-js-webgpu-translation' "$PLATFORM_TARGET"
grep -Fq 'onnx-community/opus-mt-de-en' "$PLATFORM_TARGET"
grep -Fq 'onnx-community/opus-mt-en-de' "$PLATFORM_TARGET"
grep -Fq 'webgpu' "$PLATFORM_TARGET"

printf 'Prepared audio-analysis browser transcription adapter at %s from %s\n' "$AUDIO_TARGET" "$AUDIO_ANALYSIS_REV"
printf 'Prepared platform-packages browser translation adapter at %s from %s\n' "$PLATFORM_TARGET" "$PLATFORM_PACKAGES_REV"

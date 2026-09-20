#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AUDIO_ANALYSIS_REV="bf2cb13d155a874b166305da2f8dc669a05a2a58"
SOURCE_PATH="packages/audio-analysis-transcription-wasm/index.js"
TARGET="$ROOT/site/vendor/audio-analysis-transcription.js"
PLATFORM_PACKAGES_REV="9eb1a19ba4b5bed3f02161682aa1a38abfb1f128"
TRANSLATION_SOURCE_PATH="packages/browser-translation"
TRANSLATION_TARGET="$ROOT/site/vendor/browser-translation.js"
TEMP_ROOT="${RUNNER_TEMP:-${TMPDIR:-/tmp}}"
AUDIO_WORKTREE="$TEMP_ROOT/native-whisperx-audio-analysis-${AUDIO_ANALYSIS_REV}"
TRANSLATION_WORKTREE="$TEMP_ROOT/native-whisperx-platform-packages-${PLATFORM_PACKAGES_REV}"

rm -rf "$AUDIO_WORKTREE" "$TRANSLATION_WORKTREE"
trap 'rm -rf "$AUDIO_WORKTREE" "$TRANSLATION_WORKTREE"' EXIT

git init -q "$AUDIO_WORKTREE"
git -C "$AUDIO_WORKTREE" fetch --quiet --depth=1 https://github.com/moritzbrantner/audio-analysis.git "$AUDIO_ANALYSIS_REV"
git -C "$AUDIO_WORKTREE" checkout --quiet --detach FETCH_HEAD

mkdir -p "$(dirname "$TARGET")"
cp "$AUDIO_WORKTREE/$SOURCE_PATH" "$TARGET"

grep -Fq 'export function browserTranscriptionCapabilities()' "$TARGET"
grep -Fq 'export async function transcribeAudioBlob' "$TARGET"
grep -Fq 'translation: false' "$TARGET"
grep -Fq 'server: false' "$TARGET"
grep -Fq 'cpu: false' "$TARGET"

printf 'Prepared audio-analysis browser transcription adapter at %s from %s\n' "$TARGET" "$AUDIO_ANALYSIS_REV"

command -v bun >/dev/null 2>&1 || {
  printf '%s\n' 'Bun is required to build the pinned browser translation adapter.' >&2
  exit 1
}

git init -q "$TRANSLATION_WORKTREE"
git -C "$TRANSLATION_WORKTREE" fetch --quiet --depth=1 https://github.com/moritzbrantner/platform-packages.git "$PLATFORM_PACKAGES_REV"
git -C "$TRANSLATION_WORKTREE" checkout --quiet --detach FETCH_HEAD

bun build "$TRANSLATION_WORKTREE/$TRANSLATION_SOURCE_PATH/src/browser.ts" \
  --target=browser \
  --format=esm \
  --outfile "$TRANSLATION_TARGET"

grep -Fq 'browserTranslationCapabilities' "$TRANSLATION_TARGET"
grep -Fq 'translateBrowserSegments' "$TRANSLATION_TARGET"
grep -Fq 'requiredAcceleration: "webgpu"' "$TRANSLATION_TARGET"
grep -Fq 'modelProvisioning: "browser-cache"' "$TRANSLATION_TARGET"
grep -Fq 'server: false' "$TRANSLATION_TARGET"
grep -Fq 'python: false' "$TRANSLATION_TARGET"
grep -Fq 'cpu: false' "$TRANSLATION_TARGET"

printf 'Prepared platform-packages browser translation adapter at %s from %s\n' "$TRANSLATION_TARGET" "$PLATFORM_PACKAGES_REV"

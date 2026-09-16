#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AUDIO_ANALYSIS_REV="bf2cb13d155a874b166305da2f8dc669a05a2a58"
SOURCE_PATH="packages/audio-analysis-transcription-wasm/index.js"
TARGET="$ROOT/site/vendor/audio-analysis-transcription.js"
WORKTREE="${RUNNER_TEMP:-${TMPDIR:-/tmp}}/native-whisperx-audio-analysis-${AUDIO_ANALYSIS_REV}"

rm -rf "$WORKTREE"
trap 'rm -rf "$WORKTREE"' EXIT

git init -q "$WORKTREE"
git -C "$WORKTREE" fetch --quiet --depth=1 https://github.com/moritzbrantner/audio-analysis.git "$AUDIO_ANALYSIS_REV"
git -C "$WORKTREE" checkout --quiet --detach FETCH_HEAD

mkdir -p "$(dirname "$TARGET")"
cp "$WORKTREE/$SOURCE_PATH" "$TARGET"

grep -Fq 'export function browserTranscriptionCapabilities()' "$TARGET"
grep -Fq 'export async function transcribeAudioBlob' "$TARGET"
grep -Fq 'translation: false' "$TARGET"
grep -Fq 'server: false' "$TARGET"
grep -Fq 'cpu: false' "$TARGET"

printf 'Prepared audio-analysis browser transcription adapter at %s from %s\n' "$TARGET" "$AUDIO_ANALYSIS_REV"

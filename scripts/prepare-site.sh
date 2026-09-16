#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AUDIO_ANALYSIS_REV="bf2cb13d155a874b166305da2f8dc669a05a2a58"
AUDIO_SOURCE_PATH="packages/audio-analysis-transcription-wasm/index.js"
AUDIO_TARGET="$ROOT/site/vendor/audio-analysis-transcription.js"
NLP_STACK_REV="8ddf4957bef5be662c8ca5e6d1b991cd1c96aa35"
NLP_SOURCE_PATH="browser/translation.js"
NLP_TARGET="$ROOT/site/vendor/nlp-browser-translation.js"
TEMP_ROOT="${RUNNER_TEMP:-${TMPDIR:-/tmp}}"
AUDIO_WORKTREE="$TEMP_ROOT/native-whisperx-audio-analysis-${AUDIO_ANALYSIS_REV}"
NLP_WORKTREE="$TEMP_ROOT/native-whisperx-nlp-stack-${NLP_STACK_REV}"

rm -rf "$AUDIO_WORKTREE" "$NLP_WORKTREE"
trap 'rm -rf "$AUDIO_WORKTREE" "$NLP_WORKTREE"' EXIT

fetch_source_file() {
  local repository="$1"
  local revision="$2"
  local source_path="$3"
  local target="$4"
  local worktree="$5"

  git init -q "$worktree"
  git -C "$worktree" fetch --quiet --depth=1 "$repository" "$revision"
  git -C "$worktree" checkout --quiet --detach FETCH_HEAD
  mkdir -p "$(dirname "$target")"
  cp "$worktree/$source_path" "$target"
}

fetch_source_file \
  https://github.com/moritzbrantner/audio-analysis.git \
  "$AUDIO_ANALYSIS_REV" \
  "$AUDIO_SOURCE_PATH" \
  "$AUDIO_TARGET" \
  "$AUDIO_WORKTREE"

fetch_source_file \
  https://github.com/moritzbrantner/nlp-stack.git \
  "$NLP_STACK_REV" \
  "$NLP_SOURCE_PATH" \
  "$NLP_TARGET" \
  "$NLP_WORKTREE"

grep -Fq 'export function browserTranscriptionCapabilities()' "$AUDIO_TARGET"
grep -Fq 'export async function transcribeAudioBlob' "$AUDIO_TARGET"
grep -Fq 'translation: false' "$AUDIO_TARGET"
grep -Fq 'server: false' "$AUDIO_TARGET"
grep -Fq 'cpu: false' "$AUDIO_TARGET"

grep -Fq 'export function browserTranslationCapabilities()' "$NLP_TARGET"
grep -Fq 'export async function translateBrowserSegments' "$NLP_TARGET"
grep -Fq 'requiredAcceleration: "webgpu"' "$NLP_TARGET"
grep -Fq 'server: false' "$NLP_TARGET"
grep -Fq 'python: false' "$NLP_TARGET"
grep -Fq 'cpu: false' "$NLP_TARGET"

printf 'Prepared audio-analysis browser transcription adapter at %s from %s\n' "$AUDIO_TARGET" "$AUDIO_ANALYSIS_REV"
printf 'Prepared nlp-stack browser translation adapter at %s from %s\n' "$NLP_TARGET" "$NLP_STACK_REV"

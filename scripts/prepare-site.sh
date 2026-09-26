#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AUDIO_ANALYSIS_REV="51187f9cb0f1ed2984d4a5af97e6e6c73a17c0b1"
TRANSCRIPTION_SOURCE_PATH="packages/audio-analysis-transcription-wasm/index.js"
TRANSCRIPTION_TARGET="$ROOT/site/vendor/audio-analysis-transcription.js"
SPEAKER_SOURCE_PATH="packages/audio-analysis-speakers-wasm/index.js"
SPEAKER_TARGET="$ROOT/site/vendor/audio-analysis-speakers.js"
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

mkdir -p "$(dirname "$TRANSCRIPTION_TARGET")"
cp "$AUDIO_WORKTREE/$TRANSCRIPTION_SOURCE_PATH" "$TRANSCRIPTION_TARGET"
cp "$AUDIO_WORKTREE/$SPEAKER_SOURCE_PATH" "$SPEAKER_TARGET"

grep -Fq 'export function browserTranscriptionCapabilities()' "$TRANSCRIPTION_TARGET"
grep -Fq 'export function browserTranscriptionModels()' "$TRANSCRIPTION_TARGET"
grep -Fq 'export async function decodeBrowserAudioBlob' "$TRANSCRIPTION_TARGET"
grep -Fq 'export async function transcribeAudioSamples' "$TRANSCRIPTION_TARGET"
grep -Fq 'function normalizeBrowserTranscriptText' "$TRANSCRIPTION_TARGET"
grep -Fq 'translation: false' "$TRANSCRIPTION_TARGET"
grep -Fq 'server: false' "$TRANSCRIPTION_TARGET"
grep -Fq 'cpu: false' "$TRANSCRIPTION_TARGET"

grep -Fq 'export function browserDiarizationCapabilities()' "$SPEAKER_TARGET"
grep -Fq 'export async function diarizeBrowserAudioSamples' "$SPEAKER_TARGET"
grep -Fq 'export function assignBrowserDiarizationToTranscript' "$SPEAKER_TARGET"
grep -Fq 'onnx-community/pyannote-segmentation-3.0' "$SPEAKER_TARGET"
grep -Fq 'Xenova/wavlm-base-plus-sv' "$SPEAKER_TARGET"
grep -Fq 'server: false' "$SPEAKER_TARGET"
grep -Fq 'python: false' "$SPEAKER_TARGET"

printf 'Prepared audio-analysis browser transcription and diarization adapters from %s\n' "$AUDIO_ANALYSIS_REV"

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

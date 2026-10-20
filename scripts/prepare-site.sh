#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AUDIO_ANALYSIS_REV="55ff3c04739ecfa34ee5082db5845b55e69abecf"
IO_SOURCE_PATH="packages/audio-analysis-io-wasm/index.js"
IO_TARGET="$ROOT/site/vendor/audio-analysis-io.js"
SOURCE_PATH="packages/audio-analysis-transcription-wasm/index.js"
TARGET="$ROOT/site/vendor/audio-analysis-transcription.js"
SPEAKERS_SOURCE_PATH="packages/audio-analysis-speakers-wasm/index.js"
SPEAKERS_TARGET="$ROOT/site/vendor/audio-analysis-speakers.js"
PLATFORM_PACKAGES_REV="9eb1a19ba4b5bed3f02161682aa1a38abfb1f128"
TRANSLATION_SOURCE_PATH="packages/browser-translation"
TRANSLATION_TARGET="$ROOT/site/vendor/browser-translation.js"
TEMP_ROOT="${RUNNER_TEMP:-${TMPDIR:-/tmp}}"
BUILD_ROOT="$(mktemp -d "$TEMP_ROOT/native-whisperx-browser.XXXXXX")"
AUDIO_WORKTREE="$BUILD_ROOT/audio-analysis"
TRANSLATION_WORKTREE="$BUILD_ROOT/platform-packages"
trap 'python3 -c "import shutil,sys; shutil.rmtree(sys.argv[1])" "$BUILD_ROOT"' EXIT

git init -q "$AUDIO_WORKTREE"
git -C "$AUDIO_WORKTREE" fetch --quiet --depth=1 https://github.com/moritzbrantner/audio-analysis.git "$AUDIO_ANALYSIS_REV"
git -C "$AUDIO_WORKTREE" checkout --quiet --detach FETCH_HEAD

mkdir -p "$(dirname "$TARGET")"
cp "$AUDIO_WORKTREE/$IO_SOURCE_PATH" "$IO_TARGET"
cp "$AUDIO_WORKTREE/$SOURCE_PATH" "$TARGET"
cp "$AUDIO_WORKTREE/$SPEAKERS_SOURCE_PATH" "$SPEAKERS_TARGET"
bash "$ROOT/scripts/prepare-browser-wasm.sh" "$AUDIO_WORKTREE" "$BUILD_ROOT"
bun build "$AUDIO_WORKTREE/packages/audio-analysis-transcription-wasm/alignment.ts" --target=browser --format=esm --outfile "$ROOT/site/vendor/audio-analysis-alignment.js"
mkdir -p "$ROOT/site/vendor/pkg"
cp "$AUDIO_WORKTREE/packages/audio-analysis-transcription-wasm/pkg/"* "$ROOT/site/vendor/pkg/"

grep -Fq 'export function browserAudioDecodeCapabilities()' "$IO_TARGET"
grep -Fq 'export function supportsBrowserAudioDecode()' "$IO_TARGET"
grep -Fq 'export async function decodeBrowserAudioBlob' "$IO_TARGET"
grep -Fq 'server: false' "$IO_TARGET"
grep -Fq 'python: false' "$IO_TARGET"

grep -Fq 'export function browserTranscriptionCapabilities()' "$TARGET"
grep -Fq 'export function browserTranscriptionModels()' "$TARGET"
grep -Fq 'export async function transcribeAudioSamples' "$TARGET"
grep -Fq 'function normalizeBrowserTranscriptText' "$TARGET"
grep -Fq 'translation: false' "$TARGET"
grep -Fq 'server: false' "$TARGET"
grep -Fq 'cpu: false' "$TARGET"

grep -Fq 'export function browserDiarizationCapabilities()' "$SPEAKERS_TARGET"
grep -Fq 'export function diarizeAudioSamples' "$SPEAKERS_TARGET"
grep -Fq 'export function assignBrowserSpeakersToSegments' "$SPEAKERS_TARGET"
grep -Fq 'quality: "deterministic-baseline"' "$SPEAKERS_TARGET"
grep -Fq 'server: false' "$SPEAKERS_TARGET"
grep -Fq 'python: false' "$SPEAKERS_TARGET"

printf 'Prepared audio-analysis browser audio I/O adapter at %s from %s\n' "$IO_TARGET" "$AUDIO_ANALYSIS_REV"
printf 'Prepared audio-analysis browser transcription adapter at %s from %s\n' "$TARGET" "$AUDIO_ANALYSIS_REV"
printf 'Prepared audio-analysis browser diarization adapter at %s from %s\n' "$SPEAKERS_TARGET" "$AUDIO_ANALYSIS_REV"

command -v bun >/dev/null 2>&1 || {
  printf '%s\n' 'Bun is required to build the pinned browser translation adapter.' >&2
  exit 1
}

git init -q "$TRANSLATION_WORKTREE"
git -C "$TRANSLATION_WORKTREE" fetch --quiet --depth=1 https://github.com/moritzbrantner/platform-packages.git "$PLATFORM_PACKAGES_REV"
git -C "$TRANSLATION_WORKTREE" checkout --quiet --detach FETCH_HEAD

bun build "$ROOT/ui/languages.ts" --target=browser --format=esm --outfile "$ROOT/site/vendor/workbench-languages.js"

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

# Issue #428: speech evidence and music-only hallucinations

## Scope and confirmed baseline (2026-11-03)

The CC BY *Blender 3.6 LTS - Reel* (https://www.youtube.com/watch?v=BnY2JdBrvaI)
is an 85-second music-only reproduction. The issue records fabricated `Oh!`,
`I` and repeating greetings from the native `small` model with CPU float32,
English, batch size 1 and no alignment. Silero VAD did not eliminate all of
them, and Python WhisperX also fabricated text. These are issue-reported
observations, not a new acceptance run.

## Current source-level change

- Audio Analysis already calculates no-speech probability, average log
  probability, zlib text compression and temperature fallback diagnostics.
- When all fallback attempts exceed the configured compression threshold,
  Audio Analysis now withholds the selected decoded words and records
  `compressionRejectedWindows`. This also applies to a one-temperature
  schedule. It does **not** infer silence or music.
- The reusable pipeline distinguishes energy activity, model-VAD candidates,
  a provider returning no spans, and unmeasured VAD. Its existing full-source
  ASR fallback on zero spans remains: a VAD miss must not erase quiet speech.
- Native WhisperX maps these observations to its product-owned
  `speechEvidence` report. WhisperX-compatible transcript JSON remains
  unchanged. No new model or detector is introduced.
- This draft pins the three Audio Analysis source dependencies to
  `477c6da95fca766e48b475aa0da3b698eb306990` for cross-repository CI
  validation. That is a *candidate* commit from Audio Analysis PR #234, not
  a reviewed release or a dependency upgrade approved for merging.

## Real-audio acceptance still required

Use the same pinned `small` model and 16 kHz mono input for A/B runs at the
exact candidate source revisions, and preserve the previous output as baseline.
Run the native CLI with `--model small --language en --compute-type float32
--batch-size 1 --no-align` and compare `energy`, cached `silero` and (if
available) `pyannote` VAD separately. Record every output segment,
`noSpeechProbability`, `compressionRatio`, attempted temperatures, rejected
window counts, wall time, model load time and peak memory. A run lacking the
required model must be marked unavailable, not passed.

| Independent input | Expected review gate |
| --- | --- |
| CC BY Blender reel (music only, 85 s) | No fabricated words; no unsupported speech/silence certainty |
| Quiet intelligible speech | Genuine words remain; VAD misses stay visible |
| Speech over music | Genuine speech remains at the expected times |
| Singing | Do not misclassify as speech-free merely from repetition |
| Silence | No fabricated words; do not label unavailable evidence as silence |
| Noise without speech | No fabricated words where provider evidence supports rejection |
| Ambiguous speech/noise | Preserve unknown status and reviewability |

These inputs require independent annotations and auditable license/provenance
before a real-audio reliability claim. The deterministic unit tests cover
the exhausted-compression case and surviving under-threshold candidates;
they are not substitutes for the acoustic matrix.

## Consumer and cost gate

Validate Native WhisperX in source mode against the candidate Audio Analysis
commit first. After review, select the approved exact dependency revision and
then advance the reviewed Native WhisperX revision used by YouTube Corpus.
Process the Blender video through
the normal Corpus ASR ingest/store/retrieval path, inspect the stored stream
and its provenance, and repeat with speech-over-music and quiet speech. Do not
manually filter words in Corpus or introduce a second detector there.

The change adds no inference pass, downloaded model or audio-buffer copy.
Compression is already computed per decode attempt; the new gate inspects its
existing scalar and emits bounded counters. It may still reject legitimate
highly repetitive words, so this is a **candidate reliability improvement**,
not verified completion of #428 or Audio Analysis #233.

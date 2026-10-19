# Full-video alignment chronology regression (#372)

The finite native workflow now writes valid source-timed WhisperX JSON after
independently aligning overlapping ASR segments. Audio Analysis owns the repair
in [#161](https://github.com/moritzbrantner/audio-analysis/pull/161).

The minimized real failure contains two neighboring ASR segments. Their source
intervals overlap; CTC moves the later segment's start about 13 ms before the
previous segment's aligned start. The pipeline had retained the original array
order after replacing its envelopes. The repair attaches all words and
characters by stable segment ID, then moves complete segment records into
chronological order. Measured timestamps, IDs, text, speaker metadata, and
nested associations remain attached to their segment. Strict validation remains
enabled. Aggregate text follows a changed segment order.

## Source graph

- Native WhisperX Rust code: `f2b3c23` with this change's source-dependency pins.
- Audio Analysis: `cacb23f725b86e0610404fb6a5537f43b12481f7`.
- Foundation: `9a162dfd44d3cae7f88ad993de79defc0ac94cd6`.
- Shared conventions sourceRevision: `6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.
- CUDA: NVIDIA RTX 3060 Ti; float32, batch size 1.
- Whisper small cache snapshot: `973afd24965f72e36ca33b3055d56a652f456b4d`.
- wav2vec2 cache snapshot: `22aad52d435eb6dbaf354bdad9b0da84ce7d6156`.

## Reproduction and result

The full input is video `04Q9vcbMwkg`, 639.341 seconds, downloaded with both video
and audio and remuxed into MP4. Its SHA-256 is
`08fc596aa661ab72cc51b96331f0f0e949c435537e6def47f452db293a558334`.
The 30-second control MP4 has SHA-256
`223e38dbc45ac89ada7c4e312a39eb2eeb0d99c3859decd8a0e653020e23171c`.

Prepare the exact sibling source checkouts, activate source mode, and build:

```bash
bash scripts/source-deps activate
cargo build -p native-whisperx-cli --release --features cuda --locked

yt-dlp -f 'bestvideo[height<=360]+bestaudio/best[height<=360]' \
  --merge-output-format mp4 -o 'video.%(ext)s' \
  'https://www.youtube.com/watch?v=04Q9vcbMwkg'
ffmpeg -i video.mp4 -t 30 -c copy short-video.mp4

target/release/native-whisperx transcribe video.mp4 \
  --model small --language en --model-cache-only \
  --compute-type float32 --batch-size 1 --output-dir out --format json
# Repeat with short-video.mp4 and a separate output directory.
```

| Input | Exit | Segments | Words | Last segment end | Elapsed |
| --- | --- | --- | --- | --- | --- |
| Full MP4 | 0 | 332 | 1,658 | 638.609429 s | 114.303 s |
| Short MP4 | 0 | 20 | 76 | 29.940000 s | 16.681 s |

Both JSON outputs were read back and checked for nondecreasing segment starts,
valid segment ranges, and containment of every timed word in its associated
segment. Single-run times were collected during concurrent compilation. Peak
resident set sizes were 1,323,560 KiB (full) and 1,202,668 KiB (short).

Before this repair, the installed registry binary 0.1.13 and current Native
WhisperX code with its old source pins both reproduced the backward-start error
on the complete audio stream. The short control and a full `--no-align` run
passed. Frozen ASR plus real CTC isolated the two-segment failure; either segment
alone passed. The upstream deterministic public-pipeline regression reproduced
the same error before the fix and then passed, checking IDs, text, words,
characters, speaker metadata, measured times, and aggregate text.

Local validation passed: `cargo fmt --check`, workspace Clippy with warnings as
errors, workspace tests, and workspace tests without default features, all with
`--locked` in source mode. Existing ignored resource/release tests retain their
separate tiers. No registry publication or package version change is involved.

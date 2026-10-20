# Issue 380: synchronized Review & Edit

The local release app opened an actual completed native result from #379, sought
its second segment, played the source WAV and matching parent MP4, edited exact
text/timing/speaker annotation, previewed and saved a new canonical copy plus
WhisperX JSON/SRT. Original result/audio hashes stayed unchanged; all neighboring
projected segments compared equal. Discard restored the opened text. Keyboard
Arrow Down/Up moved focus and seek between segment buttons. Opaque navigation
contained result/segment IDs, not paths or transcript text.

The recording is the approximately ten-second clip from the #372 source video.
Explicit Locate source recording selected its matching 639.341-second parent MP4;
both formats played and advanced their actual media clocks. Playback used HTTP
206 byte ranges. The second segment's existing overlap was displayed and retained
in source order. The speaker labels came from the synthetic two-dimensional
confirmed/draft transfer fixture; this checks annotation and identity separation,
not voice recognition. Annotation/save preserved the Speaker Library hash, and the
identity action opened the existing Speakers editor in another tab.

A new CPU native run transcribed and translated the clip using cached Whisper
small and Helsinki-NLP/opus-mt-en-de. Decode, VAD, ASR, translation and output each
started once. ASR took 27.538 seconds; translation took 19.388 seconds. Its canonical
Native JSON retained the original source transcript and actual model provenance.
The browser corrected translation and source independently, then exported source
SRT while preserving the corrected translated layer and provenance in the master.
Exactly one transcription request occurred during this complete review/translation
journey; preview and correction did not rerun inference.

`observations.json` records measured results, hashes and the final binary hash.
The final desktop/mobile captures reopen the persisted corrected copy after a
server restart. The 390px touch check edited/discarded text, retained a 390px
document, and verified that the video did not obscure the editor. No uncaught
page errors were observed. Audio, canonical transcript files, voice embeddings,
session tokens and media/preview capabilities remain outside the repository.

## Failures that shaped the change

The first deterministic correction/export test found that the existing subtitle
projection dropped an unaligned corrected segment beside aligned segments. The
repair retains contiguous aligned groups and emits corrected segments separately;
existing output goldens still pass. A transport test found that an early token
rejection reset connections with unread bodies; bounded request consumption now
precedes the review authorization response. Recording recovery also rejects JSON
and other non-media files using the existing product media selection policy.

Real-browser checks found wrapped textarea/select labels incorporating their
values/options; explicit accessible labels stabilized those controls. A reset
assertion was corrected to await the asynchronous response. Visual inspection
found a mobile sticky-video overlap; the final stacked layout and touch edit test
verify its repair. No failed check is represented as a passing observation.

## Reproduction and validation

Activate the exact source siblings and build the native CLI with cached models:

```bash
bash scripts/source-deps activate
cargo build -p native-whisperx-cli --release --features cuda --locked
```

Launch the workbench from a recording directory. Open a completed Jobs result,
select segment two, seek/play, change its text and exact times, choose a known
speaker annotation, preview and save a new name. Compare original bytes and
neighboring segments, then reopen the returned opaque link. Repeat with a
translated result, separately editing Source transcript and Translation and
choosing the source export layer. Restart the server to reopen saved copies.

Passed: headless correction/canonical round trips, finite/backward range rejection,
supported overlap preservation, speaker-only word timing, non-overwriting export,
source/translation separation and source SRT export; local API/session/range checks,
consumed previews, large bounded edit requests and stable copy reopening across
restart; React exact-entry/preview/save, keyboard/reset and bilingual draft tests.
Workspace/default and no-default tests, Clippy with warnings denied, formatting,
composition ownership and the frontend format/typecheck/25-test/build ladder pass.
Production assets match the checked-in build.

Source dependencies: Audio Analysis `e35f82ba87facbedc92a97352ab2dce00ccfec66`,
foundation `9a162dfd44d3cae7f88ad993de79defc0ac94cd6`.
Convention sourceRevision: `6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.
These results establish review/copy/playback behavior; #362's diarization quality
acceptance remains separate and incomplete.

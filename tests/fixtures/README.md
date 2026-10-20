# Fixtures

`whisperx-parity-sample.json` is a small reviewed fixture copied from
`rust-packages/tests/fixtures/whisperx-parity-sample.json`. It exercises
WhisperX-style segments, flat `word_segments`, speaker labels, confidences, and
unknown-field preservation.

Real FFmpeg finite media decode smoke fixtures are generated at test runtime in
temporary directories by
`crates/native-whisperx-cli/tests/real_ffmpeg_media_decode_smoke.rs`. Do not
commit binary audio or video fixtures for that smoke unless a future slice
explicitly replaces the generated-fixture approach.

`speaker-library-with-drafts.json` is synthetic canonical transfer contract data:
two-dimensional one-hot vectors, one confirmed profile and one draft. Its model
metadata exercises schema compatibility; these are not trained voice embeddings
and must not be used as recognition-quality evidence.

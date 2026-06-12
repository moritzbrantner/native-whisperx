# native-whisperx

Reusable workflow library for composing `moritzbrantner-*` transcription,
alignment, diarization, and transcript crates into a WhisperX-style pipeline.

This crate owns workflow configuration and output writing. The reusable audio,
text, model-runtime, and speaker primitives remain in `rust-packages`.


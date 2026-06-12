# Use Hugging Face cache conventions for model resolution

When users provide model IDs instead of local bundle paths, native-whisperx
should resolve assets through Hugging Face model and cache conventions. This
keeps the CLI aligned with WhisperX users' existing model ecosystem instead of
creating an app-private bundle format.

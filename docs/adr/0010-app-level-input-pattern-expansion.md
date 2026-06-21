# Expand input patterns in the CLI

native-whisperx expands wildcard input patterns itself before transcription
instead of relying only on shell expansion, because users need quoted patterns
such as `*.wav` to work consistently across shells. When no output directory is
selected, transcript files are written beside each input so multi-file commands
are collision-safe by default; a shared `--output-dir` remains available but
fails before transcription if multiple inputs would write the same basename.

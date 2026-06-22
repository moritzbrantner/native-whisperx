# Expand input patterns in the CLI

native-whisperx expands wildcard input patterns itself before transcription
instead of relying only on shell expansion, because users need quoted patterns
such as `*.wav` and `*.mp4` to work consistently across shells. Input Pattern
Expansion resolves concrete finite media file paths; it does not recursively
discover media or silently filter broad globs by extension. When no output
directory is selected, transcript files are written beside each input so
Multi-Input Transcription Run commands are collision-safe by default; a shared
`--output-dir` remains available but fails before transcription if multiple
inputs would write the same basename.

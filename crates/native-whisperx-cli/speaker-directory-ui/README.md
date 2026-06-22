# Speaker Directory UI

This is the Bun/Vite/React workspace for the CLI-served Speaker Directory UI.
It is separate from the repository contributor site in `site/`.

By default, `bun run dev` uses mocked Speaker Directory data and does not
require the Rust CLI server.

To target a real local CLI server during development, set:

```sh
NATIVE_WHISPERX_SPEAKER_DIRECTORY_API_BASE=http://127.0.0.1:PORT bun run dev
```

The app preserves the repository terminology: Speaker Directory, Speaker
Library, Speaker Trace, and Anonymous Speaker Label.

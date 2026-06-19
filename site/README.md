# native-whisperx GitHub Pages site

This directory contains the static contributor site published by the GitHub
Pages workflow. It intentionally has no JavaScript build step or package
manager dependency.

## Local preview

Open `index.html` directly, or serve the directory with Python:

```bash
python3 -m http.server 8000 -d site
```

Then open `http://127.0.0.1:8000/`.

## Updating benchmark content

Benchmark copy on the site is curated from checked-in repository notes. When
updating numbers:

1. Update the benchmark source note first.
2. Copy only contributor-safe values into `index.html`.
3. Keep the benchmark context beside the numbers: input, model, device, and
   provider path.
4. Preserve the caveat when native is not faster overall.
5. Avoid local absolute paths, smoke-root paths, private cache paths, tokens, or
   machine-specific command output.

The current source is `native-asr-batching-and-decode-speedup.md`.

## Deployment

The Pages workflow uploads this directory as a static artifact and deploys it
with GitHub Pages. Repository admins still need GitHub Pages enabled for the
repository and configured to use GitHub Actions as the publishing source.

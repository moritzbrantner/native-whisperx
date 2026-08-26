# Source CI credential boundary

The trusted source-mode workflow uses `GH_PACKAGES_TOKEN` only to read the private `moritzbrantner/audio-analysis` and `moritzbrantner/coding-tooling` repositories.

The token must be stored as a repository Actions secret and should have the narrowest available `contents:read` access to those repositories. It is never available to fork pull requests. The workflow runs only for same-repository pull requests or explicit manual dispatches and uses `persist-credentials: false` for every checkout.

Failure to provide this credential blocks trusted source-mode CI only. It must never cause an implementation task to publish crates as a workaround.

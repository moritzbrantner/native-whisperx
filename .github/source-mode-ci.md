# Trusted source-mode CI

The `source mode ci` workflow is the implementation gate for same-repository work that depends on unreleased audio source.

It is intentionally separate from registry-only CI. Fork pull requests never receive private repository credentials and continue to exercise the last published dependency graph. Release tasks still require a clean registry-only verification after source mode is deactivated.

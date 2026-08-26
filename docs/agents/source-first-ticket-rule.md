# Source-first ticket reconciliation

Repository-level development policy overrides older implementation-ticket wording that made registry publication a prerequisite for feature work.

When an open issue says that an upstream crate must already be published, that a Git/path patch is forbidden, or that work is blocked on a public release, interpret that constraint as release/distribution evidence unless the issue explicitly explains why registry resolution is itself part of the feature being implemented.

For ordinary implementation:

- use `docs/source-development.md` and the exact managed source graph;
- keep committed package coordinates and versions as the release contract;
- do not start a version bump, crates.io publication, tag, or release train merely to satisfy stale ticket wording;
- update or remove stale ticket constraints when they are encountered;
- keep genuine release/distribution tickets separate and unchanged unless the assigned task is explicitly a release.

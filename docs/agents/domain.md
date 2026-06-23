# Domain Context

This is a single-context repository.

Before routing or implementing substantial work, read:

- `CONTEXT.md`
- Relevant ADRs in `docs/adr/`

The current context is Workflow Composition. It owns the repository language for transcription, alignment, diarization, output writing, parity, Speaker Directory, Speaker Library, and Speaker Trace behavior.

Recent Workflow Composition terms include `Transcription Progress Stream`; keep
the canonical definition in `CONTEXT.md` and use it when distinguishing live
native progress from transcript outputs, final report JSON, parity reports, and
diagnostics.

When changing behavior covered by an ADR, read the relevant ADR first and preserve its decision unless the work explicitly includes superseding it with a new ADR.

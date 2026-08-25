# Agent Instructions

## Required Reading

- `CONTEXT.md`
- Relevant ADRs under `docs/adr/`
- `docs/source-development.md` for unreleased cross-repository dependency work
- The assigned GitHub issue or PRD
- Nearby tests for the behavior being changed

## Working Rules

- Preserve existing parity terminology from `CONTEXT.md`.
- Prefer small, behavior-focused changes.
- Do not revert unrelated dirty files.
- For bugs, reproduce the failing behavior before fixing where practical.
- Run the narrowest meaningful check first, then broader checks before handoff when feasible.
- Use source development mode when a Native WhisperX task needs unreleased audio changes.
- Do not publish crates or start a release train merely to unblock feature work.
- Keep a normal task to Native WhisperX plus at most two upstream repositories unless broader migration scope was explicitly assigned.
- Do not create a new crate without an independent-versioning reason.
- Registry-only dependency verification is a release gate, not a prerequisite for source-mode implementation evidence.

## Agent skills

This repository is configured for the Matt Pocock workflow skills and the long-running agent-loop workflow.

Use GitHub Issues as the durable source of truth for planning, triage, implementation slices, and agent-loop state.

Read these repo-local instructions before routing or implementing substantial work:

- Issue tracker: `docs/agents/issue-tracker.md`
- Triage labels: `docs/agents/triage-labels.md`
- Domain context: `docs/agents/domain.md`
- Planning workflow: `docs/agents/planning-workflow.md`
- TDD workflow: `docs/agents/tdd-workflow.md`

### Planning workflow

Substantial new work should be planned into GitHub PRD issues instead of implemented directly. See `docs/agents/planning-workflow.md`.

Substantial or TDD-oriented implementation work should follow `docs/agents/tdd-workflow.md`.

# Agent Instructions

## Required Reading

- `CONTEXT.md`
- Relevant ADRs under `docs/adr/`
- The assigned GitHub issue or PRD
- Nearby tests for the behavior being changed

## Working Rules

- Preserve existing parity terminology from `CONTEXT.md`.
- Prefer small, behavior-focused changes.
- Do not revert unrelated dirty files.
- For bugs, reproduce the failing behavior before fixing where practical.
- Run the narrowest meaningful check first, then broader checks before handoff when feasible.

## Agent skills

This repository is configured for the Matt Pocock workflow skills and the long-running agent-loop workflow.

Agent workflow references:

- Issue tracker: [docs/agents/issue-tracker.md](docs/agents/issue-tracker.md)
- Triage labels: [docs/agents/triage-labels.md](docs/agents/triage-labels.md)
- Domain context: [docs/agents/domain.md](docs/agents/domain.md)
- Planning workflow: [docs/agents/planning-workflow.md](docs/agents/planning-workflow.md)

### Planning workflow

Substantial new work should be planned into GitHub PRD issues instead of implemented directly. See `docs/agents/planning-workflow.md`.

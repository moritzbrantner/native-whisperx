# Planning Workflow

This repo is configured for the agent-loop workflow. GitHub Issues are the durable work queue.

## Default For Substantial Work

When a future thread settles a substantial implementation plan, default to creating a GitHub PRD issue instead of implementing directly.

Substantial work includes multi-step changes, ambiguous product behavior, cross-module work, schema/API changes, user-facing flows, or anything that would benefit from independent implementation slices.

Tiny one-shot fixes may be implemented directly.

## PRD Issue Rules

When the plan is decision-complete, synthesize the conversation into a PRD issue using the project issue tracker.

The PRD body must include explicit acceptance criteria and out-of-scope boundaries before it is marked ready for the agent loop.

Apply both labels:

- `prd`
- `ready-for-agent`

The `prd` + `ready-for-agent` label pair authorizes the agent-loop to route the PRD through issue slicing later.

Implementation slice issues created from the PRD must include:

```markdown
## Parent

#<parent-prd-issue-number>
```

Do not label implementation slices `ready-for-agent` until that parent link exists.

## Stop Point

After creating the PRD issue, report the issue number or URL and stop.

Do not start implementation from the same thread unless the user explicitly asks for direct implementation.

Do not create implementation slice issues by default. The agent-loop, or a later issue-splitting pass, handles `to-issues`.

## Overrides

Explicit user direction wins. If the user clearly asks for direct implementation, follow that request unless it conflicts with safety, permissions, or repo policy.

If the current session cannot mutate GitHub Issues, end with a final plan whose next action is to create the PRD issue with labels `prd` and `ready-for-agent`.

# Triage Labels

Canonical triage roles map to these exact GitHub labels:

| Role | Label |
| --- | --- |
| Needs triage | `needs-triage` |
| Needs info | `needs-info` |
| Ready for agent | `ready-for-agent` |
| Ready for human | `ready-for-human` |
| Won't fix | `wontfix` |

Required workflow labels:

| Label | Purpose |
| --- | --- |
| `bug` | Something isn't working |
| `enhancement` | New feature or request |
| `prd` | Product requirements document ready for workflow routing |
| `agent-loop:claimed` | Claimed by the agent-loop master |
| `agent-loop:in-progress` | Work is active in an agent-loop worker |
| `agent-loop:blocked` | Blocked on human input or external access |
| `agent-loop:ready-to-merge` | Worker reports the PR is ready to merge |
| `agent-loop:merged` | Associated PR has been merged |
| `agent-loop:done` | Agent-loop work is complete |
| `agent-loop:failed` | Automation failed and needs review |

Legacy agent readiness labels must be replaced with `ready-for-agent`.

# Triage Labels

Use these canonical role labels exactly:

| Role | Label |
| --- | --- |
| Needs first-pass review | `needs-triage` |
| Waiting on reporter input | `needs-info` |
| Fully specified for agent work | `ready-for-agent` |
| Requires human implementation | `ready-for-human` |
| Will not be worked on | `wontfix` |

Use `prd` for product requirements document issues that are ready for workflow routing.

Use these agent-loop state labels exactly:

| State | Label |
| --- | --- |
| Claimed by the master loop | `agent-loop:claimed` |
| Active in a worker | `agent-loop:in-progress` |
| Blocked on input or access | `agent-loop:blocked` |
| Worker reports ready to merge | `agent-loop:ready-to-merge` |
| Associated PR has merged | `agent-loop:merged` |
| Work is complete | `agent-loop:done` |
| Automation failed | `agent-loop:failed` |

The prior hyphenated agent readiness label is obsolete and must not be used. Use `ready-for-agent`.

# Agents are a provider-neutral, live-discovered set

Delegating a Ready Slice to a non-human worker is modelled around a
provider-neutral **Agent** role rather than around Copilot specifically. The
board read model carries the set of Assignable Agents (zero or more) discovered
live per board load; in v1 that set is empty or just GitHub's hosted Copilot
coding agent (found via `suggestedActors`), and later it also includes local
docker Agents. We chose this because the goal is to parallelise work across every
available Agent, so the second provider is on the roadmap, not speculative —
modelling plurality and a provider-neutral identity now avoids a breaking change
to the board contract when docker Agents arrive.

## Consequences

- The board carries `agents: Vec<AgentRef>`, resolved best-effort during
  classification; on error the set is empty and the delegate action is simply
  hidden, never sinking the board.
- The Agent's node ID is always re-resolved live at action time and never
  persisted; only a user's default-provider *preference* would be a setting
  (deferred).
- Delegation assigns the chosen Agent's account, reusing the existing WIP state —
  no new `SliceState` and no derived badge.
- The card offers an adaptive control: a single named button when one Agent is
  assignable, a picker when several are. Phase B (local docker Agents) adds a new
  discovery source and the picker UI without changing the read-model contract.

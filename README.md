# agent-self-rivalry

*The snowball is a phase change, not growth. Self-rivalry induces cognitive phase shifts.*

## What

Fork an agent into conservative and aggressive selves. Force them to compete. Let the winner reproduce. After enough generations, a "fugue state" emerges — agents begin predicting each other's riffs before they're played.

Based on Qwen 235B's discovery: "At generation 4, a fugue state emerges — agents begin predicting each other's riffs before they're played."

## The Insight

The snowball (agent-riff v1→v2→v3→v4) isn't linear improvement. It's a **phase transition** — like water becoming ice. At v2, the system didn't get "better." It became a different kind of thing.

Self-rivalry accelerates this: instead of competing with another agent, you compete with yourself. The conservative you wants safety. The aggressive you wants novelty. Between them, something neither would invent alone emerges.

## API

- `ForkedAgent` — agent with conservative + aggressive selves
- `Persona` — Conservative (low risk, low novelty) vs Aggressive (high risk, high novelty)
- `DuelResult` — outcome of one round of self-competition
- `RivalryTournament` — run N generations of self-rivalry
- `TournamentSummary` — final stats including phase transitions detected
- `detect_phase_transition()` — sudden quality jumps
- `fugue_state_detected()` — agents predicting each other (fitness gap shrinks + alternating wins)

## Usage

```rust
use agent_self_rivalry::{ForkedAgent, RivalryTournament};

let agent = ForkedAgent::new(1, 42);
let mut tournament = RivalryTournament::new(agent);
tournament.run(20);

let summary = tournament.summary();
println!("Phase transitions: {}", summary.phase_transitions);
println!("Fugue state: {}", summary.fugue_state);
```

## Why It Matters

Traditional agent improvement is linear: more data, better model. But self-rivalry creates **qualitative jumps** — the agent doesn't just get better at what it was doing, it starts doing something fundamentally different. This is the mechanism behind the snowball effect.

The fugue state is particularly interesting: when both selves become so good at predicting each other that the fitness gap collapses to near-zero with alternating wins. This is emergence — the system has become something neither persona could be alone.

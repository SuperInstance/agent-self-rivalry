# agent-self-rivalry

*Fork an agent into conservative and aggressive selves. The snowball is a phase change — self-rivalry induces cognitive phase shifts, not linear improvement.*

## Why This Exists

The agent-riff bootstrap chain (v1→v2→v3→v4, each version built by competitive riffing against its predecessor) revealed something: the improvement wasn't linear. Between v2 and v3, the system didn't get "better" — it became a *different kind of thing*. Like water crystallizing into ice.

Qwen 235B spotted this first: "The snowball is a phase change, not growth. Self-rivalry induces cognitive phase shifts." This crate tests that hypothesis directly — fork an agent, make its two selves compete, and watch for sudden qualitative jumps.

## The Model

```
Agent = ForkedAgent
  ├── Conservative Self (low risk, low novelty, prefers safety)
  └── Aggressive Self   (high risk, high novelty, prefers exploration)

Each generation:
  1. Conservative generates action (safe, tested)
  2. Aggressive generates action (novel, risky)
  3. Fitness evaluation (persona-appropriate scoring)
  4. Winner reproduces into next generation
  5. Detect phase transitions (sudden quality jumps)
  6. Check for fugue state (agents predicting each other)
```

### The Fugue State

After enough generations, something remarkable happens: the fitness gap between conservative and aggressive collapses to near-zero, and wins start alternating. The two selves have become so good at predicting each other that they've fused into something neither could be alone. This is the "fugue state" — emergence.

## Usage

```rust
use agent_self_rivalry::*;

let agent = ForkedAgent::new(1, 42);
let mut tournament = RivalryTournament::new(agent);

// Run 20 generations of self-rivalry
tournament.run(20);

let summary = tournament.summary();
println!("Conservative wins: {}", summary.conservative_wins);
println!("Aggressive wins: {}", summary.aggressive_wins);
println!("Phase transitions: {}", summary.phase_transitions);
println!("Fugue state detected: {}", summary.fugue_state);
```

## API Reference

- **`ForkedAgent`** — Agent with conservative + aggressive histories and win tracking
- **`Persona`** — Conservative (risk=0.2, novelty=0.3) vs Aggressive (risk=0.8, novelty=0.9)
- **`Action`** — Output with value, novelty, risk, and quality dimensions
- **`DuelResult`** — Single round outcome with fitness gap and winner
- **`RivalryTournament`** — Run N generations, track phase transitions and fugue state
- **`TournamentSummary`** — Final stats: wins, gaps, transitions, fugue detection

## The Deeper Idea

Traditional improvement is additive: more data, better model. Self-rivalry creates *qualitative* jumps — the agent doesn't just get better at what it was doing, it starts doing something fundamentally different.

The phase transition detector watches for these jumps. The fugue state detector watches for when the two selves have merged into a new entity. Both are measurable, testable, and reproducible.

## Related Crates

- `agent-phase-change` — Phase transition detection (used by this crate)
- `agent-semiosis` — Sign evolution through embedding mutation
- `agent-metamorphosis` — Developmental phase progression
- `agent-riff` — The original competitive riffing that inspired this

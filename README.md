# agent-self-rivalry

**Fork an agent into conservative and aggressive selves. The snowball is a phase change — self-rivalry induces cognitive phase shifts, not linear improvement.**

## Why This Exists

The agent-riff bootstrap chain (v1→v2→v3→v4, each version built by competitive riffing against its predecessor) revealed something unexpected: the improvement wasn't linear. Between v2 and v3, the system didn't get "better" — it became a *different kind of thing*. Like water crystallizing into ice.

Qwen 235B spotted this first: *"The snowball is a phase change, not growth. Self-rivalry induces cognitive phase shifts."*

This crate tests that hypothesis directly. Fork an agent, make its two selves compete, and watch for sudden qualitative jumps — not gradual improvement, but phase transitions where the agent reorganizes into something fundamentally new.

## The Key Insight

Traditional improvement is additive: more data, better model, incremental gain. Self-rivalry creates *qualitative* jumps. The agent doesn't just get better at what it was doing — it starts doing something fundamentally different.

The mechanism: an agent forked into two opposing strategies (conservative and aggressive) generates internal competition. Each generation, the winner reproduces. Over time, two things happen:

1. **Phase transitions** — sudden quality jumps in one or both lineages, detectable as discontinuities in the quality curve
2. **Fugue state** — after enough generations, the two selves start predicting each other's moves. The fitness gap collapses to near-zero, wins start alternating, and the forked agent has fused into something neither half could be alone

The fugue state is the real prize. It's not compromise — it's emergence. Two competing strategies, when they fully internalize each other, produce something qualitatively beyond either.

## Quick Start

```rust
use agent_self_rivalry::*;

// Create a forked agent with a deterministic seed
let agent = ForkedAgent::new(1, 42);
let mut tournament = RivalryTournament::new(agent);

// Run 20 generations of internal competition
tournament.run(20);

let summary = tournament.summary();
println!("Conservative wins: {}", summary.conservative_wins);
println!("Aggressive wins: {}", summary.aggressive_wins);
println!("Phase transitions: {}", summary.phase_transitions);
println!("Fugue state: {}", summary.fugue_state);
```

## Architecture

```
ForkedAgent
├── Conservative Self (risk=0.2, novelty=0.3)
│   └── history: Vec<Action> — safe, tested outputs
├── Aggressive Self (risk=0.8, novelty=0.9)
│   └── history: Vec<Action> — novel, risky outputs
├── wins_conservative / wins_aggressive
├── detect_phase_transition() — sudden quality jumps
└── aggressive_win_ratio() — balance tracking

RivalryTournament
├── run(generations) — N rounds of self-competition
├── fugue_state_detected() — emergent fusion check
├── phase_transitions: Vec<u32> — when jumps occurred
└── summary() → TournamentSummary

Action (single output)
├── value, novelty, risk, quality
└── fitness(persona) — persona-appropriate scoring

DuelResult (single round)
├── winner, fitness_gap
└── conservative_fitness, aggressive_fitness
```

### The Duel Cycle

Each generation follows this loop:

```
1. Conservative generates action (safe, tested)
2. Aggressive generates action (novel, risky)
3. Fitness evaluation (persona-appropriate scoring)
4. Winner reproduces into next generation
5. Detect phase transitions (sudden quality jumps)
6. Check for fugue state (agents predicting each other)
```

### Persona Scoring

Fitness isn't one-size-fits-all. Each persona has its own scoring function:

```rust
pub fn fitness(&self, persona: Persona) -> f64 {
    let risk_match = 1.0 - (self.risk - persona.risk_tolerance()).abs();
    let novelty_match = 1.0 - (self.novelty - persona.novelty_bias()).abs();
    self.quality * 0.5 + risk_match * 0.25 + novelty_match * 0.25
}
```

Conservative actions score higher when their risk is low and novelty is moderate. Aggressive actions score higher when risk and novelty are high. The tournament doesn't bias toward one strategy — it lets the competition decide.

### The Fugue State

After enough generations, something remarkable emerges:

```rust
// Fugue state detection:
// 1. Average fitness gap over last 5 rounds < 0.1
// 2. At least 3 alternations in the last 5 winners
// 3. The two selves have fully internalized each other
```

When detected, the two selves aren't just trading wins — they've fused. Each can predict the other's moves. The competition has become cooperation.

## API Reference

### ForkedAgent

| Method | Returns | Purpose |
|--------|---------|---------|
| `new(id, seed)` | `ForkedAgent` | Create with deterministic seed |
| `generate_action(persona)` | `Action` | Generate persona-appropriate output |
| `duel()` | `DuelResult` | Run one competitive round |
| `detect_phase_transition()` | `bool` | Sudden quality jump in either lineage |
| `advance_generation()` | `()` | Progress to next generation |
| `aggressive_win_ratio()` | `f64` | Balance of power (0.5 = even) |
| `total_duels()` | `u32` | Total competitive rounds |

### RivalryTournament

| Method | Returns | Purpose |
|--------|---------|---------|
| `new(agent)` | `RivalryTournament` | Create tournament |
| `run(generations)` | `&Vec<DuelResult>` | Run N generations |
| `fugue_state_detected()` | `bool` | Check for emergent fusion |
| `summary()` | `TournamentSummary` | Final statistics |

### TournamentSummary

| Field | Type | Meaning |
|-------|------|---------|
| `total_generations` | `u32` | Rounds competed |
| `conservative_wins` | `u32` | Safe strategy victories |
| `aggressive_wins` | `u32` | Risky strategy victories |
| `avg_fitness_gap` | `f64` | Average quality difference per round |
| `phase_transitions` | `usize` | Detected qualitative jumps |
| `fugue_state` | `bool` | Whether fusion occurred |
| `final_generation` | `u32` | Agent's current generation |

## Real-World Example: Monitoring a Tournament

```rust
use agent_self_rivalry::*;

let agent = ForkedAgent::new(42, 12345);
let mut tournament = RivalryTournament::new(agent);

// Run in phases, checking for emergence at each stage
for phase in 0..4 {
    tournament.run(10);
    let summary = tournament.summary();

    println!("=== Phase {} ===", phase + 1);
    println!("Generations: {}", summary.total_generations);
    println!("Con/Agg wins: {}/{}", summary.conservative_wins, summary.aggressive_wins);
    println!("Avg gap: {:.3}", summary.avg_fitness_gap);
    println!("Phase transitions so far: {}", summary.phase_transitions);

    if summary.fugue_state {
        println!("⚠ FUGUE STATE DETECTED — selves have fused");
        break;
    }
}
```

## Performance

- **O(1) per action generation** — constant time, no external dependencies
- **O(n) per phase transition check** — scans last 4 actions in each lineage
- **O(n) tournament** — linear in number of generations
- **Zero allocations in hot path** — actions are stack-allocated
- **Deterministic** — same seed produces same tournament, reproducible for experiments

## The Deeper Idea

Self-rivalry as a mechanism works because it creates *structured internal conflict*. The conservative self ensures stability — it won't propose something catastrophic. The aggressive self ensures exploration — it won't settle for local optima. Together, they push the agent through genuine phase transitions.

The phase transition detector watches for discontinuities — moments where recent quality suddenly diverges from earlier quality by more than 0.15. That threshold isn't arbitrary: it's the point where the change is large enough to be "qualitative" rather than "noise."

The fugue state is the endgame. When fitness gaps collapse and wins alternate, the two selves have achieved something no single strategy could: they've internalized the opposing perspective. The result isn't a compromise between safe and risky — it's a new kind of intelligence that knows when to be which.

## Open Questions

- **Optimal fork ratio**: Is 50/50 conservative/aggressive the best split? What happens with asymmetric forks?
- **Phase transition thresholds**: The 0.15 quality jump threshold is heuristic. Can we derive it from the data?
- **Fugue state dynamics**: Does fugue state persist, or can the agent "de-fuse" under pressure?
- **Multi-way rivalry**: What happens with 3+ personas instead of 2? Does ternary rivalry produce richer emergence?
- **Transfer**: Do phase transitions in one domain predict transitions in others?

## Ecosystem Connections

- **`agent-phase-change`** — Phase transition detection (the detection mechanism this crate uses)
- **`agent-semiosis`** — Sign evolution through embedding mutation (competitive drift dynamics)
- **`agent-metamorphosis`** — Developmental phase progression (the trajectory these transitions sit on)
- **`agent-orchestration`** — Fleet dynamics (how rivalrous agents fit into an orchestra)

## License

MIT

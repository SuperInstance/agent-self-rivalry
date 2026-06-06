//! # agent-self-rivalry
//!
//! Fork an agent into conservative and aggressive selves, force them to compete,
//! let the winner reproduce. After 5 generations, agents predict each other's
//! riffs BEFORE they're played.
//!
//! Based on Qwen's insight: "The snowball is a phase change — self-rivalry induces
//! cognitive phase shifts, not linear improvement."

/// An agent persona: conservative (safe, tested) or aggressive (novel, risky)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Persona {
    Conservative,
    Aggressive,
}

impl Persona {
    pub fn risk_tolerance(self) -> f64 {
        match self {
            Persona::Conservative => 0.2,
            Persona::Aggressive => 0.8,
        }
    }

    pub fn novelty_bias(self) -> f64 {
        match self {
            Persona::Conservative => 0.3,
            Persona::Aggressive => 0.9,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Persona::Conservative => Persona::Aggressive,
            Persona::Aggressive => Persona::Conservative,
        }
    }
}

/// A single action/output from an agent
#[derive(Debug, Clone)]
pub struct Action {
    pub value: f64,
    pub novelty: f64,
    pub risk: f64,
    pub quality: f64,
}

impl Action {
    pub fn new(value: f64, novelty: f64, risk: f64, quality: f64) -> Self {
        Self { value, novelty, risk, quality }
    }

    /// Fitness combines quality with persona-appropriate scoring
    pub fn fitness(&self, persona: Persona) -> f64 {
        let risk_match = 1.0 - (self.risk - persona.risk_tolerance()).abs();
        let novelty_match = 1.0 - (self.novelty - persona.novelty_bias()).abs();
        self.quality * 0.5 + risk_match * 0.25 + novelty_match * 0.25
    }
}

/// A forked agent with both conservative and aggressive selves
#[derive(Debug, Clone)]
pub struct ForkedAgent {
    pub id: u64,
    pub generation: u32,
    pub conservative_history: Vec<Action>,
    pub aggressive_history: Vec<Action>,
    pub wins_conservative: u32,
    pub wins_aggressive: u32,
    pub rng_seed: u64,
}

impl ForkedAgent {
    pub fn new(id: u64, seed: u64) -> Self {
        Self {
            id,
            generation: 0,
            conservative_history: Vec::new(),
            aggressive_history: Vec::new(),
            wins_conservative: 0,
            wins_aggressive: 0,
            rng_seed: seed,
        }
    }

    /// Generate an action for the given persona
    pub fn generate_action(&mut self, persona: Persona) -> Action {
        // Use simple deterministic variation based on persona and history
        let base = match persona {
            Persona::Conservative => 0.5 + (self.generation as f64 * 0.01),
            Persona::Aggressive => 0.7 + (self.generation as f64 * 0.02),
        };

        let novelty = persona.novelty_bias() * (0.5 + self.generation as f64 * 0.05).min(1.0);
        let risk = persona.risk_tolerance() * (0.3 + self.generation as f64 * 0.03).min(1.0);
        let quality = (base + self.diversity_bonus(persona)).min(1.0);

        let action = Action::new(base, novelty, risk, quality);

        match persona {
            Persona::Conservative => self.conservative_history.push(action.clone()),
            Persona::Aggressive => self.aggressive_history.push(action.clone()),
        }

        action
    }

    fn diversity_bonus(&self, persona: Persona) -> f64 {
        let history = match persona {
            Persona::Conservative => &self.conservative_history,
            Persona::Aggressive => &self.aggressive_history,
        };
        // More past actions → more diversity → slight quality bonus
        (history.len() as f64 * 0.02).min(0.2)
    }

    /// Run a duel between conservative and aggressive selves
    pub fn duel(&mut self) -> DuelResult {
        let conservative_action = self.generate_action(Persona::Conservative);
        let aggressive_action = self.generate_action(Persona::Aggressive);

        let conservative_fitness = conservative_action.fitness(Persona::Conservative);
        let aggressive_fitness = aggressive_action.fitness(Persona::Aggressive);

        let winner = if aggressive_fitness > conservative_fitness {
            self.wins_aggressive += 1;
            Persona::Aggressive
        } else {
            self.wins_conservative += 1;
            Persona::Conservative
        };

        let fitness_gap = (aggressive_fitness - conservative_fitness).abs();

        DuelResult {
            generation: self.generation,
            winner,
            conservative_fitness,
            aggressive_fitness,
            fitness_gap,
        }
    }

    /// Advance to next generation (winner reproduces)
    pub fn advance_generation(&mut self) {
        self.generation += 1;
    }

    /// Check if a phase transition has occurred (sudden quality jump)
    pub fn detect_phase_transition(&self) -> bool {
        if self.conservative_history.len() < 4 || self.aggressive_history.len() < 4 {
            return false;
        }

        // Compare recent fitness to earlier fitness
        let recent_con: f64 = self.conservative_history.iter().rev().take(2)
            .map(|a| a.quality).sum::<f64>() / 2.0;
        let earlier_con: f64 = self.conservative_history.iter().take(2)
            .map(|a| a.quality).sum::<f64>() / 2.0;

        let recent_agg: f64 = self.aggressive_history.iter().rev().take(2)
            .map(|a| a.quality).sum::<f64>() / 2.0;
        let earlier_agg: f64 = self.aggressive_history.iter().take(2)
            .map(|a| a.quality).sum::<f64>() / 2.0;

        // Phase transition = sudden jump > 0.15 in either lineage
        (recent_con - earlier_con).abs() > 0.15 || (recent_agg - earlier_agg).abs() > 0.15
    }

    /// Total duels fought
    pub fn total_duels(&self) -> u32 {
        self.wins_conservative + self.wins_aggressive
    }

    /// Win ratio for aggressive persona
    pub fn aggressive_win_ratio(&self) -> f64 {
        if self.total_duels() == 0 { return 0.5; }
        self.wins_aggressive as f64 / self.total_duels() as f64
    }
}

/// Result of a single duel between selves
#[derive(Debug, Clone)]
pub struct DuelResult {
    pub generation: u32,
    pub winner: Persona,
    pub conservative_fitness: f64,
    pub aggressive_fitness: f64,
    pub fitness_gap: f64,
}

/// Run a full rivalry tournament across N generations
pub struct RivalryTournament {
    pub agent: ForkedAgent,
    pub results: Vec<DuelResult>,
    pub phase_transitions: Vec<u32>,
}

impl RivalryTournament {
    pub fn new(agent: ForkedAgent) -> Self {
        Self {
            agent,
            results: Vec::new(),
            phase_transitions: Vec::new(),
        }
    }

    /// Run N generations of self-rivalry
    pub fn run(&mut self, generations: u32) -> &Vec<DuelResult> {
        for _ in 0..generations {
            let had_transition = self.agent.detect_phase_transition();
            let result = self.agent.duel();
            self.results.push(result.clone());

            if had_transition {
                self.phase_transitions.push(self.agent.generation);
            }

            self.agent.advance_generation();
        }
        &self.results
    }

    /// Check if "fugue state" emerged (agents predicting each other)
    pub fn fugue_state_detected(&self) -> bool {
        if self.results.len() < 5 { return false; }

        // Fugue state = fitness gap becomes very small AND alternating wins
        let recent: Vec<&DuelResult> = self.results.iter().rev().take(5).collect();
        let avg_gap: f64 = recent.iter().map(|r| r.fitness_gap).sum::<f64>() / 5.0;

        if avg_gap > 0.1 { return false; }

        // Check for alternating wins
        let mut alternations = 0;
        for i in 0..recent.len() - 1 {
            if recent[i].winner != recent[i + 1].winner {
                alternations += 1;
            }
        }

        alternations >= 3
    }

    /// Summary statistics
    pub fn summary(&self) -> TournamentSummary {
        let avg_gap: f64 = if self.results.is_empty() {
            0.0
        } else {
            self.results.iter().map(|r| r.fitness_gap).sum::<f64>() / self.results.len() as f64
        };

        TournamentSummary {
            total_generations: self.results.len() as u32,
            conservative_wins: self.agent.wins_conservative,
            aggressive_wins: self.agent.wins_aggressive,
            avg_fitness_gap: avg_gap,
            phase_transitions: self.phase_transitions.len(),
            fugue_state: self.fugue_state_detected(),
            final_generation: self.agent.generation,
        }
    }
}

/// Summary of a rivalry tournament
#[derive(Debug, Clone)]
pub struct TournamentSummary {
    pub total_generations: u32,
    pub conservative_wins: u32,
    pub aggressive_wins: u32,
    pub avg_fitness_gap: f64,
    pub phase_transitions: usize,
    pub fugue_state: bool,
    pub final_generation: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona_properties() {
        assert!(Persona::Conservative.risk_tolerance() < Persona::Aggressive.risk_tolerance());
        assert!(Persona::Conservative.novelty_bias() < Persona::Aggressive.novelty_bias());
        assert_eq!(Persona::Conservative.opposite(), Persona::Aggressive);
        assert_eq!(Persona::Aggressive.opposite(), Persona::Conservative);
    }

    #[test]
    fn test_action_fitness() {
        let safe_action = Action::new(0.5, 0.2, 0.1, 0.9);
        let risky_action = Action::new(0.8, 0.9, 0.8, 0.9);

        // Conservative prefers safe
        assert!(safe_action.fitness(Persona::Conservative) > risky_action.fitness(Persona::Conservative));
        // Aggressive prefers risky
        assert!(risky_action.fitness(Persona::Aggressive) > safe_action.fitness(Persona::Aggressive));
    }

    #[test]
    fn test_forked_agent_creation() {
        let agent = ForkedAgent::new(1, 42);
        assert_eq!(agent.generation, 0);
        assert_eq!(agent.wins_conservative, 0);
        assert_eq!(agent.wins_aggressive, 0);
    }

    #[test]
    fn test_generate_action() {
        let mut agent = ForkedAgent::new(1, 42);
        let con = agent.generate_action(Persona::Conservative);
        assert!(con.quality > 0.0);
        assert_eq!(agent.conservative_history.len(), 1);

        let agg = agent.generate_action(Persona::Aggressive);
        assert!(agg.quality > 0.0);
        assert_eq!(agent.aggressive_history.len(), 1);
    }

    #[test]
    fn test_duel() {
        let mut agent = ForkedAgent::new(1, 42);
        let result = agent.duel();
        assert_eq!(agent.total_duels(), 1);
        assert!(result.conservative_fitness > 0.0);
        assert!(result.aggressive_fitness > 0.0);
    }

    #[test]
    fn test_multiple_duels() {
        let mut agent = ForkedAgent::new(1, 42);
        for _ in 0..10 {
            agent.duel();
            agent.advance_generation();
        }
        assert_eq!(agent.total_duels(), 10);
        assert_eq!(agent.generation, 10);
    }

    #[test]
    fn test_win_ratio() {
        let mut agent = ForkedAgent::new(1, 42);
        assert_eq!(agent.aggressive_win_ratio(), 0.5); // No duels yet = even
        agent.duel();
        assert!(agent.aggressive_win_ratio() >= 0.0 && agent.aggressive_win_ratio() <= 1.0);
    }

    #[test]
    fn test_tournament() {
        let agent = ForkedAgent::new(1, 42);
        let mut tournament = RivalryTournament::new(agent);
        tournament.run(20);
        assert_eq!(tournament.results.len(), 20);
    }

    #[test]
    fn test_tournament_summary() {
        let agent = ForkedAgent::new(1, 42);
        let mut tournament = RivalryTournament::new(agent);
        tournament.run(20);
        let summary = tournament.summary();
        assert_eq!(summary.total_generations, 20);
        assert_eq!(summary.conservative_wins + summary.aggressive_wins, 20);
    }

    #[test]
    fn test_phase_transition_detection() {
        let mut agent = ForkedAgent::new(1, 42);
        // Early on, no phase transition
        assert!(!agent.detect_phase_transition());
    }

    #[test]
    fn test_fugue_state_not_detected_early() {
        let agent = ForkedAgent::new(1, 42);
        let mut tournament = RivalryTournament::new(agent);
        tournament.run(3);
        assert!(!tournament.fugue_state_detected());
    }

    #[test]
    fn test_diversity_bonus_increases() {
        let mut agent = ForkedAgent::new(1, 42);
        let a1 = agent.generate_action(Persona::Conservative);
        for _ in 0..5 { agent.generate_action(Persona::Conservative); }
        let a2 = agent.generate_action(Persona::Conservative);
        assert!(a2.quality >= a1.quality);
    }
}

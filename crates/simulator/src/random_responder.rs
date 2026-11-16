//! A responder that picks options at random (stand-in until the AI of J4).

use engine::{Decision, GameState, Responder, Rng};

/// Picks a random option, from its own seeded generator.
pub struct RandomResponder {
    rng: Rng,
}

impl RandomResponder {
    /// A responder seeded for reproducible runs.
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Rng::new(seed),
        }
    }
}

impl Responder for RandomResponder {
    fn answer(&mut self, _state: &GameState, decision: &Decision) -> Option<usize> {
        Some(self.rng.below(decision.options.len()))
    }
}

//! The human plays one seat, the AI plays the others.

use engine::{Decision, Responder, View};

use crate::policy::Policy;

/// Leaves the human's decisions open and lets the AI answer the others.
pub struct Seated {
    /// Seat of the human player.
    human: usize,
    /// The AI.
    policy: Policy,
}

impl Seated {
    /// The human at `human`, the AI elsewhere.
    #[must_use]
    pub fn new(human: usize) -> Self {
        Self {
            human,
            policy: Policy::new(),
        }
    }
}

impl Responder for Seated {
    fn answer(&mut self, view: &View<'_, '_>, decision: &Decision) -> Option<usize> {
        if decision.who == self.human {
            return None;
        }
        self.policy.answer(view, decision)
    }
}

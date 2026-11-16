//! Game setup: the choices made before the first turn.

use data::{AncientOneId, InvestigatorId};
use serde::{Deserialize, Serialize};

/// How to start a game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Setup {
    /// Seed of the random generator.
    pub seed: u64,
    /// Number of investigators, 1 to 8.
    pub players: usize,
    /// Investigators chosen; the other seats are drawn at random.
    pub investigators: Vec<InvestigatorId>,
    /// Ancient One chosen, or `None` to draw one.
    pub ancient_one: Option<AncientOneId>,
}

impl Setup {
    /// A game with random investigators and Ancient One.
    pub fn random(seed: u64, players: usize) -> Self {
        Self {
            seed,
            players,
            investigators: Vec::new(),
            ancient_one: None,
        }
    }
}

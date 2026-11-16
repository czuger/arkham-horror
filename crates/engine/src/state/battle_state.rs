//! The final battle.

use serde::{Deserialize, Serialize};

/// Progress of the final battle.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleState {
    /// Round number, from 1.
    pub round: u32,
    /// Successes kept for the next doom token.
    pub successes: u32,
}

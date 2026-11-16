//! The Rumor in play.

use data::CardId;
use serde::{Deserialize, Serialize};

/// The Rumor card in play.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveRumor {
    /// Which card.
    pub card: CardId,
    /// Tokens on the card.
    pub tokens: u8,
    /// Came into play this turn: its end-of-Mythos effect starts next turn.
    pub fresh: bool,
}

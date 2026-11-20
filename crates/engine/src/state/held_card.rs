//! A card in an investigator's hand.

use data::CardId;
use serde::{Deserialize, Serialize};

/// A card an investigator holds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeldCard {
    /// Which card.
    pub card: CardId,
    /// Turned face down after use, until the next upkeep.
    pub exhausted: bool,
    /// Acquired since the last upkeep: no upkeep roll at the next one.
    pub fresh: bool,
    /// Tokens put on the card by its own effects.
    #[serde(default)]
    pub tokens: u8,
}

impl HeldCard {
    /// A card just received.
    pub fn new(card: CardId) -> Self {
        Self {
            card,
            exhausted: false,
            fresh: true,
            tokens: 0,
        }
    }
}

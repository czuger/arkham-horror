//! Conditions on an investigator, used by cards.

use serde::{Deserialize, Serialize};

use crate::deck::Deck;
use crate::ids::CardId;

/// A yes/no question about one investigator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Condition {
    /// Holds this card (Membre de la Loge, Bénédiction…).
    HasCard(CardId),
    /// Holds at most `count` cards of this deck.
    CardCountAtMost {
        /// The deck counted.
        deck: Deck,
        /// Largest count for which the condition holds.
        count: u8,
    },
    /// Stands in a street area of Arkham.
    InStreets,
    /// Stands in an Other World.
    InOtherWorld,
    /// Is lost in time and space.
    LostInTimeAndSpace,
}

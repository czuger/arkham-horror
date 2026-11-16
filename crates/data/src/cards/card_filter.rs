//! Which of an investigator's cards an effect applies to.

use serde::{Deserialize, Serialize};

use crate::deck::Deck;

/// A family of cards in an investigator's possession.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardFilter {
    /// Objets: Common and Unique Items together.
    Items,
    /// Cards from one deck.
    Deck(Deck),
}

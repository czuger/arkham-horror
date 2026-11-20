//! Which of an investigator's cards an effect applies to.

use serde::{Deserialize, Serialize};

use crate::deck::Deck;

/// A family of cards in an investigator's possession, or in a deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardFilter {
    /// Objets: Common and Unique Items, Spells and the items of the Deputy
    /// (FAQ).
    Items,
    /// Armes : items with a weapon bonus.
    Weapons,
    /// Tomes : items whose subtitle is « Tome ».
    Tomes,
    /// Cards from one deck.
    Deck(Deck),
}

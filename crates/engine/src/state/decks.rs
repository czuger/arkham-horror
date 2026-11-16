//! The investigator card decks.

use data::{CardId, Deck};
use serde::{Deserialize, Serialize};

/// The six investigator decks. Index 0 is the top of a deck.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decks {
    /// Common items.
    pub common_items: Vec<CardId>,
    /// Unique items.
    pub unique_items: Vec<CardId>,
    /// Spells.
    pub spells: Vec<CardId>,
    /// Skills.
    pub skills: Vec<CardId>,
    /// Allies.
    pub allies: Vec<CardId>,
    /// Special cards (Blessing, Bank Loan…).
    pub specials: Vec<CardId>,
}

impl Decks {
    /// The deck of a kind.
    pub fn get(&self, deck: Deck) -> &Vec<CardId> {
        match deck {
            Deck::CommonItem => &self.common_items,
            Deck::UniqueItem => &self.unique_items,
            Deck::Spell => &self.spells,
            Deck::Skill => &self.skills,
            Deck::Ally => &self.allies,
            Deck::Special => &self.specials,
        }
    }

    /// The deck of a kind, to change it.
    pub fn get_mut(&mut self, deck: Deck) -> &mut Vec<CardId> {
        match deck {
            Deck::CommonItem => &mut self.common_items,
            Deck::UniqueItem => &mut self.unique_items,
            Deck::Spell => &mut self.spells,
            Deck::Skill => &mut self.skills,
            Deck::Ally => &mut self.allies,
            Deck::Special => &mut self.specials,
        }
    }
}

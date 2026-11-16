//! The investigator card decks.

use serde::{Deserialize, Serialize};

/// A deck investigators draw from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Deck {
    /// Objets Communs.
    CommonItem,
    /// Objets Uniques.
    UniqueItem,
    /// Sorts.
    Spell,
    /// Compétences.
    Skill,
    /// Alliés.
    Ally,
    /// Cartes spéciales (Acompte, Bénédiction…).
    Special,
}

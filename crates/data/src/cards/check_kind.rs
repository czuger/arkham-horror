//! The kinds of dice tests a card can call for or modify.

use serde::{Deserialize, Serialize};

/// A skill check (one of the six skills) or a derived check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheckKind {
    /// Vitesse.
    Speed,
    /// Discrétion.
    Sneak,
    /// Vigueur (« Combat » in the first French edition and on some cards).
    Fight,
    /// Volonté.
    Will,
    /// Savoir.
    Lore,
    /// Chance.
    Luck,
    /// Test d'Horreur (Volonté against a monster).
    Horror,
    /// Test de Combat (Vigueur plus weapons against a monster).
    Combat,
    /// Test d'Évasion (Discrétion against a monster).
    Evade,
    /// Test de Sort (Savoir, to cast a spell).
    Spell,
}

impl CheckKind {
    /// Whether this is one of the six skills printed on investigator sheets.
    pub fn is_skill(self) -> bool {
        matches!(
            self,
            Self::Speed | Self::Sneak | Self::Fight | Self::Will | Self::Lore | Self::Luck
        )
    }
}

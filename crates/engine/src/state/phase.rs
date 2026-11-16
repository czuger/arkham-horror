//! Phases of a turn.

use serde::{Deserialize, Serialize};

/// Where the game stands in the turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    /// Setting up, before the first turn.
    Setup,
    /// I. Upkeep.
    Upkeep,
    /// II. Movement.
    Movement,
    /// III. Arkham encounters.
    ArkhamEncounters,
    /// IV. Other World encounters.
    OtherWorldEncounters,
    /// V. Mythos.
    Mythos,
    /// The Ancient One is awake.
    FinalBattle,
}

impl Phase {
    /// French name shown to the player.
    pub fn label(self) -> &'static str {
        match self {
            Self::Setup => "Préparation",
            Self::Upkeep => "Entretien",
            Self::Movement => "Mouvement",
            Self::ArkhamEncounters => "Rencontres à Arkham",
            Self::OtherWorldEncounters => "Rencontres dans les Autres Mondes",
            Self::Mythos => "Mythe",
            Self::FinalBattle => "Bataille finale",
        }
    }
}

//! How a game ends.

use serde::{Deserialize, Serialize};

/// The end of a game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ending {
    /// The last gate was closed with enough gate trophies.
    GatesClosed,
    /// Six Elder Signs are on the board.
    GatesSealed,
    /// The Ancient One was defeated in the final battle.
    AncientOneBanished,
    /// The Ancient One won.
    Devoured,
}

impl Ending {
    /// Whether the investigators won.
    pub fn is_victory(self) -> bool {
        !matches!(self, Self::Devoured)
    }

    /// French sentence shown to the player.
    pub fn label(self) -> &'static str {
        match self {
            Self::GatesClosed => "Victoire : tous les portails sont fermés.",
            Self::GatesSealed => "Victoire : six Signes des Anciens scellent la ville.",
            Self::AncientOneBanished => "Victoire : le Grand Ancien est banni.",
            Self::Devoured => "Défaite : le Grand Ancien a dévoré le monde.",
        }
    }
}

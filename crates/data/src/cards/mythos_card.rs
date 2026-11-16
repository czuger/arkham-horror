//! Mythos cards.

use serde::{Deserialize, Serialize};

use crate::cards::effect::Effect;
use crate::cards::ongoing::Ongoing;
use crate::cards::rumor::Rumor;
use crate::dimension_symbol::DimensionSymbol;
use crate::ids::{CardId, LocationId};

/// Sub-type of an Environnement card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnvironmentKind {
    /// Mystique.
    Mystic,
    /// Temps (weather).
    Weather,
    /// Urbain.
    Urban,
}

/// The genre printed under the card name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MythosKind {
    /// Gros Titre: immediate effect.
    Headline,
    /// Environnement: stays in play until replaced.
    Environment(EnvironmentKind),
    /// Rumeur: stays in play until passed or failed.
    Rumor,
}

/// A Mythos card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MythosCard {
    /// Identifier (`"planetary_alignment"`).
    pub id: CardId,
    /// Name shown to the player, in French.
    pub name: String,
    /// Genre.
    pub kind: MythosKind,
    /// Printed text, in French.
    pub text: String,
    /// « Un Indice apparaît à ».
    #[serde(default)]
    pub clue: Option<LocationId>,
    /// Location where a gate opens (picture at the bottom left).
    #[serde(default)]
    pub gate: Option<LocationId>,
    /// Monsters with these symbols move along the white arrows (upper box).
    #[serde(default)]
    pub white_moves: Vec<DimensionSymbol>,
    /// Monsters with these symbols move along the black arrows (lower box).
    #[serde(default)]
    pub black_moves: Vec<DimensionSymbol>,
    /// Immediate effects (Gros Titre).
    #[serde(default)]
    pub effects: Vec<Effect>,
    /// Lasting effects while in play (Environnement, Rumeur).
    #[serde(default)]
    pub ongoing: Vec<Ongoing>,
    /// Rumor parts, for a Rumeur.
    #[serde(default)]
    pub rumor: Option<Rumor>,
    /// Card image, relative to the `assets/` folder.
    pub image: String,
}

/// Content of `data/mythos_cards.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MythosCards {
    /// Format version of the file.
    pub version: u32,
    /// The cards.
    pub cards: Vec<MythosCard>,
}

impl MythosCards {
    /// Format version this code reads.
    pub const VERSION: u32 = 1;
}

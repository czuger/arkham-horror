//! Gate cards: encounters in the Other Worlds.

use serde::{Deserialize, Serialize};

use crate::cards::effect::Effect;
use crate::ids::{CardId, OtherWorldId};
use crate::other_world::EncounterColor;

/// One encounter printed on a gate card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GateEncounter {
    /// The Other World; absent for the « Autre » entry, used everywhere else.
    #[serde(default)]
    pub world: Option<OtherWorldId>,
    /// Printed text, in French.
    pub text: String,
    /// What happens.
    #[serde(default)]
    pub effects: Vec<Effect>,
}

/// A gate card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GateCard {
    /// Identifier (the card code, `"cp003"`).
    pub id: CardId,
    /// Border colour, matched against the colours of the Other World.
    pub color: EncounterColor,
    /// Encounters, the « Autre » one last.
    pub encounters: Vec<GateEncounter>,
    /// Card image, relative to the `assets/` folder.
    pub image: String,
}

/// Content of `data/gate_cards.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GateCards {
    /// Format version of the file.
    pub version: u32,
    /// The cards.
    pub cards: Vec<GateCard>,
}

impl GateCards {
    /// Format version this code reads.
    pub const VERSION: u32 = 1;
}

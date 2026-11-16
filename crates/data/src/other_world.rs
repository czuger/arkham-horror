//! Other Worlds.

use serde::{Deserialize, Serialize};

use crate::ids::OtherWorldId;
use crate::position::Position;

/// Colour of a Gate card encounter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EncounterColor {
    /// Red.
    Red,
    /// Yellow.
    Yellow,
    /// Green.
    Green,
    /// Blue.
    Blue,
}

/// One of the 8 Other Worlds of the base game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OtherWorld {
    /// Identifier used by the other data files.
    pub id: OtherWorldId,
    /// Name shown to the player, in French.
    pub name: String,
    /// Gate card colours that trigger an encounter in this world.
    pub colors: Vec<EncounterColor>,
    /// Display position of the first area on the board image.
    pub first_area: Position,
    /// Display position of the second area on the board image.
    pub second_area: Position,
}

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

/// One place of an Other World as the scene shows it: a picture, and the two
/// places it leads to, so that a visit walks from one to a neighbour.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldScene {
    /// Identifier, unique within its world.
    pub id: String,
    /// Picture by day (or the only one), relative to `illustrations/`.
    pub image: String,
    /// Picture by night, relative to `illustrations/`, if there is one.
    #[serde(default)]
    pub night: Option<String>,
    /// The two places of the same world this one leads to (maybe the same).
    pub leads_to: [String; 2],
}

impl WorldScene {
    /// The picture to show, by night if asked and there is one.
    pub fn picture(&self, by_night: bool) -> &str {
        match &self.night {
            Some(night) if by_night => night,
            _ => &self.image,
        }
    }

    /// Every picture of the place, day then night.
    pub fn pictures(&self) -> impl Iterator<Item = &str> {
        std::iter::once(self.image.as_str()).chain(self.night.as_deref())
    }
}

/// One of the 8 Other Worlds of the base game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OtherWorld {
    /// Identifier used by the other data files.
    pub id: OtherWorldId,
    /// Name shown to the player, in French.
    pub name: String,
    /// Name shortened for the symbolic map, in French.
    pub short_name: String,
    /// The places of the world, walked through by the scene.
    pub scenes: Vec<WorldScene>,
    /// Gate card colours that trigger an encounter in this world.
    pub colors: Vec<EncounterColor>,
    /// Display position of the first area on the board image.
    pub first_area: Position,
    /// Display position of the second area on the board image.
    pub second_area: Position,
}

impl OtherWorld {
    /// Finds one of the world's places by id.
    pub fn scene(&self, id: &str) -> Option<&WorldScene> {
        self.scenes.iter().find(|scene| scene.id == id)
    }
}

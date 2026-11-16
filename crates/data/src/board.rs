//! The Arkham board as a graph.

use serde::{Deserialize, Serialize};

use crate::ids::{LocationId, OtherWorldId, StreetId};
use crate::location::Location;
use crate::other_world::OtherWorld;
use crate::special_area::{SpecialArea, SpecialAreaKind};
use crate::street::Street;

/// Content of `data/board.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Board {
    /// Format version of the file.
    pub version: u32,
    /// Board image, relative to the `assets/` folder.
    pub image: String,
    /// The street areas.
    pub streets: Vec<Street>,
    /// The locations.
    pub locations: Vec<Location>,
    /// The Other Worlds.
    pub other_worlds: Vec<OtherWorld>,
    /// Outskirts, Sky, Lost in Time and Space, Jail Cell.
    pub special_areas: Vec<SpecialArea>,
}

impl Board {
    /// Format version this code reads.
    pub const VERSION: u32 = 1;

    /// Finds a location by id.
    pub fn location(&self, id: &LocationId) -> Option<&Location> {
        self.locations.iter().find(|location| &location.id == id)
    }

    /// Finds a street area by id.
    pub fn street(&self, id: &StreetId) -> Option<&Street> {
        self.streets.iter().find(|street| &street.id == id)
    }

    /// Finds an Other World by id.
    pub fn other_world(&self, id: &OtherWorldId) -> Option<&OtherWorld> {
        self.other_worlds.iter().find(|world| &world.id == id)
    }

    /// Finds a special area by kind.
    pub fn special_area(&self, kind: SpecialAreaKind) -> Option<&SpecialArea> {
        self.special_areas.iter().find(|area| area.kind == kind)
    }

    /// Lists the locations of a street area's district.
    pub fn locations_of(&self, street: &StreetId) -> impl Iterator<Item = &Location> {
        self.locations
            .iter()
            .filter(move |location| &location.street == street)
    }
}

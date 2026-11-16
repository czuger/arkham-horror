//! Where an investigator or a monster stands.

use data::{LocationId, OtherWorldId, StreetId};
use serde::{Deserialize, Serialize};

/// Which half of an Other World.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Area {
    /// Left half, where investigators arrive.
    First,
    /// Right half, from which they go back to Arkham.
    Second,
}

/// Where an investigator stands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Place {
    /// An Arkham location.
    Location(LocationId),
    /// A street area.
    Street(StreetId),
    /// An Other World.
    OtherWorld {
        /// The world.
        world: OtherWorldId,
        /// Which half.
        area: Area,
    },
    /// Lost in time and space.
    LostInTimeAndSpace,
    /// The jail cell next to the Police Station.
    JailCell,
}

impl Place {
    /// Whether the place is a location or a street of Arkham.
    pub fn in_arkham(&self) -> bool {
        matches!(self, Self::Location(_) | Self::Street(_))
    }

    /// The location, if the place is one.
    pub fn location(&self) -> Option<&LocationId> {
        match self {
            Self::Location(id) => Some(id),
            _ => None,
        }
    }
}

/// Where a monster stands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonsterPlace {
    /// An Arkham location.
    Location(LocationId),
    /// A street area.
    Street(StreetId),
    /// The Sky, linked to every street.
    Sky,
    /// The Outskirts, where monsters wait when the limit is reached.
    Outskirts,
}

impl MonsterPlace {
    /// The investigator place that is the same area, if any.
    pub fn as_place(&self) -> Option<Place> {
        match self {
            Self::Location(id) => Some(Place::Location(id.clone())),
            Self::Street(id) => Some(Place::Street(id.clone())),
            Self::Sky | Self::Outskirts => None,
        }
    }

    /// Whether the monster counts towards the monster limit.
    pub fn counts_for_limit(&self) -> bool {
        !matches!(self, Self::Outskirts)
    }
}

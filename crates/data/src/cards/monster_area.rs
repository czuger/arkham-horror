//! Parts of the board whose monsters a card sends back to the cup.

use serde::{Deserialize, Serialize};

use crate::ids::StreetId;

/// Where the monsters affected by a card stand.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonsterArea {
    /// Every location of Arkham (not the streets).
    Locations,
    /// Every street area of Arkham.
    Streets,
    /// The street area and the locations of one district.
    District(StreetId),
    /// The Sky and the Outskirts.
    SkyAndOutskirts,
    /// « À Arkham » : locations, streets and the Sky, not the Outskirts (FAQ).
    Arkham,
}

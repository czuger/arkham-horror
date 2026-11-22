//! Street areas.

use serde::{Deserialize, Serialize};

use crate::ids::StreetId;
use crate::position::Position;

/// One of the 9 street areas, one per district.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Street {
    /// Identifier used by the other data files.
    pub id: StreetId,
    /// Name shown to the player, in French.
    pub name: String,
    /// Name shortened for the symbolic map, in French.
    pub short_name: String,
    /// Scene illustration of the district, relative to the `illustrations/` folder.
    pub illustration: String,
    /// Street areas reachable in one step. Links are symmetric.
    pub neighbours: Vec<StreetId>,
    /// Where a monster here goes when the Mythos card moves it along the white arrow.
    pub white_arrow: StreetId,
    /// Where a monster here goes when the Mythos card moves it along the black arrow.
    pub black_arrow: StreetId,
    /// Display position on the board image.
    pub position: Position,
}

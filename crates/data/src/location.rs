//! Arkham locations.

use serde::{Deserialize, Serialize};

use crate::ids::{LocationId, StreetId};
use crate::named_text::NamedText;
use crate::position::Position;

/// Whether gates can open at a location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stability {
    /// Green diamond on the board: no gate ever opens here.
    Stable,
    /// Red diamond on the board: gates and clue tokens appear here.
    Unstable,
}

/// One of the 26 Arkham locations.
///
/// A location is linked to a single street area, its own. Monsters in a
/// location always leave towards that street, whatever the arrow colour.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Location {
    /// Identifier used by the other data files.
    pub id: LocationId,
    /// Name shown to the player, in French.
    pub name: String,
    /// The street area of the location's district.
    pub street: StreetId,
    /// Stable or unstable.
    pub stability: Stability,
    /// Special ability printed under the location, if any.
    #[serde(default)]
    pub ability: Option<NamedText>,
    /// Terror level at which the location closes for good, if any.
    #[serde(default)]
    pub closes_at_terror: Option<u8>,
    /// Display position on the board image.
    pub position: Position,
}

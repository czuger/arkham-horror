//! Board areas that are neither locations, streets nor Other Worlds.

use serde::{Deserialize, Serialize};

use crate::position::Position;

/// Which special area this is. The rules give each one a fixed role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecialAreaKind {
    /// Where monsters go when the monster limit is reached.
    Outskirts,
    /// Where flying monsters wait; counts as a street linked to every street.
    Sky,
    /// Where investigators go when lost in time and space.
    LostInTimeAndSpace,
    /// Where arrested investigators go, next to the Police Station.
    JailCell,
}

/// A special area of the board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecialArea {
    /// Which area.
    pub kind: SpecialAreaKind,
    /// Name shown to the player, in French.
    pub name: String,
    /// Display position on the board image.
    pub position: Position,
}

//! The terror track printed on the board.

use serde::{Deserialize, Serialize};

use crate::position::Position;

/// The terror track: where the terror marker stands for each level.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TerrorTrack {
    /// Terror marker image, relative to the `assets/` folder.
    pub marker: String,
    /// Centre of each space, from level 0 to level 10.
    pub spaces: Vec<Position>,
}

impl TerrorTrack {
    /// Number of spaces: levels 0 to 10.
    pub const SPACES: usize = 11;

    /// Where the marker stands for a terror level (the last space beyond it).
    pub fn space(&self, level: u8) -> Option<Position> {
        let index = usize::from(level).min(self.spaces.len().checked_sub(1)?);
        self.spaces.get(index).copied()
    }
}

//! Gate markers.

use serde::{Deserialize, Serialize};

use crate::dimension_symbol::DimensionSymbol;
use crate::ids::OtherWorldId;

/// One kind of gate marker, with its number of copies in the box.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GateMarker {
    /// The Other World the gate leads to.
    pub world: OtherWorldId,
    /// Modifier to the Lore or Fight check made to close the gate.
    pub close_modifier: i8,
    /// Dimensional symbol.
    pub symbol: DimensionSymbol,
    /// Number of identical markers in the box.
    pub copies: u8,
    /// Marker image, relative to the `assets/` folder.
    pub image: String,
}

/// Content of `data/gate_markers.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GateMarkers {
    /// Format version of the file.
    pub version: u32,
    /// Image of the marker back, relative to the `assets/` folder.
    pub back_image: String,
    /// The marker kinds.
    pub markers: Vec<GateMarker>,
}

impl GateMarkers {
    /// Format version this code reads.
    pub const VERSION: u32 = 1;

    /// Total number of markers in the gate stack.
    pub fn total_copies(&self) -> u32 {
        self.markers
            .iter()
            .map(|marker| u32::from(marker.copies))
            .sum()
    }
}

//! Token images used on the investigator sheets.

use serde::{Deserialize, Serialize};

/// Images of the tokens placed on or beside an investigator sheet, relative to
/// the `assets/` folder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetMarkers {
    /// Slider (« pion coulissant »).
    pub slider: String,
    /// Sanity token (one per point).
    pub sanity: String,
    /// Stamina token (one per point).
    pub stamina: String,
    /// Money: the $1, $5 and $10 bills stacked top to bottom in one image.
    pub money: String,
    /// Clue token.
    pub clue: String,
}

impl SheetMarkers {
    /// Every image, for the asset checks.
    pub fn images(&self) -> [&str; 5] {
        [
            &self.slider,
            &self.sanity,
            &self.stamina,
            &self.money,
            &self.clue,
        ]
    }
}

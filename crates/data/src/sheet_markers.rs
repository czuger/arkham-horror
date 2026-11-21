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
    /// The $1 bill.
    pub bill_1: String,
    /// The $5 bill.
    pub bill_5: String,
    /// The $10 bill.
    pub bill_10: String,
    /// Clue token.
    pub clue: String,
}

impl SheetMarkers {
    /// Every image, for the asset checks.
    pub fn images(&self) -> [&str; 7] {
        [
            &self.slider,
            &self.sanity,
            &self.stamina,
            &self.bill_1,
            &self.bill_5,
            &self.bill_10,
            &self.clue,
        ]
    }

    /// The image of a bill, by its value; `None` for a value not printed.
    pub fn bill(&self, value: u16) -> Option<&str> {
        match value {
            1 => Some(&self.bill_1),
            5 => Some(&self.bill_5),
            10 => Some(&self.bill_10),
            _ => None,
        }
    }
}

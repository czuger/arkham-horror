use data::GameData;
use engine::GameState;

use crate::views::names::ancient_one;

/// The Ancient One sheet with a doom token on each filled space.
#[derive(Debug, Clone)]
pub struct DoomTrackView {
    /// French name of the Ancient One.
    pub name: String,
    /// Sheet image path under `/assets/`.
    pub sheet: String,
    /// Doom token image path under `/assets/`.
    pub marker: String,
    /// Where each doom token stands: left and top, in percent of the sheet.
    pub tokens: Vec<(String, String)>,
}

impl DoomTrackView {
    /// The game's Ancient One sheet with its doom tokens; `None` when the
    /// Ancient One is unknown to the data.
    pub fn new(data: &GameData, state: &GameState) -> Option<Self> {
        let sheet = ancient_one(data, &state.ancient_one)?;
        let size = sheet.image_size;
        Some(Self {
            name: sheet.name.clone(),
            sheet: sheet.image.clone(),
            marker: data.ancient_ones.doom_marker.clone(),
            tokens: sheet
                .doom_spaces
                .iter()
                .take(usize::from(state.doom))
                .map(|position| {
                    (
                        percent(position.x, size.width),
                        percent(position.y, size.height),
                    )
                })
                .collect(),
        })
    }
}

/// A pixel coordinate as a CSS percentage of the image size.
fn percent(pixels: u16, size: u16) -> String {
    format!("{:.2}%", f64::from(pixels) * 100.0 / f64::from(size.max(1)))
}

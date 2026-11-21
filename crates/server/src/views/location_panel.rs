use data::{Board, Location, Position};

use crate::views::board_geometry::{BOARD_SIZE, edge, percent};
use crate::views::location_tip::LocationTip;

/// The white area of a location on the board (name, and ability text if
/// any): an invisible hover zone showing the location's text and icons.
#[derive(Debug, Clone)]
pub struct LocationPanel {
    /// French name of the location.
    pub name: String,
    /// Left edge, in percent of the board width.
    pub left: String,
    /// Top edge, in percent of the board height.
    pub top: String,
    /// Width, in percent of the board width.
    pub width: String,
    /// Height, in percent of the board height.
    pub height: String,
    /// CSS classes telling which way the bubble opens.
    pub edge: String,
    /// Bubble shown on hover.
    pub tip: LocationTip,
}

impl LocationPanel {
    /// The hover zone of a location of `board`.
    pub fn new(board: &Board, location: &Location) -> Self {
        let panel = location.panel;
        let centre = Position {
            x: panel.x.saturating_add(panel.width / 2),
            y: panel.y.saturating_add(panel.height / 2),
        };
        Self {
            name: location.name.clone(),
            left: percent(panel.x, BOARD_SIZE.0),
            top: percent(panel.y, BOARD_SIZE.1),
            width: percent(panel.width, BOARD_SIZE.0),
            height: percent(panel.height, BOARD_SIZE.1),
            edge: edge(centre),
            tip: LocationTip::new(board, location),
        }
    }
}

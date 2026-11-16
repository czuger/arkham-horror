//! Pixel positions on the board image.

use serde::{Deserialize, Serialize};

/// A point on the board image (`assets/plateau/arkham.jpg`, 1064×1576), in pixels
/// from the top-left corner. Used only for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /// Horizontal coordinate.
    pub x: u16,
    /// Vertical coordinate.
    pub y: u16,
}

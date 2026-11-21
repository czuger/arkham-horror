//! Rectangles on the board image.

use serde::{Deserialize, Serialize};

/// A rectangle on the board image, in pixels from the top-left corner. Used
/// only for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rect {
    /// Left edge.
    pub x: u16,
    /// Top edge.
    pub y: u16,
    /// Width.
    pub width: u16,
    /// Height.
    pub height: u16,
}

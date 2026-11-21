//! Pixel size of an image.

use serde::{Deserialize, Serialize};

/// Width and height of an image, in pixels. Used only for display, to place
/// markers on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageSize {
    /// Width.
    pub width: u16,
    /// Height.
    pub height: u16,
}

use crate::views::tip::Tip;

/// The terror marker on the board's terror track.
#[derive(Debug, Clone)]
pub struct TerrorPawn {
    /// Terror level.
    pub level: u8,
    /// Marker image path under `/assets/`.
    pub image: String,
    /// Horizontal position, in percent of the board width.
    pub left: String,
    /// Vertical position, in percent of the board height.
    pub top: String,
    /// Bubble shown on hover.
    pub tip: Tip,
}

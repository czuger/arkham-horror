//! Dimensional symbols shared by gates and monsters.

use serde::{Deserialize, Serialize};

/// The dimensional symbol printed on gate markers and monster tokens.
///
/// Closing a gate sends back to the cup every monster with the same symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DimensionSymbol {
    /// Cross (R'lyeh).
    Cross,
    /// Hexagon (the Abyss).
    Hexagon,
    /// Circle (Yuggoth).
    Circle,
    /// Star (Great Hall of Celeano).
    Star,
    /// Diamond (Plateau of Leng).
    Diamond,
    /// Square (Another Dimension).
    Square,
    /// Triangle (City of the Great Race).
    Triangle,
    /// Slash (the Dreamlands).
    Slash,
    /// Crescent moon: no gate has it, so these monsters never return through a gate closing.
    Moon,
}

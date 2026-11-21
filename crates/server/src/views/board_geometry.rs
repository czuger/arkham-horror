//! Placing things on the board image, whatever size it is drawn at.

use data::Position;

/// Size in pixels of the board image the positions in `board.ron` refer to.
pub const BOARD_SIZE: (f64, f64) = (1064.0, 1576.0);

/// A pixel coordinate as a CSS percentage of the board size.
pub fn percent(pixels: u16, size: f64) -> String {
    format!("{:.2}%", f64::from(pixels) * 100.0 / size)
}

/// Which way the bubbles open from a point: towards the middle of the board,
/// so they stay inside the window (« west north »…).
pub fn edge(position: Position) -> String {
    let x = f64::from(position.x) / BOARD_SIZE.0;
    let y = f64::from(position.y) / BOARD_SIZE.1;
    let horizontal = if x < 0.3 {
        "west"
    } else if x > 0.7 {
        "east"
    } else {
        ""
    };
    let vertical = if y > 0.5 { "north" } else { "" };
    format!("{horizontal} {vertical}").trim().to_owned()
}

//! Placing things on the map, whatever size it is drawn at.

/// Size of the frame the positions in `board.ron` refer to (the printed
/// board's pixels), and the map's `viewBox`.
pub const BOARD_SIZE: (f64, f64) = (1064.0, 1576.0);

/// A pixel coordinate as a CSS percentage of the board size.
pub fn percent(pixels: u16, size: f64) -> String {
    format!("{:.2}%", f64::from(pixels) * 100.0 / size)
}

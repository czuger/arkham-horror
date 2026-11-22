use engine::View;

use crate::play_mode::PlayMode;

use crate::views::board_geometry::BOARD_SIZE;
use crate::views::board_view::BoardView;
use crate::views::spot::Spot;
use crate::views::terror_pawn::TerrorPawn;

/// The board turned a quarter right so it lies in landscape, drawn at its
/// pixel size with every token, and nothing to click.
///
/// The image file is portrait: its right-hand column (the Other Worlds)
/// becomes the bottom edge, so the « Autres Mondes » title reads the right
/// way up. A point `(x, y)` of the image lands at `(height − y, x)` on the
/// turned board.
#[derive(Debug, Clone)]
pub struct LandscapeBoardView {
    /// Board image path under `assets/`.
    pub image: String,
    /// Width of the image file, which is the height of the turned board.
    pub image_width: u16,
    /// Height of the image file, which is the width of the turned board.
    pub image_height: u16,
    /// The places with something to show, placed on the turned board.
    pub spots: Vec<Spot>,
    /// The terror marker, placed on the turned board.
    pub terror: Option<TerrorPawn>,
}

impl LandscapeBoardView {
    /// Draws every token of a state on the turned board.
    pub fn new(view: &View<'_, '_>, mode: PlayMode) -> Self {
        let board = BoardView::new(view, None, mode);
        Self {
            image: board.image,
            // safe: the board size constants are whole numbers well within u16.
            image_width: BOARD_SIZE.0 as u16,
            image_height: BOARD_SIZE.1 as u16,
            spots: board
                .spots
                .into_iter()
                .map(|spot| {
                    let (left, top) = turned(spot.point);
                    Spot { left, top, ..spot }
                })
                .collect(),
            terror: board.terror.map(|pawn| {
                let (left, top) = turned(pawn.point);
                TerrorPawn { left, top, ..pawn }
            }),
        }
    }
}

/// Left and top, in percent of the turned board, of a point of the image.
fn turned((x, y): (u16, u16)) -> (String, String) {
    let (width, height) = BOARD_SIZE;
    (
        format!("{:.2}%", (height - f64::from(y)) * 100.0 / height),
        format!("{:.2}%", f64::from(x) * 100.0 / width),
    )
}

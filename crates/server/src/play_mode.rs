use engine::Decision;
use serde::{Deserialize, Serialize};

/// Seat of the human player in a solo game; the AI plays every other seat.
pub const HUMAN_SEAT: usize = 0;

/// Who plays the investigators.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayMode {
    /// The human plays seat 0, the AI every other seat.
    #[default]
    Solo,
    /// The human plays every seat, one after the other (hot seat).
    HotSeat,
}

impl PlayMode {
    /// Whether the human plays the investigator at `seat`.
    pub fn human(self, seat: usize) -> bool {
        self == Self::HotSeat || seat == HUMAN_SEAT
    }

    /// Whether the investigator at `seat` is shown as the player's own: the
    /// human's in a solo game, the one who decides now in a hot-seat game.
    pub fn highlighted(self, seat: usize, decision: Option<&Decision>) -> bool {
        match self {
            Self::Solo => seat == HUMAN_SEAT,
            Self::HotSeat => decision.is_some_and(|decision| decision.who == seat),
        }
    }

    /// « Vous » or « IA », for the investigator at `seat`.
    pub fn player_label(self, seat: usize) -> &'static str {
        if self.human(seat) { "Vous" } else { "IA" }
    }
}

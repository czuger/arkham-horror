use engine::Setup;
use serde::{Deserialize, Serialize};

/// What is saved of a game: enough to replay it exactly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress {
    /// The setup, seed included.
    pub setup: Setup,
    /// Every answer given so far, the human's and the AI's, in order.
    pub answers: Vec<usize>,
    /// The human's answers, for undoing: position in `answers`, and
    /// `seen_events` before that answer.
    pub human_moves: Vec<(usize, usize)>,
    /// Journal lines the player had seen when they last answered; the
    /// lines after it are shown as new.
    pub seen_events: usize,
}

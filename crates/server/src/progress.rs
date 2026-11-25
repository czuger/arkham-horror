use std::collections::BTreeMap;

use engine::Setup;
use serde::{Deserialize, Serialize};

use crate::play_mode::PlayMode;

/// What is saved of a game: enough to replay it exactly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress {
    /// The setup, seed included.
    pub setup: Setup,
    /// Who plays the investigators; games saved before hot seat are solo.
    #[serde(default)]
    pub mode: PlayMode,
    /// Every answer given so far, the human's and the AI's, in order.
    pub answers: Vec<usize>,
    /// The human's answers, for undoing: position in `answers`, and
    /// `seen_events` before that answer.
    pub human_moves: Vec<(usize, usize)>,
    /// Journal lines the player had seen when they last answered; the
    /// lines after it are shown as new.
    pub seen_events: usize,
    /// Skill adjustments picked on the adjustment screen, not yet played.
    #[serde(default)]
    pub planned_sliders: PlannedSliders,
}

/// The slider stops picked for each investigator on the adjustment screen,
/// played all at once when the human moves on.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedSliders {
    /// Number of answers given when they were picked: stale after any move.
    pub move_number: usize,
    /// Stops picked on the three tracks, by seat.
    pub targets: BTreeMap<usize, [u8; 3]>,
}

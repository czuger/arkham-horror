use engine::{Shown, View};

use crate::views::journal_line::{JournalLine, journal_lines};

/// Journal lines shown beside the board.
#[derive(Debug, Clone)]
pub struct JournalView {
    /// Lines the player had already seen (the last few only).
    pub older: Vec<JournalLine>,
    /// Lines played since the player's last answer, with the cards drawn
    /// and the dice rolled.
    pub new: Vec<JournalLine>,
    /// Whether earlier lines were left out.
    pub truncated: bool,
}

/// Lines already seen that stay on screen for context.
const OLDER_SHOWN: usize = 8;

impl JournalView {
    /// The recent part of a journal.
    ///
    /// # Arguments
    ///
    /// * `seen` - lines the player had seen when they last answered.
    pub fn recent(view: &View<'_, '_>, events: &[String], shown: &[Shown], seen: usize) -> Self {
        let seen = seen.min(events.len());
        let first = seen.saturating_sub(OLDER_SHOWN);
        Self {
            older: journal_lines(view, events, shown, first, seen),
            new: journal_lines(view, events, shown, seen, events.len()),
            truncated: first > 0,
        }
    }
}

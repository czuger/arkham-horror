/// Journal lines shown beside the board.
#[derive(Debug, Clone)]
pub struct JournalView {
    /// Lines the player had already seen (the last few only).
    pub older: Vec<String>,
    /// Lines played since the player's last answer.
    pub new: Vec<String>,
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
    pub fn recent(events: &[String], seen: usize) -> Self {
        let seen = seen.min(events.len());
        let first = seen.saturating_sub(OLDER_SHOWN);
        Self {
            older: events[first..seen].to_vec(),
            new: events[seen..].to_vec(),
            truncated: first > 0,
        }
    }
}

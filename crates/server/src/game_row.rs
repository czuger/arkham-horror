/// One saved game, as read from the `game` table.
///
/// The title and summary are kept beside the record so that the home page
/// lists games without replaying them.
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct GameRow {
    /// Game number, shown in the URL.
    pub id: i64,
    /// Creation time, ISO 8601 UTC (`2026-11-21T10:00:00Z`).
    pub created_at: String,
    /// Who plays against which Ancient One.
    pub title: String,
    /// Turn and phase, or how the game ended.
    pub summary: String,
    /// Whether the game is over.
    pub over: bool,
    /// The [`crate::progress::Progress`] as JSON.
    pub record: String,
}

/// What is written for a game; the number and creation time are set by
/// the database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameContent {
    /// Who plays against which Ancient One (200 characters at most).
    pub title: String,
    /// Turn and phase, or how the game ended (200 characters at most).
    pub summary: String,
    /// Whether the game is over.
    pub over: bool,
    /// The [`crate::progress::Progress`] as JSON.
    pub record: String,
}

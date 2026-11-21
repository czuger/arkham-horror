use serde::{Deserialize, Serialize};

/// One saved game, as stored in the `partie` collection.
///
/// The title and summary are kept beside the record so that the home page
/// lists games without replaying them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameRow {
    /// Game number, shown in the URL.
    pub id: i64,
    /// Creation time, in seconds since 1970.
    pub created_unix: i64,
    /// Who plays against which Ancient One.
    pub title: String,
    /// Turn and phase, or how the game ended.
    pub summary: String,
    /// Whether the game is over.
    pub over: bool,
    /// The [`crate::progress::Progress`] as a JSON string (BSON has no
    /// unsigned 64-bit integers, which the seed needs).
    pub record: String,
}

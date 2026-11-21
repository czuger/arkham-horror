use std::sync::Arc;

use data::GameData;

use crate::database::Database;

/// Shared state handed to every request handler.
#[derive(Clone)]
pub struct AppState {
    /// The embedded database.
    pub db: Database,
    /// The game data, loaded once at startup.
    pub data: Arc<GameData>,
    /// Whether `static/htmx.min.js` exists; pages load it only then.
    pub htmx: bool,
}

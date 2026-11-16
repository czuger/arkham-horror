use crate::database::Database;

/// Shared state handed to every request handler.
#[derive(Clone)]
pub struct AppState {
    /// The embedded database.
    pub db: Database,
}

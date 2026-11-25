use std::sync::Arc;

use data::GameData;

use crate::ambiance::{Ambiance, Screen};
use crate::database::Database;
use crate::music_library::MusicLibrary;
use crate::session::Session;

/// Shared state handed to every request handler.
#[derive(Clone)]
pub struct AppState {
    /// The embedded database.
    pub db: Database,
    /// The game data, loaded once at startup.
    pub data: Arc<GameData>,
    /// Whether `static/htmx.min.js` exists; pages load it only then.
    pub htmx: bool,
    /// The music tracks, listed once at startup.
    pub music: Arc<MusicLibrary>,
}

impl AppState {
    /// The music mood of a game page, falling back on the default mood when
    /// that one has no track.
    pub fn ambiance(&self, session: &Session, screen: Screen) -> Ambiance {
        self.music.playable(Ambiance::of(session, screen))
    }
}

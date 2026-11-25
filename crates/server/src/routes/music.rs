use axum::Json;
use axum::extract::State;

use crate::app_state::AppState;
use crate::music_library::MusicLibrary;

/// The track names of each mood, as JSON:
/// `{"default": [...], "other_world": [...], "combat": [...]}`.
/// A track is then fetched at `musique/<mood>/<name>`.
pub async fn music_tracks(State(state): State<AppState>) -> Json<MusicLibrary> {
    Json(MusicLibrary::clone(&state.music))
}

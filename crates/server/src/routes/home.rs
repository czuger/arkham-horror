use askama::Template;
use axum::extract::State;
use axum::http::Uri;
use axum::response::Html;

use crate::ambiance::Ambiance;
use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::game_row::GameRow;
use crate::relative_path::root_of;

#[derive(Template)]
#[template(path = "home.html")]
struct HomePage {
    htmx: bool,
    ambiance: Ambiance,
    root: String,
    games: Vec<GameRow>,
}

/// Home page: games in progress and over, and a link to start one.
///
/// # Errors
///
/// Fails if the database or the template fails.
pub async fn home(State(state): State<AppState>, uri: Uri) -> Result<Html<String>, AppError> {
    let games = state.db.all_games().await?;
    Ok(Html(
        HomePage {
            htmx: state.htmx,
            ambiance: Ambiance::Default,
            root: root_of(uri.path()),
            games,
        }
        .render()?,
    ))
}

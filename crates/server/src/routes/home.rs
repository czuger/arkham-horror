use askama::Template;
use axum::extract::State;
use axum::response::Html;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::game_row::GameRow;

#[derive(Template)]
#[template(path = "home.html")]
struct HomePage {
    htmx: bool,
    games: Vec<GameRow>,
}

/// Home page: games in progress and over, and a link to start one.
///
/// # Errors
///
/// Fails if the database or the template fails.
pub async fn home(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let games = state.db.all_games()?;
    Ok(Html(
        HomePage {
            htmx: state.htmx,
            games,
        }
        .render()?,
    ))
}

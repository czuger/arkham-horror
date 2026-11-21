use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::session::Session;

#[derive(Template)]
#[template(path = "journal.html")]
struct JournalPage {
    htmx: bool,
    id: i64,
    events: Vec<String>,
    can_undo: bool,
}

/// The whole journal of a game.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn journal(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let session = Session::load(&state.db, &state.data, id).await?;
    Ok(Html(
        JournalPage {
            htmx: state.htmx,
            id,
            can_undo: !session.progress.human_moves.is_empty(),
            events: session.outcome.events,
        }
        .render()?,
    ))
}

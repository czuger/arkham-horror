use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use engine::inspect;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::session::Session;
use crate::views::{JournalLine, journal_lines};

#[derive(Template)]
#[template(path = "journal.html")]
struct JournalPage {
    htmx: bool,
    id: i64,
    lines: Vec<JournalLine>,
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
    let outcome = &session.outcome;
    let lines = inspect(&state.data, &outcome.state, |view| {
        journal_lines(
            view,
            &outcome.events,
            &outcome.shown,
            0,
            outcome.events.len(),
        )
    });
    Ok(Html(
        JournalPage {
            htmx: state.htmx,
            id,
            can_undo: !session.progress.human_moves.is_empty(),
            lines,
        }
        .render()?,
    ))
}

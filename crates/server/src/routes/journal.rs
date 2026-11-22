use askama::Template;
use axum::extract::{Path, State};
use axum::http::Uri;
use axum::response::Html;
use engine::inspect;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::relative_path::root_of;
use crate::session::Session;
use crate::views::{JournalLine, journal_lines};

#[derive(Template)]
#[template(path = "journal.html")]
struct JournalPage {
    htmx: bool,
    root: String,
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
    uri: Uri,
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
            root: root_of(uri.path()),
            id,
            can_undo: !session.progress.human_moves.is_empty(),
            lines,
        }
        .render()?,
    ))
}

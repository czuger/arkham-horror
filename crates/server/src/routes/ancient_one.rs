use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::session::Session;
use crate::views::DoomTrackView;

#[derive(Template)]
#[template(path = "ancient_one.html")]
struct AncientOnePage {
    htmx: bool,
    id: i64,
    track: DoomTrackView,
}

/// The Ancient One sheet of a game, with its doom tokens.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game or Ancient One; fails if the
/// replay or the template fails.
pub async fn ancient_one(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let session = Session::load(&state.db, &state.data, id)?;
    let track =
        DoomTrackView::new(&state.data, &session.outcome.state).ok_or(AppError::NotFound)?;
    Ok(Html(
        AncientOnePage {
            htmx: state.htmx,
            id,
            track,
        }
        .render()?,
    ))
}

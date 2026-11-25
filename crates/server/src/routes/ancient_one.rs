use askama::Template;
use axum::extract::{Path, State};
use axum::http::Uri;
use axum::response::Html;

use crate::ambiance::{Ambiance, Screen};
use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::relative_path::root_of;
use crate::session::Session;
use crate::views::DoomTrackView;

#[derive(Template)]
#[template(path = "ancient_one.html")]
struct AncientOnePage {
    htmx: bool,
    ambiance: Ambiance,
    root: String,
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
    uri: Uri,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let session = Session::load(&state.db, &state.data, id).await?;
    let ambiance = state.ambiance(&session, Screen::Other);
    let track =
        DoomTrackView::new(&state.data, &session.outcome.state).ok_or(AppError::NotFound)?;
    Ok(Html(
        AncientOnePage {
            htmx: state.htmx,
            ambiance,
            root: root_of(uri.path()),
            id,
            track,
        }
        .render()?,
    ))
}

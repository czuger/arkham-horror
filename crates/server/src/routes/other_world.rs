use askama::Template;
use axum::extract::{Path, State};
use axum::http::Uri;
use axum::response::{Html, IntoResponse, Response};
use engine::inspect;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::relative_path::{redirect_from, root_of};
use crate::session::Session;
use crate::views::OtherWorldView;

#[derive(Template)]
#[template(path = "other_world.html")]
struct OtherWorldPage {
    htmx: bool,
    root: String,
    id: i64,
    encounter: OtherWorldView,
    /// Where the game goes on.
    next: String,
}

/// The last Other World encounter of the investigator the human plays: a
/// pause, so that it does not go by unseen. Without one, goes to the page
/// where the current decision is answered.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn other_world(
    State(state): State<AppState>,
    uri: Uri,
    Path(id): Path<i64>,
) -> Result<Response, AppError> {
    let session = Session::load(&state.db, &state.data, id).await?;
    let Some(span) = session.other_world() else {
        return Ok(redirect_from(&uri, &session.decision_page()).into_response());
    };
    let seat = session.other_world_seat();
    let page = inspect(&state.data, &session.outcome.state, |view| {
        Some(OtherWorldPage {
            htmx: state.htmx,
            root: root_of(uri.path()),
            id,
            encounter: OtherWorldView::new(view, seat, &session.outcome, &span)?,
            next: session.decision_page(),
        })
    });
    Ok(Html(page.ok_or(AppError::NotFound)?.render()?).into_response())
}

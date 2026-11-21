use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use engine::inspect;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::session::Session;
use crate::views::LandscapeBoardView;

#[derive(Template)]
#[template(path = "landscape_board.html")]
struct LandscapeBoardPage {
    htmx: bool,
    id: i64,
    board: LandscapeBoardView,
}

/// The board of a game at its pixel size, in landscape, with every token.
/// A test page: nothing on it can be clicked.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn landscape_board(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let session = Session::load(&state.db, &state.data, id).await?;
    let board = inspect(&state.data, &session.outcome.state, LandscapeBoardView::new);
    Ok(Html(
        LandscapeBoardPage {
            htmx: state.htmx,
            id,
            board,
        }
        .render()?,
    ))
}

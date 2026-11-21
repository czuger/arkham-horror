use axum::Form;
use axum::extract::{Path, State};
use axum::response::Redirect;
use serde::Deserialize;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::session::Session;

/// An answer to the current decision, as posted.
#[derive(Debug, Deserialize)]
pub struct AnswerForm {
    /// Index of the chosen option.
    choix: usize,
    /// Number of answers given when the page was shown.
    coup: usize,
}

/// Applies the human's answer, lets the AI play up to the human's next
/// decision, saves, and goes back to the game page.
///
/// # Errors
///
/// [`AppError::BadRequest`] for an answer that is not an option; fails if
/// the replay or the database fails.
pub async fn answer(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<AnswerForm>,
) -> Result<Redirect, AppError> {
    let mut session = Session::load(&state.db, &state.data, id)?;
    if session.answer(&state.data, form.choix, form.coup)? {
        session.save(&state.db, &state.data)?;
    }
    Ok(Redirect::to(&format!("/partie/{id}")))
}

/// Takes back the human's last answer and what the AI played after it.
///
/// # Errors
///
/// Fails if the replay or the database fails.
pub async fn undo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let mut session = Session::load(&state.db, &state.data, id)?;
    if session.undo(&state.data)? {
        session.save(&state.db, &state.data)?;
    }
    Ok(Redirect::to(&format!("/partie/{id}")))
}

/// Deletes a game and goes back home.
///
/// # Errors
///
/// Fails if the database fails.
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    state.db.delete_game(id)?;
    Ok(Redirect::to("/"))
}

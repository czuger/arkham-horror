use askama::Template;
use axum::extract::State;
use axum::response::Html;

use crate::app_error::AppError;
use crate::app_state::AppState;

#[derive(Template)]
#[template(path = "home.html")]
struct HomePage {
    visits: u64,
}

/// Home page. For now (milestone J0) it only counts visits in the database.
///
/// # Errors
///
/// Fails if the database or the template fails.
pub async fn home(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let visits = state.db.record_visit()?;
    Ok(Html(HomePage { visits }.render()?))
}

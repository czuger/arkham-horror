use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use data::SheetMarkers;
use engine::inspect;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::session::Session;
use crate::views::InvestigatorMat;

#[derive(Template)]
#[template(path = "investigators.html")]
struct InvestigatorsPage {
    htmx: bool,
    id: i64,
    markers: SheetMarkers,
    mats: Vec<InvestigatorMat>,
}

/// Every investigator's sheet of a game, with sliders, tokens and trophies.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn investigators(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let session = Session::load(&state.db, &state.data, id)?;
    let mats = inspect(&state.data, &session.outcome.state, |view| {
        (0..view.state().investigators.len())
            .filter_map(|seat| InvestigatorMat::new(view, seat))
            .collect()
    });
    Ok(Html(
        InvestigatorsPage {
            htmx: state.htmx,
            id,
            markers: state.data.investigators.markers.clone(),
            mats,
        }
        .render()?,
    ))
}

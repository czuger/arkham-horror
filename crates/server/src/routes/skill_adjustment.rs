use askama::Template;
use axum::Form;
use axum::extract::{Path, State};
use axum::http::Uri;
use axum::response::{Html, IntoResponse, Redirect, Response};
use engine::inspect;
use serde::Deserialize;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::relative_path::{redirect_from, root_of};
use crate::session::Session;
use crate::views::{SkillAdjustment, adjusts_skills};

#[derive(Template)]
#[template(path = "skill_adjustment.html")]
struct SkillAdjustmentPage {
    htmx: bool,
    root: String,
    id: i64,
    slider: String,
    adjustment: SkillAdjustment,
    can_undo: bool,
}

/// The stops picked on the adjustment page, as posted.
#[derive(Debug, Deserialize)]
pub struct AdjustmentForm {
    /// Number of answers given when the page was shown.
    coup: usize,
    /// Stop picked on Vitesse / Discrétion.
    piste0: u8,
    /// Stop picked on Vigueur / Volonté.
    piste1: u8,
    /// Stop picked on Savoir / Chance.
    piste2: u8,
}

/// The Upkeep skill adjustment page: the sheet, with the stops each slider
/// may go to within the Concentration. Without an adjustment under way, goes
/// to the page where the current decision is answered.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn skill_adjustment(
    State(state): State<AppState>,
    uri: Uri,
    Path(id): Path<i64>,
) -> Result<Response, AppError> {
    let session = Session::load(&state.db, &state.data, id).await?;
    let Some(decision) = session.decision().filter(|d| adjusts_skills(d)) else {
        return Ok(redirect_from(&uri, &session.page()).into_response());
    };
    let page = inspect(&state.data, &session.outcome.state, |view| {
        Some(SkillAdjustmentPage {
            htmx: state.htmx,
            root: root_of(uri.path()),
            id,
            slider: state.data.investigators.markers.slider.clone(),
            adjustment: SkillAdjustment::new(view, decision, session.move_number())?,
            can_undo: !session.progress.human_moves.is_empty(),
        })
    });
    Ok(Html(page.ok_or(AppError::NotFound)?.render()?).into_response())
}

/// Moves the sliders to the stops picked, lets the AI play up to the human's
/// next decision, saves, and goes to the page where it is answered.
///
/// # Errors
///
/// [`AppError::BadRequest`] for stops out of reach; fails if the replay or
/// the database fails.
pub async fn adjust_skills(
    State(state): State<AppState>,
    uri: Uri,
    Path(id): Path<i64>,
    Form(form): Form<AdjustmentForm>,
) -> Result<Redirect, AppError> {
    let mut session = Session::load(&state.db, &state.data, id).await?;
    let targets = [form.piste0, form.piste1, form.piste2];
    if session.adjust_skills(&state.data, targets, form.coup)? {
        session.save(&state.db, &state.data).await?;
    }
    Ok(redirect_from(&uri, &session.page()))
}

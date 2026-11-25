use askama::Template;
use axum::Form;
use axum::extract::{Path, State};
use axum::http::Uri;
use axum::response::{Html, IntoResponse, Redirect, Response};
use engine::inspect;
use serde::Deserialize;

use crate::ambiance::{Ambiance, Screen};
use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::relative_path::{redirect_from, root_of};
use crate::session::Session;
use crate::views::{AdjustmentRoster, SkillAdjustment};

#[derive(Template)]
#[template(path = "adjustment_roster.html")]
struct AdjustmentRosterPage {
    htmx: bool,
    ambiance: Ambiance,
    root: String,
    id: i64,
    roster: AdjustmentRoster,
    can_undo: bool,
}

#[derive(Template)]
#[template(path = "skill_adjustment.html")]
struct SkillAdjustmentPage {
    htmx: bool,
    ambiance: Ambiance,
    root: String,
    id: i64,
    slider: String,
    adjustment: SkillAdjustment,
}

/// The move-on form of the adjustment screen, as posted.
#[derive(Debug, Deserialize)]
pub struct MoveOnForm {
    /// Number of answers given when the page was shown.
    coup: usize,
}

/// The stops picked on one investigator's adjustment page, as posted.
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

/// The Upkeep adjustment screen: every investigator, a button to adjust
/// each one the human plays, and one to move on. Without an adjustment
/// under way, goes to the page where the current decision is answered.
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
    let adjusting = session.human_adjusters(&state.data);
    if adjusting.is_empty() {
        return Ok(redirect_from(&uri, &session.page()).into_response());
    }
    let roster = inspect(&state.data, &session.outcome.state, |view| {
        AdjustmentRoster::new(
            view,
            &adjusting,
            &session.planned_sliders(),
            session.mode(),
            session.move_number(),
        )
    });
    let page = AdjustmentRosterPage {
        htmx: state.htmx,
        ambiance: state.ambiance(&session, Screen::Other),
        root: root_of(uri.path()),
        id,
        roster,
        can_undo: !session.progress.human_moves.is_empty(),
    };
    Ok(Html(page.render()?).into_response())
}

/// Moves on from the adjustment screen: plays the stops picked, lets the
/// AI play up to the human's next decision, saves, and goes to the page
/// where it is answered.
///
/// # Errors
///
/// [`AppError::BadRequest`] without an adjustment under way; fails if the
/// replay or the database fails.
pub async fn adjust_skills(
    State(state): State<AppState>,
    uri: Uri,
    Path(id): Path<i64>,
    Form(form): Form<MoveOnForm>,
) -> Result<Redirect, AppError> {
    let mut session = Session::load(&state.db, &state.data, id).await?;
    if session.adjust_skills(&state.data, form.coup)? {
        session.save(&state.db, &state.data).await?;
    }
    Ok(redirect_from(&uri, &session.page()))
}

/// One investigator's adjustment page: the sheet, with the stops each
/// slider may go to within the Concentration. Back to the adjustment screen
/// if they do not adjust now.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn investigator_adjustment(
    State(state): State<AppState>,
    uri: Uri,
    Path((id, seat)): Path<(i64, usize)>,
) -> Result<Response, AppError> {
    let session = Session::load(&state.db, &state.data, id).await?;
    let Some(adjuster) = session
        .human_adjusters(&state.data)
        .into_iter()
        .find(|adjuster| adjuster.seat == seat)
    else {
        return Ok(redirect_from(&uri, &roster_page(id)).into_response());
    };
    let planned = session.planned_sliders().get(&seat).copied();
    let page = inspect(&state.data, &session.outcome.state, |view| {
        Some(SkillAdjustmentPage {
            htmx: state.htmx,
            ambiance: state.ambiance(&session, Screen::Other),
            root: root_of(uri.path()),
            id,
            slider: state.data.investigators.markers.slider.clone(),
            adjustment: SkillAdjustment::new(view, adjuster, planned, session.move_number())?,
        })
    });
    Ok(Html(page.ok_or(AppError::NotFound)?.render()?).into_response())
}

/// Keeps the stops picked for one investigator and goes back to the
/// adjustment screen.
///
/// # Errors
///
/// [`AppError::BadRequest`] for stops out of reach; fails if the replay or
/// the database fails.
pub async fn plan_adjustment(
    State(state): State<AppState>,
    uri: Uri,
    Path((id, seat)): Path<(i64, usize)>,
    Form(form): Form<AdjustmentForm>,
) -> Result<Redirect, AppError> {
    let mut session = Session::load(&state.db, &state.data, id).await?;
    let targets = [form.piste0, form.piste1, form.piste2];
    if session.plan_skills(&state.data, seat, targets, form.coup)? {
        session.save(&state.db, &state.data).await?;
    }
    Ok(redirect_from(&uri, &roster_page(id)))
}

/// The adjustment screen's path.
fn roster_page(id: i64) -> String {
    format!("partie/{id}/ajustement")
}

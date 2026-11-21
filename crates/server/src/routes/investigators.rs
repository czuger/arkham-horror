use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use data::SheetMarkers;
use engine::{View, inspect};

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::session::{HUMAN_SEAT, Session};
use crate::views::{HandZones, InvestigatorMat, SheetDecision, on_sheet, sheet};

#[derive(Template)]
#[template(path = "investigator.html")]
struct InvestigatorPage {
    htmx: bool,
    id: i64,
    markers: SheetMarkers,
    tabs: Vec<Tab>,
    mat: InvestigatorMat,
    hands: HandZones,
    decision: Option<SheetDecision>,
    can_undo: bool,
}

/// A link to one investigator's page.
struct Tab {
    seat: usize,
    name: String,
    player: &'static str,
    current: bool,
}

/// The human investigator's page.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn investigators(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    page(&state, id, HUMAN_SEAT).await
}

/// One investigator's page: sheet with its sliders, tokens, trophies and
/// possessions, with the cards held in hand. On the human's page, the
/// sliders and the hands are set here.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game or seat; fails if the replay
/// or the template fails.
pub async fn investigator(
    State(state): State<AppState>,
    Path((id, seat)): Path<(i64, usize)>,
) -> Result<Html<String>, AppError> {
    page(&state, id, seat).await
}

/// Renders the page of the investigator at `seat`.
async fn page(state: &AppState, id: i64, seat: usize) -> Result<Html<String>, AppError> {
    let session = Session::load(&state.db, &state.data, id).await?;
    let decision = session
        .decision()
        .filter(|decision| seat == HUMAN_SEAT && on_sheet(decision));
    let page = inspect(&state.data, &session.outcome.state, |view| {
        let mat = InvestigatorMat::new(view, seat)?;
        let inv = view.state().investigators.get(seat)?;
        let sheet = sheet(view.data(), &inv.id)?;
        Some(InvestigatorPage {
            htmx: state.htmx,
            id,
            markers: state.data.investigators.markers.clone(),
            tabs: tabs(view, seat),
            mat,
            hands: HandZones::new(view.data(), inv, decision),
            decision: decision
                .map(|decision| SheetDecision::new(view, decision, session.move_number(), sheet)),
            can_undo: !session.progress.human_moves.is_empty(),
        })
    });
    Ok(Html(page.ok_or(AppError::NotFound)?.render()?))
}

/// Links to every investigator's page.
fn tabs(view: &View<'_, '_>, current: usize) -> Vec<Tab> {
    (0..view.state().investigators.len())
        .map(|seat| Tab {
            seat,
            name: view.name(seat),
            player: if seat == HUMAN_SEAT { "vous" } else { "IA" },
            current: seat == current,
        })
        .collect()
}

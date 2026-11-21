use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use engine::inspect;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::session::Session;
use crate::views::{CombatDecision, CombatView, HandZones};

#[derive(Template)]
#[template(path = "combat.html")]
struct CombatPage {
    htmx: bool,
    id: i64,
    combat: CombatView,
    hands: HandZones,
    decision: Option<CombatDecision>,
    /// Where to go once the combat is over.
    next: String,
    can_undo: bool,
}

/// The combat of the investigator the human plays now: the monster, every roll of the
/// combat, the cards in hand, and the decision when it is about the
/// combat.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn combat(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let session = Session::load(&state.db, &state.data, id).await?;
    let decision = session.decision().filter(|_| session.in_combat());
    let seat = session.combat_seat();
    let span = session.combat();
    let outcome = &session.outcome;
    let page = inspect(&state.data, &outcome.state, |view| {
        let inv = view.state().investigators.get(seat)?;
        Some(CombatPage {
            htmx: state.htmx,
            id,
            combat: CombatView::new(
                view,
                seat,
                outcome,
                span.as_ref(),
                session.progress.seen_events,
                decision,
            )?,
            hands: HandZones::new(view.data(), inv, decision),
            decision: decision
                .map(|decision| CombatDecision::new(view, decision, session.move_number())),
            next: session.decision_page(),
            can_undo: !session.progress.human_moves.is_empty(),
        })
    });
    Ok(Html(page.ok_or(AppError::NotFound)?.render()?))
}

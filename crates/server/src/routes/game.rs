use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use engine::inspect;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::session::{HUMAN_SEAT, Session};
use crate::views::{BoardView, DecisionView, JournalView, PartyRow, SheetView, StatusView};

#[derive(Template)]
#[template(path = "game.html")]
struct GamePage {
    htmx: bool,
    id: i64,
    seed: u64,
    board: BoardView,
    sheet: Option<SheetView>,
    party: Vec<PartyRow>,
    status: StatusView,
    decision: Option<DecisionView>,
    journal: JournalView,
    can_undo: bool,
}

/// The game page: board, sheet, status, decision form and recent journal.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn game(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let session = Session::load(&state.db, &state.data, id)?;
    let decision = session.decision();
    let outcome = &session.outcome;
    let page = inspect(&state.data, &outcome.state, |view| GamePage {
        htmx: state.htmx,
        id,
        seed: session.progress.setup.seed,
        board: BoardView::new(&state.data, &outcome.state, decision),
        sheet: SheetView::new(view, HUMAN_SEAT),
        party: PartyRow::all(view),
        status: StatusView::new(view, &outcome.status),
        decision: decision.map(|d| DecisionView::new(view, d, session.move_number())),
        journal: JournalView::recent(&outcome.events, session.progress.seen_events),
        can_undo: !session.progress.human_moves.is_empty(),
    });
    Ok(Html(page.render()?))
}

use askama::Template;
use axum::extract::{Path, State};
use axum::http::Uri;
use axum::response::Html;
use engine::inspect;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::relative_path::root_of;
use crate::session::Session;
use crate::views::{BoardView, DecisionView, JournalView, PartyRow, StatusView};

#[derive(Template)]
#[template(path = "game.html")]
struct GamePage {
    htmx: bool,
    root: String,
    id: i64,
    seed: u64,
    board: BoardView,
    party: Vec<PartyRow>,
    status: StatusView,
    decision: Option<DecisionView>,
    journal: JournalView,
    can_undo: bool,
}

/// The game page: board, decision form, status, investigators and recent
/// journal with the cards drawn and the dice rolled.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn game(
    State(state): State<AppState>,
    uri: Uri,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let session = Session::load(&state.db, &state.data, id).await?;
    let decision = session.decision();
    let outcome = &session.outcome;
    let page = inspect(&state.data, &outcome.state, |view| GamePage {
        htmx: state.htmx,
        root: root_of(uri.path()),
        id,
        seed: session.progress.setup.seed,
        board: BoardView::new(view, decision, session.mode()),
        party: PartyRow::all(view, decision, session.mode()),
        status: StatusView::new(view, &outcome.status),
        decision: decision.map(|d| {
            DecisionView::new(view, d, session.move_number()).in_combat(session.in_combat())
        }),
        journal: JournalView::recent(
            view,
            &outcome.events,
            &outcome.shown,
            session.progress.seen_events,
        ),
        can_undo: !session.progress.human_moves.is_empty(),
    });
    Ok(Html(page.render()?))
}

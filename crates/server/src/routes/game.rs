use askama::Template;
use axum::extract::{Path, State};
use axum::http::Uri;
use axum::response::Html;
use data::GameData;
use engine::inspect;

use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::relative_path::root_of;
use crate::session::Session;
use crate::views::{BoardView, DecisionView, JournalView, MapView, PartyRow, StatusView, on_sheet};

#[derive(Template)]
#[template(path = "game.html")]
struct GamePage {
    htmx: bool,
    root: String,
    id: i64,
    seed: u64,
    board: BoardView,
    map: MapView,
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
    let page = game_page(&state.data, &session, root_of(uri.path()), state.htmx);
    Ok(Html(page?))
}

/// Renders the game page of a loaded game.
///
/// # Arguments
///
/// * `root` - the way back to the site root, for `<base href>`.
/// * `htmx` - whether the pages load htmx.
///
/// # Errors
///
/// Fails if the template fails.
pub(crate) fn game_page(
    data: &GameData,
    session: &Session,
    root: String,
    htmx: bool,
) -> Result<String, askama::Error> {
    let id = session.id;
    let decision = session.decision();
    let outcome = &session.outcome;
    let answered_here = decision.filter(|d| !on_sheet(d) && !session.in_combat());
    let page = inspect(data, &outcome.state, |view| GamePage {
        htmx,
        root,
        id,
        seed: session.progress.setup.seed,
        board: BoardView::new(view, decision, session.mode()),
        map: MapView::new(view, answered_here, session.mode(), id),
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
    page.render()
}

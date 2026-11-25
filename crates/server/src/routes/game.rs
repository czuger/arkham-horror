use askama::Template;
use axum::extract::{Path, Query, State};
use axum::http::Uri;
use axum::response::Html;
use data::{CardId, GameData, MonsterId};
use engine::{Context, Deck, Exhibit, inspect};
use serde::Deserialize;

use crate::ambiance::{Ambiance, Screen};
use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::relative_path::root_of;
use crate::session::Session;
use crate::views::{
    BoardView, DecisionView, JournalView, MapView, PartyRow, SceneRequest, SceneSubject, SceneView,
    StatusView, on_sheet,
};

#[derive(Template)]
#[template(path = "game.html")]
struct GamePage {
    htmx: bool,
    ambiance: Ambiance,
    root: String,
    id: i64,
    seed: u64,
    map: MapView,
    scene: SceneView,
    party: Vec<PartyRow>,
    status: StatusView,
    decision: Option<DecisionView>,
    journal: JournalView,
    can_undo: bool,
}

/// The query of the game page.
#[derive(Debug, Deserialize)]
pub struct GameQuery {
    /// A place to preview in the scene (`?voir=woods`); the game is left
    /// as it is.
    voir: Option<String>,
}

/// The game page: map, scene, decision form, status, investigators and
/// recent journal with the cards drawn and the dice rolled.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn game(
    State(state): State<AppState>,
    uri: Uri,
    Path(id): Path<i64>,
    Query(query): Query<GameQuery>,
) -> Result<Html<String>, AppError> {
    let session = Session::load(&state.db, &state.data, id).await?;
    let root = root_of(uri.path());
    let layout = Layout {
        root,
        htmx: state.htmx,
        ambiance: state.ambiance(&session, Screen::Other),
    };
    let page = game_page(&state.data, &session, layout, query.voir.as_deref());
    Ok(Html(page?))
}

/// What every page needs besides its own content.
pub(crate) struct Layout {
    /// The way back to the site root, for `<base href>`.
    pub root: String,
    /// Whether the pages load htmx.
    pub htmx: bool,
    /// The music mood.
    pub ambiance: Ambiance,
}

/// Renders the game page of a loaded game.
///
/// # Arguments
///
/// * `preview` - the key of a place to show in the scene instead of the
///   investigator's own; an unknown key is ignored.
///
/// # Errors
///
/// Fails if the template fails.
pub(crate) fn game_page(
    data: &GameData,
    session: &Session,
    layout: Layout,
    preview: Option<&str>,
) -> Result<String, askama::Error> {
    let id = session.id;
    let decision = session.decision();
    let outcome = &session.outcome;
    let answered_here = decision.filter(|d| !on_sheet(d) && !session.in_combat());
    let page = inspect(data, &outcome.state, |view| {
        let tokens = BoardView::new(view, answered_here, session.mode());
        let request = SceneRequest {
            game: id,
            seat: session.player_seat(),
            preview: preview.and_then(|key| SceneSubject::from_key(data, key)),
            foe: foe(session),
            decision: answered_here,
            drawn: drawn_location_cards(session),
        };
        GamePage {
            htmx: layout.htmx,
            ambiance: layout.ambiance,
            root: layout.root,
            id,
            seed: session.progress.setup.seed,
            map: MapView::new(view, &tokens, id),
            scene: SceneView::new(view, &tokens, request),
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
        }
    });
    page.render()
}

/// The monster the player faces now: the first one offered to evade or
/// fight, or the foe of the combat under way.
fn foe(session: &Session) -> Option<MonsterId> {
    if !session.in_combat() {
        return None;
    }
    if let Some(decision) = session.decision()
        && let Context::Monsters(monsters) = &decision.context
    {
        return monsters.first().cloned();
    }
    session
        .combat()
        .filter(|span| span.ongoing())
        .map(|span| span.foe)
}

/// The location cards the player drew since their last answer.
fn drawn_location_cards(session: &Session) -> Vec<CardId> {
    let seat = session.player_seat();
    session
        .outcome
        .shown
        .iter()
        .filter(|shown| shown.line >= session.progress.seen_events)
        .filter_map(|shown| match &shown.what {
            Exhibit::Card {
                deck: Deck::Location,
                card,
                seat: drawer,
            } if *drawer == seat => Some(card.clone()),
            _ => None,
        })
        .collect()
}

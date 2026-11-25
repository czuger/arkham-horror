use std::collections::BTreeMap;

use askama::Template;
use axum::Form;
use axum::extract::{Path, State};
use axum::http::Uri;
use axum::response::{Html, IntoResponse, Redirect, Response};
use data::CardId;
use engine::inspect;

use crate::ambiance::{Ambiance, Screen};
use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::relative_path::{redirect_from, root_of};
use crate::session::Session;
use crate::trade_moves::TradePlan;
use crate::views::{TradeView, is_trade};

#[derive(Template)]
#[template(path = "trade.html")]
struct TradePage {
    htmx: bool,
    ambiance: Ambiance,
    root: String,
    id: i64,
    trade: TradeView,
    /// The other investigator is played by the AI, which always accepts.
    partner_ai: bool,
    can_undo: bool,
}

/// The trade page: the items and money of both investigators, to pick at
/// once. Without a trade under way, goes to the page where the current
/// decision is answered.
///
/// # Errors
///
/// [`AppError::NotFound`] for an unknown game; fails if the replay or the
/// template fails.
pub async fn trade(
    State(state): State<AppState>,
    uri: Uri,
    Path(id): Path<i64>,
) -> Result<Response, AppError> {
    let session = Session::load(&state.db, &state.data, id).await?;
    let ambiance = state.ambiance(&session, Screen::Other);
    let Some(decision) = session.decision().filter(|d| is_trade(d)) else {
        return Ok(redirect_from(&uri, &session.page()).into_response());
    };
    let page = inspect(&state.data, &session.outcome.state, |view| {
        let trade = TradeView::new(view, decision, session.move_number())?;
        let partner = match &decision.context {
            engine::Context::Trade(offer) => offer.partner,
            _ => return None,
        };
        Some(TradePage {
            htmx: state.htmx,
            ambiance,
            root: root_of(uri.path()),
            id,
            trade,
            partner_ai: !session.mode().human(partner),
            can_undo: !session.progress.human_moves.is_empty(),
        })
    });
    Ok(Html(page.ok_or(AppError::NotFound)?.render()?).into_response())
}

/// Proposes the trade picked (or cancels it), lets the other investigator
/// and the AI play up to the human's next decision, saves, and goes to the
/// page where it is answered.
///
/// # Errors
///
/// [`AppError::BadRequest`] for a malformed form or an item or amount that
/// cannot be traded; fails if the replay or the database fails.
pub async fn propose_trade(
    State(state): State<AppState>,
    uri: Uri,
    Path(id): Path<i64>,
    Form(form): Form<BTreeMap<String, String>>,
) -> Result<Redirect, AppError> {
    let mut session = Session::load(&state.db, &state.data, id).await?;
    let move_number = number(&form, "coup")?;
    let plan = (form.get("action").map(String::as_str) != Some("annuler"))
        .then(|| plan(&form))
        .transpose()?;
    if session.trade(&state.data, plan.as_ref(), move_number)? {
        session.save(&state.db, &state.data).await?;
    }
    Ok(redirect_from(&uri, &session.page()))
}

/// The items ticked (`donner…`, `prendre…`) and the amounts typed.
fn plan(form: &BTreeMap<String, String>) -> Result<TradePlan, AppError> {
    let ticked = |prefix: &str| {
        form.iter()
            .filter(|(key, _)| key.starts_with(prefix))
            .map(|(_, id)| CardId::new(id.as_str()))
            .collect()
    };
    Ok(TradePlan {
        give: ticked("donner"),
        take: ticked("prendre"),
        give_money: amount(form, "argent_donne")?,
        take_money: amount(form, "argent_pris")?,
    })
}

/// A number field; an empty or missing amount is 0.
fn amount(form: &BTreeMap<String, String>, key: &str) -> Result<u16, AppError> {
    match form.get(key).map(|text| text.trim()) {
        None | Some("") => Ok(0),
        Some(text) => text
            .parse()
            .map_err(|_| AppError::BadRequest(format!("Montant illisible : {text}"))),
    }
}

fn number(form: &BTreeMap<String, String>, key: &str) -> Result<usize, AppError> {
    form.get(key)
        .and_then(|text| text.parse().ok())
        .ok_or_else(|| AppError::BadRequest(format!("Champ manquant : {key}")))
}

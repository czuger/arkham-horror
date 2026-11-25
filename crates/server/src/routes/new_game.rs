use std::time::{SystemTime, UNIX_EPOCH};

use askama::Template;
use axum::Form;
use axum::extract::State;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::response::{Html, IntoResponse, Response};
use data::{AncientOneId, GameData, InvestigatorId};
use engine::Setup;
use serde::Deserialize;

use crate::ambiance::Ambiance;
use crate::app_error::AppError;
use crate::app_state::AppState;
use crate::play_mode::PlayMode;
use crate::relative_path::{redirect_from, root_of};
use crate::session::Session;

/// Most AI investigators in a game (8 investigators in all).
const MAX_AIS: usize = 7;
/// AI investigators proposed by default.
const DEFAULT_AIS: usize = 2;

#[derive(Template)]
#[template(path = "new_game.html")]
struct NewGamePage {
    htmx: bool,
    ambiance: Ambiance,
    root: String,
    investigators: Vec<(String, String)>,
    ancient_ones: Vec<(String, String)>,
    ai_counts: Vec<usize>,
    default_ais: usize,
    error: String,
}

/// The « Nouvelle partie » form, as posted.
#[derive(Debug, Deserialize)]
pub struct NewGameForm {
    investigateur: String,
    ia: usize,
    #[serde(default)]
    grand_ancien: String,
    #[serde(default)]
    graine: String,
    /// Ticked: the human plays every investigator (hot seat).
    #[serde(default)]
    tous: Option<String>,
    #[serde(default)]
    ia1: String,
    #[serde(default)]
    ia2: String,
    #[serde(default)]
    ia3: String,
    #[serde(default)]
    ia4: String,
    #[serde(default)]
    ia5: String,
    #[serde(default)]
    ia6: String,
    #[serde(default)]
    ia7: String,
}

/// The « Nouvelle partie » page.
///
/// # Errors
///
/// Fails if the template fails.
pub async fn new_game_form(
    State(state): State<AppState>,
    uri: Uri,
) -> Result<Html<String>, AppError> {
    Ok(Html(form_page(&state, &uri, String::new())?))
}

/// Creates a game from the form, lets the AI play up to the human's first
/// decision, and goes to the page where it is answered.
///
/// # Errors
///
/// Fails if the database, the engine or the template fails. A form that
/// makes no sense is shown again with the reason.
pub async fn create_game(
    State(state): State<AppState>,
    uri: Uri,
    Form(form): Form<NewGameForm>,
) -> Result<Response, AppError> {
    let setup = match setup_from(&state.data, &form) {
        Ok(setup) => setup,
        Err(reason) => {
            let page = form_page(&state, &uri, reason)?;
            return Ok((StatusCode::BAD_REQUEST, Html(page)).into_response());
        }
    };
    let mode = if form.tous.is_some() {
        PlayMode::HotSeat
    } else {
        PlayMode::Solo
    };
    let session = Session::start(&state.db, &state.data, setup, mode).await?;
    Ok(redirect_from(&uri, &session.page()).into_response())
}

/// Renders the form, with an error line when not empty.
fn form_page(state: &AppState, uri: &Uri, error: String) -> Result<String, AppError> {
    let data = &state.data;
    Ok(NewGamePage {
        htmx: state.htmx,
        ambiance: Ambiance::Default,
        root: root_of(uri.path()),
        investigators: data
            .investigators
            .investigators
            .iter()
            .map(|sheet| {
                (
                    sheet.id.to_string(),
                    format!("{}, {}", sheet.name, sheet.title),
                )
            })
            .collect(),
        ancient_ones: data
            .ancient_ones
            .ancient_ones
            .iter()
            .map(|sheet| (sheet.id.to_string(), sheet.name.clone()))
            .collect(),
        ai_counts: (0..=MAX_AIS).collect(),
        default_ais: DEFAULT_AIS,
        error,
    }
    .render()?)
}

/// The engine setup the form asks for, or why it makes no sense.
fn setup_from(data: &GameData, form: &NewGameForm) -> Result<Setup, String> {
    if form.ia > MAX_AIS {
        return Err(format!("Au plus {MAX_AIS} investigateurs IA."));
    }
    let ai_choices = [
        &form.ia1, &form.ia2, &form.ia3, &form.ia4, &form.ia5, &form.ia6, &form.ia7,
    ];
    let mut investigators = vec![known_investigator(data, &form.investigateur)?];
    for choice in ai_choices.iter().take(form.ia).filter(|id| !id.is_empty()) {
        let id = known_investigator(data, choice)?;
        if investigators.contains(&id) {
            return Err(format!(
                "{} est choisi deux fois.",
                crate::views::investigator_name(data, &id)
            ));
        }
        investigators.push(id);
    }
    Ok(Setup {
        seed: seed(&form.graine)?,
        players: form.ia + 1,
        investigators,
        ancient_one: ancient_one(data, &form.grand_ancien)?,
    })
}

/// An investigator id from the form, checked against the data.
fn known_investigator(data: &GameData, id: &str) -> Result<InvestigatorId, String> {
    let id = InvestigatorId::new(id);
    crate::views::sheet(data, &id)
        .map(|_| id)
        .ok_or_else(|| "Investigateur inconnu.".to_owned())
}

/// The chosen Ancient One, or `None` for « au hasard ».
fn ancient_one(data: &GameData, id: &str) -> Result<Option<AncientOneId>, String> {
    if id.is_empty() {
        return Ok(None);
    }
    let id = AncientOneId::new(id);
    if data
        .ancient_ones
        .ancient_ones
        .iter()
        .any(|sheet| sheet.id == id)
    {
        Ok(Some(id))
    } else {
        Err("Grand Ancien inconnu.".to_owned())
    }
}

/// The seed typed in, or one taken from the clock.
fn seed(text: &str) -> Result<u64, String> {
    let text = text.trim();
    if text.is_empty() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_nanos());
        // Keeping the low 64 bits is enough for a seed.
        return Ok(nanos as u64);
    }
    text.parse()
        .map_err(|_| "La graine doit être un nombre entier positif.".to_owned())
}

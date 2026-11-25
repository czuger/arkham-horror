//! The scene: a large illustration of where the player's investigator is,
//! with what stands there, the destinations offered, and the location cards
//! just drawn.

use data::{CardId, GameData, MonsterId, SpecialAreaKind};
use engine::{Area, Choice, Decision, MonsterPlace, Place, View};

use crate::illustrations::{IllustrationSize, illustration_url};
use crate::views::board_view::{BoardView, place_key, special_key};
use crate::views::image_token::ImageToken;
use crate::views::names::{ancient_one, card};
use crate::views::scene_subject::SceneSubject;
use crate::views::special_illustration::special_illustration;
use crate::views::spot::{InvestigatorToken, Spot};
use crate::views::tip::Tip;
use crate::views::tips;
use crate::views::world_scene::world_illustration;

/// The scene of the game page.
#[derive(Debug, Clone)]
pub struct SceneView {
    /// The game, for the « back to my position » link.
    pub game: i64,
    /// Name of the place (or of the monster, or of the case shown).
    pub title: String,
    /// Second line: the district of a location, the area of an Other World…
    pub subtitle: Option<String>,
    /// Link to the large illustration; `None` when there is none.
    pub image: Option<String>,
    /// An image under `assets/` for a case without illustration (the
    /// Ancient One's sheet in the final battle).
    pub fallback_image: Option<String>,
    /// Alternative text of the picture.
    pub alt: String,
    /// A preview asked from the map, not the investigator's own position.
    pub preview: bool,
    /// Investigators standing there.
    pub investigators: Vec<InvestigatorToken>,
    /// Monsters standing there.
    pub monsters: Vec<SceneMonster>,
    /// The open gate.
    pub gate: Option<ImageToken>,
    /// Clue tokens.
    pub clues: u8,
    /// Sealed by an Elder Sign.
    pub sealed: bool,
    /// The « Lieu fermé » marker.
    pub closed: Option<ImageToken>,
    /// Places the current decision offers, to pick by their picture.
    pub destinations: Vec<Destination>,
    /// Location cards the player drew since their last answer.
    pub drawn: Vec<ImageToken>,
}

/// A monster in the scene: its illustrated portrait and its bubble.
#[derive(Debug, Clone)]
pub struct SceneMonster {
    /// Name.
    pub name: String,
    /// Link to its portrait thumbnail.
    pub portrait: String,
    /// Current values and abilities.
    pub tip: Tip,
}

/// A place offered by the decision, shown by its picture.
#[derive(Debug, Clone)]
pub struct Destination {
    /// Index of the answer.
    pub index: usize,
    /// Label of the answer.
    pub label: String,
    /// Name of the place.
    pub name: String,
    /// Link to its thumbnail, if the place has an illustration.
    pub picture: Option<String>,
}

/// What the scene needs besides the game state.
#[derive(Debug, Clone)]
pub struct SceneRequest<'d> {
    /// The game id.
    pub game: i64,
    /// The player's seat.
    pub seat: usize,
    /// The place asked by `?voir=`, if any.
    pub preview: Option<SceneSubject>,
    /// The monster the player faces now, if any.
    pub foe: Option<MonsterId>,
    /// The decision answered on the game page, if any.
    pub decision: Option<&'d Decision>,
    /// Location cards the player drew since their last answer.
    pub drawn: Vec<CardId>,
}

/// Title, subtitle, illustration, fallback image and alt text of a subject.
struct Picture {
    title: String,
    subtitle: Option<String>,
    image: Option<String>,
    fallback_image: Option<String>,
}

impl SceneView {
    /// The scene for a state.
    pub fn new(view: &View<'_, '_>, board: &BoardView, request: SceneRequest<'_>) -> Self {
        let preview = request.preview.is_some();
        let subject = subject(view, &request);
        let picture = picture(view, request.seat, &subject);
        let spot = spot(view, board, request.seat, &subject);
        let own_place = !preview && matches!(subject, SceneSubject::Place(_));
        let drawn = if own_place {
            request
                .drawn
                .iter()
                .map(|id| card(view.data(), id))
                .collect()
        } else {
            Vec::new()
        };
        Self {
            game: request.game,
            alt: picture.title.clone(),
            title: picture.title,
            subtitle: picture.subtitle,
            image: picture.image,
            fallback_image: picture.fallback_image,
            preview,
            investigators: spot.investigators,
            monsters: monsters(view, request.seat, &subject),
            gate: spot.gate,
            clues: spot.clues,
            sealed: spot.sealed,
            closed: spot.closed,
            destinations: request
                .decision
                .map(|d| destinations(view, request.seat, d))
                .unwrap_or_default(),
            drawn,
        }
    }
}

/// The preview asked, else the final battle, the monster faced, or the
/// investigator's place.
fn subject(view: &View<'_, '_>, request: &SceneRequest<'_>) -> SceneSubject {
    if let Some(preview) = &request.preview {
        return preview.clone();
    }
    let state = view.state();
    if state.battle.is_some() {
        return SceneSubject::AncientOne;
    }
    if let Some(foe) = &request.foe {
        return SceneSubject::Monster(foe.clone());
    }
    match state.investigators.get(request.seat) {
        Some(inv) if !inv.eliminated => SceneSubject::Place(inv.place.clone()),
        _ => SceneSubject::Devoured,
    }
}

/// What the scene shows for a subject.
fn picture(view: &View<'_, '_>, seat: usize, subject: &SceneSubject) -> Picture {
    let data = view.data();
    let bare = |title: String, subtitle: Option<String>| Picture {
        title,
        subtitle,
        image: None,
        fallback_image: None,
    };
    match subject {
        SceneSubject::Place(place) => place_picture(view, seat, place),
        SceneSubject::Sky => bare(special_name(data, data::SpecialAreaKind::Sky), None),
        SceneSubject::Outskirts => bare(special_name(data, data::SpecialAreaKind::Outskirts), None),
        SceneSubject::Monster(id) => monster_picture(view, seat, id),
        SceneSubject::AncientOne => {
            let sheet = ancient_one(data, &view.state().ancient_one);
            Picture {
                title: sheet.map_or_else(|| "Grand Ancien".to_owned(), |s| s.name.clone()),
                subtitle: Some("Bataille finale".to_owned()),
                image: None,
                fallback_image: sheet.map(|s| s.image.clone()),
            }
        }
        SceneSubject::Devoured => bare(
            "Dévoré".to_owned(),
            Some("Votre investigateur a quitté la partie.".to_owned()),
        ),
    }
}

/// A place: its illustration, or its name only.
fn place_picture(view: &View<'_, '_>, seat: usize, place: &Place) -> Picture {
    let data = view.data();
    let board = &data.board;
    let large = |path: &str| Some(illustration_url(IllustrationSize::Large, path));
    let (subtitle, image) = match place {
        Place::Location(id) => match board.location(id) {
            Some(l) => (
                board.street(&l.street).map(|s| s.name.clone()),
                large(&l.illustration),
            ),
            None => (None, None),
        },
        Place::Street(id) => (
            Some("Rues du quartier".to_owned()),
            board.street(id).and_then(|s| large(&s.illustration)),
        ),
        Place::OtherWorld { world, area } => (
            Some(area_label(*area).to_owned()),
            world_illustration(view, seat, world, *area).and_then(|path| large(&path)),
        ),
        Place::LostInTimeAndSpace => (
            None,
            special_illustration(view, SpecialAreaKind::LostInTimeAndSpace)
                .and_then(|path| large(&path)),
        ),
        Place::JailCell => (
            None,
            special_illustration(view, SpecialAreaKind::JailCell).and_then(|path| large(&path)),
        ),
    };
    Picture {
        title: view.place_name(place),
        subtitle,
        image,
        fallback_image: None,
    }
}

/// « Autre Monde, 1re zone » or « …, 2e zone ».
pub fn area_label(area: Area) -> &'static str {
    match area {
        Area::First => "Autre Monde, 1re zone",
        Area::Second => "Autre Monde, 2e zone",
    }
}

/// The monster faced: its portrait, and where the fight takes place.
fn monster_picture(view: &View<'_, '_>, seat: usize, id: &MonsterId) -> Picture {
    let monster = view.data().monsters.monsters.iter().find(|m| &m.id == id);
    let place = view
        .state()
        .investigators
        .get(seat)
        .map(|inv| view.place_name(&inv.place));
    Picture {
        title: monster.map_or_else(|| id.to_string(), |m| m.name.clone()),
        subtitle: place.map(|place| format!("Face à vous, à {place}")),
        image: monster.map(|m| illustration_url(IllustrationSize::Large, &m.portrait)),
        fallback_image: None,
    }
}

fn special_name(data: &GameData, kind: data::SpecialAreaKind) -> String {
    data.board
        .special_area(kind)
        .map_or_else(|| special_key(kind).to_owned(), |area| area.name.clone())
}

/// The board spot of the place shown (the investigator's own place when the
/// scene shows a monster).
fn spot(view: &View<'_, '_>, board: &BoardView, seat: usize, subject: &SceneSubject) -> Spot {
    let data = view.data();
    let key = match subject {
        SceneSubject::Place(place) => place_key(data, place),
        SceneSubject::Sky => Some(special_key(data::SpecialAreaKind::Sky).to_owned()),
        SceneSubject::Outskirts => Some(special_key(data::SpecialAreaKind::Outskirts).to_owned()),
        SceneSubject::Monster(_) => view
            .state()
            .investigators
            .get(seat)
            .and_then(|inv| place_key(data, &inv.place)),
        SceneSubject::AncientOne | SceneSubject::Devoured => None,
    };
    key.and_then(|key| board.spots.iter().find(|spot| spot.key == key).cloned())
        .unwrap_or_default()
}

/// The monsters standing where the scene is, with their portraits.
fn monsters(view: &View<'_, '_>, seat: usize, subject: &SceneSubject) -> Vec<SceneMonster> {
    let here = |place: &MonsterPlace| match subject {
        SceneSubject::Place(shown) => place.as_place().as_ref() == Some(shown),
        SceneSubject::Sky => *place == MonsterPlace::Sky,
        SceneSubject::Outskirts => *place == MonsterPlace::Outskirts,
        SceneSubject::Monster(_) => {
            let own = view.state().investigators.get(seat).map(|inv| &inv.place);
            own.is_some() && place.as_place().as_ref() == own
        }
        SceneSubject::AncientOne | SceneSubject::Devoured => false,
    };
    view.state()
        .monsters
        .iter()
        .filter(|board_monster| here(&board_monster.place))
        .filter_map(|board_monster| scene_monster(view, &board_monster.monster))
        .collect()
}

fn scene_monster(view: &View<'_, '_>, id: &MonsterId) -> Option<SceneMonster> {
    let monster = view.data().monsters.monsters.iter().find(|m| &m.id == id)?;
    Some(SceneMonster {
        name: monster.name.clone(),
        portrait: illustration_url(IllustrationSize::Thumbnail, &monster.portrait),
        tip: tips::monster(view, monster),
    })
}

/// The places the decision offers, each with its thumbnail.
fn destinations(view: &View<'_, '_>, seat: usize, decision: &Decision) -> Vec<Destination> {
    decision
        .options
        .iter()
        .enumerate()
        .filter_map(|(index, option)| {
            let place = match &option.choice {
                Choice::Place(place) => place.clone(),
                Choice::Gate(id) => Place::Location(id.clone()),
                _ => return None,
            };
            Some(Destination {
                index,
                label: option.label.clone(),
                name: view.place_name(&place),
                picture: thumbnail(view, seat, &place),
            })
        })
        .collect()
}

/// The thumbnail of a place, if it has an illustration.
fn thumbnail(view: &View<'_, '_>, seat: usize, place: &Place) -> Option<String> {
    let data = view.data();
    let board = &data.board;
    let path = match place {
        Place::Location(id) => board.location(id)?.illustration.clone(),
        Place::Street(id) => board.street(id)?.illustration.clone(),
        Place::OtherWorld { world, area } => world_illustration(view, seat, world, *area)?,
        Place::LostInTimeAndSpace => {
            special_illustration(view, SpecialAreaKind::LostInTimeAndSpace)?
        }
        Place::JailCell => special_illustration(view, SpecialAreaKind::JailCell)?,
    };
    Some(illustration_url(IllustrationSize::Thumbnail, &path))
}

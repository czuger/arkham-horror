use std::collections::BTreeMap;

use data::{GameData, SpecialAreaKind};
use engine::{Area, Choice, Decision, GameState, MonsterPlace, Place, View};

use crate::play_mode::PlayMode;
use crate::views::closed_marker::closed_marker;
use crate::views::image_token::ImageToken;
use crate::views::names::{initials, investigator_name, monster_on_board};
use crate::views::spot::{InvestigatorToken, Spot};
use crate::views::tips;

/// Every token on the board and the places the current decision offers,
/// place by place.
#[derive(Debug, Clone)]
pub struct BoardView {
    /// The places with something to show.
    pub spots: Vec<Spot>,
}

impl BoardView {
    /// The tokens of a state, with the decision's place choices.
    pub fn new(view: &View<'_, '_>, decision: Option<&Decision>, mode: PlayMode) -> Self {
        let data = view.data();
        let state = view.state();
        let mut spots: BTreeMap<String, Spot> = BTreeMap::new();
        add_investigators(view, decision, mode, &mut spots);
        add_monsters(view, &mut spots);
        add_locations(data, state, &mut spots);
        add_closed(view, &mut spots);
        if let Some(decision) = decision {
            add_choices(data, decision, &mut spots);
        }
        Self {
            spots: spots
                .into_values()
                .filter(|spot| !spot.is_empty())
                .collect(),
        }
    }
}

/// The spot of a place, created on first use.
fn spot_of<'s>(
    data: &GameData,
    spots: &'s mut BTreeMap<String, Spot>,
    place: &Place,
) -> Option<&'s mut Spot> {
    let (key, name) = locate(data, place)?;
    Some(spots.entry(key.clone()).or_insert_with(|| blank(key, name)))
}

/// An empty spot.
fn blank(key: String, name: String) -> Spot {
    Spot {
        key,
        name,
        ..Spot::default()
    }
}

/// Key and French name of a place. The key is
/// the place's id (`woods`, `uptown`), `<world>` or `<world>_2` for the two
/// areas of an Other World, or a special area's key: the preview link
/// `?voir=<key>` uses it.
fn locate(data: &GameData, place: &Place) -> Option<(String, String)> {
    let board = &data.board;
    match place {
        Place::Location(id) => board.location(id).map(|l| (id.to_string(), l.name.clone())),
        Place::Street(id) => board.street(id).map(|s| (id.to_string(), s.name.clone())),
        Place::OtherWorld { world, area } => board.other_world(world).map(|w| {
            let key = match area {
                Area::First => world.to_string(),
                Area::Second => format!("{world}_2"),
            };
            (key, w.name.clone())
        }),
        Place::LostInTimeAndSpace => special(data, SpecialAreaKind::LostInTimeAndSpace),
        Place::JailCell => special(data, SpecialAreaKind::JailCell),
    }
}

/// The key of a place, as written in `?voir=` (see [`locate`]).
pub fn place_key(data: &GameData, place: &Place) -> Option<String> {
    locate(data, place).map(|(key, _)| key)
}

/// The key of a special area, as written in `?voir=`.
pub fn special_key(kind: SpecialAreaKind) -> &'static str {
    match kind {
        SpecialAreaKind::Outskirts => "outskirts",
        SpecialAreaKind::Sky => "sky",
        SpecialAreaKind::LostInTimeAndSpace => "lost_in_time_and_space",
        SpecialAreaKind::JailCell => "jail_cell",
    }
}

/// Key and name of a special area.
fn special(data: &GameData, kind: SpecialAreaKind) -> Option<(String, String)> {
    data.board
        .special_area(kind)
        .map(|area| (special_key(kind).to_owned(), area.name.clone()))
}

/// Every investigator still in the game.
fn add_investigators(
    view: &View<'_, '_>,
    decision: Option<&Decision>,
    mode: PlayMode,
    spots: &mut BTreeMap<String, Spot>,
) {
    let data = view.data();
    for (seat, inv) in view.state().investigators.iter().enumerate() {
        if inv.eliminated {
            continue;
        }
        let name = investigator_name(data, &inv.id);
        if let Some(spot) = spot_of(data, spots, &inv.place) {
            spot.investigators.push(InvestigatorToken {
                initials: initials(&name),
                tip: tips::investigator(view, seat),
                human: mode.highlighted(seat, decision),
            });
        }
    }
}

/// Monsters in Arkham, in the Sky and in the Outskirts.
fn add_monsters(view: &View<'_, '_>, spots: &mut BTreeMap<String, Spot>) {
    let data = view.data();
    for board_monster in &view.state().monsters {
        let spot = match &board_monster.place {
            MonsterPlace::Sky => special_spot(data, spots, SpecialAreaKind::Sky),
            MonsterPlace::Outskirts => special_spot(data, spots, SpecialAreaKind::Outskirts),
            other => other
                .as_place()
                .and_then(|place| spot_of(data, spots, &place)),
        };
        if let (Some(spot), Some(token)) = (spot, monster_on_board(view, &board_monster.monster)) {
            spot.monsters.push(token);
        }
    }
}

/// The spot of the Sky or the Outskirts.
fn special_spot<'s>(
    data: &GameData,
    spots: &'s mut BTreeMap<String, Spot>,
    kind: SpecialAreaKind,
) -> Option<&'s mut Spot> {
    let (key, name) = special(data, kind)?;
    Some(spots.entry(key.clone()).or_insert_with(|| blank(key, name)))
}

/// Gates, clue tokens and seals.
fn add_locations(data: &GameData, state: &GameState, spots: &mut BTreeMap<String, Spot>) {
    for gate in &state.gates {
        let Some(spot) = spot_of(data, spots, &Place::Location(gate.location.clone())) else {
            continue;
        };
        spot.gate = data.gate_markers.markers.get(gate.marker).map(|marker| {
            let world = tips::world_name(data, &marker.world);
            ImageToken {
                name: format!("Portail vers {world}"),
                image: marker.image.clone(),
                tip: tips::gate(&world, &spot.name, marker.close_modifier),
            }
        });
    }
    for (id, count) in &state.clues {
        if let Some(spot) = spot_of(data, spots, &Place::Location(id.clone())) {
            spot.clues = *count;
        }
    }
    for id in &state.seals {
        if let Some(spot) = spot_of(data, spots, &Place::Location(id.clone())) {
            spot.sealed = true;
        }
    }
}

/// The « Lieu fermé » marker on each closed location.
fn add_closed(view: &View<'_, '_>, spots: &mut BTreeMap<String, Spot>) {
    let data = view.data();
    for location in &data.board.locations {
        if let Some(marker) = closed_marker(view, &location.id)
            && let Some(spot) = spot_of(data, spots, &Place::Location(location.id.clone()))
        {
            spot.closed = Some(marker);
        }
    }
}

/// Answers that pick a place or a gate become buttons on the board.
fn add_choices(data: &GameData, decision: &Decision, spots: &mut BTreeMap<String, Spot>) {
    for (index, option) in decision.options.iter().enumerate() {
        let place = match &option.choice {
            Choice::Place(place) => place.clone(),
            Choice::Gate(id) => Place::Location(id.clone()),
            _ => continue,
        };
        if let Some(spot) = spot_of(data, spots, &place) {
            spot.choices.push((index, option.label.clone()));
        }
    }
}

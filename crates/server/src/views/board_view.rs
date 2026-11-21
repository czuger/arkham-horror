use std::collections::BTreeMap;

use data::{GameData, Position, SpecialAreaKind};
use engine::{Area, Choice, Decision, GameState, MonsterPlace, Place};

use crate::session::HUMAN_SEAT;
use crate::views::names::{initials, investigator_name, monster};
use crate::views::spot::{ImageToken, InvestigatorToken, Spot};

/// Size in pixels of the board image the positions in `board.ron` refer to.
const BOARD_SIZE: (f64, f64) = (1064.0, 1576.0);

/// The board with every token and the places the current decision offers.
#[derive(Debug, Clone)]
pub struct BoardView {
    /// Board image path under `/assets/`.
    pub image: String,
    /// The places with something to show.
    pub spots: Vec<Spot>,
}

impl BoardView {
    /// Draws the board for a state, with the decision's place choices.
    pub fn new(data: &GameData, state: &GameState, decision: Option<&Decision>) -> Self {
        let mut spots: BTreeMap<String, Spot> = BTreeMap::new();
        add_investigators(data, state, &mut spots);
        add_monsters(data, state, &mut spots);
        add_locations(data, state, &mut spots);
        if let Some(decision) = decision {
            add_choices(data, decision, &mut spots);
        }
        Self {
            image: data.board.image.clone(),
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
    let (key, name, position) = locate(data, place)?;
    Some(spots.entry(key).or_insert_with(|| blank(name, position)))
}

/// An empty spot at a position.
fn blank(name: String, position: Position) -> Spot {
    Spot {
        name,
        left: percent(position.x, BOARD_SIZE.0),
        top: percent(position.y, BOARD_SIZE.1),
        ..Spot::default()
    }
}

/// A pixel coordinate as a CSS percentage of the board size.
fn percent(pixels: u16, size: f64) -> String {
    format!("{:.2}%", f64::from(pixels) * 100.0 / size)
}

/// Key, French name and position of a place on the board image.
fn locate(data: &GameData, place: &Place) -> Option<(String, String, Position)> {
    let board = &data.board;
    match place {
        Place::Location(id) => board
            .location(id)
            .map(|l| (format!("l:{id}"), l.name.clone(), l.position)),
        Place::Street(id) => board
            .street(id)
            .map(|s| (format!("s:{id}"), s.name.clone(), s.position)),
        Place::OtherWorld { world, area } => board.other_world(world).map(|w| {
            let (rank, position) = match area {
                Area::First => (1, w.first_area),
                Area::Second => (2, w.second_area),
            };
            (format!("w:{world}:{rank}"), w.name.clone(), position)
        }),
        Place::LostInTimeAndSpace => special(data, SpecialAreaKind::LostInTimeAndSpace),
        Place::JailCell => special(data, SpecialAreaKind::JailCell),
    }
}

/// Key, name and position of a special area.
fn special(data: &GameData, kind: SpecialAreaKind) -> Option<(String, String, Position)> {
    data.board
        .special_area(kind)
        .map(|area| (format!("x:{kind:?}"), area.name.clone(), area.position))
}

/// Every investigator still in the game.
fn add_investigators(data: &GameData, state: &GameState, spots: &mut BTreeMap<String, Spot>) {
    for (seat, inv) in state.investigators.iter().enumerate() {
        if inv.eliminated {
            continue;
        }
        let name = investigator_name(data, &inv.id);
        if let Some(spot) = spot_of(data, spots, &inv.place) {
            spot.investigators.push(InvestigatorToken {
                initials: initials(&name),
                name,
                human: seat == HUMAN_SEAT,
            });
        }
    }
}

/// Monsters in Arkham, in the Sky and in the Outskirts.
fn add_monsters(data: &GameData, state: &GameState, spots: &mut BTreeMap<String, Spot>) {
    for board_monster in &state.monsters {
        let spot = match &board_monster.place {
            MonsterPlace::Sky => special_spot(data, spots, SpecialAreaKind::Sky),
            MonsterPlace::Outskirts => special_spot(data, spots, SpecialAreaKind::Outskirts),
            other => other
                .as_place()
                .and_then(|place| spot_of(data, spots, &place)),
        };
        if let (Some(spot), Some(token)) = (spot, monster(data, &board_monster.monster)) {
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
    let (key, name, position) = special(data, kind)?;
    Some(spots.entry(key).or_insert_with(|| blank(name, position)))
}

/// Gates, clue tokens, seals and closed locations.
fn add_locations(data: &GameData, state: &GameState, spots: &mut BTreeMap<String, Spot>) {
    for gate in &state.gates {
        let marker = data.gate_markers.markers.get(gate.marker);
        let token = marker.map(|marker| ImageToken {
            name: format!(
                "Portail vers {}",
                data.board
                    .other_world(&marker.world)
                    .map_or("?", |world| world.name.as_str())
            ),
            image: marker.image.clone(),
        });
        if let Some(spot) = spot_of(data, spots, &Place::Location(gate.location.clone())) {
            spot.gate = token;
        }
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
    for id in &state.closed {
        if let Some(spot) = spot_of(data, spots, &Place::Location(id.clone())) {
            spot.closed = true;
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

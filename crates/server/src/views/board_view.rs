use std::collections::BTreeMap;

use data::{GameData, Position, SpecialAreaKind};
use engine::{Area, Choice, Decision, GameState, MonsterPlace, Place, View};

use crate::session::HUMAN_SEAT;
use crate::views::board_geometry::{BOARD_SIZE, edge, percent};
use crate::views::closed_marker::closed_marker;
use crate::views::image_token::ImageToken;
use crate::views::location_panel::LocationPanel;
use crate::views::names::{initials, investigator_name, monster_on_board};
use crate::views::spot::{InvestigatorToken, Spot};
use crate::views::terror_pawn::TerrorPawn;
use crate::views::tips;

/// The board with every token and the places the current decision offers.
#[derive(Debug, Clone)]
pub struct BoardView {
    /// Board image path under `/assets/`.
    pub image: String,
    /// The places with something to show.
    pub spots: Vec<Spot>,
    /// The terror marker on its track.
    pub terror: Option<TerrorPawn>,
    /// The white area of every location, with its text and icons on hover.
    pub panels: Vec<LocationPanel>,
}

impl BoardView {
    /// Draws the board for a state, with the decision's place choices.
    pub fn new(view: &View<'_, '_>, decision: Option<&Decision>) -> Self {
        let data = view.data();
        let state = view.state();
        let mut spots: BTreeMap<String, Spot> = BTreeMap::new();
        add_investigators(view, &mut spots);
        add_monsters(view, &mut spots);
        add_locations(data, state, &mut spots);
        add_closed(view, &mut spots);
        if let Some(decision) = decision {
            add_choices(data, decision, &mut spots);
        }
        Self {
            image: data.board.image.clone(),
            spots: spots
                .into_values()
                .filter(|spot| !spot.is_empty())
                .collect(),
            terror: terror_pawn(data, state.terror),
            panels: data
                .board
                .locations
                .iter()
                .map(|location| LocationPanel::new(&data.board, location))
                .collect(),
        }
    }
}

/// The terror marker at its level on the track.
fn terror_pawn(data: &GameData, level: u8) -> Option<TerrorPawn> {
    let track = &data.board.terror_track;
    let position = track.space(level)?;
    Some(TerrorPawn {
        level,
        image: track.marker.clone(),
        left: percent(position.x, BOARD_SIZE.0),
        top: percent(position.y, BOARD_SIZE.1),
        point: (position.x, position.y),
        tip: tips::terror(data, level),
    })
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
        point: (position.x, position.y),
        edge: edge(position),
        ..Spot::default()
    }
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
fn add_investigators(view: &View<'_, '_>, spots: &mut BTreeMap<String, Spot>) {
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
                human: seat == HUMAN_SEAT,
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
    let (key, name, position) = special(data, kind)?;
    Some(spots.entry(key).or_insert_with(|| blank(name, position)))
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

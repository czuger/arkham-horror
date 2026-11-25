//! The symbolic map of Arkham: a small SVG drawing to find one's way, with
//! the places the current decision offers lit up and clickable.

use std::collections::BTreeMap;
use std::fmt;

use data::{Board, Position, Stability};
use engine::View;

use crate::views::board_geometry::{BOARD_SIZE, percent};
use crate::views::board_view::{BoardView, special_key};
use crate::views::location_tip::LocationTip;
use crate::views::map_lines::{MapArrow, MapLink, WorldFrame, arrows, links, world_frames};
use crate::views::spot::Spot;
use crate::views::tip::Tip;

/// The map: links, arrows, Other World frames and every place.
#[derive(Debug, Clone)]
pub struct MapView {
    /// The game, for the preview links.
    pub game: i64,
    /// Links between places.
    pub links: Vec<MapLink>,
    /// The monsters' arrows on the street links.
    pub arrows: Vec<MapArrow>,
    /// The frames of the Other Worlds.
    pub worlds: Vec<WorldFrame>,
    /// Every place: locations, streets, Other World areas, special areas.
    pub spaces: Vec<MapSpace>,
    /// Preview mode (« Aperçu ») on: every place shows its preview.
    pub survey: bool,
}

/// One place on the map, with symbols for what stands there.
#[derive(Debug, Clone)]
pub struct MapSpace {
    /// Its key, as in `?voir=<key>`.
    pub key: String,
    /// Full French name.
    pub name: String,
    /// Name written on the map.
    pub short_name: String,
    /// `location`, `street`, `world` or `special`: the shape and CSS class.
    pub kind: &'static str,
    /// `stable` or `unstable` for a location, empty otherwise: CSS class.
    pub stability: &'static str,
    /// Centre, x, in `board.ron` coordinates.
    pub x: i32,
    /// Centre, y.
    pub y: i32,
    /// Centre, x, in percent of the map width.
    pub left: String,
    /// Centre, y, in percent of the map height.
    pub top: String,
    /// The player's investigator stands here.
    pub here: bool,
    /// Where to draw a dot for each other investigator here.
    pub others: Vec<(i32, i32)>,
    /// Monsters here.
    pub monsters: usize,
    /// An open gate.
    pub gate: bool,
    /// Clue tokens.
    pub clues: u8,
    /// Sealed by an Elder Sign.
    pub sealed: bool,
    /// Closed.
    pub closed: bool,
    /// Answers of the current decision that pick this place: index, label.
    pub offered: Vec<(usize, String)>,
    /// The bubble: the place and what stands there.
    pub tip: SpaceTip,
}

impl MapSpace {
    /// The corners of a street's diamond, as SVG `points`.
    pub fn diamond(&self) -> String {
        let (x, y) = (self.x, self.y);
        format!(
            "{x},{} {},{y} {x},{} {},{y}",
            y - 34,
            x + 34,
            y + 34,
            x - 34
        )
    }

    /// Whether the current decision offers this place.
    pub fn is_offered(&self) -> bool {
        !self.offered.is_empty()
    }

    /// Whether the place is on the right half of the map, where its bubble
    /// opens to the left so that the map's edge does not hide it.
    pub fn east(&self) -> bool {
        f64::from(self.x) > BOARD_SIZE.0 / 2.0
    }

    /// The widest its bubble may be: the room between the place and the
    /// map's edge on the side it opens to, in units of the map's width.
    pub fn bubble_room(&self) -> String {
        let x = f64::from(self.x) * 100.0 / BOARD_SIZE.0;
        // Half a target (7 % of the map) and a small gap stand in between.
        let room = x.max(100.0 - x) - 3.5;
        format!("calc({room:.2}cqw - 10px)")
    }
}

/// The bubble of a place: a location's text and icons, or a plain bubble.
#[derive(Debug, Clone)]
pub enum SpaceTip {
    /// A location.
    Location(LocationTip),
    /// Any other place.
    Plain(Tip),
}

impl fmt::Display for SpaceTip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Location(tip) => tip.fmt(f),
            Self::Plain(tip) => tip.fmt(f),
        }
    }
}

impl MapView {
    /// The map for a state.
    ///
    /// # Arguments
    ///
    /// * `board_view` - the tokens, and the place choices of the decision
    ///   answered on this page, which light up.
    /// * `game` - the game id, for the preview links.
    pub fn new(view: &View<'_, '_>, board_view: &BoardView, game: i64) -> Self {
        let board = &view.data().board;
        let mut spots: BTreeMap<String, Spot> = board_view
            .spots
            .iter()
            .map(|spot| (spot.key.clone(), spot.clone()))
            .collect();
        let spaces = places(board)
            .into_iter()
            .map(|place| {
                let spot = spots.remove(&place.key);
                space(board, place, spot)
            })
            .collect();
        Self {
            game,
            links: links(board),
            arrows: arrows(board),
            worlds: world_frames(board),
            spaces,
            survey: false,
        }
    }

    /// The same map, in preview mode (« Aperçu ») when `survey`.
    pub fn surveying(mut self, survey: bool) -> Self {
        self.survey = survey;
        self
    }
}

/// A place of the board before its tokens are known.
#[derive(Debug, Clone)]
struct Site<'b> {
    key: String,
    kind: &'static str,
    stability: &'static str,
    name: &'b str,
    short_name: &'b str,
    position: Position,
}

/// Every place, in drawing order: streets, locations, worlds, special areas.
fn places(board: &Board) -> Vec<Site<'_>> {
    let streets = board.streets.iter().map(|s| Site {
        key: s.id.to_string(),
        kind: "street",
        stability: "",
        name: &s.name,
        short_name: &s.short_name,
        position: s.position,
    });
    let locations = board.locations.iter().map(|l| Site {
        key: l.id.to_string(),
        kind: "location",
        stability: match l.stability {
            Stability::Stable => "stable",
            Stability::Unstable => "unstable",
        },
        name: &l.name,
        short_name: &l.short_name,
        position: l.position,
    });
    let specials = board.special_areas.iter().map(|a| Site {
        key: special_key(a.kind).to_owned(),
        kind: "special",
        stability: "",
        name: &a.name,
        short_name: &a.short_name,
        position: a.position,
    });
    streets
        .chain(locations)
        .chain(world_areas(board))
        .chain(specials)
        .collect()
}

/// Both areas of every Other World.
fn world_areas(board: &Board) -> impl Iterator<Item = Site<'_>> {
    board.other_worlds.iter().flat_map(|w| {
        [
            Site {
                key: w.id.to_string(),
                kind: "world",
                stability: "",
                name: &w.name,
                short_name: "1",
                position: w.first_area,
            },
            Site {
                key: format!("{}_2", w.id),
                kind: "world",
                stability: "",
                name: &w.name,
                short_name: "2",
                position: w.second_area,
            },
        ]
    })
}

/// A place with what its spot holds.
fn space(board: &Board, place: Site<'_>, spot: Option<Spot>) -> MapSpace {
    let spot = spot.unwrap_or_default();
    let key = place.key.clone();
    let (x, y) = (i32::from(place.position.x), i32::from(place.position.y));
    let others = spot.investigators.iter().filter(|pawn| !pawn.human).count();
    MapSpace {
        name: area_name(&place),
        short_name: place.short_name.to_owned(),
        kind: place.kind,
        stability: place.stability,
        x,
        y,
        left: percent(place.position.x, BOARD_SIZE.0),
        top: percent(place.position.y, BOARD_SIZE.1),
        here: spot.investigators.iter().any(|pawn| pawn.human),
        others: (0..others)
            .map(|i| (x - 44 + 22 * i32::try_from(i).unwrap_or(0), y - 50))
            .collect(),
        monsters: spot.monsters.len(),
        gate: spot.gate.is_some(),
        clues: spot.clues,
        sealed: spot.sealed,
        closed: spot.closed.is_some(),
        offered: spot.choices.clone(),
        tip: space_tip(board, &key, &place, &spot),
        key,
    }
}

/// The name of a place, with the area of an Other World.
fn area_name(place: &Site<'_>) -> String {
    match (place.kind, place.short_name) {
        ("world", "1") => format!("{} (1re zone)", place.name),
        ("world", _) => format!("{} (2e zone)", place.name),
        _ => place.name.to_owned(),
    }
}

/// The bubble: a location's own bubble, or the name; then what stands there.
fn space_tip(board: &Board, key: &str, place: &Site<'_>, spot: &Spot) -> SpaceTip {
    let present = present(spot);
    match board.locations.iter().find(|l| l.id.as_str() == key) {
        Some(location) => {
            let mut tip = LocationTip::new(board, location);
            tip.present = present;
            SpaceTip::Location(tip)
        }
        None => SpaceTip::Plain(
            present
                .into_iter()
                .fold(Tip::new(area_name(place)), Tip::line),
        ),
    }
}

/// One line for each thing standing on a spot.
fn present(spot: &Spot) -> Vec<String> {
    let mut lines: Vec<String> = spot
        .investigators
        .iter()
        .map(|pawn| {
            if pawn.human {
                format!("{} (vous)", pawn.tip.title)
            } else {
                pawn.tip.title.clone()
            }
        })
        .collect();
    lines.extend(spot.monsters.iter().map(|monster| {
        format!(
            "Monstre : {}, danger {}",
            monster.name,
            monster.danger.gauge()
        )
    }));
    if let Some(gate) = &spot.gate {
        let modifier = gate.tip.lines.last().cloned().unwrap_or_default();
        lines.push(format!("{} ({modifier})", gate.name));
    }
    if spot.clues > 0 {
        lines.push(format!("{} Indice(s)", spot.clues));
    }
    if spot.sealed {
        lines.push("Scellé par un Signe des Anciens".to_owned());
    }
    if spot.closed.is_some() {
        lines.push("Lieu fermé".to_owned());
    }
    lines
}

//! What the scene of the game page shows.

use data::{GameData, LocationId, MonsterId, OtherWorldId, SpecialAreaKind, StreetId};
use engine::{Area, Place};

use crate::views::board_view::special_key;

/// The subject of the scene: a place, the monster faced, or a case without
/// an illustration of its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SceneSubject {
    /// A place an investigator can stand on.
    Place(Place),
    /// The Sky (preview only).
    Sky,
    /// The Outskirts (preview only).
    Outskirts,
    /// The monster the investigator faces.
    Monster(MonsterId),
    /// The final battle against the Ancient One.
    AncientOne,
    /// The investigator was devoured.
    Devoured,
}

impl SceneSubject {
    /// The subject a preview link (`?voir=<key>`) names, if the key is known:
    /// a location, a street, `<world>` or `<world>_2`, or a special area.
    pub fn from_key(data: &GameData, key: &str) -> Option<Self> {
        let board = &data.board;
        if board.location(&LocationId::new(key)).is_some() {
            return Some(Self::Place(Place::Location(LocationId::new(key))));
        }
        if board.street(&StreetId::new(key)).is_some() {
            return Some(Self::Place(Place::Street(StreetId::new(key))));
        }
        let (world, area) = match key.strip_suffix("_2") {
            Some(world) => (world, Area::Second),
            None => (key, Area::First),
        };
        if board.other_world(&OtherWorldId::new(world)).is_some() {
            let world = OtherWorldId::new(world);
            return Some(Self::Place(Place::OtherWorld { world, area }));
        }
        special(key)
    }
}

/// A special area by its key.
fn special(key: &str) -> Option<SceneSubject> {
    [
        (SpecialAreaKind::Outskirts, SceneSubject::Outskirts),
        (SpecialAreaKind::Sky, SceneSubject::Sky),
        (
            SpecialAreaKind::LostInTimeAndSpace,
            SceneSubject::Place(Place::LostInTimeAndSpace),
        ),
        (
            SpecialAreaKind::JailCell,
            SceneSubject::Place(Place::JailCell),
        ),
    ]
    .into_iter()
    .find(|(kind, _)| special_key(*kind) == key)
    .map(|(_, subject)| subject)
}

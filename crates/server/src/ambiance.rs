//! The background music the player hears: one mood per situation.

use std::fmt;

use engine::Place;
use serde::Serialize;

use crate::session::Session;

/// A music mood; each has its own folder of tracks.
///
/// Paths to music files are always built from this enum, never from a
/// string sent by the browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Ambiance {
    /// The rest of the time.
    Default,
    /// The player's investigator is in an Other World.
    OtherWorld,
    /// The player faces a monster.
    Combat,
}

/// The page shown, as far as the music is concerned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    /// The combat page, end screen included.
    Combat,
    /// The pause page after an Other World encounter.
    OtherWorld,
    /// Any other page of a game.
    Other,
}

impl Ambiance {
    /// Every mood.
    pub const ALL: [Self; 3] = [Self::Default, Self::OtherWorld, Self::Combat];

    /// The name the pages and URLs use.
    pub fn name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::OtherWorld => "other_world",
            Self::Combat => "combat",
        }
    }

    /// The folder of its tracks, inside the music folder.
    pub fn folder(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::OtherWorld => "other_worlds",
            Self::Combat => "combat",
        }
    }

    /// The mood of a game page: combat while a monster is faced or on the
    /// combat page, else Other World while the player's investigator is in
    /// one or on the encounter pause page, else the default.
    pub fn of(session: &Session, screen: Screen) -> Self {
        if screen == Screen::Combat || session.in_combat() {
            return Self::Combat;
        }
        if screen == Screen::OtherWorld || in_other_world(session) {
            return Self::OtherWorld;
        }
        Self::Default
    }
}

impl fmt::Display for Ambiance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Whether the investigator the human plays now stands in an Other World.
fn in_other_world(session: &Session) -> bool {
    session
        .outcome
        .state
        .investigators
        .get(session.player_seat())
        .is_some_and(|inv| matches!(inv.place, Place::OtherWorld { .. }))
}

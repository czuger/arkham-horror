use engine::{MonsterPlace, Status, View};

use crate::views::names::{ancient_one, card};
use crate::views::spot::ImageToken;

/// The state of the town: Ancient One, doom, terror, gates, cards in play.
#[derive(Debug, Clone)]
pub struct StatusView {
    /// The Ancient One: name and sheet image.
    pub ancient_one: ImageToken,
    /// Doom tokens, and the length of the doom track.
    pub doom: (u8, u8),
    /// Terror level.
    pub terror: u8,
    /// Turn number.
    pub turn: u32,
    /// Phase being played.
    pub phase: &'static str,
    /// Open gates.
    pub gates: usize,
    /// Elder Signs on the board.
    pub seals: usize,
    /// Monsters in Arkham and in the Sky, and the monster limit.
    pub monsters: (usize, usize),
    /// Monsters in the Outskirts.
    pub outskirts: usize,
    /// Environment card in play.
    pub environment: Option<ImageToken>,
    /// Rumor in play, with its tokens.
    pub rumor: Option<(ImageToken, u8)>,
    /// How the game ended, and whether it is a victory.
    pub ending: Option<(&'static str, bool)>,
}

impl StatusView {
    /// The status of a game.
    pub fn new(view: &View<'_, '_>, status: &Status) -> Self {
        let data = view.data();
        let state = view.state();
        let sheet = ancient_one(data, &state.ancient_one);
        let counted = state
            .monsters
            .iter()
            .filter(|monster| monster.place.counts_for_limit())
            .count();
        Self {
            ancient_one: ImageToken {
                name: sheet.map_or_else(|| state.ancient_one.to_string(), |s| s.name.clone()),
                image: sheet.map(|s| s.image.clone()).unwrap_or_default(),
            },
            doom: (state.doom, sheet.map_or(0, |s| s.doom_track)),
            terror: state.terror,
            turn: state.turn,
            phase: state.phase.label(),
            gates: state.gates.len(),
            seals: state.seals.len(),
            monsters: (counted, state.players() + 3),
            outskirts: state
                .monsters
                .iter()
                .filter(|monster| monster.place == MonsterPlace::Outskirts)
                .count(),
            environment: state.environment.as_ref().map(|id| card(data, id)),
            rumor: state
                .rumor
                .as_ref()
                .map(|rumor| (card(data, &rumor.card), rumor.tokens)),
            ending: match status {
                Status::Over(ending) => Some((ending.label(), ending.is_victory())),
                Status::Waiting(_) => None,
            },
        }
    }
}

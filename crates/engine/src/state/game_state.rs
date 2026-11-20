//! The whole state of a game.

use std::collections::BTreeMap;

use data::{AncientOneId, CardId, InvestigatorId, LocationId, MonsterId, StreetId};
use serde::{Deserialize, Serialize};

use crate::rng::Rng;
use crate::state::active_rumor::ActiveRumor;
use crate::state::battle_state::BattleState;
use crate::state::board_monster::BoardMonster;
use crate::state::decks::Decks;
use crate::state::ending::Ending;
use crate::state::investigator_state::InvestigatorState;
use crate::state::open_gate::OpenGate;
use crate::state::phase::Phase;

/// Everything that changes during a game. Serializable as is.
///
/// Maps are `BTreeMap` so that iteration order, hence replay, is stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    /// Format version of this structure.
    pub version: u32,
    /// Seed the game started from.
    pub seed: u64,
    /// The random generator, at its current position.
    pub rng: Rng,
    /// The Ancient One of the game.
    pub ancient_one: AncientOneId,
    /// The investigators in play, in seat order.
    pub investigators: Vec<InvestigatorState>,
    /// Investigator sheets not in play, available as replacements.
    pub unused_investigators: Vec<InvestigatorId>,
    /// Seat of the first player.
    pub first_player: usize,
    /// Turn number, from 1.
    pub turn: u32,
    /// Current phase.
    pub phase: Phase,
    /// Terror level, 0 to 10.
    pub terror: u8,
    /// Doom tokens on the Ancient One's track.
    pub doom: u8,
    /// Monsters in the cup.
    pub monster_cup: Vec<MonsterId>,
    /// Monsters on the board (Arkham, Sky, Outskirts).
    pub monsters: Vec<BoardMonster>,
    /// Gate marker stack, top first: indices into the gate marker list.
    pub gate_stack: Vec<usize>,
    /// Open gates.
    pub gates: Vec<OpenGate>,
    /// Locations sealed by an Elder Sign.
    pub seals: Vec<LocationId>,
    /// Clue tokens lying on locations.
    pub clues: BTreeMap<LocationId, u8>,
    /// Locations closed for good.
    pub closed: Vec<LocationId>,
    /// Investigator card decks.
    pub decks: Decks,
    /// Mythos deck, top first.
    pub mythos_deck: Vec<CardId>,
    /// Gate card deck, top first.
    pub gate_deck: Vec<CardId>,
    /// Environment card in play.
    pub environment: Option<CardId>,
    /// Rumor card in play.
    pub rumor: Option<ActiveRumor>,
    /// Headlines left in play until the end of the next turn, with whether
    /// they came out this turn.
    #[serde(default)]
    pub lingering: Vec<(CardId, bool)>,
    /// Location card set aside by the crystal ball for the next encounter in
    /// that district (R23).
    #[serde(default)]
    pub reserved_location_card: Option<(StreetId, CardId)>,
    /// Two Mythos cards are drawn each Mythos phase (Le Grand Rituel failed).
    pub two_mythos_cards: bool,
    /// Final battle progress, once the Ancient One is awake.
    pub battle: Option<BattleState>,
    /// How the game ended, once it has.
    pub ending: Option<Ending>,
}

impl GameState {
    /// Format version this code writes.
    pub const VERSION: u32 = 1;

    /// Number of players in the sense of the rules: every investigator,
    /// eliminated ones included (R2).
    pub fn players(&self) -> usize {
        self.investigators.len()
    }

    /// Seats from the first player on, eliminated investigators left out.
    pub fn seat_order(&self) -> Vec<usize> {
        let count = self.investigators.len();
        (0..count)
            .map(|offset| (self.first_player + offset) % count)
            .filter(|&seat| !self.investigators[seat].eliminated)
            .collect()
    }

    /// The open gate on a location, if any.
    pub fn gate_at(&self, location: &LocationId) -> Option<&OpenGate> {
        self.gates.iter().find(|gate| &gate.location == location)
    }

    /// Whether a location carries an Elder Sign.
    pub fn is_sealed(&self, location: &LocationId) -> bool {
        self.seals.contains(location)
    }

    /// Whether a location is closed.
    pub fn is_closed(&self, location: &LocationId) -> bool {
        self.closed.contains(location)
    }

    /// Clue tokens on a location.
    pub fn clues_at(&self, location: &LocationId) -> u8 {
        self.clues.get(location).copied().unwrap_or(0)
    }
}

//! A read-only window on the game, handed to responders.

use data::{CheckKind, GameData, LocationId, MonsterId, MonsterTrait, Resource};

use crate::decision::Waiting;
use crate::game::Game;
use crate::rules::monster_stats::MonsterStats;
use crate::state::{GameState, Place};

/// What a responder may look at while it answers: the state, the data,
/// and the values the rules work out (dice, distances, monster values).
pub struct View<'g, 'a> {
    game: &'g Game<'a>,
}

impl<'g, 'a> View<'g, 'a> {
    /// Wraps a game.
    pub(crate) fn new(game: &'g Game<'a>) -> Self {
        Self { game }
    }

    /// The state.
    pub fn state(&self) -> &GameState {
        &self.game.state
    }

    /// The game data.
    pub fn data(&self) -> &'a GameData {
        self.game.data
    }

    /// Skill value with lasting bonuses (0 if the sheet is missing).
    pub fn skill(&self, seat: usize, kind: CheckKind) -> i32 {
        self.game.skill(seat, kind).unwrap_or(0)
    }

    /// Dice a check would roll before clue tokens, never below 0.
    pub fn check_dice(&self, seat: usize, kind: CheckKind, modifier: i32) -> u32 {
        let dice = self.game.check_dice(seat, kind, modifier).unwrap_or(0);
        u32::try_from(dice.max(0)).unwrap_or(0)
    }

    /// Dice gained per clue token spent on a check.
    pub fn dice_per_clue(&self, seat: usize, kind: CheckKind) -> u32 {
        self.game.dice_per_clue(seat, kind)
    }

    /// Lowest die face that counts as a success (5, or 4 blessed, 6 cursed).
    pub fn success_face(&self, seat: usize) -> u8 {
        self.game.success_face(seat)
    }

    /// Current maximum of Sanity or Stamina.
    pub fn maximum(&self, seat: usize, resource: Resource) -> u8 {
        self.game.maximum(seat, resource).unwrap_or(0)
    }

    /// Movement points for the Movement phase.
    pub fn movement_points(&self, seat: usize) -> u32 {
        self.game.movement_points(seat).unwrap_or(0)
    }

    /// Current values of a monster, once lasting effects apply.
    pub fn monster_stats(&self, id: &MonsterId) -> Option<MonsterStats> {
        self.game.monster_stats(id).ok()
    }

    /// Whether a card or Mythos effect lets the investigator ignore a
    /// monster ability.
    pub fn ignores(&self, seat: usize, ability: MonsterTrait) -> bool {
        self.game.ignores(seat, ability)
    }

    /// Places one step away in Arkham, closed locations left out.
    pub fn neighbours(&self, place: &Place) -> Vec<Place> {
        self.game.neighbours(place)
    }

    /// Number of steps between two places of Arkham, if connected.
    pub fn distance(&self, from: &Place, to: &Place) -> Option<usize> {
        self.game.distance(from, to)
    }

    /// Whether a location is closed, for good or by a Mythos card.
    pub fn is_closed(&self, location: &LocationId) -> bool {
        self.game.is_closed(location)
    }

    /// Whether an Elder Sign could be put on this location.
    pub fn can_be_sealed(&self, location: &LocationId) -> bool {
        self.game.can_be_sealed(location).unwrap_or(false)
    }

    /// Whether spells may be cast (no Mythos card forbids it).
    pub fn spells_allowed(&self) -> bool {
        !self.game.no_spells()
    }

    /// French name of a place.
    pub fn place_name(&self, place: &Place) -> String {
        self.game.place_name(place)
    }

    /// French name of the investigator at a seat.
    pub fn name(&self, seat: usize) -> String {
        self.game.name(seat)
    }
}

/// Looks at a state through a [`View`] outside of a game being played: to
/// draw a page, for example.
///
/// # Arguments
///
/// * `look` - reads what it needs from the view; its result is returned.
pub fn inspect<R>(data: &GameData, state: &GameState, look: impl FnOnce(&View<'_, '_>) -> R) -> R {
    let mut waiting = Waiting;
    let game = Game::new(data, state.clone(), &mut waiting);
    look(&View::new(&game))
}

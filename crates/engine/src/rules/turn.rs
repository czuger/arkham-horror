//! The game loop: setup, turns, awakening.

use crate::game::Game;
use crate::halt::{Flow, Halt};
use crate::setup::Setup;

/// A game still running after this many turns is stopped as broken.
pub(crate) const MAX_TURNS: u32 = 500;

/// A turn (or final battle round) asking more decisions than this is stuck
/// in a loop and stopped as broken.
pub(crate) const MAX_ANSWERS_PER_TURN: usize = 1000;

impl Game<'_> {
    /// Plays the game from the setup to its end, or to the first decision
    /// without an answer.
    pub fn run(&mut self, setup: &Setup) -> Flow<()> {
        match self.prepare(setup) {
            Err(Halt::Awakened) => return self.final_battle(),
            other => other?,
        }
        loop {
            if self.state.turn >= MAX_TURNS {
                return Err(Halt::Broken(format!("la partie dépasse {MAX_TURNS} tours")));
            }
            match self.play_turn() {
                Err(Halt::Awakened) => return self.final_battle(),
                other => other?,
            }
        }
    }

    /// One turn: the five phases, then the first player marker moves left.
    fn play_turn(&mut self) -> Flow<()> {
        self.state.turn += 1;
        self.turn_answers = 0;
        self.log(format!("— Tour {} —", self.state.turn));
        self.upkeep_phase()?;
        self.movement_phase()?;
        self.arkham_encounter_phase()?;
        self.other_world_encounter_phase()?;
        self.mythos_phase()?;
        self.state.first_player = (self.state.first_player + 1) % self.state.players();
        Ok(())
    }
}

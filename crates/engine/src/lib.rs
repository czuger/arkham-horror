//! Rules engine: game state, decisions, answers and events.
//!
//! Deterministic and free of I/O: no web, no database. A game is fully
//! defined by its setup (seed included), the game data and the ordered list
//! of answers; [`play`] replays it from the start each time.
#![deny(missing_docs)]

mod coverage;
mod decision;
mod engine_error;
mod game;
mod halt;
mod play;
mod rng;
mod rules;
mod setup;
mod state;
#[cfg(test)]
mod tests;

pub use coverage::coverage_keys;
pub use decision::{
    Choice, Decision, DecisionKind, DecisionOption, FirstOption, Replay, Responder, Waiting,
};
pub use engine_error::EngineError;
pub use play::{Outcome, Status, play};
pub use rng::Rng;
pub use setup::Setup;
pub use state::{
    ActiveRumor, Area, BattleState, BoardMonster, Decks, Ending, GameState, HeldCard,
    InvestigatorState, MonsterPlace, OpenGate, Phase, Place,
};

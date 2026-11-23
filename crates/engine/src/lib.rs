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
mod shown;
mod state;
mod tally;
#[cfg(test)]
mod tests;
mod view;

pub use coverage::coverage_keys;
pub use decision::{
    CardAim, Choice, Context, Decision, DecisionKind, DecisionOption, FirstOption, Replay,
    Responder, TradeOffer, Waiting,
};
pub use engine_error::EngineError;
pub use play::{Outcome, Status, play};
pub use rng::Rng;
pub use rules::combat_gear::HANDS;
pub use rules::monster_stats::MonsterStats;
pub use rules::stats::of_check;
pub use setup::Setup;
pub use shown::{CombatEnd, Deck, DiceCount, Exhibit, Shown};
pub use state::{
    ActiveRumor, Area, BattleState, BoardMonster, Decks, Ending, GameState, HeldCard,
    InvestigatorState, MonsterPlace, OpenGate, Phase, Place,
};
pub use tally::Tally;
pub use view::{View, inspect};

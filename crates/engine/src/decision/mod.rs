//! Decisions, answers and responders.

mod choice;
#[expect(
    clippy::module_inception,
    reason = "the module is named after its main type"
)]
mod decision;
mod responder;

pub use choice::Choice;
pub use decision::{Decision, DecisionKind, DecisionOption};
pub use responder::{FirstOption, Replay, Responder, Waiting};

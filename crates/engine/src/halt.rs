//! Why the engine stops going forward in a straight line.

use crate::decision::Decision;

/// Unwinds the rule code through `?`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Halt {
    /// A decision has no answer yet: the game waits.
    Waiting(Decision),
    /// The answer given is not one of the options.
    Illegal {
        /// Number of answers accepted before this one.
        position: usize,
        /// The refused answer.
        answer: usize,
        /// The decision it answered.
        decision: Box<Decision>,
    },
    /// The investigator at this seat is knocked out (insane, unconscious,
    /// lost or devoured): what they were doing stops.
    KnockedOut(usize),
    /// The Ancient One awakes: the turn stops, the final battle starts.
    Awakened,
    /// The game is over; the ending is in the state.
    Over,
    /// The data or the state contradicts the rules code: a bug.
    Broken(String),
}

/// Result of rule code.
pub(crate) type Flow<T> = Result<T, Halt>;

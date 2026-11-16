//! Who answers the decisions.

use crate::decision::decision::Decision;
use crate::state::GameState;

/// Answers decisions: a human through the server, an AI policy, or a
/// recorded list of answers.
pub trait Responder {
    /// Index of the chosen option, or `None` when no answer is available
    /// yet: the game then stops and waits for it.
    fn answer(&mut self, state: &GameState, decision: &Decision) -> Option<usize>;
}

/// Never answers: the game stops at the first decision.
#[derive(Debug, Clone, Copy, Default)]
pub struct Waiting;

impl Responder for Waiting {
    fn answer(&mut self, _state: &GameState, _decision: &Decision) -> Option<usize> {
        None
    }
}

/// Plays back recorded answers, then hands over to another responder.
pub struct Replay<'a> {
    answers: &'a [usize],
    next: usize,
    then: &'a mut dyn Responder,
}

impl<'a> Replay<'a> {
    /// Plays `answers` in order, then asks `then`.
    pub fn new(answers: &'a [usize], then: &'a mut dyn Responder) -> Self {
        Self {
            answers,
            next: 0,
            then,
        }
    }
}

impl Responder for Replay<'_> {
    fn answer(&mut self, state: &GameState, decision: &Decision) -> Option<usize> {
        if let Some(&answer) = self.answers.get(self.next) {
            self.next += 1;
            return Some(answer);
        }
        self.then.answer(state, decision)
    }
}

/// Picks the first option every time. Useful in tests.
#[derive(Debug, Clone, Copy, Default)]
pub struct FirstOption;

impl Responder for FirstOption {
    fn answer(&mut self, _state: &GameState, _decision: &Decision) -> Option<usize> {
        Some(0)
    }
}

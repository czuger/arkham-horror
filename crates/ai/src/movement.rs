//! Moving through Arkham towards the turn's goal.

use engine::{Choice, Decision, View};

use crate::goal::goal;

/// One movement step: towards the goal, or stop there.
pub fn step(view: &View<'_, '_>, seat: usize, decision: &Decision) -> usize {
    let done = decision.find(&Choice::Done).unwrap_or(0);
    let Some(goal) = goal(view, seat) else {
        return done;
    };
    goal.route
        .steps
        .first()
        .and_then(|next| decision.find(&Choice::Place(next.clone())))
        .unwrap_or(done)
}

/// A place anywhere in Arkham: the turn's goal when offered.
pub fn destination(view: &View<'_, '_>, seat: usize, decision: &Decision) -> usize {
    goal(view, seat)
        .and_then(|goal| decision.find(&Choice::Place(goal.target)))
        .unwrap_or(0)
}

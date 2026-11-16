//! Lasting effects that fire at a given moment.

use data::{Moment, Ongoing};

use crate::game::Game;
use crate::halt::{Flow, Halt};

impl Game<'_> {
    /// Fires the lasting effects of an investigator for a moment.
    pub fn fire_triggers(&mut self, seat: usize, moment: Moment) -> Flow<()> {
        for (card, effect) in self.ongoing_for(seat) {
            if let Ongoing::Trigger {
                when,
                only,
                effects,
            } = effect
            {
                let applies = only
                    .as_ref()
                    .is_none_or(|condition| self.holds(seat, condition));
                if *when == moment && applies {
                    self.apply_effects(seat, effects, card.as_ref())?;
                }
            }
        }
        Ok(())
    }

    /// End-of-Mythos triggers, for every investigator.
    pub fn end_of_mythos_triggers(&mut self) -> Flow<()> {
        for seat in self.state.seat_order() {
            self.catch_knock_out(seat, |game| game.fire_triggers(seat, Moment::EndOfMythos))?;
        }
        Ok(())
    }

    /// Runs rule code for one investigator; being knocked out only stops
    /// that code, not the caller.
    pub fn catch_knock_out(
        &mut self,
        seat: usize,
        step: impl FnOnce(&mut Self) -> Flow<()>,
    ) -> Flow<()> {
        match step(self) {
            Err(Halt::KnockedOut(knocked)) if knocked == seat => Ok(()),
            other => other,
        }
    }
}

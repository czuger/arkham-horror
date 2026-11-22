//! The Upkeep skill adjustment set in one form: the engine asks it one stop
//! at a time, so the stops picked are turned into those answers here.

use data::GameData;
use engine::Choice;

use crate::app_error::AppError;
use crate::session::Session;
use crate::views::adjusts_skills;

impl Session {
    /// Moves the sliders of the investigator adjusting their skills to the
    /// stops picked, one stop at a time, then ends the adjustment. Undoing
    /// takes the whole adjustment back.
    ///
    /// # Arguments
    ///
    /// * `targets` - the stop picked on each of the three tracks.
    /// * `move_number` - [`Session::move_number`] when the form was shown.
    ///
    /// # Returns
    ///
    /// `false` when the form was out of date and nothing was done.
    ///
    /// # Errors
    ///
    /// [`AppError::BadRequest`] if no adjustment is under way, a stop does
    /// not exist, or the moves need more Concentration than is left; another
    /// [`AppError`] if the replay fails.
    pub fn adjust_skills(
        &mut self,
        data: &GameData,
        targets: [u8; 3],
        move_number: usize,
    ) -> Result<bool, AppError> {
        if move_number != self.move_number() {
            return Ok(false);
        }
        let seat = self
            .decision()
            .filter(|decision| adjusts_skills(decision))
            .map(|decision| decision.who)
            .ok_or_else(|| bad("Ce n'est pas le moment d'ajuster les compétences."))?;
        self.progress
            .human_moves
            .push((self.progress.answers.len(), self.progress.seen_events));
        self.progress.seen_events = self.outcome.events.len();
        loop {
            let wanted = next_move(self.sliders(seat)?, targets);
            let Some(decision) = self
                .decision()
                .filter(|decision| adjusts_skills(decision) && decision.who == seat)
            else {
                // The Concentration ran out: fine only if every slider is set.
                return if wanted == Choice::Done {
                    Ok(true)
                } else {
                    Err(bad("Pas assez de Concentration pour ces déplacements."))
                };
            };
            let index = decision
                .options
                .iter()
                .position(|option| option.choice == wanted)
                .ok_or_else(|| bad("Cran impossible."))?;
            self.progress.answers.push(index);
            self.replay(data)?;
            if wanted == Choice::Done {
                return Ok(true);
            }
        }
    }

    /// Where the sliders of the investigator at `seat` stand.
    fn sliders(&self, seat: usize) -> Result<[u8; 3], AppError> {
        self.outcome
            .state
            .investigators
            .get(seat)
            .map(|inv| inv.sliders)
            .ok_or(AppError::NotFound)
    }
}

/// The next one-stop move towards `targets`, or [`Choice::Done`] once there.
fn next_move(sliders: [u8; 3], targets: [u8; 3]) -> Choice {
    sliders
        .iter()
        .zip(targets)
        .enumerate()
        .find(|(_, (now, target))| **now != *target)
        .map_or(Choice::Done, |(track, (now, target))| Choice::Slider {
            track,
            stop: if target > *now { now + 1 } else { now - 1 },
        })
}

fn bad(message: &str) -> AppError {
    AppError::BadRequest(message.to_owned())
}

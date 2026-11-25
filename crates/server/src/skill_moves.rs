//! The Upkeep skill adjustment, set on one screen for every investigator:
//! the stops picked for each are kept aside, then played all at once, one
//! stop at a time as the engine asks them, when the human moves on.

use std::collections::BTreeMap;

use data::{GameData, SkillTracks};
use engine::{Choice, inspect};

use crate::app_error::AppError;
use crate::progress::PlannedSliders;
use crate::session::Session;
use crate::views::{Adjuster, adjusters, adjusts_skills};

impl Session {
    /// The investigators played by the human who adjust their skills now
    /// and next; empty outside the Upkeep adjustment.
    pub fn human_adjusters(&self, data: &GameData) -> Vec<Adjuster> {
        let Some(decision) = self.decision() else {
            return Vec::new();
        };
        let mode = self.mode();
        inspect(data, &self.outcome.state, |view| adjusters(view, decision))
            .into_iter()
            .filter(|adjuster| mode.human(adjuster.seat))
            .collect()
    }

    /// The stops picked for each seat since the last move.
    pub fn planned_sliders(&self) -> BTreeMap<usize, [u8; 3]> {
        let plan = &self.progress.planned_sliders;
        if plan.move_number == self.move_number() {
            plan.targets.clone()
        } else {
            BTreeMap::new()
        }
    }

    /// Keeps the stops picked for the investigator at `seat`, played when
    /// the human moves on.
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
    /// [`AppError::BadRequest`] if this investigator does not adjust now, a
    /// stop does not exist, or the moves need more Concentration than they
    /// have; [`AppError::NotFound`] for an unknown seat.
    pub fn plan_skills(
        &mut self,
        data: &GameData,
        seat: usize,
        targets: [u8; 3],
        move_number: usize,
    ) -> Result<bool, AppError> {
        if move_number != self.move_number() {
            return Ok(false);
        }
        let adjuster = self
            .human_adjusters(data)
            .into_iter()
            .find(|adjuster| adjuster.seat == seat)
            .ok_or_else(|| bad("Cet investigateur n'ajuste pas ses compétences maintenant."))?;
        if targets
            .iter()
            .any(|stop| usize::from(*stop) >= SkillTracks::STOPS)
        {
            return Err(bad("Cran impossible."));
        }
        let moved: u8 = self
            .sliders(seat)?
            .iter()
            .zip(targets)
            .map(|(now, target)| now.abs_diff(target))
            .sum();
        if moved > adjuster.focus {
            return Err(bad("Pas assez de Concentration pour ces déplacements."));
        }
        let mut targets_by_seat = self.planned_sliders();
        targets_by_seat.insert(seat, targets);
        self.progress.planned_sliders = PlannedSliders {
            move_number,
            targets: targets_by_seat,
        };
        Ok(true)
    }

    /// Moves on: plays the stops picked for every investigator played by
    /// the human, the others keeping their sliders. Undoing takes the whole
    /// adjustment back.
    ///
    /// # Arguments
    ///
    /// * `move_number` - [`Session::move_number`] when the form was shown.
    ///
    /// # Returns
    ///
    /// `false` when the form was out of date and nothing was done.
    ///
    /// # Errors
    ///
    /// [`AppError::BadRequest`] if no adjustment is under way or a pick no
    /// longer fits; another [`AppError`] if the replay fails.
    pub fn adjust_skills(&mut self, data: &GameData, move_number: usize) -> Result<bool, AppError> {
        if move_number != self.move_number() {
            return Ok(false);
        }
        if !self.decision().is_some_and(adjusts_skills) {
            return Err(bad("Ce n'est pas le moment d'ajuster les compétences."));
        }
        let planned = self.planned_sliders();
        self.progress.planned_sliders = PlannedSliders::default();
        self.progress
            .human_moves
            .push((self.progress.answers.len(), self.progress.seen_events));
        self.progress.seen_events = self.outcome.events.len();
        while let Some(seat) = self.human_adjusting() {
            let current = self.sliders(seat)?;
            let target = planned.get(&seat).copied().unwrap_or(current);
            self.play_slider_move(data, next_move(current, target))?;
        }
        Ok(true)
    }

    /// The seat adjusting now, when the human plays it.
    fn human_adjusting(&self) -> Option<usize> {
        self.decision()
            .filter(|decision| adjusts_skills(decision) && self.mode().human(decision.who))
            .map(|decision| decision.who)
    }

    /// Answers the adjustment under way with `wanted`.
    fn play_slider_move(&mut self, data: &GameData, wanted: Choice) -> Result<(), AppError> {
        let index = self
            .decision()
            .and_then(|decision| {
                decision
                    .options
                    .iter()
                    .position(|option| option.choice == wanted)
            })
            .ok_or_else(|| bad("Cran impossible."))?;
        self.progress.answers.push(index);
        self.replay(data)
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

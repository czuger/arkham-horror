//! Effect primitives that roll dice or make a check.

use data::{CardId, CheckKind, Effect, Resource, TableRow};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::shown::{DiceCount, Exhibit};

impl<'a> Game<'a> {
    /// Checks, rolls and picked numbers.
    pub fn apply_dice_effect(
        &mut self,
        seat: usize,
        effect: &'a Effect,
        this: Option<&CardId>,
    ) -> Flow<()> {
        match effect {
            Effect::Check {
                skill,
                modifier,
                successes,
                or_discard,
                pass,
                fail,
            } => {
                if self.discard_to_pass(seat, or_discard.as_ref())? {
                    return self.apply_effects(seat, pass, this);
                }
                let result = self.check(seat, *skill, i32::from(*modifier), *successes)?;
                let next = if result.passed { pass } else { fail };
                self.apply_effects(seat, next, this)
            }
            Effect::CheckTable {
                skill,
                modifier,
                rows,
            } => self.check_table(seat, *skill, *modifier, rows, this),
            Effect::RollDie { at_most, effects } => {
                let roll = self.die_shown(Some(seat));
                self.scratch.x = u32::from(roll);
                self.log(format!("{} lance un dé : {roll}.", self.name(seat)));
                if roll <= *at_most {
                    return self.apply_effects(seat, effects, this);
                }
                Ok(())
            }
            Effect::RollDice(count) => {
                self.roll_total(seat, *count);
                Ok(())
            }
            Effect::RollEach {
                dice,
                success,
                failure,
            } => self.roll_each(seat, *dice, success, failure, this),
            Effect::RollPerPoint {
                resource,
                devoured_at_zero,
            } => self.roll_per_point(seat, *resource, *devoured_at_zero),
            Effect::ChooseUpTo(most) => self.choose_up_to(seat, *most),
            _ => Ok(()),
        }
    }

    /// The player picks a number from 0 to `most`, read as X.
    fn choose_up_to(&mut self, seat: usize, most: u8) -> Flow<()> {
        let options = (0..=u32::from(most))
            .map(|value| option(Choice::Number(value), value.to_string()))
            .collect();
        let prompt = "Choisissez un nombre :";
        if let Choice::Number(value) = self.ask(seat, DecisionKind::CardChoice, prompt, options)? {
            self.scratch.x = value;
        }
        Ok(())
    }

    /// Offers to discard a held card instead of rolling (Whisky, Nourriture…).
    fn discard_to_pass(&mut self, seat: usize, card: Option<&CardId>) -> Flow<bool> {
        let Some(card) = card else {
            return Ok(false);
        };
        if !self.inv(seat).cards.iter().any(|held| &held.card == card) {
            return Ok(false);
        }
        let prompt = format!("Défausser {} pour réussir le test ?", self.card_name(card));
        if !self.ask_yes_no(seat, DecisionKind::Optional, prompt)? {
            return Ok(false);
        }
        self.log(format!(
            "{} défausse {} et réussit le test.",
            self.name(seat),
            self.card_name(card)
        ));
        self.discard_card(seat, card)?;
        Ok(true)
    }

    /// A check read on a table: the row with the most successes reached.
    fn check_table(
        &mut self,
        seat: usize,
        skill: CheckKind,
        modifier: i8,
        rows: &'a [TableRow],
        this: Option<&CardId>,
    ) -> Flow<()> {
        // Aim at the best row, so clue tokens can be offered to reach it.
        let target = rows.iter().map(|row| row.from).max().unwrap_or(1);
        let result = self.check(seat, skill, i32::from(modifier), target)?;
        let row = rows
            .iter()
            .filter(|row| u32::from(row.from) <= result.successes)
            .max_by_key(|row| row.from);
        match row {
            Some(row) => self.apply_effects(seat, &row.effects, this),
            None => Ok(()),
        }
    }

    /// Rolls dice and keeps their total as X.
    fn roll_total(&mut self, seat: usize, count: u8) {
        let total: u32 = self
            .dice_shown(Some(seat), count)
            .into_iter()
            .map(u32::from)
            .sum();
        self.scratch.x = total;
        self.log(format!("{} lance {count} dés : {total}.", self.name(seat)));
    }

    /// Rolls dice one at a time: effects for each success and each failure.
    fn roll_each(
        &mut self,
        seat: usize,
        dice: u8,
        success: &'a [Effect],
        failure: &'a [Effect],
        this: Option<&CardId>,
    ) -> Flow<()> {
        let face = self.success_face(seat);
        let rolls: Vec<u8> = (0..dice).map(|_| self.die()).collect();
        self.show(Exhibit::Dice {
            seat: Some(seat),
            faces: rolls.clone(),
            count: DiceCount::Successes {
                success_from: face,
                sixes_double: false,
            },
        });
        let successes = rolls.iter().filter(|&&value| value >= face).count();
        self.scratch.successes = u32::try_from(successes).unwrap_or(0);
        self.log(format!(
            "{} lance {dice} dé(s) : {successes} succès.",
            self.name(seat)
        ));
        for value in rolls {
            let effects = if value >= face { success } else { failure };
            self.apply_effects(seat, effects, this)?;
        }
        Ok(())
    }

    /// One die per point of Sanity or Stamina; one point lost per failure.
    fn roll_per_point(&mut self, seat: usize, resource: Resource, devoured: bool) -> Flow<()> {
        let dice = self.amount(seat, resource);
        let successes = self.roll_dice(seat, dice);
        let failures = dice - successes;
        self.scratch.successes = successes;
        self.log(format!(
            "{} lance {dice} dé(s) : {successes} succès, {failures} échec(s).",
            self.name(seat)
        ));
        if devoured && dice > 0 && failures >= dice {
            return self.devour(seat);
        }
        self.lose(seat, resource, failures)
    }
}

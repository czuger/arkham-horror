//! Skill checks: dice, successes, clue tokens, rerolls.

use data::{CheckKind, Ongoing};

use crate::decision::{Choice, Context, DecisionKind, DecisionOption};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::stats::of_check;

/// Result of a check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CheckResult {
    /// Whether the check passed.
    pub passed: bool,
    /// Successes rolled.
    pub successes: u32,
}

/// A check being rolled.
struct Roll {
    kind: CheckKind,
    difficulty: u32,
    dice: u32,
    successes: u32,
}

impl Game<'_> {
    /// Makes a check: rolls, offers clue tokens and rerolls while it fails.
    pub fn check(
        &mut self,
        seat: usize,
        kind: CheckKind,
        modifier: i32,
        difficulty: u8,
    ) -> Flow<CheckResult> {
        if self.offer_pass(seat, kind)? {
            self.log(format!(
                "{} réussit d'office le test {}.",
                self.name(seat),
                of_check(kind)
            ));
            let successes = u32::from(difficulty.max(1));
            self.scratch.successes = successes;
            return Ok(CheckResult {
                passed: true,
                successes,
            });
        }
        let dice = self.check_dice(seat, kind, modifier)?;
        let dice = u32::try_from(dice.max(0)).unwrap_or(0);
        let mut roll = Roll {
            kind,
            difficulty: u32::from(difficulty.max(1)),
            dice,
            successes: 0,
        };
        roll.successes = self.roll_dice(seat, dice);
        self.log(format!(
            "{} : test {} ({modifier:+}) [{}], {} dé(s), {} succès.",
            self.name(seat),
            of_check(kind),
            roll.difficulty,
            dice,
            roll.successes
        ));
        self.improve(seat, &mut roll)?;
        let passed = roll.successes >= roll.difficulty;
        self.scratch.successes = roll.successes;
        self.log(if passed { "Réussi." } else { "Raté." });
        Ok(CheckResult {
            passed,
            successes: roll.successes,
        })
    }

    /// While the check fails: clue tokens, then cards that reroll, then
    /// Mandy's Research.
    fn improve(&mut self, seat: usize, roll: &mut Roll) -> Flow<()> {
        loop {
            self.spend_clues(seat, roll)?;
            if roll.successes >= roll.difficulty {
                return Ok(());
            }
            self.scratch.reroll_one_die = false;
            if self.offer_reroll(seat, roll.kind)? {
                roll.successes = self.roll_dice(seat, roll.dice);
                self.log(format!("Relance : {} succès.", roll.successes));
            } else if std::mem::take(&mut self.scratch.reroll_one_die) && roll.dice > roll.successes
            {
                let gained = self.roll_dice(seat, 1);
                roll.successes += gained;
                self.log(format!("Un dé raté est relancé : {gained} succès de plus."));
            } else if roll.dice > roll.successes && self.mandy_research(seat)? {
                let gained = self.roll_dice(seat, roll.dice - roll.successes);
                roll.successes += gained;
                self.log(format!("Dés ratés relancés : {gained} succès de plus."));
            } else {
                return Ok(());
            }
        }
    }

    /// Rolls dice and counts successes.
    pub fn roll_dice(&mut self, seat: usize, dice: u32) -> u32 {
        let face = self.success_face(seat);
        let six = if self.scratch.sixes_double { 2 } else { 1 };
        (0..dice)
            .map(|_| self.die())
            .map(|value| match value {
                6 => six,
                value if value >= face => 1,
                _ => 0,
            })
            .sum()
    }

    /// Offers clue tokens one at a time while the check fails.
    fn spend_clues(&mut self, seat: usize, roll: &mut Roll) -> Flow<()> {
        while roll.successes < roll.difficulty {
            let options = self.clue_options(seat);
            if options.len() == 1 {
                return Ok(());
            }
            let prompt = format!(
                "Test {} : {} succès sur {}. Dépenser un Indice ?",
                of_check(roll.kind),
                roll.successes,
                roll.difficulty
            );
            let context = Context::Check {
                kind: roll.kind,
                successes: roll.successes,
                needed: roll.difficulty,
                closing_gate: self.scratch.closing_gate,
            };
            match self.ask_about(seat, DecisionKind::SpendClues, context, prompt, options)? {
                Choice::SpendClue => self.inv_mut(seat).clues -= 1,
                Choice::SubstituteClue(card) => {
                    self.log(format!(
                        "{} défausse {}.",
                        self.name(seat),
                        self.card_name(&card)
                    ));
                    self.discard_card(seat, &card)?;
                }
                _ => return Ok(()),
            }
            let extra = self.dice_per_clue(seat, roll.kind);
            roll.dice += extra;
            let gained = self.roll_dice(seat, extra);
            roll.successes += gained;
            self.log(format!("Indice dépensé : {gained} succès de plus."));
        }
        Ok(())
    }

    /// Ways to pay for one more die: clue tokens, or cards that stand in for them.
    fn clue_options(&self, seat: usize) -> Vec<DecisionOption> {
        let mut options = vec![option(Choice::Done, "Garder ce résultat")];
        if self.inv(seat).clues > 0 {
            options.push(option(Choice::SpendClue, "Dépenser 1 Indice"));
        }
        for (card, effect) in self.ongoing_for(seat) {
            if let (Some(card), Ongoing::ClueSubstitute) = (card, effect) {
                let label = format!("Défausser {} à la place d'un Indice", self.card_name(&card));
                options.push(option(Choice::SubstituteClue(card), label));
            }
        }
        options
    }
}

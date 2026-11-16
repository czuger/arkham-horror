//! The effect interpreter: applies the effect primitives of the data files.

use data::{CardId, ChoiceOption, Effect};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::stats::resource_name;

impl<'a> Game<'a> {
    /// Applies effects in order to the investigator at `seat`. `this` is the
    /// card the effects are printed on, when it matters.
    pub fn apply_effects(
        &mut self,
        seat: usize,
        effects: &'a [Effect],
        this: Option<&CardId>,
    ) -> Flow<()> {
        for effect in effects {
            self.apply_effect(seat, effect, this)?;
        }
        Ok(())
    }

    /// Applies one effect.
    fn apply_effect(&mut self, seat: usize, effect: &'a Effect, this: Option<&CardId>) -> Flow<()> {
        match effect {
            Effect::Gain(..)
            | Effect::GainRolled(_)
            | Effect::Lose(..)
            | Effect::LoseRolled(_)
            | Effect::LoseAll(_)
            | Effect::Pay(..)
            | Effect::RestoreToMax(_) => self.apply_resource_effect(seat, effect),
            Effect::Check { .. } | Effect::RollDie { .. } => {
                self.apply_dice_effect(seat, effect, this)
            }
            Effect::Choice(options) => self.choose_option(seat, options, this),
            Effect::Optional(choice) => {
                let prompt = format!("Voulez-vous : {} ?", choice.label);
                if self.ask_yes_no(seat, DecisionKind::Optional, prompt)? {
                    return self.apply_effects(seat, &choice.effects, this);
                }
                Ok(())
            }
            Effect::If {
                condition,
                then,
                otherwise,
            } => {
                let next = if self.holds(seat, condition) {
                    then
                } else {
                    otherwise
                };
                self.apply_effects(seat, next, this)
            }
            Effect::EachInvestigator { only, effects } => {
                for other in self.state.seat_order() {
                    if only
                        .as_ref()
                        .is_none_or(|condition| self.holds(other, condition))
                    {
                        self.catch_knock_out(other, |game| {
                            game.apply_effects(other, effects, this)
                        })?;
                    }
                }
                Ok(())
            }
            _ => self.apply_other_effect(seat, effect, this),
        }
    }

    /// A check or a die roll, then the effects it leads to.
    fn apply_dice_effect(
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
                pass,
                fail,
            } => {
                let result = self.check(seat, *skill, i32::from(*modifier), *successes)?;
                let next = if result.passed { pass } else { fail };
                self.apply_effects(seat, next, this)
            }
            Effect::RollDie { at_most, effects } => {
                let roll = self.die();
                self.log(format!("{} lance un dé : {roll}.", self.name(seat)));
                if roll <= *at_most {
                    return self.apply_effects(seat, effects, this);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Effects on Sanity, Stamina, money and clue tokens.
    fn apply_resource_effect(&mut self, seat: usize, effect: &Effect) -> Flow<()> {
        match effect {
            Effect::Gain(resource, amount) => self.gain(seat, *resource, u32::from(*amount)),
            Effect::GainRolled(resource) => {
                let roll = u32::from(self.die());
                self.gain(seat, *resource, roll)
            }
            Effect::Lose(resource, amount) => self.lose(seat, *resource, u32::from(*amount)),
            Effect::LoseRolled(resource) => {
                let roll = u32::from(self.die());
                self.lose(seat, *resource, roll)
            }
            Effect::LoseAll(resource) => {
                let all = self.amount(seat, *resource);
                self.lose(seat, *resource, all)
            }
            Effect::Pay(resource, amount) => {
                if !self.pay(seat, *resource, u32::from(*amount))? {
                    self.log(format!(
                        "{} ne peut pas payer {amount} {}.",
                        self.name(seat),
                        resource_name(*resource)
                    ));
                }
                Ok(())
            }
            Effect::RestoreToMax(resource) => {
                let max = u32::from(self.maximum(seat, *resource)?);
                let missing = max.saturating_sub(self.amount(seat, *resource));
                self.gain(seat, *resource, missing)
            }
            _ => Ok(()),
        }
    }

    /// « Choisissez » : one of the options that can be taken. An option that
    /// starts with a payment needs the money in hand.
    fn choose_option(
        &mut self,
        seat: usize,
        options: &'a [ChoiceOption],
        this: Option<&CardId>,
    ) -> Flow<()> {
        let mut offered = Vec::new();
        for (index, choice) in options.iter().enumerate() {
            let affordable = match choice.effects.first() {
                Some(Effect::Pay(resource, amount)) => {
                    self.amount(seat, *resource) >= u32::from(*amount)
                }
                _ => true,
            };
            if affordable {
                offered.push(option(Choice::Option(index), choice.label.clone()));
            }
        }
        if offered.is_empty() {
            return Ok(());
        }
        let Choice::Option(index) =
            self.ask(seat, DecisionKind::CardChoice, "Choisissez :", offered)?
        else {
            return Ok(());
        };
        match options.get(index) {
            Some(choice) => self.apply_effects(seat, &choice.effects, this),
            None => Ok(()),
        }
    }
}

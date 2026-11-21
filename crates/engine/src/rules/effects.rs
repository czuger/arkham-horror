//! The effect interpreter: applies the effect primitives of the data files.

use data::{CardId, ChoiceOption, Condition, Effect};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option, option_with};
use crate::halt::Flow;

impl<'a> Game<'a> {
    /// Applies effects in order to the investigator at `seat`. `this` is the
    /// card the effects are printed on, when it matters. Two `Lose` in a row
    /// are suffered at the same time.
    pub fn apply_effects(
        &mut self,
        seat: usize,
        effects: &'a [Effect],
        this: Option<&CardId>,
    ) -> Flow<()> {
        let mut index = 0;
        while let Some(effect) = effects.get(index) {
            if let (Effect::Lose(first, first_amount), Some(Effect::Lose(second, second_amount))) =
                (effect, effects.get(index + 1))
            {
                self.lose_both(
                    seat,
                    *first,
                    u32::from(*first_amount),
                    Some((*second, u32::from(*second_amount))),
                )?;
                index += 2;
                continue;
            }
            self.apply_effect(seat, effect, this)?;
            index += 1;
        }
        Ok(())
    }

    /// Applies one effect.
    fn apply_effect(&mut self, seat: usize, effect: &'a Effect, this: Option<&CardId>) -> Flow<()> {
        match effect {
            Effect::Gain(..)
            | Effect::GainRolled(_)
            | Effect::GainAmount(..)
            | Effect::GainSplit(_)
            | Effect::Lose(..)
            | Effect::LoseRolled(_)
            | Effect::LoseAmount(..)
            | Effect::LoseAll(_)
            | Effect::Pay(..)
            | Effect::RestoreToMax(_)
            | Effect::LowerMax(..) => self.apply_resource_effect(seat, effect),
            Effect::Check { .. }
            | Effect::CheckTable { .. }
            | Effect::RollDie { .. }
            | Effect::RollDice(_)
            | Effect::RollEach { .. }
            | Effect::RollPerPoint { .. }
            | Effect::ChooseUpTo(_) => self.apply_dice_effect(seat, effect, this),
            Effect::Choice(options) => self.choose_option(seat, options, this),
            Effect::Optional(choice) => self.optional(seat, choice, this),
            Effect::If {
                condition,
                then,
                otherwise,
            } => {
                let next = if self.holds_for(seat, condition, this) {
                    then
                } else {
                    otherwise
                };
                self.apply_effects(seat, next, this)
            }
            Effect::EachInvestigator { only, effects } => {
                self.each_investigator(only.as_ref(), effects, this)
            }
            Effect::OneInvestigatorHere(effects) => {
                self.one_investigator(seat, effects, this, false)
            }
            Effect::AnyInvestigator(effects) => self.one_investigator(seat, effects, this, true),
            _ => self.apply_other_effect(seat, effect, this),
        }
    }

    /// « Chaque investigateur » (who meets the condition, if any).
    fn each_investigator(
        &mut self,
        only: Option<&Condition>,
        effects: &'a [Effect],
        this: Option<&CardId>,
    ) -> Flow<()> {
        for other in self.state.seat_order() {
            if only.is_none_or(|condition| self.holds(other, condition)) {
                self.catch_knock_out(other, |game| game.apply_effects(other, effects, this))?;
            }
        }
        Ok(())
    }

    /// « Vous pouvez » : offered only when its cost can be paid.
    fn optional(
        &mut self,
        seat: usize,
        choice: &'a ChoiceOption,
        this: Option<&CardId>,
    ) -> Flow<()> {
        if !self.can_start(seat, &choice.effects)? {
            return Ok(());
        }
        let prompt = format!("Voulez-vous : {} ?", choice.label);
        if self.ask_yes_with(seat, prompt, &choice.effects)? {
            return self.apply_effects(seat, &choice.effects, this);
        }
        Ok(())
    }

    /// Whether the costs at the start of a list of effects can be paid, and
    /// the named card it then takes (with no fallback) can be had.
    pub fn can_start(&self, seat: usize, effects: &[Effect]) -> Flow<bool> {
        for effect in effects {
            let ok = match effect {
                Effect::Pay(resource, amount) => self.amount(seat, *resource) >= u32::from(*amount),
                Effect::SpendMonsterTrophies(toughness) => {
                    self.trophy_toughness(seat)? >= u32::from(*toughness)
                }
                Effect::SpendGateTrophies(count) => {
                    self.inv(seat).gate_trophies.len() >= usize::from(*count)
                }
                Effect::TakeCard { card, otherwise } if otherwise.is_empty() => {
                    return Ok(self.may_receive(seat, card) && self.in_deck(card)?);
                }
                Effect::Choice(options) => {
                    for choice in options {
                        if self.can_start(seat, &choice.effects)? {
                            return Ok(true);
                        }
                    }
                    return Ok(false);
                }
                _ => return Ok(true),
            };
            if !ok {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// « Choisissez » : one of the options that can be taken.
    fn choose_option(
        &mut self,
        seat: usize,
        options: &'a [ChoiceOption],
        this: Option<&CardId>,
    ) -> Flow<()> {
        let mut offered = Vec::new();
        for (index, choice) in options.iter().enumerate() {
            if self.can_start(seat, &choice.effects)? {
                offered.push(option_with(
                    Choice::Option(index),
                    choice.label.clone(),
                    &choice.effects,
                ));
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

    /// Effects for one investigator in the same area, chosen by the one who
    /// resolves the card (themselves included).
    fn one_investigator(
        &mut self,
        seat: usize,
        effects: &'a [Effect],
        this: Option<&CardId>,
        anywhere: bool,
    ) -> Flow<()> {
        let others = if anywhere {
            self.state.seat_order()
        } else {
            self.investigators_at(&self.inv(seat).place.clone())
        };
        let mut options = vec![option(Choice::Investigator(seat), self.name(seat))];
        for other in others.into_iter().filter(|&other| other != seat) {
            options.push(option(Choice::Investigator(other), self.name(other)));
        }
        let Choice::Investigator(target) = self.ask(
            seat,
            DecisionKind::CardChoice,
            "Pour quel investigateur ?",
            options,
        )?
        else {
            return Ok(());
        };
        self.catch_knock_out(target, |game| game.apply_effects(target, effects, this))
    }
}

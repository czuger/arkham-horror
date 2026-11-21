//! Effect primitives on Sanity, Stamina, money and clue tokens.

use data::{Amount, Effect, Resource};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::stats::resource_name;

impl Game<'_> {
    /// Gains, losses, payments and maximums.
    pub fn apply_resource_effect(&mut self, seat: usize, effect: &Effect) -> Flow<()> {
        match effect {
            Effect::Gain(resource, amount) => self.gain(seat, *resource, u32::from(*amount)),
            Effect::GainRolled(resource) => {
                let roll = self.rolled(seat);
                self.gain(seat, *resource, roll)
            }
            Effect::GainAmount(resource, amount) => {
                let value = self.amount_value(seat, *amount);
                self.gain(seat, *resource, value)
            }
            Effect::GainSplit(amount) => {
                let value = self.amount_value(seat, *amount);
                self.gain_split(seat, value)
            }
            Effect::Lose(resource, amount) => self.lose(seat, *resource, u32::from(*amount)),
            Effect::LoseRolled(resource) => {
                let roll = self.rolled(seat);
                self.lose(seat, *resource, roll)
            }
            Effect::LoseAmount(resources, amount) => {
                let value = self.amount_value(seat, *amount);
                match resources.as_slice() {
                    [first, second, ..] => {
                        self.lose_both(seat, *first, value, Some((*second, value)))
                    }
                    [only] => self.lose(seat, *only, value),
                    [] => Ok(()),
                }
            }
            Effect::LoseAll(resource) => {
                let all = self.amount(seat, *resource);
                self.lose(seat, *resource, all)
            }
            Effect::Pay(resource, amount) => self.pay_or_log(seat, *resource, *amount),
            Effect::RestoreToMax(resource) => {
                let max = u32::from(self.maximum(seat, *resource)?);
                let missing = max.saturating_sub(self.amount(seat, *resource));
                self.gain(seat, *resource, missing)
            }
            Effect::LowerMax(resource, amount) => self.lower_max(seat, *resource, *amount),
            _ => Ok(()),
        }
    }

    /// Works out an [`Amount`].
    pub fn amount_value(&self, seat: usize, amount: Amount) -> u32 {
        match amount {
            Amount::Fixed(value) => u32::from(value),
            Amount::X => self.scratch.x,
            Amount::XMinus(less) => self.scratch.x.saturating_sub(u32::from(less)),
            Amount::Successes(each) => self.scratch.successes * u32::from(each),
            Amount::HalfUp(resource) => self.amount(seat, resource).div_ceil(2),
        }
    }

    /// Rolls one die, logs it and keeps it as X.
    fn rolled(&mut self, seat: usize) -> u32 {
        let roll = u32::from(self.die_shown(Some(seat)));
        self.scratch.x = roll;
        self.log(format!("{} lance un dé : {roll}.", self.name(seat)));
        roll
    }

    /// Pays the full amount, or logs that it cannot.
    fn pay_or_log(&mut self, seat: usize, resource: Resource, amount: u8) -> Flow<()> {
        if !self.pay(seat, resource, u32::from(amount))? {
            self.log(format!(
                "{} ne peut pas payer {amount} {}.",
                self.name(seat),
                resource_name(resource)
            ));
        }
        Ok(())
    }

    /// Points split between Stamina and Sanity, as the investigator likes.
    fn gain_split(&mut self, seat: usize, points: u32) -> Flow<()> {
        if points == 0 {
            return Ok(());
        }
        let options = (0..=points)
            .map(|stamina| {
                let label = format!("{stamina} Résistance, {} Santé Mentale", points - stamina);
                option(Choice::Number(stamina), label)
            })
            .collect();
        let prompt = format!("Répartissez {points} point(s).");
        let Choice::Number(stamina) = self.ask(seat, DecisionKind::CardChoice, prompt, options)?
        else {
            return Ok(());
        };
        self.gain(seat, Resource::Stamina, stamina)?;
        self.gain(seat, Resource::Sanity, points - stamina)
    }

    /// Lowers a maximum for the rest of the game; the current value follows.
    fn lower_max(&mut self, seat: usize, resource: Resource, amount: u8) -> Flow<()> {
        let change = i8::try_from(amount).unwrap_or(i8::MAX);
        let inv = self.inv_mut(seat);
        match resource {
            Resource::Sanity => {
                inv.max_sanity_change = inv.max_sanity_change.saturating_sub(change)
            }
            _ => inv.max_stamina_change = inv.max_stamina_change.saturating_sub(change),
        }
        let max = u32::from(self.maximum(seat, resource)?);
        if self.amount(seat, resource) > max {
            let inv = self.inv_mut(seat);
            let capped = u8::try_from(max).unwrap_or(u8::MAX);
            match resource {
                Resource::Sanity => inv.sanity = capped,
                _ => inv.stamina = capped,
            }
        }
        self.log(format!(
            "{} perd définitivement {amount} point(s) de {} maximum.",
            self.name(seat),
            resource_name(resource)
        ));
        self.after_loss(seat)
    }
}

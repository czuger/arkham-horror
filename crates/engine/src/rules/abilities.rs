//! Card abilities: when they can be used, their costs, and casting spells.

use data::{Ability, CardId, Condition, Cost, Ongoing, Timing};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::stats::of_check;
use crate::state::{HeldCard, Place};

/// An ability that can be used now.
pub(crate) struct Usable<'a> {
    /// The card that carries it.
    pub card: CardId,
    /// The ability.
    pub ability: &'a Ability,
}

impl<'a> Game<'a> {
    /// Abilities of held cards whose timing matches and whose costs can be paid.
    pub fn usable_abilities(
        &self,
        seat: usize,
        timing: &dyn Fn(&Timing) -> bool,
    ) -> Flow<Vec<Usable<'a>>> {
        let mut found = Vec::new();
        for held in &self.inv(seat).cards {
            let card = self.card(&held.card)?;
            for ability in card
                .abilities
                .iter()
                .filter(|ability| timing(&ability.timing))
            {
                let only_ok = ability
                    .only
                    .as_ref()
                    .is_none_or(|condition| self.holds(seat, condition));
                if only_ok && self.can_pay(seat, held, ability)? {
                    found.push(Usable {
                        card: held.card.clone(),
                        ability,
                    });
                }
            }
        }
        Ok(found)
    }

    /// Whether the costs of an ability can be paid now.
    fn can_pay(&self, seat: usize, held: &HeldCard, ability: &Ability) -> Flow<bool> {
        for cost in &ability.cost {
            let ok = match cost {
                Cost::Exhaust => !held.exhausted,
                Cost::DiscardThis => true,
                Cost::MovementPoints(points) => self.scratch.movement_points >= u32::from(*points),
                Cost::Pay(resource, amount) => self.amount(seat, *resource) >= u32::from(*amount),
            };
            if !ok {
                return Ok(false);
            }
        }
        if ability.cast {
            return Ok(u32::from(self.inv(seat).sanity) >= self.spell_cost(seat, &held.card)?);
        }
        Ok(true)
    }

    /// Offers the usable abilities matching a timing, one at a time, until
    /// the player declines. Returns whether at least one was used.
    pub fn offer_abilities(
        &mut self,
        seat: usize,
        timing: &dyn Fn(&Timing) -> bool,
        prompt: &str,
    ) -> Flow<bool> {
        let mut used = false;
        loop {
            let usable = self.usable_abilities(seat, timing)?;
            if usable.is_empty() {
                return Ok(used);
            }
            let mut options = vec![option(Choice::Done, "Ne rien utiliser")];
            for (index, entry) in usable.iter().enumerate() {
                options.push(option(
                    Choice::Option(index),
                    format!("Utiliser {}", self.card_name(&entry.card)),
                ));
            }
            let Choice::Option(index) =
                self.ask(seat, DecisionKind::UseAbility, prompt, options)?
            else {
                return Ok(used);
            };
            let Some(entry) = usable.into_iter().nth(index) else {
                return Ok(used);
            };
            self.use_ability(seat, &entry.card, entry.ability)?;
            used = true;
        }
    }

    /// After a failed check: offers the cards that reroll it. Returns whether
    /// the check is to be rerolled.
    pub fn offer_reroll(&mut self, seat: usize, kind: data::CheckKind) -> Flow<bool> {
        let usable = self.usable_abilities(seat, &|timing| *timing == Timing::AfterCheck(kind))?;
        if usable.is_empty() {
            return Ok(false);
        }
        let mut options = vec![option(Choice::Done, "Garder le résultat")];
        for (index, entry) in usable.iter().enumerate() {
            options.push(option(
                Choice::Option(index),
                format!("Utiliser {}", self.card_name(&entry.card)),
            ));
        }
        let prompt = format!("Test {} raté.", of_check(kind));
        let Choice::Option(index) = self.ask(seat, DecisionKind::UseAbility, prompt, options)?
        else {
            return Ok(false);
        };
        let Some(entry) = usable.into_iter().nth(index) else {
            return Ok(false);
        };
        self.scratch.reroll = false;
        self.use_ability(seat, &entry.card, entry.ability)?;
        Ok(self.scratch.reroll)
    }

    /// Pays the costs of an ability, casts the spell if needed, then applies
    /// its effects. Returns whether the effects were applied.
    pub fn use_ability(&mut self, seat: usize, card: &CardId, ability: &'a Ability) -> Flow<bool> {
        self.log(format!(
            "{} utilise {}.",
            self.name(seat),
            self.card_name(card)
        ));
        for cost in &ability.cost {
            match cost {
                Cost::Exhaust => self.exhaust(seat, card),
                Cost::DiscardThis => self.discard_card(seat, card)?,
                Cost::MovementPoints(points) => {
                    self.scratch.movement_points = self
                        .scratch
                        .movement_points
                        .saturating_sub(u32::from(*points));
                }
                Cost::Pay(resource, amount) => {
                    self.pay(seat, *resource, u32::from(*amount))?;
                }
            }
        }
        if ability.cast && !self.cast(seat, card)? {
            self.after_loss(seat)?;
            return Ok(false);
        }
        self.apply_effects(seat, &ability.effects, Some(card))?;
        if ability.cast {
            self.after_loss(seat)?;
        }
        Ok(true)
    }

    /// Turns a held card face down.
    pub fn exhaust(&mut self, seat: usize, card: &CardId) {
        if let Some(held) = self
            .inv_mut(seat)
            .cards
            .iter_mut()
            .find(|held| &held.card == card && !held.exhausted)
        {
            held.exhausted = true;
        }
    }

    /// Sanity cost to cast a spell now.
    pub fn spell_cost(&self, seat: usize, card: &CardId) -> Flow<u32> {
        let printed = self
            .card(card)?
            .casting
            .map_or(0, |casting| casting.sanity_cost);
        let forced = self
            .ongoing_for(seat)
            .into_iter()
            .find_map(|(_, effect)| match effect {
                Ongoing::SpellSanityCost(cost) => Some(*cost),
                _ => None,
            });
        Ok(u32::from(forced.unwrap_or(printed)))
    }

    /// Casts a spell: pays its Sanity, then makes the Spell check. Going to 0
    /// Sanity takes effect after the spell (FAQ).
    pub fn cast(&mut self, seat: usize, card: &CardId) -> Flow<bool> {
        let cost = self.spell_cost(seat, card)?;
        let modifier = self
            .card(card)?
            .casting
            .map_or(0, |casting| casting.modifier);
        let sanity = u32::from(self.inv(seat).sanity);
        if sanity < cost {
            return Ok(false);
        }
        self.inv_mut(seat).sanity = u8::try_from(sanity - cost).unwrap_or(0);
        self.log(format!(
            "{} lance {} ({cost} Santé Mentale, test {}).",
            self.name(seat),
            self.card_name(card),
            of_check(data::CheckKind::Spell)
        ));
        Ok(self
            .check(seat, data::CheckKind::Spell, i32::from(modifier), 1)?
            .passed)
    }

    /// Whether a condition holds for an investigator.
    pub fn holds(&self, seat: usize, condition: &Condition) -> bool {
        let inv = self.inv(seat);
        match condition {
            Condition::HasCard(card) => inv.holds(card),
            Condition::CardCountAtMost { deck, count } => {
                let held = inv
                    .cards
                    .iter()
                    .filter(|held| self.card(&held.card).is_ok_and(|card| card.deck == *deck))
                    .count();
                held <= usize::from(*count)
            }
            Condition::InStreets => matches!(inv.place, Place::Street(_)),
            Condition::InOtherWorld => matches!(inv.place, Place::OtherWorld { .. }),
            Condition::LostInTimeAndSpace => inv.place == Place::LostInTimeAndSpace,
        }
    }
}

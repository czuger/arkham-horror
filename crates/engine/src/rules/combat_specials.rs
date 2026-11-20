//! Cards that replace the Combat check: the Flute of the Outer Gods and
//! Bind Monster. Neither works against an Ancient One.

use data::{CardId, CheckKind, Ongoing, Resource, SpecialId};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::combat::Combat;

/// Hands an investigator has.
const HANDS: u8 = 2;
/// Special effect of the Flute of the Outer Gods.
const FLUTE: &str = "flute_of_the_outer_gods";
/// Special effect of Bind Monster.
const BIND_MONSTER: &str = "bind_monster";

impl Game<'_> {
    /// Before the Combat check: offers the cards that replace it. Returns
    /// `Some(passed)` when one was used instead of the check.
    pub fn special_attack(&mut self, seat: usize, combat: &Combat) -> Flow<Option<bool>> {
        let flute = self.held_special(seat, FLUTE)?;
        let bind = self.bind_monster_card(seat, combat)?;
        if flute.is_none() && bind.is_none() {
            return Ok(None);
        }
        let mut options = vec![option(Choice::Done, "Faire le test de Combat")];
        for card in flute.iter().chain(bind.iter()) {
            options.push(option(
                Choice::Card(card.clone()),
                format!("Utiliser {}", self.card_name(card)),
            ));
        }
        let prompt = "Avant le test de Combat.";
        let Choice::Card(card) = self.ask(seat, DecisionKind::UseAbility, prompt, options)? else {
            return Ok(None);
        };
        self.covered.insert(crate::coverage::used_key(&card));
        if flute.as_ref() == Some(&card) {
            self.play_flute(seat, &card)?;
            return Ok(Some(true));
        }
        self.bind_monster(seat, &card, combat).map(Some)
    }

    /// Lose 3 Sanity and 3 Stamina, discard the Flute: every monster in the
    /// area is defeated (the others by `face_monsters_here`).
    fn play_flute(&mut self, seat: usize, card: &CardId) -> Flow<()> {
        self.log(format!(
            "{} joue de {}.",
            self.name(seat),
            self.card_name(card)
        ));
        self.discard_card(seat, card)?;
        self.lose_both(seat, Resource::Sanity, 3, Some((Resource::Stamina, 3)))?;
        self.scratch.flute = true;
        Ok(())
    }

    /// Bind Monster, if held and castable: both hands free, spells allowed.
    fn bind_monster_card(&self, seat: usize, combat: &Combat) -> Flow<Option<CardId>> {
        let Some(card) = self.held_special(seat, BIND_MONSTER)? else {
            return Ok(None);
        };
        let hands_free = combat.gear.hands() + self.card(&card)?.hands <= HANDS;
        let sanity_ok = u32::from(self.inv(seat).sanity) >= self.spell_cost(seat, &card)?;
        Ok((hands_free && sanity_ok && !self.no_spells()).then_some(card))
    }

    /// Casts Bind Monster: a Spell check needing as many successes as the
    /// monster's toughness. The card is discarded either way; a failed
    /// casting counts as a failed Combat check (golden rule).
    fn bind_monster(&mut self, seat: usize, card: &CardId, combat: &Combat) -> Flow<bool> {
        let cost = self.spell_cost(seat, card)?;
        let modifier = self
            .card(card)?
            .casting
            .map_or(0, |casting| casting.modifier);
        let sanity = u32::from(self.inv(seat).sanity);
        self.inv_mut(seat).sanity = u8::try_from(sanity.saturating_sub(cost)).unwrap_or(0);
        self.log(format!(
            "{} lance {} ({cost} Santé Mentale).",
            self.name(seat),
            self.card_name(card)
        ));
        self.discard_card(seat, card)?;
        let toughness = u8::try_from(combat.stats.toughness).unwrap_or(u8::MAX);
        let passed = self
            .check(seat, CheckKind::Spell, i32::from(modifier), toughness)?
            .passed;
        self.after_loss(seat)?;
        Ok(passed)
    }

    /// A held card carrying a special effect.
    fn held_special(&self, seat: usize, special: &str) -> Flow<Option<CardId>> {
        for held in &self.inv(seat).cards {
            if self.card(&held.card)?.special == Some(SpecialId::new(special)) {
                return Ok(Some(held.card.clone()));
            }
        }
        Ok(None)
    }

    /// Mythos effects that apply when a monster is defeated.
    pub fn on_defeat_effects(&mut self, seat: usize, monster: &data::MonsterId) -> Flow<()> {
        for effect in self.mythos_ongoing() {
            if let Ongoing::OnDefeat {
                monster: which,
                effects,
            } = effect
                && which == monster
            {
                self.apply_effects(seat, effects, None)?;
            }
        }
        Ok(())
    }
}

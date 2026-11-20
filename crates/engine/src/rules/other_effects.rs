//! Effect primitives about cards, tokens and reactions; the rest is sent on
//! to the place and board effects.

use data::{CardId, Effect, LocationId};

use crate::game::Game;
use crate::halt::Flow;

impl<'a> Game<'a> {
    /// Effects that are neither resources, dice nor structure.
    pub fn apply_other_effect(
        &mut self,
        seat: usize,
        effect: &'a Effect,
        this: Option<&CardId>,
    ) -> Flow<()> {
        match effect {
            Effect::Draw { deck, count, keep } => self.draw_and_keep(seat, *deck, *count, *keep),
            Effect::TakeCard { card, otherwise } => {
                if self.may_receive(seat, card)
                    && let Some(taken) = self.take_from_deck(card)?
                {
                    return self.give_card(seat, taken);
                }
                self.apply_effects(seat, otherwise, this)
            }
            Effect::TakeFirst { deck, cards } => self.take_first(seat, *deck, cards),
            Effect::SearchDeck(deck) => self.search_deck(seat, *deck),
            Effect::Shop(decks) => self.shop(seat, decks),
            Effect::Buy {
                draw,
                price,
                everyone,
            } => self.buy(seat, draw, *price, *everyone),
            Effect::BuyAny(decks) => self.buy_any(seat, decks),
            Effect::Sell { deck, multiplier } => self.sell(seat, *deck, *multiplier),
            Effect::Discard { cards, count } => self.discard_some(seat, cards, *count),
            Effect::DiscardAll(cards) => self.discard_all(seat, cards),
            Effect::DiscardHalf { cards, round_up } => self.discard_half(seat, cards, *round_up),
            Effect::DiscardCard(card) => self.discard_named(seat, card),
            Effect::DiscardThis => match this {
                Some(card) => self.discard_card(seat, card),
                None => Ok(()),
            },
            Effect::ReturnThisToBox => {
                if let Some(card) = this {
                    self.return_to_box(seat, card);
                }
                Ok(())
            }
            Effect::ReturnRandomCardToBox(deck) => {
                self.return_random_card_to_box(*deck);
                Ok(())
            }
            _ => self.apply_token_or_reaction(seat, effect, this),
        }
    }

    /// Tokens on cards, and reactions that change what is being resolved.
    fn apply_token_or_reaction(
        &mut self,
        seat: usize,
        effect: &'a Effect,
        this: Option<&CardId>,
    ) -> Flow<()> {
        match effect {
            Effect::AddTokensToThis(count) => {
                if let Some(card) = this {
                    self.add_tokens(seat, card, i16::from(*count));
                }
            }
            Effect::RemoveTokensFromThis(count) => {
                if let Some(card) = this {
                    self.add_tokens(seat, card, -i16::from(*count));
                }
            }
            Effect::CombatBonus(bonus) => self.scratch.combat_bonus += i32::from(*bonus),
            Effect::ReduceLoss(amount) => {
                self.scratch.reduce_loss =
                    self.scratch.reduce_loss.saturating_add(u32::from(*amount));
            }
            Effect::CancelLoss => self.scratch.reduce_loss = u32::MAX,
            Effect::PassCheck => self.scratch.pass_check = true,
            Effect::NoCombatDamage => self.scratch.no_combat_damage = true,
            Effect::Reroll => self.scratch.reroll = true,
            Effect::RerollOneDie => self.scratch.reroll_one_die = true,
            Effect::RedrawEncounter => self.scratch.redraw = true,
            Effect::Special(id) => return self.special_effect(seat, id, this),
            _ => return self.apply_place_effect(seat, effect),
        }
        Ok(())
    }

    /// The location of an investigator, if in one.
    pub fn location_of(&self, seat: usize) -> Option<LocationId> {
        self.inv(seat).place.location().cloned()
    }
}

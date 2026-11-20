//! Conditions printed on cards.

use data::{CardId, Condition};

use crate::game::Game;
use crate::state::Place;

impl Game<'_> {
    /// Whether a condition holds for an investigator.
    pub fn holds(&self, seat: usize, condition: &Condition) -> bool {
        self.holds_for(seat, condition, None)
    }

    /// Whether a condition holds for an investigator, `this` being the card
    /// that carries it (for its tokens).
    pub fn holds_for(&self, seat: usize, condition: &Condition, this: Option<&CardId>) -> bool {
        let inv = self.inv(seat);
        match condition {
            Condition::HasCard(card) => inv.holds(card),
            Condition::HasAny(filter) => self
                .matching_cards(seat, filter)
                .is_ok_and(|cards| !cards.is_empty()),
            Condition::CardCountAtMost { deck, count } => {
                let held = inv
                    .cards
                    .iter()
                    .filter(|held| self.card(&held.card).is_ok_and(|card| card.deck == *deck))
                    .count();
                held <= usize::from(*count)
            }
            Condition::HasAtLeast(resource, amount) => {
                self.amount(seat, *resource) >= u32::from(*amount)
            }
            Condition::InStreets => matches!(inv.place, Place::Street(_)),
            Condition::InStreet(street) => inv.place == Place::Street(street.clone()),
            Condition::AtLocation(location) => inv.place == Place::Location(location.clone()),
            Condition::InArkham => inv.place.in_arkham(),
            Condition::InOtherWorld => matches!(inv.place, Place::OtherWorld { .. }),
            Condition::LostInTimeAndSpace => inv.place == Place::LostInTimeAndSpace,
            Condition::InJail => inv.place == Place::JailCell,
            Condition::TokensOnThisAtLeast(count) => {
                this.is_some_and(|card| self.tokens_on(seat, card) >= *count)
            }
            Condition::ClosingGate => self.scratch.closing_gate,
            Condition::Not(inner) => !self.holds_for(seat, inner, this),
        }
    }

    /// Tokens on a card: the Rumor in play, or a card the investigator holds.
    pub fn tokens_on(&self, seat: usize, card: &CardId) -> u8 {
        if let Some(rumor) = self
            .state
            .rumor
            .as_ref()
            .filter(|rumor| &rumor.card == card)
        {
            return rumor.tokens;
        }
        self.inv(seat)
            .cards
            .iter()
            .find(|held| &held.card == card)
            .map_or(0, |held| held.tokens)
    }

    /// Adds (or with a negative count, removes) tokens on a card.
    pub fn add_tokens(&mut self, seat: usize, card: &CardId, count: i16) {
        let name = self.card_name(card);
        if let Some(rumor) = self
            .state
            .rumor
            .as_mut()
            .filter(|rumor| &rumor.card == card)
        {
            rumor.tokens = clamp_tokens(rumor.tokens, count);
            let tokens = rumor.tokens;
            self.log(format!("La Rumeur porte {tokens} pion(s)."));
            return;
        }
        if let Some(held) = self
            .inv_mut(seat)
            .cards
            .iter_mut()
            .find(|held| &held.card == card)
        {
            held.tokens = clamp_tokens(held.tokens, count);
            let tokens = held.tokens;
            self.log(format!("{name} porte {tokens} pion(s)."));
        }
    }
}

/// Tokens after a change, between 0 and 255.
fn clamp_tokens(tokens: u8, change: i16) -> u8 {
    u8::try_from((i16::from(tokens) + change).clamp(0, 255)).unwrap_or(u8::MAX)
}

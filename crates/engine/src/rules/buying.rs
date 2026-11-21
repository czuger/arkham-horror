//! Buying cards drawn by an effect, and taking the first card of a kind.

use data::{CardFilter, CardId, Deck, Price, Resource};

use crate::decision::{CardAim, Choice};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::card_effects::filter_accepts;

impl Game<'_> {
    /// One card drawn per deck; the buyers (the active investigator, then the
    /// others when `everyone`) may buy them at `price`; the rest is discarded.
    pub fn buy(&mut self, seat: usize, draw: &[Deck], price: Price, everyone: bool) -> Flow<()> {
        let mut drawn: Vec<CardId> = Vec::new();
        for &deck in draw {
            drawn.extend(self.draw_for(seat, deck)?);
        }
        if let Some(&deck) = draw.iter().find(|&&deck| self.draw_bonus(seat, deck) > 0) {
            drawn.extend(self.draw_for(seat, deck)?);
        }
        let mut buyers = vec![seat];
        if everyone {
            buyers.extend(
                self.state
                    .seat_order()
                    .into_iter()
                    .filter(|&other| other != seat),
            );
        }
        for buyer in buyers {
            self.buy_from(buyer, &mut drawn, price)?;
        }
        for card in drawn {
            let deck = self.card(&card)?.deck;
            self.state.decks.get_mut(deck).push(card);
        }
        Ok(())
    }

    /// Lets one investigator buy among the cards on offer, as many as wanted.
    fn buy_from(&mut self, seat: usize, offer: &mut Vec<CardId>, price: Price) -> Flow<()> {
        loop {
            let mut options = vec![option(Choice::Done, "Ne rien acheter de plus")];
            for card in offer.iter() {
                let cost = self.price_of(card, price)?;
                if self.amount(seat, Resource::Money) >= cost {
                    let label = format!("{} ({cost} $)", self.card_name(card));
                    options.push(option(Choice::Card(card.clone()), label));
                }
            }
            let Choice::Card(card) =
                self.pick_card(seat, CardAim::Buy, "Que voulez-vous acheter ?", options)?
            else {
                return Ok(());
            };
            let Some(index) = offer.iter().position(|candidate| candidate == &card) else {
                return Ok(());
            };
            let cost = self.price_of(&card, price)?;
            if !self.pay(seat, Resource::Money, cost)? {
                return Ok(());
            }
            let card = offer.remove(index);
            self.log(format!(
                "{} achète {} pour {cost} $.",
                self.name(seat),
                self.card_name(&card)
            ));
            self.give_card(seat, card)?;
        }
    }

    /// What a card costs at a given price rule.
    fn price_of(&self, card: &CardId, price: Price) -> Flow<u32> {
        let printed = u32::from(self.card(card)?.price.unwrap_or(0));
        Ok(match price {
            Price::Printed => printed,
            Price::Plus(more) => printed + u32::from(more),
            Price::Minus(less) => printed.saturating_sub(u32::from(less)),
            Price::Half => printed.div_ceil(2),
        })
    }

    /// Any card of these decks, bought at its printed price.
    pub fn buy_any(&mut self, seat: usize, decks: &[Deck]) -> Flow<()> {
        let mut offer: Vec<CardId> = Vec::new();
        for &deck in decks {
            for card in self.state.decks.get(deck) {
                if !offer.contains(card) {
                    offer.push(card.clone());
                }
            }
        }
        if self.buy_once(seat, &offer)? {
            for &deck in decks {
                let mut cards = std::mem::take(self.state.decks.get_mut(deck));
                self.state.rng.shuffle(&mut cards);
                *self.state.decks.get_mut(deck) = cards;
            }
        }
        Ok(())
    }

    /// One purchase at most among cards still in their decks; whether one
    /// was made.
    fn buy_once(&mut self, seat: usize, offer: &[CardId]) -> Flow<bool> {
        let mut options = vec![option(Choice::Done, "Ne rien acheter")];
        for card in offer.iter() {
            let cost = self.price_of(card, Price::Printed)?;
            if self.amount(seat, Resource::Money) >= cost {
                let label = format!("{} ({cost} $)", self.card_name(card));
                options.push(option(Choice::Card(card.clone()), label));
            }
        }
        let Choice::Card(card) =
            self.pick_card(seat, CardAim::Buy, "Que voulez-vous acheter ?", options)?
        else {
            return Ok(false);
        };
        let cost = self.price_of(&card, Price::Printed)?;
        if !self.pay(seat, Resource::Money, cost)? {
            return Ok(false);
        }
        if let Some(taken) = self.take_from_deck(&card)? {
            self.log(format!(
                "{} achète {} pour {cost} $.",
                self.name(seat),
                self.card_name(&taken)
            ));
            self.give_card(seat, taken)?;
        }
        Ok(true)
    }

    /// The first card of a kind from the top of a deck; the deck is then
    /// shuffled.
    pub fn take_first(&mut self, seat: usize, deck: Deck, filter: &CardFilter) -> Flow<()> {
        let mut found = None;
        for card in self.state.decks.get(deck) {
            if filter_accepts(self.card(card)?, filter) {
                found = Some(card.clone());
                break;
            }
        }
        if let Some(card) = found
            && let Some(taken) = self.take_from_deck(&card)?
        {
            self.log(format!(
                "{} prend {}.",
                self.name(seat),
                self.card_name(&taken)
            ));
            self.give_card(seat, taken)?;
        }
        let mut cards = std::mem::take(self.state.decks.get_mut(deck));
        self.state.rng.shuffle(&mut cards);
        *self.state.decks.get_mut(deck) = cards;
        Ok(())
    }
}

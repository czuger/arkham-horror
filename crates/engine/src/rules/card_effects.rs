//! Effect primitives that draw, buy, sell or discard investigator cards.

use data::{CardFilter, CardId, Deck};

use crate::decision::{Choice, DecisionKind, DecisionOption};
use crate::game::{Game, option};
use crate::halt::Flow;

impl Game<'_> {
    /// Draws `count` cards and keeps `keep` of them (all when `None`); the
    /// others go to the bottom of the deck.
    pub fn draw_and_keep(
        &mut self,
        seat: usize,
        deck: Deck,
        count: u8,
        keep: Option<u8>,
    ) -> Flow<()> {
        let mut drawn: Vec<CardId> = (0..count).filter_map(|_| self.draw_from(deck)).collect();
        let keep = usize::from(keep.unwrap_or(count)).min(drawn.len());
        for _ in 0..keep {
            let options = self.card_options(&drawn);
            let prompt = "Quelle carte gardez-vous ?";
            let Choice::Card(card) = self.ask(seat, DecisionKind::PickCard, prompt, options)?
            else {
                break;
            };
            if let Some(index) = drawn.iter().position(|candidate| candidate == &card) {
                drawn.remove(index);
            }
            self.give_card(seat, card)?;
        }
        self.state.decks.get_mut(deck).extend(drawn);
        Ok(())
    }

    /// Takes the card of one's choice in a deck, then shuffles it.
    pub fn search_deck(&mut self, seat: usize, deck: Deck) -> Flow<()> {
        let mut cards = self.state.decks.get(deck).clone();
        cards.dedup();
        if cards.is_empty() {
            return Ok(());
        }
        let options = self.card_options(&cards);
        if let Choice::Card(card) = self.ask(
            seat,
            DecisionKind::PickCard,
            "Quelle carte prenez-vous ?",
            options,
        )? && let Some(taken) = self.take_from_deck(&card)?
        {
            self.give_card(seat, taken)?;
        }
        let mut deck_cards = std::mem::take(self.state.decks.get_mut(deck));
        self.state.rng.shuffle(&mut deck_cards);
        *self.state.decks.get_mut(deck) = deck_cards;
        Ok(())
    }

    /// Looks at the top card of each deck and buys any of them at the
    /// printed price.
    pub fn shop(&mut self, seat: usize, decks: &[Deck]) -> Flow<()> {
        loop {
            let mut options = vec![option(Choice::Done, "Ne rien acheter de plus")];
            for &deck in decks {
                let Some(card) = self.state.decks.get(deck).first().cloned() else {
                    continue;
                };
                let price = self.card(&card)?.price.unwrap_or(0);
                if self.amount(seat, data::Resource::Money) >= u32::from(price) {
                    options.push(option(
                        Choice::Card(card.clone()),
                        format!("{} ({price} $)", self.card_name(&card)),
                    ));
                }
            }
            let Choice::Card(card) = self.ask(
                seat,
                DecisionKind::PickCard,
                "Que voulez-vous acheter ?",
                options,
            )?
            else {
                return Ok(());
            };
            let price = self.card(&card)?.price.unwrap_or(0);
            self.pay(seat, data::Resource::Money, u32::from(price))?;
            if let Some(taken) = self.take_from_deck(&card)? {
                self.give_card(seat, taken)?;
            }
        }
    }

    /// May sell one held card of a deck for a multiple of its price.
    pub fn sell(&mut self, seat: usize, deck: Deck, multiplier: u8) -> Flow<()> {
        let mut options = vec![option(Choice::Done, "Ne rien vendre")];
        for held in &self.inv(seat).cards {
            let card = self.card(&held.card)?;
            if card.deck == deck {
                let value = u32::from(card.price.unwrap_or(0)) * u32::from(multiplier);
                options.push(option(
                    Choice::Card(held.card.clone()),
                    format!("{} ({value} $)", card.name),
                ));
            }
        }
        if let Choice::Card(card) =
            self.ask(seat, DecisionKind::PickCard, "Vendre une carte ?", options)?
        {
            let value = u32::from(self.card(&card)?.price.unwrap_or(0)) * u32::from(multiplier);
            self.discard_card(seat, &card)?;
            self.gain(seat, data::Resource::Money, value)?;
        }
        Ok(())
    }

    /// Discards `count` held cards of the player's choice among those that
    /// match (all of them if fewer).
    pub fn discard_some(&mut self, seat: usize, filter: &CardFilter, count: u8) -> Flow<()> {
        for _ in 0..count {
            let matching = self.matching_cards(seat, filter)?;
            if matching.is_empty() {
                return Ok(());
            }
            let options = self.card_options(&matching);
            if let Choice::Card(card) = self.ask(
                seat,
                DecisionKind::PickCard,
                "Quelle carte défaussez-vous ?",
                options,
            )? {
                self.log(format!(
                    "{} défausse {}.",
                    self.name(seat),
                    self.card_name(&card)
                ));
                self.discard_card(seat, &card)?;
            }
        }
        Ok(())
    }

    /// Discards every held card that matches.
    pub fn discard_all(&mut self, seat: usize, filter: &CardFilter) -> Flow<()> {
        for card in self.matching_cards(seat, filter)? {
            self.discard_card(seat, &card)?;
        }
        Ok(())
    }

    /// Held cards matching a filter.
    fn matching_cards(&self, seat: usize, filter: &CardFilter) -> Flow<Vec<CardId>> {
        match filter {
            CardFilter::Items => self.held_items(seat),
            CardFilter::Deck(deck) => {
                let mut found = Vec::new();
                for held in &self.inv(seat).cards {
                    if self.card(&held.card)?.deck == *deck {
                        found.push(held.card.clone());
                    }
                }
                Ok(found)
            }
        }
    }

    /// One option per card.
    fn card_options(&self, cards: &[CardId]) -> Vec<DecisionOption> {
        cards
            .iter()
            .map(|card| option(Choice::Card(card.clone()), self.card_name(card)))
            .collect()
    }
}

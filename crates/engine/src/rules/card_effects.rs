//! Effect primitives that draw, buy, sell or discard investigator cards.

use data::{CardFilter, CardId, Deck, InvestigatorCard, Ongoing};

use crate::decision::{CardAim, Choice, Context, DecisionKind, DecisionOption};
use crate::game::{Game, option};
use crate::halt::Flow;

impl Game<'_> {
    /// Asks which investigator card to pick, and why.
    pub fn pick_card(
        &mut self,
        seat: usize,
        aim: CardAim,
        prompt: &str,
        options: Vec<DecisionOption>,
    ) -> Flow<Choice> {
        let context = Context::Cards(aim);
        self.ask_about(seat, DecisionKind::PickCard, context, prompt, options)
    }

    /// Draws `count` cards (plus the sheet's extra card) and keeps `keep` of
    /// them (`count` when `None`); the others go to the bottom of the deck.
    pub fn draw_and_keep(
        &mut self,
        seat: usize,
        deck: Deck,
        count: u8,
        keep: Option<u8>,
    ) -> Flow<()> {
        let keep = keep.unwrap_or(count);
        let mut drawn: Vec<CardId> = Vec::new();
        for _ in 0..count + self.draw_bonus(seat, deck) {
            drawn.extend(self.draw_for(seat, deck)?);
        }
        let keep = usize::from(keep).min(drawn.len());
        for _ in 0..keep {
            let options = self.card_options(&drawn);
            let prompt = "Quelle carte gardez-vous ?";
            let Choice::Card(card) = self.pick_card(seat, CardAim::Keep, prompt, options)? else {
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
        if let Choice::Card(card) =
            self.pick_card(seat, CardAim::Keep, "Quelle carte prenez-vous ?", options)?
            && let Some(taken) = self.take_from_deck(&card)?
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
            let Choice::Card(card) =
                self.pick_card(seat, CardAim::Buy, "Que voulez-vous acheter ?", options)?
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
            self.pick_card(seat, CardAim::Sell, "Vendre une carte ?", options)?
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
            let matching = self.losable_cards(seat, filter)?;
            if matching.is_empty() {
                return Ok(());
            }
            let options = self.card_options(&matching);
            if let Choice::Card(card) = self.pick_card(
                seat,
                CardAim::Lose,
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
        for card in self.losable_cards(seat, filter)? {
            self.discard_card(seat, &card)?;
        }
        Ok(())
    }

    /// Matching held cards that an effect can take away: not those that
    /// « cannot be lost or stolen ».
    pub fn losable_cards(&self, seat: usize, filter: &CardFilter) -> Flow<Vec<CardId>> {
        let mut found = Vec::new();
        for card in self.matching_cards(seat, filter)? {
            if !self.card(&card)?.ongoing.contains(&Ongoing::CannotBeLost) {
                found.push(card);
            }
        }
        Ok(found)
    }

    /// Held cards matching a filter.
    pub fn matching_cards(&self, seat: usize, filter: &CardFilter) -> Flow<Vec<CardId>> {
        match filter {
            CardFilter::Items => self.held_items(seat),
            CardFilter::Weapons | CardFilter::Tomes => {
                let mut found = Vec::new();
                for card in self.held_items(seat)? {
                    if self
                        .card(&card)
                        .is_ok_and(|printed| filter_accepts(printed, filter))
                    {
                        found.push(card);
                    }
                }
                Ok(found)
            }
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

/// Whether a printed card matches a filter (for cards still in a deck).
pub(crate) fn filter_accepts(card: &InvestigatorCard, filter: &CardFilter) -> bool {
    match filter {
        CardFilter::Items => matches!(card.deck, Deck::CommonItem | Deck::UniqueItem | Deck::Spell),
        CardFilter::Weapons => card.weapon.is_some(),
        CardFilter::Tomes => card.subtitle.as_deref() == Some("Tome"),
        CardFilter::Deck(deck) => card.deck == *deck,
    }
}

//! Discard effects that work out what goes: half the cards, a named card, a
//! random card of a deck.

use data::{CardFilter, CardId, Deck};

use crate::game::Game;
use crate::halt::Flow;

impl Game<'_> {
    /// Discards half the matching cards, chosen by the investigator.
    pub fn discard_half(&mut self, seat: usize, filter: &CardFilter, round_up: bool) -> Flow<()> {
        let held = self.losable_cards(seat, filter)?.len();
        let count = if round_up { held.div_ceil(2) } else { held / 2 };
        self.discard_some(seat, filter, u8::try_from(count).unwrap_or(u8::MAX))
    }

    /// Discards one held copy of a named card, if any.
    pub fn discard_named(&mut self, seat: usize, card: &CardId) -> Flow<()> {
        if self.inv(seat).holds(card) {
            self.log(format!(
                "{} défausse {}.",
                self.name(seat),
                self.card_name(card)
            ));
            self.discard_card(seat, card)?;
        }
        Ok(())
    }

    /// A random card of a deck leaves the game.
    pub fn return_random_card_to_box(&mut self, deck: Deck) {
        let cards = self.state.decks.get(deck);
        if cards.is_empty() {
            return;
        }
        let index = self.state.rng.below(cards.len());
        let card = self.state.decks.get_mut(deck).remove(index);
        self.log(format!(
            "{} est remis dans la boîte.",
            self.card_name(&card)
        ));
    }
}

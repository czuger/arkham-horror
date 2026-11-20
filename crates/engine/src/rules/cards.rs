//! Moving investigator cards between decks, hands and the box.

use data::{CardId, Deck, SpecialId, Timing};

use crate::game::Game;
use crate::halt::Flow;
use crate::state::HeldCard;

/// Special effect: Blessing and Curse cancel each other.
const BLESSING_AND_CURSE: &str = "blessing_and_curse_cancel";
/// The Bank Loan card.
pub(crate) const BANK_LOAN: &str = "bank_loan";

impl Game<'_> {
    /// Removes a held card and puts it at the bottom of its deck.
    pub fn discard_card(&mut self, seat: usize, card: &CardId) -> Flow<()> {
        if self.inv_mut(seat).remove_card(card) {
            let deck = self.card(card)?.deck;
            self.state.decks.get_mut(deck).push(card.clone());
        }
        Ok(())
    }

    /// Removes a held card from the game.
    pub fn return_to_box(&mut self, seat: usize, card: &CardId) {
        if self.inv_mut(seat).remove_card(card) {
            self.log(format!("{} est remis dans la boîte.", self.card_name(card)));
        }
    }

    /// Takes the top card of a deck, if any.
    pub fn draw_from(&mut self, deck: Deck) -> Option<CardId> {
        let cards = self.state.decks.get_mut(deck);
        (!cards.is_empty()).then(|| cards.remove(0))
    }

    /// Takes a given card out of whichever deck holds it.
    pub fn take_from_deck(&mut self, card: &CardId) -> Flow<Option<CardId>> {
        let deck = self.card(card)?.deck;
        let cards = self.state.decks.get_mut(deck);
        Ok(cards
            .iter()
            .position(|candidate| candidate == card)
            .map(|index| cards.remove(index)))
    }

    /// Whether the investigator may receive this card at all.
    pub fn may_receive(&self, seat: usize, card: &CardId) -> bool {
        let inv = self.inv(seat);
        !(card.as_str() == BANK_LOAN && (inv.no_more_bank_loans || inv.holds(card)))
    }

    /// Whether a card is still in its deck.
    pub fn in_deck(&self, card: &CardId) -> Flow<bool> {
        let deck = self.card(card)?.deck;
        Ok(self.state.decks.get(deck).contains(card))
    }

    /// Gives a card taken out of its deck to an investigator. Special cards
    /// are held once at most: a second copy only refreshes the first (FAQ),
    /// and a Blessing cancels a Curse (and back).
    pub fn give_card(&mut self, seat: usize, card: CardId) -> Flow<()> {
        self.covered.insert(crate::coverage::received_key(&card));
        let printed = self.card(&card)?;
        let deck = printed.deck;
        if deck == Deck::Special && self.inv(seat).holds(&card) {
            self.state.decks.get_mut(deck).push(card.clone());
            if let Some(held) = self
                .inv_mut(seat)
                .cards
                .iter_mut()
                .find(|held| held.card == card)
            {
                held.fresh = true;
            }
            return Ok(());
        }
        if printed.special == Some(SpecialId::new(BLESSING_AND_CURSE))
            && let Some(opposite) = self.opposite_of(seat, &card)?
        {
            self.log(format!(
                "{} annule {}.",
                self.card_name(&card),
                self.card_name(&opposite)
            ));
            self.discard_card(seat, &opposite)?;
            self.state.decks.get_mut(deck).push(card);
            return Ok(());
        }
        self.log(format!("{} reçoit {}.", self.name(seat), printed.name));
        self.inv_mut(seat).cards.push(HeldCard::new(card.clone()));
        for ability in printed
            .abilities
            .iter()
            .filter(|ability| ability.timing == Timing::OnTake)
        {
            self.covered.insert(crate::coverage::used_key(&card));
            self.apply_effects(seat, &ability.effects, Some(&card))?;
        }
        Ok(())
    }

    /// The held card that a Blessing or Curse cancels.
    fn opposite_of(&self, seat: usize, card: &CardId) -> Flow<Option<CardId>> {
        for held in &self.inv(seat).cards {
            let other = self.card(&held.card)?;
            if &held.card != card && other.special == Some(SpecialId::new(BLESSING_AND_CURSE)) {
                return Ok(Some(held.card.clone()));
            }
        }
        Ok(None)
    }

    /// Draws a card from a deck and gives it. Returns whether one was drawn.
    pub fn draw_and_give(&mut self, seat: usize, deck: Deck) -> Flow<bool> {
        if self.draw_bonus(seat, deck) > 0 {
            let any = !self.state.decks.get(deck).is_empty();
            self.draw_and_keep(seat, deck, 1, Some(1))?;
            return Ok(any);
        }
        let Some(card) = self.draw_for(seat, deck)? else {
            self.log("Le paquet est vide.");
            return Ok(false);
        };
        self.give_card(seat, card)?;
        Ok(true)
    }
}

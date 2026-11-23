//! Trading equipment between investigators at the same place (§18.6, FAQ
//! §23.7): money, Common and Unique Items, Spells, and the Deputy's items.
//! Offered during one's own movement in Arkham; the other investigator
//! accepts or refuses the whole offer.

use data::CardId;

use crate::decision::{Choice, Context, DecisionKind, DecisionOption, TradeOffer};
use crate::game::{Game, option};
use crate::halt::Flow;

impl Game<'_> {
    /// « Échanger avec … » for each investigator standing with the one at
    /// `seat` in Arkham, when either of them has something to trade.
    pub(crate) fn trade_options(&self, seat: usize) -> Flow<Vec<DecisionOption>> {
        let place = &self.inv(seat).place;
        if !place.in_arkham() {
            return Ok(Vec::new());
        }
        let mut options = Vec::new();
        for (partner, other) in self.state.investigators.iter().enumerate() {
            if partner == seat || other.eliminated || &other.place != place {
                continue;
            }
            let something = self.inv(seat).money > 0
                || other.money > 0
                || !self.held_items(seat)?.is_empty()
                || !self.held_items(partner)?.is_empty();
            if something {
                let label = format!("Échanger avec {}", self.name(partner));
                options.push(option(Choice::Investigator(partner), label));
            }
        }
        Ok(options)
    }

    /// The investigator at `seat` puts an offer together, item by item,
    /// then proposes it to `partner`, who accepts or refuses.
    pub(crate) fn trade(&mut self, seat: usize, partner: usize) -> Flow<()> {
        let mut offer = TradeOffer::new(seat, partner);
        loop {
            let options = self.offer_options(&offer)?;
            let prompt = format!("Échange avec {}", self.name(partner));
            let context = Context::Trade(Box::new(offer.clone()));
            match self.ask_about(seat, DecisionKind::Trade, context, prompt, options)? {
                Choice::Give(card) => offer.give.push(card),
                Choice::Take(card) => offer.take.push(card),
                Choice::GiveMoney(amount) => offer.give_money = amount,
                Choice::TakeMoney(amount) => offer.take_money = amount,
                Choice::Yes => break,
                _ => return Ok(()),
            }
        }
        self.propose(offer)
    }

    /// What may still be added to the offer, then cancel or propose.
    fn offer_options(&self, offer: &TradeOffer) -> Flow<Vec<DecisionOption>> {
        let mut options = vec![option(Choice::Done, "Annuler l'échange")];
        if !offer.is_empty() {
            options.push(option(Choice::Yes, "Proposer l'échange"));
        }
        for card in left_out(self.held_items(offer.trader)?, &offer.give) {
            let label = format!("Donner {}", self.card(&card)?.name);
            options.push(option(Choice::Give(card), label));
        }
        for card in left_out(self.held_items(offer.partner)?, &offer.take) {
            let label = format!("Prendre {}", self.card(&card)?.name);
            options.push(option(Choice::Take(card), label));
        }
        for amount in (1..=self.inv(offer.trader).money).filter(|n| *n != offer.give_money) {
            options.push(option(
                Choice::GiveMoney(amount),
                format!("Donner {amount} $"),
            ));
        }
        for amount in (1..=self.inv(offer.partner).money).filter(|n| *n != offer.take_money) {
            options.push(option(
                Choice::TakeMoney(amount),
                format!("Prendre {amount} $"),
            ));
        }
        Ok(options)
    }

    /// Asks the partner, then makes the exchange if they accept.
    fn propose(&mut self, offer: TradeOffer) -> Flow<()> {
        if offer.is_empty() {
            return Ok(());
        }
        let trader = self.name(offer.trader);
        let partner = self.name(offer.partner);
        let gives = self.offer_text(&offer.give, offer.give_money)?;
        let takes = self.offer_text(&offer.take, offer.take_money)?;
        let prompt = format!(
            "{trader} vous propose un échange. Vous recevez : {gives}. Vous donnez : {takes}."
        );
        let options = vec![
            option(Choice::Yes, "Accepter l'échange"),
            option(Choice::Done, "Refuser l'échange"),
        ];
        let context = Context::Trade(Box::new(offer.clone()));
        let answer = self.ask_about(
            offer.partner,
            DecisionKind::AcceptTrade,
            context,
            prompt,
            options,
        )?;
        if answer != Choice::Yes {
            self.log(format!("{partner} refuse l'échange proposé par {trader}."));
            return Ok(());
        }
        self.hand_over(offer.trader, offer.partner, &offer.give, offer.give_money);
        self.hand_over(offer.partner, offer.trader, &offer.take, offer.take_money);
        self.log(format!("{trader} échange avec {partner}."));
        if gives != "rien" {
            self.log(format!("{trader} donne à {partner} : {gives}."));
        }
        if takes != "rien" {
            self.log(format!("{partner} donne à {trader} : {takes}."));
        }
        Ok(())
    }

    /// « Revolver .38, Lanterne et 3 $ », or « rien ».
    fn offer_text(&self, cards: &[CardId], money: u16) -> Flow<String> {
        let mut parts = Vec::new();
        for card in cards {
            parts.push(self.card(card)?.name.clone());
        }
        if money > 0 {
            parts.push(format!("{money} $"));
        }
        Ok(match parts.split_last() {
            None => "rien".to_owned(),
            Some((last, [])) => last.clone(),
            Some((last, rest)) => format!("{} et {last}", rest.join(", ")),
        })
    }

    /// Moves items, as they stand, and money from one investigator to another.
    fn hand_over(&mut self, from: usize, to: usize, cards: &[CardId], money: u16) {
        for card in cards {
            if let Some(held) = self.inv_mut(from).take_card(card) {
                self.inv_mut(to).cards.push(held);
            }
        }
        let paid = money.min(self.inv(from).money);
        self.inv_mut(from).money -= paid;
        self.inv_mut(to).money += paid;
    }
}

/// The items not yet in the offer: each copy put in is taken off once.
fn left_out(mut held: Vec<CardId>, offered: &[CardId]) -> Vec<CardId> {
    for card in offered {
        if let Some(index) = held.iter().position(|item| item == card) {
            held.remove(index);
        }
    }
    held
}

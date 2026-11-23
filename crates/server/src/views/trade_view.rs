//! The trade page: what the human may give and take, all picked at once.

use engine::{Choice, Context, Decision, DecisionKind, View};

use crate::views::image_token::ImageToken;
use crate::views::names::card;

/// Whether a decision is a trade being put together.
pub fn is_trade(decision: &Decision) -> bool {
    decision.kind == DecisionKind::Trade
}

/// The trade page: both investigators' items and money.
#[derive(Debug, Clone)]
pub struct TradeView {
    /// Who proposes.
    pub trader: String,
    /// Who is asked.
    pub partner: String,
    /// Items the trader may give.
    pub give: Vec<TradeCard>,
    /// Items the trader may take.
    pub take: Vec<TradeCard>,
    /// Dollars the trader has.
    pub money: u16,
    /// Dollars the partner has.
    pub partner_money: u16,
    /// Number of answers given so far, sent back with the form.
    pub move_number: usize,
}

/// One item that may change hands: a checkbox with the card.
#[derive(Debug, Clone)]
pub struct TradeCard {
    /// Form field (`donner0`, `prendre1`…), unique even for two copies.
    pub field: String,
    /// Card id, the field's value.
    pub id: String,
    /// Name and image.
    pub token: ImageToken,
}

impl TradeView {
    /// The page for the trade `decision`; `None` for any other decision.
    pub fn new(view: &View<'_, '_>, decision: &Decision, move_number: usize) -> Option<Self> {
        let Context::Trade(offer) = &decision.context else {
            return None;
        };
        if !is_trade(decision) {
            return None;
        }
        let investigators = &view.state().investigators;
        Some(Self {
            trader: view.name(offer.trader),
            partner: view.name(offer.partner),
            give: cards(view, decision, "donner", |choice| match choice {
                Choice::Give(id) => Some(id.as_str()),
                _ => None,
            }),
            take: cards(view, decision, "prendre", |choice| match choice {
                Choice::Take(id) => Some(id.as_str()),
                _ => None,
            }),
            money: investigators.get(offer.trader)?.money,
            partner_money: investigators.get(offer.partner)?.money,
            move_number,
        })
    }
}

/// The items one option kind offers, each with its own field.
fn cards(
    view: &View<'_, '_>,
    decision: &Decision,
    field: &str,
    pick: impl Fn(&Choice) -> Option<&str>,
) -> Vec<TradeCard> {
    decision
        .options
        .iter()
        .filter_map(|option| pick(&option.choice))
        .enumerate()
        .map(|(index, id)| TradeCard {
            field: format!("{field}{index}"),
            id: id.to_owned(),
            token: card(view.data(), &data::CardId::new(id)),
        })
        .collect()
}

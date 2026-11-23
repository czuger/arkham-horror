//! A trade being put together between two investigators.

use data::CardId;
use serde::{Deserialize, Serialize};

/// What one investigator offers another: items and money each way.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeOffer {
    /// Seat of the investigator who proposes.
    pub trader: usize,
    /// Seat of the investigator asked.
    pub partner: usize,
    /// Items the trader gives.
    pub give: Vec<CardId>,
    /// Items the trader takes from the partner.
    pub take: Vec<CardId>,
    /// Dollars the trader gives.
    pub give_money: u16,
    /// Dollars the trader takes from the partner.
    pub take_money: u16,
}

impl TradeOffer {
    /// An empty offer from `trader` to `partner`.
    pub fn new(trader: usize, partner: usize) -> Self {
        Self {
            trader,
            partner,
            ..Self::default()
        }
    }

    /// Whether nothing would change hands.
    pub fn is_empty(&self) -> bool {
        self.give.is_empty() && self.take.is_empty() && self.give_money == 0 && self.take_money == 0
    }
}

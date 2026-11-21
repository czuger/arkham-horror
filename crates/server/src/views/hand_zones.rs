use data::{CardId, GameData};
use engine::{Choice, Decision, HANDS, InvestigatorState};

use crate::views::image_token::ImageToken;
use crate::views::names::card;
use crate::views::sheet_decision::prepares_combat;

/// An investigator's possessions, split between the cards held in hand and
/// the others. Weapons stay in hand from one combat to the next (R28).
#[derive(Debug, Clone)]
pub struct HandZones {
    /// Whether cards can be moved between the zones now.
    pub active: bool,
    /// Hands taken by the cards in hand.
    pub used: u8,
    /// Hands the investigator has.
    pub total: u8,
    /// Cards held in hand.
    pub in_hand: Vec<HeldCardView>,
    /// The other cards.
    pub possessions: Vec<HeldCardView>,
}

/// A card an investigator holds.
#[derive(Debug, Clone)]
pub struct HeldCardView {
    /// Name, image and bubble.
    pub token: ImageToken,
    /// Turned after use, until the next upkeep.
    pub exhausted: bool,
    /// Tokens on the card.
    pub tokens: u8,
    /// Hands it takes in a combat (0 for cards not held in hand).
    pub hands: u8,
    /// Option that moves it to the other zone, when it can move now.
    pub choice: Option<usize>,
    /// Why it cannot be taken in hand now (empty when it can, or never could).
    pub note: String,
}

impl HandZones {
    /// The zones of an investigator. `decision` is the human's open
    /// decision when it is this investigator's: weapons and spells move
    /// only while a combat is being prepared; otherwise the hands hold the
    /// weapons kept from the last combat.
    pub fn new(data: &GameData, inv: &InvestigatorState, decision: Option<&Decision>) -> Self {
        let decision = decision.filter(|decision| prepares_combat(decision));
        let mut equipped = held_in_hand(inv, decision);
        let takes = moves(decision, |choice| match choice {
            Choice::Card(card) => Some(card),
            _ => None,
        });
        let mut zones = Self {
            active: decision.is_some(),
            used: 0,
            total: HANDS,
            in_hand: Vec::new(),
            possessions: Vec::new(),
        };
        for held in &inv.cards {
            let mut view = HeldCardView {
                token: card(data, &held.card),
                exhausted: held.exhausted,
                tokens: held.tokens,
                hands: hands(data, &held.card),
                choice: None,
                note: String::new(),
            };
            if let Some(at) = equipped.iter().position(|(id, _)| id == &held.card) {
                view.choice = equipped.swap_remove(at).1;
                zones.used += view.hands;
                zones.in_hand.push(view);
            } else {
                view.choice = takes
                    .iter()
                    .find(|(id, _)| id == &held.card)
                    .map(|(_, i)| *i);
                zones.possessions.push(view);
            }
        }
        if zones.active {
            zones.explain_refusals();
        }
        zones
    }

    /// Notes why weapons and spells left out cannot be taken in hand.
    fn explain_refusals(&mut self) {
        let free = self.total.saturating_sub(self.used);
        for view in &mut self.possessions {
            if view.choice.is_some() || view.hands == 0 {
                continue;
            }
            view.note = if view.hands > free {
                format!("Il faut {} main(s) libre(s)", view.hands)
            } else if view.exhausted {
                "Épuisée".to_owned()
            } else {
                "Inutilisable maintenant".to_owned()
            };
        }
    }
}

/// The cards in hand, with the option that lets each go while a combat is
/// being prepared.
fn held_in_hand(
    inv: &InvestigatorState,
    decision: Option<&Decision>,
) -> Vec<(CardId, Option<usize>)> {
    if decision.is_none() {
        return inv
            .weapons_in_hand()
            .into_iter()
            .map(|card| (card, None))
            .collect();
    }
    moves(decision, |choice| match choice {
        Choice::Drop(card) => Some(card),
        _ => None,
    })
    .into_iter()
    .map(|(card, index)| (card, Some(index)))
    .collect()
}

/// The cards an open decision can move, with the option index.
fn moves<'d>(
    decision: Option<&'d Decision>,
    pick: impl Fn(&'d Choice) -> Option<&'d CardId>,
) -> Vec<(CardId, usize)> {
    decision.map_or_else(Vec::new, |decision| {
        decision
            .options
            .iter()
            .enumerate()
            .filter_map(|(index, option)| pick(&option.choice).map(|id| (id.clone(), index)))
            .collect()
    })
}

/// Hands a card takes in a combat.
fn hands(data: &GameData, id: &CardId) -> u8 {
    data.investigator_cards
        .card(id)
        .map_or(0, |card| card.hands)
}

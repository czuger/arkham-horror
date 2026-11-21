//! One investigator during the game.

use data::{CardId, InvestigatorId, LocationId, MonsterId};
use serde::{Deserialize, Serialize};

use crate::state::held_card::HeldCard;
use crate::state::place::Place;

/// Everything that changes about an investigator during the game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvestigatorState {
    /// The sheet played.
    pub id: InvestigatorId,
    /// Where the investigator stands.
    pub place: Place,
    /// Current Sanity.
    pub sanity: u8,
    /// Current Stamina.
    pub stamina: u8,
    /// Lasting change to the printed maximum Sanity (Cthulhu's attack…).
    pub max_sanity_change: i8,
    /// Lasting change to the printed maximum Stamina.
    pub max_stamina_change: i8,
    /// Slider stop (0 to 3) on the Speed/Sneak, Fight/Will and Lore/Luck tracks.
    pub sliders: [u8; 3],
    /// Money, in dollars.
    pub money: u16,
    /// Clue tokens.
    pub clues: u16,
    /// Cards held.
    pub cards: Vec<HeldCard>,
    /// Marker laid down: no movement at the next Movement phase.
    pub delayed: bool,
    /// « Perdez votre prochain tour » is pending.
    pub lose_next_turn: bool,
    /// The current turn is lost: every phase but the Mythos is skipped.
    pub skipping_turn: bool,
    /// No more Arkham encounter this turn (insane or unconscious).
    pub encounters_over: bool,
    /// Holds an explored marker at its location.
    pub explored: bool,
    /// Came back from an Other World this turn: may ignore the monsters there.
    pub returned_this_turn: bool,
    /// Monster trophies.
    pub monster_trophies: Vec<MonsterId>,
    /// Gate trophies: indices into the gate marker list of the game.
    pub gate_trophies: Vec<usize>,
    /// May no longer take a Bank Loan.
    pub no_more_bank_loans: bool,
    /// Out of the game (devoured during the final battle).
    pub eliminated: bool,
    /// Bonus to every check until the end of the turn.
    #[serde(default)]
    pub turn_bonus: i8,
    /// Location of the gate the investigator last went through.
    #[serde(default)]
    pub entered_gate: Option<LocationId>,
    /// Once-per-turn sheet ability already used this turn (Mandy).
    #[serde(default)]
    pub power_used: bool,
    /// Weapons kept in hand from one combat to the next (R28).
    #[serde(default)]
    pub in_hand: Vec<CardId>,
}

impl InvestigatorState {
    /// Whether the investigator holds a copy of a card.
    pub fn holds(&self, card: &CardId) -> bool {
        self.cards.iter().any(|held| &held.card == card)
    }

    /// Removes one copy of a card. Returns whether one was found.
    pub fn remove_card(&mut self, card: &CardId) -> bool {
        let Some(index) = self.cards.iter().position(|held| &held.card == card) else {
            return false;
        };
        self.cards.remove(index);
        let left = self.copies(card);
        if self.in_hand.iter().filter(|kept| *kept == card).count() > left
            && let Some(at) = self.in_hand.iter().position(|kept| kept == card)
        {
            self.in_hand.remove(at);
        }
        true
    }

    /// Copies of a card held.
    fn copies(&self, card: &CardId) -> usize {
        self.cards.iter().filter(|held| &held.card == card).count()
    }

    /// The weapons kept in hand that are still held.
    pub fn weapons_in_hand(&self) -> Vec<CardId> {
        let mut kept: Vec<CardId> = Vec::new();
        for card in &self.in_hand {
            let used = kept.iter().filter(|other| *other == card).count();
            if used < self.copies(card) {
                kept.push(card.clone());
            }
        }
        kept
    }
}

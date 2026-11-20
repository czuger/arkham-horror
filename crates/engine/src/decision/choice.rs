//! What can be answered to a decision.

use data::{CardId, CheckKind, LocationId};
use serde::{Deserialize, Serialize};

use crate::state::Place;

/// One possible answer, in a form the AI can reason about.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Choice {
    /// Stop, decline, or go on without doing anything more.
    Done,
    /// Accept.
    Yes,
    /// Put a skill slider on a stop.
    Slider {
        /// Track: 0 Speed/Sneak, 1 Fight/Will, 2 Lore/Luck.
        track: usize,
        /// Target stop, 0 to 3.
        stop: u8,
    },
    /// Go to a place.
    Place(Place),
    /// Try to evade a monster (index in the list of monsters faced).
    Evade(usize),
    /// Fight a monster (index in the list of monsters faced).
    Fight(usize),
    /// Flee during a combat.
    Flee,
    /// Attack during a combat.
    Attack,
    /// Use, play or pick a card.
    Card(CardId),
    /// Let go of a card held in hand during a combat.
    Drop(CardId),
    /// Pick the option with this index in the card text.
    Option(usize),
    /// Try to close the gate with this check (Lore or Fight).
    CloseWith(CheckKind),
    /// Seal the gate with an Elder Sign card.
    ElderSign,
    /// Spend clue tokens to seal the gate.
    SealWithClues,
    /// Spend one clue token for one more die.
    SpendClue,
    /// Discard a card instead of spending a clue token.
    SubstituteClue(CardId),
    /// Pick an investigator (by seat).
    Investigator(usize),
    /// Pick an open gate.
    Gate(LocationId),
    /// Pick a number.
    Number(u32),
}

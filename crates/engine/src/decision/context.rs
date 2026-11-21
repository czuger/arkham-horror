//! What a decision is about, beyond its prompt.

use data::{CheckKind, MonsterId};
use serde::{Deserialize, Serialize};

/// Facts about a decision that the prompt only gives in words.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Context {
    /// Nothing more than the prompt and the options.
    #[default]
    None,
    /// A check under way (clue tokens may be spent on it).
    Check {
        /// The check made.
        kind: CheckKind,
        /// Successes rolled so far.
        successes: u32,
        /// Successes needed.
        needed: u32,
        /// The check closes a gate.
        closing_gate: bool,
    },
    /// The monsters faced. `Choice::Evade(i)` and `Choice::Fight(i)` are
    /// indices into this list.
    Monsters(Vec<MonsterId>),
    /// A combat round under way.
    Combat {
        /// The monster fought; `None` against the Ancient One.
        foe: Option<MonsterId>,
        /// Combat bonus of the cards in hand.
        bonus: i32,
    },
    /// A gate the investigator may try to close.
    Gate {
        /// Modifier of the closing check.
        modifier: i32,
        /// Successes needed to close it.
        needed: u32,
        /// Clue tokens it costs to seal it once closed.
        seal_cost: u16,
    },
    /// Investigator cards to pick, and why.
    Cards(CardAim),
}

/// Why investigator cards are picked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardAim {
    /// Cards kept or taken for free.
    Keep,
    /// Cards bought (only affordable ones are offered).
    Buy,
    /// A card sold.
    Sell,
    /// Cards lost: the one picked goes away.
    Lose,
}

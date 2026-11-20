//! The extra parts of a Rumeur Mythos card.

use serde::{Deserialize, Serialize};

use crate::cards::effect::Effect;
use crate::deck::Deck;
use crate::ids::{LocationId, SpecialId, StreetId};

/// When a rumor ends, one way or the other.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RumorCondition {
    /// The card holds at least this many tokens (or monsters).
    TokensAtLeast(u8),
    /// The card holds no token (or monster) any more.
    NoTokens,
    /// An investigator did what the rumor's action asks.
    Action,
    /// The Ancient One awakes.
    AncientOneAwakens,
    /// The Terror level reaches N.
    TerrorAtLeast(u8),
    /// The deck is empty at the end of a Mythos phase.
    DeckEmpty(Deck),
    /// Coded by name in the engine.
    Special(SpecialId),
}

/// One outcome of a rumor: its condition and what follows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RumorEnd {
    /// When this outcome happens.
    pub condition: RumorCondition,
    /// What happens then; the card leaves play afterwards.
    #[serde(default)]
    pub effects: Vec<Effect>,
}

/// What an investigator may do against a rumor during the Arkham encounters,
/// in the activity street (or on a given location) instead of an encounter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RumorAction {
    /// What the player sees, in French.
    pub label: String,
    /// A location instead of the activity street.
    #[serde(default)]
    pub location: Option<LocationId>,
    /// What it does; it starts with its costs.
    pub effects: Vec<Effect>,
}

/// What a Rumeur adds to a Mythos card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rumor {
    /// Street area marked with the activity marker.
    pub activity: StreetId,
    /// Effects when the card comes into play.
    #[serde(default)]
    pub setup: Vec<Effect>,
    /// Effects at the end of each Mythos phase, from the next one on.
    #[serde(default)]
    pub each_mythos: Vec<Effect>,
    /// What investigators may do against it.
    #[serde(default)]
    pub action: Option<RumorAction>,
    /// Réussite.
    pub pass: RumorEnd,
    /// Échec.
    pub fail: RumorEnd,
}

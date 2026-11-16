//! The extra parts of a Rumeur Mythos card.

use serde::{Deserialize, Serialize};

use crate::cards::effect::Effect;
use crate::ids::{SpecialId, StreetId};

/// When a rumor ends, one way or the other.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RumorCondition {
    /// The card holds at least this many tokens.
    TokensAtLeast(u8),
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

/// What a Rumeur adds to a Mythos card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rumor {
    /// Street area marked with the activity marker.
    pub activity: StreetId,
    /// Effects at the end of each Mythos phase, from the next one on.
    #[serde(default)]
    pub each_mythos: Vec<Effect>,
    /// Réussite.
    pub pass: RumorEnd,
    /// Échec.
    pub fail: RumorEnd,
}

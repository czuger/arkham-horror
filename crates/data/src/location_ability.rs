//! The special ability printed under some Arkham locations.

use serde::{Deserialize, Serialize};

use crate::cards::Effect;

/// A location's ability, used instead of an encounter there.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocationAbility {
    /// Name, in French (« Prêt Bancaire »).
    pub name: String,
    /// Text as printed, in French.
    pub text: String,
    /// What using it does; costs come first. Empty when the ability is not
    /// an action (« Cœur de la Loge »).
    #[serde(default)]
    pub effects: Vec<Effect>,
}

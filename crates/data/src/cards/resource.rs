//! What an investigator gains, loses or pays.

use serde::{Deserialize, Serialize};

/// A counted resource of an investigator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Resource {
    /// Santé Mentale.
    Sanity,
    /// Résistance.
    Stamina,
    /// Argent, in dollars.
    Money,
    /// Pions Indice.
    Clues,
}

//! An open gate.

use data::LocationId;
use serde::{Deserialize, Serialize};

/// A gate open on a location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenGate {
    /// Where it is open.
    pub location: LocationId,
    /// Index of its marker in the gate marker list of the game.
    pub marker: usize,
}

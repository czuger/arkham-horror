//! Quantities that are worked out while an effect runs.

use serde::{Deserialize, Serialize};

use crate::cards::resource::Resource;

/// How much is gained or lost when the card does not print a plain number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Amount {
    /// A printed number.
    Fixed(u8),
    /// The value rolled by the last [`crate::Effect::RollDice`] or
    /// [`crate::Effect::RollDie`], or chosen by the last
    /// [`crate::Effect::ChooseUpTo`] (« autant »).
    X,
    /// That value minus N, never below 0 (« retirez 2, pour un minimum de 0 »).
    XMinus(u8),
    /// N for each success of the last check or dice roll.
    Successes(u8),
    /// Half of what the investigator has, rounded up.
    HalfUp(Resource),
}

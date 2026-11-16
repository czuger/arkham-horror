//! A monster on the board.

use data::MonsterId;
use serde::{Deserialize, Serialize};

use crate::state::place::MonsterPlace;

/// A monster token on the board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardMonster {
    /// Which monster.
    pub monster: MonsterId,
    /// Where it stands.
    pub place: MonsterPlace,
}

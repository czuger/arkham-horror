//! Counts of notable events over a game, for the simulator's statistics.

use serde::Serialize;

/// What happened to the investigators over a game.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct Tally {
    /// Investigators knocked out or driven insane.
    pub knocked_out: u32,
    /// Investigators devoured.
    pub devoured: u32,
    /// Investigators lost in time and space.
    pub lost_in_time: u32,
    /// Monsters defeated.
    pub monsters_defeated: u32,
    /// Gates closed (sealed ones included).
    pub gates_closed: u32,
    /// Gates sealed.
    pub gates_sealed: u32,
    /// Gates opened.
    pub gates_opened: u32,
    /// Why the Ancient One awoke, if it did.
    pub awakening: Option<&'static str>,
}

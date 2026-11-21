//! What the journal shows beside its lines: cards drawn, dice rolled,
//! combats begun and ended.

use data::{CardId, MonsterId};

/// Something worth showing next to a journal line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shown {
    /// Index of the journal line it goes with: the first line written
    /// after it (equal to the journal length if none was).
    pub line: usize,
    /// What it is.
    pub what: Exhibit,
}

/// A card drawn, a roll of dice, or the start or end of a combat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exhibit {
    /// A Mythos, location or gate card drawn, or an investigator card
    /// received.
    Card {
        /// Its deck.
        deck: Deck,
        /// The card.
        card: CardId,
        /// Who drew it (the first player for a Mythos card).
        seat: usize,
    },
    /// Dice rolled, for a check or for their value.
    Dice {
        /// Who rolled; `None` for a monster (the Chthonian).
        seat: Option<usize>,
        /// Each die, from 1 to 6.
        faces: Vec<u8>,
        /// What the roll is counted by.
        count: DiceCount,
    },
    /// An investigator faces a monster: evading it or fighting it.
    CombatStart {
        /// The investigator.
        seat: usize,
        /// The monster.
        foe: MonsterId,
    },
    /// Facing a monster is over.
    CombatEnd {
        /// The investigator.
        seat: usize,
        /// How it ended.
        end: CombatEnd,
    },
}

/// What a roll of dice is counted by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiceCount {
    /// Successes, as in a check.
    Successes {
        /// Lowest face that is a success (5, or 4 when blessed).
        success_from: u8,
        /// Each 6 counts as 2 successes (Shotgun).
        sixes_double: bool,
    },
    /// The faces added up (« lancez un dé »).
    Total,
}

/// The deck a card was drawn from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Deck {
    /// Mythos cards.
    Mythos,
    /// Arkham location cards.
    Location,
    /// Gate cards (Other World encounters).
    Gate,
    /// An investigator card received during the game (item, spell, skill,
    /// ally, special card).
    Investigator,
}

/// How facing a monster ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatEnd {
    /// The investigator evaded the monster.
    Evaded,
    /// The monster was defeated.
    Defeated,
    /// The investigator fled, or the monster could not be fought.
    Fled,
    /// The investigator was knocked out.
    KnockedOut,
    /// The game stopped it (the Ancient One awoke, the game ended).
    Stopped,
}

impl Exhibit {
    /// The successes a roll gives.
    pub fn successes(faces: &[u8], success_from: u8, sixes_double: bool) -> u32 {
        faces
            .iter()
            .map(|&face| match face {
                6 if sixes_double => 2,
                face if face >= success_from => 1,
                _ => 0,
            })
            .sum()
    }
}

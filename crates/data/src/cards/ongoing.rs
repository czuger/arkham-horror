//! Effects that last while a card is in play or held.

use serde::{Deserialize, Serialize};

use crate::cards::check_kind::CheckKind;
use crate::cards::condition::Condition;
use crate::cards::effect::Effect;
use crate::cards::resource::Resource;
use crate::ids::MonsterId;

/// Where a lasting modifier applies.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scope {
    /// Everywhere, Other Worlds included.
    #[default]
    Everywhere,
    /// Only in Arkham (« à Arkham »).
    Arkham,
}

/// A moment of the turn that can trigger a lasting effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Moment {
    /// After an investigator ends their movement.
    EndOfMovement,
    /// At the end of the Mythos phase.
    EndOfMythos,
}

/// A modifier or trigger active while its card is in play (Environnement,
/// Rumeur) or held by an investigator (items, skills, allies…).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Ongoing {
    /// +N (or −N) to every check of this kind.
    CheckBonus {
        /// The check modified.
        check: CheckKind,
        /// The bonus, negative for a penalty.
        amount: i8,
        /// Where it applies.
        #[serde(default)]
        scope: Scope,
    },
    /// +N to a skill, so to every check derived from it.
    SkillBonus {
        /// One of the six skills.
        skill: CheckKind,
        /// The bonus.
        amount: i8,
    },
    /// Each Clue spent on this kind of check adds one more die.
    ExtraClueDie(CheckKind),
    /// +N to the maximum Sanity or Stamina.
    MaxBonus(Resource, i8),
    /// Dice succeed on this face and above (Béni : 4, Maudit : 6).
    SuccessOn(u8),
    /// The card may be discarded instead of spending one Clue.
    ClueSubstitute,
    /// +N movement points during the Movement phase (negative: fewer).
    MovementPoints(i8),
    /// Every spell costs this much Sanity to cast.
    SpellSanityCost(u8),
    /// These monsters get +N toughness.
    MonsterToughness {
        /// The monsters affected.
        monsters: Vec<MonsterId>,
        /// The change.
        amount: i8,
    },
    /// This monster goes back to the cup, and is redrawn if it comes out.
    MonsterBanned(MonsterId),
    /// Runs `effects` for each investigator matching `only` at a moment.
    Trigger {
        /// When it fires.
        when: Moment,
        /// Investigators affected; all when absent.
        #[serde(default)]
        only: Option<Condition>,
        /// What happens to each of them.
        effects: Vec<Effect>,
    },
}

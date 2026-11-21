//! Monster tokens.

use serde::{Deserialize, Serialize};

use crate::dimension_symbol::DimensionSymbol;
use crate::ids::MonsterId;

/// How a monster moves, given by the border colour of its token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Movement {
    /// Black border: one step along the arrow.
    Normal,
    /// Red border: two steps along the arrow.
    Fast,
    /// Yellow border: never moves.
    Stationary,
    /// Blue border: swoops on investigators in the streets, or goes to the Sky.
    Flying,
    /// Green border: special movement described on the token.
    Special,
}

impl Movement {
    /// French name of the movement type.
    pub fn label(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Fast => "rapide",
            Self::Stationary => "immobile",
            Self::Flying => "volant",
            Self::Special => "spécial",
        }
    }
}

/// A keyword ability printed in bold on the combat side of a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonsterAbility {
    /// Embuscade: once combat starts, the investigator cannot flee.
    Ambush,
    /// Inépuisable: never kept as a trophy; back to the cup when defeated.
    Endless,
    /// Mort-vivant.
    Undead,
    /// Masque: only in the cup when Nyarlathotep is the Ancient One.
    Mask,
    /// Résistance Physique: physical weapon bonuses halved.
    PhysicalResistance,
    /// Immunité Physique: physical weapon bonuses ignored.
    PhysicalImmunity,
    /// Résistance Magique: magical weapon and spell bonuses halved.
    MagicalResistance,
    /// Immunité Magique: magical weapon and spell bonuses ignored.
    MagicalImmunity,
    /// Cauchemardesque N: lose N Sanity even after passing the Horror check.
    Nightmarish(u8),
    /// Irrésistible N: lose N Stamina even after passing the Combat check.
    Overwhelming(u8),
}

impl MonsterAbility {
    /// French name of the ability, as printed (« Cauchemardesque 1 »).
    pub fn label(&self) -> String {
        match self {
            Self::Ambush => "Embuscade".into(),
            Self::Endless => "Inépuisable".into(),
            Self::Undead => "Mort-vivant".into(),
            Self::Mask => "Masque".into(),
            Self::PhysicalResistance => "Résistance Physique".into(),
            Self::PhysicalImmunity => "Immunité Physique".into(),
            Self::MagicalResistance => "Résistance Magique".into(),
            Self::MagicalImmunity => "Immunité Magique".into(),
            Self::Nightmarish(n) => format!("Cauchemardesque {n}"),
            Self::Overwhelming(n) => format!("Irrésistible {n}"),
        }
    }
}

/// A check printed on the combat side: its modifier and the damage of a failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Check {
    /// Modifier to the investigator's skill.
    pub modifier: i8,
    /// Sanity (Horror) or Stamina (Combat) lost on a failure. Zero when the
    /// token prints a special effect instead.
    pub damage: u8,
}

/// One kind of monster token, with its number of copies in the box.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Monster {
    /// Identifier used by the other data files.
    pub id: MonsterId,
    /// Name shown to the player, in French, as printed.
    pub name: String,
    /// Number of tokens in the box.
    pub copies: u8,
    /// Dimensional symbol (movement side).
    pub symbol: DimensionSymbol,
    /// Movement type (border colour).
    pub movement: Movement,
    /// Awareness: modifier to Sneak checks made to evade it.
    pub awareness: i8,
    /// Horror check, if the token has one.
    #[serde(default)]
    pub horror: Option<Check>,
    /// Combat check, if the token has one.
    #[serde(default)]
    pub combat: Option<Check>,
    /// Toughness: successes needed to defeat it, and its trophy value.
    pub toughness: u8,
    /// Keyword abilities.
    #[serde(default)]
    pub abilities: Vec<MonsterAbility>,
    /// Special text printed on the token, in French; coded by name in the engine.
    #[serde(default)]
    pub special: Option<String>,
    /// Token image (both sides), relative to the `assets/` folder.
    pub image: String,
}

/// Content of `data/monsters.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Monsters {
    /// Format version of the file.
    pub version: u32,
    /// The monster kinds.
    pub monsters: Vec<Monster>,
}

impl Monsters {
    /// Format version this code reads.
    pub const VERSION: u32 = 1;

    /// Total number of monster tokens.
    pub fn total_copies(&self) -> u32 {
        self.monsters
            .iter()
            .map(|monster| u32::from(monster.copies))
            .sum()
    }
}

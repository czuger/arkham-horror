//! Ancient One sheets.

use serde::{Deserialize, Serialize};

use crate::ids::AncientOneId;
use crate::image_size::ImageSize;
use crate::monster::MonsterAbility;
use crate::named_text::NamedText;
use crate::position::Position;

/// The combat rating printed on an Ancient One sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatRating {
    /// A fixed modifier (« -6 »).
    Fixed(i8),
    /// « -X »: minus the Terror level when the battle starts (Hastur).
    MinusTerror,
    /// « -∞ »: no final battle, the investigators lose (Azathoth).
    Infinite,
}

/// One of the 8 Ancient One sheets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AncientOne {
    /// Identifier used by the other data files.
    pub id: AncientOneId,
    /// Name shown to the player.
    pub name: String,
    /// Modifier to the investigators' Fight checks in the final battle.
    pub combat_rating: CombatRating,
    /// Defensive keywords (resistances and immunities only).
    #[serde(default)]
    pub defenses: Vec<MonsterAbility>,
    /// The « Défenses » box as printed, when it is not plain keywords.
    #[serde(default)]
    pub defenses_note: Option<String>,
    /// The « Adorateurs » text: how monsters change during the whole game.
    pub worshippers: String,
    /// The Ancient One's power.
    pub power: NamedText,
    /// The « Début de la Bataille » text, if any.
    #[serde(default)]
    pub start_of_battle: Option<String>,
    /// The « Attaque » text.
    pub attack: String,
    /// Number of spaces on the doom track.
    pub doom_track: u8,
    /// Sheet image, relative to the `assets/` folder.
    pub image: String,
    /// Size of the sheet image.
    pub image_size: ImageSize,
    /// Centre of each doom track space on the sheet image, from the first one.
    pub doom_spaces: Vec<Position>,
}

/// Content of `data/ancient_ones.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AncientOnes {
    /// Format version of the file.
    pub version: u32,
    /// Doom token image, relative to the `assets/` folder.
    pub doom_marker: String,
    /// The sheets.
    pub ancient_ones: Vec<AncientOne>,
}

impl AncientOnes {
    /// Format version this code reads.
    pub const VERSION: u32 = 2;
}

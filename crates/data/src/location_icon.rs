//! The encounter icons printed on the locations.

use serde::{Deserialize, Serialize};

use crate::position::Position;

/// What an encounter icon announces: the rewards most often found there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncounterIcon {
    /// Allies.
    Ally,
    /// Sanity.
    Sanity,
    /// Blessing.
    Blessing,
    /// Skills.
    Skill,
    /// Clue tokens.
    Clue,
    /// Spells.
    Spell,
    /// Common Items.
    CommonItem,
    /// Money.
    Money,
    /// Stamina.
    Stamina,
    /// Unique Items.
    UniqueItem,
}

impl EncounterIcon {
    /// French name of what the icon stands for.
    pub fn label(self) -> &'static str {
        match self {
            Self::Ally => "Allié",
            Self::Sanity => "Santé Mentale",
            Self::Blessing => "Bénédiction",
            Self::Skill => "Compétence",
            Self::Clue => "Indice",
            Self::Spell => "Sort",
            Self::CommonItem => "Objet Commun",
            Self::Money => "Argent",
            Self::Stamina => "Résistance",
            Self::UniqueItem => "Objet Unique",
        }
    }
}

/// One icon printed on a location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocationIcon {
    /// What it announces.
    pub kind: EncounterIcon,
    /// Printed in reversed colours: a sure way to get it here.
    #[serde(default)]
    pub sure: bool,
    /// Centre of the icon on the board image.
    pub position: Position,
}

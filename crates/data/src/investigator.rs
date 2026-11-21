//! Investigator sheets.

use serde::{Deserialize, Serialize};

use crate::deck::Deck;
use crate::ids::{InvestigatorId, LocationId};
use crate::image_size::ImageSize;
use crate::named_text::NamedText;
use crate::sheet_markers::SheetMarkers;

/// A named card from the « Possessions fixes » section of a sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixedCard {
    /// Deck the card is taken from.
    pub deck: Deck,
    /// Card name as printed on the sheet, in French. Linked to the card data
    /// once the cards are transcribed.
    pub name: String,
}

/// The « Possessions fixes » section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixedPossessions {
    /// Starting money, in dollars.
    pub money: u8,
    /// Starting clue tokens.
    #[serde(default)]
    pub clues: u8,
    /// Named cards.
    #[serde(default)]
    pub cards: Vec<FixedCard>,
}

/// The « Possessions aléatoires » section: how many cards to draw per deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RandomPossessions {
    /// Objets Communs.
    #[serde(default)]
    pub common_items: u8,
    /// Objets Uniques.
    #[serde(default)]
    pub unique_items: u8,
    /// Sorts.
    #[serde(default)]
    pub spells: u8,
    /// Compétences.
    #[serde(default)]
    pub skills: u8,
}

/// The skill values printed on the sheet, one per slider stop (4 stops).
///
/// Each slider moves two linked skills: Speed/Sneak, Fight/Will, Lore/Luck.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillTracks {
    /// Vitesse.
    pub speed: Vec<u8>,
    /// Discrétion.
    pub sneak: Vec<u8>,
    /// Vigueur (« Combat » on older sheets).
    pub fight: Vec<u8>,
    /// Volonté.
    pub will: Vec<u8>,
    /// Savoir.
    pub lore: Vec<u8>,
    /// Chance.
    pub luck: Vec<u8>,
}

impl SkillTracks {
    /// Number of stops on each slider.
    pub const STOPS: usize = 4;

    /// Every track with its French name, for checks and display.
    pub fn named(&self) -> [(&'static str, &[u8]); 6] {
        [
            ("Vitesse", &self.speed),
            ("Discrétion", &self.sneak),
            ("Vigueur", &self.fight),
            ("Volonté", &self.will),
            ("Savoir", &self.lore),
            ("Chance", &self.luck),
        ]
    }
}

/// Where one slider's stops are on the sheet image, in pixels: the middle of
/// its two lines, and the centre of each stop from left to right.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SliderSpots {
    /// Height of the middle of the two skill lines.
    pub y: u16,
    /// Centre of each stop.
    pub x: Vec<u16>,
}

/// One of the 16 investigator sheets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Investigator {
    /// Identifier used by the other data files.
    pub id: InvestigatorId,
    /// Name shown to the player.
    pub name: String,
    /// Occupation printed under the name, in French (« L'Étudiante »).
    pub title: String,
    /// Maximum Sanity.
    pub sanity: u8,
    /// Maximum Stamina.
    pub stamina: u8,
    /// Starting location.
    pub home: LocationId,
    /// Total slider moves allowed per Upkeep.
    pub focus: u8,
    /// Fixed starting possessions.
    pub fixed: FixedPossessions,
    /// Random starting possessions.
    pub random: RandomPossessions,
    /// Skill values per slider stop.
    pub skills: SkillTracks,
    /// The investigator's special ability.
    pub ability: NamedText,
    /// Sheet image, relative to the `assets/` folder.
    pub image: String,
    /// Size of the sheet image.
    pub image_size: ImageSize,
    /// Where the three sliders' stops are on the image, in the order
    /// Speed/Sneak, Fight/Will, Lore/Luck.
    pub sliders: Vec<SliderSpots>,
}

/// Content of `data/investigators.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Investigators {
    /// Format version of the file.
    pub version: u32,
    /// Images of the tokens placed on or beside the sheets.
    pub markers: SheetMarkers,
    /// The sheets.
    pub investigators: Vec<Investigator>,
}

impl Investigators {
    /// Format version this code reads.
    pub const VERSION: u32 = 3;
}

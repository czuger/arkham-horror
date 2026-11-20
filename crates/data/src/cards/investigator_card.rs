//! Investigator cards: items, spells, skills, allies and special cards.

use serde::{Deserialize, Serialize};

use crate::cards::ability::Ability;
use crate::cards::ongoing::Ongoing;
use crate::deck::Deck;
use crate::ids::{CardId, SpecialId};

/// Arme Physique or Arme Magique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponKind {
    /// Arme Physique.
    Physical,
    /// Arme Magique.
    Magical,
}

/// The weapon part of an item: a bonus to Combat checks while held in hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Weapon {
    /// Physical or magical.
    pub kind: WeaponKind,
    /// Bonus to Combat checks.
    pub bonus: i8,
    /// « Défaussez après usage ».
    #[serde(default)]
    pub discard_after_use: bool,
    /// Bonus against Undead monsters, instead of `bonus` (Croix).
    #[serde(default)]
    pub undead_bonus: Option<i8>,
    /// Bonus when the other hand is empty, instead of `bonus` (Hache).
    #[serde(default)]
    pub alone_bonus: Option<i8>,
    /// Each 6 rolled counts as 2 successes (Fusil à Pompe).
    #[serde(default)]
    pub sixes_double: bool,
    /// Sanity lost each time it is used (Poudre d'Ibn-Ghazi).
    #[serde(default)]
    pub sanity_loss: u8,
}

/// The casting line of a spell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Casting {
    /// Modificateur de Lancer.
    pub modifier: i8,
    /// Coût en Santé Mentale.
    pub sanity_cost: u8,
    /// The modifier is the monster's awareness instead (Brumes de Releh).
    #[serde(default)]
    pub awareness_modifier: bool,
}

/// One investigator card, with its number of copies in the box.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvestigatorCard {
    /// Identifier (`"revolver_38"`).
    pub id: CardId,
    /// The deck it belongs to.
    pub deck: Deck,
    /// Name shown to the player, in French.
    pub name: String,
    /// Number of copies in the box.
    pub copies: u8,
    /// Printed price, for items.
    #[serde(default)]
    pub price: Option<u8>,
    /// Hands needed to use it in combat.
    #[serde(default)]
    pub hands: u8,
    /// Italic line under the picture (« Arme Physique », « Tome »…).
    #[serde(default)]
    pub subtitle: Option<String>,
    /// Weapon bonus, for weapons.
    #[serde(default)]
    pub weapon: Option<Weapon>,
    /// Casting line, for spells.
    #[serde(default)]
    pub casting: Option<Casting>,
    /// Printed text, in French.
    pub text: String,
    /// Lasting effects while held.
    #[serde(default)]
    pub ongoing: Vec<Ongoing>,
    /// Abilities.
    #[serde(default)]
    pub abilities: Vec<Ability>,
    /// Rule coded by name in the engine, for what the vocabulary cannot say.
    #[serde(default)]
    pub special: Option<SpecialId>,
    /// Card image, relative to the `assets/` folder.
    pub image: String,
}

/// Content of `data/investigator_cards.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvestigatorCards {
    /// Format version of the file.
    pub version: u32,
    /// The cards.
    pub cards: Vec<InvestigatorCard>,
}

impl InvestigatorCards {
    /// Format version this code reads.
    pub const VERSION: u32 = 1;

    /// Looks a card up by id.
    pub fn card(&self, id: &CardId) -> Option<&InvestigatorCard> {
        self.cards.iter().find(|card| &card.id == id)
    }
}

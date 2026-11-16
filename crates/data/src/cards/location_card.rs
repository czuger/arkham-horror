//! Location cards: one deck per district, one encounter per location.

use serde::{Deserialize, Serialize};

use crate::cards::condition::Condition;
use crate::cards::effect::Effect;
use crate::ids::{CardId, LocationId, StreetId};

/// One encounter printed on a location card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocationEncounter {
    /// The location this encounter belongs to.
    pub location: LocationId,
    /// When set, this entry replaces the normal one for investigators meeting
    /// the condition (Cœur de la Loge for Lodge members).
    #[serde(default)]
    pub requires: Option<Condition>,
    /// Printed text, in French.
    pub text: String,
    /// What happens.
    #[serde(default)]
    pub effects: Vec<Effect>,
}

/// A location card of a district deck.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocationCard {
    /// Identifier (the card code, `"clacv02"`).
    pub id: CardId,
    /// The district, named by its street area.
    pub district: StreetId,
    /// One encounter per location of the district.
    pub encounters: Vec<LocationEncounter>,
    /// Card image, relative to the `assets/` folder.
    pub image: String,
}

/// Content of `data/location_cards.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocationCards {
    /// Format version of the file.
    pub version: u32,
    /// The cards.
    pub cards: Vec<LocationCard>,
}

impl LocationCards {
    /// Format version this code reads.
    pub const VERSION: u32 = 1;
}

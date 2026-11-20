//! Conditions on an investigator, used by cards.

use serde::{Deserialize, Serialize};

use crate::cards::card_filter::CardFilter;
use crate::cards::resource::Resource;
use crate::deck::Deck;
use crate::ids::{CardId, LocationId, StreetId};

/// A yes/no question about one investigator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Condition {
    /// Holds this card (Membre de la Loge, Bénédiction…).
    HasCard(CardId),
    /// Holds at least one card of this family.
    HasAny(CardFilter),
    /// Holds at most `count` cards of this deck.
    CardCountAtMost {
        /// The deck counted.
        deck: Deck,
        /// Largest count for which the condition holds.
        count: u8,
    },
    /// Has at least N of a resource.
    HasAtLeast(Resource, u8),
    /// Stands in a street area of Arkham.
    InStreets,
    /// Stands in this street area.
    InStreet(StreetId),
    /// Stands on this location.
    AtLocation(LocationId),
    /// Stands in Arkham (a location or a street).
    InArkham,
    /// Stands in an Other World.
    InOtherWorld,
    /// Is lost in time and space.
    LostInTimeAndSpace,
    /// Is in the jail cell.
    InJail,
    /// The card carrying the effect holds at least N tokens.
    TokensOnThisAtLeast(u8),
    /// The check about to be made is for closing a gate.
    ClosingGate,
    /// The condition does not hold.
    Not(Box<Condition>),
}

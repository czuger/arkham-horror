//! Game data for Arkham Horror (2nd edition, base game): types and loading
//! from the RON files under `data/`, with consistency checks.
#![deny(missing_docs)]

mod ancient_one;
mod board;
mod cards;
mod data_error;
mod data_file;
mod deck;
mod dimension_symbol;
mod game_data;
mod gate_marker;
mod ids;
mod investigator;
mod location;
mod monster;
mod named_text;
mod other_world;
mod position;
mod special_area;
mod street;
mod validation;

pub use ancient_one::{AncientOne, AncientOnes, CombatRating};
pub use board::Board;
pub use cards::{
    Ability, CardFilter, Casting, CheckKind, ChoiceOption, Condition, Cost, Effect,
    EnvironmentKind, GateCard, GateCards, GateEncounter, InvestigatorCard, InvestigatorCards,
    LocationCard, LocationCards, LocationEncounter, Moment, MythosCard, MythosCards, MythosKind,
    Ongoing, Resource, Rumor, RumorCondition, RumorEnd, Scope, Timing, Weapon, WeaponKind,
};
pub use data_error::DataError;
pub use data_file::{DataFile, read_data_file};
pub use deck::Deck;
pub use dimension_symbol::DimensionSymbol;
pub use game_data::GameData;
pub use gate_marker::{GateMarker, GateMarkers};
pub use ids::{
    AncientOneId, CardId, InvestigatorId, LocationId, MonsterId, OtherWorldId, SpecialId, StreetId,
};
pub use investigator::{
    FixedCard, FixedPossessions, Investigator, Investigators, RandomPossessions, SkillTracks,
};
pub use location::{Location, Stability};
pub use monster::{Check, Monster, MonsterAbility, Monsters, Movement};
pub use named_text::NamedText;
pub use other_world::{EncounterColor, OtherWorld};
pub use position::Position;
pub use special_area::{SpecialArea, SpecialAreaKind};
pub use street::Street;
pub use validation::Issue;

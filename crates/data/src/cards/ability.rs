//! Abilities printed on investigator cards.

use serde::{Deserialize, Serialize};

use crate::cards::check_kind::CheckKind;
use crate::cards::condition::Condition;
use crate::cards::effect::Effect;
use crate::cards::resource::Resource;

/// When an ability may or must be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Timing {
    /// Entretien: resolved at every Upkeep; optional when it has a cost.
    Upkeep,
    /// Mouvement: during the investigator's Movement phase.
    Movement,
    /// Rencontres à Arkham.
    ArkhamEncounter,
    /// Rencontres dans un Autre Monde.
    OtherWorldEncounter,
    /// N'importe quelle phase.
    AnyPhase,
    /// Once, when the investigator receives the card (mandatory).
    OnTake,
    /// When the investigator is about to lose this resource.
    OnLoss(Resource),
    /// Right after the investigator failed a check of this kind.
    AfterCheck(CheckKind),
    /// Right after the investigator failed any skill check.
    AfterAnyCheck,
    /// Right before the investigator makes a check of this kind.
    BeforeCheck(CheckKind),
    /// Right after an encounter card was drawn for the investigator.
    AfterEncounterDraw,
}

/// What using an ability costs, paid before its effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cost {
    /// Déchargez : turn the card face down until the next Upkeep.
    Exhaust,
    /// Défaussez cette carte.
    DiscardThis,
    /// Dépensez N points de mouvement.
    MovementPoints(u8),
    /// Payez N.
    Pay(Resource, u8),
    /// Takes the place of the whole movement (Véhicule de Patrouille).
    WholeMovement,
}

/// One ability of a card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ability {
    /// When it can be used.
    pub timing: Timing,
    /// Whether a spell must be cast (Sanity cost and Spell check) first.
    #[serde(default)]
    pub cast: bool,
    /// Usable only when this condition holds.
    #[serde(default)]
    pub only: Option<Condition>,
    /// Costs paid before the effects.
    #[serde(default)]
    pub cost: Vec<Cost>,
    /// What it does.
    #[serde(default)]
    pub effects: Vec<Effect>,
}

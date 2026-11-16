//! Consistency checks on the gate cards.

use std::collections::HashSet;

use crate::cards::GateCard;
use crate::game_data::GameData;
use crate::validation::effect_refs::check_effects;
use crate::validation::issue::Issue;

/// Worlds exist and appear once, the « Autre » entry is present and last, and
/// effects refer to existing things.
pub(crate) fn check_gate_cards(data: &GameData, issues: &mut Vec<Issue>) {
    for card in &data.gate_cards.cards {
        let context = format!("carte Portail « {} »", card.id);
        check_worlds(card, &context, data, issues);
        if !other_entry_is_single_and_last(card) {
            issues.push(Issue::Mismatch {
                context: context.clone(),
                reason: "il faut une seule rencontre « Autre », placée en dernier",
            });
        }
        for encounter in &card.encounters {
            check_effects(&context, &encounter.effects, data, issues);
        }
    }
}

/// Every named world exists and is named once.
fn check_worlds(card: &GateCard, context: &str, data: &GameData, issues: &mut Vec<Issue>) {
    let mut seen = HashSet::new();
    for world in card
        .encounters
        .iter()
        .filter_map(|entry| entry.world.as_ref())
    {
        if data.board.other_world(world).is_none() {
            issues.push(Issue::UnknownReference {
                context: context.to_owned(),
                family: "Autre Monde",
                id: world.to_string(),
            });
        }
        if !seen.insert(world) {
            issues.push(Issue::DuplicateId {
                family: "rencontre d'Autre Monde",
                id: format!("{} / {world}", card.id),
            });
        }
    }
}

/// Whether the card ends with its only « Autre » entry.
fn other_entry_is_single_and_last(card: &GateCard) -> bool {
    let others = card
        .encounters
        .iter()
        .filter(|entry| entry.world.is_none())
        .count();
    others == 1
        && card
            .encounters
            .last()
            .is_some_and(|entry| entry.world.is_none())
}

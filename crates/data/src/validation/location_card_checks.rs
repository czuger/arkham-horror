//! Consistency checks on the location cards.

use crate::cards::LocationCard;
use crate::game_data::GameData;
use crate::validation::effect_refs::{check_condition, check_effects};
use crate::validation::issue::Issue;

/// Each card covers every location of its district once, and its effects
/// refer to existing things.
pub(crate) fn check_location_cards(data: &GameData, issues: &mut Vec<Issue>) {
    for card in &data.location_cards.cards {
        let context = format!("carte Lieu « {} »", card.id);
        if data.board.street(&card.district).is_none() {
            issues.push(Issue::UnknownReference {
                context: context.clone(),
                family: "rue",
                id: card.district.to_string(),
            });
            continue;
        }
        check_encounter_locations(card, &context, data, issues);
        check_district_covered(card, data, issues);
        for encounter in &card.encounters {
            if let Some(condition) = &encounter.requires {
                check_condition(&context, condition, data, issues);
            }
            check_effects(&context, &encounter.effects, data, issues);
        }
    }
}

/// Every encounter is for an existing location of the card's district.
fn check_encounter_locations(
    card: &LocationCard,
    context: &str,
    data: &GameData,
    issues: &mut Vec<Issue>,
) {
    for encounter in &card.encounters {
        match data.board.location(&encounter.location) {
            None => issues.push(Issue::UnknownReference {
                context: context.to_owned(),
                family: "lieu",
                id: encounter.location.to_string(),
            }),
            Some(location) if location.street != card.district => {
                issues.push(Issue::LocationNotInDistrict {
                    card: card.id.to_string(),
                    location: encounter.location.to_string(),
                });
            }
            Some(_) => {}
        }
    }
}

/// Each location of the district has exactly one unconditional encounter.
fn check_district_covered(card: &LocationCard, data: &GameData, issues: &mut Vec<Issue>) {
    for location in data.board.locations_of(&card.district) {
        let found = card
            .encounters
            .iter()
            .filter(|encounter| encounter.requires.is_none() && encounter.location == location.id)
            .count();
        if found != 1 {
            issues.push(Issue::EncounterCount {
                card: card.id.to_string(),
                location: location.id.to_string(),
                found,
            });
        }
    }
}

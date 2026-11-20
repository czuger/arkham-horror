//! Consistency checks on the Mythos cards.

use crate::cards::{MythosCard, MythosKind, RumorEnd};
use crate::game_data::GameData;
use crate::ids::LocationId;
use crate::location::Stability;
use crate::validation::effect_refs::{check_effects, check_ongoing};
use crate::validation::issue::Issue;

/// Clues and gates only go to unstable locations, rumor parts match the
/// genre, and effects refer to existing things.
pub(crate) fn check_mythos_cards(data: &GameData, issues: &mut Vec<Issue>) {
    for card in &data.mythos_cards.cards {
        let context = format!("carte Mythe « {} »", card.id);
        for location in card.clue.iter().chain(card.gate.iter()) {
            check_unstable(card, location, &context, data, issues);
        }
        check_rumor(card, &context, data, issues);
        if card.lingers && card.kind != MythosKind::Headline {
            issues.push(Issue::Mismatch {
                context: context.clone(),
                reason: "seul un Gros Titre reste en jeu jusqu'à la fin du prochain tour",
            });
        }
        check_effects(&context, &card.effects, data, issues);
        check_ongoing(&context, &card.ongoing, data, issues);
    }
}

/// The location exists and is unstable.
fn check_unstable(
    card: &MythosCard,
    location: &LocationId,
    context: &str,
    data: &GameData,
    issues: &mut Vec<Issue>,
) {
    match data.board.location(location) {
        None => issues.push(Issue::UnknownReference {
            context: context.to_owned(),
            family: "lieu",
            id: location.to_string(),
        }),
        Some(found) if found.stability == Stability::Stable => {
            issues.push(Issue::StableLocation {
                card: card.id.to_string(),
                location: location.to_string(),
            });
        }
        Some(_) => {}
    }
}

/// A Rumeur has rumor parts and other genres do not; the activity street exists.
fn check_rumor(card: &MythosCard, context: &str, data: &GameData, issues: &mut Vec<Issue>) {
    let is_rumor = card.kind == MythosKind::Rumor;
    if is_rumor != card.rumor.is_some() {
        issues.push(Issue::Mismatch {
            context: context.to_owned(),
            reason: "seule une Rumeur a une partie « rumor », et toute Rumeur en a une",
        });
    }
    let Some(rumor) = &card.rumor else {
        return;
    };
    if data.board.street(&rumor.activity).is_none() {
        issues.push(Issue::UnknownReference {
            context: context.to_owned(),
            family: "rue",
            id: rumor.activity.to_string(),
        });
    }
    check_effects(context, &rumor.setup, data, issues);
    check_effects(context, &rumor.each_mythos, data, issues);
    if let Some(action) = &rumor.action {
        check_effects(context, &action.effects, data, issues);
        if let Some(location) = &action.location
            && data.board.location(location).is_none()
        {
            issues.push(Issue::UnknownReference {
                context: context.to_owned(),
                family: "lieu",
                id: location.to_string(),
            });
        }
    }
    for end in [&rumor.pass, &rumor.fail] {
        let RumorEnd { effects, .. } = end;
        check_effects(context, effects, data, issues);
    }
}

//! Consistency checks on the investigator cards.

use crate::cards::InvestigatorCard;
use crate::deck::Deck;
use crate::game_data::GameData;
use crate::validation::effect_refs::{check_abilities, check_ongoing};
use crate::validation::issue::Issue;

/// Copies are positive, card parts match the deck, and effects refer to
/// existing things.
pub(crate) fn check_investigator_cards(data: &GameData, issues: &mut Vec<Issue>) {
    for card in &data.investigator_cards.cards {
        let context = format!("carte « {} »", card.id);
        if card.copies == 0 {
            issues.push(Issue::ZeroCopies {
                context: context.clone(),
            });
        }
        if let Some(reason) = shape_problem(card) {
            issues.push(Issue::Mismatch {
                context: context.clone(),
                reason,
            });
        }
        check_ongoing(&context, &card.ongoing, data, issues);
        check_abilities(&context, &card.abilities, data, issues);
    }
}

/// Spells and only spells have a casting line, items and only items a price,
/// and abilities that cast belong to spells.
fn shape_problem(card: &InvestigatorCard) -> Option<&'static str> {
    let is_spell = card.deck == Deck::Spell;
    let is_item = matches!(card.deck, Deck::CommonItem | Deck::UniqueItem);
    if is_spell != card.casting.is_some() {
        return Some("seuls les Sorts ont un modificateur de lancer et un coût");
    }
    if is_item != card.price.is_some() {
        return Some("seuls les Objets ont un prix");
    }
    if card.weapon.is_some() && !is_item {
        return Some("seuls les Objets peuvent être des armes");
    }
    if !is_spell && card.abilities.iter().any(|ability| ability.cast) {
        return Some("seul un Sort se lance");
    }
    None
}

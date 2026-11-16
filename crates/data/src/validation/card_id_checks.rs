//! Card ids are unique across every card file.

use crate::game_data::GameData;
use crate::validation::board_checks::report_duplicates;
use crate::validation::issue::Issue;

/// Reports any id used by two cards, whatever their files.
pub(crate) fn check_card_ids(data: &GameData, issues: &mut Vec<Issue>) {
    let ids = data
        .location_cards
        .cards
        .iter()
        .map(|card| card.id.as_str())
        .chain(data.gate_cards.cards.iter().map(|card| card.id.as_str()))
        .chain(data.mythos_cards.cards.iter().map(|card| card.id.as_str()))
        .chain(
            data.investigator_cards
                .cards
                .iter()
                .map(|card| card.id.as_str()),
        );
    report_duplicates("carte", ids, issues);
}

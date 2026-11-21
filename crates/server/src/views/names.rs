//! Looking up names and images in the game data.

use data::{AncientOne, CardId, GameData, Investigator, InvestigatorId, MonsterId};

use crate::views::spot::ImageToken;

/// An investigator sheet.
pub fn sheet<'d>(data: &'d GameData, id: &InvestigatorId) -> Option<&'d Investigator> {
    data.investigators
        .investigators
        .iter()
        .find(|sheet| &sheet.id == id)
}

/// French name of an investigator (the id when unknown).
pub fn investigator_name(data: &GameData, id: &InvestigatorId) -> String {
    sheet(data, id).map_or_else(|| id.to_string(), |sheet| sheet.name.clone())
}

/// « AS » for « Amanda Sharpe ».
pub fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .filter(|letter| letter.is_uppercase())
        .take(2)
        .collect()
}

/// A monster's name and token image.
pub fn monster(data: &GameData, id: &MonsterId) -> Option<ImageToken> {
    data.monsters
        .monsters
        .iter()
        .find(|monster| &monster.id == id)
        .map(|monster| ImageToken {
            name: monster.name.clone(),
            image: monster.image.clone(),
        })
}

/// French name of a monster (the id when unknown).
pub fn monster_name(data: &GameData, id: &MonsterId) -> String {
    monster(data, id).map_or_else(|| id.to_string(), |token| token.name)
}

/// The Ancient One sheet.
pub fn ancient_one<'d>(data: &'d GameData, id: &data::AncientOneId) -> Option<&'d AncientOne> {
    data.ancient_ones
        .ancient_ones
        .iter()
        .find(|sheet| &sheet.id == id)
}

/// Name and image of any card: investigator, Mythos, location or gate.
pub fn card(data: &GameData, id: &CardId) -> ImageToken {
    if let Some(card) = data.investigator_cards.card(id) {
        return ImageToken {
            name: card.name.clone(),
            image: card.image.clone(),
        };
    }
    if let Some(card) = data.mythos_cards.cards.iter().find(|card| &card.id == id) {
        return ImageToken {
            name: card.name.clone(),
            image: card.image.clone(),
        };
    }
    ImageToken {
        name: id.to_string(),
        image: String::new(),
    }
}

//! Looking up names and images in the game data.

use data::{AncientOne, CardId, GameData, Investigator, InvestigatorId, MonsterId};
use engine::View;

use crate::views::image_token::ImageToken;
use crate::views::tip::Tip;
use crate::views::tips;

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

/// A monster's name, token image and current values.
pub fn monster(view: &View<'_, '_>, id: &MonsterId) -> Option<ImageToken> {
    view.data()
        .monsters
        .monsters
        .iter()
        .find(|monster| &monster.id == id)
        .map(|monster| ImageToken {
            name: monster.name.clone(),
            image: monster.image.clone(),
            tip: tips::monster(view, monster),
        })
}

/// A monster on the board: name, token image, and a bubble with its
/// movement values and both sides of its token.
pub fn monster_on_board(view: &View<'_, '_>, id: &MonsterId) -> Option<ImageToken> {
    view.data()
        .monsters
        .monsters
        .iter()
        .find(|monster| &monster.id == id)
        .map(|monster| ImageToken {
            name: monster.name.clone(),
            image: monster.image.clone(),
            tip: tips::monster_movement_side(view, monster),
        })
}

/// French name of a monster (the id when unknown).
pub fn monster_name(data: &GameData, id: &MonsterId) -> String {
    data.monsters
        .monsters
        .iter()
        .find(|monster| &monster.id == id)
        .map_or_else(|| id.to_string(), |monster| monster.name.clone())
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
            tip: tips::investigator_card(card),
        };
    }
    if let Some(card) = data.mythos_cards.cards.iter().find(|card| &card.id == id) {
        return ImageToken {
            name: card.name.clone(),
            image: card.image.clone(),
            tip: tips::mythos_card(card),
        };
    }
    if let Some(card) = data.location_cards.cards.iter().find(|card| &card.id == id) {
        return encounter_card(
            "Carte Lieu",
            &card.image,
            card.encounters.iter().map(|e| &e.text),
        );
    }
    if let Some(card) = data.gate_cards.cards.iter().find(|card| &card.id == id) {
        let texts = card
            .text
            .iter()
            .chain(card.encounters.iter().map(|e| &e.text));
        return encounter_card("Carte Portail", &card.image, texts);
    }
    ImageToken {
        name: id.to_string(),
        image: String::new(),
        tip: Tip::new(id.to_string()),
    }
}

/// A location or gate card: its kind as name, its encounters in the bubble.
fn encounter_card<'t>(
    name: &str,
    image: &str,
    texts: impl Iterator<Item = &'t String>,
) -> ImageToken {
    ImageToken {
        name: name.to_owned(),
        image: image.to_owned(),
        tip: texts.fold(Tip::new(name.to_owned()), |tip, text| {
            tip.line(text.clone())
        }),
    }
}

//! Which cards and encounter paragraphs a game went through, so that bulk
//! simulations can list the ones never played.

use std::collections::BTreeSet;

use data::{CardId, GameData, LocationId, OtherWorldId};

/// Key of a location card paragraph.
pub(crate) fn location_key(card: &CardId, location: &LocationId) -> String {
    format!("Lieu {card} @ {location}")
}

/// Key of a gate card paragraph; `None` is the « Autre » paragraph.
pub(crate) fn gate_key(card: &CardId, world: Option<&OtherWorldId>) -> String {
    format!(
        "Portail {card} @ {}",
        world.map_or("autre", OtherWorldId::as_str)
    )
}

/// Key of a Mythos card.
pub(crate) fn mythos_key(card: &CardId) -> String {
    format!("Mythe {card}")
}

/// Key of an investigator card received.
pub(crate) fn received_key(card: &CardId) -> String {
    format!("Carte reçue {card}")
}

/// Special cards that are played (and must be seen played once) although
/// they carry no ability.
const PLAYED_SPECIALS: [&str; 2] = ["flute_of_the_outer_gods", "bind_monster"];

/// Key of an investigator card used (ability or weapon).
pub(crate) fn used_key(card: &CardId) -> String {
    format!("Carte utilisée {card}")
}

/// Every key a game could reach with these data.
#[must_use]
pub fn coverage_keys(data: &GameData) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for card in &data.location_cards.cards {
        for encounter in &card.encounters {
            keys.insert(location_key(&card.id, &encounter.location));
        }
    }
    for card in &data.gate_cards.cards {
        for encounter in &card.encounters {
            keys.insert(gate_key(&card.id, encounter.world.as_ref()));
        }
    }
    for card in &data.mythos_cards.cards {
        keys.insert(mythos_key(&card.id));
    }
    for card in &data.investigator_cards.cards {
        keys.insert(received_key(&card.id));
        let played_special = card
            .special
            .as_ref()
            .is_some_and(|id| PLAYED_SPECIALS.contains(&id.as_str()));
        if !card.abilities.is_empty() || card.weapon.is_some() || played_special {
            keys.insert(used_key(&card.id));
        }
    }
    keys
}

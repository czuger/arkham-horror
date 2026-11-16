//! The card checks catch broken card data. Each test breaks one thing in the
//! real data and expects the matching issue.

use std::path::PathBuf;

use data::{
    CardId, DataError, Deck, Effect, GameData, Issue, LocationId, MythosKind, OtherWorldId,
};

fn load() -> GameData {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    match GameData::load(&dir) {
        Ok(data) => data,
        Err(error) => panic!("{error}"),
    }
}

fn issues(data: &GameData) -> Vec<Issue> {
    match data.check() {
        Ok(()) => Vec::new(),
        Err(DataError::Invalid(issues)) => issues,
        Err(other) => panic!("unexpected error: {other}"),
    }
}

#[test]
fn encounter_outside_its_district_is_reported() {
    let mut data = load();
    data.location_cards.cards[0].encounters[0].location = LocationId::new("train_station");
    let found = issues(&data);
    assert!(
        found
            .iter()
            .any(|issue| matches!(issue, Issue::LocationNotInDistrict { .. })),
        "{found:?}"
    );
    assert!(
        found
            .iter()
            .any(|issue| matches!(issue, Issue::EncounterCount { found: 0, .. })),
        "{found:?}"
    );
}

#[test]
fn mythos_gate_on_a_stable_location_is_reported() {
    let mut data = load();
    data.mythos_cards.cards[0].gate = Some(LocationId::new("train_station"));
    let found = issues(&data);
    assert!(
        found
            .iter()
            .any(|issue| matches!(issue, Issue::StableLocation { .. })),
        "{found:?}"
    );
}

#[test]
fn unknown_card_in_a_nested_effect_is_reported() {
    let mut data = load();
    data.gate_cards.cards[0].encounters[0].effects = vec![Effect::Optional(data::ChoiceOption {
        label: "test".to_owned(),
        effects: vec![Effect::TakeCard {
            card: CardId::new("no_such_card"),
            otherwise: Vec::new(),
        }],
    })];
    let found = issues(&data);
    assert!(
        found.iter().any(|issue| matches!(
            issue,
            Issue::UnknownReference { id, .. } if id == "no_such_card"
        )),
        "{found:?}"
    );
}

#[test]
fn other_world_entry_must_be_last() {
    let mut data = load();
    let encounters = &mut data.gate_cards.cards[0].encounters;
    encounters.reverse();
    let found = issues(&data);
    assert!(
        found
            .iter()
            .any(|issue| matches!(issue, Issue::Mismatch { .. })),
        "{found:?}"
    );
}

#[test]
fn unknown_other_world_is_reported() {
    let mut data = load();
    data.gate_cards.cards[0].encounters[0].world = Some(OtherWorldId::new("atlantis"));
    let found = issues(&data);
    assert!(
        found.iter().any(|issue| matches!(
            issue,
            Issue::UnknownReference { id, .. } if id == "atlantis"
        )),
        "{found:?}"
    );
}

#[test]
fn spell_without_casting_line_is_reported() {
    let mut data = load();
    let Some(spell) = data
        .investigator_cards
        .cards
        .iter_mut()
        .find(|card| card.deck == Deck::Spell)
    else {
        panic!("no spell in the sample");
    };
    spell.casting = None;
    assert!(!issues(&data).is_empty());
}

#[test]
fn rumor_parts_must_match_the_genre() {
    let mut data = load();
    let Some(headline) = data
        .mythos_cards
        .cards
        .iter_mut()
        .find(|card| card.kind == MythosKind::Headline)
    else {
        panic!("no headline in the sample");
    };
    headline.kind = MythosKind::Rumor;
    assert!(!issues(&data).is_empty());
}

#[test]
fn duplicate_card_ids_across_files_are_reported() {
    let mut data = load();
    data.gate_cards.cards[0].id = CardId::new("revolver_38");
    let found = issues(&data);
    assert!(
        found
            .iter()
            .any(|issue| matches!(issue, Issue::DuplicateId { .. })),
        "{found:?}"
    );
}

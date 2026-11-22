//! Loads the real files under `data/` and checks them against the box contents.

use std::collections::{BTreeSet, HashSet};
use std::path::PathBuf;

use data::{Deck, GameData, MonsterAbility, MythosKind, Stability, StreetId};

fn repo_dir(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(name)
}

fn load() -> GameData {
    match GameData::load(&repo_dir("data")) {
        Ok(data) => data,
        Err(error) => panic!("{error}"),
    }
}

#[test]
fn data_files_load_and_are_consistent() {
    load();
}

#[test]
fn every_cited_image_exists() {
    if let Err(error) = load().check_assets(&repo_dir("assets")) {
        panic!("{error}");
    }
}

#[test]
fn every_illustration_exists_and_none_is_orphan() {
    let data = load();
    let dir = repo_dir("illustrations");
    if let Err(error) = data.check_illustrations(&dir) {
        panic!("{error}");
    }
    let cited: BTreeSet<String> = data.illustrations().map(str::to_owned).collect();
    assert_eq!(
        cited.len(),
        26 + 9 + 2 * 8 + 29,
        "one illustration per entry"
    );
    let mut on_disk = BTreeSet::new();
    for folder in ["lieux", "quartiers", "autres mondes", "monstres"] {
        let entries = std::fs::read_dir(dir.join(folder)).unwrap_or_else(|e| panic!("{e}"));
        for entry in entries {
            let name = entry.unwrap_or_else(|e| panic!("{e}")).file_name();
            let name = name.to_string_lossy();
            if !name.starts_with('.') {
                on_disk.insert(format!("{folder}/{name}"));
            }
        }
    }
    let orphans: Vec<&String> = on_disk.difference(&cited).collect();
    assert!(orphans.is_empty(), "illustrations never used: {orphans:?}");
}

#[test]
fn board_matches_the_box() {
    let board = load().board;
    assert_eq!(board.streets.len(), 9);
    assert_eq!(board.locations.len(), 26);
    assert_eq!(board.other_worlds.len(), 8);
    let unstable = board
        .locations
        .iter()
        .filter(|location| location.stability == Stability::Unstable)
        .count();
    assert_eq!(unstable, 11, "the rulebook counts 11 unstable locations");
}

#[test]
fn terror_closes_three_shops() {
    let board = load().board;
    let mut closing: Vec<(u8, &str)> = board
        .locations
        .iter()
        .filter_map(|location| Some((location.closes_at_terror?, location.id.as_str())))
        .collect();
    closing.sort();
    assert_eq!(
        closing,
        [
            (3, "general_store"),
            (6, "curiositie_shoppe"),
            (9, "ye_olde_magick_shoppe")
        ]
    );
}

#[test]
fn white_arrows_tour_the_town_and_black_arrows_go_back() {
    let board = load().board;
    let start = StreetId::new("northside");
    let mut current = start.clone();
    let mut visited = Vec::new();
    loop {
        let Some(street) = board.street(&current) else {
            panic!("unknown street {current}");
        };
        visited.push(current.clone());
        let next = street.white_arrow.clone();
        let Some(next_street) = board.street(&next) else {
            panic!("unknown street {next}");
        };
        assert_eq!(next_street.black_arrow, current, "black arrow of {next}");
        current = next;
        if current == start {
            break;
        }
        assert!(visited.len() < 9, "white arrows loop early");
    }
    assert_eq!(visited.len(), 9);
}

#[test]
fn gate_stack_has_sixteen_markers() {
    assert_eq!(load().gate_markers.total_copies(), 16);
}

#[test]
fn monster_cup_matches_the_box() {
    let monsters = load().monsters;
    assert_eq!(monsters.monsters.len(), 29);
    assert_eq!(monsters.total_copies(), 60);
    let masks: Vec<&str> = monsters
        .monsters
        .iter()
        .filter(|monster| monster.abilities.contains(&MonsterAbility::Mask))
        .map(|monster| monster.id.as_str())
        .collect();
    assert_eq!(masks.len(), 5, "{masks:?}");
}

#[test]
fn every_gate_symbol_has_monsters() {
    let data = load();
    for marker in &data.gate_markers.markers {
        assert!(
            data.monsters
                .monsters
                .iter()
                .any(|monster| monster.symbol == marker.symbol),
            "no monster for {:?}",
            marker.symbol
        );
    }
}

#[test]
fn sixteen_investigators_with_distinct_homes() {
    let investigators = load().investigators.investigators;
    assert_eq!(investigators.len(), 16);
    let homes: HashSet<_> = investigators.iter().map(|sheet| &sheet.home).collect();
    assert_eq!(homes.len(), 16);
}

#[test]
fn eight_ancient_ones() {
    let ancient_ones = load().ancient_ones.ancient_ones;
    assert_eq!(ancient_ones.len(), 8);
    let tracks: Vec<(&str, u8)> = ancient_ones
        .iter()
        .map(|sheet| (sheet.id.as_str(), sheet.doom_track))
        .collect();
    assert!(tracks.contains(&("azathoth", 14)));
    assert!(tracks.contains(&("yig", 10)));
}

#[test]
fn card_sample_covers_every_kind_of_card() {
    let data = load();
    let districts: HashSet<_> = data
        .location_cards
        .cards
        .iter()
        .map(|card| &card.district)
        .collect();
    assert_eq!(districts.len(), 9, "one location card per district");
    assert!(!data.gate_cards.cards.is_empty());
    let kinds: Vec<MythosKind> = data
        .mythos_cards
        .cards
        .iter()
        .map(|card| card.kind)
        .collect();
    assert!(kinds.contains(&MythosKind::Headline));
    assert!(kinds.contains(&MythosKind::Rumor));
    assert!(
        kinds
            .iter()
            .any(|kind| matches!(kind, MythosKind::Environment(_)))
    );
    for deck in [
        Deck::CommonItem,
        Deck::UniqueItem,
        Deck::Spell,
        Deck::Skill,
        Deck::Ally,
        Deck::Special,
    ] {
        assert!(
            data.investigator_cards
                .cards
                .iter()
                .any(|card| card.deck == deck),
            "no {deck:?} card"
        );
    }
}

//! Monster limit, Outskirts, surges and monster movement (§9, §16).

use data::{DimensionSymbol, LocationId, MonsterId, StreetId};

use super::{add_monster, clean_state, put_at, run};
use crate::state::MonsterPlace;

fn street(id: &str) -> MonsterPlace {
    MonsterPlace::Street(StreetId::new(id))
}

#[test]
fn monster_limit_is_players_plus_three() {
    let mut state = clean_state(2, "azathoth");
    for _ in 0..5 {
        add_monster(&mut state, "zombie", street("northside"));
    }
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.place_monster(MonsterId::new("ghoul"), street("downtown"))
    });
    assert_eq!(result, Ok(()));
    let outskirts = state
        .monsters
        .iter()
        .filter(|m| m.place == MonsterPlace::Outskirts)
        .count();
    assert_eq!(outskirts, 1);
}

#[test]
fn full_outskirts_empty_into_the_cup_and_raise_terror() {
    // 3 players: limit 6, Outskirts 5 (FAQ example).
    let mut state = clean_state(3, "azathoth");
    for _ in 0..6 {
        add_monster(&mut state, "zombie", street("northside"));
    }
    for _ in 0..5 {
        add_monster(&mut state, "ghoul", MonsterPlace::Outskirts);
    }
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.place_monster(MonsterId::new("cultist"), street("downtown"))
    });
    assert_eq!(result, Ok(()));
    assert_eq!(state.terror, 1);
    assert!(
        state
            .monsters
            .iter()
            .all(|m| m.place != MonsterPlace::Outskirts)
    );
}

#[test]
fn new_gate_brings_one_monster_and_two_from_five_players() {
    for (players, expected) in [(4, 1), (5, 2)] {
        let mut state = clean_state(players, "azathoth");
        let location = LocationId::new("woods");
        let (result, _) = run(&mut state, &[], &[], |game| {
            let monsters = if game.state.players() >= 5 { 2 } else { 1 };
            game.open_gate(&location, monsters)
        });
        assert_eq!(result, Ok(()));
        assert_eq!(state.monsters.len(), expected, "{players} joueurs");
        assert_eq!(state.doom, 1);
    }
}

#[test]
fn surge_draws_as_many_monsters_as_gates_or_players() {
    let mut state = clean_state(4, "azathoth");
    for location in ["woods", "graveyard"] {
        let (result, _) = run(&mut state, &[], &[], |game| {
            game.open_gate(&LocationId::new(location), 1)
        });
        assert_eq!(result, Ok(()));
    }
    state.monsters.clear();
    // 2 gates, 4 players: 4 monsters, 2 at each gate.
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.monster_surge(&LocationId::new("woods"))
    });
    assert_eq!(result, Ok(()));
    let at = |place: &str| {
        state
            .monsters
            .iter()
            .filter(|m| m.place == MonsterPlace::Location(LocationId::new(place)))
            .count()
    };
    assert_eq!((at("woods"), at("graveyard")), (2, 2));
}

use super::data;

#[test]
fn monsters_follow_their_arrow_and_stay_with_investigators() {
    let mut state = clean_state(1, "azathoth");
    put_at(&mut state, "bank_of_arkham");
    add_monster(&mut state, "zombie", street("northside"));
    add_monster(
        &mut state,
        "zombie",
        MonsterPlace::Location(LocationId::new("bank_of_arkham")),
    );
    let symbol = data()
        .monsters
        .monsters
        .iter()
        .find(|m| m.id.as_str() == "zombie")
        .map(|m| m.symbol);
    let Some(symbol) = symbol else {
        panic!("no zombie");
    };
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.move_monsters(&[symbol], &[])
    });
    assert_eq!(result, Ok(()));
    // Northside's white arrow leads Downtown; the zombie with Amanda stays.
    assert_eq!(state.monsters[0].place, street("downtown"));
    assert_eq!(
        state.monsters[1].place,
        MonsterPlace::Location(LocationId::new("bank_of_arkham"))
    );
}

#[test]
fn flying_monster_without_prey_goes_to_the_sky() {
    let mut state = clean_state(1, "azathoth");
    put_at(&mut state, "train_station");
    add_monster(
        &mut state,
        "byakhee",
        MonsterPlace::Location(LocationId::new("woods")),
    );
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.move_monsters(&[DimensionSymbol::Circle], &[])
    });
    assert_eq!(result, Ok(()));
    assert_eq!(state.monsters[0].place, MonsterPlace::Sky);
    // From the Sky, it swoops on an investigator in the streets.
    state.investigators[0].place = crate::state::Place::Street(StreetId::new("southside"));
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.move_monsters(&[DimensionSymbol::Circle], &[])
    });
    assert_eq!(result, Ok(()));
    assert_eq!(state.monsters[0].place, street("southside"));
}

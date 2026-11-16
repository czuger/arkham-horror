//! Insane, unconscious, lost, arrested, devoured (§14).

use data::{LocationId, OtherWorldId, Resource};

use super::{clean_state, hold, put_at, run};
use crate::halt::Halt;
use crate::state::{Area, Place};

#[test]
fn insane_in_arkham_loses_half_and_wakes_at_the_asylum() {
    let mut state = clean_state(1, "azathoth");
    put_at(&mut state, "woods");
    for card in ["revolver_38", "lantern", "whiskey", "retainer"] {
        hold(&mut state, card);
    }
    state.investigators[0].clues = 5;
    // Whiskey declined, then three items: one discarded (the first offered).
    let (result, _) = run(&mut state, &[0, 0], &[], |game| {
        game.lose(0, Resource::Sanity, 5)
    });
    assert_eq!(result, Err(Halt::KnockedOut(0)));
    let inv = &state.investigators[0];
    assert_eq!(inv.place, Place::Location(LocationId::new("arkham_asylum")));
    assert_eq!((inv.sanity, inv.clues, inv.cards.len()), (1, 3, 2));
    assert!(inv.encounters_over);
}

#[test]
fn single_item_is_kept() {
    let mut state = clean_state(1, "azathoth");
    hold(&mut state, "lantern");
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.lose(0, Resource::Stamina, 5)
    });
    assert_eq!(result, Err(Halt::KnockedOut(0)));
    let inv = &state.investigators[0];
    assert_eq!(
        inv.place,
        Place::Location(LocationId::new("st_marys_hospital"))
    );
    assert_eq!((inv.stamina, inv.cards.len()), (1, 1));
}

#[test]
fn knocked_out_in_an_other_world_is_lost_in_time_and_space() {
    let mut state = clean_state(1, "azathoth");
    state.investigators[0].place = Place::OtherWorld {
        world: OtherWorldId::new("yuggoth"),
        area: Area::First,
    };
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.lose(0, Resource::Sanity, 5)
    });
    assert_eq!(result, Err(Halt::KnockedOut(0)));
    let inv = &state.investigators[0];
    assert_eq!(inv.place, Place::LostInTimeAndSpace);
    assert_eq!((inv.sanity, inv.stamina, inv.delayed), (1, 1, true));
}

#[test]
fn both_at_zero_together_devours() {
    let mut state = clean_state(1, "azathoth");
    state.investigators[0].sanity = 1;
    state.investigators[0].stamina = 1;
    state.investigators[0]
        .monster_trophies
        .push(data::MonsterId::new("zombie"));
    // The new investigator places their three sliders.
    let (result, _) = run(&mut state, &[0, 0, 0], &[], |game| {
        game.lose_both(0, Resource::Sanity, 1, Some((Resource::Stamina, 1)))
    });
    assert_eq!(result, Err(Halt::KnockedOut(0)));
    let inv = &state.investigators[0];
    assert!(inv.skipping_turn, "the new investigator starts next turn");
    assert_eq!(inv.monster_trophies.len(), 1, "trophies are kept");
    assert!(inv.sanity > 0);
}

#[test]
fn arrest_takes_half_the_money_and_delays() {
    let mut state = clean_state(1, "azathoth");
    state.investigators[0].money = 7;
    let (_, _) = run(&mut state, &[], &[], |game| {
        game.arrest(0);
        Ok(())
    });
    let inv = &state.investigators[0];
    assert_eq!((inv.money, inv.delayed), (4, true));
    assert_eq!(inv.place, Place::JailCell);
    let (result, _) = run(&mut state, &[], &[], |game| game.movement_phase());
    assert_eq!(result, Ok(()));
    assert_eq!(
        state.investigators[0].place,
        Place::Location(LocationId::new("police_station"))
    );
}

#[test]
fn leaving_the_location_drops_the_explored_marker() {
    let mut state = clean_state(1, "azathoth");
    put_at(&mut state, "woods");
    state.investigators[0].explored = true;
    let (_, _) = run(&mut state, &[], &[], |game| {
        game.move_to(0, Place::Location(LocationId::new("bank_of_arkham")));
        Ok(())
    });
    assert!(!state.investigators[0].explored);
}

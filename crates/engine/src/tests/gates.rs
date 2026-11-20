//! Closing and sealing gates, victories, awakening by gates (§10, §15, §17).

use data::LocationId;

use super::{clean_state, hold, put_at, run};
use crate::state::{Ending, OpenGate};

fn open(state: &mut crate::state::GameState, location: &str) {
    let marker = state.gate_stack.remove(0);
    state.gates.push(OpenGate {
        location: LocationId::new(location),
        marker,
    });
}

#[test]
fn closing_the_last_gate_wins_with_enough_trophies() {
    let mut state = clean_state(1, "azathoth");
    open(&mut state, "woods");
    put_at(&mut state, "woods");
    state.investigators[0].explored = true;
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.close_gate(
            0,
            &LocationId::new("woods"),
            crate::rules::gates::Closing::Closed,
        )
    });
    assert_eq!(result, Err(crate::halt::Halt::Over));
    assert_eq!(state.ending, Some(Ending::GatesClosed));
}

#[test]
fn closing_the_last_gate_without_enough_trophies_goes_on() {
    let mut state = clean_state(2, "azathoth");
    open(&mut state, "woods");
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.close_gate(
            0,
            &LocationId::new("woods"),
            crate::rules::gates::Closing::Closed,
        )
    });
    assert_eq!(result, Ok(()));
    assert_eq!(state.investigators[0].gate_trophies.len(), 1);
}

#[test]
fn sealing_with_clues_costs_five_after_a_successful_close() {
    let mut state = clean_state(2, "azathoth");
    open(&mut state, "woods");
    open(&mut state, "graveyard");
    put_at(&mut state, "woods");
    state.investigators[0].explored = true;
    state.investigators[0].clues = 6;
    // Close with Lore (option 2), one success, then seal (yes); the other
    // investigator declines the shop where they stand.
    let lore = 1;
    let (result, _) = run(&mut state, &[lore, 0, 1], &[6, 6, 6], |game| {
        game.arkham_encounter_phase()
    });
    assert_eq!(result, Ok(()));
    assert_eq!(state.seals, vec![LocationId::new("woods")]);
    assert_eq!(state.investigators[0].clues, 1);
}

#[test]
fn elder_sign_seals_and_removes_a_doom_token() {
    let mut state = clean_state(2, "azathoth");
    open(&mut state, "woods");
    open(&mut state, "graveyard");
    state.doom = 3;
    put_at(&mut state, "woods");
    state.investigators[0].explored = true;
    hold(&mut state, "elder_sign");
    let (result, _) = run(&mut state, &[3, 1], &[], |game| {
        game.arkham_encounter_phase()
    });
    assert_eq!(result, Ok(()));
    assert_eq!(state.doom, 2);
    assert_eq!(state.seals.len(), 1);
    let inv = &state.investigators[0];
    assert_eq!((inv.sanity, inv.stamina), (4, 4));
    assert!(inv.cards.is_empty());
}

#[test]
fn six_elder_signs_win() {
    let mut state = clean_state(2, "azathoth");
    state.seals = ["a", "b", "c", "d", "e"]
        .iter()
        .map(|id| LocationId::new(*id))
        .collect();
    open(&mut state, "woods");
    open(&mut state, "graveyard");
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.close_gate(
            0,
            &LocationId::new("woods"),
            crate::rules::gates::Closing::SealedWithClues,
        )
    });
    assert_eq!(result, Err(crate::halt::Halt::Over));
    assert_eq!(state.ending, Some(Ending::GatesSealed));
}

#[test]
fn eight_open_gates_wake_the_ancient_one_with_two_players() {
    let mut state = clean_state(2, "azathoth");
    for location in [
        "woods",
        "graveyard",
        "black_cave",
        "witch_house",
        "unvisited_isle",
        "the_unnamable",
        "hibbs_roadhouse",
    ] {
        open(&mut state, location);
    }
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.open_gate(&LocationId::new("science_building"), 1)
    });
    assert_eq!(result, Err(crate::halt::Halt::Awakened));
}

#[test]
fn unexplored_investigator_is_drawn_through_the_gate() {
    let mut state = clean_state(1, "azathoth");
    open(&mut state, "woods");
    put_at(&mut state, "woods");
    let (result, _) = run(&mut state, &[], &[], |game| game.arkham_encounter_phase());
    assert_eq!(result, Ok(()));
    assert!(matches!(
        state.investigators[0].place,
        crate::state::Place::OtherWorld { .. }
    ));
    assert!(!state.investigators[0].delayed);
}

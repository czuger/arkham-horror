//! Turn structure: upkeep, delays, lost turns (§5, §6, FAQ).

use data::{CardId, LocationId};

use super::{clean_state, hold, put_at, run};

#[test]
fn retainer_pays_and_rolls_except_right_after_it_is_taken() {
    let mut state = clean_state(1, "azathoth");
    let (result, _) = run(&mut state, &[], &[], |game| {
        let card = game.take_from_deck(&CardId::new("retainer"))?;
        if let Some(card) = card {
            game.give_card(0, card)?;
        }
        Ok(())
    });
    assert_eq!(result, Ok(()));
    // First upkeep: no roll, no money.
    let (result, _) = run(&mut state, &[0], &[], |game| game.upkeep_phase());
    assert_eq!(result, Ok(()));
    assert_eq!(state.investigators[0].money, 0);
    // Second upkeep: $2, then a 1 discards it.
    let (result, _) = run(&mut state, &[0], &[1], |game| game.upkeep_phase());
    assert_eq!(result, Ok(()));
    assert_eq!(state.investigators[0].money, 2);
    assert!(state.investigators[0].cards.is_empty());
}

#[test]
fn blessing_cancels_a_curse() {
    let mut state = clean_state(1, "azathoth");
    hold(&mut state, "curse");
    let (result, _) = run(&mut state, &[], &[], |game| {
        if let Some(card) = game.take_from_deck(&CardId::new("blessing"))? {
            game.give_card(0, card)?;
        }
        Ok(())
    });
    assert_eq!(result, Ok(()));
    assert!(state.investigators[0].cards.is_empty());
}

#[test]
fn delayed_investigator_only_stands_up() {
    let mut state = clean_state(1, "azathoth");
    put_at(&mut state, "woods");
    state.investigators[0].delayed = true;
    let (result, _) = run(&mut state, &[], &[], |game| game.movement_phase());
    assert_eq!(result, Ok(()));
    let inv = &state.investigators[0];
    assert!(!inv.delayed);
    assert_eq!(
        inv.place,
        crate::state::Place::Location(LocationId::new("woods"))
    );
}

#[test]
fn lost_turn_skips_upkeep_and_movement() {
    let mut state = clean_state(1, "azathoth");
    hold(&mut state, "retainer");
    state.investigators[0].lose_next_turn = true;
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.upkeep_phase()?;
        game.movement_phase()
    });
    assert_eq!(result, Ok(()));
    assert_eq!(state.investigators[0].money, 0);
    assert!(state.investigators[0].skipping_turn);
}

#[test]
fn clues_are_picked_up_at_the_end_of_the_movement() {
    let mut state = clean_state(1, "azathoth");
    let street = data::StreetId::new("downtown");
    state.investigators[0].place = crate::state::Place::Street(street);
    state
        .clues
        .insert(LocationId::new("independence_square"), 2);
    // Option 0 ends the movement; find the Independence Square option.
    let target = crate::decision::Choice::Place(crate::state::Place::Location(LocationId::new(
        "independence_square",
    )));
    let mut index = None;
    let (_, _) = run(&mut state.clone(), &[], &[], |game| {
        let options = game.neighbours(&game.inv(0).place.clone());
        index = options
            .iter()
            .position(|place| crate::decision::Choice::Place(place.clone()) == target);
        Ok(())
    });
    let Some(index) = index else {
        panic!("Independence Square is not next to the Downtown streets");
    };
    let (result, _) = run(&mut state, &[index + 1, 0], &[], |game| {
        game.movement_phase()
    });
    assert_eq!(result, Ok(()));
    assert_eq!(state.investigators[0].clues, 2);
}

#[test]
fn a_turn_stuck_in_a_loop_of_decisions_is_stopped() {
    let mut state = clean_state(1, "azathoth");
    let answers = vec![0; crate::rules::turn::MAX_ANSWERS_PER_TURN + 1];
    let (result, _) = run(&mut state, &answers, &[], |game| -> crate::halt::Flow<()> {
        loop {
            game.ask_yes_no(0, crate::decision::DecisionKind::Optional, "Encore ?")?;
        }
    });
    assert!(
        matches!(&result, Err(crate::halt::Halt::Broken(reason)) if reason.contains("1000 décisions")),
        "{result:?}"
    );
}

//! Abilities printed on the investigator sheets.

use data::{CheckKind, Deck, InvestigatorId, LocationId, MonsterId, Resource};

use super::{clean_state, put_at, run};
use crate::state::{GameState, MonsterPlace, Place};

/// Seat 0 plays another sheet.
fn play_as(state: &mut GameState, id: &str) {
    state.investigators[0].id = InvestigatorId::new(id);
}

#[test]
fn amanda_draws_one_more_skill_and_keeps_one() {
    let mut state = clean_state(1, "azathoth");
    let before = state.decks.get(Deck::Skill).len();
    let (result, _) = run(&mut state, &[1], &[], |game| {
        game.draw_and_give(0, Deck::Skill)
    });
    assert_eq!(result, Ok(true));
    assert_eq!(state.investigators[0].cards.len(), 1);
    assert_eq!(state.decks.get(Deck::Skill).len(), before - 1);
}

#[test]
fn harvey_loses_one_less_sanity() {
    let mut state = clean_state(1, "azathoth");
    play_as(&mut state, "harvey_walters");
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.lose(0, Resource::Sanity, 2)
    });
    assert_eq!(result, Ok(()));
    assert_eq!(state.investigators[0].sanity, 4);
}

#[test]
fn jenny_gains_a_dollar_at_upkeep() {
    let mut state = clean_state(1, "azathoth");
    play_as(&mut state, "jenny_barnes");
    let (result, _) = run(&mut state, &[], &[], |game| game.sheet_upkeep(0));
    assert_eq!(result, Ok(()));
    assert_eq!(state.investigators[0].money, 1);
}

#[test]
fn joe_rolls_one_more_die_per_clue() {
    let mut state = clean_state(1, "azathoth");
    play_as(&mut state, "joe_diamond");
    let (dice, _) = run(&mut state, &[], &[], |game| {
        Ok(game.dice_per_clue(0, CheckKind::Lore))
    });
    assert_eq!(dice, Ok(2));
}

#[test]
fn sister_mary_goes_to_south_church_instead_of_getting_lost() {
    let mut state = clean_state(1, "yog_sothoth");
    play_as(&mut state, "sister_mary");
    let (result, _) = run(&mut state, &[], &[], |game| game.lose_in_time_and_space(0));
    assert_eq!(result, Ok(()));
    let inv = &state.investigators[0];
    assert_eq!(inv.place, Place::Location(LocationId::new("south_church")));
    assert!(!inv.delayed && !inv.eliminated);
}

#[test]
fn no_monster_appears_where_kate_stands() {
    let mut state = clean_state(1, "azathoth");
    play_as(&mut state, "kate_winthrop");
    put_at(&mut state, "woods");
    let woods = LocationId::new("woods");
    let (result, _) = run(&mut state, &[], &[], |game| {
        game.place_monster(
            MonsterId::new("zombie"),
            MonsterPlace::Location(woods.clone()),
        )?;
        game.monster_surge(&woods)
    });
    assert_eq!(result, Ok(()));
    assert!(state.monsters.is_empty());
}

#[test]
fn mandy_lets_a_failed_check_reroll_its_failed_dice() {
    let mut state = clean_state(1, "azathoth");
    play_as(&mut state, "mandy_thompson");
    let (result, _) = run(&mut state, &[0], &[], |game| {
        let dice = usize::try_from(game.check_dice(0, CheckKind::Lore, 0)?).unwrap_or(0);
        game.loaded_dice = std::iter::repeat_n(1, dice)
            .chain(std::iter::repeat_n(6, dice))
            .collect();
        game.check(0, CheckKind::Lore, 0, 1)
    });
    assert!(result.is_ok_and(|check| check.passed));
    assert!(state.investigators[0].power_used);
}

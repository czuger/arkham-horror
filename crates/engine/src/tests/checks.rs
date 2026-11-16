//! Skill checks (§11) and their key numbers (§21.2).

use data::CheckKind;

use super::{clean_state, hold, run};

#[test]
fn five_and_six_are_successes() {
    let mut state = clean_state(1, "azathoth");
    // Amanda: Luck 3 at stop 1.
    let (result, _) = run(&mut state, &[], &[4, 5, 1], |game| {
        game.check(0, CheckKind::Luck, 0, 1)
    });
    assert_eq!(result.map(|r| (r.passed, r.successes)), Ok((true, 1)));
    let (result, _) = run(&mut state, &[0], &[4, 4, 1], |game| {
        game.check(0, CheckKind::Luck, 0, 1)
    });
    assert_eq!(result.map(|r| r.passed), Ok(false));
}

#[test]
fn blessed_succeeds_on_four_and_cursed_only_on_six() {
    let mut state = clean_state(1, "azathoth");
    hold(&mut state, "blessing");
    let (result, _) = run(&mut state, &[], &[4, 1, 1], |game| {
        game.check(0, CheckKind::Luck, 0, 1)
    });
    assert_eq!(result.map(|r| r.passed), Ok(true));
    let mut state = clean_state(1, "azathoth");
    hold(&mut state, "curse");
    let (result, _) = run(&mut state, &[], &[5, 5, 6], |game| {
        game.check(0, CheckKind::Luck, 0, 2)
    });
    assert_eq!(result.map(|r| r.successes), Ok(1));
}

#[test]
fn modifier_changes_the_number_of_dice() {
    let mut state = clean_state(1, "azathoth");
    // Luck 3, +2: five dice, the fifth is the only success.
    let (result, _) = run(&mut state, &[], &[1, 1, 1, 1, 6], |game| {
        game.check(0, CheckKind::Luck, 2, 1)
    });
    assert_eq!(result.map(|r| r.passed), Ok(true));
}

#[test]
fn no_dice_fails_but_clues_still_roll() {
    let mut state = clean_state(1, "azathoth");
    state.investigators[0].clues = 1;
    // Luck 3, -5: no die. The clue gives one die anyway.
    let (result, _) = run(&mut state, &[1], &[6], |game| {
        game.check(0, CheckKind::Luck, -5, 1)
    });
    assert_eq!(result.map(|r| r.passed), Ok(true));
    assert_eq!(state.investigators[0].clues, 0);
}

#[test]
fn luck_skill_adds_a_die_per_clue_and_one_to_the_skill() {
    let mut state = clean_state(1, "azathoth");
    hold(&mut state, "luck");
    state.investigators[0].clues = 1;
    // Luck 3 + 1 = 4 dice, all failures; the clue gives two dice.
    let (result, _) = run(&mut state, &[1], &[1, 1, 1, 1, 2, 5], |game| {
        game.check(0, CheckKind::Luck, 0, 1)
    });
    assert_eq!(result.map(|r| r.passed), Ok(true));
}

#[test]
fn bonus_to_sneak_applies_to_evade_but_not_the_reverse() {
    let mut state = clean_state(1, "azathoth");
    state.environment = Some(data::CardId::new("raining_cats_and_dogs"));
    super::put_at(&mut state, "bank_of_arkham");
    let (dice, _) = run(&mut state, &[], &[], |game| {
        game.check_dice(0, CheckKind::Evade, 0)
    });
    // Sneak 3, +1 for Sneak checks in Arkham.
    assert_eq!(dice, Ok(4));
}

#[test]
fn bravery_rerolls_a_failed_horror_check() {
    let mut state = clean_state(1, "azathoth");
    hold(&mut state, "bravery");
    // Will 3: three failures, Bravery used, then a success.
    let (result, _) = run(&mut state, &[1], &[1, 1, 1, 1, 1, 6], |game| {
        game.check(0, CheckKind::Horror, 0, 1)
    });
    assert_eq!(result.map(|r| r.passed), Ok(true));
    assert!(state.investigators[0].cards[0].exhausted);
}

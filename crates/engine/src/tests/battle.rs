//! The final battle (§17).

use super::{clean_state, run};
use crate::halt::Halt;
use crate::state::{BattleState, Ending, Place};

#[test]
fn azathoth_awake_ends_the_game() {
    let mut state = clean_state(1, "azathoth");
    let (result, _) = run(&mut state, &[], &[], |game| game.final_battle());
    assert_eq!(result, Err(Halt::Over));
    assert_eq!(state.ending, Some(Ending::Devoured));
}

#[test]
fn one_doom_token_per_players_successes_and_the_rest_is_kept() {
    let mut state = clean_state(2, "yig");
    state.battle = Some(BattleState::default());
    state.doom = 5;
    state.investigators[0].sliders = [1, 3, 1];
    // Fight 4 - 3 (Yig) = 1 die... give more with a bonus: none; so force
    // the doom down from accumulated successes instead.
    if let Some(battle) = &mut state.battle {
        battle.successes = 1;
    }
    let (result, _) = run(&mut state, &[], &[6], |game| {
        game.attack_ancient_one_for_tests(0)
    });
    assert_eq!(result, Ok(()));
    // 1 kept + 1 rolled = 2 = players: one doom token removed, none kept.
    assert_eq!(state.doom, 4);
    assert_eq!(state.battle.as_ref().map(|b| b.successes), Some(0));
}

#[test]
fn lost_investigators_are_devoured_when_the_battle_starts() {
    let mut state = clean_state(2, "yig");
    state.investigators[0].place = Place::LostInTimeAndSpace;
    let (result, _) = run(&mut state, &[0; 64], &[1; 64], |game| game.final_battle());
    assert!(matches!(result, Err(Halt::Over) | Err(Halt::Waiting(_))));
    assert!(state.investigators[0].eliminated);
}

#[test]
fn the_doom_track_fills_before_the_battle() {
    let mut state = clean_state(1, "cthulhu");
    state.doom = 4;
    let (_, events) = run(&mut state, &[], &[], |game| game.final_battle());
    assert!(events.iter().any(|line| line.contains("Bataille finale")));
    assert!(state.battle.is_some());
    assert!(state.doom >= 4);
}

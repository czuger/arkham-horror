//! Evading and fighting (§12, §13, §20.2).

use data::{MonsterAbility, MonsterId, WeaponKind};

use super::{clean_state, hold, run};
use crate::rules::combat::Faced;
use crate::rules::combat_gear::reduced_bonus;

#[test]
fn resistance_halves_rounding_up_and_immunity_cancels() {
    let resistant = [MonsterAbility::PhysicalResistance];
    assert_eq!(reduced_bonus(&resistant, WeaponKind::Physical, 3), 2);
    assert_eq!(reduced_bonus(&resistant, WeaponKind::Magical, 3), 3);
    assert_eq!(
        reduced_bonus(&[MonsterAbility::MagicalImmunity], WeaponKind::Magical, 6),
        0
    );
}

#[test]
fn failed_evade_hurts_then_combat_starts() {
    let mut state = clean_state(1, "azathoth");
    // Zombie: awareness +1, horror -1/1, combat -1/2, toughness 1.
    // Sneak 3+1 = 4 dice fail; Will 3-1 = 2 dice pass; attack: Fight 2-1 = 1 die passes.
    let dice = [1, 1, 1, 1, 6, 1, 6];
    let (result, _) = run(&mut state, &[0], &dice, |game| {
        game.evade_or_fight(0, &MonsterId::new("zombie"), false)
    });
    assert_eq!(result, Ok(Faced::Defeated));
    let inv = &state.investigators[0];
    assert_eq!((inv.stamina, inv.sanity), (3, 5));
    assert_eq!(inv.monster_trophies, vec![MonsterId::new("zombie")]);
}

#[test]
fn horror_check_happens_once_per_combat() {
    let mut state = clean_state(1, "azathoth");
    // Fight straight away: horror fails (2 dice), attack fails, attack passes.
    let dice = [1, 1, 1, 6];
    let (result, events) = run(&mut state, &[0, 0], &dice, |game| {
        game.evade_or_fight(0, &MonsterId::new("zombie"), true)
    });
    assert_eq!(result, Ok(Faced::Defeated));
    let horror = events
        .iter()
        .filter(|line| line.contains("test d'Horreur"))
        .count();
    assert_eq!(horror, 1);
    let inv = &state.investigators[0];
    assert_eq!((inv.sanity, inv.stamina), (4, 3));
}

#[test]
fn nightmarish_and_overwhelming_hurt_on_success() {
    let mut state = clean_state(1, "azathoth");
    state.investigators[0].sliders = [1, 3, 1];
    hold(&mut state, "dynamite");
    // Dhole: horror -1/4, Nightmarish 1, combat -3/4, toughness 3,
    // Physical Resistance, Overwhelming 1. Will at stop 3 is 1: 0 dice, horror
    // fails automatically (-4). Fight 4 + 8/2 - 3 = 5 dice, three successes.
    state.investigators[0].sanity = 5;
    let dice = [6, 6, 6, 1, 1];
    let (result, _) = run(&mut state, &[1, 0, 0], &dice, |game| {
        game.evade_or_fight(0, &MonsterId::new("dhole"), true)
    });
    assert_eq!(result, Ok(Faced::Defeated));
    let inv = &state.investigators[0];
    assert_eq!((inv.sanity, inv.stamina), (1, 4));
    assert!(inv.cards.is_empty(), "dynamite is discarded after use");
}

#[test]
fn endless_monster_goes_back_to_the_cup() {
    let mut state = clean_state(1, "nyarlathotep");
    let cup = state.monster_cup.len();
    // Cultist under Nyarlathotep: Endless. No horror check; combat +1, toughness 1.
    let (result, _) = run(&mut state, &[0], &[6, 1, 1], |game| {
        game.evade_or_fight(0, &MonsterId::new("cultist"), true)
    });
    assert_eq!(result, Ok(Faced::Defeated));
    assert!(state.investigators[0].monster_trophies.is_empty());
    assert_eq!(state.monster_cup.len(), cup + 1);
}

#[test]
fn ambush_forbids_fleeing() {
    let mut state = clean_state(1, "azathoth");
    // Ghoul: Ambush. The only option is to fight: no answer is asked.
    let (result, _) = run(&mut state, &[], &[6, 1, 1, 6], |game| {
        game.evade_or_fight(0, &MonsterId::new("ghoul"), true)
    });
    assert_eq!(result, Ok(Faced::Defeated));
}

#[test]
fn a_card_in_hand_can_be_let_go_for_another() {
    let mut state = clean_state(1, "azathoth");
    hold(&mut state, "revolver_38");
    hold(&mut state, "dynamite");
    // Take the Dynamite (2 hands), let it go, take the Revolver, attack.
    let mut gear = crate::rules::combat::Gear::default();
    let (result, _) = run(&mut state, &[2, 1, 1, 0], &[], |game| {
        game.prepare_combat(0, &mut gear)
    });
    assert_eq!(result, Ok(()));
    assert_eq!((gear.bonus(), gear.hands()), (3, 1));
}

//! The hands and possessions zones, on a real combat.

use std::path::Path;

use ai::Policy;
use data::{GameData, InvestigatorId};
use engine::{Choice, Decision, Outcome, Replay, Responder, Setup, Status, View, play};

use crate::views::HandZones;

/// Plays with the AI on every seat and stops at the first decision of
/// seat 0 that `stop` picks.
struct StopAt {
    policy: Policy,
    stop: fn(&Decision) -> bool,
}

impl Responder for StopAt {
    fn answer(&mut self, view: &View<'_, '_>, decision: &Decision) -> Option<usize> {
        if decision.who == 0 && (self.stop)(decision) {
            return None;
        }
        self.policy.answer(view, decision)
    }
}

fn arming(decision: &Decision) -> bool {
    crate::views::prepares_combat(decision)
        && decision
            .options
            .iter()
            .any(|option| matches!(option.choice, Choice::Card(_)))
}

fn armed(decision: &Decision) -> bool {
    crate::views::prepares_combat(decision)
        && decision
            .options
            .iter()
            .any(|option| matches!(option.choice, Choice::Drop(_)))
}

fn run(data: &GameData, setup: &Setup, answers: &[usize], stop: fn(&Decision) -> bool) -> Outcome {
    let mut then = StopAt {
        policy: Policy::new(),
        stop,
    };
    let mut replay = Replay::new(answers, &mut then);
    play(data, setup, &mut replay).expect("game")
}

fn waiting(outcome: &Outcome) -> Option<&Decision> {
    match &outcome.status {
        Status::Waiting(decision) => Some(decision),
        Status::Over(_) => None,
    }
}

/// Michael McGlen takes his two-handed Thompson in hand: both hands are
/// used, and the other weapons wait for a free hand.
#[test]
fn a_two_handed_weapon_fills_both_hands() {
    let data = GameData::load(Path::new("../../data")).expect("data");
    let tommy = data
        .investigator_cards
        .cards
        .iter()
        .find(|card| card.name == "Mitraillette")
        .map(|card| card.id.clone())
        .expect("Mitraillette");
    let (setup, before) = (1..200)
        .find_map(|seed| {
            let setup = Setup {
                seed,
                players: 1,
                investigators: vec![InvestigatorId::new("michael_mcglen")],
                ancient_one: None,
            };
            let outcome = run(&data, &setup, &[], arming);
            let take = waiting(&outcome)?.find(&Choice::Card(tommy.clone()))?;
            Some((setup, (outcome, take)))
        })
        .expect("a combat with the Thompson at hand");
    let (outcome, take) = before;
    let inv = &outcome.state.investigators[0];
    let zones = HandZones::new(&data, inv, waiting(&outcome));
    assert!(zones.active);
    assert_eq!((zones.used, zones.total), (0, 2));
    assert!(zones.in_hand.is_empty());
    let thompson = zones
        .possessions
        .iter()
        .find(|held| held.token.name == "Mitraillette")
        .expect("in the possessions");
    assert_eq!((thompson.hands, thompson.choice), (2, Some(take)));

    let mut answers = outcome.answers.clone();
    answers.push(take);
    let after = run(&data, &setup, &answers, armed);
    let decision = waiting(&after).expect("still preparing the combat");
    let zones = HandZones::new(&data, &after.state.investigators[0], Some(decision));
    assert_eq!(zones.used, 2);
    assert_eq!(zones.in_hand.len(), 1);
    assert_eq!(zones.in_hand[0].token.name, "Mitraillette");
    assert!(zones.in_hand[0].choice.is_some(), "it can be let go");
    for held in zones.possessions.iter().filter(|held| held.hands > 0) {
        assert_eq!(held.choice, None, "{}", held.token.name);
        assert!(held.note.starts_with("Il faut"), "{}", held.note);
    }
}

/// Outside a combat, nothing moves; the hands hold the weapons kept from
/// the last combat (R28).
#[test]
fn kept_weapons_stay_in_hand_outside_a_combat() {
    let data = GameData::load(Path::new("../../data")).expect("data");
    let setup = Setup {
        seed: 5,
        players: 1,
        investigators: vec![InvestigatorId::new("michael_mcglen")],
        ancient_one: None,
    };
    let outcome = run(&data, &setup, &[], |_| true);
    let mut inv = outcome.state.investigators[0].clone();
    let zones = HandZones::new(&data, &inv, waiting(&outcome));
    assert!(!zones.active);
    assert!(zones.in_hand.is_empty());
    assert_eq!(zones.possessions.len(), inv.cards.len());
    let tommy = inv
        .cards
        .iter()
        .map(|held| held.card.clone())
        .find(|card| {
            data.investigator_cards
                .card(card)
                .is_some_and(|c| c.hands == 2)
        })
        .expect("a two-handed weapon");
    inv.in_hand = vec![tommy];
    let zones = HandZones::new(&data, &inv, None);
    assert_eq!(zones.used, 2);
    assert_eq!(zones.in_hand.len(), 1);
    assert_eq!(zones.possessions.len(), inv.cards.len() - 1);
    assert!(
        zones
            .in_hand
            .iter()
            .chain(&zones.possessions)
            .all(|held| held.choice.is_none())
    );
}

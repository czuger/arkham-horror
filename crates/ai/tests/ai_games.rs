//! Whole games played by the AI on every seat, with the real data: they
//! must end, replay exactly, and leave the human's decisions open.

use std::path::PathBuf;

use ai::Policy;
use data::GameData;
use engine::{Decision, Outcome, Replay, Responder, Setup, Status, View, Waiting, play};

fn data() -> GameData {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    match GameData::load(&dir) {
        Ok(data) => data,
        Err(error) => panic!("{error}"),
    }
}

fn finish(data: &GameData, setup: &Setup, responder: &mut dyn Responder) -> Outcome {
    match play(data, setup, responder) {
        Ok(outcome) => outcome,
        Err(error) => panic!("seed {}: {error}", setup.seed),
    }
}

#[test]
fn ai_games_end_with_one_to_eight_players() {
    let data = data();
    for seed in 0..160 {
        let players = usize::try_from(seed % 8).unwrap_or(0) + 1;
        let outcome = finish(&data, &Setup::random(seed, players), &mut Policy::new());
        assert!(
            matches!(outcome.status, Status::Over(_)),
            "seed {seed}, {players} player(s): {:?}",
            outcome.status
        );
    }
}

#[test]
fn an_ai_game_replays_exactly() {
    let data = data();
    let setup = Setup::random(42, 4);
    let first = finish(&data, &setup, &mut Policy::new());
    let mut waiting = Waiting;
    let mut replay = Replay::new(&first.answers, &mut waiting);
    let second = finish(&data, &setup, &mut replay);
    assert_eq!(first, second);
}

/// The human on seat 0, the AI elsewhere.
struct HumanFirst(Policy);

impl Responder for HumanFirst {
    fn answer(&mut self, view: &View<'_, '_>, decision: &Decision) -> Option<usize> {
        if decision.who == 0 {
            None
        } else {
            self.0.answer(view, decision)
        }
    }
}

#[test]
fn the_ai_leaves_the_human_decisions_open() {
    let data = data();
    for seed in 0..10 {
        let outcome = finish(
            &data,
            &Setup::random(seed, 3),
            &mut HumanFirst(Policy::new()),
        );
        match outcome.status {
            Status::Waiting(decision) => assert_eq!(decision.who, 0, "seed {seed}"),
            Status::Over(_) => {}
        }
    }
}

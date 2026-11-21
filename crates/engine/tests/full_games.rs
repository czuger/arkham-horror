//! Whole games played by simple responders: they must end, and replay exactly.

use std::path::PathBuf;

use data::GameData;
use engine::{FirstOption, Outcome, Replay, Setup, Status, Waiting, play};

fn data() -> GameData {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    match GameData::load(&dir) {
        Ok(data) => data,
        Err(error) => panic!("{error}"),
    }
}

/// Answers with a fixed pseudo-random sequence.
struct Scrambled(u64);

impl engine::Responder for Scrambled {
    fn answer(
        &mut self,
        _view: &engine::View<'_, '_>,
        decision: &engine::Decision,
    ) -> Option<usize> {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let pick = usize::try_from(self.0 >> 33).unwrap_or(0);
        Some(pick % decision.options.len())
    }
}

fn finish(data: &GameData, setup: &Setup, responder: &mut dyn engine::Responder) -> Outcome {
    match play(data, setup, responder) {
        Ok(outcome) => outcome,
        Err(error) => panic!("seed {}: {error}", setup.seed),
    }
}

#[test]
fn first_option_games_end() {
    let data = data();
    for seed in 0..20 {
        let outcome = finish(&data, &Setup::random(seed, 1), &mut FirstOption);
        assert!(
            matches!(outcome.status, Status::Over(_)),
            "seed {seed}: {:?}",
            outcome.status
        );
    }
}

#[test]
fn scrambled_games_end_with_several_players() {
    let data = data();
    for seed in 0..200 {
        let players = usize::try_from(seed % 8).unwrap_or(0) + 1;
        let outcome = finish(&data, &Setup::random(seed, players), &mut Scrambled(seed));
        assert!(
            matches!(outcome.status, Status::Over(_)),
            "seed {seed}: {:?}",
            outcome.status
        );
    }
}

#[test]
fn replay_gives_the_same_game() {
    let data = data();
    let setup = Setup::random(7, 3);
    let first = finish(&data, &setup, &mut Scrambled(99));
    let mut waiting = Waiting;
    let mut replay = Replay::new(&first.answers, &mut waiting);
    let second = finish(&data, &setup, &mut replay);
    assert_eq!(first, second);
}

#[test]
fn a_game_stops_at_each_decision_and_resumes() {
    let data = data();
    let setup = Setup::random(3, 2);
    let full = finish(&data, &setup, &mut Scrambled(5));
    for cut in [0, 1, 10, full.answers.len() / 2] {
        let mut waiting = Waiting;
        let mut replay = Replay::new(&full.answers[..cut], &mut waiting);
        let partial = finish(&data, &setup, &mut replay);
        assert!(matches!(partial.status, Status::Waiting(_)), "cut {cut}");
        assert_eq!(partial.answers, full.answers[..cut]);
    }
}

#[test]
fn state_survives_json() {
    let data = data();
    let outcome = finish(&data, &Setup::random(11, 2), &mut Scrambled(1));
    let text = match serde_json::to_string(&outcome.state) {
        Ok(text) => text,
        Err(error) => panic!("{error}"),
    };
    let back: engine::GameState = match serde_json::from_str(&text) {
        Ok(state) => state,
        Err(error) => panic!("{error}"),
    };
    assert_eq!(back, outcome.state);
}

//! Many games with random answers, and a summary.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use data::GameData;
use engine::{Setup, Status, play};

use crate::random_responder::RandomResponder;

/// Plays `games` games and prints how they ended.
pub fn run(
    data: &GameData,
    setup: &Setup,
    games: u64,
    missing_file: Option<&Path>,
) -> Result<(), String> {
    let mut endings: BTreeMap<&'static str, u64> = BTreeMap::new();
    let mut turns = 0_u64;
    let mut decisions = 0_u64;
    let mut failures = Vec::new();
    let mut covered = BTreeSet::new();
    for index in 0..games {
        let game_setup = Setup {
            seed: setup.seed.wrapping_add(index),
            ..setup.clone()
        };
        let mut responder = RandomResponder::new(game_setup.seed);
        match play(data, &game_setup, &mut responder) {
            Ok(outcome) => {
                let label = match outcome.status {
                    Status::Over(ending) => ending.label(),
                    Status::Waiting(_) => "en attente (anormal)",
                };
                *endings.entry(label).or_insert(0) += 1;
                turns += u64::from(outcome.state.turn);
                decisions += u64::try_from(outcome.answers.len()).unwrap_or(0);
                covered.extend(outcome.covered);
            }
            Err(error) => failures.push(format!("graine {} : {error}", game_setup.seed)),
        }
    }
    let played = games
        .saturating_sub(u64::try_from(failures.len()).unwrap_or(0))
        .max(1);
    println!(
        "{games} partie(s), {} joueur(s), graines {} et suivantes.",
        setup.players, setup.seed
    );
    for (label, count) in &endings {
        println!("  {count:>6}  {label}");
    }
    println!(
        "Durée moyenne : {:.1} tours, {:.0} décisions.",
        turns as f64 / played as f64,
        decisions as f64 / played as f64
    );
    crate::coverage_report::report(data, &covered, missing_file)?;
    for failure in &failures {
        println!("ÉCHEC {failure}");
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!("{} partie(s) en échec", failures.len()))
    }
}

//! Many games played by the AI (or at random), and a summary.

use std::collections::BTreeSet;
use std::path::Path;

use ai::Policy;
use data::GameData;
use engine::{Responder, Setup, play};

use crate::random_responder::RandomResponder;
use crate::stats::Stats;

/// Plays `games` games and prints how they ended.
pub fn run(
    data: &GameData,
    setup: &Setup,
    games: u64,
    missing_file: Option<&Path>,
    random: bool,
) -> Result<(), String> {
    let mut stats = Stats::default();
    let mut failures = Vec::new();
    let mut covered = BTreeSet::new();
    for index in 0..games {
        let game_setup = Setup {
            seed: setup.seed.wrapping_add(index),
            ..setup.clone()
        };
        let mut responder: Box<dyn Responder> = if random {
            Box::new(RandomResponder::new(game_setup.seed))
        } else {
            Box::new(Policy::new())
        };
        match play(data, &game_setup, responder.as_mut()) {
            Ok(outcome) => {
                stats.add(&outcome);
                covered.extend(outcome.covered);
            }
            Err(error) => failures.push(format!("graine {} : {error}", game_setup.seed)),
        }
    }
    let who = if random { "au hasard" } else { "par l'IA" };
    println!(
        "{games} partie(s) jouée(s) {who}, {} joueur(s), graines {} et suivantes.",
        setup.players, setup.seed
    );
    stats.print();
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

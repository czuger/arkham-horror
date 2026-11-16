//! One game in text mode: the player answers every decision by number.

use std::io::{BufRead, Write};

use data::GameData;
use engine::{Decision, Replay, Setup, Status, Waiting, play};

/// Plays a game, replaying the answers given so far at each step.
pub fn run(data: &GameData, setup: &Setup) -> Result<(), String> {
    println!(
        "Graine : {} (pour rejouer : --graine {}).",
        setup.seed, setup.seed
    );
    let mut answers: Vec<usize> = Vec::new();
    let mut shown = 0;
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        let mut waiting = Waiting;
        let mut replay = Replay::new(&answers, &mut waiting);
        let outcome = play(data, setup, &mut replay).map_err(|error| error.to_string())?;
        for event in outcome.events.iter().skip(shown) {
            println!("{event}");
        }
        shown = outcome.events.len();
        let decision = match outcome.status {
            Status::Over(_) => {
                println!("\nPartie terminée en {} tour(s).", outcome.state.turn);
                return Ok(());
            }
            Status::Waiting(decision) => decision,
        };
        show(&decision);
        let Some(answer) = read_answer(&mut lines, decision.options.len())? else {
            println!("Partie interrompue. Réponses : {answers:?}");
            return Ok(());
        };
        answers.push(answer);
    }
}

/// Prints a decision and its numbered options.
fn show(decision: &Decision) {
    println!("\n>> {}", decision.prompt);
    for (index, option) in decision.options.iter().enumerate() {
        println!("  {}. {}", index + 1, option.label);
    }
    print!("Votre choix : ");
    let _ignored = std::io::stdout().flush();
}

/// Reads a choice number; `None` at the end of the input.
fn read_answer(
    lines: &mut impl Iterator<Item = std::io::Result<String>>,
    count: usize,
) -> Result<Option<usize>, String> {
    for line in lines {
        let line = line.map_err(|error| error.to_string())?;
        match line.trim().parse::<usize>() {
            Ok(number) if (1..=count).contains(&number) => return Ok(Some(number - 1)),
            _ => print!("Un nombre entre 1 et {count}, s'il vous plaît : "),
        }
        let _ignored = std::io::stdout().flush();
    }
    Ok(None)
}

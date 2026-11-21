//! Command-line simulator: plays a game in text mode, or AI-only games in bulk.

mod args;
mod bulk;
mod coverage_report;
mod interactive;
mod random_responder;
mod stats;

use std::path::Path;
use std::process::ExitCode;

use args::{Command, parse};

/// Plays one game with the AI on every seat and prints its journal.
fn watch(data: &data::GameData, setup: &engine::Setup) -> Result<(), String> {
    let mut policy = ai::Policy::new();
    let outcome = engine::play(data, setup, &mut policy).map_err(|error| error.to_string())?;
    for event in &outcome.events {
        println!("{event}");
    }
    println!(
        "\nGraine {}, {} tour(s), {} décision(s).",
        setup.seed,
        outcome.state.turn,
        outcome.answers.len()
    );
    Ok(())
}

fn main() -> ExitCode {
    let command = match parse(std::env::args().skip(1)) {
        Ok(command) => command,
        Err(message) => {
            eprintln!("{message}\n\n{}", args::USAGE);
            return ExitCode::FAILURE;
        }
    };
    let data_dir = std::env::var("ARKHAM_DATA").unwrap_or_else(|_| "data".to_owned());
    let data = match data::GameData::load(Path::new(&data_dir)) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Données illisibles ({data_dir}) : {error}");
            return ExitCode::FAILURE;
        }
    };
    let result = match command {
        Command::Help => {
            println!("{}", args::USAGE);
            Ok(())
        }
        Command::Play(setup) => interactive::run(&data, &setup),
        Command::Watch(setup) => watch(&data, &setup),
        Command::Bulk {
            games,
            setup,
            missing_file,
            random,
        } => bulk::run(&data, &setup, games, missing_file.as_deref(), random),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

//! Command-line simulator: plays a game in text mode, or AI-only games in bulk.

mod args;
mod bulk;
mod interactive;
mod random_responder;

use std::path::Path;
use std::process::ExitCode;

use args::{Command, parse};

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
        Command::Bulk { games, setup } => bulk::run(&data, &setup, games),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

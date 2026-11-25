//! Web server for the Arkham Horror adaptation: pages, forms and embedded database.

mod ambiance;
mod app_error;
mod app_state;
mod config;
mod database;
mod embedded_files;
mod game_row;
mod illustrations;
mod music_library;
mod play_mode;
mod progress;
mod relative_path;
mod router;
mod routes;
mod same_origin;
mod schema;
mod session;
mod skill_moves;
mod startup;
#[cfg(test)]
mod tests;
mod trade_moves;
mod views;

use std::error::Error;
use std::io::IsTerminal;
use std::process::ExitCode;

use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(colors_wanted())
        .init();
    match serve().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

/// Colours only on a terminal, and never when `NO_COLOR` is set.
fn colors_wanted() -> bool {
    std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal()
}

async fn serve() -> Result<(), Box<dyn Error>> {
    let config = config::Config::from_env()?;
    startup::run(config).await?;
    Ok(())
}

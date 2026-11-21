//! Web server for the Arkham Horror adaptation: pages, forms and embedded database.

mod app_error;
mod app_state;
mod config;
mod database;
mod game_row;
mod progress;
mod router;
mod routes;
mod same_origin;
mod schema;
mod session;
mod startup;
#[cfg(test)]
mod tests;
mod views;

use std::error::Error;
use std::process::ExitCode;

use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
    match serve().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

async fn serve() -> Result<(), Box<dyn Error>> {
    let config = config::Config::from_env()?;
    startup::run(config).await?;
    Ok(())
}

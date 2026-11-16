//! Web server for the Arkham Horror adaptation: pages, forms and embedded database.

mod app_error;
mod app_state;
mod config;
mod database;
mod router;
mod routes;
mod startup;

use std::error::Error;

use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
    let config = config::Config::from_env()?;
    startup::run(config).await?;
    Ok(())
}

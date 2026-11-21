use std::sync::Arc;

use data::GameData;
use tokio::net::TcpListener;

use crate::app_state::AppState;
use crate::config::Config;
use crate::database::{Database, DatabaseError};
use crate::router::build_router;

/// The htmx library, served from the static folder when present.
pub const HTMX_FILE: &str = "htmx.min.js";

/// Failure while starting or running the server.
#[derive(Debug, thiserror::Error)]
pub enum StartupError {
    /// The database could not be opened.
    #[error(transparent)]
    Database(#[from] DatabaseError),
    /// The game data files are missing or invalid.
    #[error("données du jeu invalides : {0}")]
    Data(#[from] data::DataError),
    /// The listening socket failed.
    #[error("impossible d'écouter sur le réseau : {0}")]
    Io(#[from] std::io::Error),
}

/// Loads the game data, opens the database, binds the socket and serves until Ctrl-C.
///
/// The database is flushed and closed when the router, which holds the
/// last handle on it, is dropped at the end.
///
/// # Errors
///
/// Returns a [`StartupError`] if the data or the database cannot be opened or the
/// address cannot be bound.
pub async fn run(config: Config) -> Result<(), StartupError> {
    let data = Arc::new(GameData::load(&config.data_dir)?);
    let db = Database::open(&config.db_path)?;
    let htmx = config.static_dir.join(HTMX_FILE).is_file();
    if !htmx {
        tracing::warn!("{HTMX_FILE} absent from the static folder: pages reload in full");
    }
    let app = build_router(&config, AppState { db, data, htmx });
    let listener = TcpListener::bind(config.addr).await?;
    tracing::info!("listening on http://{}", config.addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    tracing::info!("server stopped");
    Ok(())
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(%error, "cannot listen for Ctrl-C");
    }
}

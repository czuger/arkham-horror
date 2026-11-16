use tokio::net::TcpListener;

use crate::app_state::AppState;
use crate::config::Config;
use crate::database::{Database, DatabaseError};
use crate::router::build_router;

/// Failure while starting or running the server.
#[derive(Debug, thiserror::Error)]
pub enum StartupError {
    /// The database could not be opened.
    #[error(transparent)]
    Database(#[from] DatabaseError),
    /// The listening socket failed.
    #[error("impossible d'écouter sur le réseau : {0}")]
    Io(#[from] std::io::Error),
}

/// Opens the database, binds the socket and serves until Ctrl-C.
///
/// The database is flushed and closed when the router, which holds the
/// last handle on it, is dropped at the end.
///
/// # Errors
///
/// Returns a [`StartupError`] if the database cannot be opened or the
/// address cannot be bound.
pub async fn run(config: Config) -> Result<(), StartupError> {
    let db = Database::open(&config.db_path)?;
    let app = build_router(&config, AppState { db });
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

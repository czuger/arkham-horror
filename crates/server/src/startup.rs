use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::Router;
use tokio::net::{TcpListener, UnixListener};
use tokio::signal::unix::{SignalKind, signal};

use crate::app_state::AppState;
use crate::config::{Config, Listen};
use crate::database::{Database, DatabaseError};
use crate::embedded_files::{StaticFiles, load_game_data};
use crate::music_library::MusicLibrary;
use crate::router::build_router;
use crate::session::delete_outdated_games;

/// The htmx library, served from the static files when present.
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
    /// The folder of the database could not be created.
    #[error("impossible de créer le dossier {} : {source}", .path.display())]
    Folder {
        /// The folder.
        path: PathBuf,
        /// The underlying error.
        source: io::Error,
    },
    /// The Unix socket could not be prepared.
    #[error("impossible d'écouter sur le socket {} : {source}", .path.display())]
    Socket {
        /// The socket path.
        path: PathBuf,
        /// The underlying error.
        source: io::Error,
    },
    /// The TCP listening socket failed.
    #[error("impossible d'écouter sur le réseau : {0}")]
    Io(#[from] io::Error),
}

/// Loads the game data, lists the music tracks, opens the database, deletes the games that no
/// longer fit the rules, then serves on TCP or on a Unix socket until
/// Ctrl-C or SIGTERM.
///
/// The database is closed once the server has stopped.
///
/// # Errors
///
/// Returns a [`StartupError`] if the data or the database cannot be opened or
/// the listening address or socket cannot be bound.
pub async fn run(config: Config) -> Result<(), StartupError> {
    let data = Arc::new(load_game_data()?);
    create_parent_folder(&config.db_path)?;
    let db = Database::open(&config.db_path).await?;
    let deleted = delete_outdated_games(&db, &data).await?;
    if deleted > 0 {
        tracing::warn!("{deleted} game(s) no longer fitting the rules deleted");
    }
    let htmx = StaticFiles::get(HTMX_FILE).is_some();
    if !htmx {
        tracing::warn!("{HTMX_FILE} absent from the static folder: pages reload in full");
    }
    let app = build_router(AppState {
        db: db.clone(),
        data,
        htmx,
        music: Arc::new(MusicLibrary::scan(&config.music_dir)),
    });
    let served = match &config.listen {
        Listen::Tcp(addr) => serve_tcp(*addr, app).await,
        Listen::Unix(path) => serve_unix(path, app).await,
    };
    db.close().await;
    tracing::info!("server stopped");
    served
}

/// Creates the folder that will hold the database file, if missing.
fn create_parent_folder(db_path: &Path) -> Result<(), StartupError> {
    let Some(folder) = db_path
        .parent()
        .filter(|folder| !folder.as_os_str().is_empty())
    else {
        return Ok(());
    };
    fs::create_dir_all(folder).map_err(|source| StartupError::Folder {
        path: folder.to_owned(),
        source,
    })
}

async fn serve_tcp(addr: std::net::SocketAddr, app: Router) -> Result<(), StartupError> {
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

/// Serves on a Unix socket: replaces a stale socket file, opens it to every
/// user (the proxy runs under another account) and removes it on exit.
async fn serve_unix(path: &Path, app: Router) -> Result<(), StartupError> {
    let socket_error = |source| StartupError::Socket {
        path: path.to_owned(),
        source,
    };
    remove_socket(path).map_err(socket_error)?;
    let listener = UnixListener::bind(path).map_err(socket_error)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o666)).map_err(socket_error)?;
    tracing::info!("listening on unix socket {}", path.display());
    let served = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;
    if let Err(error) = remove_socket(path) {
        tracing::error!(%error, "cannot remove the socket {}", path.display());
    }
    served.map_err(socket_error)
}

/// Removes the socket file; a missing file is fine.
fn remove_socket(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

/// Resolves on Ctrl-C or SIGTERM (sent by systemd to stop the service).
async fn shutdown_signal() {
    let mut terminate = match signal(SignalKind::terminate()) {
        Ok(terminate) => terminate,
        Err(error) => {
            tracing::error!(%error, "cannot listen for SIGTERM");
            wait_for_ctrl_c().await;
            return;
        }
    };
    tokio::select! {
        () = wait_for_ctrl_c() => {}
        _ = terminate.recv() => tracing::info!("SIGTERM received"),
    }
}

async fn wait_for_ctrl_c() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(%error, "cannot listen for Ctrl-C");
        std::future::pending::<()>().await;
    }
}

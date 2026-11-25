use std::env;
use std::net::{AddrParseError, SocketAddr};
use std::path::PathBuf;

const DEFAULT_ADDR: &str = "127.0.0.1:3000";
const DB_FILE: &str = "arkham.db";
const MUSIC_FOLDER: &str = "musiques";

/// Server settings, read from environment variables.
///
/// | Variable | Default |
/// |---|---|
/// | `ARKHAM_ADDR` | `127.0.0.1:3000` (TCP, used when `ARKHAM_SOCKET` is unset) |
/// | `ARKHAM_SOCKET` | unset: listen on TCP; set: listen on this Unix socket instead |
/// | `ARKHAM_DATA_DIR` | unset: the current folder; persistent folder for the saved games |
/// | `ARKHAM_DB` | `$ARKHAM_DATA_DIR/arkham.db`, or `arkham.db` without `ARKHAM_DATA_DIR` |
/// | `ARKHAM_MUSIC_DIR` | `$ARKHAM_DATA_DIR/musiques`, or `musiques` without `ARKHAM_DATA_DIR` |
///
/// The game data files (`data/`), the images (`assets/`) and the CSS and
/// JavaScript files (`static/`) are built into the program; they are not
/// settings. `ARKHAM_DATA_DIR` holds what the server writes, and the music
/// (too big to build in), read at startup.
#[derive(Debug, Clone)]
pub struct Config {
    /// Where the server listens.
    pub listen: Listen,
    /// SQLite database file, created on first run.
    pub db_path: PathBuf,
    /// The music folder: one subfolder of MP3 files per mood.
    pub music_dir: PathBuf,
}

/// Where the server accepts connections.
#[derive(Debug, Clone)]
pub enum Listen {
    /// A TCP address and port (local use).
    Tcp(SocketAddr),
    /// A Unix socket path (deployed behind a proxy).
    Unix(PathBuf),
}

/// Invalid configuration value.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// `ARKHAM_ADDR` is not a valid `ip:port` address.
    #[error("ARKHAM_ADDR invalide (« {value} ») : {source}")]
    InvalidAddr {
        /// The value read from the environment.
        value: String,
        /// Parsing error.
        source: AddrParseError,
    },
}

impl Config {
    /// Reads the configuration from the environment, with local defaults.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::InvalidAddr`] if `ARKHAM_ADDR` cannot be parsed.
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            listen: listen_from_env()?,
            db_path: db_path_from_env(),
            music_dir: music_dir_from_env(),
        })
    }
}

fn listen_from_env() -> Result<Listen, ConfigError> {
    if let Some(socket) = env_value("ARKHAM_SOCKET") {
        return Ok(Listen::Unix(socket.into()));
    }
    let value = env_value("ARKHAM_ADDR").unwrap_or_else(|| DEFAULT_ADDR.to_owned());
    value
        .parse()
        .map(Listen::Tcp)
        .map_err(|source| ConfigError::InvalidAddr { value, source })
}

fn db_path_from_env() -> PathBuf {
    if let Some(db) = env_value("ARKHAM_DB") {
        return db.into();
    }
    match env_value("ARKHAM_DATA_DIR") {
        Some(dir) => PathBuf::from(dir).join(DB_FILE),
        None => PathBuf::from(DB_FILE),
    }
}

fn music_dir_from_env() -> PathBuf {
    if let Some(dir) = env_value("ARKHAM_MUSIC_DIR") {
        return dir.into();
    }
    match env_value("ARKHAM_DATA_DIR") {
        Some(dir) => PathBuf::from(dir).join(MUSIC_FOLDER),
        None => PathBuf::from(MUSIC_FOLDER),
    }
}

/// The variable's value, `None` when unset or empty.
fn env_value(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.is_empty())
}

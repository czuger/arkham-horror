use std::env;
use std::net::{AddrParseError, SocketAddr};
use std::path::PathBuf;

const DEFAULT_ADDR: &str = "127.0.0.1:3000";
const DEFAULT_DB: &str = "arkham.db";
const DEFAULT_ASSETS: &str = "assets";
const DEFAULT_STATIC: &str = "static";

/// Server settings, read from environment variables.
///
/// | Variable | Default |
/// |---|---|
/// | `ARKHAM_ADDR` | `127.0.0.1:3000` |
/// | `ARKHAM_DB` | `arkham.db` |
/// | `ARKHAM_ASSETS` | `assets` |
/// | `ARKHAM_STATIC` | `static` |
#[derive(Debug, Clone)]
pub struct Config {
    /// Address and port the server listens on.
    pub addr: SocketAddr,
    /// Folder of the PoloDB database, created on first run.
    pub db_path: PathBuf,
    /// Folder of the game images (`assets/` in the repository).
    pub assets_dir: PathBuf,
    /// Folder of the CSS and JavaScript files.
    pub static_dir: PathBuf,
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
        let addr_value = env_or("ARKHAM_ADDR", DEFAULT_ADDR);
        let addr = addr_value
            .parse()
            .map_err(|source| ConfigError::InvalidAddr {
                value: addr_value.clone(),
                source,
            })?;
        Ok(Self {
            addr,
            db_path: env_or("ARKHAM_DB", DEFAULT_DB).into(),
            assets_dir: env_or("ARKHAM_ASSETS", DEFAULT_ASSETS).into(),
            static_dir: env_or("ARKHAM_STATIC", DEFAULT_STATIC).into(),
        })
    }
}

fn env_or(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_owned())
}

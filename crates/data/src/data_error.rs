//! Errors raised while loading the data files.

use std::path::PathBuf;

use crate::validation::Issue;

/// Why the game data could not be loaded. Messages are in French.
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    /// A data file could not be read.
    #[error("lecture impossible de {path} : {source}")]
    Io {
        /// The file.
        path: PathBuf,
        /// The underlying error.
        source: std::io::Error,
    },
    /// A data file is not valid RON for the expected structure.
    #[error("fichier de données mal formé {path} : {source}")]
    Parse {
        /// The file.
        path: PathBuf,
        /// The parser error, with line and column. Boxed: it is large and rare.
        source: Box<ron::error::SpannedError>,
    },
    /// A data file declares a format version this code does not read.
    #[error("{path} : version {found} non prise en charge (attendue : {expected})")]
    Version {
        /// The file.
        path: PathBuf,
        /// Version written in the file.
        found: u32,
        /// Version this code reads.
        expected: u32,
    },
    /// The files parse but contradict each other.
    #[error("données incohérentes :\n{}", format_issues(.0))]
    Invalid(Vec<Issue>),
}

/// Renders one issue per line, as a bulleted list.
fn format_issues(issues: &[Issue]) -> String {
    issues
        .iter()
        .map(|issue| format!("- {issue}"))
        .collect::<Vec<_>>()
        .join("\n")
}

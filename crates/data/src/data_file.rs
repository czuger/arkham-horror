//! Common shape of the versioned RON files under `data/`.

use std::path::Path;

use serde::de::DeserializeOwned;

use crate::ancient_one::AncientOnes;
use crate::board::Board;
use crate::cards::{GateCards, InvestigatorCards, LocationCards, MythosCards};
use crate::data_error::DataError;
use crate::gate_marker::GateMarkers;
use crate::investigator::Investigators;
use crate::monster::Monsters;

/// A top-level data file: its name in `data/` and its format version.
pub trait DataFile: DeserializeOwned {
    /// File name inside the data folder.
    const FILE_NAME: &'static str;
    /// Format version this code reads.
    const VERSION: u32;

    /// Version written in the loaded file.
    fn version(&self) -> u32;
}

impl DataFile for Board {
    const FILE_NAME: &'static str = "board.ron";
    const VERSION: u32 = Board::VERSION;

    fn version(&self) -> u32 {
        self.version
    }
}

impl DataFile for GateMarkers {
    const FILE_NAME: &'static str = "gate_markers.ron";
    const VERSION: u32 = GateMarkers::VERSION;

    fn version(&self) -> u32 {
        self.version
    }
}

impl DataFile for Monsters {
    const FILE_NAME: &'static str = "monsters.ron";
    const VERSION: u32 = Monsters::VERSION;

    fn version(&self) -> u32 {
        self.version
    }
}

impl DataFile for Investigators {
    const FILE_NAME: &'static str = "investigators.ron";
    const VERSION: u32 = Investigators::VERSION;

    fn version(&self) -> u32 {
        self.version
    }
}

impl DataFile for AncientOnes {
    const FILE_NAME: &'static str = "ancient_ones.ron";
    const VERSION: u32 = AncientOnes::VERSION;

    fn version(&self) -> u32 {
        self.version
    }
}

impl DataFile for LocationCards {
    const FILE_NAME: &'static str = "location_cards.ron";
    const VERSION: u32 = LocationCards::VERSION;

    fn version(&self) -> u32 {
        self.version
    }
}

impl DataFile for GateCards {
    const FILE_NAME: &'static str = "gate_cards.ron";
    const VERSION: u32 = GateCards::VERSION;

    fn version(&self) -> u32 {
        self.version
    }
}

impl DataFile for MythosCards {
    const FILE_NAME: &'static str = "mythos_cards.ron";
    const VERSION: u32 = MythosCards::VERSION;

    fn version(&self) -> u32 {
        self.version
    }
}

impl DataFile for InvestigatorCards {
    const FILE_NAME: &'static str = "investigator_cards.ron";
    const VERSION: u32 = InvestigatorCards::VERSION;

    fn version(&self) -> u32 {
        self.version
    }
}

/// Reads, parses and version-checks one data file from `data_dir`.
///
/// # Errors
///
/// [`DataError::Io`] if the file cannot be read, [`DataError::Parse`] if it is
/// not valid RON for `T`, [`DataError::Version`] if its version is not `T::VERSION`.
pub fn read_data_file<T: DataFile>(data_dir: &Path) -> Result<T, DataError> {
    let path = data_dir.join(T::FILE_NAME);
    let text = std::fs::read_to_string(&path).map_err(|source| DataError::Io {
        path: path.clone(),
        source,
    })?;
    let file: T = ron::from_str(&text).map_err(|source| DataError::Parse {
        path: path.clone(),
        source: Box::new(source),
    })?;
    if file.version() != T::VERSION {
        return Err(DataError::Version {
            path,
            found: file.version(),
            expected: T::VERSION,
        });
    }
    Ok(file)
}

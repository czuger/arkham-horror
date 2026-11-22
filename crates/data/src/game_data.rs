//! All the game data, loaded and checked together.

use std::path::Path;

use crate::ancient_one::AncientOnes;
use crate::board::Board;
use crate::cards::{GateCards, InvestigatorCards, LocationCards, MythosCards};
use crate::data_error::DataError;
use crate::data_file::{DataFile, parse_data_file, read_text};
use crate::gate_marker::GateMarkers;
use crate::investigator::Investigators;
use crate::monster::Monsters;
use crate::validation::{
    Issue, check_ancient_ones, check_board, check_card_ids, check_gate_cards, check_gate_markers,
    check_images, check_investigator_cards, check_investigators, check_location_cards,
    check_monsters, check_mythos_cards,
};

/// Every data file of the base game, consistent with each other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameData {
    /// The board graph.
    pub board: Board,
    /// The gate markers.
    pub gate_markers: GateMarkers,
    /// The monster tokens.
    pub monsters: Monsters,
    /// The investigator sheets.
    pub investigators: Investigators,
    /// The Ancient One sheets.
    pub ancient_ones: AncientOnes,
    /// The location cards.
    pub location_cards: LocationCards,
    /// The gate cards.
    pub gate_cards: GateCards,
    /// The Mythos cards.
    pub mythos_cards: MythosCards,
    /// The investigator cards (items, spells, skills, allies, special cards).
    pub investigator_cards: InvestigatorCards,
}

impl GameData {
    /// Loads every file from `data_dir` and runs the consistency checks.
    ///
    /// # Arguments
    ///
    /// * `data_dir` - the `data/` folder.
    ///
    /// # Errors
    ///
    /// Any [`DataError`]: unreadable or malformed file, unsupported version,
    /// or [`DataError::Invalid`] with every inconsistency found.
    pub fn load(data_dir: &Path) -> Result<Self, DataError> {
        Self::load_with(|file_name| read_text(&data_dir.join(file_name)))
    }

    /// Loads every file through `read` and runs the consistency checks.
    ///
    /// # Arguments
    ///
    /// * `read` - gives the text of a data file from its name (`board.ron`…),
    ///   for data that does not sit in a folder (files built into a program).
    ///
    /// # Errors
    ///
    /// Any error from `read`, a malformed file, an unsupported version, or
    /// [`DataError::Invalid`] with every inconsistency found.
    pub fn load_with(
        mut read: impl FnMut(&str) -> Result<String, DataError>,
    ) -> Result<Self, DataError> {
        let data = Self {
            board: load_file(&mut read)?,
            gate_markers: load_file(&mut read)?,
            monsters: load_file(&mut read)?,
            investigators: load_file(&mut read)?,
            ancient_ones: load_file(&mut read)?,
            location_cards: load_file(&mut read)?,
            gate_cards: load_file(&mut read)?,
            mythos_cards: load_file(&mut read)?,
            investigator_cards: load_file(&mut read)?,
        };
        data.check()?;
        Ok(data)
    }

    /// Runs the consistency checks that need no file access.
    ///
    /// # Errors
    ///
    /// [`DataError::Invalid`] listing every inconsistency found.
    pub fn check(&self) -> Result<(), DataError> {
        into_result(self.issues())
    }

    /// Checks that every image cited by the data exists under `assets_dir`.
    ///
    /// # Errors
    ///
    /// [`DataError::Invalid`] listing every missing image.
    pub fn check_assets(&self, assets_dir: &Path) -> Result<(), DataError> {
        let mut issues = Vec::new();
        check_images(self, assets_dir, &mut issues);
        into_result(issues)
    }

    /// Collects every inconsistency.
    fn issues(&self) -> Vec<Issue> {
        let mut issues = Vec::new();
        check_board(&self.board, &mut issues);
        check_gate_markers(&self.gate_markers, &self.board, &mut issues);
        check_monsters(&self.monsters, &mut issues);
        check_investigators(&self.investigators, &self.board, &mut issues);
        check_ancient_ones(&self.ancient_ones, &mut issues);
        check_card_ids(self, &mut issues);
        check_location_cards(self, &mut issues);
        check_gate_cards(self, &mut issues);
        check_mythos_cards(self, &mut issues);
        check_investigator_cards(self, &mut issues);
        issues
    }
}

/// Reads and parses the data file `T` through `read`.
fn load_file<T: DataFile>(
    read: &mut impl FnMut(&str) -> Result<String, DataError>,
) -> Result<T, DataError> {
    let text = read(T::FILE_NAME)?;
    parse_data_file(Path::new(T::FILE_NAME), &text)
}

/// Turns a list of issues into `Ok` when empty, [`DataError::Invalid`] otherwise.
fn into_result(issues: Vec<Issue>) -> Result<(), DataError> {
    if issues.is_empty() {
        Ok(())
    } else {
        Err(DataError::Invalid(issues))
    }
}

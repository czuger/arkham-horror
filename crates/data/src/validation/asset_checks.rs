//! Checks that every image cited by the data exists.

use std::path::Path;

use crate::game_data::GameData;
use crate::validation::issue::Issue;

/// Reports every cited image missing from `assets_dir`.
pub(crate) fn check_images(data: &GameData, assets_dir: &Path, issues: &mut Vec<Issue>) {
    for image in cited_images(data) {
        let path = assets_dir.join(image);
        if !path.is_file() {
            issues.push(Issue::MissingImage { path });
        }
    }
}

/// Reports every cited illustration missing from `illustrations_dir`.
pub(crate) fn check_illustration_files(
    data: &GameData,
    illustrations_dir: &Path,
    issues: &mut Vec<Issue>,
) {
    for illustration in cited_illustrations(data) {
        let path = illustrations_dir.join(illustration);
        if !path.is_file() {
            issues.push(Issue::MissingIllustration { path });
        }
    }
}

/// Lists the illustration paths cited by the data files, relative to
/// `illustrations/`: locations, districts, both areas of each Other World,
/// monster portraits.
pub(crate) fn cited_illustrations(data: &GameData) -> impl Iterator<Item = &str> {
    let board = &data.board;
    board
        .locations
        .iter()
        .map(|location| location.illustration.as_str())
        .chain(
            board
                .streets
                .iter()
                .map(|street| street.illustration.as_str()),
        )
        .chain(board.other_worlds.iter().flat_map(|world| {
            [
                world.illustrations.first.as_str(),
                world.illustrations.second.as_str(),
            ]
        }))
        .chain(
            data.monsters
                .monsters
                .iter()
                .map(|monster| monster.portrait.as_str()),
        )
}

/// Lists the image paths cited by the data files, relative to `assets/`.
fn cited_images(data: &GameData) -> impl Iterator<Item = &str> {
    let gates = &data.gate_markers;
    std::iter::once(data.board.image.as_str())
        .chain(std::iter::once(data.board.terror_track.marker.as_str()))
        .chain(std::iter::once(data.board.closed_marker.as_str()))
        .chain(data.board.icon_images.values().map(String::as_str))
        .chain(std::iter::once(data.ancient_ones.doom_marker.as_str()))
        .chain(data.investigators.markers.images())
        .chain(std::iter::once(gates.back_image.as_str()))
        .chain(gates.markers.iter().map(|marker| marker.image.as_str()))
        .chain(
            data.monsters
                .monsters
                .iter()
                .map(|monster| monster.image.as_str()),
        )
        .chain(
            data.investigators
                .investigators
                .iter()
                .map(|sheet| sheet.image.as_str()),
        )
        .chain(
            data.ancient_ones
                .ancient_ones
                .iter()
                .map(|sheet| sheet.image.as_str()),
        )
        .chain(card_images(data))
}

/// Lists the card images, relative to `assets/`.
fn card_images(data: &GameData) -> impl Iterator<Item = &str> {
    data.location_cards
        .cards
        .iter()
        .map(|card| card.image.as_str())
        .chain(data.gate_cards.cards.iter().map(|card| card.image.as_str()))
        .chain(
            data.mythos_cards
                .cards
                .iter()
                .map(|card| card.image.as_str()),
        )
        .chain(
            data.investigator_cards
                .cards
                .iter()
                .map(|card| card.image.as_str()),
        )
}

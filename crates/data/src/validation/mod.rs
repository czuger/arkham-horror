//! Consistency checks run when the data is loaded.

mod ancient_one_checks;
mod asset_checks;
mod board_checks;
mod card_id_checks;
mod effect_refs;
mod gate_card_checks;
mod gate_checks;
mod investigator_card_checks;
mod investigator_checks;
mod issue;
mod location_card_checks;
mod monster_checks;
mod mythos_card_checks;

pub(crate) use ancient_one_checks::check_ancient_ones;
pub(crate) use asset_checks::{check_illustration_files, check_images, cited_illustrations};
pub(crate) use board_checks::check_board;
pub(crate) use card_id_checks::check_card_ids;
pub(crate) use gate_card_checks::check_gate_cards;
pub(crate) use gate_checks::check_gate_markers;
pub(crate) use investigator_card_checks::check_investigator_cards;
pub(crate) use investigator_checks::check_investigators;
pub use issue::Issue;
pub(crate) use location_card_checks::check_location_cards;
pub(crate) use monster_checks::check_monsters;
pub(crate) use mythos_card_checks::check_mythos_cards;

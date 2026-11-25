//! The scene illustration of a special area, drawn among several.

use data::SpecialAreaKind;
use engine::View;

use crate::views::scene_pick::{mix, pick_index};

/// One of the special area's illustrations, relative to `illustrations/`;
/// `None` when it has none.
///
/// The pick looks random but stays the same for the whole turn, so the
/// picture does not change each time the page is shown.
pub fn special_illustration(view: &View<'_, '_>, kind: SpecialAreaKind) -> Option<String> {
    let choices = &view.data().board.special_area(kind)?.illustrations;
    let state = view.state();
    let index = pick_index(mix(state.seed ^ u64::from(state.turn)), choices.len())?;
    choices.get(index).cloned()
}

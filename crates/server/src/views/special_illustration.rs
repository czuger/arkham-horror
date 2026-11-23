//! The scene illustration of a special area, drawn among several.

use data::SpecialAreaKind;
use engine::View;

/// One of the special area's illustrations, relative to `illustrations/`;
/// `None` when it has none.
///
/// The pick looks random but stays the same for the whole turn, so the
/// picture does not change each time the page is shown.
pub fn special_illustration(view: &View<'_, '_>, kind: SpecialAreaKind) -> Option<String> {
    let choices = &view.data().board.special_area(kind)?.illustrations;
    let state = view.state();
    let mixed = mix(state.seed ^ u64::from(state.turn));
    let len = u64::try_from(choices.len()).ok()?;
    let index = usize::try_from(mixed.checked_rem(len)?).ok()?;
    choices.get(index).cloned()
}

/// Spreads the bits of `value` (the SplitMix64 finaliser), so that nearby
/// turns give unrelated picks.
fn mix(value: u64) -> u64 {
    let mut z = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_halves_come_up_over_a_few_turns() {
        let picks: Vec<u64> = (0..8).map(|turn| mix(42 ^ turn) % 2).collect();
        assert!(picks.contains(&0) && picks.contains(&1), "{picks:?}");
    }
}

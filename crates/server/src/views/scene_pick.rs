//! Picks that look random but stay the same each time a page is shown, so
//! that a picture does not change when the page is reloaded.

/// Spreads the bits of `value` (the SplitMix64 finaliser), so that nearby
/// keys give unrelated picks.
pub fn mix(value: u64) -> u64 {
    let mut z = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// An index below `len` drawn from `key`; `None` when `len` is 0.
pub fn pick_index(key: u64, len: usize) -> Option<usize> {
    let len = u64::try_from(len).ok()?;
    usize::try_from(key.checked_rem(len)?).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_halves_come_up_over_a_few_turns() {
        let picks: Vec<u64> = (0..8).map(|turn| mix(42 ^ turn) % 2).collect();
        assert!(picks.contains(&0) && picks.contains(&1), "{picks:?}");
    }

    #[test]
    fn nothing_is_picked_among_nothing() {
        assert_eq!(pick_index(7, 0), None);
        assert_eq!(pick_index(7, 3), Some(1));
    }
}

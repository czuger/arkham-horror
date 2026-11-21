//! Chances of rolling enough successes.

/// Chance that one die is a success when successes start at `face`.
pub fn die_chance(face: u8) -> f64 {
    f64::from(7_u8.saturating_sub(face.clamp(1, 6))) / 6.0
}

/// Chance of at least `needed` successes on `dice` dice.
pub fn chance(dice: u32, needed: u32, face: u8) -> f64 {
    if needed == 0 {
        return 1.0;
    }
    if dice < needed {
        return 0.0;
    }
    let p = die_chance(face);
    (needed..=dice).map(|hits| exactly(dice, hits, p)).sum()
}

/// Chance of exactly `hits` successes on `dice` dice.
pub fn exactly(dice: u32, hits: u32, p: f64) -> f64 {
    if hits > dice {
        return 0.0;
    }
    let mut ways = 1.0;
    for step in 0..hits {
        ways = ways * f64::from(dice - step) / f64::from(step + 1);
    }
    ways * p.powf(f64::from(hits)) * (1.0 - p).powf(f64::from(dice - hits))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_die_succeeds_one_time_in_three() {
        assert!((chance(1, 1, 5) - 1.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn more_dice_help() {
        assert!(chance(4, 1, 5) > chance(2, 1, 5));
        assert!((chance(0, 1, 5)).abs() < 1e-9);
        assert!((chance(3, 0, 5) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn blessed_dice_succeed_half_the_time() {
        assert!((chance(1, 1, 4) - 0.5).abs() < 1e-9);
    }
}

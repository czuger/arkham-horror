//! The engine's only source of randomness: a small seeded generator whose
//! state is part of the game state, so a game replays exactly.

use serde::{Deserialize, Serialize};

/// xoshiro256** generator, seeded with SplitMix64.
///
/// Written here rather than taken from a crate so that its output can never
/// change with a dependency upgrade: saved games replay from the seed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rng {
    state: [u64; 4],
}

impl Rng {
    /// Creates a generator from a seed.
    pub fn new(seed: u64) -> Self {
        let mut mix = seed;
        let mut next = || {
            mix = mix.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = mix;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        Self {
            state: [next(), next(), next(), next()],
        }
    }

    /// Next raw 64-bit value.
    fn next_u64(&mut self) -> u64 {
        let [s0, s1, s2, s3] = &mut self.state;
        let result = s1.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = *s1 << 17;
        *s2 ^= *s0;
        *s3 ^= *s1;
        *s1 ^= *s2;
        *s0 ^= *s3;
        *s2 ^= t;
        *s3 = s3.rotate_left(45);
        result
    }

    /// Uniform value in `0..bound`. Returns 0 when `bound` is 0.
    pub fn below(&mut self, bound: usize) -> usize {
        let Ok(bound) = u64::try_from(bound) else {
            return 0;
        };
        if bound == 0 {
            return 0;
        }
        // Rejection sampling removes the modulo bias.
        let zone = u64::MAX - (u64::MAX % bound);
        loop {
            let value = self.next_u64();
            if value < zone {
                return usize::try_from(value % bound).unwrap_or(0);
            }
        }
    }

    /// Rolls one six-sided die.
    pub fn die(&mut self) -> u8 {
        // safe: below(6) is at most 5.
        u8::try_from(self.below(6)).unwrap_or(0) + 1
    }

    /// Shuffles a slice in place (Fisher–Yates).
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for last in (1..items.len()).rev() {
            let other = self.below(last + 1);
            items.swap(last, other);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_rolls() {
        let mut first = Rng::new(42);
        let mut second = Rng::new(42);
        let a: Vec<u8> = (0..100).map(|_| first.die()).collect();
        let b: Vec<u8> = (0..100).map(|_| second.die()).collect();
        assert_eq!(a, b);
    }

    #[test]
    fn dice_cover_one_to_six() {
        let mut rng = Rng::new(7);
        let mut seen = [0_u32; 6];
        for _ in 0..6000 {
            seen[usize::from(rng.die() - 1)] += 1;
        }
        assert!(
            seen.iter().all(|&count| (800..1200).contains(&count)),
            "{seen:?}"
        );
    }

    #[test]
    fn known_output_never_changes() {
        // Saved games depend on this exact sequence.
        let mut rng = Rng::new(1);
        let rolls: Vec<u8> = (0..12).map(|_| rng.die()).collect();
        assert_eq!(rolls, KNOWN_ROLLS);
    }

    const KNOWN_ROLLS: [u8; 12] = [2, 5, 3, 6, 6, 5, 3, 4, 2, 5, 2, 5];
}

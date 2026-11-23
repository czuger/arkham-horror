//! How dangerous a monster is, at a glance: a rough score of what facing it
//! may cost, shown as a four-step gauge in its bubbles.

use data::{Check, MonsterAbility};
use engine::MonsterStats;

/// A monster's danger, from its current values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Danger {
    /// Little to lose.
    Low,
    /// A real cost.
    Medium,
    /// Hard to beat, costly to fail.
    High,
    /// Best avoided.
    Extreme,
}

impl Danger {
    /// The danger of a monster with these values.
    pub fn of(stats: &MonsterStats) -> Self {
        match score(stats) {
            ..=4 => Self::Low,
            5..=7 => Self::Medium,
            8..=10 => Self::High,
            _ => Self::Extreme,
        }
    }

    /// « Danger : élevé ●●●○ ».
    pub fn line(self) -> String {
        format!("Danger : {}", self.gauge())
    }

    /// « élevé ●●●○ », never split across two lines.
    pub fn gauge(self) -> String {
        let (label, filled) = match self {
            Self::Low => ("faible", 1),
            Self::Medium => ("moyen", 2),
            Self::High => ("élevé", 3),
            Self::Extreme => ("très\u{a0}élevé", 4),
        };
        format!(
            "{label}\u{a0}{}{}",
            "●".repeat(filled),
            "○".repeat(4 - filled)
        )
    }
}

/// Points for each thing that makes the monster costly: what its two checks
/// take and how hard they are, its toughness, and its abilities.
fn score(stats: &MonsterStats) -> u32 {
    let abilities: u32 = stats.abilities.iter().map(|a| ability_points(*a)).sum();
    check_points(stats.horror)
        + check_points(stats.combat)
        + stats.toughness.saturating_sub(1)
        + abilities
}

/// What a failed check takes (a special effect counts as 3), plus one point
/// per die it removes.
fn check_points(check: Option<Check>) -> u32 {
    check.map_or(0, |check| {
        let loss = if check.damage == 0 { 3 } else { check.damage };
        let harder = check.modifier.min(0).unsigned_abs();
        u32::from(loss) + u32::from(harder)
    })
}

fn ability_points(ability: MonsterAbility) -> u32 {
    match ability {
        MonsterAbility::PhysicalResistance | MonsterAbility::MagicalResistance => 1,
        MonsterAbility::PhysicalImmunity | MonsterAbility::MagicalImmunity => 2,
        MonsterAbility::Ambush => 1,
        MonsterAbility::Nightmarish(n) | MonsterAbility::Overwhelming(n) => u32::from(n),
        MonsterAbility::Endless | MonsterAbility::Undead | MonsterAbility::Mask => 0,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use data::GameData;

    use super::*;
    use crate::views::tips::printed;

    fn danger(data: &GameData, name: &str) -> Danger {
        let monster = data
            .monsters
            .monsters
            .iter()
            .find(|monster| monster.name == name)
            .expect("monster");
        Danger::of(&printed(monster))
    }

    #[test]
    fn the_gauge_runs_from_cultist_to_dhole() {
        let data = GameData::load(Path::new("../../data")).expect("data");
        assert_eq!(danger(&data, "Cultiste"), Danger::Low);
        assert_eq!(danger(&data, "Zombie"), Danger::Medium);
        assert_eq!(danger(&data, "Vampire"), Danger::High);
        assert_eq!(danger(&data, "Dhole"), Danger::Extreme);
        assert_eq!(Danger::High.line(), "Danger : élevé\u{a0}●●●○");
    }
}

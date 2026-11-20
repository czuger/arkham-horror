//! Monster values once lasting effects are applied.

use data::{Check, MonsterAbility, MonsterId, Movement, Ongoing};

use crate::game::Game;
use crate::halt::Flow;

/// The values of a monster as they stand now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MonsterStats {
    /// Modifier to Evade checks.
    pub awareness: i32,
    /// Horror check, if any.
    pub horror: Option<Check>,
    /// Combat check, if any.
    pub combat: Option<Check>,
    /// Successes needed to defeat it.
    pub toughness: u32,
    /// Abilities in bold.
    pub abilities: Vec<MonsterAbility>,
    /// Movement type.
    pub movement: Movement,
}

impl MonsterStats {
    /// Whether the monster has an ability.
    pub fn has(&self, ability: MonsterAbility) -> bool {
        self.abilities.contains(&ability)
    }
}

impl Game<'_> {
    /// Current values of a monster: printed values, then the Ancient One's
    /// worshippers and power, then the Mythos cards in play.
    pub fn monster_stats(&self, id: &MonsterId) -> Flow<MonsterStats> {
        let printed = self.monster(id)?;
        let mut stats = MonsterStats {
            awareness: i32::from(printed.awareness),
            horror: printed.horror,
            combat: printed.combat,
            toughness: u32::from(printed.toughness),
            abilities: printed.abilities.clone(),
            movement: printed.movement,
        };
        self.apply_worshippers(id, &mut stats);
        if id.as_str() == "maniac" && self.state.terror >= 6 {
            stats.combat = Some(Check {
                modifier: -2,
                damage: 3,
            });
            stats.abilities.push(MonsterAbility::Endless);
        }
        self.apply_mythos_to_monster(id, &mut stats);
        Ok(stats)
    }

    /// The Ancient One's toughness bonus and the Mythos cards in play.
    fn apply_mythos_to_monster(&self, id: &MonsterId, stats: &mut MonsterStats) {
        let mut toughness =
            i32::try_from(stats.toughness).unwrap_or(1) + self.power_toughness_bonus();
        for effect in self.mythos_ongoing() {
            match effect {
                Ongoing::MonsterToughness { monsters, amount } if monsters.contains(id) => {
                    toughness += i32::from(*amount);
                }
                Ongoing::UndeadToughness(amount) if stats.has(MonsterAbility::Undead) => {
                    toughness += i32::from(*amount);
                }
                Ongoing::FlyingDontMove if stats.movement == Movement::Flying => {
                    stats.movement = Movement::Stationary;
                }
                Ongoing::FastMoveNormally if stats.movement == Movement::Fast => {
                    stats.movement = Movement::Normal;
                }
                _ => {}
            }
        }
        stats.toughness = u32::try_from(toughness.max(1)).unwrap_or(1);
    }

    /// Whether a Mythos card in play bans this monster from play.
    pub fn is_banned(&self, id: &MonsterId) -> bool {
        self.mythos_ongoing()
            .into_iter()
            .any(|effect| matches!(effect, Ongoing::MonsterBanned(banned) if banned == id))
    }
}

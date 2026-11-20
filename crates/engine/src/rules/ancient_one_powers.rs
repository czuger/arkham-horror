//! The Ancient One's worshippers and power, coded by name (« effets
//! spéciaux »): they are active the whole game, final battle included (FAQ).

use data::{Check, MonsterAbility, MonsterId, Movement};

use crate::game::Game;
use crate::halt::Flow;
use crate::rules::monster_stats::MonsterStats;

impl Game<'_> {
    /// Id of the Ancient One of the game.
    fn ancient(&self) -> &str {
        self.state.ancient_one.as_str()
    }

    /// Worshippers: changes to some monsters.
    pub fn apply_worshippers(&self, id: &MonsterId, stats: &mut MonsterStats) {
        match (self.ancient(), id.as_str()) {
            ("azathoth", "maniac") => stats.toughness += 1,
            ("cthulhu", "cultist") => {
                stats.horror = Some(Check {
                    modifier: -2,
                    damage: 2,
                })
            }
            ("hastur", "cultist") => {
                stats.movement = Movement::Flying;
                stats.combat = Some(Check {
                    modifier: -2,
                    damage: stats.combat.map_or(1, |c| c.damage),
                });
            }
            ("ithaqua", "cultist") => stats.toughness += 2,
            ("nyarlathotep", "cultist") | ("shub_niggurath", "dark_young") => {
                stats.abilities.push(MonsterAbility::Endless);
            }
            ("yig", "cultist") => {
                stats.combat = Some(Check {
                    modifier: 0,
                    damage: 4,
                })
            }
            ("yog_sothoth", "cultist") => {
                stats.abilities.push(MonsterAbility::MagicalImmunity);
                stats.combat = Some(Check {
                    modifier: -1,
                    damage: stats.combat.map_or(1, |c| c.damage),
                });
            }
            _ => {}
        }
    }

    /// Cthulhu: maximum Sanity and Stamina reduced by 1.
    pub fn power_max_reduction(&self) -> i32 {
        i32::from(self.ancient() == "cthulhu")
    }

    /// Shub-Niggurath: every monster has +1 toughness.
    pub fn power_toughness_bonus(&self) -> i32 {
        i32::from(self.ancient() == "shub_niggurath")
    }

    /// Clue tokens needed to seal a gate (Hastur: 8).
    /// Mythos cards in play change it (« Mauvais Présages »…).
    pub fn seal_cost(&self) -> u16 {
        let base: i32 = if self.ancient() == "hastur" { 8 } else { 5 };
        let change: i32 = self
            .mythos_ongoing()
            .into_iter()
            .map(|effect| match effect {
                data::Ongoing::SealCost(amount) => i32::from(*amount),
                _ => 0,
            })
            .sum();
        u16::try_from((base + change).max(0)).unwrap_or(0)
    }

    /// Successes needed to close a gate (Yog-Sothoth: 2).
    pub fn close_difficulty(&self) -> u8 {
        if self.ancient() == "yog_sothoth" {
            2
        } else {
            1
        }
    }

    /// Whether the Mask monsters go into the cup (Nyarlathotep).
    pub fn masks_in_play(&self) -> bool {
        self.ancient() == "nyarlathotep"
    }

    /// Whether the awakening loses the game at once (Azathoth).
    pub fn awakening_is_defeat(&self) -> bool {
        self.ancient() == "azathoth"
    }

    /// Whether Weather cards are discarded without effect (Ithaqua).
    pub fn weather_cancelled(&self) -> bool {
        self.ancient() == "ithaqua"
    }

    /// Yig gains a doom token, Yog-Sothoth devours: an investigator got lost
    /// in time and space.
    pub fn on_lost_in_time(&mut self, seat: usize) -> Flow<()> {
        match self.ancient() {
            "yig" => self.add_doom(),
            "yog_sothoth" => {
                self.devour(seat)?;
                Err(crate::halt::Halt::KnockedOut(seat))
            }
            _ => Ok(()),
        }
    }

    /// Yig gains a doom token when a Cultist is defeated.
    pub fn on_monster_defeated(&mut self, id: &MonsterId) -> Flow<()> {
        if self.ancient() == "yig" && id.as_str() == "cultist" {
            return self.add_doom();
        }
        Ok(())
    }

    /// Ithaqua: investigators in the streets lose 1 Stamina at the end of
    /// the Mythos phase.
    pub fn power_end_of_mythos(&mut self) -> Flow<()> {
        if self.ancient() != "ithaqua" {
            return Ok(());
        }
        for seat in self.state.seat_order() {
            if matches!(self.inv(seat).place, crate::state::Place::Street(_)) {
                self.catch_knock_out(seat, |game| game.lose(seat, data::Resource::Stamina, 1))?;
            }
        }
        Ok(())
    }
}

//! Evading and fighting monsters (§12, §13).

use data::{CheckKind, MonsterAbility, MonsterId, Resource};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::monster_stats::MonsterStats;

/// How facing a monster ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Faced {
    /// Evaded: no combat.
    Evaded,
    /// Defeated in combat.
    Defeated,
    /// Fled from the combat, or the monster could not be fought.
    Fled,
}

/// A combat in progress.
pub(crate) struct Combat {
    /// The monster fought.
    pub monster: MonsterId,
    /// Its current values.
    pub stats: MonsterStats,
    /// Weapons and spells in hand.
    pub gear: Gear,
    /// Every Combat check fails (The Bloated Woman).
    pub doomed: bool,
}

/// Weapons and spells in hand against a foe.
#[derive(Debug, Clone, Default)]
pub(crate) struct Gear {
    /// Defenses of the foe (resistances and immunities).
    pub defenses: Vec<MonsterAbility>,
    /// Cards in hand, with the bonus each one gives.
    pub equipped: Vec<Equipped>,
}

/// A card in hand during a combat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Equipped {
    /// The card.
    pub card: data::CardId,
    /// Hands it takes.
    pub hands: u8,
    /// Combat bonus it gives against this foe (0 for a failed spell).
    pub bonus: i32,
}

impl Gear {
    /// Total combat bonus of the cards in hand.
    pub fn bonus(&self) -> i32 {
        self.equipped.iter().map(|card| card.bonus).sum()
    }

    /// Hands in use.
    pub fn hands(&self) -> u8 {
        self.equipped.iter().map(|card| card.hands).sum()
    }

    /// Whether a card is in hand.
    pub fn holds(&self, card: &data::CardId) -> bool {
        self.equipped.iter().any(|equipped| &equipped.card == card)
    }
}

impl Game<'_> {
    /// Tries to evade a monster; a failure hurts and starts the combat.
    pub fn evade_or_fight(&mut self, seat: usize, monster: &MonsterId, fight: bool) -> Flow<Faced> {
        let stats = self.monster_stats(monster)?;
        if !fight {
            self.log(format!(
                "{} tente d'échapper à {}.",
                self.name(seat),
                self.monster_name(monster)
            ));
            if self
                .check(seat, CheckKind::Evade, stats.awareness, 1)?
                .passed
            {
                return Ok(Faced::Evaded);
            }
            self.monster_hits(seat, monster, &stats)?;
        }
        self.combat(seat, monster.clone(), stats)
    }

    /// A full combat: Horror check once, then rounds of fight or flee.
    fn combat(&mut self, seat: usize, monster: MonsterId, stats: MonsterStats) -> Flow<Faced> {
        self.log(format!(
            "{} combat {}.",
            self.name(seat),
            self.monster_name(&monster)
        ));
        let gear = Gear {
            defenses: stats.abilities.clone(),
            ..Gear::default()
        };
        let mut combat = Combat {
            monster,
            stats,
            gear,
            doomed: false,
        };
        if let Some(faced) = self.before_horror(seat, &mut combat)? {
            return Ok(faced);
        }
        self.horror_check(seat, &combat)?;
        if combat.stats.combat.is_none() {
            return Ok(Faced::Fled);
        }
        loop {
            if let Some(faced) = self.combat_round(seat, &mut combat)? {
                return Ok(faced);
            }
        }
    }

    /// The Horror check: Sanity loss on a failure, Nightmarish on a success.
    fn horror_check(&mut self, seat: usize, combat: &Combat) -> Flow<()> {
        let Some(horror) = combat.stats.horror else {
            return Ok(());
        };
        let passed = !combat.doomed
            && self
                .check(seat, CheckKind::Horror, i32::from(horror.modifier), 1)?
                .passed;
        let nightmarish = combat
            .stats
            .abilities
            .iter()
            .find_map(|ability| match ability {
                MonsterAbility::Nightmarish(amount) => Some(u32::from(*amount)),
                _ => None,
            });
        let loss = if passed {
            nightmarish.unwrap_or(0)
        } else {
            u32::from(horror.damage)
        };
        self.lose(seat, Resource::Sanity, loss)
    }

    /// One round: arm, then fight or flee. Returns how the combat ended, if it did.
    fn combat_round(&mut self, seat: usize, combat: &mut Combat) -> Flow<Option<Faced>> {
        self.prepare_combat(seat, &mut combat.gear)?;
        let can_flee = !combat.stats.has(MonsterAbility::Ambush);
        self.hopeless_combat(seat, combat, can_flee)?;
        let mut options = vec![option(Choice::Attack, "Combattre")];
        if can_flee {
            options.push(option(Choice::Flee, "Fuir"));
        }
        let prompt = format!(
            "Combat contre {} : combattre ou fuir ?",
            self.monster_name(&combat.monster)
        );
        if self.ask(seat, DecisionKind::FightOrFlee, prompt, options)? == Choice::Flee {
            if self
                .check(seat, CheckKind::Evade, combat.stats.awareness, 1)?
                .passed
            {
                self.log(format!("{} s'enfuit.", self.name(seat)));
                return Ok(Some(Faced::Fled));
            }
            self.monster_hits(seat, &combat.monster, &combat.stats)?;
            return Ok(None);
        }
        let passed = self.combat_check(seat, combat)?;
        self.discard_used_weapons(seat, &mut combat.gear)?;
        if passed {
            self.monster_beaten(seat, combat)?;
            return Ok(Some(Faced::Defeated));
        }
        self.monster_hits(seat, &combat.monster, &combat.stats)?;
        Ok(None)
    }

    /// The Combat check: Fight (Lore against The Black Pharaoh), weapons and
    /// spells, the monster's combat rating; toughness successes needed.
    fn combat_check(&mut self, seat: usize, combat: &Combat) -> Flow<bool> {
        if combat.doomed {
            self.log("Le test de Combat échoue automatiquement.");
            return Ok(false);
        }
        let rating = combat
            .stats
            .combat
            .map_or(0, |check| i32::from(check.modifier));
        let toughness = u8::try_from(combat.stats.toughness).unwrap_or(u8::MAX);
        let kind = if combat.monster.as_str() == "the_black_pharaoh" {
            CheckKind::Lore
        } else {
            CheckKind::Combat
        };
        Ok(self
            .check(seat, kind, rating + combat.gear.bonus(), toughness)?
            .passed)
    }

    /// A monster defeated: Overwhelming hurts anyway, then trophy or special fate.
    fn monster_beaten(&mut self, seat: usize, combat: &Combat) -> Flow<()> {
        let overwhelming = combat
            .stats
            .abilities
            .iter()
            .find_map(|ability| match ability {
                MonsterAbility::Overwhelming(amount) => Some(u32::from(*amount)),
                _ => None,
            });
        if let Some(amount) = overwhelming {
            self.lose(seat, Resource::Stamina, amount)?;
        }
        self.monster_defeated_special(seat, &combat.monster)
    }

    /// No hope left (cannot flee, cannot roll enough dice): the investigator
    /// goes insane and the combat stops (FAQ).
    fn hopeless_combat(&mut self, seat: usize, combat: &Combat, can_flee: bool) -> Flow<()> {
        if can_flee || combat.doomed {
            return Ok(());
        }
        let rating = combat
            .stats
            .combat
            .map_or(0, |check| i32::from(check.modifier));
        let dice = self
            .check_dice(seat, CheckKind::Combat, rating + combat.gear.bonus())?
            .max(0);
        let clue_dice = i32::from(self.inv(seat).clues)
            * i32::try_from(self.dice_per_clue(seat, CheckKind::Combat)).unwrap_or(1);
        if dice + clue_dice >= i32::try_from(combat.stats.toughness).unwrap_or(i32::MAX) {
            return Ok(());
        }
        self.log("Combat sans issue.");
        self.inv_mut(seat).sanity = 0;
        self.after_loss(seat)
    }
}

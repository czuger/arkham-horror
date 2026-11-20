//! Evading and fighting monsters (§12, §13).

use data::{CheckKind, Moment, MonsterAbility, MonsterId, MonsterTrait, Resource};

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
    /// The Red Sign was cast: the monster is to be weakened.
    pub weaken: bool,
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
    /// Bonus before the foe's defenses.
    pub raw: i32,
    /// Physical or magical (spells, enchanted weapons).
    pub kind: data::WeaponKind,
    /// Bonus instead of `raw` when nothing else is in hand (Axe).
    pub alone: Option<i32>,
    /// Each 6 rolled counts as 2 successes (Shotgun).
    pub sixes_double: bool,
}

impl Gear {
    /// Total combat bonus of the cards in hand.
    pub fn bonus(&self) -> i32 {
        self.equipped.iter().map(|card| card.bonus).sum()
    }

    /// Bonuses after the foe's defenses, once these changed.
    pub fn rate(&mut self) {
        let hands = self.hands();
        for equipped in &mut self.equipped {
            let raw = match equipped.alone {
                Some(alone) if hands == equipped.hands => alone,
                _ => equipped.raw,
            };
            equipped.bonus =
                crate::rules::combat_gear::reduced_bonus(&self.defenses, equipped.kind, raw);
        }
    }

    /// Whether a card in hand doubles the sixes.
    pub fn sixes_double(&self) -> bool {
        self.equipped.iter().any(|equipped| equipped.sixes_double)
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
        let faced = self.combat(seat, monster.clone(), stats)?;
        self.fire_triggers(seat, Moment::EndOfCombat)?;
        Ok(faced)
    }

    /// A full combat: Horror check once, then rounds of fight or flee.
    fn combat(&mut self, seat: usize, monster: MonsterId, stats: MonsterStats) -> Flow<Faced> {
        self.log(format!(
            "{} combat {}.",
            self.name(seat),
            self.monster_name(&monster)
        ));
        let ignored = |ability: &MonsterAbility| match ability {
            MonsterAbility::PhysicalResistance => {
                self.ignores(seat, MonsterTrait::PhysicalResistance)
            }
            MonsterAbility::MagicalResistance => {
                self.ignores(seat, MonsterTrait::MagicalResistance)
            }
            _ => false,
        };
        let defenses = stats
            .abilities
            .iter()
            .filter(|a| !ignored(a))
            .cloned()
            .collect();
        let gear = Gear {
            defenses,
            ..Gear::default()
        };
        self.scratch.flute = false;
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
            if self.ignores(seat, MonsterTrait::Nightmarish) {
                0
            } else {
                nightmarish.unwrap_or(0)
            }
        } else {
            u32::from(horror.damage)
        };
        self.lose(seat, Resource::Sanity, loss)
    }

    /// One round: arm, then fight or flee. Returns how the combat ended, if it did.
    fn combat_round(&mut self, seat: usize, combat: &mut Combat) -> Flow<Option<Faced>> {
        self.prepare_combat(seat, &mut combat.gear)?;
        self.weaken_monster(seat, combat)?;
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
        self.scratch.no_combat_damage = false;
        let passed = match self.special_attack(seat, combat)? {
            Some(true) if self.scratch.flute => {
                self.discard_used_weapons(seat, &mut combat.gear)?;
                self.on_defeat_effects(seat, &combat.monster)?;
                self.monster_defeated_special(seat, &combat.monster)?;
                return Ok(Some(Faced::Defeated));
            }
            Some(passed) => passed,
            None => self.combat_check(seat, combat)?,
        };
        self.discard_used_weapons(seat, &mut combat.gear)?;
        if passed {
            self.monster_beaten(seat, combat)?;
            return Ok(Some(Faced::Defeated));
        }
        if std::mem::take(&mut self.scratch.no_combat_damage) {
            self.log("Les dégâts de combat sont annulés.");
            return Ok(None);
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
        self.gear_check(seat, kind, rating, &combat.gear, toughness)
    }

    /// A check made with the cards in hand: their bonus, and Shotgun sixes.
    pub fn gear_check(
        &mut self,
        seat: usize,
        kind: CheckKind,
        rating: i32,
        gear: &Gear,
        needed: u8,
    ) -> Flow<bool> {
        self.scratch.sixes_double = gear.sixes_double();
        let result = self.check(seat, kind, rating + gear.bonus(), needed);
        self.scratch.sixes_double = false;
        Ok(result?.passed)
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
        if let Some(amount) = overwhelming
            && !self.ignores(seat, MonsterTrait::Overwhelming)
        {
            self.lose(seat, Resource::Stamina, amount)?;
        }
        self.on_defeat_effects(seat, &combat.monster)?;
        self.monster_defeated_special(seat, &combat.monster)
    }

    /// No hope left (cannot roll enough dice to flee nor to win): the
    /// investigator goes insane and the combat stops (FAQ).
    fn hopeless_combat(&mut self, seat: usize, combat: &Combat, can_flee: bool) -> Flow<()> {
        if combat.doomed {
            return Ok(());
        }
        if can_flee && self.most_dice(seat, CheckKind::Evade, combat.stats.awareness)? >= 1 {
            return Ok(());
        }
        let rating = combat
            .stats
            .combat
            .map_or(0, |check| i32::from(check.modifier));
        let dice = self.most_dice(seat, CheckKind::Combat, rating + combat.gear.bonus())?;
        if dice >= i32::try_from(combat.stats.toughness).unwrap_or(i32::MAX) {
            return Ok(());
        }
        self.log("Combat sans issue.");
        self.inv_mut(seat).sanity = 0;
        self.after_loss(seat)
    }

    /// Dice a check could roll at most: its own dice plus every clue token.
    fn most_dice(&self, seat: usize, kind: CheckKind, modifier: i32) -> Flow<i32> {
        let dice = self.check_dice(seat, kind, modifier)?.max(0);
        let clue_dice = i32::from(self.inv(seat).clues)
            * i32::try_from(self.dice_per_clue(seat, kind)).unwrap_or(1);
        Ok(dice + clue_dice)
    }
}

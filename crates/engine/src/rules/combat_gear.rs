//! Weapons and spells in combat, within the two-hand limit.

use data::{Ability, CardId, Effect, MonsterAbility, MonsterId, Timing, WeaponKind};

use crate::decision::{Choice, Context, DecisionKind, DecisionOption};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::combat::{Combat, Equipped, Gear};
use crate::rules::monster_stats::MonsterStats;

/// Hands an investigator has.
const HANDS: u8 = 2;

/// Combat bonus left once the monster's resistance or immunity applies.
pub(crate) fn reduced_bonus(stats: &[MonsterAbility], kind: WeaponKind, bonus: i32) -> i32 {
    let (immunity, resistance) = match kind {
        WeaponKind::Physical => (
            MonsterAbility::PhysicalImmunity,
            MonsterAbility::PhysicalResistance,
        ),
        WeaponKind::Magical => (
            MonsterAbility::MagicalImmunity,
            MonsterAbility::MagicalResistance,
        ),
    };
    if stats.contains(&immunity) {
        0
    } else if stats.contains(&resistance) {
        (bonus + 1) / 2
    } else {
        bonus
    }
}

/// Whether an ability is a combat spell (« jusqu'à la fin de ce combat »).
fn is_combat_spell(ability: &Ability) -> bool {
    ability.cast
        && ability.timing == Timing::AnyPhase
        && ability.effects.iter().any(|effect| match effect {
            Effect::CombatBonus(_) => true,
            Effect::Special(id) => COMBAT_SPECIALS.contains(&id.as_str()),
            _ => false,
        })
}

/// Special spells cast during a combat.
const COMBAT_SPECIALS: [&str; 2] = ["enchant_weapon", "red_sign"];

/// French name of a monster ability.
fn ability_name(ability: &MonsterAbility) -> String {
    match ability {
        MonsterAbility::Ambush => "Embuscade".into(),
        MonsterAbility::Endless => "Inépuisable".into(),
        MonsterAbility::Undead => "Mort-vivant".into(),
        MonsterAbility::Mask => "Masque".into(),
        MonsterAbility::PhysicalResistance => "Résistance Physique".into(),
        MonsterAbility::PhysicalImmunity => "Immunité Physique".into(),
        MonsterAbility::MagicalResistance => "Résistance Magique".into(),
        MonsterAbility::MagicalImmunity => "Immunité Magique".into(),
        MonsterAbility::Nightmarish(n) => format!("Cauchemardesque {n}"),
        MonsterAbility::Overwhelming(n) => format!("Irrésistible {n}"),
    }
}

impl Game<'_> {
    /// Before the Combat check: take weapons and cast combat spells, one at a
    /// time, as long as hands are free; cards already in hand can be let go.
    /// `foe` is the monster fought, `None` against the Ancient One.
    pub fn prepare_combat(
        &mut self,
        seat: usize,
        foe: Option<&MonsterId>,
        gear: &mut Gear,
    ) -> Flow<()> {
        loop {
            let options = self.gear_options(seat, gear)?;
            let prompt = format!(
                "Mains utilisées : {}/{HANDS}. Bonus de combat : {:+}.",
                gear.hands(),
                gear.bonus()
            );
            let context = Context::Combat {
                foe: foe.cloned(),
                bonus: gear.bonus(),
            };
            match self.ask_about(seat, DecisionKind::PrepareCombat, context, prompt, options)? {
                Choice::Card(card) => self.equip(seat, gear, card)?,
                Choice::Drop(card) => {
                    gear.equipped.retain(|equipped| equipped.card != card);
                    self.log(format!(
                        "{} lâche {}.",
                        self.name(seat),
                        self.card_name(&card)
                    ));
                }
                _ => return Ok(()),
            }
        }
    }

    /// Cards that can be taken in hand or let go now.
    fn gear_options(&self, seat: usize, gear: &Gear) -> Flow<Vec<DecisionOption>> {
        let mut options = vec![option(Choice::Attack, "Passer au combat")];
        for held in &self.inv(seat).cards {
            let choice = Choice::Card(held.card.clone());
            if gear.holds(&held.card) || options.iter().any(|o| o.choice == choice) {
                continue;
            }
            let card = self.card(&held.card)?;
            let fits = gear.hands() + card.hands <= HANDS;
            let weapon = card.weapon.is_some();
            let spell =
                !held.exhausted && !self.no_spells() && card.abilities.iter().any(is_combat_spell);
            if fits && (weapon || spell) {
                options.push(option(choice, format!("Prendre en main {}", card.name)));
            }
        }
        for equipped in &gear.equipped {
            let label = format!("Lâcher {}", self.card_name(&equipped.card));
            options.push(option(Choice::Drop(equipped.card.clone()), label));
        }
        Ok(options)
    }

    /// Takes a weapon in hand or casts a combat spell. A spell takes its
    /// hands even when the casting fails.
    fn equip(&mut self, seat: usize, gear: &mut Gear, card: CardId) -> Flow<()> {
        let printed = self.card(&card)?;
        let (mut raw, mut kind) = (0, WeaponKind::Magical);
        let (mut alone, mut sixes_double) = (None, false);
        if let Some(weapon) = printed.weapon {
            self.covered.insert(crate::coverage::used_key(&card));
            (raw, kind) = (i32::from(weapon.bonus), weapon.kind);
            if let Some(bonus) = weapon.undead_bonus
                && gear.defenses.contains(&MonsterAbility::Undead)
            {
                raw = i32::from(bonus);
            }
            alone = weapon.alone_bonus.map(i32::from);
            sixes_double = weapon.sixes_double;
        } else if let Some(ability) = printed.abilities.iter().find(|a| is_combat_spell(a)) {
            self.scratch.combat_bonus = 0;
            self.scratch.enchant_weapon = false;
            self.scratch.red_sign = false;
            if self.use_ability(seat, &card, ability)? {
                raw = self.scratch.combat_bonus;
                gear.weaken |= self.scratch.red_sign;
                if self.scratch.enchant_weapon {
                    self.enchant_weapon(seat, gear)?;
                }
            }
        }
        gear.equipped.push(Equipped {
            card,
            hands: printed.hands,
            bonus: 0,
            raw,
            kind,
            alone,
            sixes_double,
        });
        gear.rate();
        Ok(())
    }

    /// Enchant Weapon: a physical weapon in hand becomes magical for the
    /// rest of the combat.
    fn enchant_weapon(&mut self, seat: usize, gear: &mut Gear) -> Flow<()> {
        let options: Vec<_> = gear
            .equipped
            .iter()
            .filter(|equipped| equipped.kind == WeaponKind::Physical)
            .map(|equipped| {
                let label = self.card_name(&equipped.card);
                option(Choice::Card(equipped.card.clone()), label)
            })
            .collect();
        if options.is_empty() {
            self.log("Aucune Arme Physique en main à enchanter.");
            return Ok(());
        }
        let prompt = "Quelle arme enchantez-vous ?";
        if let Choice::Card(card) = self.ask(seat, DecisionKind::PrepareCombat, prompt, options)? {
            for equipped in gear.equipped.iter_mut().filter(|e| e.card == card) {
                equipped.kind = WeaponKind::Magical;
            }
            self.log(format!(
                "{} devient une Arme Magique.",
                self.card_name(&card)
            ));
        }
        Ok(())
    }

    /// The Red Sign: toughness down by 1 (at least 1) and one special
    /// ability other than Magical Immunity ignored, for this combat.
    pub fn weaken_monster(&mut self, seat: usize, combat: &mut Combat) -> Flow<()> {
        if !std::mem::take(&mut combat.gear.weaken) {
            return Ok(());
        }
        combat.stats.toughness = combat.stats.toughness.saturating_sub(1).max(1);
        let options: Vec<_> = combat
            .stats
            .abilities
            .iter()
            .enumerate()
            .filter(|(_, ability)| !matches!(ability, MonsterAbility::MagicalImmunity))
            .map(|(index, ability)| option(Choice::Option(index), ability_name(ability)))
            .collect();
        self.log(format!(
            "Force de {} : {}.",
            self.monster_name(&combat.monster),
            combat.stats.toughness
        ));
        if options.is_empty() {
            return Ok(());
        }
        let prompt = "Quelle capacité du monstre ignorez-vous ?";
        if let Choice::Option(index) =
            self.ask(seat, DecisionKind::PrepareCombat, prompt, options)?
            && index < combat.stats.abilities.len()
        {
            let ignored = combat.stats.abilities.remove(index);
            combat.gear.defenses.retain(|defense| defense != &ignored);
            combat.gear.rate();
            self.log(format!("{} est ignorée.", ability_name(&ignored)));
        }
        Ok(())
    }

    /// After the Combat check: weapons that cost Sanity take it, and those
    /// good for one use are discarded.
    pub fn discard_used_weapons(&mut self, seat: usize, gear: &mut Gear) -> Flow<()> {
        let mut kept = Vec::new();
        for equipped in std::mem::take(&mut gear.equipped) {
            let printed = self.card(&equipped.card)?;
            let loss = printed.weapon.map_or(0, |weapon| weapon.sanity_loss);
            if loss > 0 {
                self.lose(seat, data::Resource::Sanity, u32::from(loss))?;
            }
            if printed
                .weapon
                .is_some_and(|weapon| weapon.discard_after_use)
            {
                self.log(format!("{} est défaussé après usage.", printed.name));
                self.discard_card(seat, &equipped.card)?;
            } else {
                kept.push(equipped);
            }
        }
        gear.equipped = kept;
        Ok(())
    }

    /// Combat damage, or the monster's special damage.
    pub fn monster_hits(
        &mut self,
        seat: usize,
        monster: &data::MonsterId,
        stats: &MonsterStats,
    ) -> Flow<()> {
        if self.monster_special_hit(seat, monster)? {
            return Ok(());
        }
        let damage = stats.combat.map_or(0, |check| u32::from(check.damage));
        self.lose(seat, data::Resource::Stamina, damage)
    }
}

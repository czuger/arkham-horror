//! Weapons and spells in combat, within the two-hand limit.

use data::{Ability, CardId, Effect, MonsterAbility, Timing, WeaponKind};

use crate::decision::{Choice, DecisionKind, DecisionOption};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::combat::{Equipped, Gear};
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
        && ability
            .effects
            .iter()
            .any(|effect| matches!(effect, Effect::CombatBonus(_)))
}

impl Game<'_> {
    /// Before the Combat check: take weapons and cast combat spells, one at a
    /// time, as long as hands are free; cards already in hand can be let go.
    pub fn prepare_combat(&mut self, seat: usize, gear: &mut Gear) -> Flow<()> {
        loop {
            let options = self.gear_options(seat, gear)?;
            let prompt = format!(
                "Mains utilisées : {}/{HANDS}. Bonus de combat : {:+}.",
                gear.hands(),
                gear.bonus()
            );
            match self.ask(seat, DecisionKind::PrepareCombat, prompt, options)? {
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
            let spell = !held.exhausted && card.abilities.iter().any(is_combat_spell);
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
        let mut bonus = 0;
        if let Some(weapon) = printed.weapon {
            bonus = reduced_bonus(&gear.defenses, weapon.kind, i32::from(weapon.bonus));
        } else if let Some(ability) = printed.abilities.iter().find(|a| is_combat_spell(a)) {
            self.scratch.combat_bonus = 0;
            if self.use_ability(seat, &card, ability)? {
                bonus = reduced_bonus(
                    &gear.defenses,
                    WeaponKind::Magical,
                    self.scratch.combat_bonus,
                );
            }
        }
        gear.equipped.push(Equipped {
            card,
            hands: printed.hands,
            bonus,
        });
        Ok(())
    }

    /// Weapons good for one use are discarded after the Combat check.
    pub fn discard_used_weapons(&mut self, seat: usize, gear: &mut Gear) -> Flow<()> {
        let mut kept = Vec::new();
        for equipped in std::mem::take(&mut gear.equipped) {
            let printed = self.card(&equipped.card)?;
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

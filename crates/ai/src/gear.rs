//! Weapons and combat spells: what they are worth against a foe.

use data::{CardId, CheckKind, Effect, InvestigatorCard, MonsterAbility, WeaponKind};
use engine::View;

use crate::odds::chance;

/// Hands an investigator has.
const HANDS: u8 = 2;

/// Combat bonus a card would give against a foe with these defenses,
/// weighted by the chance of casting it for a spell. `None` when the card
/// is no use in a combat.
pub fn combat_bonus(
    view: &View<'_, '_>,
    seat: usize,
    card: &InvestigatorCard,
    defenses: &[MonsterAbility],
) -> Option<f64> {
    if let Some(weapon) = card.weapon {
        let raw = match weapon.undead_bonus {
            Some(bonus) if defenses.contains(&MonsterAbility::Undead) => bonus,
            _ => weapon.bonus,
        };
        return Some(reduced(defenses, weapon.kind, f64::from(raw)));
    }
    let bonus = card.abilities.iter().find_map(|ability| {
        let bonus = ability.effects.iter().find_map(|effect| match effect {
            Effect::CombatBonus(bonus) => Some(*bonus),
            _ => None,
        })?;
        ability.cast.then_some(bonus)
    })?;
    if !view.spells_allowed() {
        return None;
    }
    let casting = card.casting?;
    let inv = view.state().investigators.get(seat)?;
    if inv.sanity <= casting.sanity_cost.saturating_add(1) {
        return None;
    }
    let dice = view.check_dice(seat, CheckKind::Spell, i32::from(casting.modifier));
    let cast = chance(dice, 1, view.success_face(seat));
    Some(cast * reduced(defenses, WeaponKind::Magical, f64::from(bonus)))
}

/// Bonus left once the foe's resistance or immunity applies.
fn reduced(defenses: &[MonsterAbility], kind: WeaponKind, bonus: f64) -> f64 {
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
    if defenses.contains(&immunity) {
        0.0
    } else if defenses.contains(&resistance) {
        ((bonus + 1.0) / 2.0).floor()
    } else {
        bonus
    }
}

/// Best total bonus of the cards held, within two hands.
pub fn best_bonus(view: &View<'_, '_>, seat: usize, defenses: &[MonsterAbility]) -> f64 {
    let Some(inv) = view.state().investigators.get(seat) else {
        return 0.0;
    };
    let mut usable: Vec<(u8, f64, &CardId)> = Vec::new();
    for held in &inv.cards {
        let Some(card) = view.data().investigator_cards.card(&held.card) else {
            continue;
        };
        if held.exhausted && card.weapon.is_none() {
            continue;
        }
        if let Some(bonus) = combat_bonus(view, seat, card, defenses)
            && bonus > 0.0
            && !usable.iter().any(|(_, _, id)| *id == &held.card)
        {
            usable.push((card.hands, bonus, &held.card));
        }
    }
    let mut best = 0.0_f64;
    for (index, &(hands, bonus, _)) in usable.iter().enumerate() {
        if hands > HANDS {
            continue;
        }
        best = best.max(bonus);
        for &(other_hands, other_bonus, _) in usable.iter().skip(index + 1) {
            if hands + other_hands <= HANDS {
                best = best.max(bonus + other_bonus);
            }
        }
    }
    best
}

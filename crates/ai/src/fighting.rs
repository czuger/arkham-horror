//! Facing monsters: evade, fight or flee, and what to hold in hand.

use data::{CheckKind, MonsterId};
use engine::{Choice, Decision, MonsterStats, View};

use crate::gear::{best_bonus, combat_bonus};
use crate::odds::chance;

/// Fight rather than evade from this chance of winning on.
const SURE_WIN: f64 = 0.65;
/// Below this chance, a combat round is not worth it when fleeing is likely.
const POOR_WIN: f64 = 0.3;

/// Chances against one monster.
struct Odds {
    /// Of evading it (or fleeing it).
    evade: f64,
    /// Of winning one combat round with the best cards in hand.
    win: f64,
    /// The investigator would be knocked out by one more hit or Horror.
    fragile: bool,
}

/// Chances against a monster, with `bonus` from the cards in hand (the
/// best possible hand when `None`).
fn odds(view: &View<'_, '_>, seat: usize, stats: &MonsterStats, bonus: Option<i32>) -> Odds {
    let face = view.success_face(seat);
    let evade = chance(
        view.check_dice(seat, CheckKind::Evade, stats.awareness),
        1,
        face,
    );
    let bonus = bonus.map_or_else(|| best_bonus(view, seat, &stats.abilities), f64::from);
    let rating = stats.combat.map_or(0, |check| i32::from(check.modifier));
    let bonus = bonus.round() as i32;
    let dice = view.check_dice(seat, CheckKind::Combat, rating + bonus);
    let clue_dice = clue_dice(view, seat, CheckKind::Combat);
    let win = chance(dice + clue_dice, stats.toughness, face);
    let inv = view.state().investigators.get(seat);
    let damage = stats.combat.map_or(0, |check| check.damage);
    let horror = stats.horror.map_or(0, |check| check.damage);
    let fragile = inv.is_none_or(|inv| inv.stamina <= damage || inv.sanity <= horror);
    Odds {
        evade,
        win,
        fragile,
    }
}

/// Dice the investigator would add by spending clue tokens beyond those
/// kept for a seal (half of them, as a cautious guess).
fn clue_dice(view: &View<'_, '_>, seat: usize, kind: CheckKind) -> u32 {
    let clues = view
        .state()
        .investigators
        .get(seat)
        .map_or(0, |inv| u32::from(inv.clues));
    clues / 2 * view.dice_per_clue(seat, kind)
}

/// Chance of winning one combat round against a monster with the best
/// cards in hand.
pub fn fight_chance(view: &View<'_, '_>, seat: usize, monster: &MonsterId) -> Option<f64> {
    let stats = view.monster_stats(monster)?;
    Some(odds(view, seat, &stats, None).win)
}

/// Whether fighting beats evading.
fn should_fight(odds: &Odds) -> bool {
    if odds.fragile {
        return odds.win > odds.evade + 0.2;
    }
    odds.win >= SURE_WIN || (odds.evade < 0.4 && odds.win > odds.evade)
}

/// Which monster to deal with, and how: the surest move first.
pub fn face(
    view: &View<'_, '_>,
    seat: usize,
    decision: &Decision,
    monsters: &[MonsterId],
) -> usize {
    let mut best = (0, f64::MIN);
    for (index, option) in decision.options.iter().enumerate() {
        let (target, fight) = match option.choice {
            Choice::Evade(target) => (target, false),
            Choice::Fight(target) => (target, true),
            _ => continue,
        };
        let Some(stats) = monsters.get(target).and_then(|id| view.monster_stats(id)) else {
            continue;
        };
        let odds = odds(view, seat, &stats, None);
        if fight != should_fight(&odds) {
            continue;
        }
        let sure = if fight { odds.win } else { odds.evade };
        if sure > best.1 {
            best = (index, sure);
        }
    }
    best.0
}

/// Attack or flee this combat round.
pub fn fight_or_flee(
    view: &View<'_, '_>,
    seat: usize,
    decision: &Decision,
    foe: Option<&MonsterId>,
    bonus: i32,
) -> usize {
    let attack = decision.find(&Choice::Attack).unwrap_or(0);
    let Some(stats) = foe.and_then(|id| view.monster_stats(id)) else {
        return attack;
    };
    let Some(flee) = decision.find(&Choice::Flee) else {
        return attack;
    };
    let odds = odds(view, seat, &stats, Some(bonus));
    if odds.win <= f64::EPSILON {
        return flee;
    }
    let attack_wins = if odds.fragile {
        odds.win >= odds.evade
    } else {
        odds.win >= POOR_WIN || odds.win >= odds.evade
    };
    if attack_wins { attack } else { flee }
}

/// Before the Combat check: take the card that adds the most, else attack.
pub fn prepare(
    view: &View<'_, '_>,
    seat: usize,
    decision: &Decision,
    foe: Option<&MonsterId>,
) -> usize {
    let defenses = foe
        .and_then(|id| view.monster_stats(id))
        .map(|stats| stats.abilities)
        .unwrap_or_default();
    let mut best: Option<(usize, f64)> = None;
    for (index, option) in decision.options.iter().enumerate() {
        let Choice::Card(card) = &option.choice else {
            continue;
        };
        let Some(printed) = view.data().investigator_cards.card(card) else {
            continue;
        };
        let worth = combat_bonus(view, seat, printed, &defenses).unwrap_or(0.0);
        if worth > 0.0 && best.is_none_or(|(_, top)| worth > top) {
            best = Some((index, worth));
        }
    }
    if let Some((index, _)) = best {
        return index;
    }
    decision.find(&Choice::Attack).unwrap_or(0)
}

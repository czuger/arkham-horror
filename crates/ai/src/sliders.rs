//! Skill sliders: where to put them for the turn ahead.

use data::{Deck, Resource};
use engine::{Choice, Decision, View};

use crate::goal::{closing_soon, goal};
use crate::knob::knob;
use crate::value::amount_of;

/// How much the skills matter this turn.
struct Needs {
    /// Movement points worth having (more are of little use).
    speed: f64,
    /// Movement points minus the printed Speed at the current stop: the
    /// modifiers in play.
    speed_offset: f64,
    /// Weights of Sneak, Fight, Will, Lore and Luck.
    sneak: f64,
    /// Fight.
    fight: f64,
    /// Will.
    will: f64,
    /// Lore.
    lore: f64,
    /// Luck.
    luck: f64,
}

/// What the turn ahead calls for: enough Speed to reach the goal, Fight
/// or Lore to close a gate, Will when Sanity is low, Lore for spells.
fn needs(view: &View<'_, '_>, seat: usize) -> Needs {
    if view.state().battle.is_some() {
        return battle_needs(view);
    }
    let steps = goal(view, seat).map_or(0, |goal| goal.route.steps.len());
    let spells = view.state().investigators.get(seat).is_some_and(|inv| {
        inv.cards.iter().any(|held| {
            view.data()
                .investigator_cards
                .card(&held.card)
                .is_some_and(|card| card.deck == Deck::Spell)
        })
    });
    let closing = closing_soon(view, seat);
    Needs {
        speed: f64::from(u32::try_from(steps).unwrap_or(0)),
        speed_offset: speed_offset(view, seat),
        sneak: 1.0,
        fight: if closing { 1.5 } else { 1.0 },
        will: if amount_of(view, seat, Resource::Sanity) <= 3 {
            1.2
        } else {
            0.7
        },
        lore: if closing {
            1.5
        } else if spells {
            1.0
        } else {
            0.6
        },
        luck: 0.9,
    }
}

/// In the final battle: Fight above all, then the skill the Ancient One
/// attacks.
fn battle_needs(view: &View<'_, '_>) -> Needs {
    let attacked = |name: &str| {
        if attack_skill(view.state().ancient_one.as_str()) == Some(name) {
            1.5
        } else {
            0.2
        }
    };
    Needs {
        speed: 0.0,
        speed_offset: 0.0,
        sneak: attacked("sneak"),
        fight: 3.0 + attacked("fight"),
        will: attacked("will"),
        lore: attacked("lore"),
        luck: attacked("luck"),
    }
}

/// The skill an Ancient One's attack checks (Speed is not on a slider
/// worth guarding here).
fn attack_skill(ancient_one: &str) -> Option<&'static str> {
    Some(match ancient_one {
        "hastur" => "luck",
        "ithaqua" => "fight",
        "nyarlathotep" => "lore",
        "shub_niggurath" => "sneak",
        "yog_sothoth" => "will",
        _ => return None,
    })
}

/// Movement points minus the printed Speed at the current stop.
fn speed_offset(view: &View<'_, '_>, seat: usize) -> f64 {
    let stop = view
        .state()
        .investigators
        .get(seat)
        .map_or(0, |inv| inv.sliders[0]);
    let printed = track_values(view, seat, 0)
        .and_then(|(speed, _)| speed.get(usize::from(stop)).copied())
        .unwrap_or(0);
    f64::from(view.movement_points(seat)) - f64::from(printed)
}

/// Values of the two skills of a track at each stop.
fn track_values<'a>(
    view: &View<'_, 'a>,
    seat: usize,
    track: usize,
) -> Option<(&'a [u8], &'a [u8])> {
    let id = &view.state().investigators.get(seat)?.id;
    let sheet = view
        .data()
        .investigators
        .investigators
        .iter()
        .find(|sheet| &sheet.id == id)?;
    let skills = &sheet.skills;
    Some(match track {
        0 => (&skills.speed, &skills.sneak),
        1 => (&skills.fight, &skills.will),
        _ => (&skills.lore, &skills.luck),
    })
}

/// Worth of a stop on a track.
fn stop_worth(view: &View<'_, '_>, seat: usize, needs: &Needs, track: usize, stop: u8) -> f64 {
    let Some((left, right)) = track_values(view, seat, track) else {
        return 0.0;
    };
    let at = usize::from(stop);
    let left = f64::from(left.get(at).copied().unwrap_or(0));
    let right = f64::from(right.get(at).copied().unwrap_or(0));
    match track {
        0 => {
            let points = (left + needs.speed_offset).max(0.0);
            knob("SPEED", 1.5) * points.min(needs.speed) + knob("SPEED_EXTRA", 0.1) * points + needs.sneak * knob("SNEAK", 1.0) * right
        }
        1 => needs.fight * left + needs.will * right,
        _ => needs.lore * left + needs.luck * right,
    }
}

/// Best stop of a track.
fn best_stop(view: &View<'_, '_>, seat: usize, needs: &Needs, track: usize) -> u8 {
    (0..4_u8)
        .max_by(|&a, &b| {
            stop_worth(view, seat, needs, track, a)
                .total_cmp(&stop_worth(view, seat, needs, track, b))
        })
        .unwrap_or(0)
}

/// Initial placement or upkeep adjustment: move each slider towards its
/// best stop; stop adjusting once all are there.
pub fn adjust(view: &View<'_, '_>, seat: usize, decision: &Decision) -> usize {
    let current = view
        .state()
        .investigators
        .get(seat)
        .map_or([0; 3], |inv| inv.sliders);
    let needs = needs(view, seat);
    let mut best: Option<(usize, f64)> = None;
    for (index, option) in decision.options.iter().enumerate() {
        let Choice::Slider { track, stop } = option.choice else {
            continue;
        };
        let target = best_stop(view, seat, &needs, track);
        let from = current.get(track).copied().unwrap_or(0);
        let gain = stop_worth(view, seat, &needs, track, target)
            - stop_worth(view, seat, &needs, track, from);
        let placing = stop == target && decision.find(&Choice::Done).is_none();
        let towards = stop.abs_diff(target) < from.abs_diff(target);
        let worth = if placing { 100.0 } else { gain };
        if (placing || (towards && gain > 0.01)) && best.is_none_or(|(_, top)| worth > top) {
            best = Some((index, worth));
        }
    }
    best.map_or_else(
        || decision.find(&Choice::Done).unwrap_or(0),
        |(index, _)| index,
    )
}

//! The cheapest way between two places of Arkham, monsters counted as
//! detours.

use data::CheckKind;
use engine::{MonsterPlace, Place, View};

use crate::odds::chance;

/// Cost of meeting a monster that cannot be evaded for sure, in steps.
const MONSTER_COST: f64 = 3.0;

/// A way through Arkham.
pub struct Route {
    /// Places to go through, the target last; empty when already there.
    pub steps: Vec<Place>,
    /// Steps plus the danger of the monsters met.
    pub cost: f64,
}

/// The cheapest route from where the investigator stands to `target`.
pub fn route(view: &View<'_, '_>, seat: usize, target: &Place) -> Option<Route> {
    let start = view.state().investigators.get(seat)?.place.clone();
    let mut known: Vec<(Place, f64, Option<usize>)> = vec![(start, 0.0, None)];
    let mut done: Vec<bool> = vec![false];
    loop {
        let next = (0..known.len())
            .filter(|&index| !done[index])
            .min_by(|&a, &b| known[a].1.total_cmp(&known[b].1))?;
        done[next] = true;
        let (place, cost, _) = known[next].clone();
        if &place == target {
            return Some(Route {
                steps: steps_to(&known, next),
                cost,
            });
        }
        for neighbour in view.neighbours(&place) {
            let step = cost + 1.0 + danger(view, seat, &neighbour);
            match known.iter().position(|(seen, _, _)| seen == &neighbour) {
                Some(index) if !done[index] && step < known[index].1 => {
                    known[index] = (neighbour, step, Some(next));
                }
                Some(_) => {}
                None => {
                    known.push((neighbour, step, Some(next)));
                    done.push(false);
                }
            }
        }
    }
}

/// The places from the start to an entry, the start left out.
fn steps_to(known: &[(Place, f64, Option<usize>)], mut index: usize) -> Vec<Place> {
    let mut steps = Vec::new();
    while let Some((place, _, Some(previous))) = known.get(index) {
        steps.push(place.clone());
        index = *previous;
    }
    steps.reverse();
    steps
}

/// Danger of the monsters on a place: the chance of failing to evade each.
pub fn danger(view: &View<'_, '_>, seat: usize, place: &Place) -> f64 {
    let at = match place {
        Place::Location(id) => MonsterPlace::Location(id.clone()),
        Place::Street(id) => MonsterPlace::Street(id.clone()),
        _ => return 0.0,
    };
    let face = view.success_face(seat);
    view.state()
        .monsters
        .iter()
        .filter(|monster| monster.place == at)
        .filter_map(|monster| view.monster_stats(&monster.monster))
        .map(|stats| {
            let dice = view.check_dice(seat, CheckKind::Evade, stats.awareness);
            (1.0 - chance(dice, 1, face)) * crate::knob::knob("MONSTER_COST", MONSTER_COST)
        })
        .sum()
}

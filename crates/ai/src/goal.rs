//! Where an investigator wants to be at the end of the movement.

use data::{Deck, LocationId, Resource};
use engine::{Area, MonsterPlace, Place, View};

use crate::fighting::fight_chance;
use crate::route::{Route, route};
use crate::rumor::rumor_spot;
use crate::value::amount_of;

/// Where Stamina is restored.
const HOSPITAL: &str = "st_marys_hospital";
/// Where Sanity is restored.
const ASYLUM: &str = "arkham_asylum";
/// Shops: location, least money worth the trip, and worth of the trip.
const SHOPS: [(&str, u16, f64); 3] = [
    ("general_store", 4, 2.5),
    ("curiositie_shoppe", 6, 2.5),
    ("ye_olde_magick_shoppe", 6, 1.5),
];
/// Cards held (special cards left out) from which shopping stops.
const ENOUGH_CARDS: usize = 6;
/// Least Stamina and Sanity to go through a gate.
const READY_FOR_GATE: u32 = 3;
/// Price of full care at the Hospital or the Asylum.
const HEALING_PRICE: u32 = 2;
/// Extra cost of a gate another investigator already deals with.
const GATE_TAKEN: f64 = 8.0;
/// Chance of winning from which a monster is worth hunting.
const HUNTING_ODDS: f64 = 0.7;

/// Why an investigator heads somewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reason {
    /// Close the gate just explored.
    Close,
    /// Heal at the Hospital or the Asylum.
    Heal,
    /// Go through a gate.
    Gate,
    /// Pick up clue tokens.
    Clues,
    /// Fight a monster.
    Hunt,
    /// Buy at the store.
    Shop,
    /// Act against the rumor in play.
    Rumor,
    /// Nothing better: stay.
    Stay,
}

/// The place to reach, and the way there.
pub struct Goal {
    /// The place to end the movement on.
    pub target: Place,
    /// The way there from where the investigator stands.
    pub route: Route,
    /// Why.
    pub reason: Reason,
}

/// A place worth going to, with how much it costs to get there minus what
/// it brings (lower is better).
struct Candidate {
    /// The place.
    place: Place,
    /// Worth subtracted from the route cost.
    worth: f64,
    /// Why.
    reason: Reason,
}

/// What an investigator in Arkham aims for this turn: close the gate just
/// explored, heal when low, else the best of going through a gate, picking
/// up clue tokens, hunting a monster and shopping.
pub fn goal(view: &View<'_, '_>, seat: usize) -> Option<Goal> {
    let inv = view.state().investigators.get(seat)?;
    let here = inv.place.clone();
    if !here.in_arkham() {
        return None;
    }
    if inv.explored
        && here
            .location()
            .is_some_and(|id| view.state().gate_at(id).is_some())
    {
        return to(view, seat, &here, Reason::Close);
    }
    if let Some(goal) = healing(view, seat) {
        return Some(goal);
    }
    let mut candidates = gates(view, seat);
    candidates.extend(clues(view, seat));
    candidates.extend(hunts(view, seat));
    if let Some((place, worth)) = rumor_spot(view, seat) {
        candidates.push(Candidate {
            place,
            worth,
            reason: Reason::Rumor,
        });
    }
    candidates.extend(shops(view, seat));
    candidates
        .iter()
        .filter_map(|candidate| {
            let goal = to(view, seat, &candidate.place, candidate.reason)?;
            Some((goal.route.cost - candidate.worth, goal))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, goal)| goal)
        .or_else(|| to(view, seat, &here, Reason::Stay))
}

/// Where an investigator in Arkham means to go this turn, and why (for
/// showing the AI's plans to the player).
///
/// # Returns
///
/// `None` outside Arkham.
pub fn intention(view: &View<'_, '_>, seat: usize) -> Option<(Place, Reason)> {
    goal(view, seat).map(|goal| (goal.target, goal.reason))
}

/// The Hospital or the Asylum when Stamina or Sanity is low. Without the
/// $2 for full care, a point a turn is not worth staying for unless at 1.
fn healing(view: &View<'_, '_>, seat: usize) -> Option<Goal> {
    let can_pay = amount_of(view, seat, Resource::Money) >= HEALING_PRICE;
    let low = if can_pay { 2 } else { 1 };
    for (resource, place) in [(Resource::Stamina, HOSPITAL), (Resource::Sanity, ASYLUM)] {
        let held = amount_of(view, seat, resource);
        let most = u32::from(view.maximum(seat, resource));
        if (held <= low || (can_pay && held * 3 <= most)) && held < most {
            let id = LocationId::new(place);
            if !view.is_closed(&id) {
                return to(view, seat, &Place::Location(id), Reason::Heal);
            }
        }
    }
    None
}

/// The way to one place.
fn to(view: &View<'_, '_>, seat: usize, target: &Place, reason: Reason) -> Option<Goal> {
    let route = route(view, seat, target)?;
    Some(Goal {
        target: target.clone(),
        route,
        reason,
    })
}

/// Clue tokens needed to seal a gate.
pub fn seal_cost(view: &View<'_, '_>) -> u32 {
    if view.state().ancient_one.as_str() == "hastur" {
        8
    } else {
        5
    }
}

/// Open gates, when healthy enough; worth more when they can be sealed,
/// much less when another investigator already deals with them.
fn gates(view: &View<'_, '_>, seat: usize) -> Vec<Candidate> {
    let ready = amount_of(view, seat, Resource::Stamina) >= READY_FOR_GATE
        && amount_of(view, seat, Resource::Sanity) >= READY_FOR_GATE;
    if !ready {
        return Vec::new();
    }
    let can_seal = amount_of(view, seat, Resource::Clues) >= seal_cost(view);
    view.state()
        .gates
        .iter()
        .map(|gate| {
            let sealable = can_seal && view.can_be_sealed(&gate.location);
            let taken = taken_by_others(view, seat, &gate.location);
            Candidate {
                place: Place::Location(gate.location.clone()),
                worth: 7.0 + if sealable { 4.0 } else { 0.0 } - GATE_TAKEN * taken,
                reason: Reason::Gate,
            }
        })
        .collect()
}

/// Other investigators already going through this gate, or standing on it.
fn taken_by_others(view: &View<'_, '_>, seat: usize, location: &LocationId) -> f64 {
    let count = view
        .state()
        .investigators
        .iter()
        .enumerate()
        .filter(|&(other, inv)| other != seat && !inv.eliminated)
        .filter(|(_, inv)| {
            let inside = matches!(inv.place, Place::OtherWorld { .. })
                && inv.entered_gate.as_ref() == Some(location);
            inside || inv.place.location() == Some(location)
        })
        .count();
    f64::from(u32::try_from(count).unwrap_or(0))
}

/// Locations with clue tokens and no monster, while short of a seal.
fn clues(view: &View<'_, '_>, seat: usize) -> Vec<Candidate> {
    if amount_of(view, seat, Resource::Clues) >= seal_cost(view) {
        return Vec::new();
    }
    view.state()
        .clues
        .iter()
        .filter(|(id, count)| **count > 0 && !view.is_closed(id))
        .filter(|(id, _)| {
            !view
                .state()
                .monsters
                .iter()
                .any(|monster| monster.place == MonsterPlace::Location((*id).clone()))
        })
        .map(|(id, count)| Candidate {
            place: Place::Location(id.clone()),
            worth: 1.5 * f64::from(*count),
            reason: Reason::Clues,
        })
        .collect()
}

/// Monsters in Arkham that the investigator would beat, more worth it as
/// the monster limit nears.
fn hunts(view: &View<'_, '_>, seat: usize) -> Vec<Candidate> {
    let state = view.state();
    let in_town = state
        .monsters
        .iter()
        .filter(|monster| monster.place.as_place().is_some())
        .count();
    let limit = state.players() + 3;
    let pressure = if in_town + 1 >= limit { 4.0 } else { 1.0 };
    state
        .monsters
        .iter()
        .filter_map(|monster| {
            let place = monster.place.as_place()?;
            let odds = fight_chance(view, seat, &monster.monster)?;
            (odds >= HUNTING_ODDS).then_some(Candidate {
                place,
                worth: pressure,
                reason: Reason::Hunt,
            })
        })
        .collect()
}

/// Whether the investigator comes back to Arkham this turn, in front of
/// the gate just explored.
pub fn closing_soon(view: &View<'_, '_>, seat: usize) -> bool {
    view.state().investigators.get(seat).is_some_and(|inv| {
        inv.explored
            || matches!(
                inv.place,
                Place::OtherWorld {
                    area: Area::Second,
                    ..
                }
            )
    })
}

/// Open shops the investigator can afford, while short of cards; the
/// General Store only while holding fewer than two weapons.
fn shops(view: &View<'_, '_>, seat: usize) -> Vec<Candidate> {
    let Some(inv) = view.state().investigators.get(seat) else {
        return Vec::new();
    };
    let cards = |keep: &dyn Fn(&data::InvestigatorCard) -> bool| {
        inv.cards
            .iter()
            .filter(|held| {
                view.data()
                    .investigator_cards
                    .card(&held.card)
                    .is_some_and(keep)
            })
            .count()
    };
    if cards(&|card| card.deck != Deck::Special) >= ENOUGH_CARDS {
        return Vec::new();
    }
    let weapons = cards(&|card| card.weapon.is_some());
    SHOPS
        .iter()
        .filter(|(id, money, _)| {
            inv.money >= *money
                && !view.is_closed(&LocationId::new(*id))
                && (*id != "general_store" || weapons < 2)
        })
        .map(|(id, _, worth)| Candidate {
            place: Place::Location(LocationId::new(*id)),
            worth: *worth,
            reason: Reason::Shop,
        })
        .collect()
}

//! Rumors: where to act against the one in play, when the investigator can.

use data::{CardFilter, Deck, Effect, MythosCard, Resource, Rumor, RumorCondition};
use engine::{Place, View};

use crate::fighting::fight_chance;
use crate::value::amount_of;

/// Worth of ending a rumor in the investigators' favour.
pub const RUMOR_PASSED: f64 = 6.0;

/// Where to stand to act against the rumor in play, and how much it is
/// worth, when the investigator can do what it asks.
pub fn rumor_spot(view: &View<'_, '_>, seat: usize) -> Option<(Place, f64)> {
    let active = view.state().rumor.as_ref()?;
    let card = view
        .data()
        .mythos_cards
        .cards
        .iter()
        .find(|card| card.id == active.card)?;
    let rumor = card.rumor.as_ref()?;
    if is_great_ritual(rumor) {
        return great_ritual_spot(view, seat, rumor);
    }
    let action = rumor.action.as_ref()?;
    if !action
        .effects
        .iter()
        .all(|effect| can_do(view, seat, effect))
    {
        return None;
    }
    let place = action.location.as_ref().map_or_else(
        || Place::Street(rumor.activity.clone()),
        |location| Place::Location(location.clone()),
    );
    Some((
        place,
        RUMOR_PASSED + urgency(card, f64::from(active.tokens)),
    ))
}

/// Le Grand Rituel ends when spells are discarded in its streets.
fn is_great_ritual(rumor: &Rumor) -> bool {
    matches!(&rumor.pass.condition, RumorCondition::Special(id) if id.as_str() == "great_ritual_spells_discarded")
}

/// The streets of Le Grand Rituel, when holding enough spells.
fn great_ritual_spot(view: &View<'_, '_>, seat: usize, rumor: &Rumor) -> Option<(Place, f64)> {
    let needed = if view.state().players() >= 5 { 4 } else { 3 };
    (held(view, seat, Deck::Spell) >= needed)
        .then(|| (Place::Street(rumor.activity.clone()), RUMOR_PASSED + 2.0))
}

/// More pressing as tokens pile up on a rumor that fails with many.
fn urgency(card: &MythosCard, tokens: f64) -> f64 {
    match card.rumor.as_ref().map(|rumor| &rumor.fail.condition) {
        Some(RumorCondition::TokensAtLeast(limit)) => 4.0 * tokens / f64::from(*limit),
        _ => 1.0,
    }
}

/// Whether the investigator can pay or do one part of a rumor's action.
fn can_do(view: &View<'_, '_>, seat: usize, effect: &Effect) -> bool {
    let Some(inv) = view.state().investigators.get(seat) else {
        return false;
    };
    match effect {
        Effect::Discard {
            cards: CardFilter::Deck(deck),
            count,
        } => held(view, seat, *deck) >= usize::from(*count),
        Effect::Pay(resource, amount) => amount_of(view, seat, *resource) >= u32::from(*amount),
        Effect::SpendGateTrophies(count) => inv.gate_trophies.len() >= usize::from(*count),
        Effect::Special(id) => match id.as_str() {
            "rumor_clue_removal" => amount_of(view, seat, Resource::Clues) > 0,
            "experiment_fight" => can_fight_experiment(view, seat),
            _ => false,
        },
        _ => true,
    }
}

/// Healthy and likely to beat the rumor's monsters.
fn can_fight_experiment(view: &View<'_, '_>, seat: usize) -> bool {
    let healthy = amount_of(view, seat, Resource::Stamina) >= 4
        && amount_of(view, seat, Resource::Sanity) >= 3;
    let monsters = view
        .state()
        .rumor
        .as_ref()
        .map(|rumor| rumor.monsters.as_slice())
        .unwrap_or_default();
    healthy
        && monsters
            .iter()
            .all(|monster| fight_chance(view, seat, monster).is_some_and(|odds| odds >= 0.6))
}

/// Cards of a deck held.
fn held(view: &View<'_, '_>, seat: usize, deck: Deck) -> usize {
    view.state().investigators.get(seat).map_or(0, |inv| {
        inv.cards
            .iter()
            .filter(|held| {
                view.data()
                    .investigator_cards
                    .card(&held.card)
                    .is_some_and(|card| card.deck == deck)
            })
            .count()
    })
}

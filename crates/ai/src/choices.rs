//! Card and encounter choices, clue tokens, gates and arbitrations.

use data::{CardId, CheckKind, Deck, Resource};
use engine::{CardAim, Choice, Decision, Place, View};

use crate::goal::seal_cost;
use crate::odds::chance;
use crate::route::danger;
use crate::value::{amount_of, deck_value, effects_value, special_card_value};

/// Least worth for a card ability to be used.
const ABILITY_WORTH: f64 = 0.5;
/// Least worth for a card to be bought.
const BUYING_WORTH: f64 = 2.0;
/// Sanity kept when trading it for something.
const SANITY_KEPT: u32 = 3;

/// The option whose effects are worth the most; `None` when no option
/// carries effects.
pub fn best_effects(view: &View<'_, '_>, seat: usize, decision: &Decision) -> Option<usize> {
    if decision
        .options
        .iter()
        .all(|option| option.effects.is_empty())
    {
        return None;
    }
    decision
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| (index, effects_value(view, seat, &option.effects)))
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(index, _)| index)
}

/// « Choisissez » and the other picks of a card text.
pub fn card_choice(view: &View<'_, '_>, seat: usize, decision: &Decision) -> usize {
    if let Some(index) = best_effects(view, seat, decision) {
        return index;
    }
    if decision
        .options
        .iter()
        .any(|option| matches!(option.choice, Choice::Number(_)))
    {
        return number(view, seat, decision);
    }
    decision.find(&Choice::Investigator(seat)).unwrap_or(0)
}

/// A number: Stamina in a Stamina/Sanity split, or Sanity traded away.
fn number(view: &View<'_, '_>, seat: usize, decision: &Decision) -> usize {
    let most = u32::try_from(decision.options.len().saturating_sub(1)).unwrap_or(0);
    let split = decision
        .options
        .iter()
        .any(|option| option.label.contains("Résistance"));
    let wanted = if split {
        let missing = |resource| {
            u32::from(view.maximum(seat, resource)).saturating_sub(amount_of(view, seat, resource))
        };
        missing(Resource::Stamina).min(most)
    } else {
        amount_of(view, seat, Resource::Sanity)
            .saturating_sub(SANITY_KEPT)
            .min(most)
    };
    decision.find(&Choice::Number(wanted)).unwrap_or(0)
}

/// « Vous pouvez » : yes when the effects are worth it; a healer picks
/// the most hurt investigator.
pub fn optional(view: &View<'_, '_>, seat: usize, decision: &Decision) -> usize {
    let yes = decision.find(&Choice::Yes);
    if let Some(index) = yes
        && let Some(option) = decision.options.get(index)
        && !option.effects.is_empty()
    {
        let worth = effects_value(view, seat, &option.effects);
        return if worth > 0.0 {
            index
        } else {
            decision.find(&Choice::Done).unwrap_or(index)
        };
    }
    if let Some(index) = yes {
        return index;
    }
    most_hurt(view, decision).unwrap_or(0)
}

/// The investigator option with the least Stamina plus Sanity.
fn most_hurt(view: &View<'_, '_>, decision: &Decision) -> Option<usize> {
    decision
        .options
        .iter()
        .enumerate()
        .filter_map(|(index, option)| match option.choice {
            Choice::Investigator(other) => {
                let health = amount_of(view, other, Resource::Stamina)
                    + amount_of(view, other, Resource::Sanity);
                Some((index, health))
            }
            _ => None,
        })
        .min_by_key(|&(_, health)| health)
        .map(|(index, _)| index)
}

/// A card ability, when worth it.
pub fn use_ability(view: &View<'_, '_>, seat: usize, decision: &Decision) -> usize {
    if let Some(yes) = decision.find(&Choice::Yes) {
        return yes;
    }
    let done = decision.find(&Choice::Done).unwrap_or(0);
    let Some(index) = best_effects(view, seat, decision) else {
        return done;
    };
    let worth = decision
        .options
        .get(index)
        .map_or(0.0, |option| effects_value(view, seat, &option.effects));
    if worth >= ABILITY_WORTH { index } else { done }
}

/// Worth of a card to the investigator.
pub fn card_worth(view: &View<'_, '_>, card: &CardId) -> f64 {
    if let Some(worth) = special_card_value(card) {
        return worth;
    }
    let Some(printed) = view.data().investigator_cards.card(card) else {
        return 0.0;
    };
    let weapon = printed
        .weapon
        .map_or(0.0, |weapon| f64::from(weapon.bonus.max(0)) * 0.8);
    deck_value(printed.deck).max(weapon)
}

/// Keep, buy, sell or lose a card.
pub fn pick_card(view: &View<'_, '_>, seat: usize, decision: &Decision, aim: CardAim) -> usize {
    let done = decision.find(&Choice::Done);
    let cards = decision
        .options
        .iter()
        .enumerate()
        .filter_map(|(index, option)| match &option.choice {
            Choice::Card(card) => Some((index, card_worth(view, card))),
            _ => None,
        });
    let pick = match aim {
        CardAim::Keep => cards.max_by(|a, b| a.1.total_cmp(&b.1)),
        CardAim::Lose => cards.min_by(|a, b| a.1.total_cmp(&b.1)),
        CardAim::Buy => cards
            .filter(|&(_, worth)| worth >= BUYING_WORTH && !well_armed(view, seat))
            .max_by(|a, b| a.1.total_cmp(&b.1)),
        CardAim::Sell => cards
            .filter(|&(_, worth)| worth < 1.0)
            .min_by(|a, b| a.1.total_cmp(&b.1)),
    };
    pick.map(|(index, _)| index).or(done).unwrap_or(0)
}

/// Holds enough cards already: no more shopping.
fn well_armed(view: &View<'_, '_>, seat: usize) -> bool {
    view.state().investigators.get(seat).is_some_and(|inv| {
        inv.cards
            .iter()
            .filter(|held| {
                view.data()
                    .investigator_cards
                    .card(&held.card)
                    .is_some_and(|card| card.deck != Deck::Special)
            })
            .count()
            >= 6
    })
}

/// Spend a clue token on a failing check: always to close a gate; for a
/// combat, a Horror or an evasion when hurt; otherwise only tokens beyond
/// those kept for a seal.
pub fn spend_clue(
    view: &View<'_, '_>,
    seat: usize,
    decision: &Decision,
    kind: CheckKind,
    closing_gate: bool,
) -> usize {
    let spend = decision.find(&Choice::SpendClue);
    let done = decision.find(&Choice::Done).unwrap_or(0);
    let Some(spend) = spend else {
        return done;
    };
    let clues = amount_of(view, seat, Resource::Clues);
    let stamina = amount_of(view, seat, Resource::Stamina);
    let sanity = amount_of(view, seat, Resource::Sanity);
    let seal = seal_cost(view);
    let worth_it = view.state().battle.is_some()
        || (closing_gate && clues != seal)
        || clues > seal
        || match kind {
            CheckKind::Combat | CheckKind::Evade => {
                f64::from(stamina) <= crate::knob::knob("SPEND_HP", 2.0)
            }
            CheckKind::Horror => f64::from(sanity) <= crate::knob::knob("SPEND_HP", 2.0),
            _ => false,
        };
    if worth_it { spend } else { done }
}

/// The largest number offered (clue tokens against a rumor).
pub fn largest_number(decision: &Decision) -> usize {
    decision
        .options
        .iter()
        .enumerate()
        .filter_map(|(index, option)| match option.choice {
            Choice::Number(count) => Some((index, count)),
            _ => None,
        })
        .max_by_key(|&(_, count)| count)
        .map_or(0, |(index, _)| index)
}

/// In front of a gate: seal it with an Elder Sign if one is held, else
/// try to close it with the better of Lore and Fight.
pub fn close_gate(view: &View<'_, '_>, seat: usize, decision: &Decision, modifier: i32) -> usize {
    if let Some(index) = decision.find(&Choice::ElderSign)
        && amount_of(view, seat, Resource::Stamina) > 1
        && amount_of(view, seat, Resource::Sanity) > 1
    {
        return index;
    }
    let face = view.success_face(seat);
    let try_with = |kind| chance(view.check_dice(seat, kind, modifier), 1, face);
    let kind = if try_with(CheckKind::Lore) >= try_with(CheckKind::Fight) {
        CheckKind::Lore
    } else {
        CheckKind::Fight
    };
    decision
        .find(&Choice::CloseWith(kind))
        .or_else(|| decision.find(&Choice::Yes))
        .unwrap_or(0)
}

/// Back from an Other World: through the gate with the fewest monsters.
pub fn return_gate(view: &View<'_, '_>, seat: usize, decision: &Decision) -> usize {
    decision
        .options
        .iter()
        .enumerate()
        .filter_map(|(index, option)| match &option.choice {
            Choice::Gate(location) => Some((
                index,
                danger(view, seat, &Place::Location(location.clone())),
            )),
            _ => None,
        })
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map_or(0, |(index, _)| index)
}

/// First player's arbitration: the worst for the investigators (R4), so a
/// monster goes for the most hurt one.
pub fn arbitration(view: &View<'_, '_>, decision: &Decision) -> usize {
    most_hurt(view, decision).unwrap_or(0)
}

//! Rough worth of card effects for one investigator: gains count up,
//! losses and dangers count down.

use data::{Amount, CardId, CheckKind, Deck, Effect, Resource, TableRow};
use engine::View;

use crate::odds::{chance, die_chance, exactly};
use crate::rumor::RUMOR_PASSED;

/// Worth of a Stamina or Sanity point when the investigator is low.
const LOW_HEALTH_POINT: f64 = 2.5;
/// Worth of one dollar.
const DOLLAR: f64 = 0.3;
/// Cost of being knocked out or going insane.
const KNOCKED_OUT: f64 = 8.0;

/// Worth of a list of effects, applied in order.
pub fn effects_value(view: &View<'_, '_>, seat: usize, effects: &[Effect]) -> f64 {
    effects
        .iter()
        .map(|effect| effect_value(view, seat, effect))
        .sum()
}

/// Worth of one effect.
fn effect_value(view: &View<'_, '_>, seat: usize, effect: &Effect) -> f64 {
    if let Some(value) = resource_value(view, seat, effect) {
        return value;
    }
    if let Some(value) = branch_value(view, seat, effect) {
        return value;
    }
    if let Some(value) = card_effect_value(view, seat, effect) {
        return value;
    }
    board_value(effect)
}

/// Gains, losses and payments.
fn resource_value(view: &View<'_, '_>, seat: usize, effect: &Effect) -> Option<f64> {
    Some(match effect {
        Effect::Gain(resource, amount) => gain(view, seat, *resource, f64::from(*amount)),
        Effect::GainRolled(resource) => gain(view, seat, *resource, 3.5),
        Effect::GainAmount(resource, amount) => {
            gain(view, seat, *resource, amount_guess(view, seat, *amount))
        }
        Effect::GainSplit(amount) => {
            let points = amount_guess(view, seat, *amount);
            gain(view, seat, Resource::Stamina, points / 2.0)
                + gain(view, seat, Resource::Sanity, points / 2.0)
        }
        Effect::Lose(resource, amount) | Effect::Pay(resource, amount) => {
            -loss(view, seat, *resource, f64::from(*amount))
        }
        Effect::LoseRolled(resource) => -loss(view, seat, *resource, 3.5),
        Effect::LoseAmount(resources, amount) => {
            let amount = amount_guess(view, seat, *amount);
            -resources
                .iter()
                .map(|resource| loss(view, seat, *resource, amount))
                .sum::<f64>()
        }
        Effect::LoseAll(resource) => {
            let held = f64::from(amount_of(view, seat, *resource));
            -loss(view, seat, *resource, held)
        }
        Effect::RestoreToMax(resource) => {
            let missing = f64::from(view.maximum(seat, *resource))
                - f64::from(amount_of(view, seat, *resource));
            gain(view, seat, *resource, missing.max(0.0))
        }
        Effect::LowerMax(_, amount) => -2.0 * f64::from(*amount),
        Effect::RollPerPoint { resource, .. } => {
            let points = f64::from(amount_of(view, seat, *resource));
            -loss(view, seat, *resource, points * 2.0 / 3.0)
        }
        _ => return None,
    })
}

/// Checks, rolls, choices and conditions: the expected worth of the branches.
fn branch_value(view: &View<'_, '_>, seat: usize, effect: &Effect) -> Option<f64> {
    let face = view.success_face(seat);
    Some(match effect {
        Effect::Check {
            skill,
            modifier,
            successes,
            pass,
            fail,
            ..
        } => {
            let dice = view.check_dice(seat, *skill, i32::from(*modifier));
            let p = chance(dice, u32::from(*successes), face);
            p * effects_value(view, seat, pass) + (1.0 - p) * effects_value(view, seat, fail)
        }
        Effect::CheckTable {
            skill,
            modifier,
            rows,
        } => table_value(view, seat, *skill, *modifier, rows),
        Effect::RollDie { at_most, effects } => {
            f64::from(*at_most).min(6.0) / 6.0 * effects_value(view, seat, effects)
        }
        Effect::RollEach {
            dice,
            success,
            failure,
        } => {
            let p = die_chance(face);
            f64::from(*dice)
                * (p * effects_value(view, seat, success)
                    + (1.0 - p) * effects_value(view, seat, failure))
        }
        Effect::Choice(options) => options
            .iter()
            .map(|option| effects_value(view, seat, &option.effects))
            .fold(f64::MIN, f64::max)
            .max(-50.0),
        Effect::Optional(option) => effects_value(view, seat, &option.effects).max(0.0),
        Effect::If {
            then, otherwise, ..
        } => (effects_value(view, seat, then) + effects_value(view, seat, otherwise)) / 2.0,
        Effect::EachInvestigator { effects, .. }
        | Effect::OneInvestigatorHere(effects)
        | Effect::AnyInvestigator(effects) => effects_value(view, seat, effects),
        _ => return None,
    })
}

/// Expected worth of a check table: each row weighted by the chance of
/// rolling its number of successes.
fn table_value(
    view: &View<'_, '_>,
    seat: usize,
    skill: CheckKind,
    modifier: i8,
    rows: &[TableRow],
) -> f64 {
    let dice = view.check_dice(seat, skill, i32::from(modifier));
    let p = die_chance(view.success_face(seat));
    (0..=dice)
        .map(|hits| {
            let row = rows
                .iter()
                .filter(|row| u32::from(row.from) <= hits)
                .max_by_key(|row| row.from);
            let worth = row.map_or(0.0, |row| effects_value(view, seat, &row.effects));
            exactly(dice, hits, p) * worth
        })
        .sum()
}

/// Cards drawn, taken, bought or lost.
fn card_effect_value(view: &View<'_, '_>, seat: usize, effect: &Effect) -> Option<f64> {
    Some(match effect {
        Effect::Draw { deck, count, keep } => f64::from(keep.unwrap_or(*count)) * deck_value(*deck),
        Effect::TakeCard { card, otherwise } => {
            special_card_value(card).unwrap_or(2.0) + effects_value(view, seat, otherwise) * 0.2
        }
        Effect::TakeFirst { deck, .. } => deck_value(*deck),
        Effect::SearchDeck(deck) => deck_value(*deck) + 0.5,
        Effect::Shop(_) | Effect::Buy { .. } | Effect::BuyAny(_) => 0.3,
        Effect::Discard { count, .. } => -1.5 * f64::from(*count),
        Effect::DiscardAll(_) => -1.5 * held_items(view, seat),
        Effect::DiscardHalf { .. } => -0.75 * held_items(view, seat),
        Effect::DiscardCard(_) | Effect::DiscardThis | Effect::ReturnThisToBox => -1.0,
        Effect::TrophyFromCup | Effect::TrophyFromBoard => 1.5,
        Effect::SpendMonsterTrophies(toughness) => -0.4 * f64::from(*toughness),
        Effect::SpendGateTrophies(count) => -2.0 * f64::from(*count),
        _ => return None,
    })
}

/// Movement, statuses and the state of the board.
fn board_value(effect: &Effect) -> f64 {
    match effect {
        Effect::Devoured => -50.0,
        Effect::LostInTimeAndSpace => -6.0,
        Effect::GateAndMonsterAppear => -4.0,
        Effect::GateAppears | Effect::AddDoom | Effect::RemoveAllElderSigns => -3.0,
        Effect::Arrested | Effect::LoseNextTurn => -2.0,
        Effect::MonsterAppears => -1.5,
        Effect::Delayed | Effect::VisitOtherWorld(_) => -1.0,
        Effect::RaiseTerror(amount) => -f64::from(*amount),
        Effect::TerrorTo(_) => -2.0,
        Effect::MonstersInStreet { count, .. } => -f64::from(*count),
        Effect::CloseEntryGate | Effect::CloseGateOfChoice => 5.0,
        Effect::RumorPasses => RUMOR_PASSED,
        Effect::Special(id) if id.as_str() == "rumor_clue_removal" => 1.0,
        Effect::Special(id) if id.as_str() == "experiment_fight" => 2.0,
        Effect::RemoveDoom => 3.0,
        Effect::ReturnMonsters { .. } => 1.0,
        Effect::CancelLoss | Effect::PassCheck => 2.0,
        Effect::Reroll | Effect::NoCombatDamage => 1.5,
        Effect::ReduceLoss(amount) => f64::from(*amount),
        Effect::RerollOneDie
        | Effect::BonusUntilEndOfTurn(_)
        | Effect::EncounterHere { .. }
        | Effect::GainMovementPoints(_) => 0.4,
        _ => 0.0,
    }
}

/// Worth of a card of a deck, on average.
pub fn deck_value(deck: Deck) -> f64 {
    match deck {
        Deck::Skill | Deck::Ally => 3.0,
        Deck::UniqueItem => 2.5,
        Deck::Spell => 2.0,
        Deck::CommonItem => 1.5,
        Deck::Special => 1.0,
    }
}

/// Worth of the special cards that are not plain gains.
pub fn special_card_value(card: &CardId) -> Option<f64> {
    Some(match card.as_str() {
        "blessing" => 3.0,
        "curse" => -3.0,
        "bank_loan" => -1.0,
        "retainer" | "deputy" => 2.5,
        "silver_twilight_membership" => 0.5,
        _ => return None,
    })
}

/// Worth of gaining points of a resource.
fn gain(view: &View<'_, '_>, seat: usize, resource: Resource, amount: f64) -> f64 {
    match resource {
        Resource::Money => amount * DOLLAR,
        Resource::Clues => amount,
        Resource::Sanity | Resource::Stamina => {
            let room = f64::from(view.maximum(seat, resource))
                - f64::from(amount_of(view, seat, resource));
            let points = amount.min(room.max(0.0));
            points * health_point(view, seat, resource)
        }
    }
}

/// Cost of losing points of a resource; dropping to 0 costs much more.
fn loss(view: &View<'_, '_>, seat: usize, resource: Resource, amount: f64) -> f64 {
    let held = f64::from(amount_of(view, seat, resource));
    match resource {
        Resource::Money => amount.min(held) * DOLLAR,
        Resource::Clues => amount.min(held),
        Resource::Sanity | Resource::Stamina => {
            let fall = if amount >= held { KNOCKED_OUT } else { 0.0 };
            amount.min(held) * health_point(view, seat, resource) + fall
        }
    }
}

/// Worth of one Stamina or Sanity point, higher when low.
fn health_point(view: &View<'_, '_>, seat: usize, resource: Resource) -> f64 {
    if amount_of(view, seat, resource) <= 2 {
        LOW_HEALTH_POINT
    } else {
        1.0
    }
}

/// What the investigator holds of a resource.
pub fn amount_of(view: &View<'_, '_>, seat: usize, resource: Resource) -> u32 {
    let Some(inv) = view.state().investigators.get(seat) else {
        return 0;
    };
    match resource {
        Resource::Sanity => u32::from(inv.sanity),
        Resource::Stamina => u32::from(inv.stamina),
        Resource::Money => u32::from(inv.money),
        Resource::Clues => u32::from(inv.clues),
    }
}

/// A guess of an amount worked out at play time.
fn amount_guess(view: &View<'_, '_>, seat: usize, amount: Amount) -> f64 {
    match amount {
        Amount::Fixed(value) => f64::from(value),
        Amount::X => 3.5,
        Amount::XMinus(less) => (3.5 - f64::from(less)).max(0.0),
        Amount::Successes(each) => f64::from(each),
        Amount::HalfUp(resource) => f64::from(amount_of(view, seat, resource).div_ceil(2)),
    }
}

/// Items held, as a number.
fn held_items(view: &View<'_, '_>, seat: usize) -> f64 {
    let Some(inv) = view.state().investigators.get(seat) else {
        return 0.0;
    };
    let items = inv
        .cards
        .iter()
        .filter(|held| {
            view.data()
                .investigator_cards
                .card(&held.card)
                .is_some_and(|card| matches!(card.deck, Deck::CommonItem | Deck::UniqueItem))
        })
        .count();
    f64::from(u32::try_from(items).unwrap_or(0))
}

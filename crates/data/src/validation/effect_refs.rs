//! Checks on what card effects refer to: cards, streets, monsters.

use crate::cards::{Ability, CheckKind, Condition, Effect, Ongoing};
use crate::game_data::GameData;
use crate::validation::issue::Issue;

/// Checks every effect of a list, nested ones included.
pub(crate) fn check_effects(
    context: &str,
    effects: &[Effect],
    data: &GameData,
    issues: &mut Vec<Issue>,
) {
    for effect in effects {
        check_effect(context, effect, data, issues);
        for nested in nested_effects(effect) {
            check_effects(context, nested, data, issues);
        }
    }
}

/// Checks lasting effects, their triggers included.
pub(crate) fn check_ongoing(
    context: &str,
    ongoing: &[Ongoing],
    data: &GameData,
    issues: &mut Vec<Issue>,
) {
    for item in ongoing {
        match item {
            Ongoing::SkillBonus { skill, .. } | Ongoing::ExtraClueDie(skill) => {
                check_is_skill(context, *skill, issues);
            }
            Ongoing::MonsterToughness { monsters, .. } => {
                for monster in monsters {
                    check_monster(context, monster.as_str(), data, issues);
                }
            }
            Ongoing::MonsterBanned(monster) => {
                check_monster(context, monster.as_str(), data, issues);
            }
            Ongoing::Trigger { only, effects, .. } => {
                if let Some(condition) = only {
                    check_condition(context, condition, data, issues);
                }
                check_effects(context, effects, data, issues);
            }
            Ongoing::CheckBonus { .. }
            | Ongoing::MaxBonus(..)
            | Ongoing::SuccessOn(_)
            | Ongoing::ClueSubstitute
            | Ongoing::MovementPoints(_)
            | Ongoing::SpellSanityCost(_) => {}
        }
    }
}

/// Checks the abilities of a card.
pub(crate) fn check_abilities(
    context: &str,
    abilities: &[Ability],
    data: &GameData,
    issues: &mut Vec<Issue>,
) {
    for ability in abilities {
        if let Some(condition) = &ability.only {
            check_condition(context, condition, data, issues);
        }
        check_effects(context, &ability.effects, data, issues);
    }
}

/// Checks the ids a condition refers to.
pub(crate) fn check_condition(
    context: &str,
    condition: &Condition,
    data: &GameData,
    issues: &mut Vec<Issue>,
) {
    if let Condition::HasCard(card) = condition
        && data.investigator_cards.card(card).is_none()
    {
        issues.push(unknown(context, "carte", card.as_str()));
    }
}

/// Checks one effect, without its nested effects.
fn check_effect(context: &str, effect: &Effect, data: &GameData, issues: &mut Vec<Issue>) {
    match effect {
        Effect::TakeCard { card, .. } if data.investigator_cards.card(card).is_none() => {
            issues.push(unknown(context, "carte", card.as_str()));
        }
        Effect::MonstersInStreet { street, .. } if data.board.street(street).is_none() => {
            issues.push(unknown(context, "rue", street.as_str()));
        }
        Effect::Check { successes: 0, .. } => issues.push(Issue::Mismatch {
            context: context.to_owned(),
            reason: "un test demande au moins 1 succès",
        }),
        Effect::Draw {
            count,
            keep: Some(keep),
            ..
        } if keep > count => issues.push(Issue::Mismatch {
            context: context.to_owned(),
            reason: "on garde plus de cartes qu'on n'en pioche",
        }),
        Effect::If { condition, .. } => check_condition(context, condition, data, issues),
        Effect::EachInvestigator {
            only: Some(condition),
            ..
        } => check_condition(context, condition, data, issues),
        _ => {}
    }
}

/// The effect lists nested inside an effect.
fn nested_effects(effect: &Effect) -> Vec<&[Effect]> {
    match effect {
        Effect::Check { pass, fail, .. } => vec![pass, fail],
        Effect::RollDie { effects, .. } | Effect::EachInvestigator { effects, .. } => {
            vec![effects]
        }
        Effect::TakeCard { otherwise, .. } => vec![otherwise],
        Effect::Choice(options) => options
            .iter()
            .map(|option| option.effects.as_slice())
            .collect(),
        Effect::Optional(option) => vec![&option.effects],
        Effect::If {
            then, otherwise, ..
        } => vec![then, otherwise],
        _ => Vec::new(),
    }
}

/// Reports a skill bonus given to a derived check.
fn check_is_skill(context: &str, skill: CheckKind, issues: &mut Vec<Issue>) {
    if !skill.is_skill() {
        issues.push(Issue::Mismatch {
            context: context.to_owned(),
            reason: "un bonus de compétence porte sur l'une des six compétences",
        });
    }
}

/// Reports an unknown monster id.
fn check_monster(context: &str, id: &str, data: &GameData, issues: &mut Vec<Issue>) {
    if !data
        .monsters
        .monsters
        .iter()
        .any(|monster| monster.id.as_str() == id)
    {
        issues.push(unknown(context, "monstre", id));
    }
}

/// Builds an [`Issue::UnknownReference`].
fn unknown(context: &str, family: &'static str, id: &str) -> Issue {
    Issue::UnknownReference {
        context: context.to_owned(),
        family,
        id: id.to_owned(),
    }
}

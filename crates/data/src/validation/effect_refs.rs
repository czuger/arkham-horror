//! Checks on what card effects refer to: cards, places, monsters.

use crate::cards::{Ability, CheckKind, Condition, Effect, MonsterArea, Ongoing};
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
    let refs = Refs { context, data };
    for item in ongoing {
        match item {
            Ongoing::SkillBonus { skill, .. } | Ongoing::ExtraClueDie(skill) => {
                check_is_skill(context, *skill, issues);
            }
            Ongoing::MonsterToughness { monsters, .. } => {
                for monster in monsters {
                    refs.monster(monster.as_str(), issues);
                }
            }
            Ongoing::MonsterBanned(monster) => refs.monster(monster.as_str(), issues),
            Ongoing::OnDefeat { monster, effects } => {
                refs.monster(monster.as_str(), issues);
                check_effects(context, effects, data, issues);
            }
            Ongoing::GateCloseModifier { world, .. } => refs.world(world.as_str(), issues),
            Ongoing::LocationClosed(location) => refs.location(location.as_str(), issues),
            Ongoing::StreetClosed(street) => refs.street(street.as_str(), issues),
            Ongoing::Trigger { only, effects, .. } => {
                if let Some(condition) = only {
                    check_condition(context, condition, data, issues);
                }
                check_effects(context, effects, data, issues);
            }
            _ => {}
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
    let refs = Refs { context, data };
    match condition {
        Condition::HasCard(card) => refs.card(card.as_str(), issues),
        Condition::InStreet(street) => refs.street(street.as_str(), issues),
        Condition::AtLocation(location) => refs.location(location.as_str(), issues),
        Condition::Not(inner) => check_condition(context, inner, data, issues),
        _ => {}
    }
}

/// Checks one effect, without its nested effects.
fn check_effect(context: &str, effect: &Effect, data: &GameData, issues: &mut Vec<Issue>) {
    let refs = Refs { context, data };
    match effect {
        Effect::TakeCard { card, .. } | Effect::DiscardCard(card) => {
            refs.card(card.as_str(), issues);
        }
        Effect::MonstersInStreet { street, .. } => refs.street(street.as_str(), issues),
        Effect::MoveTo(location) => refs.location(location.as_str(), issues),
        Effect::VisitOtherWorld(world) => refs.world(world.as_str(), issues),
        Effect::ReturnMonsters { area, only, .. } => {
            if let MonsterArea::District(street) = area {
                refs.street(street.as_str(), issues);
            }
            for monster in only {
                refs.monster(monster.as_str(), issues);
            }
        }
        Effect::Check { .. } | Effect::CheckTable { .. } => {
            check_dice_effect(context, effect, data, issues)
        }
        Effect::Draw {
            count,
            keep: Some(keep),
            ..
        } if keep > count => issues.push(mismatch(
            context,
            "on garde plus de cartes qu'on n'en pioche",
        )),
        Effect::If { condition, .. }
        | Effect::EachInvestigator {
            only: Some(condition),
            ..
        } => check_condition(context, condition, data, issues),
        _ => {}
    }
}

/// Checks a check or a check table.
fn check_dice_effect(context: &str, effect: &Effect, data: &GameData, issues: &mut Vec<Issue>) {
    let refs = Refs { context, data };
    match effect {
        Effect::Check {
            successes,
            or_discard,
            ..
        } => {
            if *successes == 0 {
                issues.push(mismatch(context, "un test demande au moins 1 succès"));
            }
            if let Some(card) = or_discard {
                refs.card(card.as_str(), issues);
            }
        }
        Effect::CheckTable { rows, .. }
            if rows.first().is_none_or(|row| row.from != 0)
                || rows.windows(2).any(|pair| pair[0].from >= pair[1].from) =>
        {
            issues.push(mismatch(
                context,
                "une table commence à 0 succès, par lignes croissantes",
            ));
        }
        _ => {}
    }
}

/// The effect lists nested inside an effect.
fn nested_effects(effect: &Effect) -> Vec<&[Effect]> {
    match effect {
        Effect::Check { pass, fail, .. } => vec![pass, fail],
        Effect::CheckTable { rows, .. } => rows.iter().map(|row| row.effects.as_slice()).collect(),
        Effect::RollEach {
            success, failure, ..
        } => vec![success, failure],
        Effect::RollDie { effects, .. }
        | Effect::EachInvestigator { effects, .. }
        | Effect::OneInvestigatorHere(effects)
        | Effect::AnyInvestigator(effects) => vec![effects],
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
        issues.push(mismatch(
            context,
            "un bonus de compétence porte sur l'une des six compétences",
        ));
    }
}

/// Looks ids up in the game data and reports the unknown ones.
struct Refs<'a> {
    context: &'a str,
    data: &'a GameData,
}

impl Refs<'_> {
    /// An investigator card.
    fn card(&self, id: &str, issues: &mut Vec<Issue>) {
        if !self
            .data
            .investigator_cards
            .cards
            .iter()
            .any(|card| card.id.as_str() == id)
        {
            issues.push(unknown(self.context, "carte", id));
        }
    }

    /// A monster.
    fn monster(&self, id: &str, issues: &mut Vec<Issue>) {
        if !self
            .data
            .monsters
            .monsters
            .iter()
            .any(|monster| monster.id.as_str() == id)
        {
            issues.push(unknown(self.context, "monstre", id));
        }
    }

    /// A street area.
    fn street(&self, id: &str, issues: &mut Vec<Issue>) {
        if !self
            .data
            .board
            .streets
            .iter()
            .any(|street| street.id.as_str() == id)
        {
            issues.push(unknown(self.context, "rue", id));
        }
    }

    /// A location.
    fn location(&self, id: &str, issues: &mut Vec<Issue>) {
        if !self
            .data
            .board
            .locations
            .iter()
            .any(|location| location.id.as_str() == id)
        {
            issues.push(unknown(self.context, "lieu", id));
        }
    }

    /// An Other World.
    fn world(&self, id: &str, issues: &mut Vec<Issue>) {
        if !self
            .data
            .board
            .other_worlds
            .iter()
            .any(|world| world.id.as_str() == id)
        {
            issues.push(unknown(self.context, "Autre Monde", id));
        }
    }
}

/// Builds an [`Issue::Mismatch`].
fn mismatch(context: &str, reason: &'static str) -> Issue {
    Issue::Mismatch {
        context: context.to_owned(),
        reason,
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

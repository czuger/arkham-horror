//! Derived values: skills, maximums, lasting bonuses.

use data::{CardId, CheckKind, MonsterTrait, Ongoing, Resource, Scope};

use crate::game::Game;
use crate::halt::Flow;

/// The skill a check is made with: special checks use their parent skill.
pub(crate) fn base_skill(kind: CheckKind) -> CheckKind {
    match kind {
        CheckKind::Evade => CheckKind::Sneak,
        CheckKind::Combat => CheckKind::Fight,
        CheckKind::Horror => CheckKind::Will,
        CheckKind::Spell => CheckKind::Lore,
        skill => skill,
    }
}

/// French name of a check.
pub(crate) fn check_name(kind: CheckKind) -> &'static str {
    match kind {
        CheckKind::Speed => "Vitesse",
        CheckKind::Sneak => "Discrétion",
        CheckKind::Fight => "Vigueur",
        CheckKind::Will => "Volonté",
        CheckKind::Lore => "Savoir",
        CheckKind::Luck => "Chance",
        CheckKind::Horror => "Horreur",
        CheckKind::Combat => "Combat",
        CheckKind::Evade => "Évasion",
        CheckKind::Spell => "Sort",
    }
}

/// « de Vitesse », « d'Horreur »…: the check name after « test ».
pub(crate) fn of_check(kind: CheckKind) -> String {
    let name = check_name(kind);
    if name.starts_with(['H', 'É']) {
        format!("d'{name}")
    } else {
        format!("de {name}")
    }
}

/// French name of a resource.
pub(crate) fn resource_name(resource: Resource) -> &'static str {
    match resource {
        Resource::Sanity => "Santé Mentale",
        Resource::Stamina => "Résistance",
        Resource::Money => "$",
        Resource::Clues => "Indice(s)",
    }
}

/// A lasting effect and the card it comes from (`None` for Mythos cards).
pub(crate) type Source<'a> = (Option<CardId>, &'a Ongoing);

impl<'a> Game<'a> {
    /// Lasting effects that apply to an investigator: held cards, then the
    /// Environment and Rumor in play.
    pub fn ongoing_for(&self, seat: usize) -> Vec<Source<'a>> {
        let mut found = Vec::new();
        for held in &self.inv(seat).cards {
            if let Ok(card) = self.card(&held.card) {
                found.extend(
                    card.ongoing
                        .iter()
                        .map(|effect| (Some(held.card.clone()), effect)),
                );
            }
        }
        found.extend(
            self.mythos_ongoing()
                .into_iter()
                .map(|effect| (None, effect)),
        );
        found
    }

    /// Whether a lasting effect held by the investigator, or in play, matches.
    pub fn has_ongoing(&self, seat: usize, test: impl Fn(&Ongoing) -> bool) -> bool {
        self.ongoing_for(seat)
            .into_iter()
            .any(|(_, effect)| test(effect))
    }

    /// Whether the investigator holds a card that ignores this monster ability.
    pub fn ignores(&self, seat: usize, ability: MonsterTrait) -> bool {
        self.has_ongoing(seat, |effect| *effect == Ongoing::Ignore(ability))
    }

    /// Whether a lasting effect of the Mythos cards in play matches.
    pub fn mythos_has(&self, test: impl Fn(&Ongoing) -> bool) -> bool {
        self.mythos_ongoing().into_iter().any(test)
    }

    /// Lasting effects of the Environment and Rumor in play.
    pub fn mythos_ongoing(&self) -> Vec<&'a Ongoing> {
        let data = self.data;
        let in_play = [
            self.state.environment.as_ref(),
            self.state.rumor.as_ref().map(|rumor| &rumor.card),
        ];
        in_play
            .into_iter()
            .flatten()
            .chain(self.state.lingering.iter().map(|(card, _)| card))
            .filter_map(|id| data.mythos_cards.cards.iter().find(|card| &card.id == id))
            .flat_map(|card| card.ongoing.iter())
            .collect()
    }

    /// Value of a skill on the sheet at the current slider stop.
    fn printed_skill(&self, seat: usize, skill: CheckKind) -> Flow<i32> {
        let sheet = self.sheet(seat)?;
        let sliders = self.inv(seat).sliders;
        let (track, values) = match skill {
            CheckKind::Speed => (0, &sheet.skills.speed),
            CheckKind::Sneak => (0, &sheet.skills.sneak),
            CheckKind::Fight => (1, &sheet.skills.fight),
            CheckKind::Will => (1, &sheet.skills.will),
            CheckKind::Lore => (2, &sheet.skills.lore),
            _ => (2, &sheet.skills.luck),
        };
        Ok(values
            .get(usize::from(sliders[track]))
            .map_or(0, |&value| i32::from(value)))
    }

    /// Skill value with lasting skill bonuses.
    pub fn skill(&self, seat: usize, kind: CheckKind) -> Flow<i32> {
        let skill = base_skill(kind);
        let mut value = self.printed_skill(seat, skill)?;
        for (_, effect) in self.ongoing_for(seat) {
            if let Ongoing::SkillBonus {
                skill: bonus_skill,
                amount,
            } = effect
                && *bonus_skill == skill
            {
                value += i32::from(*amount);
            }
        }
        Ok(value)
    }

    /// Dice rolled for a check before clue tokens.
    pub fn check_dice(&self, seat: usize, kind: CheckKind, modifier: i32) -> Flow<i32> {
        let in_arkham = self.inv(seat).place.in_arkham();
        let mut dice = self.skill(seat, kind)? + modifier;
        for (_, effect) in self.ongoing_for(seat) {
            if let Ongoing::CheckBonus {
                check,
                amount,
                scope,
            } = effect
            {
                let applies = *check == kind || *check == base_skill(kind);
                let in_scope = *scope == Scope::Everywhere || in_arkham;
                if applies && in_scope {
                    dice += i32::from(*amount);
                }
            }
        }
        Ok(dice)
    }

    /// Lowest die face that counts as a success (5, or 4 blessed, 6 cursed).
    pub fn success_face(&self, seat: usize) -> u8 {
        self.ongoing_for(seat)
            .into_iter()
            .find_map(|(_, effect)| match effect {
                Ongoing::SuccessOn(face) => Some(*face),
                _ => None,
            })
            .unwrap_or(5)
    }

    /// Dice gained per clue token spent on a check.
    pub fn dice_per_clue(&self, seat: usize, kind: CheckKind) -> u32 {
        let skill = base_skill(kind);
        let extra = self
            .ongoing_for(seat)
            .into_iter()
            .filter(|(_, effect)| matches!(effect, Ongoing::ExtraClueDie(bonus) if *bonus == skill))
            .count();
        1 + u32::try_from(extra).unwrap_or(0) + self.sheet_clue_dice(seat)
    }

    /// Current maximum of Sanity or Stamina (never below 0).
    pub fn maximum(&self, seat: usize, resource: Resource) -> Flow<u8> {
        let sheet = self.sheet(seat)?;
        let inv = self.inv(seat);
        let (printed, change) = match resource {
            Resource::Sanity => (sheet.sanity, inv.max_sanity_change),
            _ => (sheet.stamina, inv.max_stamina_change),
        };
        let mut value = i32::from(printed) + i32::from(change) - self.power_max_reduction();
        for (_, effect) in self.ongoing_for(seat) {
            if let Ongoing::MaxBonus(bonus_resource, amount) = effect
                && *bonus_resource == resource
            {
                value += i32::from(*amount);
            }
        }
        Ok(u8::try_from(value.max(0)).unwrap_or(u8::MAX))
    }

    /// Movement points for the Movement phase.
    pub fn movement_points(&self, seat: usize) -> Flow<u32> {
        let mut points = self.skill(seat, data::CheckKind::Speed)?;
        for (_, effect) in self.ongoing_for(seat) {
            if let Ongoing::MovementPoints(amount) = effect {
                points += i32::from(*amount);
            }
        }
        Ok(u32::try_from(points.max(0)).unwrap_or(0))
    }
}

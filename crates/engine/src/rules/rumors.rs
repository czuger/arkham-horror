//! The Rumor in play: coming in, its progress, the investigators' action
//! against it, and its two endings.

use data::{CardFilter, CardId, Deck, MythosCard, Resource, Rumor, RumorCondition, StreetId};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::combat::Faced;
use crate::state::{ActiveRumor, MonsterPlace, Place};

/// Le Grand Rituel: discard spells in the streets to stop it.
const SPELLS_DISCARDED: &str = "great_ritual_spells_discarded";

impl<'a> Game<'a> {
    /// A Rumor comes into play, unless one already is.
    pub fn start_rumor(&mut self, card: &'a MythosCard) -> Flow<()> {
        if self.state.rumor.is_some() {
            self.log("Une Rumeur est déjà en jeu : celle-ci est ignorée.");
            self.state.mythos_deck.push(card.id.clone());
            return Ok(());
        }
        self.state.rumor = Some(ActiveRumor {
            card: card.id.clone(),
            tokens: 0,
            fresh: true,
            monsters: Vec::new(),
        });
        let first = self.state.first_player;
        if let Some(rumor) = &card.rumor {
            self.apply_effects(first, &rumor.setup, Some(&card.id))?;
        }
        self.apply_effects(first, &card.effects, None)
    }

    /// The Rumor in play and its text.
    fn active_rumor(&self) -> Flow<Option<(ActiveRumor, &'a MythosCard, &'a Rumor)>> {
        let Some(active) = self.state.rumor.clone() else {
            return Ok(None);
        };
        let card = self.mythos_card(&active.card)?;
        Ok(card.rumor.as_ref().map(|rumor| (active, card, rumor)))
    }

    /// End of the Mythos phase: the Rumor's lasting effect (from the turn
    /// after it came in), then its endings.
    pub fn rumor_progress(&mut self) -> Flow<()> {
        let Some((active, _, rumor)) = self.active_rumor()? else {
            return Ok(());
        };
        if active.fresh {
            if let Some(active) = &mut self.state.rumor {
                active.fresh = false;
            }
        } else {
            let first = self.state.first_player;
            self.apply_effects(first, &rumor.each_mythos, Some(&active.card))?;
        }
        self.rumor_check(true)
    }

    /// Ends the Rumor when its success or failure condition holds.
    pub fn rumor_check(&mut self, end_of_mythos: bool) -> Flow<()> {
        let Some((active, _, rumor)) = self.active_rumor()? else {
            return Ok(());
        };
        let first = self.state.first_player;
        if self.condition_met(&active, &rumor.pass.condition, end_of_mythos) {
            return self.end_rumor(first, true);
        }
        if self.condition_met(&active, &rumor.fail.condition, end_of_mythos) {
            return self.end_rumor(first, false);
        }
        Ok(())
    }

    /// Whether a Rumor's condition holds now.
    fn condition_met(&self, active: &ActiveRumor, condition: &RumorCondition, end: bool) -> bool {
        let count = usize::from(active.tokens) + active.monsters.len();
        match condition {
            RumorCondition::TokensAtLeast(needed) => count >= usize::from(*needed),
            RumorCondition::NoTokens => count == 0,
            RumorCondition::TerrorAtLeast(level) => self.state.terror >= *level,
            RumorCondition::DeckEmpty(deck) => end && self.state.decks.get(*deck).is_empty(),
            RumorCondition::Action
            | RumorCondition::AncientOneAwakens
            | RumorCondition::Special(_) => false,
        }
    }

    /// The Rumor ends; the card goes back to the box (card text) and its
    /// leftover monsters to the cup.
    pub fn end_rumor(&mut self, seat: usize, passed: bool) -> Flow<()> {
        let Some((active, card, rumor)) = self.active_rumor()? else {
            return Ok(());
        };
        self.state.rumor = None;
        let (verdict, end) = if passed {
            ("est déjouée", &rumor.pass)
        } else {
            ("se réalise", &rumor.fail)
        };
        self.log(format!("La Rumeur « {} » {verdict}.", card.name));
        self.scratch.rumor_monsters = active.monsters;
        self.apply_effects(seat, &end.effects, None)?;
        let left = std::mem::take(&mut self.scratch.rumor_monsters);
        self.state.monster_cup.extend(left);
        Ok(())
    }

    /// An action solved the Rumor.
    pub fn rumor_passes(&mut self, seat: usize) -> Flow<()> {
        self.end_rumor(seat, true)
    }

    /// Monsters from the cup go onto the Rumor card.
    pub fn add_monsters_to_rumor(&mut self, count: u8) -> Flow<()> {
        for _ in 0..count {
            let monster = self.draw_monster()?;
            if let Some(rumor) = &mut self.state.rumor {
                rumor.monsters.push(monster);
            }
        }
        let held = self
            .state
            .rumor
            .as_ref()
            .map_or(0, |rumor| rumor.monsters.len());
        self.log(format!("La Rumeur porte {held} monstre(s)."));
        Ok(())
    }

    /// During the Arkham encounters, in the Rumor's streets (or on its
    /// location), an investigator may act against it. Whether an action
    /// was taken on a location, which then replaces the encounter.
    pub fn rumor_action(&mut self, seat: usize) -> Flow<bool> {
        let Some((active, card, rumor)) = self.active_rumor()? else {
            return Ok(false);
        };
        if rumor.pass.condition == RumorCondition::Special(data::SpecialId::new(SPELLS_DISCARDED)) {
            self.great_ritual(seat, card, rumor)?;
            return Ok(false);
        }
        let Some(action) = &rumor.action else {
            return Ok(false);
        };
        let place = match &action.location {
            Some(location) => Place::Location(location.clone()),
            None => Place::Street(rumor.activity.clone()),
        };
        if self.inv(seat).place != place || !self.can_start(seat, &action.effects)? {
            return Ok(false);
        }
        let prompt = format!("« {} » : {} ?", card.name, action.label);
        if !self.ask_yes_with(seat, prompt, &action.effects)? {
            return Ok(false);
        }
        self.apply_effects(seat, &action.effects, Some(&active.card))?;
        Ok(action.location.is_some())
    }

    /// Le Grand Rituel: 3 spells (4 with 5 players or more) discarded in
    /// its streets.
    fn great_ritual(&mut self, seat: usize, card: &MythosCard, rumor: &Rumor) -> Flow<()> {
        if self.inv(seat).place != Place::Street(rumor.activity.clone()) {
            return Ok(());
        }
        let needed: u8 = if self.state.players() >= 5 { 4 } else { 3 };
        let spells = self
            .inv(seat)
            .cards
            .iter()
            .filter(|held| self.card(&held.card).is_ok_and(|c| c.deck == Deck::Spell))
            .count();
        if spells < usize::from(needed) {
            return Ok(());
        }
        let prompt = format!("Défausser {needed} Sorts contre « {} » ?", card.name);
        if !self.ask_yes_no(seat, DecisionKind::Optional, prompt)? {
            return Ok(());
        }
        self.discard_some(seat, &CardFilter::Deck(Deck::Spell), needed)?;
        self.end_rumor(seat, true)
    }

    /// Clue tokens spent one for one to take tokens off the Rumor.
    pub fn rumor_clue_removal(&mut self, seat: usize, this: Option<&CardId>) -> Flow<()> {
        let tokens = self
            .state
            .rumor
            .as_ref()
            .map_or(0, |rumor| u32::from(rumor.tokens));
        let most = tokens.min(self.amount(seat, Resource::Clues));
        let Some(card) = this.filter(|_| most > 0) else {
            return Ok(());
        };
        let options = (0..=most)
            .map(|count| option(Choice::Number(count), format!("{count} Indice(s)")))
            .collect();
        let prompt = "Combien d'Indices dépensez-vous ?";
        let Choice::Number(count) = self.ask(seat, DecisionKind::SpendClues, prompt, options)?
        else {
            return Ok(());
        };
        if count == 0 || !self.pay(seat, Resource::Clues, count)? {
            return Ok(());
        }
        self.add_tokens(seat, card, -i16::try_from(count).unwrap_or(i16::MAX));
        self.rumor_check(false)
    }

    /// L'Expérience Terrible: fight the Rumor's monsters one by one; the
    /// defeated ones are trophies, fleeing ends the action.
    pub fn experiment_fight(&mut self, seat: usize) -> Flow<()> {
        loop {
            let monsters = self
                .state
                .rumor
                .as_ref()
                .map(|r| r.monsters.clone())
                .unwrap_or_default();
            let mut options = vec![option(Choice::Done, "Arrêter")];
            for (index, monster) in monsters.iter().enumerate() {
                options.push(option(Choice::Option(index), self.monster_name(monster)));
            }
            if options.len() == 1 {
                break;
            }
            let prompt = "Quel monstre de la Rumeur combattez-vous ?";
            let Choice::Option(index) =
                self.ask(seat, DecisionKind::FaceMonster, prompt, options)?
            else {
                break;
            };
            let Some(monster) = monsters.get(index).cloned() else {
                break;
            };
            let faced = self.evade_or_fight(seat, &monster, true)?;
            if faced != Faced::Defeated {
                break;
            }
            if let Some(rumor) = &mut self.state.rumor {
                rumor.monsters.remove(index);
            }
        }
        self.rumor_check(false)
    }

    /// L'Expérience Terrible fails: Terror to 10 and the monsters freed in
    /// the streets of the University.
    pub fn experiment_release(&mut self) -> Flow<()> {
        self.raise_terror_to(10)?;
        let street = MonsterPlace::Street(StreetId::new("miskatonic_u"));
        for monster in std::mem::take(&mut self.scratch.rumor_monsters) {
            self.place_monster(monster, street.clone())?;
        }
        Ok(())
    }
}

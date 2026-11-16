//! Rumor conditions that need an investigator's action.

use data::{CardFilter, Deck, RumorCondition, SpecialId};

use crate::decision::DecisionKind;
use crate::game::Game;
use crate::halt::Flow;
use crate::state::Place;

/// Le Grand Rituel: discard spells in the streets to stop it.
const SPELLS_DISCARDED: &str = "great_ritual_spells_discarded";

impl Game<'_> {
    /// Le Grand Rituel: an investigator in the Rumor's streets during the
    /// Arkham encounters may discard 3 spells (4 with 5 players or more).
    pub fn rumor_street_action(&mut self, seat: usize) -> Flow<()> {
        let Some(active) = self.state.rumor.clone() else {
            return Ok(());
        };
        let card = self.mythos_card(&active.card)?;
        let Some(rumor) = &card.rumor else {
            return Ok(());
        };
        if rumor.pass.condition != RumorCondition::Special(SpecialId::new(SPELLS_DISCARDED)) {
            return Ok(());
        }
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
        let prompt = format!(
            "Défausser {needed} Sorts pour faire échouer « {} » ?",
            card.name
        );
        if !self.ask_yes_no(seat, DecisionKind::Optional, prompt)? {
            return Ok(());
        }
        self.discard_some(seat, &CardFilter::Deck(Deck::Spell), needed)?;
        self.log(format!("La Rumeur « {} » est déjouée.", card.name));
        self.state.rumor = None;
        self.apply_effects(seat, &rumor.pass.effects, None)
    }
}

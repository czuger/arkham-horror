//! Phase IV: Other World encounters.

use data::{GateCard, GateEncounter, OtherWorldId};

use crate::game::Game;
use crate::halt::Flow;
use crate::state::{Phase, Place};

impl<'a> Game<'a> {
    /// The Other World encounter phase, investigator by investigator.
    pub fn other_world_encounter_phase(&mut self) -> Flow<()> {
        self.state.phase = Phase::OtherWorldEncounters;
        for seat in self.state.seat_order() {
            if self.inv(seat).skipping_turn {
                continue;
            }
            if let Place::OtherWorld { world, .. } = self.inv(seat).place.clone() {
                self.catch_knock_out(seat, |game| game.other_world_encounter(seat, &world))?;
            }
        }
        Ok(())
    }

    /// Draws gate cards until one has a colour of the world, then applies the
    /// world's paragraph or « Autre ». Cards go under the deck.
    fn other_world_encounter(&mut self, seat: usize, world: &OtherWorldId) -> Flow<()> {
        let colors = self.world(world)?.colors.clone();
        let mut card = None;
        for _ in 0..self.state.gate_deck.len() {
            let id = self.state.gate_deck.remove(0);
            self.state.gate_deck.push(id.clone());
            let candidate = self.gate_card(&id)?;
            if colors.contains(&candidate.color) {
                card = Some(candidate);
                break;
            }
        }
        let Some(card) = card else {
            self.log("Aucune carte Portail de la bonne couleur.");
            return Ok(());
        };
        let Some(encounter) = Self::world_paragraph(card, world) else {
            return Ok(());
        };
        self.log(format!(
            "Rencontre dans {} : {}",
            self.world(world)?.name,
            encounter.text
        ));
        self.apply_effects(seat, &encounter.effects, None)
    }

    /// A gate card by id.
    fn gate_card(&self, id: &data::CardId) -> Flow<&'a GateCard> {
        let data: &'a data::GameData = self.data;
        data.gate_cards
            .cards
            .iter()
            .find(|card| &card.id == id)
            .ok_or_else(|| crate::halt::Halt::Broken(format!("carte Portail « {id} » introuvable")))
    }

    /// The paragraph for a world, or « Autre ».
    fn world_paragraph<'c>(card: &'c GateCard, world: &OtherWorldId) -> Option<&'c GateEncounter> {
        card.encounters
            .iter()
            .find(|encounter| encounter.world.as_ref() == Some(world))
            .or_else(|| {
                card.encounters
                    .iter()
                    .find(|encounter| encounter.world.is_none())
            })
    }
}

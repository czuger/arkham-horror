//! Phase IV: Other World encounters.

use data::{GateCard, GateEncounter, OtherWorldId, Timing};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option_with};
use crate::halt::Flow;
use crate::shown::{Deck, Exhibit};
use crate::state::{Phase, Place};

/// Most redraws in a row, so a loop of redraw effects always ends.
const MAX_REDRAWS: usize = 5;

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

    /// An encounter in an Other World; Œil du Dragon may redraw the card.
    pub fn other_world_encounter(&mut self, seat: usize, world: &OtherWorldId) -> Flow<()> {
        for _ in 0..MAX_REDRAWS {
            self.scratch.redraw = false;
            let Some((card, encounter)) = self.pick_gate_encounter(seat, world)? else {
                return Ok(());
            };
            self.log(format!(
                "Rencontre dans {} : {}",
                self.world(world)?.name,
                encounter.text
            ));
            self.covered.insert(crate::coverage::gate_key(
                &card.id,
                encounter.world.as_ref(),
            ));
            let prompt = "Utiliser une capacité avant de résoudre la rencontre ?";
            self.offer_abilities(
                seat,
                &|timing| *timing == Timing::AfterEncounterDraw,
                prompt,
            )?;
            if !self.scratch.redraw {
                return self.apply_effects(seat, &encounter.effects, None);
            }
        }
        self.scratch.redraw = false;
        Ok(())
    }

    /// Draws the gate card (two for Gloria, who picks one) and finds the
    /// paragraph for the world.
    fn pick_gate_encounter(
        &mut self,
        seat: usize,
        world: &OtherWorldId,
    ) -> Flow<Option<(&'a GateCard, &'a GateEncounter)>> {
        let mut found: Vec<(&'a GateCard, &'a GateEncounter)> = Vec::new();
        for _ in 0..self.gate_draws(seat) {
            let Some(card) = self.draw_gate_card(world)? else {
                break;
            };
            if found.iter().any(|(other, _)| other.id == card.id) {
                continue;
            }
            self.show(Exhibit::Card {
                deck: Deck::Gate,
                card: card.id.clone(),
                seat,
            });
            if let Some(encounter) = Self::world_paragraph(card, world) {
                found.push((card, encounter));
            }
        }
        if found.is_empty() {
            self.log("Aucune carte Portail de la bonne couleur.");
        }
        if found.len() <= 1 {
            return Ok(found.pop());
        }
        let options = found
            .iter()
            .enumerate()
            .map(|(index, (_, encounter))| {
                option_with(
                    Choice::Option(index),
                    encounter.text.clone(),
                    &encounter.effects,
                )
            })
            .collect();
        let prompt = "Quelle rencontre suivez-vous ?";
        let Choice::Option(index) = self.ask(seat, DecisionKind::CardChoice, prompt, options)?
        else {
            return Ok(None);
        };
        Ok(found.get(index).copied())
    }

    /// Draws gate cards until one has a colour of the world. Cards go under
    /// the deck; the colourless card reshuffles it.
    fn draw_gate_card(&mut self, world: &OtherWorldId) -> Flow<Option<&'a GateCard>> {
        let colors = self.world(world)?.colors.clone();
        for _ in 0..self.state.gate_deck.len() {
            let id = self.state.gate_deck.remove(0);
            self.state.gate_deck.push(id.clone());
            let candidate = self.gate_card(&id)?;
            let Some(color) = candidate.color else {
                self.log("Carte Portail sans couleur : le paquet Portail est remélangé.");
                let mut deck = std::mem::take(&mut self.state.gate_deck);
                self.state.rng.shuffle(&mut deck);
                self.state.gate_deck = deck;
                continue;
            };
            if colors.contains(&color) {
                return Ok(Some(candidate));
            }
        }
        Ok(None)
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

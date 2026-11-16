//! Phase III: Arkham encounters, and closing or sealing gates.

use data::{CardId, CheckKind, LocationEncounter, LocationId, Resource, SpecialId};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::gates::Closing;
use crate::state::{Phase, Place};

/// The Elder Sign card's special effect.
const ELDER_SIGN: &str = "elder_sign";

impl<'a> Game<'a> {
    /// The Arkham encounter phase, investigator by investigator.
    pub fn arkham_encounter_phase(&mut self) -> Flow<()> {
        self.state.phase = Phase::ArkhamEncounters;
        for seat in self.state.seat_order() {
            let inv = self.inv(seat);
            if !inv.skipping_turn && !inv.encounters_over {
                self.catch_knock_out(seat, |game| game.arkham_encounter(seat))?;
            }
        }
        Ok(())
    }

    /// One investigator: Rumor action in the streets, gate (drawn in, or
    /// close and seal when explored), or a location card.
    fn arkham_encounter(&mut self, seat: usize) -> Flow<()> {
        self.rumor_street_action(seat)?;
        let Place::Location(location) = self.inv(seat).place.clone() else {
            return Ok(());
        };
        if self.state.gate_at(&location).is_some() {
            if self.inv(seat).explored {
                return self.close_attempt(seat, &location);
            }
            return self.draw_through_gate(seat, &location, false);
        }
        let Some(encounter) = self.draw_location_encounter(seat, &location)? else {
            return Ok(());
        };
        self.log(format!(
            "Rencontre à {} : {}",
            self.location(&location)?.name,
            encounter.text
        ));
        self.apply_effects(seat, &encounter.effects, None)
    }

    /// Shuffles the district deck and reads the paragraph of the location
    /// (the one with a fulfilled condition replaces the normal one).
    fn draw_location_encounter(
        &mut self,
        seat: usize,
        location: &LocationId,
    ) -> Flow<Option<&'a LocationEncounter>> {
        let data: &'a data::GameData = self.data;
        let street = self.location(location)?.street.clone();
        let deck: Vec<_> = data
            .location_cards
            .cards
            .iter()
            .filter(|card| card.district == street)
            .collect();
        if deck.is_empty() {
            self.log("Aucune carte Lieu pour ce quartier.");
            return Ok(None);
        }
        let card = deck[self.state.rng.below(deck.len())];
        let here = card
            .encounters
            .iter()
            .filter(|encounter| &encounter.location == location);
        let mut normal = None;
        for encounter in here {
            match &encounter.requires {
                Some(condition) if self.holds(seat, condition) => return Ok(Some(encounter)),
                Some(_) => {}
                None => normal = Some(encounter),
            }
        }
        Ok(normal)
    }

    /// With an explored marker: try to close the gate (Lore or Fight), seal it
    /// with clue tokens after closing, or seal it with an Elder Sign.
    fn close_attempt(&mut self, seat: usize, location: &LocationId) -> Flow<()> {
        let Some(marker) = self.state.gate_at(location).map(|gate| gate.marker) else {
            return Ok(());
        };
        let modifier = self
            .data
            .gate_markers
            .markers
            .get(marker)
            .map_or(0, |m| m.close_modifier);
        let sealable = self.can_be_sealed(location)?;
        let mut options = vec![
            option(Choice::Done, "Ne rien tenter"),
            option(
                Choice::CloseWith(CheckKind::Lore),
                format!("Fermer avec un test de Savoir ({modifier:+})"),
            ),
            option(
                Choice::CloseWith(CheckKind::Fight),
                format!("Fermer avec un test de Vigueur ({modifier:+})"),
            ),
        ];
        let elder_sign = self.elder_sign_card(seat)?;
        if let (Some(card), true) = (&elder_sign, sealable) {
            options.push(option(
                Choice::ElderSign,
                format!("Sceller avec {}", self.card_name(card)),
            ));
        }
        match self.ask(
            seat,
            DecisionKind::CloseGate,
            "Le portail est devant vous.",
            options,
        )? {
            Choice::CloseWith(kind) => {
                self.close_with_check(seat, location, kind, modifier, sealable)
            }
            Choice::ElderSign => {
                let Some(card) = elder_sign else {
                    return Ok(());
                };
                self.return_to_box(seat, &card);
                self.close_gate(seat, location, Closing::SealedWithElderSign)?;
                self.lose_both(seat, Resource::Stamina, 1, Some((Resource::Sanity, 1)))
            }
            _ => Ok(()),
        }
    }

    /// Closing check; on a success, may seal with clue tokens.
    fn close_with_check(
        &mut self,
        seat: usize,
        location: &LocationId,
        kind: CheckKind,
        modifier: i8,
        sealable: bool,
    ) -> Flow<()> {
        let difficulty = self.close_difficulty();
        if !self
            .check(seat, kind, i32::from(modifier), difficulty)?
            .passed
        {
            return Ok(());
        }
        let cost = self.seal_cost();
        let can_seal = sealable && self.inv(seat).clues >= cost;
        let prompt = format!("Dépenser {cost} Indices pour sceller le portail ?");
        if can_seal && self.ask_yes_no(seat, DecisionKind::CloseGate, prompt)? {
            self.inv_mut(seat).clues -= cost;
            return self.close_gate(seat, location, Closing::SealedWithClues);
        }
        self.close_gate(seat, location, Closing::Closed)
    }

    /// A held card with the Elder Sign effect.
    fn elder_sign_card(&self, seat: usize) -> Flow<Option<CardId>> {
        for held in &self.inv(seat).cards {
            if self.card(&held.card)?.special == Some(SpecialId::new(ELDER_SIGN)) {
                return Ok(Some(held.card.clone()));
            }
        }
        Ok(None)
    }
}

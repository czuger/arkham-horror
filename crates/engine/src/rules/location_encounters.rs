//! Location cards: drawing them, picking one, redrawing.

use data::{CardId, LocationCard, LocationEncounter, LocationId, Timing};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option, option_with};
use crate::halt::Flow;
use crate::state::Place;

/// Most redraws in a row, so a loop of redraw effects always ends.
const MAX_REDRAWS: usize = 5;

impl<'a> Game<'a> {
    /// An encounter on a location: `draw` cards of its district are drawn and
    /// the investigator follows one. Œil du Dragon or a card may redraw.
    pub fn location_encounter(&mut self, seat: usize, location: &LocationId, draw: u8) -> Flow<()> {
        for _ in 0..MAX_REDRAWS {
            self.scratch.redraw = false;
            let Some((card, encounter)) = self.pick_location_encounter(seat, location, draw)?
            else {
                return Ok(());
            };
            self.log(format!(
                "Rencontre à {} : {}",
                self.location(location)?.name,
                encounter.text
            ));
            self.covered
                .insert(crate::coverage::location_key(card, location));
            let prompt = "Utiliser une capacité avant de résoudre la rencontre ?";
            self.offer_abilities(
                seat,
                &|timing| *timing == Timing::AfterEncounterDraw,
                prompt,
            )?;
            if self.scratch.redraw {
                continue;
            }
            self.apply_effects(seat, &encounter.effects, None)?;
            let still_here = self.inv(seat).place == Place::Location(location.clone());
            if !(self.scratch.redraw && still_here) {
                break;
            }
        }
        self.scratch.redraw = false;
        Ok(())
    }

    /// Draws `draw` cards of the district; the investigator keeps one.
    fn pick_location_encounter(
        &mut self,
        seat: usize,
        location: &LocationId,
        draw: u8,
    ) -> Flow<Option<(&'a CardId, &'a LocationEncounter)>> {
        let mut drawn = Vec::new();
        for _ in 0..self.location_draws(seat, draw) {
            if let Some(card) = self.draw_location_card(location)?
                && !drawn
                    .iter()
                    .any(|other: &&LocationCard| other.id == card.id)
            {
                drawn.push(card);
            }
        }
        let mut encounters = Vec::new();
        for card in drawn {
            if let Some(encounter) = self.paragraph_for(seat, card, location) {
                encounters.push((&card.id, encounter));
            }
        }
        if encounters.len() <= 1 {
            return Ok(encounters.pop());
        }
        let options = encounters
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
        Ok(encounters.get(index).copied())
    }

    /// A random card of the location's district, or the one set aside by
    /// the crystal ball (R23).
    fn draw_location_card(&mut self, location: &LocationId) -> Flow<Option<&'a LocationCard>> {
        let data: &'a data::GameData = self.data;
        let street = self.location(location)?.street.clone();
        if let Some((reserved_street, card)) = self.state.reserved_location_card.clone()
            && reserved_street == street
        {
            self.state.reserved_location_card = None;
            return Ok(self.location_card(&card));
        }
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
        Ok(Some(deck[self.state.rng.below(deck.len())]))
    }

    /// A location card by id.
    pub fn location_card(&self, id: &CardId) -> Option<&'a LocationCard> {
        let data: &'a data::GameData = self.data;
        data.location_cards.cards.iter().find(|card| &card.id == id)
    }

    /// The paragraph of a card for a location: one whose condition holds
    /// replaces the normal one.
    fn paragraph_for(
        &self,
        seat: usize,
        card: &'a LocationCard,
        location: &LocationId,
    ) -> Option<&'a LocationEncounter> {
        let mut normal = None;
        for encounter in card.encounters.iter().filter(|e| &e.location == location) {
            match &encounter.requires {
                Some(condition) if self.holds(seat, condition) => return Some(encounter),
                Some(_) => {}
                None => normal = Some(encounter),
            }
        }
        normal
    }

    /// Crystal ball: the top card of a district deck is set aside for the
    /// next encounter in that district (R23).
    pub fn crystal_ball(&mut self, seat: usize) -> Flow<()> {
        let data: &'a data::GameData = self.data;
        let mut districts: Vec<_> = data
            .location_cards
            .cards
            .iter()
            .map(|c| &c.district)
            .collect();
        districts.dedup();
        let mut options = Vec::new();
        for (index, district) in districts.iter().enumerate() {
            options.push(option(
                Choice::Option(index),
                self.street(district)?.name.clone(),
            ));
        }
        let prompt = "Quel paquet Lieu regardez-vous ?";
        let Choice::Option(index) = self.ask(seat, DecisionKind::CardChoice, prompt, options)?
        else {
            return Ok(());
        };
        let Some(&district) = districts.get(index) else {
            return Ok(());
        };
        let deck: Vec<_> = data
            .location_cards
            .cards
            .iter()
            .filter(|card| &card.district == district)
            .collect();
        if deck.is_empty() {
            return Ok(());
        }
        let card = deck[self.state.rng.below(deck.len())];
        self.state.reserved_location_card = Some((district.clone(), card.id.clone()));
        self.log(format!(
            "{} voit la prochaine rencontre de {}.",
            self.name(seat),
            self.street(district)?.name.clone()
        ));
        Ok(())
    }
}

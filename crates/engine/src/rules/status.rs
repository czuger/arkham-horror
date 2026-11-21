//! Investigator status: insane, unconscious, lost, devoured, arrested, delayed.

use data::{CardId, Deck, LocationId};

use crate::decision::{CardAim, Choice};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::state::{Area, Place};

/// Where insane investigators wake up.
pub(crate) const ASYLUM: &str = "arkham_asylum";
/// Where unconscious investigators wake up.
pub(crate) const HOSPITAL: &str = "st_marys_hospital";
/// Where arrested investigators go after the jail cell.
pub(crate) const POLICE_STATION: &str = "police_station";
/// The Retainer card, lost when knocked out.
pub(crate) const RETAINER: &str = "retainer";

/// Special cards that count as items (FAQ): the Deputy's gear.
const DEPUTY_ITEMS: [&str; 2] = ["deputys_revolver", "patrol_wagon"];

impl Game<'_> {
    /// Moves an investigator. Leaving a location drops the explored marker.
    pub fn move_to(&mut self, seat: usize, place: Place) {
        let inv = self.inv_mut(seat);
        if inv.place != place {
            inv.explored = false;
        }
        inv.place = place;
    }

    /// Sends an investigator through the gate on a location, to the first
    /// area of its Other World.
    pub fn draw_through_gate(
        &mut self,
        seat: usize,
        location: &LocationId,
        delayed: bool,
    ) -> Flow<()> {
        let Some(gate) = self.state.gate_at(location) else {
            return Ok(());
        };
        let world = self
            .data
            .gate_markers
            .markers
            .get(gate.marker)
            .map(|marker| marker.world.clone());
        let Some(world) = world else {
            return Ok(());
        };
        self.log(format!(
            "{} est aspiré vers {}.",
            self.name(seat),
            self.world(&world)?.name
        ));
        self.move_to(
            seat,
            Place::OtherWorld {
                world,
                area: Area::First,
            },
        );
        if delayed {
            self.inv_mut(seat).delayed = true;
        }
        self.inv_mut(seat).entered_gate = Some(location.clone());
        Ok(())
    }

    /// Insane (Sanity 0) or unconscious (Stamina 0): loses half the items and
    /// clue tokens and every Retainer, then wakes up at the Asylum or the
    /// Hospital, or gets lost in time and space from an Other World.
    pub fn knock_out(&mut self, seat: usize, insane: bool) -> Flow<()> {
        let word = if insane {
            "devient fou"
        } else {
            "tombe inconscient"
        };
        self.log(format!("{} {word}.", self.name(seat)));
        self.tally.knocked_out += 1;
        self.lose_half_belongings(seat)?;
        let inv = self.inv_mut(seat);
        inv.encounters_over = true;
        let mary = inv.id.as_str() == crate::rules::investigator_powers::SISTER_MARY;
        if matches!(inv.place, Place::OtherWorld { .. }) && !mary {
            inv.sanity = 1;
            inv.stamina = 1;
            return self.lose_in_time_and_space(seat);
        }
        let (target, label) = if insane {
            (ASYLUM, "l'Asile d'Arkham")
        } else {
            (HOSPITAL, "l'Hôpital Sainte-Marie")
        };
        let inv = self.inv_mut(seat);
        if insane {
            inv.sanity = 1;
        } else {
            inv.stamina = 1;
        }
        self.move_to(seat, Place::Location(LocationId::new(target)));
        self.log(format!("{} se réveille à {label}.", self.name(seat)));
        Ok(())
    }

    /// Discards half the items (player's choice), half the clue tokens and
    /// every Retainer.
    fn lose_half_belongings(&mut self, seat: usize) -> Flow<()> {
        let items: Vec<CardId> = self.held_items(seat)?;
        for _ in 0..items.len() / 2 {
            let mut options = Vec::new();
            for card in self.held_items(seat)? {
                options.push(option(Choice::Card(card.clone()), self.card_name(&card)));
            }
            if let Choice::Card(card) =
                self.pick_card(seat, CardAim::Lose, "Objet à défausser", options)?
            {
                self.log(format!(
                    "{} défausse {}.",
                    self.name(seat),
                    self.card_name(&card)
                ));
                self.discard_card(seat, &card)?;
            }
        }
        let inv = self.inv_mut(seat);
        inv.clues -= inv.clues / 2;
        let retainer = CardId::new(RETAINER);
        while self.inv(seat).holds(&retainer) {
            self.discard_card(seat, &retainer)?;
        }
        Ok(())
    }

    /// Cards that count as items: Common and Unique Items and Spells.
    pub fn held_items(&self, seat: usize) -> Flow<Vec<CardId>> {
        let mut items = Vec::new();
        for held in &self.inv(seat).cards {
            let deck = self.card(&held.card)?.deck;
            let deputy_item = DEPUTY_ITEMS.contains(&held.card.as_str());
            if deputy_item || matches!(deck, Deck::CommonItem | Deck::UniqueItem | Deck::Spell) {
                items.push(held.card.clone());
            }
        }
        Ok(items)
    }

    /// Lost in time and space: delayed on the special area.
    pub fn lose_in_time_and_space(&mut self, seat: usize) -> Flow<()> {
        if self.guardian_angel(seat)? {
            return Ok(());
        }
        self.log(format!(
            "{} est perdu dans le temps et l'espace.",
            self.name(seat)
        ));
        self.tally.lost_in_time += 1;
        self.move_to(seat, Place::LostInTimeAndSpace);
        self.inv_mut(seat).delayed = true;
        self.on_lost_in_time(seat)
    }

    /// Arrested: half the money, delayed, in the jail cell.
    pub fn arrest(&mut self, seat: usize) {
        let inv = self.inv_mut(seat);
        inv.money -= inv.money / 2;
        inv.delayed = true;
        self.move_to(seat, Place::JailCell);
        self.log(format!("{} est arrêté et mis en cellule.", self.name(seat)));
    }

    /// Devoured: eliminated during the final battle, otherwise replaced by a
    /// new investigator who starts next turn.
    pub fn devour(&mut self, seat: usize) -> Flow<()> {
        self.log(format!("{} est dévoré.", self.name(seat)));
        self.tally.devoured += 1;
        let cards: Vec<CardId> = self
            .inv(seat)
            .cards
            .iter()
            .map(|held| held.card.clone())
            .collect();
        for card in cards {
            self.discard_card(seat, &card)?;
        }
        if self.state.battle.is_some() {
            let inv = self.inv_mut(seat);
            inv.eliminated = true;
            inv.sanity = 0;
            inv.stamina = 0;
            return Ok(());
        }
        self.replace_investigator(seat)
    }
}

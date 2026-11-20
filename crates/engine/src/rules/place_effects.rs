//! Effect primitives that move the investigator or change their turn.

use data::{Effect, LocationId, OtherWorldId};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::{Flow, Halt};
use crate::state::{Area, Phase, Place};

impl Game<'_> {
    /// Moves, trips to Other Worlds, arrests and lost turns.
    pub fn apply_place_effect(&mut self, seat: usize, effect: &Effect) -> Flow<()> {
        match effect {
            Effect::MoveToStreet => {
                if let Place::Location(id) = &self.inv(seat).place {
                    let street = self.location(id)?.street.clone();
                    self.move_to(seat, Place::Street(street));
                }
                Ok(())
            }
            Effect::MoveTo(location) => self.move_to_location(seat, location),
            Effect::MoveAnywhereInArkham => self.move_anywhere(seat),
            Effect::MoveToMythosGate => self.move_to_mythos_gate(seat),
            Effect::EncounterHere { draw } => match self.location_of(seat) {
                Some(location) => self.location_encounter(seat, &location, *draw),
                None => Ok(()),
            },
            Effect::VisitOtherWorld(world) => self.visit_other_world(seat, world),
            Effect::ReturnToArkham => self.return_to_arkham(seat),
            Effect::AdvanceInOtherWorld => self.advance_in_other_world(seat),
            Effect::GainMovementPoints(points) => {
                self.scratch.movement_points += u32::from(*points);
                Ok(())
            }
            Effect::BonusUntilEndOfTurn(bonus) => {
                let inv = self.inv_mut(seat);
                inv.turn_bonus = inv.turn_bonus.saturating_add(*bonus);
                Ok(())
            }
            _ => self.apply_status_effect(seat, effect),
        }
    }

    /// Arrests, lost turns, being devoured.
    fn apply_status_effect(&mut self, seat: usize, effect: &Effect) -> Flow<()> {
        match effect {
            Effect::Arrested => {
                if self.mythos_has(|effect| *effect == data::Ongoing::NoArrests) {
                    self.log("Personne ne peut être arrêté.");
                    return Ok(());
                }
                self.arrest(seat);
                Err(Halt::KnockedOut(seat))
            }
            Effect::LostInTimeAndSpace => {
                self.lose_in_time_and_space(seat)?;
                Err(Halt::KnockedOut(seat))
            }
            Effect::Devoured => {
                self.devour(seat)?;
                Err(Halt::KnockedOut(seat))
            }
            Effect::Delayed => {
                self.inv_mut(seat).delayed = true;
                self.log(format!("{} est retardé.", self.name(seat)));
                Ok(())
            }
            Effect::LoseNextTurn => {
                self.inv_mut(seat).lose_next_turn = true;
                self.log(format!("{} perdra son prochain tour.", self.name(seat)));
                Ok(())
            }
            _ => self.apply_board_effect(seat, effect),
        }
    }

    /// Goes straight to a location: no monster faced, no clue picked up.
    fn move_to_location(&mut self, seat: usize, location: &LocationId) -> Flow<()> {
        self.log(format!(
            "{} va à {}.",
            self.name(seat),
            self.location(location)?.name
        ));
        self.move_to(seat, Place::Location(location.clone()));
        Ok(())
    }

    /// Goes to any location or street of Arkham, player's choice. During the
    /// Arkham encounters, a location brings an encounter at once.
    pub fn move_anywhere(&mut self, seat: usize) -> Flow<()> {
        let mut options = Vec::new();
        for place in self.arkham_places() {
            let label = self.place_name(&place);
            options.push(option(Choice::Place(place), label));
        }
        let Choice::Place(place) =
            self.ask(seat, DecisionKind::Destination, "Où allez-vous ?", options)?
        else {
            return Ok(());
        };
        self.log(format!(
            "{} va à {}.",
            self.name(seat),
            self.place_name(&place)
        ));
        self.move_to(seat, place.clone());
        if self.state.phase != Phase::ArkhamEncounters {
            return Ok(());
        }
        let Place::Location(location) = place else {
            return Ok(());
        };
        if self.state.gate_at(&location).is_some() {
            self.draw_through_gate(seat, &location, false)?;
            return Err(Halt::KnockedOut(seat));
        }
        self.location_encounter(seat, &location, 1)
    }

    /// Draws Mythos cards until one shows a gate location, and goes there
    /// (R22). The cards go back under the deck.
    fn move_to_mythos_gate(&mut self, seat: usize) -> Flow<()> {
        for _ in 0..self.state.mythos_deck.len() {
            let Some(card) = self.draw_mythos()? else {
                return Ok(());
            };
            self.state.mythos_deck.push(card.id.clone());
            if let Some(location) = &card.gate {
                return self.move_to_location(seat, location);
            }
        }
        Ok(())
    }

    /// An encounter in an Other World, then straight back where it started.
    fn visit_other_world(&mut self, seat: usize, world: &OtherWorldId) -> Flow<()> {
        let before = self.inv(seat).place.clone();
        let there = Place::OtherWorld {
            world: world.clone(),
            area: Area::First,
        };
        self.log(format!(
            "{} est emporté vers {}.",
            self.name(seat),
            self.world(world)?.name
        ));
        self.move_to(seat, there.clone());
        self.other_world_encounter(seat, world)?;
        if self.inv(seat).place == there {
            self.move_to(seat, before);
            self.log(format!("{} revient aussitôt.", self.name(seat)));
        }
        Ok(())
    }

    /// From the first area to the second; from the second, back to Arkham.
    fn advance_in_other_world(&mut self, seat: usize) -> Flow<()> {
        let Place::OtherWorld { world, area } = self.inv(seat).place.clone() else {
            return Ok(());
        };
        if area == Area::Second {
            return self.return_to_arkham(seat);
        }
        self.log(format!(
            "{} avance dans {}.",
            self.name(seat),
            self.world(&world)?.name
        ));
        self.move_to(
            seat,
            Place::OtherWorld {
                world,
                area: Area::Second,
            },
        );
        Ok(())
    }
}

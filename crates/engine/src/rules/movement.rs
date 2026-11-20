//! Phase II: Movement.

use data::{Ability, Effect, LocationId, Moment, Ongoing, Timing};

use crate::decision::{Choice, DecisionKind, DecisionOption};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::abilities::Usable;
use crate::rules::status::POLICE_STATION;
use crate::state::{Area, Phase, Place};

/// Abilities usable during the Movement phase, combat spells left out.
fn movement_timing(timing: &Timing) -> bool {
    matches!(timing, Timing::Movement | Timing::AnyPhase)
}

/// Whether an ability only makes sense in a combat.
fn combat_only(ability: &Ability) -> bool {
    ability
        .effects
        .iter()
        .any(|effect| matches!(effect, Effect::CombatBonus(_)))
}

impl Game<'_> {
    /// The Movement phase, investigator by investigator.
    pub fn movement_phase(&mut self) -> Flow<()> {
        self.state.phase = Phase::Movement;
        for seat in self.state.seat_order() {
            if !self.inv(seat).skipping_turn {
                self.catch_knock_out(seat, |game| game.movement(seat))?;
            }
        }
        Ok(())
    }

    /// One investigator's movement, by where they stand.
    fn movement(&mut self, seat: usize) -> Flow<()> {
        let delayed = self.inv(seat).delayed;
        if delayed {
            self.inv_mut(seat).delayed = false;
            self.log(format!("{} se relève.", self.name(seat)));
        }
        match self.inv(seat).place.clone() {
            Place::JailCell if delayed => {
                self.move_to(seat, Place::Location(LocationId::new(POLICE_STATION)));
                Ok(())
            }
            Place::OtherWorld { world, area } if !delayed => {
                self.other_world_movement(seat, world, area)
            }
            place if place.in_arkham() => {
                if delayed {
                    let mut evaded = Vec::new();
                    self.face_monsters_here(seat, &mut evaded)?;
                    return Ok(());
                }
                self.arkham_movement(seat)
            }
            _ => Ok(()),
        }
    }

    /// Movement in Arkham: one step per movement point; monsters must be
    /// evaded or fought to leave and where the movement ends.
    fn arkham_movement(&mut self, seat: usize) -> Flow<()> {
        self.scratch.movement_points = self.movement_points(seat)?;
        let mut evaded = Vec::new();
        loop {
            let place = self.inv(seat).place.clone();
            let points = self.scratch.movement_points;
            let usable = self.usable_abilities(seat, &movement_timing)?;
            let options = self.movement_options(&place, points, &usable);
            let prompt = format!(
                "Mouvement depuis {} ({points} point(s))",
                self.place_name(&place)
            );
            match self.ask(seat, DecisionKind::Move, prompt, options)? {
                Choice::Place(next) => {
                    if !self.face_monsters_here(seat, &mut evaded)? {
                        break;
                    }
                    evaded.clear();
                    self.scratch.movement_points -= 1;
                    self.log(format!(
                        "{} va à {}.",
                        self.name(seat),
                        self.place_name(&next)
                    ));
                    self.move_to(seat, next);
                }
                Choice::Option(index) => {
                    if let Some(entry) = usable.into_iter().nth(index) {
                        self.use_ability(seat, &entry.card, entry.ability)?;
                    }
                }
                _ => {
                    self.face_monsters_here(seat, &mut evaded)?;
                    break;
                }
            }
        }
        self.end_movement(seat)
    }

    /// Options of one movement step: stop, a neighbour, or a card ability.
    fn movement_options(
        &self,
        place: &Place,
        points: u32,
        usable: &[Usable<'_>],
    ) -> Vec<DecisionOption> {
        let mut options = vec![option(Choice::Done, "Terminer le mouvement ici")];
        if points > 0 {
            let stuck = self.street_closed(place);
            for next in self.neighbours(place) {
                if stuck || self.street_closed(&next) {
                    continue;
                }
                let label = format!("Aller à {}", self.place_name(&next));
                options.push(option(Choice::Place(next), label));
            }
        }
        for (index, entry) in usable
            .iter()
            .enumerate()
            .filter(|(_, e)| !combat_only(e.ability))
        {
            options.push(option(
                Choice::Option(index),
                format!("Utiliser {}", self.card_name(&entry.card)),
            ));
        }
        options
    }

    /// End of an Arkham movement: pick up the clue tokens, then triggers.
    fn end_movement(&mut self, seat: usize) -> Flow<()> {
        if let Place::Location(id) = self.inv(seat).place.clone() {
            let clues = self.state.clues.remove(&id).unwrap_or(0);
            if clues > 0 {
                self.inv_mut(seat).clues += u16::from(clues);
                self.log(format!("{} ramasse {clues} Indice(s).", self.name(seat)));
            }
        }
        self.fire_triggers(seat, Moment::EndOfMovement)
    }

    /// In an Other World: first area to second, second area back to Arkham.
    fn other_world_movement(
        &mut self,
        seat: usize,
        world: data::OtherWorldId,
        area: Area,
    ) -> Flow<()> {
        let prompt = "Avant de vous déplacer dans l'Autre Monde";
        self.offer_abilities(seat, &|timing| *timing == Timing::Movement, prompt)?;
        if !matches!(self.inv(seat).place, Place::OtherWorld { .. }) {
            return Ok(());
        }
        match area {
            Area::First => {
                let name = self.world(&world)?.name.clone();
                self.move_to(
                    seat,
                    Place::OtherWorld {
                        world,
                        area: Area::Second,
                    },
                );
                self.log(format!("{} avance dans {name}.", self.name(seat)));
                Ok(())
            }
            Area::Second => self.return_to_arkham(seat),
        }
    }

    /// Back to Arkham through an open gate to the world being left, with an
    /// explored marker; lost in time and space if there is none.
    pub fn return_to_arkham(&mut self, seat: usize) -> Flow<()> {
        let Place::OtherWorld { world, .. } = self.inv(seat).place.clone() else {
            return Ok(());
        };
        let mut options = Vec::new();
        let any_gate = self.has_ongoing(seat, |effect| *effect == Ongoing::AnyGateReturn);
        for gate in &self.state.gates {
            let leads_there = any_gate
                || self
                    .data
                    .gate_markers
                    .markers
                    .get(gate.marker)
                    .is_some_and(|m| m.world == world);
            if leads_there {
                let label = self.location(&gate.location)?.name.clone();
                options.push(option(Choice::Gate(gate.location.clone()), label));
            }
        }
        if options.is_empty() {
            self.lose_in_time_and_space(seat)?;
            return Ok(());
        }
        let Choice::Gate(location) = self.ask(
            seat,
            DecisionKind::ReturnThroughGate,
            "Par quel portail revenez-vous ?",
            options,
        )?
        else {
            return Ok(());
        };
        self.log(format!(
            "{} revient à {}.",
            self.name(seat),
            self.location(&location)?.name
        ));
        self.move_to(seat, Place::Location(location));
        let inv = self.inv_mut(seat);
        inv.explored = true;
        inv.returned_this_turn = true;
        self.fire_triggers(seat, Moment::BackFromOtherWorld)
    }
}

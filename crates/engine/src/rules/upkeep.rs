//! Phase I: Upkeep.

use data::{CardId, Timing};

use crate::decision::{Choice, Context, DecisionKind, DecisionOption};
use crate::game::{Game, option};
use crate::halt::{Flow, Halt};
use crate::rules::abilities::is_mandatory;
use crate::state::{Phase, Place};

/// French names of the slider tracks.
const TRACKS: [&str; 3] = ["Vitesse/Discrétion", "Vigueur/Volonté", "Savoir/Chance"];

impl Game<'_> {
    /// The Upkeep phase, investigator by investigator.
    pub fn upkeep_phase(&mut self) -> Flow<()> {
        self.state.phase = Phase::Upkeep;
        let mut seats = Vec::new();
        for seat in self.state.seat_order() {
            let inv = self.inv_mut(seat);
            if inv.lose_next_turn {
                inv.lose_next_turn = false;
                inv.skipping_turn = true;
            }
            if !self.inv(seat).skipping_turn {
                seats.push(seat);
            }
        }
        self.upkeep_all(&seats)
    }

    /// The upkeep of `seats`, in order, then their skill adjustments one
    /// after the other, so they can be set together. An investigator
    /// knocked out during their upkeep does not adjust.
    pub fn upkeep_all(&mut self, seats: &[usize]) -> Flow<()> {
        let mut adjusting = Vec::new();
        for &seat in seats {
            match self.upkeep(seat) {
                Ok(()) => adjusting.push(seat),
                Err(Halt::KnockedOut(knocked)) if knocked == seat => {}
                Err(halt) => return Err(halt),
            }
        }
        for (index, &seat) in adjusting.iter().enumerate() {
            let after = adjusting.get(index + 1..).unwrap_or_default();
            self.adjust_skills(seat, after)?;
        }
        Ok(())
    }

    /// One investigator's upkeep: back from Lost in Time and Space, refresh
    /// cards, upkeep actions. The skill adjustment follows everyone's upkeep.
    fn upkeep(&mut self, seat: usize) -> Flow<()> {
        if self.inv(seat).place == Place::LostInTimeAndSpace && !self.inv(seat).delayed {
            self.move_anywhere(seat)?;
        }
        for held in &mut self.inv_mut(seat).cards {
            held.exhausted = false;
        }
        self.upkeep_actions(seat)?;
        self.sheet_upkeep(seat)
    }

    /// Mandatory upkeep actions of the held cards, in the player's order.
    /// Cards acquired since the last upkeep skip their roll.
    fn upkeep_actions(&mut self, seat: usize) -> Flow<()> {
        let mut pending: Vec<CardId> = Vec::new();
        for held in &mut self.state.investigators[seat].cards {
            if held.fresh {
                held.fresh = false;
            } else {
                pending.push(held.card.clone());
            }
        }
        pending.retain(|card| {
            self.card(card)
                .is_ok_and(|printed| printed.abilities.iter().any(is_mandatory))
        });
        pending.dedup();
        while !pending.is_empty() {
            let options: Vec<DecisionOption> = pending
                .iter()
                .map(|card| option(Choice::Card(card.clone()), self.card_name(card)))
                .collect();
            let Choice::Card(card) = self.ask(
                seat,
                DecisionKind::UpkeepOrder,
                "Action d'Entretien suivante",
                options,
            )?
            else {
                break;
            };
            pending.retain(|other| other != &card);
            if !self.inv(seat).holds(&card) {
                continue;
            }
            let printed = self.card(&card)?;
            for ability in printed.abilities.iter().filter(|a| is_mandatory(a)) {
                self.covered.insert(crate::coverage::used_key(&card));
                self.log(format!(
                    "Entretien de {} : {}.",
                    self.name(seat),
                    printed.name
                ));
                self.apply_effects(seat, &ability.effects, Some(&card))?;
            }
        }
        let prompt = "Utiliser une capacité d'Entretien ?";
        self.offer_abilities(seat, &|timing| *timing == Timing::Upkeep, prompt)?;
        Ok(())
    }

    /// Moves the sliders, one stop at a time, within the focus; `after`
    /// adjust next.
    fn adjust_skills(&mut self, seat: usize, after: &[usize]) -> Flow<()> {
        let mut focus = self.sheet(seat)?.focus;
        while focus > 0 {
            let mut options = vec![option(Choice::Done, "Terminer l'ajustement")];
            options.extend(self.slider_moves(seat));
            let prompt = format!("Ajustement des compétences (Concentration restante : {focus})");
            let context = Context::Focus {
                remaining: focus,
                after: after.to_vec(),
            };
            let Choice::Slider { track, stop } =
                self.ask_about(seat, DecisionKind::AdjustSkills, context, prompt, options)?
            else {
                return Ok(());
            };
            self.inv_mut(seat).sliders[track] = stop;
            focus -= 1;
        }
        Ok(())
    }

    /// Every one-stop slider move.
    fn slider_moves(&self, seat: usize) -> Vec<DecisionOption> {
        let sliders = self.inv(seat).sliders;
        let mut moves = Vec::new();
        for (track, name) in TRACKS.iter().enumerate() {
            let stop = sliders[track];
            if stop > 0 {
                moves.push(option(
                    Choice::Slider {
                        track,
                        stop: stop - 1,
                    },
                    format!("{name} : vers la gauche"),
                ));
            }
            if stop < 3 {
                moves.push(option(
                    Choice::Slider {
                        track,
                        stop: stop + 1,
                    },
                    format!("{name} : vers la droite"),
                ));
            }
        }
        moves
    }

    /// Initial slider placement: any stop on each track, focus ignored.
    pub fn place_sliders(&mut self, seat: usize) -> Flow<()> {
        for (track, name) in TRACKS.iter().enumerate() {
            let options = (0..4)
                .map(|stop| {
                    option(
                        Choice::Slider { track, stop },
                        format!("{name} : cran {}", stop + 1),
                    )
                })
                .collect();
            let prompt = format!("Placement initial du curseur {name}");
            if let Choice::Slider { stop, .. } =
                self.ask(seat, DecisionKind::PlaceSliders, prompt, options)?
            {
                self.inv_mut(seat).sliders[track] = stop;
            }
        }
        Ok(())
    }
}

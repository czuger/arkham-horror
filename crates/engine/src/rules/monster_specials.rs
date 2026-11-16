//! Monster texts coded by name (« effets spéciaux » of the monster tokens).

use data::{CheckKind, Deck, MonsterId, Resource};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::{Flow, Halt};
use crate::rules::combat::{Combat, Faced};
use crate::state::Place;

impl Game<'_> {
    /// Checks some Mask monsters ask for before the Horror check. Returns how
    /// the combat ended when it ends there.
    pub fn before_horror(&mut self, seat: usize, combat: &mut Combat) -> Flow<Option<Faced>> {
        match combat.monster.as_str() {
            "the_bloated_woman" => {
                combat.doomed = !self.check(seat, CheckKind::Will, -2, 1)?.passed;
                Ok(None)
            }
            "the_black_man" => {
                if self.check(seat, CheckKind::Luck, -1, 1)?.passed {
                    self.gain(seat, Resource::Clues, 2)?;
                    Ok(Some(Faced::Fled))
                } else {
                    self.devour(seat)?;
                    Err(Halt::KnockedOut(seat))
                }
            }
            _ => Ok(None),
        }
    }

    /// Monsters whose failed check does something else than Stamina loss.
    /// Returns whether the hit was handled here.
    pub fn monster_special_hit(&mut self, seat: usize, monster: &MonsterId) -> Flow<bool> {
        match monster.as_str() {
            "dimensional_shambler" => {
                self.lose_in_time_and_space(seat)?;
                Err(Halt::KnockedOut(seat))
            }
            "nightgaunt" => self.nightgaunt_carries(seat),
            "elder_thing" => {
                self.elder_thing_takes(seat)?;
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    /// Nightgaunt: drawn through the nearest open gate (FAQ: in an Other
    /// World, back to Arkham with an explored marker; no gate, no effect).
    fn nightgaunt_carries(&mut self, seat: usize) -> Flow<bool> {
        let place = self.inv(seat).place.clone();
        if let Place::OtherWorld { .. } = place {
            self.return_to_arkham(seat)?;
            return Err(Halt::KnockedOut(seat));
        }
        let mut nearest: Option<(usize, data::LocationId)> = None;
        for gate in &self.state.gates {
            let distance = self.distance(&place, &Place::Location(gate.location.clone()));
            if let Some(distance) = distance
                && nearest.as_ref().is_none_or(|(best, _)| distance < *best)
            {
                nearest = Some((distance, gate.location.clone()));
            }
        }
        let Some((_, location)) = nearest else {
            return Ok(true);
        };
        self.draw_through_gate(seat, &location, false)?;
        Err(Halt::KnockedOut(seat))
    }

    /// Elder Thing: a failed Combat check costs a weapon or spell, if any.
    fn elder_thing_takes(&mut self, seat: usize) -> Flow<()> {
        let mut options = Vec::new();
        for held in &self.inv(seat).cards {
            let card = self.card(&held.card)?;
            if card.weapon.is_some() || card.deck == Deck::Spell {
                options.push(option(Choice::Card(held.card.clone()), card.name.clone()));
            }
        }
        if options.is_empty() {
            return Ok(());
        }
        let prompt = "La Chose Très Ancienne vous prend une Arme ou un Sort.";
        if let Choice::Card(card) = self.ask(seat, DecisionKind::PickCard, prompt, options)? {
            self.discard_card(seat, &card)?;
        }
        Ok(())
    }

    /// Where a defeated monster goes: Mi-Go and Warlock go back to the box
    /// with a reward, the others become trophies.
    pub fn monster_defeated_special(&mut self, seat: usize, monster: &MonsterId) -> Flow<()> {
        match monster.as_str() {
            "mi_go" => {
                self.log("Le Mi-Go est remis dans la boîte.");
                self.draw_and_give(seat, Deck::UniqueItem)?;
                Ok(())
            }
            "warlock" => {
                self.log("Le Sorcier est remis dans la boîte.");
                self.gain(seat, Resource::Clues, 2)
            }
            _ => self.defeat_monster(seat, monster.clone()),
        }
    }
}

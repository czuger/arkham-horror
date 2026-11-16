//! The doom track, the Terror level and the awakening.

use data::{Deck, LocationId};

use crate::game::Game;
use crate::halt::{Flow, Halt};
use crate::state::{MonsterPlace, Place};

impl Game<'_> {
    /// Adds a doom token. A full track wakes the Ancient One.
    pub fn add_doom(&mut self) -> Flow<()> {
        let track = self.ancient_one()?.doom_track;
        if self.state.doom < track {
            self.state.doom += 1;
            self.log(format!("Pion Destin : {}/{track}.", self.state.doom));
        }
        if self.state.battle.is_none() && self.state.doom >= track {
            return self.awaken("l'échelle du destin est pleine");
        }
        Ok(())
    }

    /// Wakes the Ancient One: the turn stops at once.
    pub fn awaken<T>(&mut self, reason: &str) -> Flow<T> {
        if self.state.battle.is_some() {
            return Err(Halt::Broken(format!(
                "réveil pendant la bataille : {reason}"
            )));
        }
        self.log(format!("Le Grand Ancien se réveille : {reason} !"));
        Err(Halt::Awakened)
    }

    /// Raises the Terror level by one, with all its consequences.
    pub fn raise_terror(&mut self) -> Flow<()> {
        if self.state.terror >= 10 {
            return self.add_doom();
        }
        self.state.terror += 1;
        self.log(format!("Niveau de Terreur : {}.", self.state.terror));
        if !self.state.decks.get(Deck::Ally).is_empty() {
            let index = self.state.rng.below(self.state.decks.allies.len());
            let ally = self.state.decks.allies.remove(index);
            self.log(format!("{} quitte la ville.", self.card_name(&ally)));
        }
        self.close_terror_locations()?;
        if self.state.terror == 10 {
            self.log("Arkham est envahie : plus de limite de monstres.");
            self.clear_outskirts();
            self.add_doom()?;
        }
        Ok(())
    }

    /// Closes the locations whose Terror threshold is reached; whoever is
    /// inside goes to the street.
    fn close_terror_locations(&mut self) -> Flow<()> {
        let terror = self.state.terror;
        let closing: Vec<_> = self
            .data
            .board
            .locations
            .iter()
            .filter(|location| location.closes_at_terror == Some(terror))
            .collect();
        for location in closing {
            self.log(format!("{} ferme définitivement.", location.name));
            self.close_location(&location.id, &location.street)?;
        }
        Ok(())
    }

    /// Closes a location and moves investigators and monsters to its street.
    pub fn close_location(&mut self, id: &LocationId, street: &data::StreetId) -> Flow<()> {
        self.state.closed.push(id.clone());
        let here = Place::Location(id.clone());
        for seat in 0..self.state.investigators.len() {
            if self.inv(seat).place == here {
                self.move_to(seat, Place::Street(street.clone()));
            }
        }
        for monster in &mut self.state.monsters {
            if monster.place == MonsterPlace::Location(id.clone()) {
                monster.place = MonsterPlace::Street(street.clone());
            }
        }
        Ok(())
    }

    /// Checks the awakening conditions tied to gates and monsters.
    pub fn check_monster_awakening(&mut self) -> Flow<()> {
        if self.state.battle.is_some() {
            return Ok(());
        }
        let players = self.state.players();
        if self.state.terror >= 10 && self.monsters_in_arkham() >= 2 * (players + 3) {
            return self.awaken("Arkham est submergée de monstres");
        }
        Ok(())
    }

    /// Checks the awakening condition on the number of open gates.
    pub fn check_gate_awakening(&mut self) -> Flow<()> {
        let limit = match self.state.players() {
            0..=2 => 8,
            3..=4 => 7,
            5..=6 => 6,
            _ => 5,
        };
        if self.state.battle.is_none() && self.state.gates.len() >= limit {
            return self.awaken("trop de portails sont ouverts");
        }
        Ok(())
    }
}

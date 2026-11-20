//! Opening, closing and sealing gates.

use data::{LocationId, Stability};

use crate::game::Game;
use crate::halt::Flow;
use crate::state::{Ending, MonsterPlace, OpenGate, Place};

/// How a gate came to be closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Closing {
    /// Closed by a check: the investigator keeps the marker.
    Closed,
    /// Closed and sealed with clue tokens.
    SealedWithClues,
    /// Sealed with an Elder Sign card: one doom token leaves the track.
    SealedWithElderSign,
    /// Closed by a card that gives no trophy: the marker goes back to the stack.
    NoTrophy,
}

impl Game<'_> {
    /// Opens a new gate on a location: doom token, gate marker, clue tokens
    /// discarded, investigators drawn in and delayed, then `monsters` monsters.
    pub fn open_gate(&mut self, location: &LocationId, monsters: usize) -> Flow<()> {
        self.add_doom()?;
        if self.state.gate_stack.is_empty() {
            return self.awaken("il n'y a plus de marqueur Portail");
        }
        let marker = self.state.gate_stack.remove(0);
        self.state.gates.push(OpenGate {
            location: location.clone(),
            marker,
        });
        self.state.clues.remove(location);
        let world = self
            .data
            .gate_markers
            .markers
            .get(marker)
            .map(|m| m.world.clone());
        let world_name = match &world {
            Some(id) => self.world(id)?.name.clone(),
            None => String::new(),
        };
        self.log(format!(
            "Un portail vers {world_name} s'ouvre à {}.",
            self.location(location)?.name
        ));
        let here = Place::Location(location.clone());
        for seat in self.state.seat_order() {
            if self.inv(seat).place == here {
                self.draw_through_gate(seat, location, true)?;
            }
        }
        self.check_gate_awakening()?;
        for _ in 0..monsters {
            let monster = self.draw_monster()?;
            self.place_monster(monster, MonsterPlace::Location(location.clone()))?;
        }
        Ok(())
    }

    /// Closes the gate on a location. Monsters of its symbol go back to the
    /// cup (Outskirts included). Checks the victory conditions.
    pub fn close_gate(&mut self, seat: usize, location: &LocationId, how: Closing) -> Flow<()> {
        let Some(index) = self
            .state
            .gates
            .iter()
            .position(|gate| &gate.location == location)
        else {
            return Ok(());
        };
        let gate = self.state.gates.remove(index);
        if how == Closing::NoTrophy {
            self.state.gate_stack.push(gate.marker);
        } else {
            self.inv_mut(seat).gate_trophies.push(gate.marker);
            self.inv_mut(seat).explored = false;
        }
        self.log(format!(
            "{} ferme le portail de {}.",
            self.name(seat),
            self.location(location)?.name
        ));
        self.return_monsters_of_gate(gate.marker)?;
        if matches!(how, Closing::SealedWithClues | Closing::SealedWithElderSign) {
            self.seal(location, how == Closing::SealedWithElderSign)?;
        }
        self.check_gate_victory()
    }

    /// Sends the monsters sharing the gate's symbol back to the cup.
    fn return_monsters_of_gate(&mut self, marker: usize) -> Flow<()> {
        let Some(symbol) = self.data.gate_markers.markers.get(marker).map(|m| m.symbol) else {
            return Ok(());
        };
        let mut kept = Vec::new();
        for monster in std::mem::take(&mut self.state.monsters) {
            if self.monster(&monster.monster)?.symbol == symbol {
                self.log(format!(
                    "{} retourne dans la réserve.",
                    self.monster_name(&monster.monster)
                ));
                self.state.monster_cup.push(monster.monster);
            } else {
                kept.push(monster);
            }
        }
        self.state.monsters = kept;
        Ok(())
    }

    /// Puts an Elder Sign on a location. Taken from the doom track for an
    /// Elder Sign card, from the pool otherwise.
    fn seal(&mut self, location: &LocationId, from_track: bool) -> Flow<()> {
        if from_track {
            self.state.doom = self.state.doom.saturating_sub(1);
        }
        self.state.seals.push(location.clone());
        self.log(format!(
            "Le lieu {} est scellé.",
            self.location(location)?.name
        ));
        if self.state.seals.len() >= 6 {
            return self.end(Ending::GatesSealed);
        }
        Ok(())
    }

    /// Whether a gate can be sealed: only on an unstable location.
    pub fn can_be_sealed(&self, location: &LocationId) -> Flow<bool> {
        Ok(self.location(location)?.stability == Stability::Unstable
            && !self.mythos_has(|effect| *effect == data::Ongoing::NoSealing))
    }

    /// Victory by closing: no gate left and at least as many gate trophies
    /// as players.
    fn check_gate_victory(&mut self) -> Flow<()> {
        if !self.state.gates.is_empty() {
            return Ok(());
        }
        let trophies: usize = self
            .state
            .investigators
            .iter()
            .map(|inv| inv.gate_trophies.len())
            .sum();
        if trophies >= self.state.players() {
            return self.end(Ending::GatesClosed);
        }
        Ok(())
    }

    /// Ends the game.
    pub fn end<T>(&mut self, ending: Ending) -> Flow<T> {
        self.state.ending = Some(ending);
        self.log(ending.label());
        Err(crate::halt::Halt::Over)
    }
}

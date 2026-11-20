//! Effect primitives on the board: monsters, gates, trophies, doom, Terror.

use data::{Effect, MonsterArea, MonsterId};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::{Flow, Halt};
use crate::rules::gates::Closing;
use crate::state::{MonsterPlace, Place};

impl Game<'_> {
    /// Monsters, gates and the Ancient One's tracks.
    pub fn apply_board_effect(&mut self, seat: usize, effect: &Effect) -> Flow<()> {
        match effect {
            Effect::MonsterAppears => self.encounter_monster(seat),
            Effect::GateAndMonsterAppear => self.gate_appears_here(seat, 1),
            Effect::GateAppears => self.gate_appears_here(seat, 0),
            Effect::MonstersInStreet { street, count } => {
                for _ in 0..*count {
                    let monster = self.draw_monster()?;
                    self.place_monster(monster, MonsterPlace::Street(street.clone()))?;
                }
                Ok(())
            }
            Effect::ReturnMonsters {
                area,
                only,
                terror_if_any,
            } => self.return_monsters(area, only, *terror_if_any),
            Effect::CloseEntryGate => match self.inv(seat).entered_gate.clone() {
                Some(location) => self.close_gate(seat, &location, Closing::NoTrophy),
                None => Ok(()),
            },
            Effect::CloseGateOfChoice => self.close_gate_of_choice(seat),
            _ => self.apply_track_effect(seat, effect),
        }
    }

    /// Trophies, doom, Terror, Elder Signs and the Rumor.
    fn apply_track_effect(&mut self, seat: usize, effect: &Effect) -> Flow<()> {
        match effect {
            Effect::TrophyFromCup => self.trophy_from_cup(seat),
            Effect::TrophyFromBoard => return self.trophy_from_board(seat),
            Effect::SpendMonsterTrophies(toughness) => {
                return self.spend_monster_trophies(seat, *toughness);
            }
            Effect::SpendGateTrophies(count) => self.spend_gate_trophies(seat, *count),
            Effect::AddDoom => return self.add_doom(),
            Effect::RemoveDoom => {
                self.state.doom = self.state.doom.saturating_sub(1);
                self.log(format!("Un pion Destin est retiré : {}.", self.state.doom));
            }
            Effect::RaiseTerror(steps) => {
                for _ in 0..*steps {
                    self.raise_terror()?;
                }
            }
            Effect::TerrorTo(level) => self.raise_terror_to(*level)?,
            Effect::RemoveAllElderSigns => {
                self.state.seals.clear();
                self.log("Tous les Signes des Anciens quittent le plateau.");
            }
            Effect::AddMonstersToThis(count) => return self.add_monsters_to_rumor(*count),
            Effect::RumorPasses => return self.rumor_passes(seat),
            other => return Err(Halt::Broken(format!("effet non géré : {other:?}"))),
        }
        Ok(())
    }

    /// A gate opens on the investigator's location and draws them in,
    /// delayed, with `monsters` monsters.
    fn gate_appears_here(&mut self, seat: usize, monsters: usize) -> Flow<()> {
        let Place::Location(location) = self.inv(seat).place.clone() else {
            return Ok(());
        };
        if self.state.is_sealed(&location)
            || self.state.gate_at(&location).is_some()
            || self.stabilized_location(&location)
        {
            return Ok(());
        }
        self.open_gate(&location, monsters)?;
        Err(Halt::KnockedOut(seat))
    }

    /// Monsters of an area (of some kinds only) go back to the cup; the
    /// Terror may rise when at least one did.
    fn return_monsters(
        &mut self,
        area: &MonsterArea,
        only: &[MonsterId],
        terror: bool,
    ) -> Flow<()> {
        let mut kept = Vec::new();
        let mut returned = 0;
        for monster in std::mem::take(&mut self.state.monsters) {
            let kind = only.is_empty() || only.contains(&monster.monster);
            if kind && self.in_area(&monster.place, area)? {
                self.state.monster_cup.push(monster.monster);
                returned += 1;
            } else {
                kept.push(monster);
            }
        }
        self.state.monsters = kept;
        self.log(format!(
            "{returned} monstre(s) retourne(nt) dans la réserve."
        ));
        if terror && returned > 0 {
            self.raise_terror()?;
        }
        Ok(())
    }

    /// Whether a monster place lies in an area of the board.
    fn in_area(&self, place: &MonsterPlace, area: &MonsterArea) -> Flow<bool> {
        Ok(match (area, place) {
            (MonsterArea::Arkham, _) => true,
            (MonsterArea::Locations, MonsterPlace::Location(_)) => true,
            (MonsterArea::Streets, MonsterPlace::Street(_)) => true,
            (MonsterArea::SkyAndOutskirts, MonsterPlace::Sky | MonsterPlace::Outskirts) => true,
            (MonsterArea::District(district), MonsterPlace::Street(street)) => district == street,
            (MonsterArea::District(district), MonsterPlace::Location(location)) => {
                &self.location(location)?.street == district
            }
            _ => false,
        })
    }

    /// Closes an open gate chosen by the investigator.
    fn close_gate_of_choice(&mut self, seat: usize) -> Flow<()> {
        let mut options = Vec::new();
        for gate in &self.state.gates {
            let label = self.location(&gate.location)?.name.clone();
            options.push(option(Choice::Gate(gate.location.clone()), label));
        }
        if options.is_empty() {
            return Ok(());
        }
        let prompt = "Quel portail fermez-vous ?";
        let Choice::Gate(location) = self.ask(seat, DecisionKind::CloseGate, prompt, options)?
        else {
            return Ok(());
        };
        self.close_gate(seat, &location, Closing::NoTrophy)
    }
}

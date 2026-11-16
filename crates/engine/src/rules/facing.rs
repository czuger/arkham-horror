//! Facing monsters: on the board during movement, or drawn by an encounter.

use data::MonsterId;

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::combat::Faced;
use crate::state::{MonsterPlace, Place};

impl Game<'_> {
    /// « Un monstre apparaît ! »: evade or fight it; it never stays on the
    /// board and goes back to the cup unless defeated.
    pub fn encounter_monster(&mut self, seat: usize) -> Flow<()> {
        if let Some(location) = self.location_of(seat)
            && self.state.is_sealed(&location)
        {
            return Ok(());
        }
        let monster = self.draw_monster()?;
        self.log(format!(
            "Un monstre apparaît : {}.",
            self.monster_name(&monster)
        ));
        let options = vec![
            option(Choice::Evade(0), "S'échapper"),
            option(Choice::Fight(0), "Combattre"),
        ];
        let prompt = format!("{} vous fait face.", self.monster_name(&monster));
        let fight = self.ask(seat, DecisionKind::FaceMonster, prompt, options)? == Choice::Fight(0);
        let result = self.evade_or_fight(seat, &monster, fight);
        if result != Ok(Faced::Defeated) {
            self.state.monster_cup.push(monster);
        }
        result.map(|_| ())
    }

    /// Faces, in the player's order, every monster on a place that was not
    /// evaded yet. Returns `false` when a combat started (movement over).
    pub fn face_monsters_here(&mut self, seat: usize, evaded: &mut Vec<MonsterId>) -> Flow<bool> {
        let place = match &self.inv(seat).place {
            Place::Location(id) => MonsterPlace::Location(id.clone()),
            Place::Street(id) => MonsterPlace::Street(id.clone()),
            _ => return Ok(true),
        };
        let mut waiting: Vec<MonsterId> = self
            .monsters_at(&place)
            .into_iter()
            .map(|index| self.state.monsters[index].monster.clone())
            .collect();
        for gone in evaded.iter() {
            if let Some(index) = waiting.iter().position(|id| id == gone) {
                waiting.remove(index);
            }
        }
        let mut free = true;
        while !waiting.is_empty() {
            let (index, fight) = self.pick_monster(seat, &waiting)?;
            let monster = waiting.remove(index);
            match self.evade_or_fight(seat, &monster, fight)? {
                Faced::Evaded => evaded.push(monster),
                Faced::Defeated => {
                    self.remove_monster_at(&place, &monster);
                    free = false;
                }
                Faced::Fled => free = false,
            }
        }
        Ok(free)
    }

    /// Which monster to face next, and whether to fight it.
    fn pick_monster(&mut self, seat: usize, waiting: &[MonsterId]) -> Flow<(usize, bool)> {
        let mut options = Vec::new();
        for (index, monster) in waiting.iter().enumerate() {
            let name = self.monster_name(monster);
            options.push(option(Choice::Evade(index), format!("S'échapper : {name}")));
            options.push(option(Choice::Fight(index), format!("Combattre : {name}")));
        }
        match self.ask(
            seat,
            DecisionKind::FaceMonster,
            "Monstres à affronter",
            options,
        )? {
            Choice::Fight(index) => Ok((index, true)),
            Choice::Evade(index) => Ok((index, false)),
            _ => Ok((0, false)),
        }
    }

    /// Removes one board monster of a kind from a place.
    fn remove_monster_at(&mut self, place: &MonsterPlace, monster: &MonsterId) {
        let found = self
            .state
            .monsters
            .iter()
            .position(|candidate| &candidate.place == place && &candidate.monster == monster);
        if let Some(index) = found {
            self.state.monsters.remove(index);
        }
    }
}

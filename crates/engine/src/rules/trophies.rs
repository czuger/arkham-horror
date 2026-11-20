//! Monster and gate trophies: claimed and spent by card effects.

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;

impl Game<'_> {
    /// Total printed toughness of an investigator's monster trophies.
    pub fn trophy_toughness(&self, seat: usize) -> Flow<u32> {
        let mut total = 0;
        for monster in &self.inv(seat).monster_trophies {
            total += u32::from(self.monster(monster)?.toughness);
        }
        Ok(total)
    }

    /// Spends monster trophies, one at a time, until their toughness reaches
    /// `needed`. Spent monsters go back to the cup.
    pub fn spend_monster_trophies(&mut self, seat: usize, needed: u8) -> Flow<()> {
        let mut spent = 0;
        while spent < u32::from(needed) && !self.inv(seat).monster_trophies.is_empty() {
            let mut options = Vec::new();
            for (index, monster) in self.inv(seat).monster_trophies.iter().enumerate() {
                let label = format!(
                    "{} (force {})",
                    self.monster_name(monster),
                    self.monster(monster)?.toughness
                );
                options.push(option(Choice::Option(index), label));
            }
            let prompt = format!("Trophées dépensés : {spent} sur {needed}. Lequel dépenser ?");
            let Choice::Option(index) = self.ask(seat, DecisionKind::PickCard, prompt, options)?
            else {
                return Ok(());
            };
            if index >= self.inv(seat).monster_trophies.len() {
                return Ok(());
            }
            let monster = self.inv_mut(seat).monster_trophies.remove(index);
            spent += u32::from(self.monster(&monster)?.toughness);
            self.state.monster_cup.push(monster);
        }
        self.log(format!(
            "{} dépense des trophées de monstre ({spent} en force).",
            self.name(seat)
        ));
        Ok(())
    }

    /// Spends gate trophies: the markers go back to the stack.
    pub fn spend_gate_trophies(&mut self, seat: usize, count: u8) {
        for _ in 0..count {
            if self.inv(seat).gate_trophies.is_empty() {
                break;
            }
            let marker = self.inv_mut(seat).gate_trophies.remove(0);
            self.state.gate_stack.push(marker);
        }
        self.log(format!(
            "{} dépense {count} trophée(s) de portail.",
            self.name(seat)
        ));
    }

    /// A monster from the cup, taken as a trophy (Endless ones included).
    pub fn trophy_from_cup(&mut self, seat: usize) {
        if self.state.monster_cup.is_empty() {
            return;
        }
        let index = self.state.rng.below(self.state.monster_cup.len());
        let monster = self.state.monster_cup.remove(index);
        self.log(format!(
            "{} prend {} comme trophée.",
            self.name(seat),
            self.monster_name(&monster)
        ));
        self.inv_mut(seat).monster_trophies.push(monster);
    }

    /// A monster of the board, chosen by the investigator, taken as a trophy.
    pub fn trophy_from_board(&mut self, seat: usize) -> Flow<()> {
        let options: Vec<_> = self
            .state
            .monsters
            .iter()
            .enumerate()
            .map(|(index, monster)| {
                let label = format!(
                    "{} ({})",
                    self.monster_name(&monster.monster),
                    self.monster_place_name(&monster.place)
                );
                option(Choice::Option(index), label)
            })
            .collect();
        if options.is_empty() {
            return Ok(());
        }
        let prompt = "Quel monstre prendre comme trophée ?";
        let Choice::Option(index) = self.ask(seat, DecisionKind::PickCard, prompt, options)? else {
            return Ok(());
        };
        if index >= self.state.monsters.len() {
            return Ok(());
        }
        let monster = self.state.monsters.remove(index).monster;
        self.log(format!(
            "{} prend {} comme trophée.",
            self.name(seat),
            self.monster_name(&monster)
        ));
        self.inv_mut(seat).monster_trophies.push(monster);
        Ok(())
    }
}

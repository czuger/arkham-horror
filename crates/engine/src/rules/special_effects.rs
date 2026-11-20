//! Card effects coded by name (« effet spécial »).

use data::{CardId, MonsterId, SpecialId};

use crate::game::Game;
use crate::halt::{Flow, Halt};
use crate::rules::gates::Closing;
use crate::state::MonsterPlace;

impl Game<'_> {
    /// Runs a special effect by its id.
    pub fn special_effect(
        &mut self,
        seat: usize,
        id: &SpecialId,
        this: Option<&CardId>,
    ) -> Flow<()> {
        match id.as_str() {
            "no_more_bank_loans" => self.inv_mut(seat).no_more_bank_loans = true,
            "two_mythos_cards_each_turn" => {
                self.state.two_mythos_cards = true;
                self.log("Désormais, on pioche 2 cartes Mythe à chaque phase Mythe.");
            }
            "enchant_weapon" => self.scratch.enchant_weapon = true,
            "red_sign" => self.scratch.red_sign = true,
            "crystal_ball" => return self.crystal_ball(seat),
            "body_swap" => return self.swap_body(seat),
            "dimensional_ray" => return self.dimensional_ray(seat),
            "elder_things_return" => self.elder_things_return(),
            "rumor_clue_removal" => return self.rumor_clue_removal(seat, this),
            "experiment_fight" => return self.experiment_fight(seat),
            "experiment_release" => return self.experiment_release(),
            other => return Err(Halt::Broken(format!("effet spécial non codé : {other}"))),
        }
        Ok(())
    }

    /// One die per open gate: a success closes it, with no trophy.
    fn dimensional_ray(&mut self, seat: usize) -> Flow<()> {
        let gates: Vec<_> = self
            .state
            .gates
            .iter()
            .map(|gate| gate.location.clone())
            .collect();
        for location in gates {
            if self.roll_dice(seat, 1) > 0 {
                self.close_gate(seat, &location, Closing::NoTrophy)?;
            }
        }
        Ok(())
    }

    /// Elder Thing trophies come back to life on the River Docks.
    fn elder_things_return(&mut self) {
        let elder_thing = MonsterId::new("elder_thing");
        let mut back = 0;
        for inv in &mut self.state.investigators {
            let before = inv.monster_trophies.len();
            inv.monster_trophies
                .retain(|monster| monster != &elder_thing);
            back += before - inv.monster_trophies.len();
        }
        for _ in 0..back {
            self.state.monsters.push(crate::state::BoardMonster {
                monster: elder_thing.clone(),
                place: MonsterPlace::Location(data::LocationId::new("river_docks")),
            });
        }
        self.log(format!(
            "{back} Chose(s) Très Ancienne(s) revien(nen)t sur les Quais."
        ));
    }
}

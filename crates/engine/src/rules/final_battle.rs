//! The final battle against the Ancient One (§17).

use data::{CheckKind, CombatRating};

use crate::game::Game;
use crate::halt::{Flow, Halt};
use crate::rules::combat::Gear;
use crate::state::{BattleState, Ending, Phase, Place};

impl Game<'_> {
    /// From the awakening to the end of the game.
    pub fn final_battle(&mut self) -> Flow<()> {
        if self.awakening_is_defeat() {
            return self.end(Ending::Devoured);
        }
        self.state.phase = Phase::FinalBattle;
        self.state.battle = Some(BattleState::default());
        self.state.doom = self.ancient_one()?.doom_track;
        self.log(format!(
            "Bataille finale contre {} !",
            self.ancient_one()?.name
        ));
        if let Some(card) = self.state.environment.take() {
            self.state.mythos_deck.push(card);
        }
        if let Some(rumor) = self.state.rumor.take() {
            self.state.mythos_deck.push(rumor.card);
        }
        for seat in self.state.seat_order() {
            if self.inv(seat).place == Place::LostInTimeAndSpace {
                self.devour(seat)?;
            }
        }
        self.check_all_devoured()?;
        self.start_of_battle()?;
        self.check_all_devoured()?;
        loop {
            self.battle_round()?;
        }
    }

    /// One round: refresh, investigators attack, the Ancient One attacks.
    fn battle_round(&mut self) -> Flow<()> {
        let round = self.state.battle.as_ref().map_or(0, |battle| battle.round) + 1;
        if let Some(battle) = &mut self.state.battle {
            battle.round = round;
        }
        self.log(format!("Round {round}."));
        for seat in self.state.seat_order() {
            self.catch_knock_out(seat, |game| game.upkeep(seat))?;
        }
        self.state.first_player = (self.state.first_player + 1) % self.state.players();
        self.check_all_devoured()?;
        for seat in self.state.seat_order() {
            self.catch_knock_out(seat, |game| game.attack_ancient_one(seat))?;
        }
        for seat in self.state.seat_order() {
            self.catch_knock_out(seat, |game| game.ancient_one_attacks(seat, round))?;
        }
        if self.state.ancient_one.as_str() == "cthulhu" {
            self.add_doom()?;
        }
        self.check_all_devoured()
    }

    /// One investigator's Combat check against the Ancient One. Successes
    /// add up for the group; each « players » successes remove a doom token.
    fn attack_ancient_one(&mut self, seat: usize) -> Flow<()> {
        let ancient = self.ancient_one()?;
        let rating = match ancient.combat_rating {
            CombatRating::Fixed(value) => i32::from(value),
            CombatRating::MinusTerror => -i32::from(self.state.terror),
            CombatRating::Infinite => return Ok(()),
        };
        let mut gear = Gear {
            defenses: ancient.defenses.clone(),
            ..Gear::default()
        };
        self.prepare_combat(seat, &mut gear)?;
        let players = u32::try_from(self.state.players()).unwrap_or(1);
        let kept = self
            .state
            .battle
            .as_ref()
            .map_or(0, |battle| battle.successes);
        let needed = (u32::from(self.state.doom) * players).saturating_sub(kept);
        let needed = u8::try_from(needed).unwrap_or(u8::MAX);
        let result = self.check(seat, CheckKind::Combat, rating + gear.bonus(), needed)?;
        self.discard_used_weapons(seat, &mut gear)?;
        let mut successes = kept + result.successes;
        while successes >= players && self.state.doom > 0 {
            successes -= players;
            self.state.doom -= 1;
            self.log(format!(
                "Un pion Destin est retiré : il en reste {}.",
                self.state.doom
            ));
        }
        if let Some(battle) = &mut self.state.battle {
            battle.successes = successes;
        }
        if self.state.doom == 0 {
            return self.end(Ending::AncientOneBanished);
        }
        Ok(())
    }

    /// One attack on the Ancient One, for the unit tests.
    #[cfg(test)]
    pub fn attack_ancient_one_for_tests(&mut self, seat: usize) -> Flow<()> {
        self.attack_ancient_one(seat)
    }

    /// Ends the game when no investigator is left.
    fn check_all_devoured(&mut self) -> Flow<()> {
        if self.state.investigators.iter().all(|inv| inv.eliminated) {
            return self.end(Ending::Devoured);
        }
        Ok(())
    }

    /// Eliminates an investigator during the battle.
    pub fn eliminate(&mut self, seat: usize) -> Flow<()> {
        self.devour(seat)?;
        Err(Halt::KnockedOut(seat))
    }
}

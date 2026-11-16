//! Start-of-battle effects and attacks of the Ancient Ones, coded by name.

use data::{CheckKind, Resource};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;

/// The Curse card.
const CURSE: &str = "curse";

impl Game<'_> {
    /// Start-of-battle effects.
    pub fn start_of_battle(&mut self) -> Flow<()> {
        let ancient = self.state.ancient_one.as_str().to_owned();
        for seat in self.state.seat_order() {
            let inv = self.inv(seat);
            let devoured = match ancient.as_str() {
                "nyarlathotep" => inv.clues == 0,
                "shub_niggurath" => inv.monster_trophies.is_empty(),
                "yog_sothoth" => inv.gate_trophies.is_empty(),
                "yig" => inv.holds(&data::CardId::new(CURSE)),
                _ => false,
            };
            if devoured {
                self.catch_knock_out(seat, |game| game.eliminate(seat))?;
                continue;
            }
            match ancient.as_str() {
                "yig" => self.curse(seat)?,
                "ithaqua" => self.ithaqua_strips(seat)?,
                _ => {}
            }
        }
        Ok(())
    }

    /// Gives the Curse card.
    fn curse(&mut self, seat: usize) -> Flow<()> {
        if let Some(card) = self.take_from_deck(&data::CardId::new(CURSE))? {
            self.give_card(seat, card)?;
        }
        Ok(())
    }

    /// Ithaqua: one die per item, discarded on a failure.
    fn ithaqua_strips(&mut self, seat: usize) -> Flow<()> {
        for card in self.held_items(seat)? {
            if self.roll_dice(seat, 1) == 0 {
                self.log(format!(
                    "{} perd {}.",
                    self.name(seat),
                    self.card_name(&card)
                ));
                self.discard_card(seat, &card)?;
            }
        }
        Ok(())
    }

    /// The Ancient One's attack on one investigator. The check modifier
    /// starts at +1 and drops by 1 each round.
    pub fn ancient_one_attacks(&mut self, seat: usize, round: u32) -> Flow<()> {
        let modifier = 2 - i32::try_from(round).unwrap_or(i32::MAX);
        let ancient = self.state.ancient_one.as_str().to_owned();
        let (kind, loss) = match ancient.as_str() {
            "cthulhu" => return self.cthulhu_attacks(seat),
            "hastur" => (CheckKind::Luck, Loss::Resource(Resource::Sanity, 2)),
            "ithaqua" => (CheckKind::Fight, Loss::Resource(Resource::Stamina, 2)),
            "nyarlathotep" => (CheckKind::Lore, Loss::Clue),
            "shub_niggurath" => (CheckKind::Sneak, Loss::MonsterTrophy),
            "yig" => (CheckKind::Speed, Loss::Both),
            "yog_sothoth" => (CheckKind::Will, Loss::GateTrophy),
            _ => return Ok(()),
        };
        if self.check(seat, kind, modifier, 1)?.passed {
            return Ok(());
        }
        self.attack_loss(seat, loss)
    }

    /// What a failed attack check costs.
    fn attack_loss(&mut self, seat: usize, loss: Loss) -> Flow<()> {
        let inv = self.inv_mut(seat);
        let emptied = match loss {
            Loss::Resource(resource, amount) => return self.lose(seat, resource, amount),
            Loss::Both => {
                return self.lose_both(seat, Resource::Sanity, 1, Some((Resource::Stamina, 1)));
            }
            Loss::Clue => {
                inv.clues = inv.clues.saturating_sub(1);
                inv.clues == 0
            }
            Loss::MonsterTrophy => {
                inv.monster_trophies.pop();
                inv.monster_trophies.is_empty()
            }
            Loss::GateTrophy => {
                inv.gate_trophies.pop();
                inv.gate_trophies.is_empty()
            }
        };
        if emptied {
            return self.eliminate(seat);
        }
        Ok(())
    }

    /// Cthulhu: lower the maximum Sanity or Stamina by 1, then a doom token.
    fn cthulhu_attacks(&mut self, seat: usize) -> Flow<()> {
        let options = vec![
            option(Choice::Option(0), "Maximum de Santé Mentale −1"),
            option(Choice::Option(1), "Maximum de Résistance −1"),
        ];
        let choice = self.ask(seat, DecisionKind::CardChoice, "Cthulhu attaque.", options)?;
        let inv = self.inv_mut(seat);
        if choice == Choice::Option(0) {
            inv.max_sanity_change -= 1;
        } else {
            inv.max_stamina_change -= 1;
        }
        let sanity = self
            .maximum(seat, Resource::Sanity)?
            .min(self.inv(seat).sanity);
        let stamina = self
            .maximum(seat, Resource::Stamina)?
            .min(self.inv(seat).stamina);
        let inv = self.inv_mut(seat);
        inv.sanity = sanity;
        inv.stamina = stamina;
        self.after_loss(seat)
    }
}

/// What a failed attack check costs.
#[derive(Debug, Clone, Copy)]
enum Loss {
    Resource(Resource, u32),
    Both,
    Clue,
    MonsterTrophy,
    GateTrophy,
}

//! Effect primitives about cards, places, monsters and the card itself.

use data::{CardId, Effect, LocationId, SpecialId};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::{Flow, Halt};
use crate::state::{MonsterPlace, Place};

impl<'a> Game<'a> {
    /// Effects that are neither resources nor structure.
    pub fn apply_other_effect(
        &mut self,
        seat: usize,
        effect: &'a Effect,
        this: Option<&CardId>,
    ) -> Flow<()> {
        match effect {
            Effect::Draw { deck, count, keep } => self.draw_and_keep(seat, *deck, *count, *keep),
            Effect::TakeCard { card, otherwise } => {
                if self.may_receive(seat, card)
                    && let Some(taken) = self.take_from_deck(card)?
                {
                    return self.give_card(seat, taken);
                }
                self.apply_effects(seat, otherwise, this)
            }
            Effect::SearchDeck(deck) => self.search_deck(seat, *deck),
            Effect::Shop(decks) => self.shop(seat, decks),
            Effect::Sell { deck, multiplier } => self.sell(seat, *deck, *multiplier),
            Effect::Discard { cards, count } => self.discard_some(seat, cards, *count),
            Effect::DiscardAll(cards) => self.discard_all(seat, cards),
            Effect::DiscardThis => match this {
                Some(card) => self.discard_card(seat, card),
                None => Ok(()),
            },
            Effect::ReturnThisToBox => {
                if let Some(card) = this {
                    self.return_to_box(seat, card);
                }
                Ok(())
            }
            Effect::AddTokensToThis(count) => {
                if let Some(rumor) = &mut self.state.rumor {
                    rumor.tokens += count;
                    let tokens = rumor.tokens;
                    self.log(format!("La Rumeur porte {tokens} pion(s)."));
                }
                Ok(())
            }
            Effect::CombatBonus(bonus) => {
                self.scratch.combat_bonus += i32::from(*bonus);
                Ok(())
            }
            Effect::ReduceLoss(amount) => {
                self.scratch.reduce_loss += u32::from(*amount);
                Ok(())
            }
            Effect::Reroll => {
                self.scratch.reroll = true;
                Ok(())
            }
            Effect::Special(id) => self.special_effect(seat, id),
            _ => self.apply_place_effect(seat, effect),
        }
    }

    /// Effects that move the investigator or bring monsters and gates.
    fn apply_place_effect(&mut self, seat: usize, effect: &Effect) -> Flow<()> {
        match effect {
            Effect::MoveToStreet => {
                if let Place::Location(id) = &self.inv(seat).place {
                    let street = self.location(id)?.street.clone();
                    self.move_to(seat, Place::Street(street));
                }
                Ok(())
            }
            Effect::MoveAnywhereInArkham => self.move_anywhere(seat),
            Effect::ReturnToArkham => self.return_to_arkham(seat),
            Effect::Arrested => {
                self.arrest(seat);
                Err(Halt::KnockedOut(seat))
            }
            Effect::LostInTimeAndSpace => {
                self.lose_in_time_and_space(seat)?;
                Err(Halt::KnockedOut(seat))
            }
            Effect::Delayed => {
                self.inv_mut(seat).delayed = true;
                self.log(format!("{} est retardé.", self.name(seat)));
                Ok(())
            }
            Effect::LoseNextTurn => {
                self.inv_mut(seat).lose_next_turn = true;
                self.log(format!("{} perdra son prochain tour.", self.name(seat)));
                Ok(())
            }
            Effect::MonsterAppears => self.encounter_monster(seat),
            Effect::GateAndMonsterAppear => self.gate_appears_here(seat),
            Effect::MonstersInStreet { street, count } => {
                for _ in 0..*count {
                    let monster = self.draw_monster()?;
                    self.place_monster(monster, MonsterPlace::Street(street.clone()))?;
                }
                Ok(())
            }
            other => Err(Halt::Broken(format!("effet non géré : {other:?}"))),
        }
    }

    /// Goes to any location or street of Arkham, player's choice.
    pub fn move_anywhere(&mut self, seat: usize) -> Flow<()> {
        let mut options = Vec::new();
        for place in self.arkham_places() {
            let label = self.place_name(&place);
            options.push(option(Choice::Place(place), label));
        }
        if let Choice::Place(place) =
            self.ask(seat, DecisionKind::Destination, "Où allez-vous ?", options)?
        {
            self.log(format!(
                "{} va à {}.",
                self.name(seat),
                self.place_name(&place)
            ));
            self.move_to(seat, place);
        }
        Ok(())
    }

    /// « Un portail et un monstre apparaissent ! » on the investigator's
    /// location: the gate draws them in, delayed; the monster stays.
    fn gate_appears_here(&mut self, seat: usize) -> Flow<()> {
        let Place::Location(location) = self.inv(seat).place.clone() else {
            return Ok(());
        };
        if self.state.is_sealed(&location) || self.state.gate_at(&location).is_some() {
            return Ok(());
        }
        self.open_gate(&location, 1)?;
        Err(Halt::KnockedOut(seat))
    }

    /// Special effects coded by name.
    fn special_effect(&mut self, seat: usize, id: &SpecialId) -> Flow<()> {
        match id.as_str() {
            "no_more_bank_loans" => {
                self.inv_mut(seat).no_more_bank_loans = true;
                Ok(())
            }
            "two_mythos_cards_each_turn" => {
                self.state.two_mythos_cards = true;
                self.log("Désormais, on pioche 2 cartes Mythe à chaque phase Mythe.");
                Ok(())
            }
            other => Err(Halt::Broken(format!("effet spécial non codé : {other}"))),
        }
    }

    /// The location of an investigator, if in one.
    pub fn location_of(&self, seat: usize) -> Option<LocationId> {
        self.inv(seat).place.location().cloned()
    }
}

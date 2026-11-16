//! Setting up a game (§3) and bringing in a new investigator.

use data::{CardId, Deck, InvestigatorId, MonsterAbility, Resource, Stability};

use crate::game::Game;
use crate::halt::{Flow, Halt};
use crate::setup::Setup;
use crate::state::{InvestigatorState, Phase, Place};

impl<'a> Game<'a> {
    /// The whole setup, up to and including the first Mythos card.
    pub fn prepare(&mut self, setup: &Setup) -> Flow<()> {
        self.state.phase = Phase::Setup;
        self.fill_decks();
        self.fill_cup();
        for location in &self.data.board.locations {
            if location.stability == Stability::Unstable {
                self.state.clues.insert(location.id.clone(), 1);
            }
        }
        let chosen = self.pick_investigators(setup)?;
        self.state.first_player = self.state.rng.below(chosen.len());
        for id in chosen {
            self.state.investigators.push(self.blank_investigator(id)?);
        }
        for seat in self.state.seat_order() {
            self.bring_in(seat)?;
        }
        for seat in self.state.seat_order() {
            self.place_sliders(seat)?;
        }
        self.first_mythos()
    }

    /// Shuffled decks, monster-free: investigator cards, Mythos, gate cards and markers.
    fn fill_decks(&mut self) {
        for card in &self.data.investigator_cards.cards {
            let deck = self.state.decks.get_mut(card.deck);
            deck.extend(std::iter::repeat_n(
                card.id.clone(),
                usize::from(card.copies),
            ));
        }
        for deck in [
            Deck::CommonItem,
            Deck::UniqueItem,
            Deck::Spell,
            Deck::Skill,
            Deck::Ally,
            Deck::Special,
        ] {
            let mut cards = std::mem::take(self.state.decks.get_mut(deck));
            self.state.rng.shuffle(&mut cards);
            *self.state.decks.get_mut(deck) = cards;
        }
        let mut mythos: Vec<CardId> = self
            .data
            .mythos_cards
            .cards
            .iter()
            .map(|card| card.id.clone())
            .collect();
        self.state.rng.shuffle(&mut mythos);
        self.state.mythos_deck = mythos;
        let mut gates: Vec<CardId> = self
            .data
            .gate_cards
            .cards
            .iter()
            .map(|card| card.id.clone())
            .collect();
        self.state.rng.shuffle(&mut gates);
        self.state.gate_deck = gates;
        let mut markers: Vec<usize> = Vec::new();
        for (index, marker) in self.data.gate_markers.markers.iter().enumerate() {
            markers.extend(std::iter::repeat_n(index, usize::from(marker.copies)));
        }
        self.state.rng.shuffle(&mut markers);
        self.state.gate_stack = markers;
    }

    /// The monster cup; Masks only for Nyarlathotep.
    fn fill_cup(&mut self) {
        let masks = self.masks_in_play();
        for monster in &self.data.monsters.monsters {
            if masks || !monster.abilities.contains(&MonsterAbility::Mask) {
                let copies = usize::from(monster.copies);
                self.state
                    .monster_cup
                    .extend(std::iter::repeat_n(monster.id.clone(), copies));
            }
        }
    }

    /// Chosen investigators, the other seats drawn among the unused sheets.
    fn pick_investigators(&mut self, setup: &Setup) -> Flow<Vec<InvestigatorId>> {
        let mut unused: Vec<InvestigatorId> = self
            .data
            .investigators
            .investigators
            .iter()
            .map(|s| s.id.clone())
            .collect();
        let mut chosen = Vec::new();
        for id in &setup.investigators {
            let index = unused.iter().position(|candidate| candidate == id);
            let Some(index) = index else {
                return Err(Halt::Broken(format!(
                    "investigateur « {id} » inconnu ou en double"
                )));
            };
            chosen.push(unused.remove(index));
        }
        while chosen.len() < setup.players && !unused.is_empty() {
            let index = self.state.rng.below(unused.len());
            chosen.push(unused.remove(index));
        }
        self.state.unused_investigators = unused;
        Ok(chosen)
    }

    /// An investigator at home with the printed Sanity, Stamina, money and clues.
    fn blank_investigator(&self, id: InvestigatorId) -> Flow<InvestigatorState> {
        let sheet = self
            .data
            .investigators
            .investigators
            .iter()
            .find(|sheet| sheet.id == id)
            .ok_or_else(|| Halt::Broken(format!("investigateur « {id} » introuvable")))?;
        Ok(InvestigatorState {
            id,
            place: Place::Location(sheet.home.clone()),
            sanity: sheet.sanity,
            stamina: sheet.stamina,
            max_sanity_change: 0,
            max_stamina_change: 0,
            sliders: [0; 3],
            money: u16::from(sheet.fixed.money),
            clues: u16::from(sheet.fixed.clues),
            cards: Vec::new(),
            delayed: false,
            lose_next_turn: false,
            skipping_turn: false,
            encounters_over: false,
            explored: false,
            returned_this_turn: false,
            monster_trophies: Vec::new(),
            gate_trophies: Vec::new(),
            no_more_bank_loans: false,
            eliminated: false,
        })
    }

    /// Fixed then random possessions; Sanity and Stamina at their maximum.
    fn bring_in(&mut self, seat: usize) -> Flow<()> {
        let sheet = self.sheet(seat)?;
        for fixed in &sheet.fixed.cards {
            let found = self
                .data
                .investigator_cards
                .cards
                .iter()
                .find(|card| card.deck == fixed.deck && card.name == fixed.name);
            let taken = match found {
                Some(card) => self.take_from_deck(&card.id)?,
                None => None,
            };
            match taken {
                Some(card) => self.give_card(seat, card)?,
                None => self.log(format!(
                    "{} : « {} » n'est pas dans l'échantillon de cartes.",
                    sheet.name, fixed.name
                )),
            }
        }
        let random = sheet.random;
        let draws = [
            (Deck::CommonItem, random.common_items),
            (Deck::UniqueItem, random.unique_items),
            (Deck::Spell, random.spells),
            (Deck::Skill, random.skills),
        ];
        for (deck, count) in draws {
            for _ in 0..count {
                self.draw_and_give(seat, deck)?;
            }
        }
        let sanity = self.maximum(seat, Resource::Sanity)?;
        let stamina = self.maximum(seat, Resource::Stamina)?;
        let inv = self.inv_mut(seat);
        inv.sanity = sanity;
        inv.stamina = stamina;
        Ok(())
    }

    /// First Mythos card: Rumors and cards without a gate are put under the
    /// deck until a fitting card comes (FAQ).
    fn first_mythos(&mut self) -> Flow<()> {
        for _ in 0..self.state.mythos_deck.len() {
            let Some(card) = self.draw_mythos()? else {
                break;
            };
            if card.kind == data::MythosKind::Rumor || card.gate.is_none() {
                self.state.mythos_deck.push(card.id.clone());
                continue;
            }
            return self.resolve_mythos(card);
        }
        Err(Halt::Broken(
            "aucune carte Mythe ne peut ouvrir le premier portail".to_owned(),
        ))
    }

    /// A devoured investigator is replaced by a random unused sheet, who
    /// keeps the trophies and starts next turn.
    pub fn replace_investigator(&mut self, seat: usize) -> Flow<()> {
        let old = self.inv(seat).clone();
        self.state.unused_investigators.push(old.id.clone());
        let index = self.state.rng.below(self.state.unused_investigators.len());
        let id = self.state.unused_investigators.remove(index);
        let mut fresh = self.blank_investigator(id)?;
        fresh.monster_trophies = old.monster_trophies;
        fresh.gate_trophies = old.gate_trophies;
        fresh.skipping_turn = true;
        self.state.investigators[seat] = fresh;
        self.log(format!("{} rejoint la partie.", self.name(seat)));
        self.bring_in(seat)?;
        self.place_sliders(seat)
    }
}

//! Phase V: Mythos.

use data::{CardId, EnvironmentKind, LocationId, MythosCard, MythosKind, Ongoing};

use crate::game::Game;
use crate::halt::Flow;
use crate::shown::{Deck, Exhibit};
use crate::state::{MonsterPlace, Phase, Place};

impl<'a> Game<'a> {
    /// The Mythos phase: one card (two after Le Grand Rituel failed, the
    /// first one only opening its gate), then the end-of-Mythos effects.
    pub fn mythos_phase(&mut self) -> Flow<()> {
        self.state.phase = Phase::Mythos;
        if self.state.two_mythos_cards
            && let Some(card) = self.draw_mythos()?
        {
            self.log(format!("Carte Mythe (portail seulement) : {}.", card.name));
            self.mythos_gate(card)?;
            self.state.mythos_deck.push(card.id.clone());
        }
        if let Some(card) = self.draw_mythos()? {
            self.resolve_mythos(card)?;
        }
        self.end_of_mythos()
    }

    /// Draws the top Mythos card. A « remélangez » card goes back, the deck
    /// is shuffled and another card is drawn.
    pub fn draw_mythos(&mut self) -> Flow<Option<&'a MythosCard>> {
        for _ in 0..=self.state.mythos_deck.len() {
            if self.state.mythos_deck.is_empty() {
                break;
            }
            let id = self.state.mythos_deck.remove(0);
            let card = self.mythos_card(&id)?;
            self.show(Exhibit::Card {
                deck: Deck::Mythos,
                card: id.clone(),
                seat: self.state.first_player,
            });
            if card.kind != MythosKind::Reshuffle {
                return Ok(Some(card));
            }
            self.covered.insert(crate::coverage::mythos_key(&id));
            self.log(format!(
                "Carte Mythe : {}. Le paquet est remélangé.",
                card.name
            ));
            self.state.mythos_deck.push(id);
            let mut deck = std::mem::take(&mut self.state.mythos_deck);
            self.state.rng.shuffle(&mut deck);
            self.state.mythos_deck = deck;
        }
        self.log("Le paquet Mythe est vide.");
        Ok(None)
    }

    /// A Mythos card by id.
    pub fn mythos_card(&self, id: &CardId) -> Flow<&'a MythosCard> {
        let data: &'a data::GameData = self.data;
        data.mythos_cards
            .cards
            .iter()
            .find(|card| &card.id == id)
            .ok_or_else(|| crate::halt::Halt::Broken(format!("carte Mythe « {id} » introuvable")))
    }

    /// Resolves a Mythos card in rule order: gate, clue, monster movement,
    /// then its ability.
    pub fn resolve_mythos(&mut self, card: &'a MythosCard) -> Flow<()> {
        self.covered.insert(crate::coverage::mythos_key(&card.id));
        self.log(format!("Carte Mythe : {}.", card.name));
        self.mythos_gate(card)?;
        if let Some(location) = &card.clue {
            self.mythos_clue(location)?;
        }
        self.move_monsters(&card.white_moves, &card.black_moves)?;
        self.mythos_ability(card)
    }

    /// Step 1: a sealed location stops everything, an open gate causes a
    /// monster surge, otherwise a new gate opens.
    fn mythos_gate(&mut self, card: &MythosCard) -> Flow<()> {
        let Some(location) = &card.gate else {
            return Ok(());
        };
        if self.state.is_sealed(location) {
            self.log("Le lieu est scellé : rien ne se passe.");
            return Ok(());
        }
        if self.stabilized_location(location) {
            self.log("Le stabilisateur de flux de Kate empêche le portail de s'ouvrir.");
            return Ok(());
        }
        if self.state.gate_at(location).is_some() {
            return self.monster_surge(location);
        }
        let monsters = if self.state.players() >= 5 { 2 } else { 1 };
        self.open_gate(location, monsters)
    }

    /// Step 2: a clue token appears, unless a gate is open there.
    fn mythos_clue(&mut self, location: &LocationId) -> Flow<()> {
        if self.state.gate_at(location).is_some() {
            return Ok(());
        }
        let here = self.investigators_at(&Place::Location(location.clone()));
        let prompt = "Qui prend l'Indice ?";
        if let Some(seat) = self.first_player_picks(&here, prompt)? {
            self.log(format!("{} ramasse l'Indice.", self.name(seat)));
            self.inv_mut(seat).clues += 1;
            return Ok(());
        }
        *self.state.clues.entry(location.clone()).or_insert(0) += 1;
        self.log(format!(
            "Un Indice apparaît à {}.",
            self.location(location)?.name
        ));
        Ok(())
    }

    /// Step 4: the card's own effect, by kind.
    fn mythos_ability(&mut self, card: &'a MythosCard) -> Flow<()> {
        match card.kind {
            MythosKind::Headline => {
                self.apply_effects(self.state.first_player, &card.effects, None)?;
                if card.lingers {
                    self.state.lingering.push((card.id.clone(), true));
                } else {
                    self.state.mythos_deck.push(card.id.clone());
                }
            }
            MythosKind::Reshuffle => {}
            MythosKind::Environment(kind) => {
                if kind == EnvironmentKind::Weather && self.weather_cancelled() {
                    self.log("Ithaqua balaie la carte Temps.");
                    self.state.mythos_deck.push(card.id.clone());
                    return Ok(());
                }
                if let Some(previous) = self.state.environment.replace(card.id.clone()) {
                    self.state.mythos_deck.push(previous);
                }
                self.banish_banned_monsters(card);
                self.evacuate_closed(card)?;
                self.apply_effects(self.state.first_player, &card.effects, None)?;
            }
            MythosKind::Rumor => self.start_rumor(card)?,
        }
        if card.kind == MythosKind::Headline && card.lingers {
            self.evacuate_closed(card)?;
        }
        Ok(())
    }

    /// Whoever stands in a location that the card closes goes to the street.
    fn evacuate_closed(&mut self, card: &'a MythosCard) -> Flow<()> {
        for effect in &card.ongoing {
            if let Ongoing::LocationClosed(id) = effect {
                let street = self.location(id)?.street.clone();
                self.evacuate(id, &street)?;
            }
        }
        Ok(())
    }

    /// Monsters banned by a new Environment go back to the cup.
    fn banish_banned_monsters(&mut self, card: &MythosCard) {
        for effect in &card.ongoing {
            if let Ongoing::MonsterBanned(id) = effect {
                let (gone, kept): (Vec<_>, Vec<_>) = std::mem::take(&mut self.state.monsters)
                    .into_iter()
                    .partition(|monster| &monster.monster == id);
                self.state.monsters = kept;
                self.state
                    .monster_cup
                    .extend(gone.into_iter().map(|monster| monster.monster));
            }
        }
    }

    /// Monster surge: as many monsters as open gates or players, spread as
    /// evenly as possible, the surging gate getting the most.
    pub fn monster_surge(&mut self, location: &LocationId) -> Flow<()> {
        if self.stabilized_location(location) {
            self.log("Le stabilisateur de flux de Kate annule la vague de monstres.");
            return Ok(());
        }
        self.log("Vague de monstres !");
        let mut gates: Vec<LocationId> = self
            .state
            .gates
            .iter()
            .map(|gate| gate.location.clone())
            .collect();
        gates.retain(|gate| gate != location);
        gates.insert(0, location.clone());
        let count = gates.len().max(self.state.players());
        // Kate's gates get no monster; the others share the same count (FAQ).
        gates.retain(|gate| !self.stabilized_location(gate));
        if gates.is_empty() {
            return Ok(());
        }
        let mut shares = vec![count / gates.len(); gates.len()];
        let mut extra = count % gates.len();
        if extra > 0 {
            shares[0] += 1;
            extra -= 1;
        }
        self.share_extra_monsters(&gates, &mut shares, extra)?;
        for (gate, share) in gates.iter().zip(shares) {
            for _ in 0..share {
                let monster = self.draw_monster()?;
                self.place_monster(monster, MonsterPlace::Location(gate.clone()))?;
            }
        }
        Ok(())
    }

    /// End of the Mythos phase: Rumor progress, lasting triggers, the
    /// Ancient One's power, then the turn's flags are reset.
    fn end_of_mythos(&mut self) -> Flow<()> {
        self.rumor_progress()?;
        self.end_of_mythos_triggers()?;
        self.power_end_of_mythos()?;
        self.expire_lingering();
        for inv in &mut self.state.investigators {
            inv.turn_bonus = 0;
            inv.skipping_turn = false;
            inv.encounters_over = false;
            inv.returned_this_turn = false;
            inv.power_used = false;
        }
        Ok(())
    }

    /// Headlines that last « jusqu'à la fin du prochain tour » go back to
    /// the deck one Mythos phase after they came out.
    fn expire_lingering(&mut self) {
        let mut kept = Vec::new();
        for (card, fresh) in std::mem::take(&mut self.state.lingering) {
            if fresh {
                kept.push((card, false));
            } else {
                self.state.mythos_deck.push(card);
            }
        }
        self.state.lingering = kept;
    }

    /// The first player spreads the monsters left over by an uneven surge,
    /// at most one more per other gate.
    fn share_extra_monsters(
        &mut self,
        gates: &[LocationId],
        shares: &mut [usize],
        extra: usize,
    ) -> Flow<()> {
        for _ in 0..extra {
            let mut options = Vec::new();
            for (index, gate) in gates.iter().enumerate().skip(1) {
                if shares[index] < shares[0]
                    && shares[index] == shares[1..].iter().copied().min().unwrap_or(0)
                {
                    let label = self.location(gate)?.name.clone();
                    options.push(crate::game::option(
                        crate::decision::Choice::Gate(gate.clone()),
                        label,
                    ));
                }
            }
            let first = self.state.first_player;
            let prompt = "Quel portail reçoit un monstre de plus ?";
            let choice = self.ask(
                first,
                crate::decision::DecisionKind::Arbitration,
                prompt,
                options,
            )?;
            if let crate::decision::Choice::Gate(gate) = choice
                && let Some(index) = gates.iter().position(|candidate| candidate == &gate)
            {
                shares[index] += 1;
            }
        }
        Ok(())
    }
}

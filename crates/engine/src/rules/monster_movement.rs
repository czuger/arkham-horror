//! Monster movement during the Mythos phase.

use data::{DimensionSymbol, Movement, Resource};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::rules::status::{ASYLUM, HOSPITAL};
use crate::state::{MonsterPlace, Place};

/// Which arrow a monster follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arrow {
    White,
    Black,
}

impl Game<'_> {
    /// Moves the monsters whose symbol is in one of the movement boxes.
    pub fn move_monsters(
        &mut self,
        white: &[DimensionSymbol],
        black: &[DimensionSymbol],
    ) -> Flow<()> {
        for index in 0..self.state.monsters.len() {
            let id = self.state.monsters[index].monster.clone();
            let symbol = self.monster(&id)?.symbol;
            let arrow = if white.contains(&symbol) {
                Arrow::White
            } else if black.contains(&symbol) {
                Arrow::Black
            } else {
                continue;
            };
            if self.state.monsters[index].place == MonsterPlace::Outskirts {
                continue;
            }
            match (self.monster_stats(&id)?.movement, id.as_str()) {
                (Movement::Stationary, _) => {}
                (Movement::Fast, _) => {
                    if self.step(index, arrow) {
                        self.step(index, arrow);
                    }
                }
                (Movement::Flying, _) => self.fly(index)?,
                (Movement::Special, "hound_of_tindalos") => self.hound_moves(index)?,
                (Movement::Special, "chthonian") => self.chthonian_shakes()?,
                _ => {
                    self.step(index, arrow);
                }
            }
        }
        Ok(())
    }

    /// Whether investigators stand where a monster is.
    fn monster_meets_investigator(&self, place: &MonsterPlace) -> bool {
        place
            .as_place()
            .is_some_and(|place| !self.investigators_at(&place).is_empty())
    }

    /// One step along an arrow. Returns whether the monster can go on
    /// (it moved and met nobody).
    fn step(&mut self, index: usize, arrow: Arrow) -> bool {
        let place = self.state.monsters[index].place.clone();
        if self.monster_meets_investigator(&place) {
            return false;
        }
        let next = match &place {
            MonsterPlace::Location(id) => self.data.board.location(id).map(|l| l.street.clone()),
            MonsterPlace::Street(id) => self.data.board.street(id).map(|street| match arrow {
                Arrow::White => street.white_arrow.clone(),
                Arrow::Black => street.black_arrow.clone(),
            }),
            _ => None,
        };
        let Some(next) = next else {
            return false;
        };
        let next = MonsterPlace::Street(next);
        self.state.monsters[index].place = next.clone();
        !self.monster_meets_investigator(&next)
    }

    /// Flying monsters swoop on investigators in nearby streets, or wait in
    /// the Sky (FAQ rules).
    fn fly(&mut self, index: usize) -> Flow<()> {
        let place = self.state.monsters[index].place.clone();
        if self.monster_meets_investigator(&place) {
            return Ok(());
        }
        let candidates: Vec<Place> = match &place {
            MonsterPlace::Sky => self
                .data
                .board
                .streets
                .iter()
                .map(|s| Place::Street(s.id.clone()))
                .collect(),
            other => other
                .as_place()
                .map(|place| self.neighbours(&place))
                .unwrap_or_default()
                .into_iter()
                .filter(|place| matches!(place, Place::Street(_)))
                .collect(),
        };
        let targets: Vec<usize> = candidates
            .iter()
            .flat_map(|place| self.investigators_at(place))
            .collect();
        let target = self.weakest_sneak(&targets)?;
        let next = match target {
            Some(seat) => match &self.inv(seat).place {
                Place::Street(id) => MonsterPlace::Street(id.clone()),
                _ => return Ok(()),
            },
            None if place == MonsterPlace::Sky => return Ok(()),
            None => MonsterPlace::Sky,
        };
        self.state.monsters[index].place = next;
        Ok(())
    }

    /// Among some investigators, the one with the lowest Sneak; the first
    /// player breaks ties.
    fn weakest_sneak(&mut self, seats: &[usize]) -> Flow<Option<usize>> {
        let mut lowest = Vec::new();
        let mut best = i32::MAX;
        for &seat in seats {
            let sneak = self.skill(seat, data::CheckKind::Sneak)?;
            if sneak < best {
                best = sneak;
                lowest.clear();
            }
            if sneak == best && !lowest.contains(&seat) {
                lowest.push(seat);
            }
        }
        self.first_player_picks(&lowest, "Quel investigateur le monstre attaque-t-il ?")
    }

    /// The first player picks one investigator among several.
    pub fn first_player_picks(&mut self, seats: &[usize], prompt: &str) -> Flow<Option<usize>> {
        if seats.len() <= 1 {
            return Ok(seats.first().copied());
        }
        let options = seats
            .iter()
            .map(|&seat| option(Choice::Investigator(seat), self.name(seat)))
            .collect();
        let first = self.state.first_player;
        match self.ask(first, DecisionKind::Arbitration, prompt, options)? {
            Choice::Investigator(seat) => Ok(Some(seat)),
            _ => Ok(None),
        }
    }

    /// Hound of Tindalos: straight to the nearest investigator inside an
    /// Arkham location other than the Hospital and the Asylum.
    fn hound_moves(&mut self, index: usize) -> Flow<()> {
        let place = self.state.monsters[index].place.clone();
        if self.monster_meets_investigator(&place) {
            return Ok(());
        }
        let Some(from) = place.as_place() else {
            return Ok(());
        };
        let mut nearest = Vec::new();
        let mut best = usize::MAX;
        for seat in self.state.seat_order() {
            let Place::Location(id) = &self.inv(seat).place else {
                continue;
            };
            if id.as_str() == HOSPITAL || id.as_str() == ASYLUM {
                continue;
            }
            let Some(distance) = self.distance(&from, &self.inv(seat).place) else {
                continue;
            };
            if distance < best {
                best = distance;
                nearest.clear();
            }
            if distance == best {
                nearest.push(seat);
            }
        }
        if let Some(seat) = self.weakest_sneak(&nearest)?
            && let Place::Location(id) = self.inv(seat).place.clone()
        {
            self.state.monsters[index].place = MonsterPlace::Location(id);
        }
        Ok(())
    }

    /// Chthonian: instead of moving, on 4–6 every investigator in Arkham
    /// loses 1 Stamina.
    fn chthonian_shakes(&mut self) -> Flow<()> {
        let roll = self.die();
        self.log(format!("Le Chthonien fait trembler la terre : {roll}."));
        if roll < 4 {
            return Ok(());
        }
        for seat in self.state.seat_order() {
            if self.inv(seat).place.in_arkham() {
                self.catch_knock_out(seat, |game| game.lose(seat, Resource::Stamina, 1))?;
            }
        }
        Ok(())
    }
}

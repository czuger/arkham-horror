//! The monster cup, placing monsters, the monster limit and the Outskirts.

use data::{MonsterAbility, MonsterId};

use crate::game::Game;
use crate::halt::Flow;
use crate::state::{BoardMonster, MonsterPlace};

impl Game<'_> {
    /// Draws a monster from the cup. An empty cup wakes the Ancient One.
    /// Monsters banned by a Mythos card go back and another is drawn.
    pub fn draw_monster(&mut self) -> Flow<MonsterId> {
        let mut set_aside = Vec::new();
        let drawn = loop {
            if self.state.monster_cup.is_empty() {
                self.state.monster_cup.append(&mut set_aside);
                return self.awaken("la réserve de monstres est vide");
            }
            let index = self.state.rng.below(self.state.monster_cup.len());
            let id = self.state.monster_cup.remove(index);
            if self.is_banned(&id) {
                set_aside.push(id);
            } else {
                break id;
            }
        };
        self.state.monster_cup.append(&mut set_aside);
        Ok(drawn)
    }

    /// Monster limit, or `None` once Arkham is overrun (Terror 10).
    pub fn monster_limit(&self) -> Option<usize> {
        (self.state.terror < 10).then(|| self.state.players() + 3)
    }

    /// Monsters that count towards the limit: in Arkham and in the Sky.
    pub fn monsters_in_arkham(&self) -> usize {
        self.state
            .monsters
            .iter()
            .filter(|monster| monster.place.counts_for_limit())
            .count()
    }

    /// Puts a monster on the board, or in the Outskirts when the limit is
    /// reached. A full Outskirts empties into the cup and raises the Terror.
    pub fn place_monster(&mut self, id: MonsterId, place: MonsterPlace) -> Flow<()> {
        let over_limit = self
            .monster_limit()
            .is_some_and(|limit| self.monsters_in_arkham() >= limit);
        if !over_limit {
            self.log(format!(
                "{} apparaît ({}).",
                self.monster_name(&id),
                self.monster_place_name(&place)
            ));
            self.state
                .monsters
                .push(BoardMonster { monster: id, place });
            return self.check_monster_awakening();
        }
        self.log(format!("{} part en Périphérie.", self.monster_name(&id)));
        self.state.monsters.push(BoardMonster {
            monster: id,
            place: MonsterPlace::Outskirts,
        });
        let in_outskirts = self.monsters_at(&MonsterPlace::Outskirts).len();
        if in_outskirts > 8_usize.saturating_sub(self.state.players()) {
            self.log("La Périphérie déborde : ses monstres retournent dans la réserve.");
            self.clear_outskirts();
            self.raise_terror()?;
        }
        Ok(())
    }

    /// Sends every monster of the Outskirts back to the cup.
    pub fn clear_outskirts(&mut self) {
        let (gone, kept): (Vec<_>, Vec<_>) = std::mem::take(&mut self.state.monsters)
            .into_iter()
            .partition(|monster| monster.place == MonsterPlace::Outskirts);
        self.state.monsters = kept;
        self.state
            .monster_cup
            .extend(gone.into_iter().map(|monster| monster.monster));
    }

    /// Indices of the board monsters standing on a place.
    pub fn monsters_at(&self, place: &MonsterPlace) -> Vec<usize> {
        self.state
            .monsters
            .iter()
            .enumerate()
            .filter(|(_, monster)| &monster.place == place)
            .map(|(index, _)| index)
            .collect()
    }

    /// A defeated monster: kept as a trophy, unless it is Endless.
    pub fn defeat_monster(&mut self, seat: usize, id: MonsterId) -> Flow<()> {
        self.log(format!(
            "{} bat {}.",
            self.name(seat),
            self.monster_name(&id)
        ));
        self.on_monster_defeated(&id)?;
        if self.monster_stats(&id)?.has(MonsterAbility::Endless) {
            self.state.monster_cup.push(id);
        } else {
            self.inv_mut(seat).monster_trophies.push(id);
        }
        Ok(())
    }

    /// French name of a monster place.
    pub fn monster_place_name(&self, place: &MonsterPlace) -> String {
        match place {
            MonsterPlace::Sky => "Ciel".to_owned(),
            MonsterPlace::Outskirts => "Périphérie".to_owned(),
            other => other
                .as_place()
                .map_or_else(String::new, |place| self.place_name(&place)),
        }
    }
}

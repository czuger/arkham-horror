//! Moving around the board graph.

use std::collections::VecDeque;

use data::{LocationId, Ongoing};

use crate::game::Game;
use crate::state::Place;

impl Game<'_> {
    /// Whether a location is closed, for good or by a Mythos card in play.
    pub fn is_closed(&self, id: &LocationId) -> bool {
        self.state.is_closed(id)
            || self.mythos_has(|effect| *effect == Ongoing::LocationClosed(id.clone()))
    }

    /// Whether investigators may not enter or leave this street area.
    pub fn street_closed(&self, place: &Place) -> bool {
        let Place::Street(street) = place else {
            return false;
        };
        self.mythos_has(|effect| *effect == Ongoing::StreetClosed(street.clone()))
    }

    /// Places one step away in Arkham. Closed locations are left out.
    pub fn neighbours(&self, place: &Place) -> Vec<Place> {
        match place {
            Place::Location(id) => self
                .data
                .board
                .location(id)
                .map(|location| vec![Place::Street(location.street.clone())])
                .unwrap_or_default(),
            Place::Street(id) => {
                let mut found: Vec<Place> = self
                    .data
                    .board
                    .locations_of(id)
                    .filter(|location| !self.is_closed(&location.id))
                    .map(|location| Place::Location(location.id.clone()))
                    .collect();
                if let Some(street) = self.data.board.street(id) {
                    found.extend(street.neighbours.iter().cloned().map(Place::Street));
                }
                found
            }
            _ => Vec::new(),
        }
    }

    /// Number of steps between two places of Arkham, if connected.
    pub fn distance(&self, from: &Place, to: &Place) -> Option<usize> {
        let mut seen = vec![from.clone()];
        let mut queue = VecDeque::from([(from.clone(), 0)]);
        while let Some((place, steps)) = queue.pop_front() {
            if &place == to {
                return Some(steps);
            }
            for next in self.neighbours(&place) {
                if !seen.contains(&next) {
                    seen.push(next.clone());
                    queue.push_back((next, steps + 1));
                }
            }
        }
        None
    }

    /// Every location and street of Arkham that is not closed.
    pub fn arkham_places(&self) -> Vec<Place> {
        let board = &self.data.board;
        let locations = board
            .locations
            .iter()
            .filter(|location| !self.is_closed(&location.id))
            .map(|location| Place::Location(location.id.clone()));
        let streets = board
            .streets
            .iter()
            .map(|street| Place::Street(street.id.clone()));
        locations.chain(streets).collect()
    }

    /// Seats of the investigators standing on a place.
    pub fn investigators_at(&self, place: &Place) -> Vec<usize> {
        self.state
            .seat_order()
            .into_iter()
            .filter(|&seat| &self.inv(seat).place == place)
            .collect()
    }
}

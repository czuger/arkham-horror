//! The place of an Other World the scene shows: a visit starts at a place
//! drawn among the world's, and its second area shows one of the two places
//! that one leads to, so that crossing the world feels like a walk.

use data::{OtherWorld, OtherWorldId, WorldScene};
use engine::{Area, Place, View};

use crate::views::scene_pick::{mix, pick_index};

/// The picture of an Other World area for the investigator at `seat`,
/// relative to `illustrations/`: the visit they are making there, or the
/// next one when they stand elsewhere (what a preview shows is then what
/// they will find).
pub fn world_illustration(
    view: &View<'_, '_>,
    seat: usize,
    world: &OtherWorldId,
    area: Area,
) -> Option<String> {
    let inv = view.state().investigators.get(seat)?;
    let inside = matches!(&inv.place, Place::OtherWorld { world: here, .. } if here == world);
    let visit = if inside {
        inv.world_visits
    } else {
        inv.world_visits.saturating_add(1)
    };
    visit_illustration(view, seat, world, area, visit)
}

/// The picture of an Other World area during the `visit`-th trip of the
/// investigator at `seat` to any Other World.
pub fn visit_illustration(
    view: &View<'_, '_>,
    seat: usize,
    world: &OtherWorldId,
    area: Area,
    visit: u32,
) -> Option<String> {
    let data = view.data();
    let state = view.state();
    let key = visit_key(state.seed, seat, world, visit);
    let scene = walked_scene(data.board.other_world(world)?, key, area)?;
    let by_night = state.terror >= data.board.night_from_terror;
    Some(scene.picture(by_night).to_owned())
}

/// The place shown in `area`: drawn among all for the first area, one of
/// the two it leads to for the second.
fn walked_scene(world: &OtherWorld, key: u64, area: Area) -> Option<&WorldScene> {
    let first = world.scenes.get(pick_index(key, world.scenes.len())?)?;
    match area {
        Area::First => Some(first),
        Area::Second => {
            let way = pick_index(mix(key), first.leads_to.len())?;
            world.scene(first.leads_to.get(way)?)
        }
    }
}

/// Mixes the game, the investigator, the world and the visit together.
fn visit_key(seed: u64, seat: usize, world: &OtherWorldId, visit: u32) -> u64 {
    let world_bits = world
        .as_str()
        .bytes()
        .fold(0, |bits, byte| mix(bits ^ u64::from(byte)));
    let seat = u64::try_from(seat).unwrap_or(u64::MAX);
    mix(mix(mix(mix(seed) ^ seat) ^ world_bits) ^ u64::from(visit))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::Path;

    use data::GameData;

    use super::*;

    fn data() -> GameData {
        GameData::load(Path::new("../../data")).expect("data")
    }

    #[test]
    fn the_second_area_is_a_place_the_first_leads_to() {
        let data = data();
        for world in &data.board.other_worlds {
            for visit in 0..40 {
                let key = visit_key(7, 0, &world.id, visit);
                let first = walked_scene(world, key, Area::First).expect("first");
                let second = walked_scene(world, key, Area::Second).expect("second");
                assert!(
                    first.leads_to.contains(&second.id),
                    "{}: {} → {}",
                    world.id,
                    first.id,
                    second.id
                );
            }
        }
    }

    #[test]
    fn visits_start_at_many_places() {
        let data = data();
        let world = data.board.other_worlds.first().expect("a world");
        let starts: BTreeSet<&str> = (0..40)
            .filter_map(|visit| {
                let key = visit_key(7, 0, &world.id, visit);
                walked_scene(world, key, Area::First).map(|scene| scene.id.as_str())
            })
            .collect();
        assert!(starts.len() >= world.scenes.len() / 2, "{starts:?}");
    }

    #[test]
    fn a_place_with_a_night_picture_shows_it_by_night_only() {
        let data = data();
        let scene = data
            .board
            .other_worlds
            .iter()
            .flat_map(|world| &world.scenes)
            .find(|scene| scene.night.is_some())
            .expect("a place with a night picture");
        assert_eq!(scene.picture(false), scene.image);
        assert_eq!(Some(scene.picture(true)), scene.night.as_deref());
    }
}

//! Consistency checks on the places of each Other World.

use crate::board::Board;
use crate::other_world::OtherWorld;
use crate::validation::board_checks::report_duplicates;
use crate::validation::issue::Issue;

/// Every world has places with unique ids, and every link leads to one of its
/// own places.
pub(crate) fn check_world_scenes(board: &Board, issues: &mut Vec<Issue>) {
    for world in &board.other_worlds {
        if world.scenes.is_empty() {
            issues.push(Issue::NoWorldScene {
                world: world.id.to_string(),
            });
            continue;
        }
        report_duplicates(
            "lieu d'Autre Monde",
            world.scenes.iter().map(|scene| scene.id.as_str()),
            issues,
        );
        for scene in &world.scenes {
            for target in &scene.leads_to {
                check_reference(world, &scene.id, target, issues);
            }
        }
    }
}

/// Reports `target` when it is not a place of `world`.
fn check_reference(world: &OtherWorld, from: &str, target: &str, issues: &mut Vec<Issue>) {
    if world.scene(target).is_none() {
        issues.push(Issue::UnknownReference {
            context: format!("Autre Monde « {} », {from}", world.id),
            family: "lieu",
            id: target.to_owned(),
        });
    }
}

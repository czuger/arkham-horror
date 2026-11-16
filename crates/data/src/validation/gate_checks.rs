//! Consistency checks on the gate markers.

use crate::board::Board;
use crate::gate_marker::GateMarkers;
use crate::validation::issue::Issue;

/// Every marker leads to a known world with at least one copy, and every
/// world has a marker leading to it.
pub(crate) fn check_gate_markers(gates: &GateMarkers, board: &Board, issues: &mut Vec<Issue>) {
    for marker in &gates.markers {
        if board.other_world(&marker.world).is_none() {
            issues.push(Issue::UnknownReference {
                context: "marqueur Portail".to_owned(),
                family: "Autre Monde",
                id: marker.world.to_string(),
            });
        }
        if marker.copies == 0 {
            issues.push(Issue::ZeroCopies {
                context: format!("marqueur Portail « {} »", marker.world),
            });
        }
    }
    for world in &board.other_worlds {
        if !gates.markers.iter().any(|marker| marker.world == world.id) {
            issues.push(Issue::WorldWithoutGate {
                world: world.id.to_string(),
            });
        }
    }
}

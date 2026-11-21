//! Consistency checks on the board graph.

use std::collections::HashSet;

use crate::board::Board;
use crate::location_icon::EncounterIcon;
use crate::special_area::SpecialAreaKind;
use crate::terror_track::TerrorTrack;
use crate::validation::issue::Issue;

/// Runs every board check and appends the problems found to `issues`.
pub(crate) fn check_board(board: &Board, issues: &mut Vec<Issue>) {
    check_unique_ids(board, issues);
    check_location_streets(board, issues);
    check_street_links(board, issues);
    check_arrows(board, issues);
    check_districts(board, issues);
    check_other_worlds(board, issues);
    check_special_areas(board, issues);
    check_terror_track(board, issues);
    check_icon_images(board, issues);
}

/// Reports ids used twice within the same family.
fn check_unique_ids(board: &Board, issues: &mut Vec<Issue>) {
    report_duplicates("rue", board.streets.iter().map(|s| s.id.as_str()), issues);
    report_duplicates(
        "lieu",
        board.locations.iter().map(|l| l.id.as_str()),
        issues,
    );
    report_duplicates(
        "Autre Monde",
        board.other_worlds.iter().map(|w| w.id.as_str()),
        issues,
    );
}

/// Pushes a [`Issue::DuplicateId`] for every id seen more than once.
pub(crate) fn report_duplicates<'a>(
    family: &'static str,
    ids: impl Iterator<Item = &'a str>,
    issues: &mut Vec<Issue>,
) {
    let mut seen = HashSet::new();
    for id in ids {
        if !seen.insert(id) {
            issues.push(Issue::DuplicateId {
                family,
                id: id.to_owned(),
            });
        }
    }
}

/// Every location belongs to an existing street.
fn check_location_streets(board: &Board, issues: &mut Vec<Issue>) {
    for location in &board.locations {
        if board.street(&location.street).is_none() {
            issues.push(Issue::UnknownReference {
                context: format!("lieu « {} »", location.id),
                family: "rue",
                id: location.street.to_string(),
            });
        }
    }
}

/// Street links point to existing streets, never to themselves, and go both ways.
fn check_street_links(board: &Board, issues: &mut Vec<Issue>) {
    for street in &board.streets {
        for neighbour_id in &street.neighbours {
            if neighbour_id == &street.id {
                issues.push(Issue::SelfLink {
                    street: street.id.to_string(),
                });
                continue;
            }
            let Some(neighbour) = board.street(neighbour_id) else {
                issues.push(Issue::UnknownReference {
                    context: format!("voisins de la rue « {} »", street.id),
                    family: "rue",
                    id: neighbour_id.to_string(),
                });
                continue;
            };
            if !neighbour.neighbours.contains(&street.id) {
                issues.push(Issue::AsymmetricLink {
                    from: street.id.to_string(),
                    to: neighbour_id.to_string(),
                });
            }
        }
    }
}

/// Both monster arrows of a street lead to one of its neighbours.
fn check_arrows(board: &Board, issues: &mut Vec<Issue>) {
    for street in &board.streets {
        for (colour, target) in [
            ("blanche", &street.white_arrow),
            ("noire", &street.black_arrow),
        ] {
            if !street.neighbours.contains(target) {
                issues.push(Issue::ArrowNotNeighbour {
                    street: street.id.to_string(),
                    colour,
                    target: target.to_string(),
                });
            }
        }
    }
}

/// Every street area has at least one location.
fn check_districts(board: &Board, issues: &mut Vec<Issue>) {
    for street in &board.streets {
        if board.locations_of(&street.id).next().is_none() {
            issues.push(Issue::EmptyDistrict {
                street: street.id.to_string(),
            });
        }
    }
}

/// Every Other World has at least one encounter colour.
fn check_other_worlds(board: &Board, issues: &mut Vec<Issue>) {
    for world in &board.other_worlds {
        if world.colors.is_empty() {
            issues.push(Issue::NoEncounterColour {
                world: world.id.to_string(),
            });
        }
    }
}

/// The four special areas the rules rely on are all present.
fn check_special_areas(board: &Board, issues: &mut Vec<Issue>) {
    for kind in [
        SpecialAreaKind::Outskirts,
        SpecialAreaKind::Sky,
        SpecialAreaKind::LostInTimeAndSpace,
        SpecialAreaKind::JailCell,
    ] {
        if board.special_area(kind).is_none() {
            issues.push(Issue::MissingSpecialArea { kind });
        }
    }
}

/// The terror track has one space per level, 0 to 10.
fn check_terror_track(board: &Board, issues: &mut Vec<Issue>) {
    if board.terror_track.spaces.len() != TerrorTrack::SPACES {
        issues.push(Issue::Mismatch {
            context: "échelle de Terreur".to_owned(),
            reason: "il faut 11 cases, de 0 à 10",
        });
    }
}

/// Every encounter icon has an image.
fn check_icon_images(board: &Board, issues: &mut Vec<Issue>) {
    for icon in EncounterIcon::ALL {
        if !board.icon_images.contains_key(&icon) {
            issues.push(Issue::Mismatch {
                context: format!("icône de rencontre {}", icon.label()),
                reason: "pas d'image dans `icon_images`",
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{LocationId, OtherWorldId, StreetId};
    use crate::location::{Location, Stability};
    use crate::other_world::{EncounterColor, OtherWorld};
    use crate::position::Position;
    use crate::rect::Rect;
    use crate::special_area::SpecialArea;
    use crate::street::Street;

    const ORIGIN: Position = Position { x: 0, y: 0 };

    fn street(id: &str, neighbour: &str) -> Street {
        Street {
            id: StreetId::new(id),
            name: id.to_owned(),
            neighbours: vec![StreetId::new(neighbour)],
            white_arrow: StreetId::new(neighbour),
            black_arrow: StreetId::new(neighbour),
            position: ORIGIN,
        }
    }

    fn location(id: &str, street: &str) -> Location {
        Location {
            id: LocationId::new(id),
            name: id.to_owned(),
            street: StreetId::new(street),
            stability: Stability::Stable,
            ability: None,
            closes_at_terror: None,
            icons: Vec::new(),
            panel: Rect {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            },
            position: ORIGIN,
        }
    }

    fn special(kind: SpecialAreaKind) -> SpecialArea {
        SpecialArea {
            kind,
            name: String::new(),
            position: ORIGIN,
        }
    }

    /// Two linked streets with one location each: passes every check.
    fn small_board() -> Board {
        Board {
            version: Board::VERSION,
            image: String::new(),
            icon_images: EncounterIcon::ALL
                .into_iter()
                .map(|icon| (icon, String::new()))
                .collect(),
            streets: vec![street("a", "b"), street("b", "a")],
            locations: vec![location("la", "a"), location("lb", "b")],
            other_worlds: vec![OtherWorld {
                id: OtherWorldId::new("w"),
                name: String::new(),
                colors: vec![EncounterColor::Red],
                first_area: ORIGIN,
                second_area: ORIGIN,
            }],
            special_areas: vec![
                special(SpecialAreaKind::Outskirts),
                special(SpecialAreaKind::Sky),
                special(SpecialAreaKind::LostInTimeAndSpace),
                special(SpecialAreaKind::JailCell),
            ],
            terror_track: TerrorTrack {
                marker: String::new(),
                spaces: vec![ORIGIN; TerrorTrack::SPACES],
            },
            closed_marker: String::new(),
        }
    }

    fn issues(board: &Board) -> Vec<Issue> {
        let mut issues = Vec::new();
        check_board(board, &mut issues);
        issues
    }

    #[test]
    fn small_board_is_valid() {
        assert_eq!(issues(&small_board()), []);
    }

    #[test]
    fn icon_without_image_is_reported() {
        let mut board = small_board();
        board.icon_images.remove(&EncounterIcon::Spell);
        assert_eq!(issues(&board).len(), 1);
    }

    #[test]
    fn one_way_link_is_reported() {
        let mut board = small_board();
        board.streets.push(street("c", "a"));
        board.locations.push(location("lc", "c"));
        assert!(issues(&board).contains(&Issue::AsymmetricLink {
            from: "c".to_owned(),
            to: "a".to_owned(),
        }));
    }

    #[test]
    fn arrow_to_a_non_neighbour_is_reported() {
        let mut board = small_board();
        board.streets[0].white_arrow = StreetId::new("a");
        assert_eq!(
            issues(&board),
            [Issue::ArrowNotNeighbour {
                street: "a".to_owned(),
                colour: "blanche",
                target: "a".to_owned(),
            }]
        );
    }

    #[test]
    fn unknown_street_and_duplicate_location_are_reported() {
        let mut board = small_board();
        board.locations.push(location("la", "nowhere"));
        let found = issues(&board);
        assert!(found.contains(&Issue::DuplicateId {
            family: "lieu",
            id: "la".to_owned(),
        }));
        assert!(found.contains(&Issue::UnknownReference {
            context: "lieu « la »".to_owned(),
            family: "rue",
            id: "nowhere".to_owned(),
        }));
    }

    #[test]
    fn missing_sky_and_empty_district_are_reported() {
        let mut board = small_board();
        board
            .special_areas
            .retain(|area| area.kind != SpecialAreaKind::Sky);
        board.locations.pop();
        assert_eq!(
            issues(&board),
            [
                Issue::EmptyDistrict {
                    street: "b".to_owned()
                },
                Issue::MissingSpecialArea {
                    kind: SpecialAreaKind::Sky
                },
            ]
        );
    }
}

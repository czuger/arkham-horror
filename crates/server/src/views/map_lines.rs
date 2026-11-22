//! The lines of the symbolic map: links between places, the monsters' arrows
//! and the frames of the Other Worlds. Coordinates are those of `board.ron`.

use data::{Board, OtherWorld, Position};

/// A link between two places.
#[derive(Debug, Clone)]
pub struct MapLink {
    /// Start, x.
    pub x1: i32,
    /// Start, y.
    pub y1: i32,
    /// End, x.
    pub x2: i32,
    /// End, y.
    pub y2: i32,
    /// Street to street (thick), or location to its street (thin).
    pub between_streets: bool,
}

/// A small triangle on a street link, pointing the way monsters move along
/// the white or the black arrow.
#[derive(Debug, Clone)]
pub struct MapArrow {
    /// The triangle's corners, as SVG `points`.
    pub points: String,
    /// White arrow (else black).
    pub white: bool,
}

/// The frame around the two areas of an Other World, with its name.
#[derive(Debug, Clone)]
pub struct WorldFrame {
    /// Left edge.
    pub x: i32,
    /// Top edge.
    pub y: i32,
    /// Width.
    pub width: i32,
    /// Height.
    pub height: i32,
    /// Short name, written at the top of the frame.
    pub label: String,
    /// Full name, for screen readers.
    pub name: String,
    /// Where the label is written, x (centre).
    pub label_x: i32,
    /// Where the label is written, y (baseline).
    pub label_y: i32,
}

/// Every link: street to street once, and each location to its street.
pub fn links(board: &Board) -> Vec<MapLink> {
    let mut links = Vec::new();
    for street in &board.streets {
        for neighbour in &street.neighbours {
            if neighbour.as_str() > street.id.as_str()
                && let Some(other) = board.street(neighbour)
            {
                links.push(link(street.position, other.position, true));
            }
        }
    }
    for location in &board.locations {
        if let Some(street) = board.street(&location.street) {
            links.push(link(location.position, street.position, false));
        }
    }
    links
}

fn link(from: Position, to: Position, between_streets: bool) -> MapLink {
    MapLink {
        x1: i32::from(from.x),
        y1: i32::from(from.y),
        x2: i32::from(to.x),
        y2: i32::from(to.y),
        between_streets,
    }
}

/// The white and the black arrow of every street.
pub fn arrows(board: &Board) -> Vec<MapArrow> {
    let mut arrows = Vec::new();
    for street in &board.streets {
        for (target, white) in [(&street.white_arrow, true), (&street.black_arrow, false)] {
            if let Some(other) = board.street(target) {
                arrows.push(arrow(street.position, other.position, white));
            }
        }
    }
    arrows
}

/// A triangle a third of the way along the link, beside it: white arrows on
/// one side, black on the other, so a pair never overlaps.
fn arrow(from: Position, to: Position, white: bool) -> MapArrow {
    let (ax, ay) = (f64::from(from.x), f64::from(from.y));
    let (dx, dy) = (f64::from(to.x) - ax, f64::from(to.y) - ay);
    let length = dx.hypot(dy).max(1.0);
    let (ux, uy) = (dx / length, dy / length);
    let side = if white { 14.0 } else { -14.0 };
    let (cx, cy) = (
        ax + ux * length * 0.36 - uy * side,
        ay + uy * length * 0.36 + ux * side,
    );
    let corners = [
        (cx + ux * 13.0, cy + uy * 13.0),
        (cx - ux * 9.0 - uy * 9.0, cy - uy * 9.0 + ux * 9.0),
        (cx - ux * 9.0 + uy * 9.0, cy - uy * 9.0 - ux * 9.0),
    ];
    MapArrow {
        points: corners
            .iter()
            .map(|(x, y)| format!("{x:.0},{y:.0}"))
            .collect::<Vec<_>>()
            .join(" "),
        white,
    }
}

/// The frame of each Other World, around its two areas.
pub fn world_frames(board: &Board) -> Vec<WorldFrame> {
    board.other_worlds.iter().map(world_frame).collect()
}

fn world_frame(world: &OtherWorld) -> WorldFrame {
    let top = i32::from(world.first_area.y.min(world.second_area.y));
    let bottom = i32::from(world.first_area.y.max(world.second_area.y));
    let x = i32::from(world.first_area.x);
    WorldFrame {
        x: x - 78,
        y: top - 62,
        width: 156,
        height: bottom - top + 88,
        label: world.short_name.clone(),
        name: world.name.clone(),
        label_x: x,
        label_y: top - 32,
    }
}

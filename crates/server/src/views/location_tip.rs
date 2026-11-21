use askama::Template;
use data::{EncounterIcon, Location, LocationIcon, Stability};

use crate::views::board_geometry::BOARD_SIZE;

/// Size in pixels of an icon drawn in a bubble.
const ICON_SIZE: f64 = 30.0;
/// Size in pixels of the board piece cut around an icon's centre.
const ICON_CUT: f64 = 15.0;

/// The bubble of a location: stability, ability, closing, and each encounter
/// icon cut out of the board image with what it means. Rendered by
/// `location_tip.html`, already escaped.
#[derive(Debug, Clone, Template)]
#[template(path = "location_tip.html")]
pub struct LocationTip {
    /// Name of the location.
    pub name: String,
    /// Whether the location is stable.
    pub stable: bool,
    /// Ability name and text, if any.
    pub ability: Option<(String, String)>,
    /// Terror level at which the location closes, if any.
    pub closes_at_terror: Option<u8>,
    /// Board image path under `/assets/`.
    pub board_image: String,
    /// The encounter icons.
    pub icons: Vec<IconLine>,
}

/// One encounter icon in a location's bubble.
#[derive(Debug, Clone)]
pub struct IconLine {
    /// What it stands for.
    pub label: &'static str,
    /// What it tells the player.
    pub meaning: &'static str,
    /// Printed in reversed colours: a sure way to get it here.
    pub sure: bool,
    /// CSS `background-size` that zooms the board to the icon's scale.
    pub size: String,
    /// CSS `background-position` that centres the icon.
    pub offset: String,
}

impl LocationTip {
    /// The bubble of a location drawn on the board image `board_image`.
    pub fn new(board_image: &str, location: &Location) -> Self {
        Self {
            name: location.name.clone(),
            stable: location.stability == Stability::Stable,
            ability: location
                .ability
                .as_ref()
                .map(|a| (a.name.clone(), a.text.clone())),
            closes_at_terror: location.closes_at_terror,
            board_image: board_image.to_owned(),
            icons: location.icons.iter().map(icon_line).collect(),
        }
    }
}

/// An icon, its meaning and how to cut it out of the board image.
fn icon_line(icon: &LocationIcon) -> IconLine {
    let scale = ICON_SIZE / ICON_CUT;
    let x = f64::from(icon.position.x) * scale - ICON_SIZE / 2.0;
    let y = f64::from(icon.position.y) * scale - ICON_SIZE / 2.0;
    IconLine {
        label: icon.kind.label(),
        meaning: meaning(icon.kind),
        sure: icon.sure,
        size: format!(
            "{:.0}px {:.0}px",
            BOARD_SIZE.0 * scale,
            BOARD_SIZE.1 * scale
        ),
        offset: format!("{:.0}px {:.0}px", -x, -y),
    }
}

/// What an icon tells the player about the encounters of a location.
fn meaning(icon: EncounterIcon) -> &'static str {
    match icon {
        EncounterIcon::Ally => "on peut y rencontrer un Allié",
        EncounterIcon::Sanity => "on y regagne souvent de la Santé Mentale",
        EncounterIcon::Blessing => "on peut y être Béni",
        EncounterIcon::Skill => "on peut y gagner une Compétence",
        EncounterIcon::Clue => "on y trouve souvent des pions Indice",
        EncounterIcon::Spell => "on peut y apprendre un Sort",
        EncounterIcon::CommonItem => "on y trouve souvent des Objets Communs",
        EncounterIcon::Money => "on y gagne souvent de l'Argent",
        EncounterIcon::Stamina => "on y regagne souvent de la Résistance",
        EncounterIcon::UniqueItem => "on y trouve souvent des Objets Uniques",
    }
}

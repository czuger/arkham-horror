use askama::Template;
use data::{Board, EncounterIcon, Location, LocationIcon, Stability};

/// The bubble of a location: stability, ability, closing, and each encounter
/// icon with what it means. Rendered by `location_tip.html`, already escaped.
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
    /// Icon image path under `/assets/` (empty when there is none).
    pub image: String,
}

impl LocationTip {
    /// The bubble of a location of `board`.
    pub fn new(board: &Board, location: &Location) -> Self {
        Self {
            name: location.name.clone(),
            stable: location.stability == Stability::Stable,
            ability: location
                .ability
                .as_ref()
                .map(|a| (a.name.clone(), a.text.clone())),
            closes_at_terror: location.closes_at_terror,
            icons: location
                .icons
                .iter()
                .map(|icon| icon_line(board, icon))
                .collect(),
        }
    }
}

/// An icon, its meaning and its image.
fn icon_line(board: &Board, icon: &LocationIcon) -> IconLine {
    IconLine {
        label: icon.kind.label(),
        meaning: meaning(icon.kind),
        sure: icon.sure,
        image: board
            .icon_images
            .get(&icon.kind)
            .cloned()
            .unwrap_or_default(),
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

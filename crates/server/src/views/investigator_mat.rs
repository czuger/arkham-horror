use data::{Investigator, Resource};
use engine::{InvestigatorState, View};

use crate::session::HUMAN_SEAT;
use crate::views::image_token::ImageToken;
use crate::views::names::{monster, sheet};
use crate::views::tips;

/// Bill values printed on the money image, largest first.
const BILLS: [u16; 3] = [10, 5, 1];

/// One investigator's play area: the sheet with its sliders, the Sanity,
/// Stamina, money and clue tokens beside it, and the trophies.
#[derive(Debug, Clone)]
pub struct InvestigatorMat {
    /// Name.
    pub name: String,
    /// Title (« la Journaliste »…).
    pub title: String,
    /// Who plays it: « Vous » or « IA ».
    pub player: &'static str,
    /// Where the investigator stands.
    pub place: String,
    /// Sheet image path under `/assets/`.
    pub image: String,
    /// The three sliders: left and top of each, in percent of the sheet.
    pub sliders: Vec<(String, String)>,
    /// Sanity, and its maximum.
    pub sanity: (u8, u8),
    /// Stamina, and its maximum.
    pub stamina: (u8, u8),
    /// Money.
    pub money: u16,
    /// The bills making up the money, largest first (10, 5 or 1).
    pub bills: Vec<u16>,
    /// Clue tokens.
    pub clues: u16,
    /// Monster trophies.
    pub monster_trophies: Vec<ImageToken>,
    /// Gate trophies.
    pub gate_trophies: Vec<ImageToken>,
    /// Whether the investigator has been eliminated.
    pub eliminated: bool,
}

impl InvestigatorMat {
    /// The play area of the investigator at `seat`; `None` when the seat is
    /// empty or the sheet unknown.
    pub fn new(view: &View<'_, '_>, seat: usize) -> Option<Self> {
        let data = view.data();
        let inv = view.state().investigators.get(seat)?;
        let sheet = sheet(data, &inv.id)?;
        Some(Self {
            name: sheet.name.clone(),
            title: sheet.title.clone(),
            player: if seat == HUMAN_SEAT { "Vous" } else { "IA" },
            place: view.place_name(&inv.place),
            image: sheet.image.clone(),
            sliders: sliders(sheet, inv),
            sanity: (inv.sanity, view.maximum(seat, Resource::Sanity)),
            stamina: (inv.stamina, view.maximum(seat, Resource::Stamina)),
            money: inv.money,
            bills: bills(inv.money),
            clues: inv.clues,
            monster_trophies: inv
                .monster_trophies
                .iter()
                .filter_map(|id| monster(view, id))
                .collect(),
            gate_trophies: gate_trophies(view, inv),
            eliminated: inv.eliminated,
        })
    }
}

/// Where each slider stands on the sheet, in percent of the image.
fn sliders(sheet: &Investigator, inv: &InvestigatorState) -> Vec<(String, String)> {
    let size = sheet.image_size;
    sheet
        .sliders
        .iter()
        .zip(inv.sliders)
        .filter_map(|(spots, stop)| {
            let x = spots.x.get(usize::from(stop))?;
            Some((percent(*x, size.width), percent(spots.y, size.height)))
        })
        .collect()
}

/// The fewest bills making up an amount.
fn bills(money: u16) -> Vec<u16> {
    let mut rest = money;
    let mut out = Vec::new();
    for bill in BILLS {
        while rest >= bill {
            out.push(bill);
            rest -= bill;
        }
    }
    out
}

/// The gate markers won, with their Other World.
fn gate_trophies(view: &View<'_, '_>, inv: &InvestigatorState) -> Vec<ImageToken> {
    let data = view.data();
    inv.gate_trophies
        .iter()
        .filter_map(|index| data.gate_markers.markers.get(*index))
        .map(|marker| {
            let world = tips::world_name(data, &marker.world);
            ImageToken {
                name: format!("Portail vers {world}"),
                image: marker.image.clone(),
                tip: tips::gate_trophy(&world),
            }
        })
        .collect()
}

/// A pixel coordinate as a CSS percentage of the image size.
fn percent(pixels: u16, size: u16) -> String {
    format!("{:.2}%", f64::from(pixels) * 100.0 / f64::from(size.max(1)))
}

use data::{Investigator, Resource, SheetMarkers};
use engine::{InvestigatorState, View};

use crate::play_mode::PlayMode;
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
    /// The bills making up the money, one row per value, largest first.
    pub bills: Vec<BillRow>,
    /// Clue tokens.
    pub clues: u16,
    /// Monster trophies.
    pub monster_trophies: Vec<ImageToken>,
    /// Gate trophies.
    pub gate_trophies: Vec<ImageToken>,
    /// Whether the investigator has been eliminated.
    pub eliminated: bool,
    /// Statuses worth a line (delayed, arrested…).
    pub flags: Vec<&'static str>,
    /// Special ability: name and text.
    pub ability: (String, String),
}

/// The bills of one value: $10, $5 or $1.
#[derive(Debug, Clone)]
pub struct BillRow {
    /// Value of one bill, in dollars.
    pub value: u16,
    /// Bill image path under `/assets/`.
    pub image: String,
    /// How many bills of that value.
    pub count: u16,
}

impl InvestigatorMat {
    /// The play area of the investigator at `seat`; `None` when the seat is
    /// empty or the sheet unknown.
    pub fn new(view: &View<'_, '_>, seat: usize, mode: PlayMode) -> Option<Self> {
        let data = view.data();
        let inv = view.state().investigators.get(seat)?;
        let sheet = sheet(data, &inv.id)?;
        Some(Self {
            name: sheet.name.clone(),
            title: sheet.title.clone(),
            player: mode.player_label(seat),
            place: view.place_name(&inv.place),
            image: sheet.image.clone(),
            sliders: sliders(sheet, inv),
            sanity: (inv.sanity, view.maximum(seat, Resource::Sanity)),
            stamina: (inv.stamina, view.maximum(seat, Resource::Stamina)),
            money: inv.money,
            bills: bills(&data.investigators.markers, inv.money),
            clues: inv.clues,
            monster_trophies: inv
                .monster_trophies
                .iter()
                .filter_map(|id| monster(view, id))
                .collect(),
            gate_trophies: gate_trophies(view, inv),
            eliminated: inv.eliminated,
            flags: flags(inv),
            ability: (sheet.ability.name.clone(), sheet.ability.text.clone()),
        })
    }
}

/// Where each slider stands on the sheet, in percent of the image.
fn sliders(sheet: &Investigator, inv: &InvestigatorState) -> Vec<(String, String)> {
    inv.sliders
        .iter()
        .enumerate()
        .filter_map(|(track, stop)| slider_spot(sheet, track, *stop))
        .collect()
}

/// Where a stop of a slider track is on the sheet: left and top, in percent
/// of the image; `None` for an unknown track or stop.
pub fn slider_spot(sheet: &Investigator, track: usize, stop: u8) -> Option<(String, String)> {
    let size = sheet.image_size;
    let spots = sheet.sliders.get(track)?;
    let x = spots.x.get(usize::from(stop))?;
    Some((percent(*x, size.width), percent(spots.y, size.height)))
}

/// The fewest bills making up an amount, grouped by value.
fn bills(markers: &SheetMarkers, money: u16) -> Vec<BillRow> {
    let mut rest = money;
    BILLS
        .iter()
        .filter_map(|value| {
            let count = rest / value;
            rest %= value;
            let image = markers.bill(*value)?.to_owned();
            (count > 0).then_some(BillRow {
                value: *value,
                image,
                count,
            })
        })
        .collect()
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

/// Statuses shown on the investigator's page.
fn flags(inv: &InvestigatorState) -> Vec<&'static str> {
    [
        (inv.delayed, "Retardé"),
        (inv.lose_next_turn, "Perdra son prochain tour"),
        (inv.skipping_turn, "Passe ce tour"),
        (inv.explored, "A exploré l'Autre Monde"),
        (inv.no_more_bank_loans, "Plus de prêt bancaire"),
    ]
    .into_iter()
    .filter_map(|(on, label)| on.then_some(label))
    .collect()
}

/// A pixel coordinate as a CSS percentage of the image size.
fn percent(pixels: u16, size: u16) -> String {
    format!("{:.2}%", f64::from(pixels) * 100.0 / f64::from(size.max(1)))
}

use data::Resource;
use engine::{InvestigatorState, View};

use crate::views::names::{card, monster_name, sheet};
use crate::views::spot::ImageToken;

/// The human investigator's sheet: values, sliders, cards and trophies.
#[derive(Debug, Clone)]
pub struct SheetView {
    /// Name.
    pub name: String,
    /// Title (« la Journaliste »…).
    pub title: String,
    /// Sheet image path under `/assets/`.
    pub image: String,
    /// Where the investigator stands.
    pub place: String,
    /// Sanity, and its maximum.
    pub sanity: (u8, u8),
    /// Stamina, and its maximum.
    pub stamina: (u8, u8),
    /// Money.
    pub money: u16,
    /// Clue tokens.
    pub clues: u16,
    /// Focus.
    pub focus: u8,
    /// The three sliders.
    pub sliders: Vec<SliderView>,
    /// Cards held: name, image, and whether exhausted.
    pub cards: Vec<(ImageToken, bool)>,
    /// Monster trophies, by name.
    pub monster_trophies: Vec<String>,
    /// Gate trophies.
    pub gate_trophies: usize,
    /// Statuses worth a line (delayed, arrested…).
    pub flags: Vec<&'static str>,
    /// Special ability: name and text.
    pub ability: (String, String),
}

/// One slider: the two skills it sets, and each stop's values.
#[derive(Debug, Clone)]
pub struct SliderView {
    /// « Vitesse / Discrétion ».
    pub skills: String,
    /// Each stop: the two values, and whether the slider is there.
    pub stops: Vec<(u8, u8, bool)>,
}

impl SheetView {
    /// The sheet of the investigator at `seat`.
    pub fn new(view: &View<'_, '_>, seat: usize) -> Option<Self> {
        let data = view.data();
        let inv = view.state().investigators.get(seat)?;
        let sheet = sheet(data, &inv.id)?;
        Some(Self {
            name: sheet.name.clone(),
            title: sheet.title.clone(),
            image: sheet.image.clone(),
            place: view.place_name(&inv.place),
            sanity: (inv.sanity, view.maximum(seat, Resource::Sanity)),
            stamina: (inv.stamina, view.maximum(seat, Resource::Stamina)),
            money: inv.money,
            clues: inv.clues,
            focus: sheet.focus,
            sliders: sliders(&sheet.skills, inv),
            cards: inv
                .cards
                .iter()
                .map(|held| (card(data, &held.card), held.exhausted))
                .collect(),
            monster_trophies: inv
                .monster_trophies
                .iter()
                .map(|id| monster_name(data, id))
                .collect(),
            gate_trophies: inv.gate_trophies.len(),
            flags: flags(inv),
            ability: (sheet.ability.name.clone(), sheet.ability.text.clone()),
        })
    }
}

/// The three sliders, pairing the six skills two by two.
fn sliders(tracks: &data::SkillTracks, inv: &InvestigatorState) -> Vec<SliderView> {
    let named = tracks.named();
    (0..3)
        .map(|track| {
            let (left_name, left) = named[track * 2];
            let (right_name, right) = named[track * 2 + 1];
            SliderView {
                skills: format!("{left_name} / {right_name}"),
                stops: left
                    .iter()
                    .zip(right)
                    .enumerate()
                    .map(|(stop, (a, b))| (*a, *b, usize::from(inv.sliders[track]) == stop))
                    .collect(),
            }
        })
        .collect()
}

/// Statuses shown on the sheet.
fn flags(inv: &InvestigatorState) -> Vec<&'static str> {
    [
        (inv.delayed, "Retardé"),
        (inv.lose_next_turn, "Perdra son prochain tour"),
        (inv.skipping_turn, "Passe ce tour"),
        (inv.explored, "A exploré l'Autre Monde"),
        (inv.no_more_bank_loans, "Plus de prêt bancaire"),
        (inv.eliminated, "Éliminé"),
    ]
    .into_iter()
    .filter_map(|(on, label)| on.then_some(label))
    .collect()
}

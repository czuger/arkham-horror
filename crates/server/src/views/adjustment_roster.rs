use std::collections::BTreeMap;

use engine::View;

use crate::play_mode::PlayMode;
use crate::views::names::sheet;
use crate::views::skill_adjustment::{Adjuster, skill_labels};

/// The Upkeep adjustment screen: every investigator, each with a button to
/// adjust their skills, and one button to move on.
#[derive(Debug, Clone)]
pub struct AdjustmentRoster {
    /// The investigators, in seat order from the first player.
    pub rows: Vec<AdjusterRow>,
    /// Number of answers given so far, sent back with the form.
    pub move_number: usize,
}

/// One investigator on the adjustment screen.
#[derive(Debug, Clone)]
pub struct AdjusterRow {
    /// Seat, for the link to their adjustment page.
    pub seat: usize,
    /// Name.
    pub name: String,
    /// Sheet image, relative to `assets/`.
    pub image: String,
    /// The three skill pairs, as picked if they were, else as they stand.
    pub skills: Vec<String>,
    /// Whether the human may adjust them now.
    pub can_adjust: bool,
    /// Concentration, when they adjust.
    pub focus: Option<u8>,
    /// « Curseurs déplacés », « Joué par l'IA »…
    pub note: String,
}

impl AdjustmentRoster {
    /// The screen for the adjustment under way.
    ///
    /// # Arguments
    ///
    /// * `adjusting` - who adjusts now and next, see
    ///   [`crate::views::adjusters`].
    /// * `planned` - the stops already picked, by seat.
    /// * `mode` - who plays which seat.
    /// * `move_number` - [`crate::session::Session::move_number`].
    pub fn new(
        view: &View<'_, '_>,
        adjusting: &[Adjuster],
        planned: &BTreeMap<usize, [u8; 3]>,
        mode: PlayMode,
        move_number: usize,
    ) -> Self {
        let rows = view
            .state()
            .seat_order()
            .into_iter()
            .filter_map(|seat| {
                let adjuster = adjusting.iter().find(|adjuster| adjuster.seat == seat);
                row(view, seat, adjuster, planned.get(&seat).copied(), mode)
            })
            .collect();
        Self { rows, move_number }
    }
}

/// The row of the investigator at `seat`; `None` if unknown.
fn row(
    view: &View<'_, '_>,
    seat: usize,
    adjuster: Option<&Adjuster>,
    planned: Option<[u8; 3]>,
    mode: PlayMode,
) -> Option<AdjusterRow> {
    let inv = view.state().investigators.get(seat)?;
    let sheet = sheet(view.data(), &inv.id)?;
    let can_adjust = adjuster.is_some() && mode.human(seat);
    let note = match (adjuster, mode.human(seat), planned) {
        (None, _, _) => "Pas d'ajustement ce tour.",
        (Some(_), false, _) => "Ajusté par l'IA.",
        (Some(_), true, Some(stops)) if stops != inv.sliders => "Curseurs déplacés.",
        (Some(_), true, _) => "Curseurs inchangés.",
    };
    Some(AdjusterRow {
        seat,
        name: view.name(seat),
        image: sheet.image.clone(),
        skills: skill_labels(sheet, planned.unwrap_or(inv.sliders)),
        can_adjust,
        focus: adjuster.map(|adjuster| adjuster.focus),
        note: note.to_owned(),
    })
}

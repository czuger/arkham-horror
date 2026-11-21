use data::Investigator;
use engine::{Choice, Context, Decision, DecisionKind, View};

use crate::views::decision_view::DecisionView;
use crate::views::investigator_mat::slider_spot;

/// Whether a decision is answered on the investigator's own page rather
/// than on the game page: slider placement and adjustment, and the weapons
/// and spells taken in hand before a combat.
pub fn on_sheet(decision: &Decision) -> bool {
    matches!(
        decision.kind,
        DecisionKind::PlaceSliders | DecisionKind::AdjustSkills
    ) || prepares_combat(decision)
}

/// Whether a decision picks the weapons and spells held in a combat.
pub fn prepares_combat(decision: &Decision) -> bool {
    decision.kind == DecisionKind::PrepareCombat
        && matches!(decision.context, Context::Combat { .. })
}

/// A decision answered on the investigator's page: stops to click on the
/// sheet, cards to drag (see [`crate::views::HandZones`]), and buttons for
/// the rest.
#[derive(Debug, Clone)]
pub struct SheetDecision {
    /// Who decides, the question and the move number.
    pub form: DecisionView,
    /// Slider stops that answer it, drawn on the sheet.
    pub stops: Vec<StopTarget>,
    /// The other answers, as buttons.
    pub buttons: Vec<(usize, String)>,
}

/// A slider stop that can be clicked on the sheet.
#[derive(Debug, Clone)]
pub struct StopTarget {
    /// Option index.
    pub index: usize,
    /// Left of the stop, in percent of the sheet.
    pub left: String,
    /// Top of the stop, in percent of the sheet.
    pub top: String,
    /// What the click does.
    pub label: String,
}

impl SheetDecision {
    /// The page form of a decision taken on `sheet`.
    pub fn new(
        view: &View<'_, '_>,
        decision: &Decision,
        move_number: usize,
        sheet: &Investigator,
    ) -> Self {
        let mut stops = Vec::new();
        let mut buttons = Vec::new();
        let cards_move = prepares_combat(decision);
        for (index, option) in decision.options.iter().enumerate() {
            match &option.choice {
                Choice::Slider { track, stop } => match slider_spot(sheet, *track, *stop) {
                    Some((left, top)) => stops.push(StopTarget {
                        index,
                        left,
                        top,
                        label: option.label.clone(),
                    }),
                    None => buttons.push((index, option.label.clone())),
                },
                Choice::Card(_) | Choice::Drop(_) if cards_move => {}
                _ => buttons.push((index, option.label.clone())),
            }
        }
        Self {
            form: DecisionView::new(view, decision, move_number),
            stops,
            buttons,
        }
    }
}

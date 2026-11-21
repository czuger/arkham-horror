use engine::{Context, Decision, View, of_check};

use crate::views::names::monster_name;
use crate::views::sheet_decision::on_sheet;

/// The decision the human must take, as a form.
#[derive(Debug, Clone)]
pub struct DecisionView {
    /// Who decides.
    pub who: String,
    /// The question.
    pub prompt: String,
    /// What the decision is about, in a few words (successes so far…).
    pub hint: String,
    /// The answers: index and label.
    pub options: Vec<(usize, String)>,
    /// Number of answers given so far, sent back with the form.
    pub move_number: usize,
    /// Answered on the investigator's page, not with these buttons.
    pub on_sheet: bool,
    /// Answered on the combat page, not with these buttons.
    pub in_combat: bool,
}

impl DecisionView {
    /// The form for a decision.
    pub fn new(view: &View<'_, '_>, decision: &Decision, move_number: usize) -> Self {
        Self {
            who: view.name(decision.who),
            prompt: decision.prompt.clone(),
            hint: hint(view, &decision.context),
            options: decision
                .options
                .iter()
                .enumerate()
                .map(|(index, option)| (index, option.label.clone()))
                .collect(),
            move_number,
            on_sheet: on_sheet(decision),
            in_combat: false,
        }
    }

    /// The same form, answered on the combat page when `in_combat`.
    pub fn in_combat(mut self, in_combat: bool) -> Self {
        self.in_combat = in_combat && !self.on_sheet;
        self
    }
}

/// A short French line about the decision's context.
fn hint(view: &View<'_, '_>, context: &Context) -> String {
    match context {
        Context::Check {
            kind,
            successes,
            needed,
            ..
        } => format!(
            "Test {} : {successes} succès sur {needed} nécessaire(s).",
            of_check(*kind)
        ),
        Context::Combat { foe: Some(foe), .. } => {
            format!("Combat contre : {}.", monster_name(view.data(), foe))
        }
        Context::Monsters(monsters) if !monsters.is_empty() => {
            let names: Vec<String> = monsters
                .iter()
                .map(|id| monster_name(view.data(), id))
                .collect();
            format!("Monstres présents : {}.", names.join(", "))
        }
        Context::Gate {
            modifier, needed, ..
        } => format!("Fermer : modificateur {modifier:+}, {needed} succès nécessaire(s)."),
        _ => String::new(),
    }
}

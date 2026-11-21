use engine::{Context, Decision, View, of_check};

use crate::views::names::monster_name;

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
        }
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

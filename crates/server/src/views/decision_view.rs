use engine::{CardAim, Choice, Context, Decision, View, of_check};

use crate::views::image_token::ImageToken;
use crate::views::names::{card, monster_name};
use crate::views::sheet_decision::on_sheet;
use crate::views::skill_adjustment::adjusts_skills;
use crate::views::trade_view::is_trade;

/// The decision the human must take, as a form.
#[derive(Debug, Clone)]
pub struct DecisionView {
    /// Who decides.
    pub who: String,
    /// The question.
    pub prompt: String,
    /// What the decision is about, in a few words (successes so far…).
    pub hint: String,
    /// The answers given by plain buttons: index and label.
    pub options: Vec<(usize, String)>,
    /// When picking a card (to keep, buy, sell or lose): the cards, shown at
    /// their own size, each a button.
    pub cards: Vec<CardOption>,
    /// Number of answers given so far, sent back with the form.
    pub move_number: usize,
    /// Answered on the investigator's page, not with these buttons.
    pub on_sheet: bool,
    /// Answered on the combat page, not with these buttons.
    pub in_combat: bool,
    /// The Upkeep skill adjustment: the answer that leaves the sliders as
    /// they are, the rest being set on the adjustment page.
    pub keep_skills: Option<usize>,
    /// A trade being put together, on the trade page.
    pub trading: bool,
}

/// A card that can be picked: the answer index, the card and the button text.
#[derive(Debug, Clone)]
pub struct CardOption {
    /// Answer index.
    pub index: usize,
    /// Name and image.
    pub token: ImageToken,
    /// Button text (« Acheter Revolver .38 (4 $) »).
    pub label: String,
}

impl DecisionView {
    /// The form for a decision.
    pub fn new(view: &View<'_, '_>, decision: &Decision, move_number: usize) -> Self {
        let cards = card_choices(view, decision);
        Self {
            who: view.name(decision.who),
            prompt: decision.prompt.clone(),
            hint: hint(view, &decision.context),
            options: decision
                .options
                .iter()
                .enumerate()
                .filter(|(index, _)| !cards.iter().any(|card| card.index == *index))
                .map(|(index, option)| (index, option.label.clone()))
                .collect(),
            cards,
            move_number,
            on_sheet: on_sheet(decision),
            in_combat: false,
            keep_skills: keep_skills(decision),
            trading: is_trade(decision),
        }
    }

    /// The same form, answered on the combat page when `in_combat`.
    pub fn in_combat(mut self, in_combat: bool) -> Self {
        self.in_combat = in_combat && !self.on_sheet;
        self
    }
}

/// In a skill adjustment, the answer that ends it at once.
fn keep_skills(decision: &Decision) -> Option<usize> {
    if !adjusts_skills(decision) {
        return None;
    }
    decision
        .options
        .iter()
        .position(|option| option.choice == Choice::Done)
}

/// The cards to pick from when keeping, buying, selling or losing one; empty
/// for any other decision.
fn card_choices(view: &View<'_, '_>, decision: &Decision) -> Vec<CardOption> {
    let Context::Cards(aim) = decision.context else {
        return Vec::new();
    };
    let verb = match aim {
        CardAim::Keep => "Prendre",
        CardAim::Buy => "Acheter",
        CardAim::Sell => "Vendre",
        CardAim::Lose => "Défausser",
    };
    decision
        .options
        .iter()
        .enumerate()
        .filter_map(|(index, option)| match &option.choice {
            Choice::Card(id) => Some(CardOption {
                index,
                token: card(view.data(), id),
                label: format!("{verb} {}", option.label),
            }),
            _ => None,
        })
        .collect()
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

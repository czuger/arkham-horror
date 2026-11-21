use ai::Reason;
use data::Resource;
use engine::{Decision, View};

use crate::play_mode::PlayMode;
use crate::views::names::investigator_name;
use crate::views::tip::Tip;
use crate::views::tips;

/// One investigator in the table of every investigator.
#[derive(Debug, Clone)]
pub struct PartyRow {
    /// Name.
    pub name: String,
    /// Whether it is shown as the player's own (see [`PlayMode::highlighted`]).
    pub human: bool,
    /// Whether this investigator is the first player.
    pub first: bool,
    /// Where they stand.
    pub place: String,
    /// Sanity, and its maximum.
    pub sanity: (u8, u8),
    /// Stamina, and its maximum.
    pub stamina: (u8, u8),
    /// Money.
    pub money: u16,
    /// Clue tokens.
    pub clues: u16,
    /// Number of cards held.
    pub cards: usize,
    /// What the AI means to do this turn, if it says.
    pub plan: String,
    /// Whether they left the game.
    pub eliminated: bool,
    /// Bubble shown on hover over the name.
    pub tip: Tip,
}

impl PartyRow {
    /// Every investigator, in seat order.
    pub fn all(view: &View<'_, '_>, decision: Option<&Decision>, mode: PlayMode) -> Vec<Self> {
        let state = view.state();
        state
            .investigators
            .iter()
            .enumerate()
            .map(|(seat, inv)| Self {
                name: investigator_name(view.data(), &inv.id),
                human: mode.highlighted(seat, decision),
                first: seat == state.first_player,
                place: view.place_name(&inv.place),
                sanity: (inv.sanity, view.maximum(seat, Resource::Sanity)),
                stamina: (inv.stamina, view.maximum(seat, Resource::Stamina)),
                money: inv.money,
                clues: inv.clues,
                cards: inv.cards.len(),
                plan: if mode.human(seat) {
                    String::new()
                } else {
                    plan(view, seat)
                },
                eliminated: inv.eliminated,
                tip: tips::investigator(view, seat),
            })
            .collect()
    }
}

/// « Fermer le portail de la Gare », for the AI seats in Arkham.
fn plan(view: &View<'_, '_>, seat: usize) -> String {
    let Some((place, reason)) = ai::intention(view, seat) else {
        return String::new();
    };
    let target = view.place_name(&place);
    match reason {
        Reason::Close => format!("fermer le portail ({target})"),
        Reason::Heal => format!("se soigner ({target})"),
        Reason::Gate => format!("passer le portail ({target})"),
        Reason::Clues => format!("prendre des Indices ({target})"),
        Reason::Hunt => format!("chasser un monstre ({target})"),
        Reason::Shop => format!("faire des achats ({target})"),
        Reason::Rumor => format!("agir contre la Rumeur ({target})"),
        Reason::Stay => "rester sur place".to_owned(),
    }
}

//! The « Lieu fermé » marker on a closed location.

use data::{GameData, LocationId, MythosCard, Ongoing};
use engine::{GameState, View};

use crate::views::image_token::ImageToken;
use crate::views::tip::Tip;

/// The marker for a location closed for good or by a Mythos card in play;
/// `None` when the location is open.
pub fn closed_marker(view: &View<'_, '_>, id: &LocationId) -> Option<ImageToken> {
    if !view.is_closed(id) {
        return None;
    }
    let data = view.data();
    let state = view.state();
    let reason = match closing_card(data, state, id) {
        Some(card) if !state.is_closed(id) => {
            format!("Fermé par la carte Mythe « {} ».", card.name)
        }
        _ => "Fermé jusqu'à la fin de la partie.".to_owned(),
    };
    let name = view.place_name(&engine::Place::Location(id.clone()));
    Some(ImageToken {
        name: format!("{name} : lieu fermé"),
        image: data.board.closed_marker.clone(),
        tip: Tip::new("Lieu fermé".to_owned())
            .line(name)
            .line(reason)
            .line("Personne ne peut y entrer.".to_owned()),
    })
}

/// The Mythos card in play that closes the location, if any.
fn closing_card<'d>(
    data: &'d GameData,
    state: &GameState,
    id: &LocationId,
) -> Option<&'d MythosCard> {
    let closes = Ongoing::LocationClosed(id.clone());
    state
        .environment
        .iter()
        .chain(state.rumor.iter().map(|rumor| &rumor.card))
        .chain(state.lingering.iter().map(|(card, _)| card))
        .filter_map(|card| data.mythos_cards.cards.iter().find(|c| &c.id == card))
        .find(|card| card.ongoing.contains(&closes))
}

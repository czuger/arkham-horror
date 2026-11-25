//! The page shown after an Other World encounter, so that it does not go by
//! unseen when no answer was needed.

use engine::{Outcome, View};

use crate::illustrations::{IllustrationSize, illustration_url};
use crate::views::journal_line::{JournalLine, journal_lines};
use crate::views::other_world_span::OtherWorldSpan;
use crate::views::scene_view::area_label;
use crate::views::world_scene::visit_illustration;

/// An Other World encounter: the area's picture and every line of it.
#[derive(Debug, Clone)]
pub struct OtherWorldView {
    /// Who had the encounter.
    pub who: String,
    /// The world's name.
    pub world: String,
    /// « Autre Monde, 1re zone ».
    pub area: String,
    /// The area's illustration, if any.
    pub image: Option<String>,
    /// What happened, with the card drawn and the dice rolled.
    pub lines: Vec<JournalLine>,
    /// Where the investigator stands now.
    pub now_at: String,
}

impl OtherWorldView {
    /// The encounter `span` of the investigator at `seat`.
    pub fn new(
        view: &View<'_, '_>,
        seat: usize,
        outcome: &Outcome,
        span: &OtherWorldSpan,
    ) -> Option<Self> {
        let data = view.data();
        let inv = view.state().investigators.get(seat)?;
        let to = span.end.unwrap_or(outcome.events.len());
        Some(Self {
            who: view.name(seat),
            world: data.board.other_world(&span.world)?.name.clone(),
            area: area_label(span.area).to_owned(),
            image: visit_illustration(view, seat, &span.world, span.area, inv.world_visits)
                .map(|path| illustration_url(IllustrationSize::Large, &path)),
            lines: journal_lines(view, &outcome.events, &outcome.shown, span.start, to),
            now_at: view.place_name(&inv.place),
        })
    }
}

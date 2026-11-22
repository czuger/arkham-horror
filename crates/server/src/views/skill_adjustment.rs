use data::{Investigator, SkillTracks};
use engine::{Context, Decision, DecisionKind, View};

use crate::views::investigator_mat::slider_spot;
use crate::views::names::sheet;

/// French names of the two skills of each slider track.
const TRACKS: [(&str, &str); 3] = [
    ("Vitesse", "Discrétion"),
    ("Vigueur", "Volonté"),
    ("Savoir", "Chance"),
];

/// Whether a decision is the Upkeep skill adjustment.
pub fn adjusts_skills(decision: &Decision) -> bool {
    decision.kind == DecisionKind::AdjustSkills
}

/// The Concentration left in a skill adjustment, `None` for any other
/// decision.
pub fn focus_left(decision: &Decision) -> Option<u8> {
    match decision.context {
        Context::Focus { remaining } if adjusts_skills(decision) => Some(remaining),
        _ => None,
    }
}

/// The skill adjustment page: the sheet, each track's stops to pick, and the
/// Concentration that bounds the moves.
#[derive(Debug, Clone)]
pub struct SkillAdjustment {
    /// Who adjusts.
    pub name: String,
    /// Sheet image, relative to `assets/`.
    pub image: String,
    /// Concentration left: the most stops moved in all.
    pub focus: u8,
    /// The three tracks.
    pub tracks: Vec<TrackChoice>,
    /// Number of answers given so far, sent back with the form.
    pub move_number: usize,
}

/// One slider track and the stops it may go to.
#[derive(Debug, Clone)]
pub struct TrackChoice {
    /// Form field (`piste0`…).
    pub field: String,
    /// « Vitesse / Discrétion ».
    pub name: String,
    /// The stop the slider stands on.
    pub current: u8,
    /// Every stop of the track.
    pub stops: Vec<StopChoice>,
}

/// A stop the slider may be put on.
#[derive(Debug, Clone)]
pub struct StopChoice {
    /// Stop number, from 0 (left).
    pub stop: u8,
    /// Left of the stop, in percent of the sheet.
    pub left: String,
    /// Top of the stop, in percent of the sheet.
    pub top: String,
    /// The two skill values there (« Vitesse 2 · Discrétion 3 »).
    pub label: String,
    /// Where the slider stands now.
    pub checked: bool,
    /// Within the Concentration left.
    pub reachable: bool,
}

impl SkillAdjustment {
    /// The page for the skill adjustment `decision`; `None` for any other
    /// decision or an unknown investigator.
    pub fn new(view: &View<'_, '_>, decision: &Decision, move_number: usize) -> Option<Self> {
        let focus = focus_left(decision)?;
        let inv = view.state().investigators.get(decision.who)?;
        let sheet = sheet(view.data(), &inv.id)?;
        let tracks = inv
            .sliders
            .iter()
            .enumerate()
            .map(|(track, current)| track_choice(sheet, track, *current, focus))
            .collect();
        Some(Self {
            name: view.name(decision.who),
            image: sheet.image.clone(),
            focus,
            tracks,
            move_number,
        })
    }
}

/// The stops of one track, with their place on the sheet.
fn track_choice(sheet: &Investigator, track: usize, current: u8, focus: u8) -> TrackChoice {
    let (left_skill, right_skill) = TRACKS.get(track).copied().unwrap_or(("?", "?"));
    let stops = (0..SkillTracks::STOPS)
        .filter_map(|stop| u8::try_from(stop).ok())
        .filter_map(|stop| {
            let (left, top) = slider_spot(sheet, track, stop)?;
            Some(StopChoice {
                stop,
                left,
                top,
                label: stop_label(&sheet.skills, track, stop, (left_skill, right_skill)),
                checked: stop == current,
                reachable: stop.abs_diff(current) <= focus,
            })
        })
        .collect();
    TrackChoice {
        field: format!("piste{track}"),
        name: format!("{left_skill} / {right_skill}"),
        current,
        stops,
    }
}

/// « Vitesse 2 · Discrétion 3 »: the two values printed at a stop.
fn stop_label(skills: &SkillTracks, track: usize, stop: u8, names: (&str, &str)) -> String {
    let named = skills.named();
    let value = |index: usize| {
        named
            .get(index)
            .and_then(|(_, values)| values.get(usize::from(stop)))
            .map_or_else(|| "?".to_owned(), u8::to_string)
    };
    format!(
        "{} {} · {} {}",
        names.0,
        value(2 * track),
        names.1,
        value(2 * track + 1)
    )
}

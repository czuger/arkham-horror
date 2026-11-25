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

/// An investigator who adjusts their skills in the Upkeep under way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Adjuster {
    /// Seat.
    pub seat: usize,
    /// Concentration left: the most stops moved in all.
    pub focus: u8,
}

/// Who adjusts their skills now and next, in order, with their
/// Concentration; empty for any other decision.
pub fn adjusters(view: &View<'_, '_>, decision: &Decision) -> Vec<Adjuster> {
    let Context::Focus { remaining, after } = &decision.context else {
        return Vec::new();
    };
    if !adjusts_skills(decision) {
        return Vec::new();
    }
    let now = Adjuster {
        seat: decision.who,
        focus: *remaining,
    };
    let next = after.iter().filter_map(|&seat| {
        let inv = view.state().investigators.get(seat)?;
        let focus = sheet(view.data(), &inv.id)?.focus;
        Some(Adjuster { seat, focus })
    });
    std::iter::once(now).chain(next).collect()
}

/// One investigator's skill adjustment page: the sheet, each track's stops
/// to pick, and the Concentration that bounds the moves.
#[derive(Debug, Clone)]
pub struct SkillAdjustment {
    /// Seat of who adjusts.
    pub seat: usize,
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
    /// Where the slider stands now, or the stop already picked for it.
    pub checked: bool,
    /// Within the Concentration left.
    pub reachable: bool,
}

impl SkillAdjustment {
    /// The page of `adjuster`, with the stops already `planned` for them
    /// picked; `None` for an unknown investigator.
    pub fn new(
        view: &View<'_, '_>,
        adjuster: Adjuster,
        planned: Option<[u8; 3]>,
        move_number: usize,
    ) -> Option<Self> {
        let inv = view.state().investigators.get(adjuster.seat)?;
        let sheet = sheet(view.data(), &inv.id)?;
        let targets = planned.unwrap_or(inv.sliders);
        let tracks = inv
            .sliders
            .iter()
            .zip(targets)
            .enumerate()
            .map(|(track, (current, target))| {
                track_choice(sheet, track, (*current, target), adjuster.focus)
            })
            .collect();
        Some(Self {
            seat: adjuster.seat,
            name: view.name(adjuster.seat),
            image: sheet.image.clone(),
            focus: adjuster.focus,
            tracks,
            move_number,
        })
    }
}

/// « Vitesse 2 · Discrétion 3 », … : the skill values of a sheet with its
/// sliders on `stops`.
pub fn skill_labels(sheet: &Investigator, stops: [u8; 3]) -> Vec<String> {
    TRACKS
        .iter()
        .zip(stops)
        .enumerate()
        .map(|(track, (names, stop))| stop_label(&sheet.skills, track, stop, *names))
        .collect()
}

/// The stops of one track, with their place on the sheet; `stops` are where
/// the slider stands and where it is to go.
fn track_choice(sheet: &Investigator, track: usize, stops: (u8, u8), focus: u8) -> TrackChoice {
    let (current, target) = stops;
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
                checked: stop == target,
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

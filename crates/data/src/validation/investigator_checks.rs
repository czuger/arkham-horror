//! Consistency checks on the investigator sheets.

use crate::board::Board;
use crate::investigator::{Investigator, Investigators, SkillTracks};
use crate::validation::board_checks::report_duplicates;
use crate::validation::issue::Issue;

/// Ids are unique, homes exist, and every sheet is complete.
pub(crate) fn check_investigators(
    investigators: &Investigators,
    board: &Board,
    issues: &mut Vec<Issue>,
) {
    report_duplicates(
        "investigateur",
        investigators
            .investigators
            .iter()
            .map(|sheet| sheet.id.as_str()),
        issues,
    );
    for sheet in &investigators.investigators {
        if board.location(&sheet.home).is_none() {
            issues.push(Issue::UnknownReference {
                context: format!("maison de « {} »", sheet.id),
                family: "lieu",
                id: sheet.home.to_string(),
            });
        }
        check_positive_values(sheet, issues);
        check_tracks(sheet, issues);
    }
}

/// Sanity, Stamina and Focus are all positive.
fn check_positive_values(sheet: &Investigator, issues: &mut Vec<Issue>) {
    for (label, value) in [
        ("Santé Mentale", sheet.sanity),
        ("Résistance", sheet.stamina),
        ("Concentration", sheet.focus),
    ] {
        if value == 0 {
            issues.push(Issue::Zero {
                context: format!("{label} de « {} »", sheet.id),
            });
        }
    }
}

/// Every skill track has one value per slider stop.
fn check_tracks(sheet: &Investigator, issues: &mut Vec<Issue>) {
    for (skill, values) in sheet.skills.named() {
        if values.len() != SkillTracks::STOPS {
            issues.push(Issue::WrongTrackLength {
                investigator: sheet.id.to_string(),
                skill,
                found: values.len(),
            });
        }
    }
}

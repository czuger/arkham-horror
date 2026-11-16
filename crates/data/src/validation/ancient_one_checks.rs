//! Consistency checks on the Ancient One sheets.

use crate::ancient_one::AncientOnes;
use crate::monster::MonsterAbility;
use crate::validation::board_checks::report_duplicates;
use crate::validation::issue::Issue;

/// Ids are unique, defenses are resistances or immunities, doom tracks are not empty.
pub(crate) fn check_ancient_ones(ancient_ones: &AncientOnes, issues: &mut Vec<Issue>) {
    report_duplicates(
        "Grand Ancien",
        ancient_ones
            .ancient_ones
            .iter()
            .map(|sheet| sheet.id.as_str()),
        issues,
    );
    for sheet in &ancient_ones.ancient_ones {
        if !sheet.defenses.iter().copied().all(is_defense) {
            issues.push(Issue::NotADefense {
                ancient_one: sheet.id.to_string(),
            });
        }
        if sheet.doom_track == 0 {
            issues.push(Issue::Zero {
                context: format!("échelle du destin de « {} »", sheet.id),
            });
        }
    }
}

/// Whether a keyword can appear in the « Défenses » box.
fn is_defense(ability: MonsterAbility) -> bool {
    matches!(
        ability,
        MonsterAbility::PhysicalResistance
            | MonsterAbility::PhysicalImmunity
            | MonsterAbility::MagicalResistance
            | MonsterAbility::MagicalImmunity
    )
}

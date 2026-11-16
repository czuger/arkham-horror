//! Consistency checks on the monster tokens.

use crate::monster::Monsters;
use crate::validation::board_checks::report_duplicates;
use crate::validation::issue::Issue;

/// Monster ids are unique and every kind has at least one token.
pub(crate) fn check_monsters(monsters: &Monsters, issues: &mut Vec<Issue>) {
    report_duplicates(
        "monstre",
        monsters.monsters.iter().map(|monster| monster.id.as_str()),
        issues,
    );
    for monster in &monsters.monsters {
        if monster.copies == 0 {
            issues.push(Issue::ZeroCopies {
                context: format!("monstre « {} »", monster.id),
            });
        }
    }
}

use data::MonsterId;
use engine::{CombatEnd, Exhibit, Shown};

/// One investigator facing a monster, as the journal tells it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombatSpan {
    /// The monster.
    pub foe: MonsterId,
    /// First journal line of the combat.
    pub start: usize,
    /// How it ended, and the journal line after its last; `None` while it
    /// goes on.
    pub end: Option<(CombatEnd, usize)>,
}

impl CombatSpan {
    /// The last time the investigator at `seat` faced a monster.
    pub fn last(shown: &[Shown], seat: usize) -> Option<Self> {
        let at = shown.iter().rposition(
            |item| matches!(&item.what, Exhibit::CombatStart { seat: s, .. } if *s == seat),
        )?;
        let Exhibit::CombatStart { foe, .. } = &shown[at].what else {
            return None;
        };
        let end = shown[at..].iter().find_map(|item| match &item.what {
            Exhibit::CombatEnd { seat: s, end } if *s == seat => Some((*end, item.line)),
            _ => None,
        });
        Some(Self {
            foe: foe.clone(),
            start: shown[at].line,
            end,
        })
    }

    /// The seat of the investigator who faced a monster last.
    pub fn last_seat(shown: &[Shown]) -> Option<usize> {
        shown.iter().rev().find_map(|item| match &item.what {
            Exhibit::CombatStart { seat, .. } => Some(*seat),
            _ => None,
        })
    }

    /// Whether the combat goes on.
    pub fn ongoing(&self) -> bool {
        self.end.is_none()
    }

    /// Whether the combat ended at or after journal line `line`.
    pub fn ended_since(&self, line: usize) -> bool {
        self.end.is_some_and(|(_, at)| at >= line)
    }
}

/// How a combat ended, in French.
pub fn end_label(end: CombatEnd) -> &'static str {
    match end {
        CombatEnd::Evaded => "Vous avez échappé au monstre.",
        CombatEnd::Defeated => "Vous avez vaincu le monstre.",
        CombatEnd::Fled => "Vous avez fui le combat.",
        CombatEnd::KnockedOut => "Vous avez perdu connaissance ou la raison.",
        CombatEnd::Stopped => "Le combat s'est arrêté.",
    }
}

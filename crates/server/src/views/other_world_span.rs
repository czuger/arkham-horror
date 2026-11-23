use data::OtherWorldId;
use engine::{Area, Exhibit, Shown};

/// One investigator's Other World encounter, as the journal tells it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtherWorldSpan {
    /// The world.
    pub world: OtherWorldId,
    /// The area the investigator stood in.
    pub area: Area,
    /// First journal line of the encounter.
    pub start: usize,
    /// The journal line after its last; `None` while it goes on.
    pub end: Option<usize>,
}

impl OtherWorldSpan {
    /// The last Other World encounter of the investigator at `seat`.
    pub fn last(shown: &[Shown], seat: usize) -> Option<Self> {
        let at = shown.iter().rposition(
            |item| matches!(&item.what, Exhibit::OtherWorldStart { seat: s, .. } if *s == seat),
        )?;
        let Exhibit::OtherWorldStart { world, area, .. } = &shown[at].what else {
            return None;
        };
        let end = shown[at..].iter().find_map(|item| match &item.what {
            Exhibit::OtherWorldEnd { seat: s } if *s == seat => Some(item.line),
            _ => None,
        });
        Some(Self {
            world: world.clone(),
            area: *area,
            start: shown[at].line,
            end,
        })
    }

    /// The seat of the investigator who had an Other World encounter last.
    pub fn last_seat(shown: &[Shown]) -> Option<usize> {
        shown.iter().rev().find_map(|item| match &item.what {
            Exhibit::OtherWorldStart { seat, .. } => Some(*seat),
            _ => None,
        })
    }

    /// Whether the encounter ended after the first `seen` journal lines were
    /// read, as [`crate::views::CombatSpan::ended_since`].
    pub fn ended_since(&self, seen: usize) -> bool {
        self.end.is_some_and(|at| at > seen)
    }
}

#[cfg(test)]
mod tests {
    use data::OtherWorldId;
    use engine::{Area, Exhibit, Shown};

    use super::OtherWorldSpan;

    #[test]
    fn an_encounter_ended_before_the_last_answer_is_not_shown_again() {
        let shown = [
            Shown {
                line: 40,
                what: Exhibit::OtherWorldStart {
                    seat: 0,
                    world: OtherWorldId::new("yuggoth"),
                    area: Area::Second,
                },
            },
            Shown {
                line: 43,
                what: Exhibit::OtherWorldEnd { seat: 0 },
            },
        ];
        let span = OtherWorldSpan::last(&shown, 0).expect("encounter");
        assert_eq!((span.start, span.end), (40, Some(43)));
        assert!(span.ended_since(42));
        assert!(!span.ended_since(43));
        assert!(OtherWorldSpan::last(&shown, 1).is_none());
    }
}

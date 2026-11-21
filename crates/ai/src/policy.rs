//! The priority-rule AI: one answer per decision kind.

use engine::{Context, Decision, DecisionKind, Responder, View};

use crate::{choices, fighting, movement, sliders};

/// Card abilities used in a row before the AI stops, so that a free
/// ability is never used forever.
const MOST_ABILITIES_IN_A_ROW: u32 = 3;

/// An AI investigator playing by simple priority rules: survive, close and
/// seal gates, pick up clue tokens, fight only with good odds.
#[derive(Debug, Clone, Default)]
pub struct Policy {
    /// Card abilities used in a row.
    abilities_in_a_row: u32,
}

impl Policy {
    /// A new AI.
    pub fn new() -> Self {
        Self::default()
    }

    /// The chosen option for a decision.
    pub fn choose(&mut self, view: &View<'_, '_>, decision: &Decision) -> usize {
        let seat = decision.who;
        let pick = match (&decision.kind, &decision.context) {
            (DecisionKind::PlaceSliders | DecisionKind::AdjustSkills, _) => {
                sliders::adjust(view, seat, decision)
            }
            (DecisionKind::Move, _) => movement::step(view, seat, decision),
            (DecisionKind::Destination, _) => movement::destination(view, seat, decision),
            (DecisionKind::FaceMonster, Context::Monsters(monsters)) => {
                fighting::face(view, seat, decision, monsters)
            }
            (DecisionKind::FightOrFlee, Context::Combat { foe, bonus }) => {
                fighting::fight_or_flee(view, seat, decision, foe.as_ref(), *bonus)
            }
            // Always look again at the cards in hand: `prepare` keeps what is good.
            (DecisionKind::ChangeHands, _) => decision.find(&engine::Choice::Yes).unwrap_or(0),
            (DecisionKind::PrepareCombat, Context::Combat { foe, .. }) => {
                fighting::prepare(view, seat, decision, foe.as_ref())
            }
            (
                DecisionKind::SpendClues,
                Context::Check {
                    kind, closing_gate, ..
                },
            ) => choices::spend_clue(view, seat, decision, *kind, *closing_gate),
            (DecisionKind::SpendClues, _) => choices::largest_number(decision),
            (DecisionKind::CloseGate, Context::Gate { modifier, .. }) => {
                choices::close_gate(view, seat, decision, *modifier)
            }
            (DecisionKind::PickCard, Context::Cards(aim)) => {
                choices::pick_card(view, seat, decision, *aim)
            }
            (DecisionKind::CardChoice, _) => choices::card_choice(view, seat, decision),
            (DecisionKind::Optional, _) => choices::optional(view, seat, decision),
            (DecisionKind::UseAbility, _) => self.ability(view, seat, decision),
            (DecisionKind::ReturnThroughGate, _) => choices::return_gate(view, seat, decision),
            (DecisionKind::Arbitration, _) => choices::arbitration(view, decision),
            _ => choices::best_effects(view, seat, decision)
                .or_else(|| decision.find(&engine::Choice::Yes))
                .unwrap_or(0),
        };
        if decision.kind != DecisionKind::UseAbility {
            self.abilities_in_a_row = 0;
        }
        pick.min(decision.options.len().saturating_sub(1))
    }

    /// A card ability, never more than a few in a row.
    fn ability(&mut self, view: &View<'_, '_>, seat: usize, decision: &Decision) -> usize {
        let done = decision.find(&engine::Choice::Done);
        if self.abilities_in_a_row >= MOST_ABILITIES_IN_A_ROW
            && let Some(done) = done
        {
            self.abilities_in_a_row = 0;
            return done;
        }
        let pick = choices::use_ability(view, seat, decision);
        if Some(pick) == done {
            self.abilities_in_a_row = 0;
        } else {
            self.abilities_in_a_row += 1;
        }
        pick
    }
}

impl Responder for Policy {
    fn answer(&mut self, view: &View<'_, '_>, decision: &Decision) -> Option<usize> {
        Some(self.choose(view, decision))
    }
}

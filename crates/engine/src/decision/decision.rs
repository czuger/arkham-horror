//! A decision the engine waits for.

use serde::{Deserialize, Serialize};

use data::Effect;

use crate::decision::choice::Choice;
use crate::decision::context::Context;

/// What the decision is about. The AI uses it to pick a policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionKind {
    /// Initial slider placement.
    PlaceSliders,
    /// Upkeep skill adjustment, within the focus.
    AdjustSkills,
    /// Order of the upkeep actions.
    UpkeepOrder,
    /// Where to go back from Lost in Time and Space.
    LeaveLostInTime,
    /// Next step of the Arkham movement.
    Move,
    /// Which gate to come back through from an Other World.
    ReturnThroughGate,
    /// Which monster to deal with next, and how.
    FaceMonster,
    /// Fight or flee during a combat.
    FightOrFlee,
    /// Before a combat: keep the weapons in hand or change them.
    ChangeHands,
    /// Which weapon or spell to use before a Combat check.
    PrepareCombat,
    /// Spend clue tokens on a failed check.
    SpendClues,
    /// Use an optional card ability now.
    UseAbility,
    /// Pick one option of an encounter or card.
    CardChoice,
    /// « Vous pouvez » : do it or not.
    Optional,
    /// Pick the cards to keep, buy, sell or discard.
    PickCard,
    /// Pick a place anywhere in Arkham.
    Destination,
    /// Try to close or seal the gate.
    CloseGate,
    /// Arbitration by the first player.
    Arbitration,
}

/// A choice someone must make before the game can go on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    /// Seat of the investigator who decides.
    pub who: usize,
    /// What it is about.
    pub kind: DecisionKind,
    /// The question, in French.
    pub prompt: String,
    /// The legal answers. The answer is an index into this list.
    pub options: Vec<DecisionOption>,
    /// What the decision is about.
    #[serde(default)]
    pub context: Context,
}

/// One legal answer with its French label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionOption {
    /// The answer.
    pub choice: Choice,
    /// Its label, in French.
    pub label: String,
    /// Card effects this answer leads to, when it picks a card paragraph or
    /// option (empty otherwise).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
}

impl Decision {
    /// Index of the first option equal to a choice.
    pub fn find(&self, choice: &Choice) -> Option<usize> {
        self.options
            .iter()
            .position(|option| &option.choice == choice)
    }
}

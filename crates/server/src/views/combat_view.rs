use data::{CheckKind, MonsterAbility, MonsterId, Resource};
use engine::{Choice, Context, Decision, DecisionKind, Outcome, View};

use crate::illustrations::{IllustrationSize, illustration_url};
use crate::views::combat_span::{CombatSpan, end_label};
use crate::views::decision_view::DecisionView;
use crate::views::image_token::ImageToken;
use crate::views::journal_line::{JournalLine, journal_lines};
use crate::views::names::monster;
use crate::views::prepares_combat;

/// The human investigator's combat: the monster, the investigator, every
/// line and roll of the combat, and the decision to take.
#[derive(Debug, Clone)]
pub struct CombatView {
    /// The investigator's name.
    pub who: String,
    /// Sanity, and its maximum.
    pub sanity: (u8, u8),
    /// Stamina, and its maximum.
    pub stamina: (u8, u8),
    /// Clue tokens.
    pub clues: u16,
    /// Lowest die face that is a success.
    pub success_from: u8,
    /// The monster of the combat under way or just ended.
    pub foe: Option<FoeView>,
    /// The journal of that combat, with every roll.
    pub lines: Vec<JournalLine>,
    /// How it ended, when it did.
    pub ending: Option<&'static str>,
    /// Monsters to face next (evade or fight), when that is the decision.
    pub to_face: Vec<FoeView>,
}

/// A monster faced: both sides of its token and its current values.
#[derive(Debug, Clone)]
pub struct FoeView {
    /// Name, token image and bubble.
    pub token: ImageToken,
    /// Its values, one per line.
    pub stats: Vec<String>,
    /// Dice the investigator would roll against it, one check per line.
    pub dice: Vec<String>,
    /// Link to its large illustrated portrait.
    pub portrait: String,
}

/// The decision taken on the combat page.
#[derive(Debug, Clone)]
pub struct CombatDecision {
    /// Who decides, the question and the move number.
    pub form: DecisionView,
    /// The answers given by buttons.
    pub buttons: Vec<(usize, String)>,
    /// Before a round: fight or flee with the cards in hand, or change them.
    pub hands: Option<HandsChoice>,
}

/// Before a round: keep the cards in hand and fight or flee, or change them.
#[derive(Debug, Clone)]
pub struct HandsChoice {
    /// Option that keeps the cards in hand.
    pub keep: usize,
    /// Option that changes them.
    pub change: usize,
    /// What is in hand.
    pub held: String,
    /// Whether fleeing is allowed (no Ambush).
    pub can_flee: bool,
}

impl CombatView {
    /// The monster the scene shows: the one fought, else the first to face.
    pub fn scene_foe(&self) -> Option<&FoeView> {
        self.foe.as_ref().or_else(|| self.to_face.first())
    }

    /// The combat page of the investigator at `seat`.
    ///
    /// # Arguments
    ///
    /// * `span` - the last combat, shown when under way or ended since `seen`.
    /// * `seen` - journal lines the player had seen at their last answer.
    pub fn new(
        view: &View<'_, '_>,
        seat: usize,
        outcome: &Outcome,
        span: Option<&CombatSpan>,
        seen: usize,
        decision: Option<&Decision>,
    ) -> Option<Self> {
        let inv = view.state().investigators.get(seat)?;
        let bonus = decision.and_then(|decision| match &decision.context {
            Context::Combat { bonus, .. } => Some(*bonus),
            _ => None,
        });
        let span = span.filter(|span| span.ongoing() || span.ended_since(seen));
        let to_face = match decision.map(|decision| (&decision.kind, &decision.context)) {
            Some((DecisionKind::FaceMonster, Context::Monsters(monsters))) => monsters
                .iter()
                .filter_map(|id| foe(view, seat, id, bonus))
                .collect(),
            _ => Vec::new(),
        };
        Some(Self {
            who: view.name(seat),
            sanity: (inv.sanity, view.maximum(seat, Resource::Sanity)),
            stamina: (inv.stamina, view.maximum(seat, Resource::Stamina)),
            clues: inv.clues,
            success_from: view.success_face(seat),
            foe: span.and_then(|span| foe(view, seat, &span.foe, bonus)),
            lines: span.map_or_else(Vec::new, |span| {
                let to = span.end.map_or(outcome.events.len(), |(_, line)| line);
                journal_lines(view, &outcome.events, &outcome.shown, span.start, to)
            }),
            ending: span
                .and_then(|span| span.end)
                .map(|(end, _)| end_label(end)),
            to_face,
        })
    }
}

/// A monster with its values and the dice rolled against it.
fn foe(view: &View<'_, '_>, seat: usize, id: &MonsterId, bonus: Option<i32>) -> Option<FoeView> {
    let token = monster(view, id)?;
    let stats = view.monster_stats(id)?;
    let mut dice = vec![format!(
        "Évasion : {} dé(s)",
        view.check_dice(seat, CheckKind::Evade, stats.awareness)
    )];
    if let Some(horror) = stats.horror {
        dice.push(format!(
            "Horreur : {} dé(s)",
            view.check_dice(seat, CheckKind::Horror, i32::from(horror.modifier))
        ));
    }
    if let Some(combat) = stats.combat {
        let modifier = i32::from(combat.modifier) + bonus.unwrap_or(0);
        let dice_count = view.check_dice(seat, CheckKind::Combat, modifier);
        dice.push(match bonus {
            Some(bonus) => format!(
                "Combat : {dice_count} dé(s), armes en main comprises ({bonus:+}), {} succès nécessaire(s)",
                stats.toughness
            ),
            None => format!(
                "Combat : {dice_count} dé(s) sans les armes, {} succès nécessaire(s)",
                stats.toughness
            ),
        });
    }
    let portrait = view
        .data()
        .monsters
        .monsters
        .iter()
        .find(|monster| &monster.id == id)
        .map(|monster| illustration_url(IllustrationSize::Large, &monster.portrait))
        .unwrap_or_default();
    Some(FoeView {
        stats: token.tip.lines.clone(),
        token,
        dice,
        portrait,
    })
}

impl CombatDecision {
    /// The form of a decision taken on the combat page. Weapons and spells
    /// taken in hand or let go are moved as cards (see
    /// [`crate::views::HandZones`]), not by these buttons.
    pub fn new(view: &View<'_, '_>, decision: &Decision, move_number: usize) -> Self {
        let preparing = prepares_combat(decision);
        let buttons = decision
            .options
            .iter()
            .enumerate()
            .filter(|(_, option)| {
                !(preparing && matches!(option.choice, Choice::Card(_) | Choice::Drop(_)))
            })
            .map(|(index, option)| (index, option.label.clone()))
            .collect();
        Self {
            form: DecisionView::new(view, decision, move_number),
            buttons,
            hands: hands_choice(view, decision),
        }
    }
}

/// The choice before a round, when the decision is to keep or change the
/// cards in hand.
fn hands_choice(view: &View<'_, '_>, decision: &Decision) -> Option<HandsChoice> {
    if decision.kind != DecisionKind::ChangeHands {
        return None;
    }
    let find = |wanted: &Choice| {
        decision
            .options
            .iter()
            .position(|option| &option.choice == wanted)
    };
    let keep = find(&Choice::Done)?;
    let change = find(&Choice::Yes)?;
    let can_flee = match &decision.context {
        Context::Combat { foe: Some(foe), .. } => view
            .monster_stats(foe)
            .is_none_or(|stats| !stats.has(MonsterAbility::Ambush)),
        _ => true,
    };
    Some(HandsChoice {
        keep,
        change,
        held: decision.options[keep].label.clone(),
        can_flee,
    })
}

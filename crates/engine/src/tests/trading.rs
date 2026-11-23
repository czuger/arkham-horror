//! Trading equipment between investigators (§18.6, §23.7).

use std::collections::VecDeque;

use data::CardId;

use super::{clean_state, data, hold, put_at};
use crate::decision::{Choice, Decision, DecisionKind, Responder};
use crate::game::Game;
use crate::state::{GameState, HeldCard};
use crate::view::View;

/// Answers by choice rather than by index; records the decisions asked.
struct ByChoice {
    answers: VecDeque<Choice>,
    asked: Vec<Decision>,
}

impl Responder for ByChoice {
    fn answer(&mut self, _view: &View<'_, '_>, decision: &Decision) -> Option<usize> {
        self.asked.push(decision.clone());
        let wanted = self.answers.pop_front()?;
        decision.find(&wanted)
    }
}

/// Two investigators in the Woods: seat 0 with a Revolver .38 in hand, a
/// Bravery skill and 5 $; seat 1 with a Whiskey and 3 $.
fn two_in_the_woods() -> GameState {
    let mut state = clean_state(2, "azathoth");
    put_at(&mut state, "woods");
    state.investigators[1].place = state.investigators[0].place.clone();
    hold(&mut state, "revolver_38");
    hold(&mut state, "bravery");
    state.investigators[0].in_hand = vec![CardId::new("revolver_38")];
    state.investigators[0].money = 5;
    state.investigators[1]
        .cards
        .push(HeldCard::new(CardId::new("whiskey")));
    state.investigators[1].money = 3;
    state
}

fn play(
    state: &mut GameState,
    answers: Vec<Choice>,
    step: fn(&mut Game) -> crate::halt::Flow<()>,
) -> Vec<Decision> {
    let mut responder = ByChoice {
        answers: answers.into(),
        asked: Vec::new(),
    };
    let mut game = Game::new(data(), state.clone(), &mut responder);
    assert_eq!(step(&mut game), Ok(()));
    *state = game.state;
    responder.asked
}

fn names(state: &GameState, seat: usize) -> Vec<&str> {
    state.investigators[seat]
        .cards
        .iter()
        .map(|held| held.card.as_str())
        .collect()
}

#[test]
fn an_accepted_trade_moves_items_and_money_both_ways() {
    let mut state = two_in_the_woods();
    let answers = vec![
        Choice::Investigator(1),
        Choice::Give(CardId::new("revolver_38")),
        Choice::Take(CardId::new("whiskey")),
        Choice::GiveMoney(4),
        Choice::TakeMoney(1),
        Choice::Yes,
        Choice::Yes,
        Choice::Done,
        Choice::Done,
    ];
    let asked = play(&mut state, answers, |game| game.movement_phase());
    assert_eq!(names(&state, 0), ["bravery", "whiskey"]);
    assert_eq!(names(&state, 1), ["revolver_38"]);
    assert!(state.investigators[0].in_hand.is_empty());
    assert_eq!(state.investigators[0].money, 2);
    assert_eq!(state.investigators[1].money, 6);
    let accept = asked
        .iter()
        .find(|decision| decision.kind == DecisionKind::AcceptTrade)
        .expect("the partner is asked");
    assert_eq!(accept.who, 1);
    // Skills do not trade.
    let offer = asked
        .iter()
        .find(|decision| decision.kind == DecisionKind::Trade)
        .expect("a trade");
    assert!(offer.find(&Choice::Give(CardId::new("bravery"))).is_none());
}

#[test]
fn a_refused_trade_changes_nothing() {
    let mut state = two_in_the_woods();
    let before = state.clone();
    let answers = vec![
        Choice::Investigator(1),
        Choice::Give(CardId::new("revolver_38")),
        Choice::Yes,
        Choice::Done,
        Choice::Done,
        Choice::Done,
    ];
    play(&mut state, answers, |game| game.movement_phase());
    assert_eq!(state.investigators, before.investigators);
}

#[test]
fn no_trade_is_offered_to_someone_elsewhere() {
    let mut state = two_in_the_woods();
    put_at(&mut state, "unvisited_isle");
    let asked = play(&mut state, vec![Choice::Done, Choice::Done], |game| {
        game.movement_phase()
    });
    assert!(asked.iter().all(|decision| {
        decision
            .options
            .iter()
            .all(|option| !matches!(option.choice, Choice::Investigator(_)))
    }));
}

//! Unit tests of the rules, on crafted states with forced dice.

mod battle;
mod checks;
mod combat;
mod gates;
mod monsters;
mod status;
mod turn;

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::OnceLock;

use data::{AncientOneId, CardId, GameData, InvestigatorId, LocationId, MonsterId};

use crate::decision::{Decision, FirstOption, Responder};
use crate::game::Game;
use crate::halt::Flow;
use crate::play::initial_state;
use crate::setup::Setup;
use crate::state::{BoardMonster, GameState, HeldCard, MonsterPlace, Place};

/// The real game data, loaded once.
pub(crate) fn data() -> &'static GameData {
    static DATA: OnceLock<GameData> = OnceLock::new();
    DATA.get_or_init(|| {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
        GameData::load(&dir).unwrap_or_else(|error| panic!("{error}"))
    })
}

/// Answers from a list; fails the test when the list runs out.
struct Scripted(VecDeque<usize>);

impl Responder for Scripted {
    fn answer(&mut self, _state: &GameState, _decision: &Decision) -> Option<usize> {
        self.0.pop_front()
    }
}

/// A prepared game with Amanda Sharpe (and random others), then emptied:
/// no gate, no monster on the board, no clue, no card in hand, doom 0.
pub(crate) fn clean_state(players: usize, ancient_one: &str) -> GameState {
    let setup = Setup {
        seed: 1,
        players,
        investigators: vec![InvestigatorId::new("amanda_sharpe")],
        ancient_one: Some(AncientOneId::new(ancient_one)),
    };
    let state = initial_state(data(), &setup).unwrap_or_else(|error| panic!("{error}"));
    let mut responder = FirstOption;
    let mut game = Game::new(data(), state, &mut responder);
    if let Err(halt) = game.prepare(&setup) {
        panic!("{halt:?}");
    }
    let mut state = game.state;
    empty_board(&mut state);
    state
}

/// Takes gates, monsters, clue tokens, Mythos cards and held cards away,
/// and gives every investigator 5 Sanity and Stamina, sliders on stop 2.
fn empty_board(state: &mut GameState) {
    for gate in std::mem::take(&mut state.gates) {
        state.gate_stack.push(gate.marker);
    }
    for monster in std::mem::take(&mut state.monsters) {
        state.monster_cup.push(monster.monster);
    }
    state.clues.clear();
    for card in [
        state.environment.take(),
        state.rumor.take().map(|rumor| rumor.card),
    ]
    .into_iter()
    .flatten()
    {
        state.mythos_deck.push(card);
    }
    state.doom = 0;
    state.first_player = 0;
    for inv in &mut state.investigators {
        for held in std::mem::take(&mut inv.cards) {
            state
                .decks
                .get_mut(
                    data()
                        .investigator_cards
                        .card(&held.card)
                        .map_or(data::Deck::Special, |c| c.deck),
                )
                .push(held.card);
        }
        inv.clues = 0;
        inv.money = 0;
        inv.sanity = 5;
        inv.stamina = 5;
        inv.sliders = [1, 1, 1];
        inv.delayed = false;
    }
}

/// Runs rule code on a state with scripted answers and forced dice.
/// Returns the result and the journal.
pub(crate) fn run<T>(
    state: &mut GameState,
    answers: &[usize],
    dice: &[u8],
    step: impl FnOnce(&mut Game) -> Flow<T>,
) -> (Flow<T>, Vec<String>) {
    let mut responder = Scripted(answers.iter().copied().collect());
    let mut game = Game::new(data(), state.clone(), &mut responder);
    game.loaded_dice = dice.iter().copied().collect();
    let result = step(&mut game);
    let events = game.events.clone();
    *state = game.state;
    (result, events)
}

/// Puts the investigator at seat 0 on a location.
pub(crate) fn put_at(state: &mut GameState, location: &str) {
    state.investigators[0].place = Place::Location(LocationId::new(location));
}

/// Gives a card to the investigator at seat 0.
pub(crate) fn hold(state: &mut GameState, card: &str) {
    let card = CardId::new(card);
    for deck in [
        data::Deck::CommonItem,
        data::Deck::UniqueItem,
        data::Deck::Spell,
        data::Deck::Skill,
        data::Deck::Ally,
        data::Deck::Special,
    ] {
        let cards = state.decks.get_mut(deck);
        if let Some(index) = cards.iter().position(|candidate| candidate == &card) {
            cards.remove(index);
        }
    }
    let mut held = HeldCard::new(card);
    held.fresh = false;
    state.investigators[0].cards.push(held);
}

/// Puts a monster on the board.
pub(crate) fn add_monster(state: &mut GameState, monster: &str, place: MonsterPlace) {
    let id = MonsterId::new(monster);
    if let Some(index) = state
        .monster_cup
        .iter()
        .position(|candidate| candidate == &id)
    {
        state.monster_cup.remove(index);
    }
    state.monsters.push(BoardMonster { monster: id, place });
}

//! Playing a game: from a setup and a responder to an outcome.

use data::GameData;

use crate::decision::{Decision, Responder};
use crate::engine_error::EngineError;
use crate::game::Game;
use crate::halt::Halt;
use crate::rng::Rng;
use crate::setup::Setup;
use crate::state::{Decks, Ending, GameState, Phase};

/// Where a game stands after playing as far as possible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// Waiting for an answer to this decision.
    Waiting(Decision),
    /// Over.
    Over(Ending),
}

/// The result of playing a game as far as the answers allow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    /// The state reached.
    pub state: GameState,
    /// Waiting for a decision, or over.
    pub status: Status,
    /// Every answer accepted, in order. Replaying them with the same setup
    /// and data gives the same game.
    pub answers: Vec<usize>,
    /// The journal, in French.
    pub events: Vec<String>,
    /// Cards and encounter paragraphs played (keys of [`crate::coverage_keys`]).
    pub covered: std::collections::BTreeSet<String>,
    /// Counts of notable events.
    pub tally: crate::tally::Tally,
}

/// Plays a game from its setup, asking `responder` for every decision,
/// until the game ends or the responder has no answer.
///
/// # Errors
///
/// [`EngineError::BadSetup`] for an impossible setup,
/// [`EngineError::IllegalAnswer`] when an answer is out of range,
/// [`EngineError::Broken`] when the rules code meets a state it does not handle.
pub fn play(
    data: &GameData,
    setup: &Setup,
    responder: &mut dyn Responder,
) -> Result<Outcome, EngineError> {
    let state = initial_state(data, setup)?;
    let mut game = Game::new(data, state, responder);
    let result = game.run(setup);
    let status = match result {
        Err(Halt::Waiting(decision)) => Status::Waiting(decision),
        Err(Halt::Over) => match game.state.ending {
            Some(ending) => Status::Over(ending),
            None => {
                return Err(EngineError::Broken(
                    "fin de partie sans résultat".to_owned(),
                ));
            }
        },
        Err(Halt::Illegal {
            position,
            answer,
            decision,
        }) => {
            return Err(EngineError::IllegalAnswer {
                position,
                answer,
                decision,
            });
        }
        Err(Halt::Broken(reason)) => return Err(EngineError::Broken(reason)),
        Err(other) => return Err(EngineError::Broken(format!("arrêt inattendu : {other:?}"))),
        Ok(()) => {
            return Err(EngineError::Broken(
                "la partie s'arrête sans fin".to_owned(),
            ));
        }
    };
    Ok(Outcome {
        state: game.state,
        status,
        answers: game.answers,
        events: game.events,
        covered: game.covered,
        tally: game.tally,
    })
}

/// The state before setup: random generator and Ancient One.
pub(crate) fn initial_state(data: &GameData, setup: &Setup) -> Result<GameState, EngineError> {
    if !(1..=8).contains(&setup.players) || setup.investigators.len() > setup.players {
        return Err(EngineError::BadSetup(format!(
            "{} joueur(s) demandé(s)",
            setup.players
        )));
    }
    let mut rng = Rng::new(setup.seed);
    let ones = &data.ancient_ones.ancient_ones;
    let ancient_one = match &setup.ancient_one {
        Some(id) if ones.iter().any(|one| &one.id == id) => id.clone(),
        Some(id) => {
            return Err(EngineError::BadSetup(format!(
                "Grand Ancien « {id} » inconnu"
            )));
        }
        None => match ones.get(rng.below(ones.len())) {
            Some(one) => one.id.clone(),
            None => return Err(EngineError::BadSetup("aucun Grand Ancien".to_owned())),
        },
    };
    Ok(GameState {
        version: GameState::VERSION,
        seed: setup.seed,
        rng,
        ancient_one,
        investigators: Vec::new(),
        unused_investigators: Vec::new(),
        first_player: 0,
        turn: 0,
        phase: Phase::Setup,
        terror: 0,
        doom: 0,
        monster_cup: Vec::new(),
        monsters: Vec::new(),
        gate_stack: Vec::new(),
        gates: Vec::new(),
        seals: Vec::new(),
        clues: std::collections::BTreeMap::new(),
        closed: Vec::new(),
        decks: Decks::default(),
        mythos_deck: Vec::new(),
        gate_deck: Vec::new(),
        environment: None,
        rumor: None,
        lingering: Vec::new(),
        reserved_location_card: None,
        two_mythos_cards: false,
        battle: None,
        ending: None,
    })
}

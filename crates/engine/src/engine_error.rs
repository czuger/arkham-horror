//! Errors of the engine.

use thiserror::Error;

use crate::decision::Decision;

/// Why a game could not be played. Displayed in French.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EngineError {
    /// The setup asks for something impossible.
    #[error("préparation impossible : {0}")]
    BadSetup(String),
    /// An answer is not one of the options of its decision.
    #[error("réponse n° {position} illégale : {answer} n'est pas une option")]
    IllegalAnswer {
        /// Number of answers accepted before it.
        position: usize,
        /// The refused answer.
        answer: usize,
        /// The decision it answered.
        decision: Box<Decision>,
    },
    /// The rules code met a state it does not handle: a bug.
    #[error("erreur interne du moteur : {0}")]
    Broken(String),
}

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::database::DatabaseError;

/// Failure while handling a request.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// The database failed.
    #[error(transparent)]
    Database(#[from] DatabaseError),
    /// An HTML template failed to render.
    #[error("erreur de gabarit : {0}")]
    Template(#[from] askama::Error),
    /// The engine refused to replay a saved game.
    #[error(transparent)]
    Engine(#[from] engine::EngineError),
    /// A saved game could not be read or written as JSON.
    #[error("partie enregistrée illisible : {0}")]
    Json(#[from] serde_json::Error),
    /// No game with this number.
    #[error("Partie introuvable.")]
    NotFound,
    /// The game no longer fits the rules and was deleted.
    #[error("Cette partie ne suit plus les règles du jeu : elle a été supprimée.")]
    Outdated,
    /// The request does not make sense (illegal answer, bad form).
    #[error("{0}")]
    BadRequest(String),
    /// A form posted from another site.
    #[error("Formulaire refusé : il ne vient pas de ce site.")]
    Forbidden,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Outdated => StatusCode::GONE,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Database(_) | Self::Template(_) | Self::Engine(_) | Self::Json(_) => {
                tracing::error!(error = %self, "request failed");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erreur interne du serveur.",
                )
                    .into_response();
            }
        };
        (status, self.to_string()).into_response()
    }
}

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::database::DatabaseError;

/// Failure while handling a request; shown to the player as a 500 page.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// The database failed.
    #[error(transparent)]
    Database(#[from] DatabaseError),
    /// An HTML template failed to render.
    #[error("erreur de gabarit : {0}")]
    Template(#[from] askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!(error = %self, "request failed");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erreur interne du serveur.",
        )
            .into_response()
    }
}

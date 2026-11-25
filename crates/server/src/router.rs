use axum::Router;
use axum::http::HeaderValue;
use axum::http::header::HeaderName;
use axum::middleware;
use axum::routing::{get, post};
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::ambiance::Ambiance;
use crate::app_state::AppState;
use crate::embedded_files;
use crate::illustrations;
use crate::routes;
use crate::same_origin::same_origin;

/// Builds the application router: pages, forms, `robots.txt`, the
/// built-in images and static files, and the music.
///
/// Every response carries `X-Robots-Tag: noindex, nofollow`; forms posted
/// from another site are refused.
pub fn build_router(state: AppState) -> Router {
    let music_dir = state.music.dir.clone();
    let router = Ambiance::ALL
        .into_iter()
        .fold(Router::new(), |router, ambiance| {
            router.nest_service(
                &format!("/musique/{ambiance}"),
                ServeDir::new(music_dir.join(ambiance.folder())),
            )
        });
    router
        .route("/", get(routes::home))
        .route(
            "/nouvelle",
            get(routes::new_game_form).post(routes::create_game),
        )
        .route("/partie/{id}", get(routes::game))
        .route("/partie/{id}/reponse", post(routes::answer))
        .route("/partie/{id}/annuler", post(routes::undo))
        .route("/partie/{id}/supprimer", post(routes::delete))
        .route("/partie/{id}/journal", get(routes::journal))
        .route("/partie/{id}/combat", get(routes::combat))
        .route("/partie/{id}/grand-ancien", get(routes::ancient_one))
        .route("/partie/{id}/autre-monde", get(routes::other_world))
        .route(
            "/partie/{id}/ajustement",
            get(routes::skill_adjustment).post(routes::adjust_skills),
        )
        .route(
            "/partie/{id}/echange",
            get(routes::trade).post(routes::propose_trade),
        )
        .route("/partie/{id}/investigateurs", get(routes::investigators))
        .route(
            "/partie/{id}/investigateurs/{seat}",
            get(routes::investigator),
        )
        .route("/musique/pistes", get(routes::music_tracks))
        .route("/robots.txt", get(routes::robots))
        .route("/assets/{*path}", get(embedded_files::asset))
        .route("/static/{*path}", get(embedded_files::static_file))
        .route("/illustrations/{*path}", get(illustrations::illustration))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-robots-tag"),
            HeaderValue::from_static("noindex, nofollow"),
        ))
        .layer(middleware::from_fn(same_origin))
        .with_state(state)
}

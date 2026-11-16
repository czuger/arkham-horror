use axum::Router;
use axum::http::HeaderValue;
use axum::http::header::HeaderName;
use axum::routing::get;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::app_state::AppState;
use crate::config::Config;
use crate::routes;

/// Builds the application router: pages, `robots.txt`, images and static files.
///
/// Every response carries `X-Robots-Tag: noindex, nofollow`.
pub fn build_router(config: &Config, state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::home))
        .route("/robots.txt", get(routes::robots))
        .nest_service("/assets", ServeDir::new(&config.assets_dir))
        .nest_service("/static", ServeDir::new(&config.static_dir))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-robots-tag"),
            HeaderValue::from_static("noindex, nofollow"),
        ))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;
    use crate::database::Database;

    async fn get(path: &str) -> (StatusCode, String, Option<String>) {
        let dir = tempfile::tempdir().expect("temp dir");
        let config = Config {
            addr: "127.0.0.1:0".parse().expect("addr"),
            db_path: dir.path().join("test.db"),
            assets_dir: "../../assets".into(),
            static_dir: "../../static".into(),
        };
        let db = Database::open(&config.db_path).expect("open");
        let app = build_router(&config, AppState { db });
        let request = Request::get(path).body(Body::empty()).expect("request");
        let response = app.oneshot(request).await.expect("response");
        let status = response.status();
        let robots = response
            .headers()
            .get("x-robots-tag")
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        (status, String::from_utf8_lossy(&bytes).into_owned(), robots)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn home_page_counts_visits() {
        let (status, body, robots) = get("/").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("Bonjour"));
        assert!(body.contains("vue 1 fois"));
        assert_eq!(robots.as_deref(), Some("noindex, nofollow"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn robots_txt_forbids_crawling() {
        let (status, body, _) = get("/robots.txt").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("Disallow: /"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn static_files_are_served() {
        let (status, _, _) = get("/static/style.css").await;
        assert_eq!(status, StatusCode::OK);
    }
}

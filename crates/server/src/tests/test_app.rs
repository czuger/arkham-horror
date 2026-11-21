//! A server on a temporary database, driven without a network.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use data::GameData;
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::app_state::AppState;
use crate::config::Config;
use crate::database::Database;
use crate::router::build_router;

/// A response: status, `Location` header and body.
pub struct Reply {
    pub status: StatusCode,
    pub location: Option<String>,
    pub robots: Option<String>,
    pub body: String,
}

/// The application on a database folder.
pub struct TestApp {
    pub router: Router,
}

impl TestApp {
    /// Opens (or reopens) the application on a database folder.
    pub fn open(db_path: &Path) -> Self {
        let root = PathBuf::from("../..");
        let config = Config {
            addr: "127.0.0.1:0".parse().expect("addr"),
            db_path: db_path.to_owned(),
            assets_dir: root.join("assets"),
            static_dir: root.join("static"),
            data_dir: root.join("data"),
        };
        let data = Arc::new(GameData::load(&config.data_dir).expect("data"));
        let db = Database::open(&config.db_path).expect("open");
        let state = AppState {
            db,
            data,
            htmx: false,
        };
        Self {
            router: build_router(&config, state),
        }
    }

    pub async fn get(&self, path: &str) -> Reply {
        let request = Request::get(path).body(Body::empty()).expect("request");
        self.send(request).await
    }

    pub async fn post(&self, path: &str, form: &str) -> Reply {
        let request = Request::post(path)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(form.to_owned()))
            .expect("request");
        self.send(request).await
    }

    pub async fn send(&self, request: Request<Body>) -> Reply {
        let response = self
            .router
            .clone()
            .oneshot(request)
            .await
            .expect("response");
        let text = |name| {
            response
                .headers()
                .get(name)
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned)
        };
        let status = response.status();
        let location = text(header::LOCATION.as_str());
        let robots = text("x-robots-tag");
        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        Reply {
            status,
            location,
            robots,
            body: String::from_utf8_lossy(&bytes).into_owned(),
        }
    }
}

/// The move number a game page's decision form carries.
pub fn move_number(page: &str) -> Option<usize> {
    let start = page.find(r#"name="coup" value=""#)? + r#"name="coup" value=""#.len();
    let rest = &page[start..];
    rest[..rest.find('"')?].parse().ok()
}

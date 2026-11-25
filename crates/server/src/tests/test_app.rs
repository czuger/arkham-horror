//! A server on a temporary database, driven without a network.

use std::path::Path;
use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::app_state::AppState;
use crate::database::Database;
use crate::embedded_files::load_game_data;
use crate::music_library::MusicLibrary;
use crate::router::build_router;

/// A response: status, `Location` header and body.
pub struct Reply {
    pub status: StatusCode,
    pub location: Option<String>,
    pub robots: Option<String>,
    pub body: String,
}

/// The application on a database file.
pub struct TestApp {
    pub router: Router,
}

impl TestApp {
    /// Opens (or reopens) the application on a database file, without
    /// music.
    pub async fn open(db_path: &Path) -> Self {
        Self::with_music(db_path, MusicLibrary::default()).await
    }

    /// Opens the application on a database file and a music library.
    pub async fn with_music(db_path: &Path, music: MusicLibrary) -> Self {
        let data = Arc::new(load_game_data().expect("data"));
        let db = Database::open(db_path).await.expect("open");
        let state = AppState {
            db,
            data,
            htmx: false,
            music: Arc::new(music),
        };
        Self {
            router: build_router(state),
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
        let from = request.uri().path().to_owned();
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
        let location = text(header::LOCATION.as_str()).map(|location| {
            assert!(!location.starts_with('/'), "absolute redirect: {location}");
            resolve(&from, &location)
        });
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

/// The absolute path a browser reaches when following the relative
/// `location` from the request path `from`.
pub fn resolve(from: &str, location: &str) -> String {
    let mut segments: Vec<&str> = from.split('/').collect();
    segments.pop();
    for segment in location.split('/') {
        match segment {
            "." => {}
            ".." => {
                if segments.len() > 1 {
                    segments.pop();
                }
            }
            _ => segments.push(segment),
        }
    }
    let path = segments.join("/");
    if path.is_empty() {
        "/".to_owned()
    } else {
        path
    }
}

/// The move number a game page's decision form carries.
pub fn move_number(page: &str) -> Option<usize> {
    let start = page.find(r#"name="coup" value=""#)? + r#"name="coup" value=""#.len();
    let rest = &page[start..];
    rest[..rest.find('"')?].parse().ok()
}

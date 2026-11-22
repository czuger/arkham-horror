//! The illustrations: served with a fingerprint, shown on the scene, the map
//! and the combat page.

use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use tower::ServiceExt;

use crate::illustrations::{IllustrationSize, illustration_url};
use crate::tests::test_app::TestApp;

fn temp() -> tempfile::TempDir {
    tempfile::tempdir().expect("temp dir")
}

/// Gets `path` and returns the status and the `Cache-Control` header.
async fn cache_of(app: &TestApp, path: &str) -> (StatusCode, Option<String>) {
    let request = Request::get(path).body(Body::empty()).expect("request");
    let response = app.router.clone().oneshot(request).await.expect("response");
    let kind = response.headers().get(header::CONTENT_TYPE);
    if response.status() == StatusCode::OK {
        assert_eq!(kind.and_then(|k| k.to_str().ok()), Some("image/jpeg"));
    }
    let cache = response
        .headers()
        .get(header::CACHE_CONTROL)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    (response.status(), cache)
}

#[tokio::test(flavor = "current_thread")]
async fn illustrations_are_served_shrunk_and_cacheable() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
    for size in [IllustrationSize::Large, IllustrationSize::Thumbnail] {
        let url = illustration_url(size, "autres mondes/rlyeh_2.png");
        assert!(url.contains("autres%20mondes/rlyeh_2.jpg?v="), "{url}");
        let (status, cache) = cache_of(&app, &format!("/{url}")).await;
        assert_eq!(status, StatusCode::OK, "{url}");
        assert!(cache.is_some_and(|c| c.contains("immutable")));
        let bare = url.split('?').next().expect("path");
        let (status, cache) = cache_of(&app, &format!("/{bare}")).await;
        assert_eq!((status, cache), (StatusCode::OK, None));
    }
    let (status, _) = cache_of(&app, "/illustrations/grand/nothing.jpg").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
